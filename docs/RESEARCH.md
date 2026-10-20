# Research and evidence

## Scope

This file records primary sources that informed the Quicksilver implementation kit.
The review date is October 20, 2026 UTC.
Upstream `main` documents describe a development revision, not every installed release.
The kit contains architecture decisions and proposed targets, not a claim of feature parity or superior measured speed.

## Primary source register

| ID | Source | Verified fact | Consequence for Quicksilver |
| --- | --- | --- | --- |
| O1 | [OMP CLI reference](https://github.com/can1357/oh-my-pi/blob/main/docs/cli-reference.md) | OMP accepts positional prompts, file attachments, and an explicit workspace directory. | The handoff uses the supported interactive launch interface. |
| O2 | [OMP context files](https://github.com/can1357/oh-my-pi/blob/main/docs/context-files.md) | OMP discovers root and ancestor instruction files with precedence rules. | The handoff names the root instructions explicitly. |
| O3 | [OMP LSP configuration](https://github.com/can1357/oh-my-pi/blob/main/docs/lsp-config.md) | Server definitions support commands, arguments, file types, and root markers. | The example registers a prebuilt Quicksilver executable. |
| O4 | [OMP server defaults](https://github.com/can1357/oh-my-pi/blob/main/packages/coding-agent/src/lsp/defaults.json) | OMP defines ElixirLS and Expert. Its ElixirLS settings enable Dialyzer. | The example disables both default Elixir servers. |
| O5 | [OMP LSP tool](https://github.com/can1357/oh-my-pi/blob/main/docs/tools/lsp.md) | Concrete-file diagnostics run against applicable servers and use a bounded result wait. | End-to-end tests include client deadlines and effective server selection. |
| E1 | [ElixirLS README](https://github.com/elixir-lsp/elixir-ls) | ElixirLS uses project compilation and cached Dialyzer analysis. | A repeated full PLT rebuild requires evidence from a real trace. |
| E2 | [Expert architecture](https://github.com/expert-lsp/expert/blob/main/pages/architecture.md) | Expert separates its manager from the project engine and compiles project code. | The benchmark distinguishes compilation, index work, and transport latency. |
| T1 | [Tree-sitter introduction](https://tree-sitter.github.io/tree-sitter/) | Tree-sitter supplies incremental concrete syntax trees and error recovery. | Syntax analysis can form an independent source layer. |
| T2 | [Elixir grammar](https://github.com/elixir-lang/tree-sitter-elixir) | The Elixir organization supplies a Tree-sitter grammar. | Quicksilver can reuse a maintained grammar. |
| R1 | [rust-analyzer architecture](https://rust-analyzer.github.io/book/contributing/architecture.html) | Its analysis model supports source deltas, incremental queries, and immutable analysis snapshots. | Quicksilver adopts the separation as a design principle. |

## Established facts

ElixirLS describes expensive initial analysis followed by cached analysis through `.elixir_ls/dialyzer_manifest`.
Its documentation does not prescribe a complete PLT rebuild after every source edit.
Its workspace symbol feature also depends on Dialyzer integration. [E1]

Expert's current design performs both document and project compilation.
It stores project build artifacts separately and maintains a source index.
Its engine uses the project's Elixir and OTP versions. [E2]

Tree-sitter produces syntax structure.
The grammar does not establish Elixir name resolution or complete macro semantics.
That distinction is an architectural inference from the parser's scope. [T1, T2]

rust-analyzer separates syntax, incremental semantic state, and compiler integration.
Its source architecture demonstrates a relevant design pattern.
It provides no ready-made Elixir semantic model. [R1]

## Decisions based on those facts

Quicksilver uses Rust for a persistent source engine.
The source engine must answer supported queries without a compiler or PLT dependency.
Optional BEAM work must enrich versioned source facts without control over source availability.
Those are product requirements, not statements about the supplied scaffold's completeness.

The initial parser supplies a useful implementation boundary.
The supported semantic subset requires a separate resolver and independent fixtures.
Dynamic code generation requires explicit uncertainty or versioned compiler evidence.
The project must not infer semantic precision from parser speed.

## Claims that this research does not establish

- Quicksilver is faster than ElixirLS or Expert on the user's project.
- Rust alone removes project compilation latency.
- Every observed Elixir delay is a PLT rebuild.
- Every supported Elixir macro can resolve from static syntax.
- A source-only result proves that the project compiles or passes tests.
- Current OMP `main` matches the user's installed binary.
- The supplied scaffold satisfies the final source-navigation objective.

## Required local evidence

The implementation must capture the user's representative project trace before a comparative performance claim.
That trace must identify the actual slow operation and active processes.
It must separate source edits from dependency or toolchain changes.
It must distinguish a fresh process from a reused process.
The comparison must use the same query and expected answer in every server.

The release evidence must include a pinned OMP version and target corpus.
The semantic manifest must preserve positive, negative, and unsupported cases.
The performance record must preserve failed attempts and incomplete results.
[QUALITY_GATES.md](QUALITY_GATES.md) and [PERFORMANCE.md](PERFORMANCE.md) define those requirements.

## Source maintenance

1. Record an immutable upstream revision for each dependency at implementation time.
2. Verify that installed OMP behavior matches the selected reference revision.
3. Update this register when an external behavior changes a design decision.
4. Preserve the reason for each changed decision in [DECISIONS.md](DECISIONS.md).
