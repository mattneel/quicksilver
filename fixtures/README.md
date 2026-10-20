# Semantic fixtures

## Evidence class

These fixtures define future Quicksilver acceptance cases.
All 35 cases have `status: pending` and `evidence_class: target`.
No Elixir compiler or semantic oracle ran during fixture creation.
The kit's structural validator does not change that status.

The examples require independent review before a semantic accuracy claim.
Their expected answers describe the intended contract.
They are not snapshots of Quicksilver's current output.
The supplied scaffold does not implement most required operations.

## Layout

| Path | Purpose |
| --- | --- |
| `mix_project/` | A Mix project with `deps: []` and valid core language examples |
| `plain/` | Unicode, CRLF, and intentionally malformed source |
| `scenarios/` | Source variants for a logical document race |
| `worktrees/` | Two synthetic workspace roots with equal relative source paths |
| `offline_project/` | A project with one local path dependency |
| `offline_dependency/` | Source for that local dependency |
| `templates/` | HEEx and EEx expressions for future source-map tests |
| `cases.json` | The target manifest and expected outcomes |

Malformed files remain outside the core Mix project's `lib/` directory.
Template cases do not add Phoenix dependencies to the core project.
The synthetic workspace roots are not actual Git worktrees.
A real Git-worktree acceptance trace remains separate work.

## Case isolation

Each case's `files` list uses paths relative to the kit root.
The list defines the case's source snapshot.
The future runner must not include unrelated fixture groups automatically.
A case with explicit roots must preserve those root identities.

The complete core Mix project supplies a separate compiler oracle input.
Its compilation does not define each navigation case's search domain.
The offline project uses only local dependency source.
Its offline source-query test must prohibit compiler and dependency-fetch processes.

## Anchor convention

Every location identifies its source file and a unique literal anchor.
The anchor occurs on the same line as the selected text.
The `occurrence` value selects a one-based occurrence before that anchor.
The marker itself is not part of the selected token.

The UTF-16 case includes an independent coordinate calculation.
JavaScript string length counts UTF-16 code units for that calculation.
A code-point calculation supplies the UTF-8 byte count.
The CRLF file contains actual carriage-return and newline pairs.

The fixture runner must resolve anchors from current file bytes.
It must fail when an anchor is absent or ambiguous.
It must fail when the selected occurrence is absent.
It must never silently use a stale line number.

## Expected result model

The `expected` objects contain fixture assertions.
They are not JSON-RPC responses or extra fields on standard LSP results.
The runner compares standard results and provenance through the applicable protocol contract.

| Classification | Required result | Accuracy treatment |
| --- | --- | --- |
| `supported_exact` | Exact declared result within the named domain | Included in the supported-case denominator |
| `expected_refusal` | Explicit refusal with no fabricated locations or edits | Reported separately from supported correctness |

A case with several assertions passes only when all assertions pass.
Negative diagnostic and reference cases require separate false-positive counts.
A refused case never increases the positive correctness numerator.
A malformed-source case can require exact facts inside a partial validity domain.

Phase P1 cases contain document-state assertions.
Some also name a later semantic assertion for P3.
The implementation must satisfy both stages before it marks the whole case complete.

## Oracle procedure

1. Select a supported Elixir and OTP pair.
2. Record both versions and the fixture source hash.
3. Compile the core Mix project.
4. Inspect declarations and callable arities through an independent BEAM tool.
5. Verify source locations against the hand-authored anchors.
6. Review lexical scope expectations against the Elixir documentation.
7. Compare the expected reference sets with an independent source review.
8. Record every disagreement before a fixture change.
9. Run Quicksilver against the isolated case snapshot.
10. Preserve the oracle and Quicksilver results separately.

## Core compiler command

```bash
cd fixtures/mix_project
mix compile
```

This command compiles fixture code and creates build artifacts inside the fixture project.
It is an oracle preparation step.
It does not replace the semantic fixture runner.

## Refusal cases

The source-only `use` case lacks verified macro expansion.
The dynamic remote call lacks a known module.
The public rename case contains a dynamic function name in `apply/3`.
Each case requires explicit uncertainty or refusal under the protocol contract.

The worker-enabled `use` case is a separate positive case.
It requires real compiler evidence and source provenance.
Its macro uses `quote location: :keep` to retain definition source locations.
The compiler oracle must still verify those exact locations.
Its presence does not convert the source-only refusal into an implementation failure.

## Remaining work

The future semantic runner must materialize the event scenarios.
It must apply the protocol opt-in for provenance tests.
It must preserve expected document versions during asynchronous operations.
It must record denominator counts and refusal counts separately.

The template oracle requires a pinned compatible template engine.
The fixture corpus does not install that engine.
The release suite must add real projects and adversarial cases beyond these 35 examples.

## Primary language references

- [Elixir special forms](https://elixir.hexdocs.pm/Kernel.SpecialForms.html)
- [Elixir Kernel](https://elixir.hexdocs.pm/Kernel.html)
- [Elixir Protocol](https://elixir.hexdocs.pm/Protocol.html)
- [Elixir Code](https://elixir.hexdocs.pm/Code.html)
- [Tree-sitter Elixir grammar](https://github.com/elixir-lang/tree-sitter-elixir)

The oracle field in each case identifies the relevant references.
Those references support language rules.
They do not establish that these fixture expectations passed execution.

