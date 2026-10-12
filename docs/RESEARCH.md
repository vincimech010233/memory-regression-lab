# Research and scope — 2026-10-12

OBSERVED: [hyperfine](https://github.com/sharkdp/hyperfine) supplies repeated command timing and outlier/warmup controls; [heaptrack](https://github.com/KDE/heaptrack) traces allocations and call stacks; [Google Benchmark](https://github.com/google/benchmark/blob/main/docs/user_guide.md) supports benchmark aggregates and custom memory metrics. INFERRED justification: combine raw exact-child Linux RSS with explicitly scoped fixture allocation counters and an always-inconclusive exploratory comparison. Competitors are broader and generally preferable for production profiling; differentiation is the audit trail/teaching scope, not novel statistics.

Maintenance claims are limited to observed public documentation; future maintenance is NOT_VERIFIED. This project is a portfolio implementation, not a claim of market or research novelty.
