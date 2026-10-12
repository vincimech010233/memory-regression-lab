# Learn

LEARNING: Allocation requests, retained heap and peak RSS describe different quantities.

EVIDENCE: tests/functional.py, fixture source scopes and results/report.json.

IMPACT: Avoids treating resident memory or requested bytes as leak evidence.

CONFIDENCE: High for explicitly executed fixture/reference checks; external validity is NOT_VERIFIED.

CARRY_FORWARD: Keep deterministic functional gates separate from noisy performance observations.

UNRESOLVED: Controlled thermal/isolation experiments, allocator tracing, leak detection, small-sample statistical coverage and production workloads NOT_VERIFIED.

LEARNING: the initial inconclusive report was regenerated without an archive of its raw samples.

EVIDENCE: results/legacy-run-aggregate.json retains only rounded original observations; original raw samples are NOT_VERIFIED and are not reconstructed. Current report.json has complete raw samples and configuration.

IMPACT: historical evidence cannot be fully recovered.

CONFIDENCE: High for current evidence and tested overwrite protection; historical raw values NOT_VERIFIED.

CARRY_FORWARD: use fresh paths; CLI now refuses existing outputs and exclusively creates files.

UNRESOLVED: first execution raw samples are unavailable.
