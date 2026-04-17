# TASK-M11-12 Preflight Report

## 1. ticket id

`TASK-M11-12`

This preflight is for revised `TASK-M11-12 = D2 test suite + workspace validation`, not the stale older numbering where this work appeared as `M11-11`.

## 2. verdict

`HALT`

## 3. timestamp UTC

`2026-04-15T05:47:38Z`

## 4. branch / HEAD observed

- Branch: `feature/provider-resolution-seam`
- HEAD: `0c9dac407e82fccdfe16337bc2c05a6aeb816ca5`
- Expected accepted latest HEAD: `0c9dac407e82fccdfe16337bc2c05a6aeb816ca5`
- Match result: exact
- Working tree: noisy / not clean
  - tracked modification: `.gitignore`
  - many untracked docs artifacts, `.codex`, and `src-rust/target/`
  - `git diff --name-only HEAD -- src-rust` returned no paths, so there is no tracked live source drift under `src-rust` relative to accepted `HEAD`

## 5. authority reviewed

- `AGENTS.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md`
- `docs/Current/TASK-M11-10_SPLIT_PLAN_AND_TEMPORARY_AUTHORITY_AMENDMENT.md`
- `docs/archive/reports/M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md`
- `docs/archive/reports/MPWO_REVISION_REPORT_20260414T003218Z.md`
- `docs/archive/reports/TASK-M11-08R_CLOSEOUT_REPORT_20260415T004401Z.md`
- `docs/archive/reports/TASK-M11-08B_CLOSEOUT_REPORT_20260415T011952Z.md`
- `docs/archive/reports/TASK-M11-09_CLOSEOUT_REPORT_20260415T014220Z.md`
- `docs/archive/reports/TASK-M11-10A_CLOSEOUT_REPORT_20260415T024141Z.md`
- `docs/archive/reports/TASK-M11-10B1_CLOSEOUT_REPORT_20260415T034453Z.md`
- `docs/archive/reports/TASK-M11-10B2_CLOSEOUT_REPORT_20260415T041344Z.md`
- `docs/archive/reports/M11_CONVERGENCE_REVIEW_20260415T042825Z.md`
- `docs/archive/reports/TASK-M11-11_CLOSEOUT_REPORT_20260415T053856Z.md`
- `docs/archive/reports/TASK-M11_D2_RECON_REPORT_20260414T012649Z.md`

## 6. accepted-baseline comparison

- All supplied accepted baseline commits were verified as present in local git object storage:
  - `dc772aa`
  - `fe21969`
  - `828b08e`
  - `eb26773`
  - `1472024`
  - `cf8201f`
  - `0942e4a`
  - `25518cac29d34353cb58c8811da1040a3da69247`
  - `b4ad28ee9eb1e434f935fd2de70c4b402a5c74da`
  - `4ef9547dab51959f7b39c473f929b81f05ee1134`
  - `ea046c52da82dfd9778f4065bd36b36e28d73c8a`
  - `3812df04ec87ce0e96f851da2d18ab38b24f0b99`
  - `bfabfd5934b0afe801f2e7de9f15a7f6258c563b`
  - `0c9dac407e82fccdfe16337bc2c05a6aeb816ca5`
- Accepted split baseline is present in live code and remains the controlling runtime target:
  - `08R` root `SessionBudget` wiring
  - `08B` inherited parent shared-session accounting
  - `09` child `max_tokens`
  - `10A` child `allow_fallback`
  - `10B1` layered child-budget seam
  - `10B2` child/team `budget_usd` carriage
  - `11` `QueryEvent` expansion / observability
- The landed layered `SessionBudget` model from `10B1/10B2` is the live validation baseline, not the stale older broad `M11-10` wording.

## 7. verified target files / symbols / commands

- Files inspected:
  - `src-rust/crates/api/src/provider_types.rs`
  - `src-rust/crates/query/src/provider_resolution.rs`
  - `src-rust/crates/query/src/health_cache.rs`
  - `src-rust/crates/query/src/session_budget.rs`
  - `src-rust/crates/query/src/lib.rs`
  - `src-rust/crates/query/src/agent_tool.rs`
  - `src-rust/crates/tools/src/team_tool.rs`
  - `src-rust/crates/tui/src/app.rs`
- Symbols / seams verified live:
  - `TrustDomain::for_provider`
  - `model_supports_capability`
  - `provider_supports_capability`
  - `resolve_provider_with_fallback`
  - `HealthCache::{new,get,insert,probe_if_stale}`
  - `SessionBudget::{new,child_scope,record_cost,check_and_cancel,child_cancel_token,shared_budget}`
  - `register_session_budget`
  - `session_budget_for_session`
  - `with_registered_session_budget`
  - `QueryConfig.session_budget`
  - `AgentInput.{max_tokens,allow_fallback,budget_usd}`
  - `AgentSpec.{max_tokens,allow_fallback,budget_usd}`
  - `AgentRunParams.{max_tokens_override,allow_fallback,budget_usd}`
  - `QueryEvent::{WorkerProviderResolved,WorkerBudgetExceeded,SessionBudgetExceeded}`
  - `TEAM_RUNNER_OBSERVABILITY_PREFIX`
  - `extract_team_runner_observability`
- Commands run:
  - `git branch --show-current`
  - `git rev-parse HEAD`
  - `git status --short --branch`
  - `git diff --name-only HEAD -- src-rust`
  - `cargo fmt --all -- --check`
  - `cargo build --workspace`
  - `cargo test --workspace`
  - `cargo clippy --workspace --all-targets -- -D warnings`
- Additional read-only classification probes run:
  - isolated `claurst-query` tests for:
    - `agent_tool::tests::teamcreate_mixed_providers_per_agent_dispatch`
    - `provider_resolution::tests::fallback_disabled_returns_suggestion_text`
    - `provider_resolution::tests::fallback_same_domain_returns_healthy_cloud_candidate`
    - `provider_resolution::tests::fallback_cross_domain_is_prohibited`
    - `tests::provider_registry_none_uses_legacy_anthropic_client_path`

## 8. exact scope confirmation in current repo reality

- Confirmed scope is revised `TASK-M11-12 = D2 test suite + workspace validation`.
- Confirmed this ticket should validate the landed split path, not reopen architecture:
  - no provider-resolution redesign
  - no hosted Ollama behavior change
  - no `ToolContext.session_budget`
  - no `ToolContext.health_cache`
  - no reopening of `10B1`, `10B2`, or `11`
- Live repo reality does not force an architecture split of the runtime baseline.
- Live repo reality does force a readiness distinction:
  - content scope can remain one narrow validation ticket
  - closure against the mandated workspace gates is currently blocked by baseline failures outside any new M11-12 test authoring

## 9. D2 validation coverage findings

- `TrustDomain` classification:
  - live in `crates/api/src/provider_types.rs`
  - directly testable now
  - no dedicated tests exist yet
- Same-domain fallback:
  - live in `resolve_provider_with_fallback(...)`
  - existing unit test exists and passes in isolated execution
- Cross-domain prohibition:
  - existing unit test exists and passes in isolated execution
- `allow_fallback = false`:
  - existing unit test exists and passes in isolated execution
- Capability matching helpers:
  - `model_supports_capability()` and `provider_supports_capability()` already have unit tests
- `HealthCache` TTL / probe behavior:
  - existing unit tests already cover cache hit/miss, TTL expiry, success caching, provider-error mapping, and timeout mapping
- Root `SessionBudget` check/cancel:
  - existing unit tests already cover threshold-triggered cancel and child-token cancellation
- Inherited parent shared-session accounting from `08B`:
  - existing `session_budget` unit tests cover parent/child and descendant spend propagation plus task-local nearest-budget lookup
  - child/team runtime carriage exists in `agent_tool.rs`, but direct spawn-path tests are not present yet
- Child `max_tokens` override from `09`:
  - runtime fields and wiring are present
  - no direct unit or integration tests exist yet
- Child `allow_fallback` from `10A`:
  - runtime fields and wiring are present
  - provider-resolution seam itself is tested
  - child spawn-path wiring is not directly tested yet
- Layered child `budget_usd` semantics from `10B1/10B2`:
  - `SessionBudget::child_scope(...)` behavior is directly unit-tested
  - child/team carriage via `agent_tool.rs` and `team_tool.rs` is live
  - direct carriage tests are not present yet
- `WorkerProviderResolved`, `WorkerBudgetExceeded`, `SessionBudgetExceeded`:
  - variants and emit paths are live
  - `tui/src/app.rs` already has no-op exhaustiveness handling
  - no dedicated unit tests currently cover parsing / emission / TeamCreate sanitization
- `#[ignore]` assessment:
  - no required D2 acceptance slice is inherently untestable against current repo reality
  - no mandatory `#[ignore]` cases were identified for the listed runtime slices
  - gaps are missing tests, not impossible tests

## 10. test-placement findings

- Live test-tree reality:
  - `src-rust/crates/query/tests/` does not exist
  - `src-rust/crates/api/tests/` does not exist
  - only top-level crate test trees currently found were:
    - `src-rust/crates/core/tests`
    - `src-rust/crates/tui/tests`
- Existing test style in the relevant crates is primarily inline unit tests:
  - `query/src/provider_resolution.rs`
  - `query/src/health_cache.rs`
  - `query/src/session_budget.rs`
  - `query/src/lib.rs`
  - `query/src/agent_tool.rs`
  - several `api/src/*` files also use inline `#[cfg(test)]`
- Narrowest realistic placement recommendation:
  - inline-only, split across the owning files
  - do not create `src-rust/crates/query/tests/` or `src-rust/crates/api/tests/` as the first choice
- Recommended placement by surface:
  - `crates/api/src/provider_types.rs`
    - add `TrustDomain::for_provider()` tests inline
  - `crates/query/src/provider_resolution.rs`
    - extend existing fallback / capability tests only if needed
  - `crates/query/src/health_cache.rs`
    - existing coverage already strong
  - `crates/query/src/session_budget.rs`
    - existing coverage already strong for root and layered budget math
  - `crates/query/src/agent_tool.rs`
    - add child spawn-path tests for `max_tokens`, `allow_fallback`, and `budget_usd` carriage if these cannot be better exercised from `lib.rs`
  - `crates/query/src/lib.rs`
    - add `QueryEvent` / observability extraction / session-budget event tests inline
- Important placement note:
  - the failing direct `TeamCreateTool` test demonstrates that raw tool output now carries observability markers and is sanitized by query-side logic
  - for `M11-12`, event-expansion tests belong primarily in `query`-owned surfaces, not in `tools`-only assertions against raw output

## 11. workspace-gate findings

- `cargo fmt --all -- --check`
  - result: `FAIL`
  - observed failure class: formatting-only diffs
  - affected live files included:
    - `crates/api/src/provider_types.rs`
    - `crates/query/src/agent_tool.rs`
    - `crates/query/src/health_cache.rs`
    - `crates/query/src/lib.rs`
    - `crates/query/src/provider_resolution.rs`
  - classification: `pre-existing baseline drift`

- `cargo build --workspace`
  - result: `PASS`

- `cargo test --workspace`
  - result: `FAIL`
  - primary direct failure:
    - `agent_tool::tests::teamcreate_mixed_providers_per_agent_dispatch`
    - failure reason: raw team output now includes `[[CLAURST_QUERY_OBS:...]]` metadata after `TASK-M11-11`, but the test still expects the pre-observability plain string
  - subsequent failures in the same run:
    - several `provider_resolution` and `lib.rs` tests failed with `PoisonError`
    - those failures come from `with_isolated_provider_auth(...).lock().unwrap()` after the first panic poisoned the shared test mutex
  - isolated re-runs showed:
    - `fallback_disabled_returns_suggestion_text`: `PASS`
    - `fallback_same_domain_returns_healthy_cloud_candidate`: `PASS`
    - `fallback_cross_domain_is_prohibited`: `PASS`
    - `provider_registry_none_uses_legacy_anthropic_client_path`: `PASS`
  - classification:
    - direct teamcreate assertion failure: `ticket-relevant risk` on the accepted `M11-11` observability seam
    - poison-lock cascades: `out-of-scope noise` secondary to that first failure

- `cargo clippy --workspace --all-targets -- -D warnings`
  - result: `FAIL`
  - failure occurs in `crates/core` before any M11-12-specific test additions
  - representative failures included:
    - `clippy::collapsible-match`
    - `clippy::redundant-pattern-matching`
    - `clippy::unnecessary-map-or`
    - `clippy::new-without-default`
    - `clippy::manual-strip`
    - `clippy::should-implement-trait`
    - `clippy::field-reassign-with-default`
  - classification: `pre-existing baseline drift` and `out-of-scope workspace blocker`
  - realism assessment:
    - the proposed full-workspace clippy gate is not currently realistic as a required M11-12 closure gate without separate baseline cleanup or explicit authority for a narrower invocation

## 12. drift found

- `docs/Current/MPWO_WORK_ORDER_PACK.md` is still a revision-summary document rather than a detailed executable pack
  - this is documentation-shape drift already noted by earlier reviews
  - current prompt plus accepted split closeouts provide sufficient controlling interpretation
- Workspace gate drift exists before M11-12 starts:
  - rustfmt baseline not clean
  - one `claurst-query` test is stale against accepted `M11-11` behavior
  - full-workspace clippy is blocked by unrelated `claurst-core` lint debt
- No structural code drift was found against the accepted split runtime baseline itself

## 13. blockers, if any

- `yes`
- Blocker 1:
  - mandatory workspace validation is already failing before new M11-12 tests are authored
- Blocker 2:
  - `cargo clippy --workspace --all-targets -- -D warnings` is blocked by unrelated `claurst-core` lint debt, so M11-12 cannot realistically close against that gate as-written without non-ticket cleanup or explicit authority change
- Blocker 3:
  - `cargo test --workspace` currently fails because a query test still expects pre-`M11-11` TeamCreate output without observability payload markers

## 14. hosted Ollama invariant assessment

`preserved`

Basis:

- this was a read-only preflight; no runtime behavior changed
- same-domain fallback behavior remains on the accepted `TASK-M11-05` seam and isolated fallback tests passed
- no hosted-Ollama-specific request shaping or auth logic was touched
- no reopening of provider-resolution policy is required for the M11-12 test plan
- all required D2 acceptance slices appear locally testable with fake providers and local harnesses, so hosted Ollama live behavior does not need to be exercised to satisfy M11-12

## 15. exact recommendation for next step

- Do not start M11-12 implementation yet under the current gate wording.
- Recommendation:
  - keep revised `TASK-M11-12` as one narrow validation ticket on the corrected accepted path
  - do not split its content into separate test-authoring and validation tickets
  - but halt execution until authority explicitly resolves baseline gate blockers
- The next required decision is not another docs-only correction.
  - Convergence-review wording plus this preflight are sufficient to define M11-12 against the accepted split baseline.
  - Another docs-only correction is not required first.
- The next authority action should explicitly choose one of:
  - separate baseline-cleanup work before M11-12, covering:
    - rustfmt drift
    - stale `claurst-query` TeamCreate observability test
    - unrelated workspace clippy debt
  - or an explicit narrowed validation gate for M11-12, especially for clippy, if the project intends M11-12 to remain scoped to D2 tests only
- Once that gate issue is resolved, the narrowest M11-12 implementation plan is:
  - add inline tests in `api/provider_types.rs` for `TrustDomain`
  - add / extend inline tests in `query` for:
    - child `max_tokens`
    - child `allow_fallback`
    - child `budget_usd` carriage
    - `QueryEvent` extraction / emission / sanitization
  - keep `src-rust/crates/query/tests/` absent unless an end-to-end harness proves truly necessary
