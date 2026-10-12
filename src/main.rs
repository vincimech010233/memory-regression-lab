use memory_regression_lab::{Config, Program, summarize};
use serde_json::{Value, json};
use std::{
    io::{Read, Write},
    os::unix::process::CommandExt,
    process::{Command, Stdio},
    time::{Duration, Instant},
};
fn resolved_output(path: &str) -> Result<std::path::PathBuf, String> {
    let path = std::path::Path::new(path);
    let filename = path.file_name().ok_or("report filename required")?;
    let parent = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(std::path::Path::new("."));
    Ok(std::fs::canonicalize(parent)
        .map_err(|e| e.to_string())?
        .join(filename))
}
fn create_report(path: &str, bytes: impl AsRef<[u8]>) -> Result<(), String> {
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|e| e.to_string())?;
    file.write_all(bytes.as_ref()).map_err(|e| e.to_string())
}
fn measure(p: &Program, timeout: u64) -> Result<Value, String> {
    let mut cmd = Command::new(&p.executable);
    cmd.args(&p.args)
        .stdout(Stdio::piped())
        .stderr(Stdio::null());
    // Async-signal-safe libc calls only between fork and exec; bounds apply per child.
    unsafe {
        cmd.pre_exec(|| {
            let limit = libc::rlimit {
                rlim_cur: 512 * 1024 * 1024,
                rlim_max: 512 * 1024 * 1024,
            };
            if libc::setrlimit(libc::RLIMIT_AS, &limit) != 0 {
                return Err(std::io::Error::last_os_error());
            }
            Ok(())
        });
    }
    let start = Instant::now();
    let mut child = cmd.spawn().map_err(|e| e.to_string())?;
    let pid = child.id() as libc::pid_t;
    let stream = child.stdout.take().unwrap();
    let reader = std::thread::spawn(move || {
        let mut b = vec![];
        stream.take(65537).read_to_end(&mut b).map(|_| b)
    });
    let mut status = 0;
    let mut usage: libc::rusage = unsafe { std::mem::zeroed() };
    let mut timed_out = false;
    loop {
        let found = unsafe { libc::wait4(pid, &mut status, libc::WNOHANG, &mut usage) };
        if found == pid {
            break;
        }
        if found < 0 {
            let error = std::io::Error::last_os_error();
            if error.raw_os_error() == Some(libc::EINTR) {
                continue;
            }
            let _ = child.kill();
            loop {
                let r = unsafe { libc::wait4(pid, &mut status, 0, &mut usage) };
                if r >= 0 || std::io::Error::last_os_error().raw_os_error() != Some(libc::EINTR) {
                    break;
                }
            }
            let _ = reader.join();
            return Err(error.to_string());
        }
        if start.elapsed() > Duration::from_millis(timeout) {
            timed_out = true;
            let _ = child.kill();
            loop {
                let reaped = unsafe { libc::wait4(pid, &mut status, 0, &mut usage) };
                if reaped == pid {
                    break;
                }
                let error = std::io::Error::last_os_error();
                if error.raw_os_error() == Some(libc::EINTR) {
                    continue;
                }
                let _ = reader.join();
                return Err(format!(
                    "timeout cleanup could not confirm reaping: {error}"
                ));
            }
            break;
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    let elapsed = start.elapsed().as_secs_f64() * 1000.0;
    let bytes = reader
        .join()
        .map_err(|_| "output reader failed")?
        .map_err(|e| e.to_string())?;
    if timed_out {
        return Err("fixture timed out; killed and reaped".into());
    }
    if !libc::WIFEXITED(status) || libc::WEXITSTATUS(status) != 0 {
        return Err(format!("fixture failed wait_status={status}"));
    }
    if bytes.len() > 65536 {
        return Err("fixture output exceeds 64 KiB".into());
    }
    let instrumentation: Value =
        serde_json::from_slice(&bytes).map_err(|e| format!("fixture instrumentation: {e}"))?;
    for key in [
        "allocation_calls",
        "allocated_bytes",
        "peak_live_bytes",
        "checksum",
    ] {
        if !instrumentation[key].is_u64() {
            return Err(format!("missing integer metric {key}"));
        }
    }
    Ok(
        json!({"elapsed_ms":elapsed,"peak_rss_kib":usage.ru_maxrss,"instrumentation":instrumentation}),
    )
}
fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        return Err("usage: memory-regression-lab CONFIG REPORT_JSON REPORT_MD".into());
    }
    let json_path = resolved_output(&args[2])?;
    let markdown_path = resolved_output(&args[3])?;
    if json_path == markdown_path
        || std::fs::symlink_metadata(&json_path).is_ok()
        || std::fs::symlink_metadata(&markdown_path).is_ok()
    {
        return Err("reports already exist or overlap; choose distinct fresh output paths to preserve evidence".into());
    }
    if std::fs::metadata(&args[1])
        .map_err(|e| e.to_string())?
        .len()
        > 65536
    {
        return Err("config exceeds 64 KiB".into());
    }
    let config: Config =
        serde_json::from_str(&std::fs::read_to_string(&args[1]).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    config.validate()?;
    for _ in 0..config.warmups {
        measure(&config.baseline, config.timeout_ms)?;
        measure(&config.candidate, config.timeout_ms)?;
    }
    let mut a = vec![];
    let mut b = vec![];
    for i in 0..config.repeats {
        if i % 2 == 0 {
            a.push(measure(&config.baseline, config.timeout_ms)?);
            b.push(measure(&config.candidate, config.timeout_ms)?);
        } else {
            b.push(measure(&config.candidate, config.timeout_ms)?);
            a.push(measure(&config.baseline, config.timeout_ms)?);
        }
    }
    if a.iter()
        .chain(&b)
        .any(|s| s["instrumentation"]["checksum"] != a[0]["instrumentation"]["checksum"])
    {
        return Err("functional checksums differ; comparison rejected".into());
    }
    let times = |s: &[Value]| {
        s.iter()
            .map(|x| x["elapsed_ms"].as_f64().unwrap())
            .collect::<Vec<_>>()
    };
    let sa = summarize(&times(&a))?;
    let sb = summarize(&times(&b))?;
    let paired = a
        .iter()
        .zip(&b)
        .map(|(x, y)| y["elapsed_ms"].as_f64().unwrap() - x["elapsed_ms"].as_f64().unwrap())
        .collect::<Vec<_>>();
    let mean = paired.iter().sum::<f64>() / paired.len() as f64;
    let sd =
        (paired.iter().map(|d| (d - mean).powi(2)).sum::<f64>() / (paired.len() - 1) as f64).sqrt();
    let w = 1.96 * sd / (paired.len() as f64).sqrt();
    let r = json!({"classification":"OBSERVED","config":config,"pair_order":(0..config.repeats).map(|i|if i%2==0{"baseline_then_candidate"}else{"candidate_then_baseline"}).collect::<Vec<_>>(),"verdict":"INCONCLUSIVE: timing observations are not proof of a regression","baseline":{"summary":sa,"samples":a},"candidate":{"summary":sb,"samples":b},"paired_candidate_minus_baseline_ms":{"mean":mean,"approximate_ci95":[mean-w,mean+w]},"metrics":"Linux wait4 peak RSS includes process image and allocator behavior. Instrumentation counts fixture allocator calls/bytes; neither proves leaks. Normal 1.96 intervals are exploratory and can under-cover at small n. Single-process trusted fixtures only."});
    create_report(&args[2], serde_json::to_string_pretty(&r).unwrap())
        .map_err(|e| e.to_string())?;
    create_report(&args[3],format!("# Memory Regression Lab\n\nOBSERVED; {}\n\nBaseline mean {:.3} ms; candidate mean {:.3} ms. Paired mean difference {:.3} ms, exploratory interval [{:.3}, {:.3}] ms.\n\n{}\n",r["verdict"].as_str().unwrap(),r["baseline"]["summary"]["mean"].as_f64().unwrap(),r["candidate"]["summary"]["mean"].as_f64().unwrap(),mean,mean-w,mean+w,r["metrics"].as_str().unwrap())).map_err(|e|e.to_string())?;
    Ok(())
}
fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(2)
    }
}
