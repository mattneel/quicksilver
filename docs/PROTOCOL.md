# Protocol contracts

## Status and conformance boundary

Quicksilver uses LSP 3.17 as its baseline protocol contract. [S1]
Additional protocol versions require explicit compatibility tests.
This document separates the supplied bootstrap from target behavior.
Target requirements do not describe features that the bootstrap already implements.

Standard LSP responses must retain standard LSP shapes.
Custom metadata belongs in negotiated extensions or fields that explicitly accept opaque data.
An opaque field does not authorize a different result type.

## 1. Supplied bootstrap

The bootstrap supports local standard input and output.
It owns open Elixir documents and does not read project files.
It advertises full text synchronization and document symbols.
It publishes syntax diagnostics after accepted text updates.
It clears those diagnostics after document close.

The supplied capability subset is:

```json
{
  "positionEncoding": "utf-16",
  "textDocumentSync": {
    "openClose": true,
    "change": 1
  },
  "documentSymbolProvider": true,
  "experimental": {
    "quicksilver": {
      "stage": "bootstrap",
      "diagnosticCoverage": "syntax-only",
      "projectExecution": false,
      "statusMethod": "quicksilver/status"
    }
  }
}
```

The encoding value can be `utf-8` after client negotiation.
The bootstrap selects the first supported encoding from the client's list.
It falls back to `utf-16` when no supported preference exists.

The bootstrap also supplies `quicksilver/status`:

```json
{
  "stage": "bootstrap",
  "openDocuments": 0,
  "positionEncoding": "utf-16",
  "diagnosticCoverage": "syntax-only",
  "semanticReady": false,
  "projectExecution": false
}
```

This bootstrap extension has no `protocolVersion` field.
It is not the target version 1 contract below.
The bootstrap has no semantic index, worker, persistence, provenance query, or source barrier.
Its synchronous loop does not establish the target concurrency or deadline guarantees.

| Bootstrap resource | Supplied limit |
|---|---:|
| Frame payload | 16 MiB |
| Frame headers | 8 KiB |
| One open document | 2 MiB |
| Open documents | 256 |

The code and verification report define the exact observed bootstrap behavior.
A future change must update this section and [FEATURE_MATRIX.md](FEATURE_MATRIX.md) together.

## 2. Transport and lifecycle

### 2.1 Framing

LSP frames use `Content-Length` and UTF-8 JSON payloads. [S1]
The length counts payload bytes.
The header terminator is `\r\n\r\n`.
Logs must use standard error or an explicit log sink.

PRO-001 requires one output owner for complete frames.
Concurrent tasks cannot interleave header and payload bytes.
The parser must accept fragmented headers and payloads.
It must also accept several complete frames within one read.

PRO-002 requires documented header and payload limits.
The parser must reject overflow, duplicate conflicting lengths, invalid UTF-8, and oversized frames.
An invalid frame cannot cause an unbounded allocation.
The implementation must define whether recovery remains possible after each framing error.

### 2.2 Lifecycle states

| State | Accepted transition | Request behavior |
|---|---|---|
| Before initialization | `initialize` | Other ordinary requests receive `ServerNotInitialized`. |
| Initialization accepted | `initialized` | The server does not start normal feature dispatch early. |
| Ready | Feature requests and document notifications | Negotiated capabilities apply. |
| Shutdown accepted | `exit` | New requests receive `InvalidRequest`. |
| Exited | None | No protocol output remains. |

`shutdown` returns a successful `null` result.
The process exits after `exit`.
An exit after orderly shutdown uses exit code zero.
An exit without orderly shutdown uses a nonzero code. [S1]

PRO-003 requires exactly one response for every accepted request.
Notifications receive no response.
Unsupported requests receive `MethodNotFound`.
Unknown notifications receive no protocol error response.
Duplicate initialization receives a controlled error.

The target must stop worker processes and release owned resources during shutdown.
It must not wait indefinitely for a blocked worker.
Forced worker termination does not authorize deletion of project files.

## 3. Capability negotiation

PRO-004 forbids an advertised capability without its acceptance evidence.
Capabilities describe the methods that the server supports.
They do not assert that every Elixir construct supports those methods.
Unsupported semantic domains retain explicit per-request behavior.

| Capability family | Target gate | Required client compatibility |
|---|---|---|
| Full synchronization | P0 and P1 | Valid open, change, and close sequence |
| Incremental synchronization | P1 | Range conversion and ordered edit batches |
| Document symbols | P0 through P3 | Hierarchical or flat result form |
| Workspace symbols | P2 and P3 | Optional resolve support |
| Definition, declaration, and implementation | P3 | `Location` or negotiated `LocationLink` form |
| References and highlights | P3 | Complete supported identity domain |
| Hover and signature help | P3 through P5 | Supported markup and signature formats |
| Completion and resolve | P5 | Snippets, insert modes, and resolve properties |
| Diagnostics | P0 through P4 | Push baseline or negotiated pull provider |
| Semantic tokens | P5 | Negotiated legend and full, range, or delta mode |
| Rename and prepare | P5 | Complete semantic scope and edit preconditions |
| Code actions and resolve | P5 | Supported action kinds and resolve properties |
| Formatting | P4 and P5 | Explicit runtime execution policy where required |
| Call hierarchy | P3 through P5 | Complete supported graph domain |
| Type hierarchy | Optional future scope | Complete supported type relation domain |
| Inlay hints and code lenses | Optional future scope | Resolve behavior and evidence origin |
| File operations | P1 and P5 | File filters and workspace edit capabilities |
| Selection and fold ranges | P2 and P5 | Correct nested source ranges |

Dynamic registration requires the corresponding client capability.
The server must not send registration requests to an incompatible client.
Unsupported optional client properties require a tested fallback.
The server must not infer capabilities from an editor name.

## 4. Documents and coordinates

### 4.1 Position encoding

The target supports UTF-8 and UTF-16 coordinates first.
UTF-16 remains the baseline fallback.
An additional encoding requires dedicated tests.
Internal source spans use byte offsets into the captured document. [S2]

PRO-005 requires correct conversion around non-ASCII text.
The test corpus must include supplementary Unicode characters and combining marks.
CRLF input must retain correct line boundaries.
A malformed range cannot produce a panic or an edit inside an encoded character.
The coordinate layer must follow LSP rules for positions beyond a line's text.

### 4.2 Full and incremental updates

The bootstrap requires full content updates.
It rejects ranged updates.
The target advertises incremental synchronization only after the P1 gate.

PRO-006 requires sequential application of each `contentChanges` entry.
Every range refers to the state after the preceding entry.
The server validates the complete batch before atomic publication.
An invalid entry rejects the batch without a partial document mutation.
An entry without `range` replaces the complete text.

The document version must increase after an accepted change.
The internal source generation remains separate from the client version.
An obsolete version must not replace newer text.
The server records rejected updates without a notification response.

The target marks a document desynchronized after an unrecoverable edit mismatch.
Dependent requests then receive an explicit error.
A valid full replacement or close-and-open sequence can restore synchronization.
The server must not invent a document version.

`didSave` does not implicitly replace a newer client overlay.
Its optional text requires a consistency check.
A save can request background compilation only under the authorized worker policy.
No save handler can wait for compilation before source service continues.

### 4.3 External file events

PRO-007 requires consistent handling of external writes and atomic replacement.
File events trigger verified content reads for closed files.
An event for an open file must not overwrite the client overlay.
Watcher overflow or failure invalidates the affected discovery coverage.
Bounded reconciliation restores that coverage.

Workspace folder changes create or retire the relevant project contexts.
File rename events update path identity and index tombstones atomically.
The server must not treat `willRenameFiles` as permission to move files itself.
It returns edits through the negotiated client contract.

## 5. Request execution

### 5.1 Snapshot order

PRO-008 requires a request snapshot after all earlier document notifications enter the coordinator.
The request keeps that snapshot throughout its analysis.
Responses for independent requests can complete in a different order.
Result correctness must not depend on response order.

A later normal edit does not automatically invalidate an earlier read-only response.
The response can still describe its coherent captured snapshot.
The client can cancel obsolete work.
The server must not use `ContentModified` for every ordinary edit. [S1]

### 5.2 Deadlines and cancellation

Each request receives a deadline at admission.
The budget includes queue delay.
The scheduler must preserve enough capacity for lifecycle and cancellation messages.
Long tasks must poll cancellation at bounded work intervals.

| Condition | Error code | Required meaning |
|---|---:|---|
| Invalid JSON | `-32700` | `ParseError` |
| Invalid request envelope | `-32600` | `InvalidRequest` |
| Unknown request method | `-32601` | `MethodNotFound` |
| Invalid method parameters | `-32602` | `InvalidParams` |
| Unexpected internal failure | `-32603` | `InternalError` |
| Request before initialization | `-32002` | `ServerNotInitialized` |
| Client cancellation observed | `-32800` | `RequestCancelled` |
| Exceptional content invalidation | `-32801` | `ContentModified` |
| Permitted server cancellation | `-32802` | `ServerCancelled` |
| Deadline, unsupported scope, or unavailable required evidence | `-32803` | `RequestFailed` |

`ServerCancelled` applies only to methods that permit server cancellation. [S1]
It is not a generic timeout code.
An ordinary expired budget uses `RequestFailed` with a precise message.
Client cancellation does not require the server to pretend that the request succeeded.

PRO-009 requires one terminal response after an observed cancellation.
A race with normal completion still produces only one response.
Cancellation must reach cache, index, and worker waits that belong to that request.
Cancellation of one request does not cancel unrelated background work.

### 5.3 Empty, unknown, and partial results

PRO-010 forbids an empty success result that conceals a known unsupported semantic scope.
A complete supported query can return an empty result.
An unresolved definition can return the method's normal no-result form.
It must not claim a proved absence of definitions.

| Feature | Correct incomplete behavior |
|---|---|
| Completion | `CompletionList.isIncomplete` can signal an incomplete list. |
| References | A known incomplete required domain receives `RequestFailed`. |
| Rename | Incomplete proof receives an atomic refusal. |
| Hover | Text can state the origin or age of supplementary metadata. |
| Definition | Only verified current target ranges enter normal locations. |
| Workspace symbols | The declared search coverage governs the result claim. |
| Optional status | The extension carries detailed coverage and provenance. |

The server must not add `partial`, `stale`, or `confidence` to a standard `Location`.
It must not add `isIncomplete` to reference arrays.
It must not place an error message inside a fabricated source location.

## 6. Progress and partial results

Work progress and partial result progress are different mechanisms. [S1]
Work progress describes task state.
Partial result progress transfers part of a method's result.
Each requires its own valid token and method support.

PRO-011 permits partial results only when the request supplies a `partialResultToken`.
The method must also define a partial result type.
Each chunk must match that type.
The final response must follow that method's terminal result contract.
It must not repeat previously emitted result items.

Work progress uses a client-provided token or a successfully created server token.
Server token creation requires the corresponding client support.
Every begun progress operation must end on success, error, or cancellation.
A progress percentage must describe a measurable bounded task.
Unknown total work uses indeterminate progress.

The server must not label a partial semantic graph as complete after its final chunk.
A progress token does not change semantic coverage requirements.
Clients without progress support receive the normal bounded request behavior.

## 7. Diagnostics

### 7.1 Producer model

The target diagnostic store separates syntax, compiler, and optional Dialyzer producers.
Each producer supplies a source identity and input generation.
The aggregate view includes only records valid for its captured source snapshot.
One producer cannot erase another producer's valid diagnostics accidentally.

PRO-012 forbids publication of obsolete compiler diagnostics as current results.
An edit invalidates affected compiler records before new syntax diagnostics appear.
An unavailable worker does not prevent current syntax diagnostics.
A failed compile does not remove source definitions.

A diagnostic uses a real source range when precise evidence exists.
The server must not attach a range-less worker failure to line zero of an arbitrary file.
That failure belongs in project status or an appropriate log notification.
Diagnostic messages identify the actual producer.

### 7.2 Push diagnostics

The bootstrap uses `textDocument/publishDiagnostics`.
The target retains push diagnostics for compatible clients without pull support.
An empty diagnostic array clears the previously published set for that URI.
Document versions accompany applicable client-owned diagnostics when the client supports them. [S1]

The target maintains one coherent aggregate set per URI.
Out-of-order producer completion cannot replace a newer aggregate generation.
Close behavior depends on whether project diagnostics remain active for closed files.
That policy requires explicit tests and feature documentation.

### 7.3 Pull diagnostics

Pull diagnostics remain a P4 target capability.
The provider must support `textDocument/diagnostic` before it advertises `diagnosticProvider`.
Workspace diagnostics require a separate gate.
Refresh requests require client refresh support.

PRO-013 requires opaque result IDs that identify the diagnostic view and its validity inputs.
`kind: "unchanged"` requires a matching valid `previousResultId`.
A cache miss, worker wait, or unchanged source version does not establish unchanged diagnostics.
A changed producer generation can change the result without a source edit.

Workspace reports must preserve URI and version semantics.
Related-document reports require the relevant client support.
Partial workspace results must use the method's defined partial payload.
Cancellation must retain the required terminal response.

A client receives one primary diagnostic transport for the same producer set.
The server must avoid duplicate push and pull reports for identical diagnostics.
Producer unavailability appears through status and explicit coverage.
It must not become a false complete clean result.

## 8. Completion, tokens, and edits

### 8.1 Completion and resolve

Completion items must honor the client's snippet and insert capabilities.
Unsupported snippets require plain-text alternatives.
Deferred properties require negotiated resolve support.
Resolve data must retain enough source identity to reject obsolete edits.
It must not contain secret environment values.

`isIncomplete` describes the completion list only.
A resolve request cannot change the symbol identity silently.
Additional edits require the same current-source and overlap checks as primary edits.

### 8.2 Semantic tokens

The token provider must negotiate one stable legend per session.
Tokens must use the selected encoding and required relative positions.
Full, range, and delta modes require separate tests.
An unknown delta base must produce a full result where the method permits it.
It must not apply a delta to a different document generation.

### 8.3 Rename and workspace edits

PRO-014 requires `prepareRename` and final rename to verify semantic scope independently.
A preparation response does not reserve document contents.
The final response must pass all SEM-018 conditions.

Versioned `TextDocumentEdit` entries are the preferred edit form.
Open documents require their exact captured client version.
The server must verify that the client supports `documentChanges` before that form appears.
An unversioned edit cannot provide an equivalent concurrency guarantee.

The first rename gate covers one open document and a proven local variable domain.
Closed-file or cross-file rename requires a stronger client application contract.
LSP alone provides no content-hash precondition for a closed document's bytes.
That limitation must remain explicit in the supported feature scope.

The default edit policy is strict.
Strict mode refuses closed-file and cross-file rename without an explicit content-hash application contract.
That contract must verify every expected hash at the client mutation boundary.
It must reject the entire edit if any precondition fails.
A server-side hash check before the response does not prevent a later client-side race.
Independent per-file checks do not establish atomic multi-file edits.
The client contract must state its transaction boundary and failure recovery limits.

PRO-015 requires deterministic, nonoverlapping edits with valid source ranges.
An unsupported location invalidates the complete proposed rename.
Resource operations require negotiated support and annotations where applicable.
The server never applies returned edits directly to disk.

Code actions with workspace edits require the applicable workspace edit preconditions.

Formatting returns the standard `TextEdit[]` response for the captured document.
Standard formatting responses carry no document version or content-hash precondition.
The server verifies its current overlay before the response, but that check does not prevent later client edits.
Strict automated format application requires a client-side expected-content check at the mutation boundary.
Formatter execution can invoke project plugins.
The worker execution policy therefore applies when a formatter needs project code.
The server must not run `mix format` automatically in source-only mode.

## 9. Target Quicksilver extension version 1

### 9.1 Negotiation

This extension is optional and target-only.
It is separate from the bootstrap status shape.
Clients can use all supported standard features without this extension.

The client requests features inside `InitializeParams.capabilities.experimental`:

```json
{
  "quicksilver": {
    "protocolVersion": 1,
    "status": true,
    "queryProvenance": true,
    "synchronize": true,
    "batch": true
  }
}
```

The server echoes the accepted subset inside `ServerCapabilities.experimental`.
It must not echo a feature that lacks implementation evidence.
A mismatched version leaves the extension disabled.
The server must not send unsolicited custom notifications to an unnegotiated client.

### 9.2 `quicksilver/status`

Parameters are an empty object.
The response describes current service state without a compile wait.
It contains no source text or environment secrets.

```json
{
  "protocolVersion": 1,
  "sessionId": "session-opaque",
  "sourceSnapshotId": "snapshot-opaque",
  "openDocuments": 4,
  "positionEncoding": "utf-16",
  "projects": [
    {
      "projectContextId": "context-opaque",
      "sourceState": "partial",
      "discoveryComplete": false,
      "workerState": "disabled",
      "desiredCompileGeneration": null,
      "compiledGeneration": null
    }
  ]
}
```

`sourceState` uses `complete`, `partial`, or `unavailable`.
Its domain is the project's declared source discovery scope.
It does not assert complete semantics for arbitrary macros.

`workerState` uses these values:

- `disabled`
- `unauthorized`
- `runtime_unavailable`
- `runtime_unsupported`
- `dependencies_unavailable`
- `ready`
- `compiling`
- `failed`

Additional optional version 1 fields must preserve all existing field meanings.
An incompatible meaning requires another protocol version.

### 9.3 `quicksilver/queryProvenance`

The request identifies the latest completed occurrence of an LSP request ID on the same connection.
Integer and string IDs remain distinct.
The client must query before it reuses that ID.
The server retains a bounded recent history.

```json
{
  "requestId": 42
}
```

An available record uses this shape:

```json
{
  "protocolVersion": 1,
  "state": "available",
  "requestId": 42,
  "requestSequence": 87,
  "method": "textDocument/definition",
  "sourceSnapshotId": "snapshot-opaque",
  "coverage": {
    "state": "complete",
    "domain": "resolved source identity within project context"
  },
  "origins": ["source_resolved"],
  "compiledGenerations": []
}
```

The record describes evidence at response completion.
Later edits do not change the retained record.
An evicted or unknown request returns `state: "unavailable"` with a reason.
An active request returns `state: "pending"`.
The server must not substitute provenance from another occurrence or connection.

### 9.4 `quicksilver/synchronize`

This request provides a source barrier for known external edits.
It never requests compilation.
Each URI must belong to an approved source domain.
The expected hash uses SHA-256 over the complete UTF-8 document bytes.

```json
{
  "documents": [
    {
      "uri": "file:///workspace/project/lib/example.ex",
      "sha256": "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
    }
  ]
}
```

The successful response identifies one snapshot with all requested hashes:

```json
{
  "protocolVersion": 1,
  "sourceSnapshotId": "snapshot-opaque",
  "documents": 1
}
```

A hash mismatch returns `RequestFailed` with a conflict reason.
An open overlay conflict does not authorize disk text to replace client text.
The method follows normal deadlines and cancellation.
It cannot promise complete workspace discovery from a partial document list.

### 9.5 `quicksilver/batch`

This optional method combines read-only source queries under one snapshot.
It does not define a new standard LSP method.
The request contains at most 64 items with unique string IDs.
Each item contains a supported method name and that method's ordinary parameters.

```json
{
  "items": [
    {
      "id": "outline",
      "method": "textDocument/documentSymbol",
      "params": {
        "textDocument": {
          "uri": "file:///workspace/project/lib/example.ex"
        }
      }
    }
  ]
}
```

Version 1 permits these methods when the server advertises their standard capability:

- `textDocument/documentSymbol`
- `workspace/symbol`
- `textDocument/definition`
- `textDocument/declaration`
- `textDocument/references`
- `textDocument/documentHighlight`
- `textDocument/hover`
- `textDocument/signatureHelp`

The coordinator reconciles known target inputs before one snapshot capture.
Every item uses that same snapshot.
The batch cannot execute edits, compile commands, or another batch.
Per-item progress tokens are unsupported in version 1.

The response contains `protocolVersion`, `sourceSnapshotId`, and one entry per input item.
Each entry has its original ID and exactly one `result` or `error` field.
The `result` uses the standard method's result type without modification.
An optional sibling `provenance` field uses the version 1 evidence vocabulary.

The parent request has one deadline and cancellation token.
Each item also retains its normal bounded query budget.
An expired item returns an explicit error entry.
Items without remaining budget receive explicit deadline errors.
Client cancellation terminates the batch with `RequestCancelled`.

The server must not omit failed or unattempted items from a successful batch envelope.
It must enforce the configured aggregate output limit before final serialization.
A batch result must never combine several source generations to finish faster.

## 10. Internal BEAM worker protocol version 1

### 10.1 Boundary and framing

This is a private worker protocol, not an LSP extension.
The bootstrap does not implement it.
The Rust host starts an approved local executable with an explicit argument vector.
The default transport uses child standard input and output.
No default remote listener exists.

Frames use the same byte-counted header structure as the LSP transport.
Bodies use JSON-RPC 2.0 envelopes with worker-specific methods.
Every payload has bounded depth, string size, and collection size.
The host treats worker payloads as untrusted input.

PRO-016 reserves worker standard output for protocol frames.
Project subprocess output requires separate capture.
Project logs must not enter the control stream.
If project code can bypass normal output redirection, the host must isolate that execution in another process.
Protocol corruption terminates the worker session without termination of source service.

### 10.2 Handshake

The host sends `worker/initialize` before any project operation.
The request includes these fields:

| Field | Requirement |
|---|---|
| `protocolVersion` | Integer `1` |
| `workerBuildId` | Host-expected worker artifact identity |
| `projectContextId` | Context identity from the Rust host |
| `worktreeId` | Immutable ownership identity |
| `projectRoot` | Approved absolute root |
| `buildRoot` | Dedicated Quicksilver-owned output root |
| `mixEnv` | Explicit selected Mix environment |
| `mixTarget` | Explicit selected Mix target |
| `allowProjectExecution` | Explicit authorization flag |
| `allowDependencyFetch` | Must be `false` in version 1 |

The worker response includes its protocol, build, Elixir, OTP, architecture, and supported operation identities.
The host verifies those values before admission.
A mismatch disables enrichment and preserves source service.
The host must not attempt an automatic runtime installation.

Environment values travel through the approved process environment.
They must not appear in ordinary logs or status results.
The compile context fingerprint represents relevant values without disclosure.
The implementation must document which values affect that fingerprint.

### 10.3 Methods

| Method | Direction | Contract |
|---|---|---|
| `worker/initialize` | Host to worker | Negotiates protocol and verifies runtime context |
| `worker/compile` | Host to worker | Runs one authorized background compile generation |
| `worker/readMetadata` | Host to worker | Reads compatible artifact metadata without application startup |
| `worker/format` | Host to worker | Formats supplied text under explicit formatter policy |
| `worker/shutdown` | Host to worker | Ends the session and returns `null` |
| `$/cancelRequest` | Host to worker | Cancels a request where possible |
| `worker/progress` | Worker to host | Reports bounded generation progress |
| `worker/log` | Worker to host | Supplies bounded sanitized logs |

An optional Dialyzer operation requires a later explicit capability.
Version 1 does not require it for any other operation.
The host must not send an unsupported worker method.

### 10.4 Compile request and result

`worker/compile` carries one desired compile generation.
Its request includes these inputs:

- Project context and generation IDs.
- Source input manifest identity.
- Dependency and lockfile fingerprint.
- Compiler options fingerprint.
- Explicit compile resource identities.
- Request deadline and resource budget.

The worker accepts at most one active compile per context.
The Rust coordinator coalesces later desired generations.
The worker must not fetch dependencies or start the application as an implicit prerequisite.
Missing inputs produce a structured unavailable state.

The terminal result uses `complete`, `compile_failed`, `cancelled`, or `inputs_changed`.
Each result retains the actual generation ID.
It also includes an observed input manifest and its verification method.
The host never relabels an older result as the newest desired generation.

A complete result supplies bounded metadata records and diagnostic records.
Large record sets use a worker-specific paged transfer with explicit generation and sequence numbers.
The host publishes that generation only after the final manifest and all records agree.
An interrupted transfer cannot expose a partial compiled snapshot as complete.

### 10.5 Source and diagnostic records

Each source-bound record identifies the canonical source object and its content hash.
Precise spans use absolute UTF-8 byte offsets into those bytes.
The worker must normalize runtime-specific coordinates through tested adapters.
Unknown coordinates remain absent.

Metadata records distinguish declarations from generated definitions.
They retain original source mappings when those mappings exist.
The host rejects ranges that exceed the referenced content.
The host also rejects records from another project context or manifest.

### 10.6 Failure and recovery

PRO-017 requires a worker heartbeat or bounded operation deadline.
A blocked worker receives cancellation and then bounded process termination.
The host uses an owned process group where the platform supports it.
It must not terminate unrelated BEAM processes.

Worker restart attempts require a bounded backoff policy.
Repeated failure disables enrichment until an explicit retry or relevant context change.
The source index and active client connection remain available.
Restart never clears source facts or requires a PLT rebuild.

## 11. Required protocol evidence

| ID | Required adversarial test |
|---|---|
| PRO-018 | Split headers, split UTF-8 payloads, concatenated frames, and malformed lengths |
| PRO-019 | String request IDs, duplicate completion races, shutdown, and unexpected EOF |
| PRO-020 | Full text updates, incremental batches, old versions, and malformed ranges |
| PRO-021 | Emoji, combining marks, CRLF, and out-of-range positions |
| PRO-022 | OMP disk writes with open overlays and closed files |
| PRO-023 | Deadline expiry and client cancellation during every long query class |
| PRO-024 | Partial result assembly without duplicates and correct terminal responses |
| PRO-025 | Push and pull diagnostic generations after rapid edits |
| PRO-026 | Unsupported semantic scope without a false empty success |
| PRO-027 | Rename races, unsupported edit capabilities, and overlapping edits |
| PRO-028 | Worker crash, corrupt output, runtime mismatch, and blocked compilation |
| PRO-029 | Extension negotiation, absent features, old bootstrap status, and version mismatch |

Protocol tests must validate complete JSON structures.
Substring assertions do not establish protocol conformance.
The external test client must remain independent of the server's serializer.

## Sources

The baseline specification controls standard field meanings and method result shapes.
Quicksilver-specific extensions and worker messages are original project contracts.

- [S1] Language Server Protocol 3.17 specification: <https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/>
- [S2] Tree-sitter byte and point coordinates: <https://tree-sitter.github.io/tree-sitter/using-parsers/2-basic-parsing.html>
- [S3] Source form of the LSP 3.17 specification: <https://raw.githubusercontent.com/microsoft/language-server-protocol/gh-pages/_specifications/lsp/3.17/specification.md>
- [S4] Rust `lsp-server` API: <https://docs.rs/lsp-server/0.7.9/lsp_server/>
- [S5] Rust LSP type definitions: <https://docs.rs/lsp-types/latest/lsp_types/>
- [S6] Elixir compiler and metadata APIs: <https://elixir.hexdocs.pm/Code.html>

Implementation reviews must compare each feature with its exact baseline specification section.
