# Implementation plan

## Plan model

`planning/tasks.json` contains the authoritative dependency graph.
Each task has deliverables, acceptance conditions, verification commands, and evidence paths.
The graph supports parallel work within a phase.
The final release task depends on every mandatory acceptance area.

The seed includes a working scaffold.
The first task re-verifies that scaffold in the actual implementation environment.
Every later capability starts as pending.
Code availability alone never changes a task to complete.

## Phase order

| Phase | Objective | Exit condition |
| --- | --- | --- |
| P0 | Establish the baseline and its evidence | Seed behavior and gaps are independently verified |
| P1 | Stabilize transport, documents, and workspace identity | Protocol and synchronization tests cover normal and adversarial sequences |
| P2 | Build the incremental source index | Cold and warm source queries work without project execution |
| P3 | Add exact supported semantics and navigation | Supported scope fixtures pass with complete expected targets |
| P4 | Add optional BEAM enrichment | Worker failures never block source behavior |
| P5 | Add completion, refactors, and templates | Each advertised feature passes its capability gate |
| P6 | Prove OMP behavior and release quality | Platform, semantic, stress, performance, and package gates pass |

## P0: Baseline

The baseline contains LSP initialization and a narrow source feature set.
Its value is a verified executable path through the entire protocol boundary.
It does not contain a workspace semantic engine.

The implementation agent first verifies the shipped evidence on its machine.
The report identifies the actual Rust and Python versions.
The report records unavailable tools without a passing substitute.

## P1: Protocol and document state

The transport requires explicit lifecycle states and bounded resource use.
Document state requires monotonic revisions and correct encoding conversion.
Workspace identity requires canonical paths without accidental worktree collapse.
The implementation first stabilizes these boundaries before a persistent semantic index depends on them.

A test sequence can contain document changes before a pending request finishes.
It can contain a close after an external disk write.
It can contain a failed parse followed by a valid edit.
Each sequence needs a deterministic outcome.

## P2: Source index

This phase creates the principal latency advantage.
Parsing and source index updates happen without a Mix process.
The first useful result does not wait for a complete workspace scan.
Persisted data has a schema version and explicit invalidation keys.

The query engine can use Salsa after a focused compatibility evaluation.
Its input model includes file content and relevant project configuration.
Filesystem reads remain outside pure semantic queries.
Each cache entry must have a stated dependency model.

## P3: Semantics and navigation

This phase supplies real code intelligence beyond a syntax outline.
The resolver models lexical scope and Elixir identities.
Function identity includes the module and arity.
Default arguments and pipe syntax require explicit arity rules.
Aliases and imports require correct scope and precedence.

References use resolved identities rather than textual name matches.
The test corpus contains identical names with different meanings.
Rename starts with complete local scopes.
Project rename requires a stronger completeness gate.
Open-document versions do not protect closed files.
Closed-file edits require an explicit client precondition contract or a strict refusal.

## P4: Optional BEAM worker

The worker runs under a supported project runtime.
Its build state remains separate from the user's normal build directories.
It supplies compiler diagnostics and metadata from compiled modules.
The source service owns its availability independently.

The worker can coalesce repeated compile requests.
It does not fetch dependencies automatically.
It does not run the application as a normal analysis step.
It does not convert a failed compile into empty successful diagnostics.
Dialyzer remains an explicit optional job.

## P5: Feature completion

Completion uses the same resolver as navigation.
Hover and signatures distinguish declarations from inferred analysis.
Formatting uses an explicit formatter command and versioned document content.
Code actions preserve scope and version boundaries.
Semantic tokens share the resolver and preserve exact encoded ranges.
Selection and folding ranges use the syntax model with explicit malformed-source tests.

EEx and HEEx require dedicated position mapping.
The implementation must not advertise a template extension as complete because embedded text parses as Elixir.
The feature matrix tracks template support independently.

## P6: Integration and release

The OMP gate verifies the actual client behavior.
The test uses a prebuilt Quicksilver executable.
Both legacy Elixir server entries remain disabled during comparison.
The report identifies the OMP version and effective configuration.

The release matrix starts with Linux x86_64 and macOS arm64.
Windows support remains experimental until its filesystem and process tests pass.
Cross-compilation alone does not establish platform support.
The initial enriched-mode runtime pair is selected from the official Elixir compatibility matrix.
Additional runtime pairs require independent execution evidence.

## Parallel work

| Workstream | Eligible work | Shared boundary |
| --- | --- | --- |
| Protocol | Transport, encoding, cancellation, and client sequences | Request and document contracts |
| Analysis | Parse trees, scopes, index, and resolver | Immutable snapshot API |
| Worker | Runtime discovery, compile jobs, metadata, and recovery | Versioned worker protocol |
| Quality | Fixtures, independent oracles, benchmarks, and packaging | Feature matrix and gate definitions |

File ownership must be explicit before parallel edits start.
The coordinator owns changes to shared contracts.
Each worker supplies verification evidence with its implementation.
The coordinator resolves contract drift before dependent tasks continue.

## Decision changes

Architecture choices can change when evidence supports a better implementation.
The product invariants remain requirements.
A change to the incremental engine requires the same invalidation tests.
A change to a performance budget requires an explicit decision with measurements.
A narrower feature promise requires an explicit feature matrix update and a stated reason.

## Completion evidence

The final report includes source revisions for all results.
It includes precise feature denominators and deliberate unsupported cases.
It includes actual performance samples and their failed requests.
It includes executable installation and recovery procedures.

The final task remains pending until all mandatory areas pass.
An unavailable external environment can block that task.
The implementation still completes every unaffected task before it reports the blocker.

