# Start here

## Purpose

This directory is the Quicksilver implementation kit.
It contains the project vision, a tested starting slice, and the instructions for a sustained OMP implementation session.
The full target appears in `docs/VISION.md`.
The current implementation status appears in `docs/FEATURE_MATRIX.md`.

## Extract the ZIP

Extract the archive into your existing project root.

```bash
unzip /absolute/path/to/quicksilver-project-kit.zip -d ~/src/quicksilver
cd ~/src/quicksilver
```

The ZIP contains root-level files.
It does not contain an extra `quicksilver/` wrapper directory.

## Launch OMP

From the project root, run the implementation prompt:

```bash
omp @prompts/IMPLEMENT.md "Implement Quicksilver through every release gate in planning/tasks.json."
```

OMP supports an initial prompt with file attachments.
Its context loader recognizes a root `AGENTS.md`.
The explicit attachment carries the execution objective even when another context file takes precedence.
The integration document links the exact upstream sources.

The kit does not select a model or alter approval settings.
Your harness retains its configured model and tool permissions.
The implementation prompt permits routine engineering decisions within the project.

## Existing harness session

Give the harness this instruction:

```text
Read AGENTS.md and prompts/IMPLEMENT.md in ~/src/quicksilver.
Treat planning/tasks.json as the implementation ledger.
Implement the complete Quicksilver vision through all release gates.
Continue after each phase and preserve evidence in reports/.
```

## Resume after interruption

Resume the prior OMP session:

```bash
omp --continue "Read prompts/RESUME.md and resume the first incomplete release gate."
```

For a new session, attach the recovery prompt:

```bash
omp @prompts/RESUME.md "Resume Quicksilver from the repository evidence."
```

The task ledger and `planning/PROGRESS.md` survive a session boundary.
Task completion requires evidence, not a previous agent statement.

## Inspect the scaffold

Install the pinned Rust toolchain through your usual toolchain manager.

```bash
rustup toolchain install 1.99.1 --profile minimal --component rustfmt --component clippy
```

Build and verify the project:

```bash
cargo build --locked
python3 scripts/verify.py
```

The first Cargo build downloads the locked dependencies.
Later verification supports an offline Cargo cache.
No Elixir toolchain is required for the scaffold.
Future worker and semantic gates require the documented Elixir and OTP matrix.

## Inspect task readiness

List tasks whose dependencies are complete:

```bash
python3 scripts/task_status.py
```

List every task:

```bash
python3 scripts/task_status.py --all
```

The status script only reads the ledger.
It does not change completion state.

## Client integration

The initial OMP session develops Rust code in this repository.
Its ordinary Rust tooling remains useful during development.
The example Elixir configuration lives in `examples/omp/quicksilver.lsp.json`.
That example is deliberately outside OMP's automatic configuration directories.

After the relevant feature gates pass, follow `docs/OMP_INTEGRATION.md`.
The client must use the built executable.
The client must disable both `elixirls` and `expert` for the evaluated project.
A replacement test otherwise risks extra analysis from another server.

## Definition of complete

The complete project includes source intelligence and optional BEAM enrichment.
It includes semantic verification and performance evidence.
It includes a tested OMP integration and reproducible release packages.
The scaffold alone does not satisfy completion.
Public publication remains a separate action under the owner's control.

The final implementation report must identify all unsupported language cases.
It must preserve any failed or unavailable verification gate.
It must never convert an unavailable environment into a pass.

