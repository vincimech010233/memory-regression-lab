# Acceptance criteria

1. Rust summary matches hand-calculated reference and rejects invalid/small samples.
2. Both compiled language fixtures preserve checksum while requested memory rises; oversized input rejects.
3. Failed and timed-out child runs fail without success report.
4. Actual bounded repeated experiment saves raw samples/RSS/dispersion and uncertain verdict.
5. Rust fmt/lint/tests and JavaScript dashboard tests pass; limitations separate metrics.

All criteria must be executed locally or explicitly listed as unresolved. Reviewer acceptance is bounded by this profile.
