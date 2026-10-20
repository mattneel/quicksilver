# Operations and recovery

## Status

The scaffold supports stdio mode and a small status response.
The target service adds index and worker status after the corresponding task gates pass.
The protocol specification defines the target schema.

The target status identifies these independent states:

- Server lifecycle.
- Source revision and index coverage.
- Worker runtime and job state.
- Compiler result revision.
- Explicit deep-analysis job state.
- Cache schema and project identity.

## Local development procedure

1. Run `python3 scripts/validate_kit.py`.
2. Run `cargo test --locked`.
3. Run `cargo clippy --locked --all-targets -- -D warnings`.
4. Run `cargo fmt --all -- --check`.
5. Run `python3 scripts/lsp_smoke.py --server target/debug/quicksilver`.
6. Record new evidence under `reports/`.

## Benchmark procedure

1. Build the release executable with `cargo build --locked --release`.
2. Run the scaffold benchmark command below.
3. Preserve its raw samples and binary hash.
4. Compare only results with equivalent workloads and environments.

```bash
python3 scripts/benchmark.py --server target/release/quicksilver --iterations 200 --output reports/local/scaffold-benchmark.json
```

The initial benchmark measures document symbols after source edits.
It does not measure the future resolver or BEAM worker.
The performance specification requires additional workloads before a release claim.

## Runtime recovery

Worker failure leaves source mode available.
The target manager applies bounded restart backoff to a failed worker.
The manager preserves a worker failure reason in status and logs.
Repeated failure never starts an unbounded process loop.

Stale compiler facts remain separate from current source facts.
The client can request a new compile after the underlying failure is resolved.
An explicit deep-analysis job can fail independently.

## Cache recovery procedure

1. Stop the Quicksilver process for the affected project.
2. Preserve its latest diagnostic report.
3. Rename only that project's `.quicksilver/` directory for investigation.
4. Restart Quicksilver in source mode.
5. Verify source query recovery before the worker starts.

This procedure applies after persistent caches exist.
The seed does not create a project cache.
The procedure never removes `_build`, `deps`, `.elixir_ls`, or `.expert`.

## Logging

Stdio mode reserves stdout for LSP frames.
Human-readable logs use stderr or an explicit log file.
The target service includes request IDs and source revisions in diagnostic logs.
It excludes source text and environment secrets from default logs.
Verbose source traces require an explicit diagnostic setting.

## Release procedure

1. Verify every mandatory task in `planning/tasks.json`.
2. Run the supported platform and runtime matrix.
3. Run the complete semantic fixture suite.
4. Run the repeated-edit and worker-failure workloads.
5. Verify OMP integration with one selected Elixir server.
6. Run `python3 scripts/validate_kit.py --release`.
7. Build the source and binary archives.
8. Generate checksums for each archive.
9. Verify installation in a clean directory.
10. Record deliberate limitations in the release notes.

Public publication is a separate owner-controlled action.
The release candidate can be complete before public publication.

## Reports

Each report identifies its scope and evidence class.

| Evidence class | Meaning |
| --- | --- |
| Scaffold | Executed against the included narrow implementation |
| Semantic fixture | Executed against explicit supported or unsupported source cases |
| Compiler oracle | Executed against a real supported Elixir and OTP pair |
| OMP integration | Executed through the actual harness and effective configuration |
| Platform | Executed on the named operating system and architecture |
| Target | Proposed acceptance condition without a passing measurement |

An evidence class never upgrades itself through prose.
A target remains a target until an actual verification run supplies evidence.

