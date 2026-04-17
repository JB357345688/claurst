# TASK-M11-12 Execution Report

## 1. ticket id

`TASK-M11-12`

This execution is for revised `TASK-M11-12 = D2 test suite + workspace validation` under the clarified narrowed gate.

## 2. execution verdict

`PASS-WITH-NOTES`

## 3. timestamp UTC

`2026-04-15T06:19:25Z`

## 4. branch / HEAD before execution

- Branch: `feature/provider-resolution-seam`
- HEAD before execution: `0c9dac407e82fccdfe16337bc2c05a6aeb816ca5`
- Expected branch / HEAD from prompt matched exactly before editing.

## 5. authority reviewed

- `AGENTS.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md`
- `docs/Current/TASK-M11-10_SPLIT_PLAN_AND_TEMPORARY_AUTHORITY_AMENDMENT.md`
- `docs/archive/reports/M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md`
- `docs/archive/reports/MPWO_REVISION_REPORT_20260414T003218Z.md`
- `docs/archive/reports/M11_CONVERGENCE_REVIEW_20260415T042825Z.md`
- `docs/archive/reports/TASK-M11-11_CLOSEOUT_REPORT_20260415T053856Z.md`
- `docs/archive/reports/TASK-M11-12_PREFLIGHT_REPORT_20260415T054738Z.md`
- `docs/archive/reports/TASK-M11-12_GATE_CLARIFICATION_REPORT_20260415T060015Z.md`

## 6. files changed

- `src-rust/crates/api/src/provider_types.rs`
- `src-rust/crates/query/src/agent_tool.rs`
- `src-rust/crates/query/src/lib.rs`

Patch hygiene note:

- Intended active diff is limited to the three files above.
- Unrelated repo noise remains present in untracked docs artifacts, `.codex`, `.gitignore`, and `src-rust/target/`, but was not touched for this ticket.

## 7. exact test/validation changes made

- Added inline `TrustDomain::for_provider()` tests in `provider_types.rs` covering the accepted local aliases and cloud fallthrough.
- Added inline `agent_tool.rs` tests for:
  - direct child `max_tokens` override propagation
  - child `allow_fallback` propagation to same-domain provider fallback
  - inherited shared-session budget reuse when child `budget_usd` is absent
  - layered child-local budget creation when child `budget_usd` is present
  - `WorkerBudgetExceeded` metadata emission for child-local budget overruns
- Updated the stale query-side `teamcreate_mixed_providers_per_agent_dispatch` assertion so it validates accepted `TASK-M11-11` behavior:
  - raw TeamCreate per-agent output now expects the `[[CLAURST_QUERY_OBS:...]]` suffix
  - the test now parses and validates the encoded observability payload instead of rejecting it
  - the same test now also validates per-agent `max_tokens` override propagation on the team-runner path
- Added inline `lib.rs` tests for:
  - TeamCreate observability sanitization plus emission of `WorkerProviderResolved` and `WorkerBudgetExceeded`
  - `SessionBudgetExceeded` emission only when a shared session budget newly crosses its limit
- Applied rustfmt to the touched files only; no runtime logic was changed.

## 8. D2 acceptance coverage summary

Directly covered after this execution, using the accepted split baseline (`08R`, `08B`, `09`, `10A`, `10B1`, `10B2`, `11`):

1. `trust-domain classification`
   - Newly covered by `provider_types::tests::trust_domain_marks_local_provider_aliases_as_local`
   - Newly covered by `provider_types::tests::trust_domain_marks_non_local_providers_as_cloud`
2. `same-domain fallback`
   - Existing direct coverage in `provider_resolution::tests::fallback_same_domain_returns_healthy_cloud_candidate`
   - Newly covered on the child runtime seam in `agent_tool::tests::agent_tool_allow_fallback_uses_same_domain_provider`
3. `cross-domain prohibition`
   - Existing direct coverage in `provider_resolution::tests::fallback_cross_domain_is_prohibited`
4. `allow_fallback = false`
   - Existing direct coverage in `provider_resolution::tests::fallback_disabled_returns_suggestion_text`
5. `HealthCache TTL / probe behavior`
   - Existing direct coverage in `health_cache::*` unit tests including TTL expiry and `probe_if_stale(...)`
6. `root SessionBudget check/cancel`
   - Existing direct coverage in `session_budget::tests::check_and_cancel_triggers_at_threshold`
   - Existing direct coverage in `session_budget::tests::child_token_is_cancelled_with_root`
7. `inherited parent shared-session accounting from 08B`
   - Existing direct coverage in `session_budget::tests::registered_budget_is_visible_for_session`
   - Existing direct coverage in `session_budget::tests::task_local_scope_prefers_nearest_active_budget`
   - Newly covered on the query-owned child seam in `agent_tool::tests::child_session_budget_reuses_inherited_budget_when_child_limit_absent`
8. `child max_tokens override from 09`
   - Newly covered in `agent_tool::tests::agent_tool_respects_max_tokens_override`
   - Newly covered on the team-runner path in `agent_tool::tests::teamcreate_mixed_providers_per_agent_dispatch`
9. `child allow_fallback from 10A`
   - Newly covered in `agent_tool::tests::agent_tool_allow_fallback_uses_same_domain_provider`
10. `layered child budget_usd semantics from 10B1/10B2`
    - Existing direct coverage in `session_budget::tests::child_scope_records_against_local_and_parent_budget`
    - Existing direct coverage in `session_budget::tests::descendant_scope_chains_all_active_budget_caps`
    - Newly covered on the query-owned child seam in `agent_tool::tests::child_session_budget_wraps_parent_when_child_limit_present`
11. `WorkerProviderResolved`
    - Newly covered in `tests::teamcreate_observability_is_sanitized_and_emitted`
    - Newly validated at the raw team-runner seam in `agent_tool::tests::teamcreate_mixed_providers_per_agent_dispatch`
12. `WorkerBudgetExceeded`
    - Newly covered in `agent_tool::tests::worker_budget_exceeded_event_reports_child_limit`
    - Newly covered in `tests::teamcreate_observability_is_sanitized_and_emitted`
13. `SessionBudgetExceeded`
    - Newly covered in `tests::session_budget_exceeded_event_emits_only_on_new_cancellation`

Coverage gap assessment:

- No required acceptance slice was silently skipped.
- No extra `src-rust/crates/query/tests/` or `src-rust/crates/api/tests/` tree was needed; all additions remained inline on the owning files.

## 9. blocking validation commands run

- `cd src-rust && cargo build --workspace`
- `cd src-rust && cargo test -p claurst-api`
- `cd src-rust && cargo test -p claurst-query`

## 10. blocking validation results

- `cargo build --workspace`: `PASS`
  - finished successfully in the workspace dev profile
- `cargo test -p claurst-api`: `PASS`
  - `32 passed; 0 failed`
- `cargo test -p claurst-query`: `PASS`
  - `138 passed; 0 failed`
  - note: test output still reports a pre-existing warning in `crates/query/src/compact.rs` for an unused `Role` import, but the blocking test gate passed cleanly

## 11. informational probe results

- `cd src-rust && cargo fmt --all -- --check`
  - result: `PASS`
- `cd src-rust && cargo clippy --workspace --all-targets -- -D warnings`
  - result: `FAIL`
  - classification: non-blocking baseline debt under the clarified narrowed gate
  - first failing crate: `claurst-core`
  - representative failure classes recorded:
    - `clippy::collapsible-match` / `clippy::collapsible-if`
    - `clippy::redundant-pattern-matching`
    - `clippy::unnecessary-map-or`
    - `clippy::new-without-default`
    - `clippy::manual-strip`
    - `clippy::derivable-impls`
    - `clippy::field-reassign-with-default`
  - representative failing files recorded:
    - `crates/core/src/session_storage.rs`
    - `crates/core/src/attachments.rs`
    - `crates/core/src/feature_flags.rs`
    - `crates/core/src/skill_discovery.rs`
    - `crates/core/src/lib.rs`
    - `crates/core/src/bash_classifier.rs`
    - `crates/core/src/system_prompt.rs`

## 12. deviations from ticket, if any

- No scope deviation in implementation.
- The execution remained limited to revised `TASK-M11-12 = D2 test suite + workspace validation` under the clarified narrowed gate.
- Workspace clippy remains failing, but this is explicitly recorded as informational and non-blocking for this ticket per the clarified acceptance authority.

## 13. blockers, if any

- No blocking ticket-local blocker remains.
- Non-blocking note only: workspace clippy still surfaces unrelated `claurst-core` lint debt outside the M11-12 validation delta.

## 14. hosted Ollama invariant assessment

`preserved`

Basis:

- This execution changed tests only.
- No runtime provider-resolution logic was changed.
- No hosted Ollama request-shaping, auth, or fallback behavior was changed.
- `provider_resolution.rs` and `health_cache.rs` runtime code remained untouched.

## 15. ready for verification

`yes`
