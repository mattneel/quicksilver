# Implement Quicksilver

Implement the complete Quicksilver project in this repository.
Use `AGENTS.md` as the durable engineering contract.
Use `planning/tasks.json` as the dependency graph and completion ledger.
Treat the included code as the starting slice.
Do not treat that slice as the final deliverable.

## Execution procedure

1. Read `START_HERE.md` and `AGENTS.md`.
2. Read `docs/VISION.md` and `docs/IMPLEMENTATION_PLAN.md`.
3. Inspect the code and the recorded kit verification.
4. Run `python3 scripts/task_status.py`.
5. Verify the baseline with `python3 scripts/verify.py`.
6. Select the first ready task that advances the current phase.
7. Delegate independent work with explicit file ownership.
8. Implement the task through its acceptance conditions.
9. Request independent review at each phase boundary.
10. Update the ledger and `planning/PROGRESS.md` after each coherent slice.
11. Continue through every release gate.

## Outcome

The result is a production candidate for an Elixir language server in Rust.
It includes source intelligence, optional BEAM enrichment, and verified OMP integration.
Source queries remain useful during compile failures and repeated agent edits.
PLT work never controls source query availability.
The release report contains measured correctness and latency evidence.

## Decision authority

The user authorized sustained implementation and routine engineering decisions.
Choose and document sensible defaults.
Preserve the core invariants when a local implementation choice changes.
Keep all normal tool permissions and external publication requirements.
Do not ask for a routine phase approval.

## Completion rule

Do not stop after architecture, a skeleton, or a fast symbol demo.
Do not mark an unavailable verification gate as complete.
If an external blocker remains, complete every unaffected task.
Record the exact blocker and the next command for recovery.

