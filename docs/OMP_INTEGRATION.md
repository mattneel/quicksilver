# Oh-My-Pi integration

## Status

Quicksilver is an implementation kit for a Rust language server for Elixir.
The supplied scaffold covers document state and limited syntax analysis.
Project navigation and the optional BEAM worker remain implementation work.
The full objective includes phases P0 through P6 in the project plan.

The example configuration belongs in an Elixir target project after a local build.
It does not activate Quicksilver in this Rust repository.

## Start the implementation session

1. Extract the kit into `~/src/quicksilver`.
2. Open a terminal in that directory.
3. Verify the installed OMP version.

```bash
omp --version
```

4. Start OMP with the implementation prompt.

```bash
omp @prompts/IMPLEMENT.md "Implement Quicksilver through every release gate in planning/tasks.json."
```

## Handoff behavior

OMP accepts a positional prompt and `@file` attachments.
The command above starts an interactive session with the attached handoff.
The `-p` option selects a noninteractive session that exits after the prompt completes.
The `--cwd` option selects the workspace directory.
These behaviors appear in the [OMP CLI reference][omp-cli].

OMP discovers standalone `AGENTS.md` files from the current directory and relevant ancestors.
A native `.omp/AGENTS.md` at the same depth can take priority.
The handoff explicitly names the root file so the implementation objective remains clear.
These rules appear in the [OMP context documentation][omp-context].

## Resume an incomplete implementation

1. Open a terminal in `~/src/quicksilver`.
2. Start the resume prompt from the repository checkpoint.

```bash
omp @prompts/RESUME.md "Resume Quicksilver from the repository evidence."
```

## Delegation requirements

Each delegated implementation task must name its owned paths.
Each task must include the relevant acceptance gate and the required verification commands.
Each task must explicitly require the root `AGENTS.md` and applicable specifications.
A task response must distinguish changed code from executed verification.
The parent task must resolve integration failures before it declares a phase complete.

## Build the server

1. Open a terminal in the Quicksilver repository.
2. Build the release executable.

```bash
cargo build --release --locked
```

3. Verify the executable identity.

```bash
target/release/quicksilver --version
```

4. Run the protocol smoke test.

```bash
python3 scripts/lsp_smoke.py --server target/release/quicksilver
```

## Register an Elixir target project

1. Open `examples/omp/quicksilver.lsp.json`.
2. Replace `/ABSOLUTE/PATH/TO/quicksilver/target/release/quicksilver` with the executable's actual absolute path.
3. Open the target project's existing `.omp/lsp.json`, if it exists.
4. Merge the example server entries into that file.
5. Preserve unrelated server entries.
6. Start OMP from the target directory that contains `mix.exs`.
7. Request the LSP status action.

```json
{"action":"status"}
```

8. Verify that Quicksilver is the only active Elixir server for this test.

## Configuration contract

The example disables both `elixirls` and `expert`.
It registers `quicksilver` as a distinct server with `--stdio`.
It initially selects `.ex` and `.exs` files through Mix root markers.
HEEx and EEx remain outside this initial registration.
Their required P5 acceptance gates precede template registration.

The command points directly to a prebuilt executable.
It never invokes `cargo run` for an LSP request or session.
A build command in the launch path introduces avoidable startup work.

OMP merges server definitions from several locations.
Its server objects merge shallowly, while nested objects such as `settings` replace the lower value.
Root-marker discovery starts at the session directory.
The [configuration guide][omp-lsp-config] defines these rules and the supported fields.

OMP's current defaults define both ElixirLS and Expert for the same Elixir file extensions.
The ElixirLS definition explicitly enables Dialyzer.
The [default definitions][omp-defaults] establish those values.
Other custom Elixir server names require separate inspection in the target environment.

OMP's concrete-file diagnostics action queries all applicable servers.
The target configuration therefore disables both default Elixir servers for an isolated Quicksilver test.
The [LSP tool reference][omp-lsp-tool] defines this behavior.

## Reload configuration

1. After a configuration change, request a workspace reload through OMP's LSP tool.

```json
{"action":"reload","file":"*"}
```

2. Request status again.
3. Verify the executable path and active server names.

## Exercise the supplied scaffold

1. Select an existing `.ex` file in the target project.
2. Request its document symbols through OMP's LSP tool.
3. Introduce a syntax error in a disposable copy of the file.
4. Request diagnostics for that copy.
5. Remove the syntax error.
6. Verify that syntax diagnostics clear for the new document version.

## Meaning of a scaffold result

An empty syntax report establishes no syntax errors from the scaffold's parser for that document version.
It establishes no project compilation result or semantic correctness.
The scaffold supplies no definitions or references across files.
Unknown request methods return the standard method error.
The [feature matrix](FEATURE_MATRIX.md) records the exact boundary.

The scaffold supplies a bootstrap `quicksilver/status` method.
Its response explicitly reports `semanticReady: false` and `projectExecution: false`.
The complete version 1 extension remains a required P5 target under QS-025.
It includes status, query provenance, snapshot batches, and the external-source synchronization barrier.
That contract requires the opt-in capability and behavior in [PROTOCOL.md](PROTOCOL.md).

## Full OMP acceptance

The final implementation must pass an OMP edit-and-query trace against a pinned target corpus.
That trace must cover both open documents and external disk edits.
Its results must include source freshness and unsupported semantic cases.
The trace must demonstrate source navigation while compilation fails or the optional BEAM worker is unavailable.
The [quality gates](QUALITY_GATES.md) and [performance contract](PERFORMANCE.md) define the evidence.

## Source scope

The linked OMP documents describe upstream `main`, reviewed on October 20, 2026 UTC.
The installed OMP version and effective configuration determine actual client behavior.
The release record must include both values.

[omp-cli]: https://github.com/can1357/oh-my-pi/blob/main/docs/cli-reference.md
[omp-context]: https://github.com/can1357/oh-my-pi/blob/main/docs/context-files.md
[omp-lsp-config]: https://github.com/can1357/oh-my-pi/blob/main/docs/lsp-config.md
[omp-defaults]: https://github.com/can1357/oh-my-pi/blob/main/packages/coding-agent/src/lsp/defaults.json
[omp-lsp-tool]: https://github.com/can1357/oh-my-pi/blob/main/docs/tools/lsp.md
