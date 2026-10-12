use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize)]
pub struct Summary {
    pub n: usize,
    pub mean: f64,
    pub median: f64,
    pub sample_sd: f64,
    pub approximate_mean_ci95: [f64; 2],
    pub min: f64,
    pub max: f64,
}
pub fn summarize(xs: &[f64]) -> Result<Summary, String> {
    if xs.len() < 3 || xs.iter().any(|x| !x.is_finite() || *x < 0.0) {
        return Err("at least 3 finite nonnegative samples required".into());
    }
    let n = xs.len();
    let mean = xs.iter().sum::<f64>() / n as f64;
    let sd = (xs.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (n - 1) as f64).sqrt();
    let mut sorted = xs.to_vec();
    sorted.sort_by(f64::total_cmp);
    let median = if n.is_multiple_of(2) {
        (sorted[n / 2 - 1] + sorted[n / 2]) / 2.0
    } else {
        sorted[n / 2]
    };
    let width = 1.96 * sd / (n as f64).sqrt();
    Ok(Summary {
        n,
        mean,
        median,
        sample_sd: sd,
        approximate_mean_ci95: [mean - width, mean + width],
        min: sorted[0],
        max: sorted[n - 1],
    })
}
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Program {
    pub executable: String,
    pub args: Vec<String>,
}
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub baseline: Program,
    pub candidate: Program,
    pub repeats: usize,
    pub warmups: usize,
    pub timeout_ms: u64,
}
impl Config {
    pub fn validate(&self) -> Result<(), String> {
        if !(3..=30).contains(&self.repeats)
            || self.warmups > 3
            || !(10..=2000).contains(&self.timeout_ms)
        {
            return Err("bounds: repeats 3..30, warmups 0..3, timeout_ms 10..2000".into());
        }
        for p in [&self.baseline, &self.candidate] {
            if p.executable.is_empty() || p.args.len() > 8 || p.args.iter().any(|s| s.len() > 256) {
                return Err("invalid program".into());
            }
        }
        Ok(())
    }
}
