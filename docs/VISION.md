# Quicksilver product vision

## Problem

An agent edits several files and immediately requests code intelligence.
An expensive project analysis can delay every subsequent decision.
Repeated startup or broad invalidation can multiply that delay.
Quicksilver makes source intelligence available independently of those expensive operations.

The target user uses Oh-My-Pi with Elixir projects.
Ordinary editor clients remain first-class protocol consumers.
The server uses standard LSP wherever the protocol can express the required behavior.
Optional extensions expose analysis state that standard responses cannot represent.

## Outcome

A new checkout supplies useful document intelligence before the project compiles.
A warm workspace supplies current navigation after a small edit.
A failed compile leaves source declarations and supported reference queries available.
A background worker can supply additional compiler facts without control over those paths.

The implementation never treats speed as permission for incorrect semantic claims.
It distinguishes supported exact results from unsupported queries.
It preserves the difference between a complete empty result and unavailable analysis.

## Product modes

| Mode | Required tools | Available behavior | Prohibited dependency |
| --- | --- | --- | --- |
| Source | Quicksilver binary | Syntax, source symbols, supported resolution, references, and source refactors | Mix, Erlang, successful compilation, or a PLT |
| Enriched | Quicksilver plus a supported Elixir and OTP pair | Source features plus compiler diagnostics and metadata | Worker completion before a source response |
| Explicit deep analysis | Enriched mode plus configured analysis tools | Optional Dialyzer results and requested deeper analysis | Automatic deep analysis after every edit |

Source mode is the initial default.
Project execution requires an explicit client or project setting.
Tests can enable enriched mode for controlled fixture projects.
An OS process boundary protects availability but does not provide an OS sandbox.

## Required user journeys

### First source query

The agent starts in an Elixir project without build artifacts.
It requests document symbols and a directly resolvable definition.
Quicksilver parses the required files and returns supported results.
Dependency compilation does not run as a prerequisite.
The status response discloses any incomplete workspace index.

### Repeated edits

The agent changes a function body repeatedly.
Quicksilver updates the document overlay and affected query state.
Unchanged module declarations retain their identity.
Obsolete work receives cancellation before its result replaces current state.
Navigation uses a coherent snapshot after each accepted edit.

### External file writes

The agent changes a file through a filesystem tool.
The client or server watcher reports the external change.
Quicksilver reconciles the disk version and invalidates affected state.
An open document overlay retains precedence until the client updates or closes it.
The race has a deterministic, documented outcome.

### Compile failure

A project has a syntax error or a missing dependency.
Supported source features remain available for unaffected files.
The worker reports its failed state without a restart of the source service.
Compiler diagnostics preserve their source revision.
A stale compiler result never replaces current syntax diagnostics.

### Worktree switch

Two worktrees contain different dependency versions and source text.
Each worktree has independent document overlays and analysis state.
Persistent cache keys include the required project and toolchain identities.
No result crosses the worktree boundary.
The implementation verifies symlinks and case behavior on each supported platform.

### Refactor

The agent requests a supported rename with preview.
Quicksilver resolves every target in the claimed scope.
It returns edits only when the evidence is complete for that scope.
Open-document edits carry the source document version.
Closed-file edits require a verified client precondition contract or a strict refusal.
Acceptance tests verify rejection when source content changes before application.
Known dynamic or macro ambiguity produces a specific refusal.

## Full feature scope

| Feature group | Required release behavior |
| --- | --- |
| Lifecycle | Initialization, shutdown, exit, configuration, cancellation, and truthful capabilities |
| Documents | Incremental and full synchronization, open overlays, external writes, and negotiated position encoding |
| Workspaces | Mix roots, umbrellas, workspace folders, plain source folders, and worktree isolation |
| Syntax | Error-tolerant trees, exact ranges, document symbols, and syntax diagnostics |
| Source index | Persistent declaration index with incremental file updates and explicit readiness |
| Resolution | Modules, aliases, imports, requires, scopes, function arity, captures, defaults, and supported delegates |
| Navigation | Definitions, references, workspace symbols, implementation lookup, and supported call hierarchy |
| Information | Hover documentation, typespec facts, signatures, and source context |
| Completion | Contextual names, imports, aliases, members, callbacks, and explicit incomplete state |
| Structure | Semantic tokens, selection ranges, and folding ranges with exact encoded positions |
| Refactors | Safe local rename, supported project rename, code actions, and formatter integration |
| Enrichment | Runtime discovery, compile diagnostics, BEAM metadata, generated exports, and independent recovery |
| Templates | Explicit EEx and HEEx support with correct embedded Elixir positions |
| Agent support | Bounded waits, fresh responses, batch-friendly queries, status, and OMP integration |
| Operations | Cache diagnostics, worker logs, reproducible packages, and crash recovery |

The semantic specification defines supported resolution boundaries.
The release does not promise static resolution of arbitrary runtime module values.
It does not promise complete expansion of arbitrary macros without project execution.
These limits must not remove ordinary exact cases from the acceptance denominator.

## Performance contract

Performance budgets are acceptance targets until measured.
The kit's scaffold benchmark is only a baseline for a narrow source feature.
The production corpus and semantic workload determine release evidence.

The target service measures these independent outcomes:

- Cold startup to the first useful source result.
- Warm document edit to fresh source symbols.
- Warm definition and reference latency.
- Tail latency during worker compilation.
- Repeated-edit invalidation volume.
- Memory use and process recovery over a sustained session.

The exact budgets and method appear in `PERFORMANCE.md`.
An average alone cannot establish the latency contract.
Failed requests and timeouts remain visible in every report.

## Non-goals

- A new Elixir compiler or an alternate BEAM runtime.
- A universal static answer for arbitrary runtime behavior.
- A requirement to run Dialyzer during normal source navigation.
- A dependency on a specific editor or agent vendor.
- A cloud service for local source queries.
- A public extension marketplace release before verification.
- A rewrite of the user's application.

## Delivery contract

The project is complete when every required task and release gate has passing evidence.
The release includes a working binary and reproducible source package.
It includes a tested OMP configuration that selects Quicksilver alone for Elixir.
It includes a stated support matrix and a complete list of deliberate limitations.
It includes benchmark results that identify the tested corpus and machine.

The task ledger is executable project state, not an estimate.
Dates and throughput claims require observations.
Unverified platforms remain outside the support claim.
The owner controls public distribution.

## Success measures

| Measure | Required interpretation |
| --- | --- |
| Source availability | No compile or PLT dependency on supported source requests |
| Correctness | Exact results for the documented supported fixture set |
| Honesty | Explicit refusal or partial status for known unsupported cases |
| Freshness | No obsolete result that replaces a newer accepted document revision |
| Isolation | No worktree, runtime, or dependency contamination |
| Responsiveness | Passing percentile budgets under the documented workload |
| Recovery | Source service survives worker failure and recovers cache state |
| Operability | A developer can install, inspect, recover, and remove the server |

