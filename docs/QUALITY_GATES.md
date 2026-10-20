# Quality gates

## Acceptance rule

Quicksilver requires semantic correctness and bounded source-query latency.
Fast incorrect answers fail the product objective.
A transport scaffold alone fails the full completion gate.
The acceptance record must preserve both successful evidence and unresolved failures.

Every verification result must use one of four states:

| State | Meaning |
| --- | --- |
| PASS | The required assertion ran and passed. |
| FAIL | The assertion ran and failed. |
| UNAVAILABLE | A required dependency or environment was unavailable. |
| NOT_RUN | The assertion did not run. |

Only PASS satisfies a gate.
A timeout, empty log, or unsupported operation cannot become PASS.
A retry must preserve the initial failure and its cause.

## P0: Kit and scaffold audit

The kit must contain its required documents and executable source.
Its example JSON must parse and disable both default Elixir servers.
All local documentation links must resolve.
The lockfile and dependency declarations must support a reproducible build.
The scaffold's advertised capabilities must match its implemented handlers.

The protocol smoke test must cover these behaviors:

- Framed requests and fragmented input.
- Initialization followed by the initialized notification.
- UTF-16 default and negotiated UTF-8 positions.
- Full text changes with increasing document versions.
- Syntax errors followed by a valid correction.
- Document symbols with correct ranges.
- Empty diagnostic publication after document close.
- Unsupported methods and clean shutdown.

This gate contains no semantic navigation claim.
Its result must state the exact tested executable and source revision.

## P1: Protocol and workspace integrity

The document store must resolve buffer, save, and disk events under the contract in [PROTOCOL.md](PROTOCOL.md).
The server must reject invalid changes without corrupting its prior document state.
Stale or duplicate versions must not replace newer contents.
Closed documents must resolve through the current workspace snapshot when the target feature requires disk access.

The suite must include these cases:

- UTF-8 multibyte characters before query positions.
- UTF-16 surrogate pairs before query positions.
- CRLF and LF documents with identical semantic expectations.
- Concurrent open buffers and external file edits.
- Save notifications before and after disk watcher events.
- Delete, recreate, and rename events.
- Worktree paths with the same relative filename.
- Symlink identity under the documented workspace policy.
- Cancellation before dispatch, during analysis, and after completion.
- Request deadlines during a busy index or worker.
- Malformed frames and unsupported request methods.
- Resource limits for documents, queues, and responses.

Canceled requests must follow the protocol's completion rules.
Canceled work must release its query resources.
Late responses must not replace data for a newer request or source revision.

## P2: Incremental source index

The index must derive entries from a versioned workspace snapshot.
A body-only edit must avoid a full project rebuild when exported declarations remain unchanged.
A declaration edit must invalidate all affected dependents.
Instrumentation must record parsed files and invalidated queries for both cases.

The cache suite must cover these cases:

- Fresh cache creation and restart reuse.
- Content changes with unchanged file timestamps.
- Grammar or index schema version changes.
- Corrupt and incomplete cache entries.
- Interrupted writes and concurrent readers.
- Source deletion and dependency source changes.
- Worktrees with distinct content at the same relative path.

A cache miss must preserve source availability through a bounded fallback.
A corrupt cache must not produce a trusted semantic result.
The suite must demonstrate that unchanged dependency files avoid repeated parse work.

## P3: Independent semantic fixtures

The fixture manifest is the denominator for semantic accuracy.
The manifest must identify every case before the measured run.
It must contain these fields:

- Stable case identifier and feature family.
- Source files and content hashes.
- Query position and negotiated position encoding.
- Expected canonical symbol identity.
- Expected definition or reference locations.
- Supported or intentionally unsupported classification.
- Oracle source and reviewer record.
- Relevant Elixir and OTP versions, where applicable.

Expected results must come from independent review or a separate compiler-based oracle.
The tested resolver must never generate its own expected answers.
A copied snapshot of actual output is insufficient evidence without independent review.
An oracle comparison must preserve disagreements as review cases.

### Required semantic families

The fixture corpus must cover every supported rule in [ELIXIR_SEMANTICS.md](ELIXIR_SEMANTICS.md).
It must include these families:

- Nested modules and aliases with limited scopes.
- Imports with `only` and `except` restrictions.
- Function arity, default arguments, and multiple clauses.
- Local variables, shadowed variables, and pinned variables.
- Remote calls and local calls with identical names.
- Captures, pipelines, and operators.
- Private functions and public functions.
- Protocols, implementations, and behaviors.
- Quoted code and generated code boundaries.
- Incomplete files with recoverable syntax errors.
- Dynamic module or function expressions with explicit uncertainty.
- Ambiguous or unresolved symbols.

### Accuracy calculations

For definitions, `N_supported` is the number of eligible manifest cases for the stated supported subset.
`N_exact` is the number with the exact expected symbol and location.
Definition accuracy is `N_exact / N_supported`.
Unsupported cases remain visible in `N_unsupported` and never inflate accuracy.
A zero denominator reports `N/A` and fails a required feature gate.

For references, each case has an independently expected set `E` and returned set `R`.
True positives equal `R ∩ E`.
False positives equal `R - E`.
False negatives equal `E - R`.
The report must show micro precision, micro recall, and exact-set accuracy per semantic family.

Precision equals `TP / (TP + FP)`.
Recall equals `TP / (TP + FN)`.
Zero-denominator metrics report `N/A` with the relevant counts.
Negative cases require a separate false-positive count.
Intentional unsupported cases require the explicit outcome from the protocol contract.

The deterministic supported suite requires 100 percent exact definition and reference results.
This threshold applies to the declared fixture subset, not all possible Elixir programs.
A sampled real-project corpus must report its own counts and limitations.
No aggregate score can hide a failed required semantic family.

## P4: Optional BEAM enrichment

Source queries must remain available without Elixir, OTP, network access, or a valid project build.
The worker must execute only under the configured project policy.
Its metadata must identify the source and toolchain state that produced it.
Expired metadata must never override newer source facts.

The failure suite must cover these states:

- Missing dependencies and unavailable network access.
- A compiler error in an unrelated file.
- A failed or terminated worker.
- A changed Elixir or OTP executable.
- A dependency lockfile change.
- Stale BEAM files after a source edit.
- A long compiler task during repeated source queries.
- Worker output after a newer workspace revision.

A full compile or PLT operation must never block a source-only request.
The trace must identify every process launch and compiler invocation.
An unexpected invocation after an ordinary source edit fails this gate.
The process tree must return to its baseline after worker shutdown.

QS-023 requires the Dialyzer integration, with runtime use disabled by default.
An explicit configured job controls each analysis run.
PLT identity must preserve the runtime and dependency fingerprints.
Canceled or failed jobs must preserve source-query availability and honest job status.

## P5: Completion, refactors, and templates

Every write-producing action must use a versioned edit with a reviewable scope.
Rename tests must include unresolved references and stale document versions.
An incomplete reference set must prevent an authoritative whole-project rename.
The suite must verify that unrelated text remains unchanged.

Template support requires independent source-map tests for each supported file type.
The OMP file registration must expand only after the corresponding support gate passes.
Formatter tests must distinguish formatting failure from a valid unchanged result.
Completion tests must verify the documented incomplete-result behavior.

Semantic token tests must verify the negotiated legend, valid ranges, and one coherent document revision.
Selection ranges must contain their requested positions and form valid parent chains.
Fold ranges must respect source boundaries, client limits, and malformed-edit recovery.
These structural gates require UTF-8, UTF-16, CRLF, empty files, and nested constructs.

QS-025 requires the complete opt-in version 1 extension.
Status, query provenance, batch queries, and source synchronization require independent acceptance evidence.
Batches must use one coherent snapshot and bounded output.
The source barrier must refuse an external replacement that conflicts with an open buffer.
An unsupported custom version must preserve standard LSP operation.

Type hierarchy, inlay hints, and code lenses remain optional future scope.
They do not replace any required P5 gate.

## P6: OMP, performance, and release

The final OMP trace must identify its installed OMP version and effective server configuration.
It must use a prebuilt Quicksilver executable.
It must demonstrate source queries before optional compiler work completes.
The trace must include both successful results and explicit unavailable results.

The release must satisfy [PERFORMANCE.md](PERFORMANCE.md) on the declared reference environments.
Repeated edits must preserve semantic correctness and stay within the accepted latency budget.
Memory and process counts must settle after sustained workload.
All supported platform binaries must pass the same protocol and semantic gates.

## Verification procedure

1. Run the kit validator.

```bash
python3 scripts/validate_kit.py
```

2. Run the Rust test suite.

```bash
cargo test --workspace --locked
```

3. Build the release executable.

```bash
cargo build --release --locked
```

4. Run the protocol smoke test.

```bash
python3 scripts/lsp_smoke.py --server target/release/quicksilver
```

5. Run the initial latency probe.

```bash
python3 scripts/benchmark.py --server target/release/quicksilver --iterations 1000
```

6. Record each command's result and environment.
7. Run the phase-specific suites that the implementation adds.
8. Update the feature matrix with the corresponding evidence.

## Scope of the supplied scripts

The supplied scripts establish a kit audit and a narrow executable probe.
They do not replace the future semantic, worker, or OMP suites.
Their output must name the tested feature and corpus.
No script can declare the full vision complete from initialization or syntax-only success.
