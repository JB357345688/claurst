# TASK-M9-01 Preflight Report

## Ticket
`TASK-M9-01`

## Timestamp UTC
`20260413T045856Z`

## Branch
`feature/provider-resolution-seam`

## Verdict
`READY-WITH-NOTES`

## Repo State Summary
- `git branch --show-current` -> `feature/provider-resolution-seam`
- `git status --short --branch` shows no tracked modifications, no staged changes, and substantial untracked workspace noise under `.codex/`, `docs/Current/`, `docs/Orchestrator_planning/`, `docs/archive/provider_orchestrator/`, `docs/archive/reports/`, and `src-rust/target/`
- `git diff --name-only` -> empty
- `git diff --cached --name-only` -> empty
- `git log --oneline --decorate -n 20` shows `HEAD` at `b5b6dd4 (HEAD -> feature/provider-resolution-seam) TASK-M8-11 reconcile M8 workspace validation and formatting`, with the immediate prior chain aligned to M8 ticket history
- Review-basis implication for later execution: tracked tree is clean, but untracked noise should remain outside any later M9-01 patch/review basis

## Authority Reviewed
- `AGENTS.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md`

## Verified Files / Symbols / Commands
- Files:
  - `AGENTS.md`
  - `docs/Current/MPWO_WORK_ORDER_PACK.md`
  - `src-rust/crates/query/src/provider_resolution.rs`
  - `src-rust/crates/api/src/model_registry.rs`
  - `docs/archive/reports/TASK-M7-05_EXECUTION_REPORT_20260411T230547Z.md`
  - `docs/archive/reports/TASK-M7-05_REVIEW_REPORT_20260411T231155Z.md`
  - `docs/archive/reports/TASK-M8-11_REVIEW_REPORT_20260413T002956Z.md`
  - `docs/archive/reports/TASK-M8-11_CLOSEOUT_REPORT_20260413T003451Z.md`
  - `docs/archive/reports/TASK-M8-11_COMMIT_VERIFICATION_REPORT_20260413T003542Z.md`
- Symbols:
  - `resolve_provider_identity`
  - `#[cfg(test)] mod tests`
  - `assert_identity`
  - `assert_provider_model_conflict`
  - `ModelRegistry::new`
  - `ModelRegistry::find_provider_for_model`
  - `normalize_ollama_api_base`
- Commands:
  - `git branch --show-current`
  - `git status --short --branch`
  - `git diff --name-only`
  - `git diff --cached --name-only`
  - `git log --oneline --decorate -n 20`

## Dependency Baseline Confirmed
- `docs/Current/MPWO_WORK_ORDER_PACK.md` marks `TASK-M9-01` as depending on `M8-11`
- Current branch reality is compatible with starting `M9-01` after accepted `M8-11`:
  - `HEAD` is the `TASK-M8-11` closeout commit `b5b6dd4`
  - `docs/archive/reports/TASK-M8-11_REVIEW_REPORT_20260413T002956Z.md` records verdict `PASS`
  - `docs/archive/reports/TASK-M8-11_COMMIT_VERIFICATION_REPORT_20260413T003542Z.md` records verdict `VERIFIED` and confirms clean tracked state after the closeout commit
- Hosted Ollama compatibility baseline preserved
- The standing hosted-Ollama invariant from MPWO section 2A remains a preserved background constraint for this ticket and does not widen `TASK-M9-01` scope

## Exact M9-01 Contract
- Objective: verify all 12 precedence-matrix rows have passing unit tests and fill any gaps left from `M7-05`
- Owned scope: `src-rust/crates/query/src/provider_resolution.rs`, local `#[cfg(test)]` only
- Later execution steps authorized by the work order:
  1. enumerate existing tests against `P1-P12`
  2. add any missing rows
  3. run `cargo test -p claurst-query -- provider_resolution`
- Constraints:
  - unit tests only
  - no integration tests
  - no widening into other M9 tickets
- Validation target later: all 12 precedence-row tests pass

## Current Code Reality
- `src-rust/crates/query/src/provider_resolution.rs` exists at the expected path
- `resolve_provider_identity(...)` exists and is currently defined at line `101`
- Existing `#[cfg(test)] mod tests` exists at line `254`
- The existing test module already contains helper functions used for the precedence audit:
  - `assert_identity`
  - `assert_provider_model_conflict`
- The same module already contains twelve explicitly named precedence-row tests:
  - `p1_explicit_provider_matches_model_prefix`
  - `p2_explicit_provider_with_bare_model`
  - `p3_explicit_provider_conflicts_with_model_prefix`
  - `p4_no_provider_with_known_model_prefix`
  - `p5_explicit_provider_conflicts_with_reverse_model_prefix`
  - `p6_explicit_anthropic_pin_with_bare_model`
  - `p7_no_provider_with_unknown_namespace_defaults`
  - `p8_no_provider_bare_model_registry_resolves`
  - `p9_no_provider_bare_model_registry_has_no_match`
  - `p10_no_provider_without_model_registry_defaults`
  - `p11_explicit_provider_with_nested_slash_model`
  - `p12_local_provider_with_bare_model`
- The owned unit-test surface remains the smallest correct surface for later M9-01 execution
- Adjacent test code in the module also covers `normalize_ollama_api_base(...)` and `materialize_provider(...)`, but those are not part of the active M9-01 audit surface

## P1-P12 Coverage Matrix Audit

| P-row | Intended scenario summary | Existing test name(s) | Status | Basis for classification |
|---|---|---|---|---|
| `P1` | Explicit provider matches model prefix | `p1_explicit_provider_matches_model_prefix` | `COVERED` | Asserts `Some("openai")` plus `openai/gpt-4o` resolves to provider `openai`, model `gpt-4o`, source `ExplicitProvider` |
| `P2` | Explicit provider with bare model | `p2_explicit_provider_with_bare_model` | `COVERED` | Asserts explicit `openai` wins for bare `gpt-4o` and preserves `ExplicitProvider` source |
| `P3` | Explicit provider conflicts with known model prefix | `p3_explicit_provider_conflicts_with_model_prefix` | `COVERED` | Uses exact conflict input from the matrix and asserts `ProviderModelConflict` details |
| `P4` | No provider, model has known prefix | `p4_no_provider_with_known_model_prefix` | `COVERED` | Asserts `google/gemini-2.5-flash` resolves via `ModelStringPrefix` with stripped model id |
| `P5` | Reverse conflict case of `P3` | `p5_explicit_provider_conflicts_with_reverse_model_prefix` | `COVERED` | Asserts explicit `anthropic` plus `openai/gpt-4o` returns the mirrored `ProviderModelConflict` case |
| `P6` | Explicit anthropic pin with bare model | `p6_explicit_anthropic_pin_with_bare_model` | `COVERED` | Asserts explicit anthropic pin preserves provider/model and `ExplicitProvider` source |
| `P7` | No provider, model has unknown namespace | `p7_no_provider_with_unknown_namespace_defaults` | `COVERED` | Exercises `meta-llama/Llama-3.3-70B` with no registry and asserts default anthropic fallback, matching the no-registry branch of the matrix |
| `P8` | No provider, bare model, registry resolves | `p8_no_provider_bare_model_registry_resolves` | `COVERED` | Exercises the registry-resolution branch using `ModelRegistry::new()` and `gemini-3-flash-preview`; result source is asserted as `ModelRegistry`. Note: this relies on current built-in registry/heuristic behavior rather than an explicit test-local registration mutator |
| `P9` | No provider, bare model, registry present but no match | `p9_no_provider_bare_model_registry_has_no_match` | `COVERED` | Uses `ModelRegistry::new()` with an unknown bare model and asserts default fallback when lookup returns no provider |
| `P10` | No provider and no model registry | `p10_no_provider_without_model_registry_defaults` | `COVERED` | Asserts no-registry default fallback for a bare anthropic model |
| `P11` | Explicit provider with nested-slash model | `p11_explicit_provider_with_nested_slash_model` | `COVERED` | Asserts `openrouter/meta-llama/Llama-3.3-70B` strips only the leading provider prefix and preserves the nested remainder |
| `P12` | Local provider (`ollama`) with bare model | `p12_local_provider_with_bare_model` | `COVERED` | Asserts explicit local provider precedence for bare `llama3` with `ExplicitProvider` source |

Coverage summary:
- Covered rows: `P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12`
- Missing rows: none
- Redundant or overlapping tests:
  - `P3` and `P5` are intentionally mirrored conflict directions, not accidental duplication
  - `P7`, `P9`, and `P10` all end in default fallback but exercise materially different preconditions, so they are not redundant
  - `P2`, `P6`, and `P12` are all explicit-provider bare-model cases, but they target distinct matrix rows and provider classes
- Mismatched names or assertions:
  - No naming/assertion mismatch was found that would make a current row classification incorrect
  - `P8` carries a coverage-quality note, not a semantic mismatch: the test is valid for branch coverage, but it depends on current public `ModelRegistry::new()` behavior instead of explicit per-test registry setup

## Likely Smallest Edit Surface For Execution
- Smallest correct surface remains the existing `#[cfg(test)]` module in `src-rust/crates/query/src/provider_resolution.rs`
- Based on current read-only evidence, later execution may require no source edit at all if the goal is only to confirm coverage and run the filtered test command
- If a follow-up execution decides to tighten `P8` semantics, that adjustment should still remain inside the existing local test module only

## Drift Found
- No file-path drift found
- No symbol/function drift found
- No missing or relocated `#[cfg(test)]` section found
- No evidence that the owned unit-test surface has moved or that production code must be touched for M9-01
- Historical line-shape drift only:
  - the original `M7-05` instructions expected adding a new bottom-of-file test module
  - repo reality already has an existing bottom-of-file `#[cfg(test)]` module, and `M7-05` already populated it
  - this is non-blocking drift

## Blockers
- None for preflight
- No structural blocker was found that would prevent later M9-01 execution as written

## Notes
- `TASK-M7-05` appears to have already added the required `P1-P12` tests, so `TASK-M9-01` currently reads as an audit/confirmation pass rather than an obvious missing-coverage implementation ticket
- The main carry-forward note is `P8`: it is covered, but its setup depends on current `ModelRegistry::new()` resolution behavior rather than an explicit test-local registration API
- The untracked workspace/report noise does not block the ticket, but it should be called out again in any later execution/review phase so the review basis stays scope-clean
