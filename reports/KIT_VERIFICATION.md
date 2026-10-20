# Kit verification

## Result

The supplied Rust scaffold passed its executable preparation checks on 2026-10-20 UTC.
This report covers the implementation kit and its narrow source feature set.
The complete Quicksilver language server remains the objective of the receiving harness.

All 32 implementation tasks remain pending.
The receiving harness must verify each task in its actual environment before a completion claim.
The release validator correctly rejects this starting state.

## Environment

| Field | Observed value |
| --- | --- |
| Platform | Linux x86_64, kernel 6.18.55, glibc 2.39 |
| Rust compiler | `rustc 1.99.1 (58cf8f9c3 2026-10-13)` |
| Cargo | `cargo 1.99.1 (5f94df478 2026-08-27)` |
| Python | 3.12.14 |
| Logical CPUs reported to Python | 9 |
| Cargo dependencies | Locked and available in the preparation cache |
| Elixir, Erlang, Mix, OMP, and Bun | Unavailable in the preparation environment |

The final Rust verification used a fresh target directory.
No stale Cargo fingerprint or compiler diagnostic entered the accepted report.
The first Cargo build on another machine requires the locked dependency downloads.

## Executed checks

| Check | Result | Evidence |
| --- | --- | --- |
| Kit structure and local links | Passed: 32 tasks, 35 cases, and 86 source anchors | [kit-structure.json](kit-structure.json) |
| Rust formatting | Passed | [scaffold-verification.json](scaffold-verification.json) |
| Rust unit tests | 18 passed, zero failures | [scaffold-verification.json](scaffold-verification.json) |
| Clippy with `-D warnings` | Passed with no error diagnostics | [scaffold-verification.json](scaffold-verification.json) |
| Debug executable build | Passed | [scaffold-verification.json](scaffold-verification.json) |
| Release executable build | Passed | [scaffold-build.json](scaffold-build.json) |
| External stdio protocol smoke | 16 groups passed | [scaffold-smoke.json](scaffold-smoke.json) |
| Repeated-edit scaffold benchmark | 1000 successful samples, zero failures | [scaffold-benchmark.json](scaffold-benchmark.json) |
| Release guard | Correctly refused all 32 pending gates | [release-guard.json](release-guard.json) |

The raw suite report preserves exact command arguments, exit codes, output, and elapsed time.
The verifier checks error diagnostics independently of process exit codes.
The [review report](KIT_REVIEW.md) records the resolved code, protocol, fixture, and evidence defects.
The [source provenance record](source-provenance.json) identifies all Rust modules, Cargo inputs, and Python verification scripts by hash.

## Current executable scope

The CLI supports `--stdio`, `--version`, and `--help`.
The server supports the basic LSP lifecycle and full-text document synchronization.
It supplies source declaration outlines and syntax diagnostics for open Elixir documents.
It supports UTF-16 positions and negotiated UTF-8 positions.

The smoke suite verifies CRLF, non-BMP characters, and exact declaration selections.
It verifies stale edit rejection, malformed-source recovery, diagnostic clear on close, and flat symbol compatibility.
It also verifies fragmented headers, fragmented bodies, split UTF-8 bytes, and exit semantics.

The transport limits headers to 8 KiB and payloads to 16 MiB before allocation.
The document limit is 2 MiB per open document, with at most 256 open documents.
An outline has a traversal depth limit of 256.
Each syntax report contains at most 100 parser diagnostics.

The bootstrap reparses the complete open document after each accepted full-text change.
It has no workspace resolver, persistent index, optional worker, or semantic navigation implementation.
Its status response explicitly identifies syntax-only coverage and unavailable semantic readiness.
The [feature matrix](../docs/FEATURE_MATRIX.md) records the exact boundaries.

## Scaffold benchmark

The workload used one synthetic document with 250 functions and 7553 source bytes.
The test changed a declaration name and function body before each symbol request.
Each result had to contain the current edit's unique declaration marker.
The client also required diagnostics for that accepted document version.

| Measurement | Observed value |
| --- | ---: |
| Requested warm samples | 1000 |
| Successful warm samples | 1000 |
| Failed requests | 0 |
| Unattempted samples | 0 |
| Warm p50 | 7.0428 ms |
| Warm p95 | 9.9906 ms |
| Warm p99 | 12.4144 ms |
| Warm maximum | 21.8439 ms |
| Cold process start to first symbols | 15.4271 ms, one observation |

The warm timer covers the full-text update and the fresh document-symbol response.
Diagnostic receipt is an additional correctness check outside that response timer.
The report retains every warm sample, the corpus hash, and the release binary hash.
Percentiles use the nearest-rank method.

These results describe the supplied scaffold on a shared Linux preparation environment.
They do not establish semantic definition or reference latency.
They do not establish behavior during compilation, a production workspace scan, or a PLT job.
The single cold observation does not establish a cold percentile.
The [production budgets](../docs/PERFORMANCE.md) remain unmeasured acceptance targets.

## Future fixture corpus

The catalog contains 35 pending target cases.
It separates 32 intended supported-exact cases from three expected refusals.
The structural check verifies all referenced files and 86 literal source anchors.
Source review verified the CRLF bytes and the expected UTF-16 and UTF-8 coordinates.

No Elixir compiler or semantic oracle executed those cases.
The supplied scaffold has no semantic fixture runner.
QS-002 requires that independent runner.
QS-028 requires broader hygiene and real-project coverage plus a requirement coverage matrix.

The reference cases are an implementation input.
Their presence does not establish passing semantic behavior.

## Unexecuted gates

| Gate | Current state |
| --- | --- |
| Elixir and OTP compiler matrix | Not executed |
| Generated macro metadata oracle | Not executed |
| EEx or HEEx engine oracle | Not executed |
| OMP harness trace | Not executed |
| Semantic correctness suite | No semantic engine or acceptance runner yet |
| Worker failure and compile concurrency | No worker implementation yet |
| Production latency and 10000-edit soak | Not executed |
| macOS arm64 or Windows execution | Not executed |
| GitHub Actions workflow | Supplied, not executed by GitHub during preparation |
| Complete release gate | Incomplete by design |

No unavailable environment receives a substitute pass.
The kit does not claim a stable production release.

## Reproduction

From the extracted project root, run:

```bash
python3 scripts/verify.py
cargo build --locked --release
python3 scripts/benchmark.py --server target/release/quicksilver --iterations 1000 --output reports/local/scaffold-benchmark.json
```

Use `python3 scripts/verify.py --offline` only after the Cargo dependencies exist locally.
The suite honors `CARGO_TARGET_DIR` for an isolated verification build.
The normal reports under `reports/local/` preserve new environment-specific results.
The shipped preparation reports remain a separate baseline.
