# Independent Quicksilver review

Review the current implementation against the project contracts.

1. Read `AGENTS.md` and `docs/VISION.md`.
2. Read the task acceptance conditions before the implementation summary.
3. Inspect the code and recorded verification evidence.
4. Challenge source freshness and cache invalidation assumptions.
5. Challenge semantic completeness claims with counterexamples.
6. Verify behavior during a failed or stalled BEAM worker.
7. Verify UTF-16 and UTF-8 positions with non-ASCII fixtures.
8. Verify external file writes and open document precedence.
9. Inspect every advertised capability and its acceptance gate.
10. Inspect benchmark denominators, failures, and cache states.
11. Inspect OMP evidence for duplicate Elixir servers.
12. Report material defects with file paths and reproducible steps.

Review output must distinguish defects from optional improvements.
A passing smoke test does not establish full semantic correctness.
A low median does not establish acceptable tail latency.

