# Kit review

## Scope

The review covered the supplied scaffold, project contracts, OMP handoff, test client, and future fixture catalog.
The reviewers worked on separate file sets and challenged shared contracts.
This report records resolved preparation findings.
It does not establish production semantic correctness.

## Resolved findings

| Finding | Resolution | Evidence |
| --- | --- | --- |
| Incoming LSP lengths allowed excessive allocation | The transport validates frame and header limits before payload allocation. | Rust transport tests and fragmented-frame smoke |
| A Python range helper accepted oversized columns | The helper rejects invalid lines, columns, and encoded character boundaries. | Exact Unicode range smoke and direct helper review |
| Flat-symbol verification accepted an empty list | The check requires the exact expected declaration names. | Flat `SymbolInformation` smoke |
| Close verification accepted a stale diagnostic version | The helper distinguishes an absent version from any version. | Close and diagnostic-clear smoke |
| A pipe write assumed that every byte was accepted | The client repeats short writes and bounds cleanup. | Fragmented-frame smoke and direct helper review |
| A count-only benchmark accepted stale declaration sets | Every edit has a unique declaration marker that the response must contain. | The 1000-edit benchmark |
| Cached Clippy error text accompanied a zero exit | The final suite used a fresh build directory. The verifier now rejects error diagnostics independently of exit status. | `scaffold-verification.json` |
| Rename requirements overclaimed closed-file protection | The contracts distinguish open-document versions from closed-file client preconditions. | QS-021 and the protocol refactor contract |
| Standard formatting responses lacked a clear race contract | Strict automatic application requires a verified client content-hash check. | QS-022 and the protocol format contract |
| Several target features had no task owner | Semantic tokens and structural ranges now have explicit task acceptance. Optional future features remain separate. | QS-020, QS-024, and the feature matrix |
| A generated-definition fixture assumed the macro's source location | The macro now uses `quote location: :keep`. A real compiler oracle must still verify the origin. | FX-023 and the official quote documentation |
| The initial semantic catalog lacked direct hygiene coverage | The release task requires additional hygiene and real-project cases plus a requirement coverage matrix. | QS-028 |

The [Elixir quote documentation](https://elixir.hexdocs.pm/Kernel.SpecialForms.html#quote/2-stacktrace-information) defines the location option used by FX-023.
The fixture change aligns its intended source contract with that documented behavior.
It does not replace compiler execution.

## Fixture review limits

The review verified 86 source anchors across 35 pending cases.
It checked literal source locations and Unicode coordinates from file bytes.
It reviewed intended imports, default arities, macros, protocols, behaviours, variable scopes, and template mappings.
No Elixir or template compiler ran.

The catalog contains 32 intended supported-exact cases and three expected refusals.
Those counts describe target cases, not passing semantic tests.
Future execution must preserve separate accuracy and refusal denominators.

## Remaining engineering work

The source resolver, persistent index, worker, advanced features, and production client traces remain implementation tasks.
All 32 task statuses remain pending for the receiving harness.
The harness must reverify the baseline in its own environment.
The full release gate intentionally fails at this starting state.
