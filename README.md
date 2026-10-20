# Quicksilver

**Fast navigation. Deep analysis in the background.**

Quicksilver is a Rust language server for Elixir, with agent workflows as a primary use case.
Its target architecture keeps source intelligence independent of project compilation and Dialyzer.
An optional BEAM worker supplies additional semantic facts and compiler diagnostics.

## Start

Read [START_HERE.md](START_HERE.md).

From the project root, launch the implementation session:

```bash
omp @prompts/IMPLEMENT.md "Implement Quicksilver through every release gate in planning/tasks.json."
```

## What this kit contains

The kit contains a runnable Rust scaffold and a complete implementation specification.
The scaffold is the first engineering slice.
It is not the complete language server.

| Area | Included state | Target state |
| --- | --- | --- |
| Transport | Stdio LSP lifecycle | Bounded requests, cancellation, recovery, and client compatibility |
| Documents | Full text synchronization | Versioned overlays, incremental edits, and external file events |
| Syntax | Elixir parser, document symbols, and syntax diagnostics | Incremental source model with precise scope information |
| Workspace | Future implementation | Persistent index with isolated worktree state |
| Semantics | Future implementation | Definitions, references, hover, completion, and safe refactors |
| BEAM integration | Future implementation | Optional compiler and macro metadata worker |
| OMP | Handoff prompt and example configuration | Verified complete workflow with one selected Elixir server |
| Quality | Executable scaffold verification and benchmark harness | Semantic, latency, stress, platform, and release evidence |

The detailed status appears in [docs/FEATURE_MATRIX.md](docs/FEATURE_MATRIX.md).
Measured kit results appear in [reports/KIT_VERIFICATION.md](reports/KIT_VERIFICATION.md).
Target performance budgets appear in [docs/PERFORMANCE.md](docs/PERFORMANCE.md).

## Product contract

- A source query never waits for a PLT build.
- A source query does not require a successful Mix build.
- Each analysis result belongs to a coherent source snapshot.
- An unknown result never masquerades as a complete empty result.
- A failed optional worker never disables source features.
- An advertised capability has a corresponding acceptance test.
- A repeated edit invalidates only the required analysis state.

## Build the scaffold

Install the Rust version in `rust-toolchain.toml`.

```bash
cargo build --locked
python3 scripts/lsp_smoke.py --server target/debug/quicksilver
```

Run all local verification:

```bash
python3 scripts/verify.py
```

The scripts require Python 3.11 or later.
The default verification does not require Elixir, Erlang, Mix, or OMP.
Later release gates explicitly require those tools.

## Implementation map

| File | Purpose |
| --- | --- |
| [AGENTS.md](AGENTS.md) | Durable instructions for the implementation agent |
| [prompts/IMPLEMENT.md](prompts/IMPLEMENT.md) | Initial execution prompt |
| [prompts/RESUME.md](prompts/RESUME.md) | Recovery after a session boundary |
| [prompts/REVIEW.md](prompts/REVIEW.md) | Independent release review |
| [docs/VISION.md](docs/VISION.md) | Product boundaries and complete delivery criteria |
| [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) | Processes, snapshots, caches, and request paths |
| [docs/ELIXIR_SEMANTICS.md](docs/ELIXIR_SEMANTICS.md) | Required language behavior and intentional limits |
| [docs/PROTOCOL.md](docs/PROTOCOL.md) | LSP and worker contracts |
| [docs/IMPLEMENTATION_PLAN.md](docs/IMPLEMENTATION_PLAN.md) | Phases and execution order |
| [planning/tasks.json](planning/tasks.json) | Machine-readable dependency graph and acceptance conditions |
| [docs/QUALITY_GATES.md](docs/QUALITY_GATES.md) | Correctness and release evidence |
| [docs/PERFORMANCE.md](docs/PERFORMANCE.md) | Measurement method and proposed budgets |
| [docs/OMP_INTEGRATION.md](docs/OMP_INTEGRATION.md) | Harness and client configuration |
| [docs/RESEARCH.md](docs/RESEARCH.md) | Primary sources and evidence limits |
| [docs/DECISIONS.md](docs/DECISIONS.md) | Architecture decisions and their consequences |
| [docs/OPERATIONS.md](docs/OPERATIONS.md) | Recovery, diagnostics, and release operations |

## Development defaults

The CLI name is `quicksilver`.
The baseline transport is `quicksilver --stdio`.
The target cache directory is `.quicksilver/` inside each project root.
The scaffold does not create that cache.
Cargo registry publication is disabled until the release gates pass.

The source license is MIT.
The ZIP excludes build outputs and external dependencies.

