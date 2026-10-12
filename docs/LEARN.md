# Learn

LEARNING: Allocation requests, retained heap and peak RSS describe different quantities.

EVIDENCE: tests/functional.py, fixture source scopes and results/report.json.

IMPACT: Avoids treating resident memory or requested bytes as leak evidence.

CONFIDENCE: High for explicitly executed fixture/reference checks; external validity is NOT_VERIFIED.

CARRY_FORWARD: Keep deterministic functional gates separate from noisy performance observations.

UNRESOLVED: Controlled thermal/isolation experiments, allocator tracing, leak detection, small-sample statistical coverage and production workloads NOT_VERIFIED.
