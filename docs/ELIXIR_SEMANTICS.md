# Elixir semantics contract

## Status and scope

This document defines semantic capability gates for the target server.
The initial scaffold supplies syntax analysis only.
Its document symbols do not establish name resolution or compiler parity.
The required behavior applies to each runtime version in the tested compatibility matrix.

Tree-sitter supplies a concrete syntax tree. [S1]
It does not supply Elixir name resolution, macro execution, or compiler diagnostics.
The Rust analyzer must implement each supported semantic rule explicitly.
Compiler-backed evidence enters through the optional worker.

## 1. Evidence and completeness

Every semantic fact must carry an origin and a validity domain.
The analyzer must preserve that information through query evaluation.
A confidence percentage cannot replace an evidence record.

| Origin | Meaning | Permitted claim |
|---|---|---|
| `source_syntax` | A syntax node directly expresses the fact. | Textual definition or syntactic structure |
| `source_resolved` | Supported scope rules resolve the fact. | Exact identity within the recorded domain |
| `compiled` | A verified worker generation supplies the fact. | Compiler-derived fact for that generation |
| `dependency_metadata` | A compatible dependency artifact supplies the fact. | Artifact-specific documentation or definitions |
| `heuristic` | A rule proposes a candidate without proof. | Optional suggestion with explicit qualification |

| State | Meaning | Result policy |
|---|---|---|
| `complete` | The supported domain is fully analyzed. | Complete result for that named domain |
| `partial` | Known facts exist, but coverage is incomplete. | No complete reference or rename claim |
| `ambiguous` | More than one candidate remains valid. | Candidate presentation only where the protocol supports it |
| `unsupported` | The construct exceeds implemented semantics. | Explicit unsupported result |
| `unavailable` | Required input or runtime evidence is absent. | Explicit unavailable result |

SEM-001 requires distinct representations for an empty complete result and an unresolved result.
SEM-002 forbids heuristic facts from automatic edits or complete reference graphs.
SEM-003 requires each query to declare its search domain.
Examples include one lexical scope, one project, or an explicit set of dependency artifacts.

LSP conversion rules reside in [PROTOCOL.md](PROTOCOL.md).
Standard location arrays cannot carry an invented completeness field.

## 2. Syntax facts and source locations

P2 must represent modules, definitions, clauses, parameters, attributes, and direct source references.
Each fact records its source hash and byte range.
Display ranges and identifier selection ranges remain separate.
The coordinate layer converts byte ranges into the negotiated LSP encoding.

SEM-004 requires extraction to respect syntax node roles.
A token inside a string, comment, or quoted data does not automatically become a runtime reference.
An error node must not cause a sibling declaration to inherit a fabricated scope.
Facts outside damaged syntax retain their own explicit validity domain.

The initial grammar supports `.ex` and `.exs` source.
Template syntax requires separate gates.
Unknown file types cannot enter the Elixir semantic index through an extension guess.

### Required syntax fixtures

- Multiple modules in one file.
- Nested modules and multiline definition heads.
- Function clauses, guards, defaults, and operators.
- Private functions, macros, guards, and delegates.
- UTF-8 source, UTF-16 coordinates, and CRLF text.
- Incomplete expressions, missing delimiters, and error recovery.
- Strings, sigils, interpolation, comments, and quoted code.

## 3. Symbol identity

| Symbol | Required identity |
|---|---|
| Module | Project context and fully resolved module atom |
| Function | Module identity, function name, and arity |
| Macro | Module identity, macro name, and arity |
| Function clause | Function identity and clause source identity |
| Local variable | Lexical scope and distinct binding identity |
| Attribute | Module scope, attribute name, and applicable access point |
| Type | Module identity, type name, and type arity |
| Callback | Behavior identity, callback kind, name, and arity |
| Protocol implementation | Protocol identity and implemented type identity |

SEM-005 forbids a shared identity for every occurrence of the same spelling.
Function and macro namespaces require explicit treatment.
Erlang module atoms remain distinct from Elixir aliases.
Generated symbols retain both generated identity and expansion provenance when available.

An Elixir module can appear in more than one source location.
Conditional definitions and generated modules can make that relationship ambiguous.
The analyzer must not select the first file with a name match as an authoritative definition.

## 4. Lexical scopes and variables

The scope graph records nesting, declaration order, and construct-specific visibility.
A scope edge is a semantic relation, not an indentation relation.
The implementation must distinguish function clauses, anonymous functions, and branch scopes.
Each rule requires compiler-verified fixtures. [S2]

SEM-006 requires a new binding identity after a valid rebinding operation.
The right side of a match uses the environment before its new bindings enter scope.
Pinned variables refer to the applicable existing binding.
Pattern positions and expression positions require different analysis.

```elixir
x = 1
x = x + 1
{x, fn x -> x + 1 end}
```

The fixture must distinguish the initial binding, the rebound value, and the anonymous function parameter.
References to those identities must not collapse into one spelling-based result.

The supported scope model must cover these constructs:

- Function parameters, guards, and bodies.
- Anonymous function parameters and captures.
- `case`, `cond`, and `receive` clauses.
- `with` generators, body, and `else` clauses.
- `for` generators, filters, and reduction clauses.
- `try`, `rescue`, `catch`, `else`, and `after` sections.
- Pattern matches, pins, and repeated pattern variables.

SEM-007 requires explicit fixtures for visibility across every construct boundary.
A successful syntax parse does not satisfy this gate.
Unknown macro effects can invalidate a scope's completeness proof.
The server must preserve the reason for that invalidation.

## 5. Aliases, imports, and required modules

Aliases apply through the supported lexical environment at the query position.
An alias declaration does not rewrite every token with the same text.
The analyzer must handle `as` aliases and grouped aliases separately.
Nested module names require verified Elixir rules, not unconditional string concatenation. [S2]

Imports require module identity and effective import filters.
`only` and `except` affect candidate visibility.
An unresolved import preserves ambiguity for affected unqualified calls.
An explicit remote call can remain resolvable despite an unrelated unknown import.

`require` establishes macro availability under Elixir's rules.
It does not make every exported function an unqualified local name.
`use` can invoke user-defined macro behavior. [S3]
The source analyzer must not assume a fixed list of effects for arbitrary `use` calls.

SEM-008 requires position-sensitive directive facts.
SEM-009 requires explicit candidate sets for ambiguous imports.
SEM-010 forbids blanket failure for unrelated source facts after one unsupported directive.
The analyzer must track the smallest domain that becomes uncertain.

## 6. Calls, arity, and definitions

### 6.1 Direct calls

Resolution must distinguish these source forms:

- A local function call.
- An imported function or macro call.
- A remote call through a resolved module.
- An anonymous function invocation through `fun.(...)`.
- A variable reference without call syntax.
- An unresolved expression that can represent multiple forms.

SEM-011 requires arity after a supported pipe transformation.
The rule applies only when the pipe operation has known semantics.
A capture such as `&Module.function/2` retains its explicit arity.
Operator calls require the same identity checks as named calls.

### 6.2 Clauses and default arguments

One function identity can contain several clauses.
Document symbols can group those clauses without loss of their source ranges.
Default arguments create a callable arity family under compiler rules. [S3]
The source model must represent the family and its declaration origin.

SEM-012 requires fixtures for default heads and implementation clauses.
The analyzer must detect uncertainty from dynamic heads or generated wrappers.
It must not manufacture a source location for an implicit wrapper.
Signature help must distinguish a declared signature from an inferred call candidate.

### 6.3 Delegates and generated calls

`defdelegate` requires a declared identity and a target relation.
The relation includes any explicit target name change.
Navigation can expose the declaration and the verified delegated target.
A delegate does not authorize edits in an external dependency.

Dynamic calls require a bounded policy.
Literal arguments to `apply/3` can supply a candidate after expression analysis.
Nonliteral modules or names remain dynamic.
A candidate alone does not prove complete references.
The analyzer must not interpret arbitrary expressions to obtain a target.

## 7. Macros and compile-time execution

Macros accept syntax and can return different syntax. [S4]
Their results can depend on project code and the compile environment.
Pure source parsing cannot establish every generated definition.
Quicksilver does not claim full compiler parity without direct evidence.

SEM-013 divides macro support into explicit categories:

| Category | Source behavior | Enrichment behavior |
|---|---|---|
| Verified language construct | Implements versioned scope and identity rules | Differential compiler tests |
| Versioned framework rule | Supplies documented facts within a declared domain | Confirms compatible dependency identity |
| Arbitrary macro | Preserves call syntax and uncertainty | Uses authorized compiled evidence |
| Unavailable macro dependency | Reports unavailable enrichment | Retains independent source service |

An adapter must declare the package versions that it supports.
A version mismatch disables that adapter's semantic claims.
Adapters must not execute project macros in the Rust process.
An adapter cannot upgrade a heuristic result to complete evidence without a proof rule.

### 7.1 Quotation and hygiene

`quote`, `unquote`, and `bind_quoted` require separate scope treatment.
Quoted syntax does not automatically execute at its source location.
Variable hygiene includes context beyond the variable spelling. [S4]
Macro expansion metadata can identify generated source relationships.
Missing metadata remains missing.

SEM-014 forbids automatic rename across an unknown quotation or hygiene boundary.
`var!`, implicit binding inspection, and unverified macro scopes require rejection.
An expansion range without an original source mapping cannot receive a fabricated file edit.

### 7.2 Compile graph

The graph records dependency kinds separately:

- Runtime call dependencies.
- Macro and other compile-time dependencies.
- Export dependencies where supported by compiler evidence.
- Struct expansion dependencies.
- Compile environment and external resource dependencies.
- Unknown dependencies from unsupported dynamic behavior.

Compiler tracers and Mix cross-reference data can supplement this graph. [S5] [S6]
The worker must record the compiler version and event schema.
Unknown tracer events must not crash the worker.
A source call graph cannot substitute for the compiler dependency graph.

SEM-015 requires conservative invalidation when a compile dependency is unknown.
An arbitrary macro can read inputs outside the known graph.
The worker must not certify hermetic compilation without enforced input controls.
Compiled metadata retains that limitation in its provenance.

## 8. Attributes, structs, behaviors, and protocols

### 8.1 Module attributes

The analyzer separates attribute declarations from attribute reads.
Literal values can support source facts without evaluation.
Dynamic attribute expressions remain unknown until compatible evidence exists.
An attribute's value can depend on its access point during compilation. [S3]

### 8.2 Structs and exceptions

Literal `defstruct` fields can support structural completion.
The server must distinguish a struct field from an arbitrary map key.
Computed field declarations require compatible compiled metadata.
Default values do not establish a static field type by themselves.
`defexception` requires its own verified generation rules.

### 8.3 Behaviors and callbacks

Declared callbacks define interface facts.
`@impl` annotations connect implementation declarations to that interface when identity resolves.
The server must preserve callback kind and arity.
Optional callbacks require distinct metadata.
An implementation search requires coverage of the declared search domain.

### 8.4 Protocols

Protocol definitions and implementations require separate identities.
Generated dispatch modules do not automatically become user-editable declarations.
Implementation lookup must account for its indexed project and dependency domain.
The server cannot claim all runtime implementations from a partial source scan.

SEM-016 requires separate fixtures for each construct above.
One generic macro extractor does not satisfy their semantic gates.

## 9. Documentation and type information

### 9.1 Documentation sources

Current source documentation takes precedence for an edited declaration.
Compatible BEAM documentation can supplement unchanged dependency symbols.
`Code.fetch_docs/1` accepts a module or BEAM path. [S5]
The worker must preserve hidden or absent documentation states.
It must not invent text for unavailable docs.

Documentation output must retain origin and source identity internally.
A stale compiled excerpt can appear only with an explicit stale label where the LSP result supports text.
A stale source location cannot become a current definition result.
The server must not render arbitrary documentation as executable HTML.

### 9.2 Types and contracts

| Information | Origin | Required label |
|---|---|---|
| `@spec` | Author declaration | Declared spec |
| `@type`, `@typep`, `@opaque` | Author declaration | Declared type with visibility |
| `@callback`, `@macrocallback` | Author declaration | Declared callback |
| Source inference | Quicksilver rule | Inferred source information |
| Dialyzer result | Optional analysis generation | Dialyzer result with generation |
| Compiler type diagnostic | Supported compiler version | Compiler diagnostic |

Elixir typespecs and Dialyzer serve different roles. [S7] [S8]
SEM-017 forbids presentation of a declared spec as a proven runtime guarantee.
An inferred Dialyzer contract must not replace the author's spec silently.
The absence of a Dialyzer warning does not prove program correctness.

Navigation, symbols, and declared specs must not require a PLT.
Optional Dialyzer analysis requires a separate capability and cache contract.
The server must identify which engine produced each diagnostic.

## 10. Safe rename and edit scope

P5 begins with local variable rename inside one proven lexical domain.
Module rename and public function rename remain disabled until their stronger gates pass.
The implementation must not equate a complete syntax scan with a complete semantic graph.

SEM-018 permits rename only when all conditions hold:

- The target resolves to one supported symbol identity.
- Every relevant declaration and reference has current source evidence.
- The containing scope has a complete resolution graph.
- No unknown macro or reflection boundary affects that identity.
- The new name is valid for the symbol category.
- The new name causes no capture, shadow conflict, or changed resolution.
- Every edit refers to the exact verified document bytes.
- The client supports the required edit preconditions.

The preparation result does not reserve a future source revision.
The final rename request must repeat all source and completeness checks.
Overlapping edits invalidate the whole proposed edit.
An edit must not include files outside the permitted domain.

SEM-019 requires atomic refusal when one relevant location lacks proof.
The server must not return an apparently safe subset of an unsafe rename.
Clients without sufficient version safeguards receive a narrower supported domain or a refusal.
Exact LSP edit requirements appear in [PROTOCOL.md](PROTOCOL.md).

## 11. Templates and framework semantics

HEEx, EEx, and embedded sigils require separate parser and source-map gates.
Template ranges must map to the original host document bytes.
A transformed temporary file cannot appear as the user's source location.
Malformed template regions must not corrupt adjacent Elixir ranges.

Framework support must declare its scope explicitly:

- Phoenix components, attributes, and slots.
- Verified routes and template expressions.
- Ecto query bindings and schema declarations.
- ExUnit declarations and case templates.
- Other DSL adapters with pinned package identities.

SEM-020 forbids a generic claim of framework support from one successful completion example.
Each adapter needs positive, negative, incomplete-source, and version-mismatch fixtures.
Unsupported constructs preserve syntax service and truthful unavailable semantics.

## 12. Semantic proof and release claims

The semantic corpus must contain source fixtures and expected identities.
Each supported rule needs at least one adversarial counterexample.
Compiler-derived expectations must record the exact runtime pair.
The evaluation must measure accuracy and coverage separately.

| Gate | Minimum evidence |
|---|---|
| Source declaration | Exact byte range and syntax-role fixture |
| Scope resolution | Positive and negative visibility fixtures |
| Definition | Unique identity and correct target range |
| References | Complete declared domain and all expected locations |
| Rename | Identity preservation, no capture, and edit preconditions |
| Generated symbol | Compatible compiler evidence and source provenance |
| Framework adapter | Package compatibility and adversarial fixtures |

SEM-021 forbids release claims broader than the verified corpus and declared supported domain.
A comparison against another LSP does not establish ground truth by itself.
Performance improvements cannot excuse changed meaning or unsupported completeness claims.

## Sources

Sources establish language behavior. Requirement IDs define Quicksilver's implementation obligations.

- [S1] Elixir Tree-sitter grammar: <https://github.com/elixir-lang/tree-sitter-elixir>
- [S2] Elixir special forms, scope, aliases, and quotation: <https://elixir.hexdocs.pm/Kernel.SpecialForms.html>
- [S3] Elixir `Kernel`, definitions and language constructs: <https://elixir.hexdocs.pm/Kernel.html>
- [S4] Elixir `Macro`, syntax and hygiene metadata: <https://elixir.hexdocs.pm/Macro.html>
- [S5] Elixir `Code`, documentation and compiler tracers: <https://elixir.hexdocs.pm/Code.html>
- [S6] Mix cross-reference tasks and dependency categories: <https://mix.hexdocs.pm/Mix.Tasks.Xref.html>
- [S7] Elixir typespecs: <https://elixir.hexdocs.pm/typespecs.html>
- [S8] Erlang Dialyzer: <https://www.erlang.org/doc/apps/dialyzer/dialyzer.html>

The fixture matrix must pin versions. These documentation URLs describe their current published versions.
