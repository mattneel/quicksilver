# Feature matrix

## Interpretation

The scaffold is a narrow executable starting point.
It does not satisfy the complete Quicksilver objective.
The phase sequence ends at P6 after the semantic and release gates pass.

| Status | Meaning |
| --- | --- |
| Supplied | The scaffold contains code for the stated scope. Execution evidence remains separate. |
| Partial | A limited form exists. The target requirement remains incomplete. |
| Planned | The specification requires the feature. The scaffold contains no implementation. |
| Deferred | The capability remains outside the current supported scope. |

The handoff's validation record identifies the commands that actually ran.
A supplied feature does not imply a passing test or a measured latency result.

## Transport and document state

| Capability | Scaffold status | Exact current scope | Target phase |
| --- | --- | --- | --- |
| Executable entry point | Supplied | `quicksilver --stdio`, `--version`, and `--help` | P0 |
| JSON-RPC over stdio | Supplied | LSP framing and narrow request dispatch | P0, P1 |
| Initialize and shutdown | Supplied | Basic client lifecycle | P0, P1 |
| Position encoding | Supplied | UTF-16 default and negotiated UTF-8 | P0, P1 |
| Open and close documents | Supplied | In-memory Elixir document state | P0, P1 |
| Full document changes | Supplied | New complete text with a newer version | P0, P1 |
| Incremental ranged changes | Planned | No scaffold support | P1 |
| Disk and buffer reconciliation | Planned | No scaffold file reads | P1 |
| Workspace folders and worktrees | Planned | No project model | P1 |
| Cancellation under load | Planned | No demonstrated cooperative cancellation contract | P1, P6 |
| Request deadlines and fair scheduling | Planned | No hard latency guarantee | P1, P6 |
| Bootstrap status | Supplied | `quicksilver/status` reports syntax-only scope and no semantic readiness | P0 |
| Full status and provenance contract | Planned | No protocol version 1 or query provenance implementation | P5, QS-025 |
| Snapshot batch queries | Planned | No negotiated batch method | P5, QS-025 |
| External-source synchronization barrier | Planned | No explicit source barrier | P5, QS-025 |

## Source intelligence

| Capability | Scaffold status | Exact current scope | Target phase |
| --- | --- | --- | --- |
| Elixir syntax diagnostics | Supplied | Parser diagnostics for open Elixir documents | P0, P1 |
| Diagnostic versions | Supplied | Reports identify the open document version | P0, P1 |
| Diagnostic clear on close | Supplied | An empty report clears the closed document | P0, P1 |
| Document symbols | Supplied | Source declarations in an open document | P0, P2 |
| Hierarchical symbol output | Supplied | `DocumentSymbol` for capable clients | P0 |
| Flat symbol output | Supplied | `SymbolInformation` for other clients | P0 |
| Incremental syntax tree reuse | Planned | Full text synchronization is not incremental analysis | P2 |
| Persistent source index | Planned | No cache or project scan | P2 |
| Workspace symbols | Planned | No workspace search | P2 |
| Module and function identity | Partial | Display names exist. Semantic identity remains incomplete. | P2, P3 |
| Definitions across files | Planned | No semantic resolver | P3 |
| References across files | Planned | No semantic reference graph | P3 |
| Alias and import resolution | Planned | No lexical environment model | P3 |
| Arity and scope analysis | Planned | No semantic guarantees | P3 |
| Hover and documentation | Planned | No semantic hover handler | P3, P4 |
| Signature help | Planned | No argument or signature handler | P3, QS-014 |
| Behavior and protocol implementation lookup | Planned | No implementation graph | P3, P4 |
| Call hierarchy | Planned | No static call graph | P3, QS-015 |
| Incomplete or failed project compilation | Partial | Open syntax remains available. Project navigation does not exist yet. | P3, P4 |

## Optional BEAM work and advanced features

| Capability | Scaffold status | Exact current scope | Target phase |
| --- | --- | --- | --- |
| BEAM metadata | Planned | No BEAM file reader or worker | P4 |
| Compiler diagnostics | Planned | Syntax diagnostics only | P4 |
| Macro-generated definitions | Planned | No macro execution or semantic expansion | P4 |
| Dialyzer integration | Planned | Required implementation with optional runtime use. No current Dialyzer process. | P4, QS-023 |
| Compilation without source-query blockage | Planned | No compiler integration exists to exercise | P4, P6 |
| Completion | Planned | No completion handler | P5 |
| Semantic tokens | Planned | No semantic token handler or negotiated legend | P5, QS-020 |
| Rename and refactor actions | Planned | No write-producing LSP methods | P5 |
| Formatting | Planned | No formatter integration | P5 |
| HEEx and EEx | Planned | Required template support. Excluded from the initial OMP example's file types. | P5, QS-024 |
| Selection ranges | Planned | No structural selection handler | P5, QS-024 |
| Fold ranges | Planned | No fold range handler | P5, QS-024 |
| Type hierarchy | Deferred | Optional future scope beyond the required release gates | Future decision |
| Inlay hints | Deferred | Optional future scope beyond the required release gates | Future decision |
| Code lenses | Deferred | Optional future scope beyond the required release gates | Future decision |
| Full project diagnostics | Planned | No project-wide correctness result | P4, P6 |
| OMP end-to-end trace | Planned | Configuration example only | P6 |
| Reproducible benchmark history | Planned | Measurement scripts provide an initial interface | P6 |
| Release support matrix | Planned | No stable release or cross-platform guarantee | P6 |

## Capability advertisement

The scaffold advertises full text synchronization and document symbols.
Its experimental metadata identifies the bootstrap status method.
The complete extension requires QS-025 and protocol version 1.
It does not advertise semantic navigation or refactor support.
An unsupported request method returns JSON-RPC error `-32601`.
Future semantic limitations follow [PROTOCOL.md](PROTOCOL.md), including its explicit failure and provenance rules.

Arbitrary compile-time Elixir execution remains a semantic boundary.
The final product must describe unresolved cases without false certainty.
The supported subset must satisfy [ELIXIR_SEMANTICS.md](ELIXIR_SEMANTICS.md).

## Scaffold limits

The server reparses the complete document after each accepted full-text change.
Its main loop processes work serially.
It limits each document to 2 MiB and the open set to 256 documents.
The outline traversal has a depth limit of 256.
Each syntax report contains at most 100 parser diagnostics.

The outline omits these source forms:

- Declarations inside quoted code.
- Dynamic module names.
- Declarations inside `defimpl` bodies.
- Functions with operator names.

Function clauses remain separate outline entries.
Default arguments do not generate additional arities in the outline.
These limits define the supplied implementation, not the complete semantic target.

## Completion rule

P0 proves only that the kit and narrow scaffold are usable implementation inputs.
The project is complete only after all required P1 through P6 gates pass.
Each capability promotion requires its independent fixture evidence.
Each release claim requires the matching performance and OMP evidence.
An empty result, skipped test, or absent benchmark never promotes a status.
