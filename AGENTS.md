# Quicksilver engineering instructions

## Objective

Quicksilver is an Elixir language server in Rust.
Its primary objective is useful source intelligence during rapid agent edits.
Source requests remain independent of project compilation and Dialyzer.
The optional BEAM worker adds compiler facts without control over source availability.

The user authorized implementation of the complete project vision.
The user accepts sustained agent work and substantial token use.
Routine technical choices do not require a new approval request.
External publication and actions outside the project retain their normal approval requirements.

## Read order

1. Read `START_HERE.md`.
2. Read `docs/VISION.md`.
3. Read `docs/IMPLEMENTATION_PLAN.md`.
4. Read `planning/tasks.json` and `planning/PROGRESS.md`.
5. Read the specifications for the selected task.
6. Read `reports/KIT_VERIFICATION.md` before any claim about the starting state.

## Operating rules

1. Inspect the current files and toolchain before each new phase.
2. Preserve changes that the user or another agent created.
3. Select a ready task from `planning/tasks.json`.
4. Record its status as `in_progress` before the implementation starts.
5. Delegate independent work when that work has clear file ownership.
6. Implement the smallest coherent slice that satisfies the task contract.
7. Verify the slice against independent acceptance evidence.
8. Record commands, outputs, and limitations in `reports/`.
9. Update the feature matrix and task ledger together.
10. Continue to the next ready task without a routine confirmation request.

## State and checkpoints

The task ledger is the canonical source for task dependencies and completion state.
`planning/PROGRESS.md` contains the latest execution checkpoint.
`docs/FEATURE_MATRIX.md` contains the externally visible capability state.
The specifications contain the intended behavior.
The code and verification evidence contain the observed behavior.

Use these task states:

- `pending`: The task has no verified implementation.
- `in_progress`: The task has an active owner.
- `blocked`: A concrete prerequisite prevents completion.
- `done`: The implementation and every acceptance condition have evidence.

Before a session ends, update `planning/PROGRESS.md`.
Record the active task and the next exact command.
Record any uncommitted change and unresolved failure.
Keep source revisions or content hashes with benchmark results.
Use local commits when the repository already uses Git.
If this directory has no repository, initialize a local repository before the first implementation checkpoint.
Do not create or push a remote without the owner's authorization.

## Required engineering invariants

- Source navigation has no dependency on Mix, a PLT, or a worker response.
- A failed compile does not erase current source declarations.
- A document edit creates a new coherent source revision.
- Requests use immutable snapshots or an equivalent consistency boundary.
- Open document text has precedence over disk text.
- External writes receive explicit synchronization and invalidation.
- Paths and caches preserve project and worktree identity.
- A result carries valid positions in the negotiated encoding.
- A timeout and a complete empty result are different outcomes.
- A capability appears in `initialize` only after its acceptance gate passes.
- Standard LSP result shapes remain standard.
- Optional metadata uses a negotiated extension or a separate status request.
- Compiler facts retain their source revision and runtime identity.
- Logging never writes nonprotocol text to stdout in stdio mode.
- Source-only mode never launches Mix or fetches dependencies.
- A compile request never launches the Phoenix application server.
- Diagnostics from obsolete revisions never replace current diagnostics.
- Worker restarts do not restart the source index.
- Rename requires complete evidence for its claimed scope.
- Feature coverage, latency, and memory are separate release gates.

## Scope control

The included scaffold does not define the final scope.
Passing its smoke test does not establish semantic correctness.
A fast syntax scanner alone does not complete Quicksilver.
The full vision includes the optional BEAM worker and the documented language features.
Unsupported cases require explicit behavior and test coverage.

Do not remove hard acceptance cases to improve a reported success rate.
Do not weaken budgets after a failure without a recorded decision and supporting evidence.
Do not replace exact semantic tests with broad substring tests.
Do not report a mocked compiler result as a real compiler test.
Do not use an empty array to conceal a known unsupported semantic query.
Do not use `cargo run` as the installed LSP command.
Do not put a full workspace scan on every request path.
Do not put Dialyzer on the source request path.
Do not implement a second Elixir compiler as a prerequisite for useful source features.
Do not copy an upstream implementation without its required license and provenance.

## Implementation choices

Rust is the selected implementation language.
The seed uses a single package for review simplicity.
Extract modules or workspace crates when actual dependency boundaries justify them.
The architecture documents describe those boundaries rather than mandate empty crates.

Tree-sitter supplies the initial Elixir syntax tree.
The resolver must supply Elixir scope and identity semantics.
Salsa is the preferred candidate for incremental queries.
An alternative remains acceptable when its invalidation and latency evidence meet the same contracts.

The seed pins its verified Rust toolchain and dependency lockfile.
A dependency update requires a compatibility reason and verification evidence.
The worker runtime matrix requires actual compatible Elixir and OTP pairs.
The release support matrix must name tested pairs rather than broad unverified ranges.

## Task completion procedure

1. Read every acceptance condition for the task.
2. Run the required verification on the current code revision.
3. Record failures as failures.
4. Record unavailable external environments as unavailable.
5. Add a report path to the task's `evidence` array.
6. Set the task to `done` only after all required conditions pass.
7. Update `docs/FEATURE_MATRIX.md` for any public behavior change.
8. Run `python3 scripts/validate_kit.py` after a ledger change.

## Review procedure

1. Assign an independent reviewer after each phase.
2. Give the reviewer the requirements and evidence before the implementation explanation.
3. Request counterexamples for correctness and invalidation claims.
4. Resolve material findings before the next dependent phase.
5. Record any accepted limitation in the feature matrix.

## Tests and evidence

The existing Python protocol client is an independent external test boundary.
The semantic fixture catalog defines expected behaviors that the seed cannot yet pass.
Those fixtures are future acceptance material.
The initial benchmark measures the scaffold only.
It does not establish production semantic latency.

New semantic tests must state their oracle.
The compiler is an oracle for compilation behavior and generated exports.
The compiler is not an oracle for a complete source reference graph.
Hand-authored fixtures and source identity rules provide that separate oracle.

Performance reports must include the exact binary and source revision.
They must include hardware, corpus size, cache state, and sample count.
They must retain failures and timeouts in the results.
Warm and cold results must remain separate.

## Communication

Give concise progress updates during sustained work.
Explain findings and their consequences.
State the next uncertainty that the work will resolve.
Do not end a phase with an offer to continue.
Continue until the project satisfies all release gates or a concrete external blocker prevents further authorized work.

## Technical writing

Use American spelling and complete grammar.
Use descriptive sentences with at most 25 words.
Use simple tenses and active voice.
Use procedural sentences with at most 20 words.
Put one instruction in each procedural sentence.
Separate procedures from descriptions.
Use `can`, `will`, or `must` for modal verbs.
Avoid contractions and present perfect constructions.
Use `verify` consistently for validation instructions.
Preserve code identifiers and exact error messages.

## Final implementation report

The report must include these sections:

- Delivered features and deliberate limitations.
- The supported platform and runtime matrix.
- Correctness evidence and coverage denominators.
- Cold, warm, and repeated-edit latency results.
- Memory and process recovery results.
- OMP integration evidence with the selected servers.
- Reproducible install and recovery commands.
- Remaining blocked gates, if any.

The phrase `full vision complete` requires every task to pass its acceptance conditions.
A blocked release gate prevents that claim.

