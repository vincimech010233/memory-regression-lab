# Critical review — Memory Regression Lab

Reviewer: separate agent using the developer's model; independent execution pass, not an independent model or external audit. Scope: bounded synthetic single-process fixtures, source, tests, report arithmetic, CI and dependency documentation. No existing repository changes or publication.

## Decision and retained initial findings

VERIFIED: **VALIDATED_LOCAL** for the bounded criteria in `docs/ACCEPTANCE.md`, after the corrections below. The initial gate was PARTIALLY_VALIDATED; that critique is retained. No production regression, leak detection, process-tree isolation or general statistical validity is established. Final manual publication still requires repository and privacy/artifact checks. Failed/inconclusive evidence is preserved in `results/reviewer-fixtures/observations.json`.

1. **Report provenance incomplete.** OBSERVED by inspection: reports contain samples and summaries, but omit executables/arguments, repeats, warmups, timeout and pair execution order. `examples/experiment.json` provides the current intended experiment but is not bound to each report. Correction required: embed the validated configuration and ordering convention in reports, preserving the distinction between user-supplied configuration and observed execution. This is essential to assess what the recorded numbers represent. VERIFIED correction: regenerated report embeds the exact validated config and seven alternating pair orders. Reviewer independently compared these against the example and recalculated regenerated summary/paired arithmetic; see `results/reviewer-fixtures/retest-observations.json`.
2. **Child cleanup on wait errors.** OBSERVED by inspection: a negative `wait4` result kills the child then returns without reaping or joining the output reader. EINTR should be retried; other errors need cleanup and reaping. VERIFIED correction by source inspection: EINTR is retried, unrecoverable wait errors kill/reap/join, and the timeout path confirms exact-PID reaping before claiming success. OBSERVED: the three Python functional tests passed again after this correction. Actual signal-interruption/error fault injection remains NOT_VERIFIED.

## Independently executed checks

OBSERVED: `timeout 90 cargo test --locked --jobs 2` passed four Rust tests; `cargo clippy --locked -- -D warnings` and `cargo fmt --check` passed; `node --test tests/dashboard.test.js` passed one test; `python3 tests/functional.py` passed three tests for both compiled fixtures, child failure and timeout. The reviewer reused the developer's compiled C++ artifact and did not independently rebuild the extracted toolchain.

OBSERVED: four additional CLI cases used a tiny local single-process Python fixture, three repeats and a 2-second per-child timeout. Valid instrumentation produced a report; oversized output, malformed JSON and checksum mismatch exited 2 without fresh report files. Oversized-output failure was observed as a child failure after pipe closure rather than the dedicated size diagnostic; it still failed safely for this bounded case. Each case/config and the observation record is in `results/reviewer-fixtures/`.

VERIFIED: the saved actual seven-pair experiment has seven samples per arm. Independent Python `statistics` calculations matched mean, median, sample standard deviation, min/max and the exploratory paired interval to floating-point tolerance. This validates report arithmetic, not statistical confidence coverage or production regression detection.

## Measurement and uncertainty assessment

VERIFIED by source inspection: `wait4` uses the exact direct-child PID and records Linux peak RSS; it does not conflate accumulated usage from prior children. Baseline/candidate order alternates and warmups are excluded from summaries. Wall time includes process start, polling and scheduling. The reported conclusion stays INCONCLUSIVE even when an exploratory interval excludes zero.

The Rust allocator reports successful allocation requests before reporting; C++ counts one explicitly instrumented `new[]` allocation. They have different scopes and are not cross-language equivalent counters. Peak requested live bytes, total allocated bytes and resident memory are different quantities. Neither establishes a leak. The fixed endpoint checksum is a narrow functional check, not proof that all workload results are equivalent.

The 1.96 normal intervals are disclosed as exploratory and can under-cover at small n. [NIST mean confidence intervals](https://www.itl.nist.gov/div898/handbook/eda/section3/eda352.htm) describe sample-standard-deviation intervals using a t critical value. This project does not establish independence, normality, thermal stability, controlled isolation or statistical power. No deterministic timing thresholds or automatic regression claims are accepted.

## Security, provenance and remaining limits

VERIFIED by inspection: config rejects unknown fields and enforces bounded repeats/warmups/timeouts; direct children get a 512 MiB virtual-address limit; stdout reading retains at most 65,537 bytes. This is not a sandbox: the executable is trusted and can access the user's files. Child descendants are unsupported; inherited pipes can keep the reader blocked outside the direct-child timeout. General process-tree supervision is NOT_VERIFIED.

OBSERVED: project MIT license, locked dependencies, license metadata and bounded OSV review are present. Exhaustive legal review, supply-chain authenticity, absence of vulnerabilities and original authorship beyond supplied provenance are NOT_VERIFIED. The extracted compiler is ignored and is not part of distributable source. Hosted CI execution is NOT_VERIFIED; its configuration uses available standard Linux tooling and deterministic checks without timing thresholds or credentials.

INFERRED: transparent fixture accounting and retained raw samples offer credible educational/portfolio value alongside mature profilers; `docs/RESEARCH.md` acknowledges hyperfine, heaptrack and Google Benchmark. No novel profiling or statistical capability is claimed. Remaining follow-up: maintained process-tree supervision if scope expands, richer checksum validation for production workloads, and experiment/toolchain identity hashes before making historical comparisons.


## Post-review correction — historical experiment preservation

OBSERVED: final audit found that report regeneration after provenance correction superseded the first inconclusive benchmark before its raw samples were archived. `results/legacy-run-aggregate.json` retains only rounded previously observed summaries and the INCONCLUSIVE outcome. Historical raw samples are explicitly NOT_VERIFIED/unrecoverable in the project evidence; they were not reconstructed. The initial reviewer arithmetic checks describe observed execution, but they cannot substitute for the missing original sample archive.

Decision: **VALIDATED_LOCAL remains accepted for the current implementation and current fully retained seven-pair experiment, with an unresolved historical evidence-preservation gap.** Current raw samples/config/order support the bounded acceptance criteria. A claim that every historical experiment was completely retained is rejected. The final user delivery must disclose the lost initial raw samples; this process defect is not repaired retroactively by the prevention change.

VERIFIED by source inspection: existing JSON or Markdown output paths reject before measurement, and each report uses exclusive `create_new`, preventing replacement of an existing destination. OBSERVED: `python3 tests/functional.py` now passes four tests, including separate existing-JSON/existing-Markdown cases that preserve old bytes and create no companion output. No new expensive experiment was run for this review.

At this inspection, the early overlap guard compared path strings; aliases such as `x.json` and `./x.json` were not recognized as overlap before config loading. Exclusive creation still prevents overwriting. A bounded nonexistent-config probe confirmed the limitation without running a benchmark. The first reviewer probe itself failed due a bytes/string assertion mismatch; the corrected probe completed and confirmed the result. Early alias rejection therefore requires correction or narrowed documentation. Two report writes are sequential, not an atomic pair; a late I/O failure can leave a partial report set, which must retain its failure status.


Alias follow-up — VERIFIED correction by source inspection: output parents are canonicalized and basenames compared before configuration loading; `symlink_metadata` detects existing/dangling destinations. OBSERVED: the bounded `same.json` versus `./same.json` probe now exits 2 with the fresh-output-path error before attempting a missing config, and creates no output. No benchmark ran. The early-alias finding is resolved. Sequential/non-atomic pair output and the historical raw-sample gap remain explicitly disclosed; the qualified VALIDATED_LOCAL decision is unchanged.
