# Architecture decisions

## Decision status

`Accepted` identifies a project requirement.
`Candidate` identifies a choice that still requires comparative evidence.
`Deferred` identifies work outside the current prerequisite chain.
A candidate does not authorize a performance or compatibility claim.

| ID | Decision | Status |
|---|---|---|
| DEC-001 | Rust implementation | Accepted |
| DEC-002 | Source service independent of BEAM work | Accepted |
| DEC-003 | Tree-sitter syntax foundation | Accepted |
| DEC-004 | Salsa incremental query engine | Candidate |
| DEC-005 | Immutable versioned source snapshots | Accepted |
| DEC-006 | Persistent index with a storage interface | Accepted |
| DEC-007 | SQLite persistence backend | Candidate |
| DEC-008 | Per-worktree ownership and context isolation | Accepted |
| DEC-009 | Optional local BEAM worker over pipes | Accepted |
| DEC-010 | Explicit project execution authority | Accepted |
| DEC-011 | No automatic dependency retrieval | Accepted |
| DEC-012 | Coalesced background compilation | Accepted |
| DEC-013 | Provenance before compiled metadata admission | Accepted |
| DEC-014 | Separate diagnostic producers and optional Dialyzer | Accepted |
| DEC-015 | Conservative semantic scope and rename | Accepted |
| DEC-016 | Standard LSP results with optional extensions | Accepted |
| DEC-017 | Explicit external-edit synchronization | Accepted |
| DEC-018 | Versioned framework adapters | Accepted |
| DEC-019 | Measured capability and release gates | Accepted |
| DEC-020 | DAP, remote service, and compiler replacement | Deferred |

## DEC-001: Rust implementation

Rust is the selected language for Quicksilver.
The existing LSP and parser packages reduce initial infrastructure work. [S1] [S2]
The toolchain file and lockfile define the current pinned build inputs.
Documentation must not repeat a separate conflicting dependency matrix.

This choice does not establish lower query latency by itself.
The request architecture and amount of repeated work control that outcome.
Elixir compiler behavior still belongs to the optional BEAM execution boundary.
The project does not require a Rust implementation of the Elixir compiler.

**Acceptance evidence:** Reproducible builds, platform tests, and the measured performance gates.

## DEC-002: Independent source service

Source queries must remain available before compilation succeeds.
They must also remain available during a failed compile or PLT operation.
This requirement controls the dependency direction between the Rust service and the worker.
No optimization can introduce a compile prerequisite for basic source queries.

The source analyzer supplies only facts that its implemented rules support.
Its availability does not authorize unsupported semantic claims.
The result policy therefore tracks availability and semantic coverage separately.

**Acceptance evidence:** Useful source results with no runtime, missing dependencies, a blocked worker, and a failed compile.

## DEC-003: Tree-sitter syntax foundation

The initial parser uses the Elixir Tree-sitter grammar.
The grammar already supplies a syntax foundation and Rust integration. [S2]
It does not supply the scope graph or an Elixir compiler.

Incremental parser reuse requires an accurate edit description.
Tree-sitter exposes source edit and previous-tree APIs for that purpose. [S3]
The implementation must preserve old snapshots while it constructs a new tree.
Cached node ranges cannot survive an edit without correct version treatment.

Parser output and grammar versions belong in the cache identity.
A grammar update requires source-range and error-recovery tests.
The project can replace the parser only after equivalent correctness and performance evidence.

**Acceptance evidence:** Syntax corpus, Unicode ranges, incremental edit equivalence, and malformed-source recovery.

## DEC-004: Incremental query engine

Salsa is the preferred candidate for incremental semantic queries.
Its model tracks explicit inputs and reuses derived results. [S4]
The candidate evaluation must compare it with a small explicit dependency engine.
The comparison must use the same semantics and fixture corpus.

The selected engine must preserve these properties:

- Fine-grained dependency invalidation.
- Immutable request views.
- Bounded cancellation latency.
- Explicit environment and configuration inputs.
- Predictable memory under long edit sequences.
- Observable cache reuse and recomputation counts.

A warm lookup microbenchmark alone cannot select the engine.
The evaluation must include repeated edits and dependency changes.
The engine must not read untracked mutable state from a query.
Salsa memoization does not establish durable cache persistence.

**Selection gate:** P2 evidence under [PERFORMANCE.md](PERFORMANCE.md) and [QUALITY_GATES.md](QUALITY_GATES.md).

## DEC-005: Immutable snapshots

Every source request uses one immutable logical snapshot.
Snapshots retain source identity, project context, and coverage manifests.
The storage representation can share immutable data between generations.
The design does not require a complete physical copy per edit.

A coordinator owns input mutation and publication.
Request handlers never combine mutable global maps with captured source ranges.
This boundary permits concurrent requests without mixed revisions.
Memory retention requires explicit limits and cancellation policy.

**Acceptance evidence:** Concurrent edit traces, snapshot isolation, bounded retention, and deterministic repeated results.

## DEC-006: Persistent source index

The target persists source facts to reduce repeated process startup work.
Persistence belongs behind `IndexStore`.
The store publishes complete generations atomically.
Its records include content hashes and compatibility identities.

Cache loss remains recoverable from source inputs.
Initialization cannot depend on a complete cache load.
A corrupt record cannot enter a request snapshot.
Recovery must affect only Quicksilver-owned state.

**Acceptance evidence:** Cold recovery, schema mismatch, interrupted writes, disk exhaustion, and targeted reconstruction.

## DEC-007: Storage backend

SQLite is a persistence candidate.
Its WAL mode supports concurrent readers and one writer. [S5]
WAL also creates checkpoint and filesystem constraints.
The implementation must not assume that WAL works on every remote filesystem.

The candidate comparison must measure these conditions:

- Warm cache hydration.
- High-frequency small source updates.
- Long snapshot retention.
- Checkpoint interference with foreground latency.
- Crash recovery and corrupted records.
- Disk usage after repeated worktree changes.

An embedded store or content-addressed file layout remains an acceptable alternative.
The alternative must satisfy the same atomicity and recovery contract.
Backend selection must precede a stable on-disk schema commitment.

**Selection gate:** A recorded P2 comparison with source revisions and raw measurements.

## DEC-008: Worktree and project context isolation

A worktree owns its mutable source index and worker state.
A repository remote or module name does not identify that state.
Runtime selection, Mix environment, and Mix target define separate project contexts.
An umbrella context coordinates its child applications without duplicate compilation.

Shared dependency caches contain immutable compatible artifacts only.
The server must verify artifact identity before cross-context reuse.
Path dependencies remain explicit ownership boundaries.
Symlink resolution must preserve the client's URI and prevent discovery cycles.

**Acceptance evidence:** Concurrent worktrees, same-name modules, overlapping roots, umbrella projects, and path dependencies.

## DEC-009: Local BEAM worker

The optional worker uses local pipes and a versioned message contract.
The default design creates no TCP listener.
It also requires no Erlang distribution connection between Rust and the worker.
The worker uses approved project runtime executables.

The protocol isolates failures and retains source service after a worker crash.
The process boundary does not provide an operating-system sandbox.
Compile code can still access resources that its operating system permits.
Separate platform controls require their own enforcement evidence.

**Acceptance evidence:** Worker crash, invalid output, runtime mismatch, deadline expiry, and absence of a default listener.

## DEC-010: Project execution authority

Source-only mode does not execute `mix.exs`, macros, or formatter plugins.
Worker activation requires explicit project execution authority.
The authority applies to the actual workspace and selected runtime context.
An implementation must honor authority that the user already supplied.

Static runtime hints do not authorize arbitrary discovery commands.
The runtime selector does not source shell startup files automatically.
Execution uses an explicit argument vector.
The server does not build shell commands from project strings.

The worker must not call application entrypoints as a compile prerequisite.
This restriction does not make arbitrary compile-time code harmless.
Documentation must describe the actual execution boundary.

**Acceptance evidence:** A malicious test project remains inert in source-only mode.

## DEC-011: Dependency retrieval

Quicksilver does not retrieve dependencies automatically.
Missing Hex packages, tools, or runtime components produce an unavailable state.
Source service continues with its existing inputs.
The server supplies a precise reason without an interactive subprocess prompt.

Dependency installation remains an explicit developer operation.
A worker must not invoke `mix deps.get` as a hidden recovery action.
Authorized project code can still initiate network activity without a real sandbox.
The tool's own policy and the operating-system policy remain distinct.

**Acceptance evidence:** Missing dependency fixtures produce no Quicksilver-initiated download.

## DEC-012: Compile coalescence

One project context permits one active compilation.
Later edits replace one pending desired generation.
The coordinator does not preserve a compile job for every intermediate edit.
Completion publishes the actual compiled generation only after provenance checks.

Cancellation and process termination require bounded policies.
Worker restarts must not force source index reconstruction.
A changed toolchain or build context invalidates incompatible compiled output.
It does not erase independent source facts.

**Acceptance evidence:** Edit bursts, concurrent umbrella saves, compile cancellation, and latest-generation convergence.

## DEC-013: Compiled evidence and input consistency

Every compiled result retains an input manifest and verification method.
A source hash match alone does not prove compatibility with another runtime or compile environment.
The admission policy considers all recorded dependency axes.
Unknown inputs remain explicit limitations.

A before-and-after hash comparison detects ordinary file changes during compilation.
It does not prove an immutable or hermetic input snapshot.
Exact snapshot claims require controlled input materialization or equivalent verified read provenance.
The implementation must also account for path-sensitive compile behavior.

Generated metadata without sufficient source identity cannot authorize a source edit.
Stale documentation can appear with a clear origin label in supported textual results.
Stale source ranges cannot appear as current navigation targets.

**Acceptance evidence:** Source races, dependency changes, runtime changes, external resources, and obsolete worker results.

## DEC-014: Diagnostic producers and Dialyzer

Syntax, compiler, and Dialyzer diagnostics remain separate producers.
The aggregate diagnostic view records which producer generations it contains.
An obsolete producer cannot replace current diagnostics.
A failed compile cannot remove source navigation.

Dialyzer remains optional.
Its PLT cache uses a separate compatibility fingerprint.
Declared specs and documentation do not require a PLT.
Project analysis results and dependency PLTs require distinct invalidation rules. [S6]

**Acceptance evidence:** A disabled or blocked Dialyzer leaves source features available.

## DEC-015: Conservative semantic scope

The source analyzer implements supported Elixir rules explicitly.
Arbitrary macros remain outside a pure syntax guarantee. [S7]
The server records the smallest uncertain domain.
It preserves unrelated facts that still have valid evidence.

Rename starts with a local variable in one complete lexical domain.
It requires valid versions and a proof against capture.
Public symbols and cross-file edits require stronger coverage and application preconditions.
Unknown reflection or macro behavior blocks an unsafe edit.

**Acceptance evidence:** Identity fixtures, counterexamples, unsupported states, and unchanged semantics after accepted edits.

## DEC-016: Standard protocol shapes

The baseline wire contract is LSP 3.17.
Standard responses retain standard result types. [S8]
Optional Quicksilver methods supply status, provenance, and a source barrier.
Clients without those methods still receive supported standard behavior.

The bootstrap status shape remains explicitly separate from extension version 1.
No client can infer version 1 from the bootstrap method name alone.
Each extension feature requires negotiation and its own acceptance evidence.
Protocol conformance tests use an independent client.

**Acceptance evidence:** Capability negotiation, standard-schema validation, and extension version mismatch tests.

## DEC-017: External edits and client ownership

The LSP client owns text for open documents.
A disk write does not automatically change that text.
The OMP integration must provide valid document notifications or a deliberate disk-owned lifecycle.
The source barrier verifies expected hashes without compilation.

Watchers accelerate change discovery.
They do not prove the contents of a file.
Verified reads and hashes supply content evidence.
Watcher gaps invalidate relevant discovery coverage until reconciliation completes.

**Acceptance evidence:** Actual OMP edit sequences, atomic replacement, lost events, and overlay conflicts.

## DEC-018: Framework adapters

Framework semantics require versioned adapters with explicit scope.
An adapter can supply facts for a supported DSL version.
It cannot convert guesses into complete semantic evidence.
Unsupported package versions disable the affected claims.

Templates require correct host-to-template source maps.
Generated temporary locations must not leak into user edits.
Adapter tests require malformed input and unsupported constructs.
The source core remains usable without any adapter.

**Acceptance evidence:** Package-version fixtures, source maps, negative cases, and template edit races.

## DEC-019: Release evidence

Feature correctness, coverage, latency, and memory are separate gates.
An improvement in one measure does not waive another gate.
Performance targets are proposed requirements until actual measurements satisfy them.
The project must retain unsuccessful requests in its benchmark denominator.

The supplied scaffold does not complete the full vision.
Every advertised target capability needs protocol and semantic evidence.
The OMP release gate requires the actual client, process lifecycle, and file update sequence.
Mocked worker output cannot establish compiler compatibility.

**Acceptance evidence:** The complete P6 record in [QUALITY_GATES.md](QUALITY_GATES.md).

## DEC-020: Deferred products

These products do not form prerequisites for the core language server:

- A Debug Adapter Protocol server.
- A public remote language service.
- A general MCP gateway.
- A replacement Elixir compiler.
- A universal macro evaluator in Rust.
- A project application runtime manager.

The roadmap still includes the documented optional BEAM worker and essential LSP features.
This decision does not reduce that target scope to the bootstrap.
A later expansion requires a new decision and acceptance conditions.

## Decision update procedure

1. Identify the affected decision and requirement IDs.
2. Describe the observed problem with evidence.
3. Compare alternatives against the same acceptance conditions.
4. Record compatibility and migration effects.
5. Update the selected decision and dependent specifications.
6. Update the feature matrix and task ledger.

## Sources

- [S1] Rust LSP transport scaffold: <https://docs.rs/lsp-server/0.7.9/lsp_server/>
- [S2] Elixir Tree-sitter grammar: <https://github.com/elixir-lang/tree-sitter-elixir>
- [S3] Tree-sitter edits, previous trees, and concurrency: <https://tree-sitter.github.io/tree-sitter/using-parsers/3-advanced-parsing.html>
- [S4] Salsa incremental query model: <https://salsa-rs.github.io/salsa/overview.html>
- [S5] SQLite WAL behavior and limitations: <https://www.sqlite.org/wal.html>
- [S6] Erlang Dialyzer: <https://www.erlang.org/doc/apps/dialyzer/dialyzer.html>
- [S7] Elixir macro behavior: <https://elixir.hexdocs.pm/Macro.html>
- [S8] Language Server Protocol 3.17: <https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/>

External sources support factual context. The decision status records Quicksilver's chosen policy.
