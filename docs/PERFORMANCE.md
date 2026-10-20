# Performance contract

## Status

All numerical budgets in this document are proposed targets.
They are unmeasured product goals, not benchmark results or speed guarantees.
The supplied scaffold does not implement the full source engine.
The final gate requires measurements against correct expected results.

The objective is useful source intelligence without a compiler or PLT barrier.
Compiler diagnostics and optional BEAM enrichment have separate lifecycles.
Their duration must not determine source-query availability.

## Reference environment

The initial target environment is a laptop or workstation with at least eight logical CPUs and 32 GiB of memory.
The source corpus and caches reside on a local SSD.
The benchmark uses a release executable and a fixed Rust toolchain.
The release record must identify the actual machine and operating system.
Different environments require separate result series.

Every result record must include these fields:

- CPU model and architecture.
- Logical CPU count and power mode.
- Total memory and storage type.
- Operating system and kernel version.
- Quicksilver source revision and executable hash.
- Rust toolchain and dependency lockfile hash.
- Elixir and OTP versions for worker comparisons.
- OMP version and effective settings for client traces.
- Corpus identifier, revision, and source hash.
- Cache state and process state.
- Iteration count and request mix.
- Concurrent workloads and worker mode.

## Corpora

| Corpus | Target size | Required content | Role |
| --- | --- | --- | --- |
| Small | 100 Elixir files, approximately 25,000 lines | Plain Mix project with declarations and references | Startup and local development |
| Medium | 1,000 Elixir files, approximately 250,000 lines | Umbrella project with dependency source and macros | Primary latency gate |
| Large | 10,000 Elixir files, approximately 2.5 million lines | Multiple applications and duplicate symbol names | Scaling and capacity |

Exact file counts and byte counts replace these estimates in each corpus manifest.
The manifest must state whether dependency source counts toward the total.
Synthetic corpora require separate labels from real projects.
The initial script fixture is a microbenchmark and cannot stand in for the medium corpus.

## Proposed latency budgets

The following targets apply to the medium corpus on the reference environment.
Warm document requests use documents no larger than 100 KiB.
Request duration includes transport and validates the returned source revision.

| Operation | p50 target | p95 target | p99 target |
| --- | ---: | ---: | ---: |
| Process spawn through initialize response | 50 ms | 150 ms | 300 ms |
| First useful source query with no Quicksilver cache | 100 ms | 500 ms | 1,000 ms |
| Syntax update after one warm edit | 10 ms | 25 ms | 50 ms |
| Warm definition or document-symbol query | 20 ms | 50 ms | 100 ms |
| Warm reference query across the workspace | 50 ms | 150 ms | 300 ms |
| Warm query during optional compiler work | 20 ms | 50 ms | 100 ms |

The workspace reference target uses a complete result with at most 1,000 locations.
Larger outputs require separate throughput and response-size measurements.
Truncated output never counts as a complete reference result.

Cold construction of the complete medium source index has a proposed p95 target of 10 seconds.
The first useful query target remains independent from complete index construction.
The protocol must expose incomplete workspace coverage under [PROTOCOL.md](PROTOCOL.md).
An empty result during incomplete coverage is not a successful semantic answer.

## Resource budgets

The proposed steady-state RSS target is 512 MiB for the source process on the medium corpus.
The optional BEAM worker has a separate resource record and configured limit.
The initial worker limit requires measurement before release approval.
Its memory never disappears inside the source-process total.

The source process must produce no monotonic document or query retention after the bounded workload stabilizes.
The release test must report RSS at fixed checkpoints across 10,000 edit cycles.
The last 2,000 cycles must show no unexplained retained-object growth.
Allocator high-water behavior requires an explicit explanation and bounded retained state.

Worker termination must leave no child process or stale request owner.
An ordinary source edit must not start a compiler or PLT rebuild.
Process and invalidation counters must demonstrate those properties.

## Cache and process scenarios

| Scenario | Initial state | Required measurement |
| --- | --- | --- |
| Cold source | New process and empty Quicksilver cache | Initialize, first useful query, complete index |
| Restart | New process and valid persistent cache | Cache validation and first useful query |
| Warm idle | Existing process and complete source snapshot | Correct query latency without an edit |
| Warm edit | Existing process and one body edit | Correct query latency for the new revision |
| Declaration edit | Existing process and changed exported declaration | Correct dependent invalidation and query latency |
| Edit burst | Ten changes per second with interleaved queries | Tail latency, canceled work, and final correctness |
| Compiler active | Warm source engine and busy optional worker | Source-query latency and separate worker progress |
| Compiler failed | Warm source engine and failed optional worker | Continued source availability and explicit worker failure |

Cold source does not imply an empty operating-system page cache.
The result record must state whether page-cache state was controlled.
An uncontrolled page cache requires that limitation in the comparison.
No benchmark procedure requires privileged system-cache deletion.

## Measurement method

Each timed request starts when the client writes its first protocol byte.
It ends when the client receives the complete response for that request identifier.
For diagnostics, the endpoint is the accepted report for the expected document version.
Initialization ends at its response and never substitutes for source readiness.

The suite must record these separate durations:

- Process startup and protocol initialization.
- Project discovery and cache validation.
- Index construction and dependency invalidation.
- Parse, resolution, and result serialization.
- Client transport and OMP tool overhead.
- Optional compiler and worker execution.

Warm results require at least 1,000 measured requests per operation and scenario.
Each warm series follows 100 unreported warmup requests.
Cold and restart series require at least 100 independent process launches.
The report must show p50, p95, and p99 with their sample counts.
Small samples require an explicit tail-estimate limitation.

Percentiles use the nearest-rank method over sorted durations.
Timeouts and failures remain in the attempt denominator.
The report must show their counts and configured deadlines separately.
A success-only percentile table cannot satisfy the gate when failed attempts exist.
The gate requires zero incorrect responses on the deterministic benchmark corpus.

## Correctness guard

Every measured query must assert its expected result and source revision.
The benchmark must change source content between edit iterations.
A server that repeats a cached answer for the previous revision must fail.
A method error, empty placeholder, or timed-out request must fail the required operation.
The benchmark must not reward a server for skipped analysis.

The harness must verify that the executable supports the measured capability.
An unsupported capability reports NOT_RUN or FAIL according to the required gate.
It never produces a successful zero-duration sample.
The scaffold's syntax and symbol timings cannot become semantic navigation results.

## Initial probe procedure

1. Build the release executable.

```bash
cargo build --release --locked
```

2. Run the supplied probe.

```bash
python3 scripts/benchmark.py --server target/release/quicksilver --iterations 1000
```

3. Record the tested operations and fixture size.
4. Record the environment and executable identity.
5. Extend the probe for each new accepted capability.

## Benchmark history

The implementation must retain machine-readable benchmark records by source revision and corpus identity.
Each record must contain raw samples or a durable reference to them.
A baseline comparison requires the same operation and environment class.
Different cache states or different result completeness require separate comparisons.
An absent baseline reports NO_BASELINE and never implies improvement.

The release gate fails when a required budget fails.
A regression against a prior accepted baseline requires an explained decision, even when the absolute budget passes.
Any budget change requires a documented reason and a new accepted baseline.
The project must not revise a target solely to label a failed run successful.

## Current evidence boundary

The source code and scripts support initial experimentation.
They supply no measured claim about full project navigation or BEAM isolation.
The full benchmark suite remains P6 implementation work.
Results from the target Elixir project determine whether Quicksilver resolves the original OMP latency problem.
