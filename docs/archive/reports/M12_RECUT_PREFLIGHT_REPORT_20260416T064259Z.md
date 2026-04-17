# M12 Recut Preflight Report

## 1. Ticket ID

`M12-RECUT-PREFLIGHT`

## 2. Verdict

`M12-SATISFIED-BY-AUDIT`

## 3. Timestamp UTC

`20260416T064259Z`

## 4. Branch / HEAD / worktree summary

- Branch: `feature/provider-resolution-seam`
- HEAD: `63595c387ac8fd2f5adbf9cf75d45a724153c3db`
- HEAD subject: `docs(authority): recast M12 as post-M11 coverage audit`
- `63595c387ac8fd2f5adbf9cf75d45a724153c3db` is present and is the current `HEAD`: `yes`
- Accepted post-M11 baseline referenced by the live pack: `b157924e130fdf71c09a3787b47dd5eb1f31d542`

Current worktree state is noisy and must be treated as explicit scope noise for later attribution:

- tracked modification: `.gitignore`
- tracked deletion in worktree: `docs/Current/TASK-M11-10_SPLIT_PLAN_AND_TEMPORARY_AUTHORITY_AMENDMENT.md`
- untracked `docs/Current/*` files, including `MPWO_WORK_ORDER_PACK_pre_M10_revision.md` and `RFC_PROVIDER_AWARE_WORKER_FABRIC_v3.1.md`
- untracked archive/report material under `docs/archive/reports/`
- untracked archive-only surrogate context under `docs/archive/provider_orchestrator/`
- untracked `src-rust/target/`

Preflight assessment: this noise can confuse later closure/review scope, but it does not block this audit because the active authority file is explicit and the D2 source/test surfaces inspected below are not locally modified.

## 5. Authority reviewed

Controlling authority reviewed:

- `AGENTS.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md`

Historical/context evidence reviewed only as non-controlling traceability:

- `docs/archive/provider_orchestrator/d2_test_micro_patch_report.md`
- `docs/archive/provider_orchestrator/provider_reconciliation_report.md`
- `docs/archive/reports/M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md`
- `docs/archive/reports/TASK-M11-08B_CLOSEOUT_REPORT_20260415T011952Z.md`
- `docs/archive/reports/TASK-M11-10A_CLOSEOUT_REPORT_20260415T024141Z.md`
- `docs/archive/reports/TASK-M11-10B2_CLOSEOUT_REPORT_20260415T041344Z.md`
- `docs/archive/reports/TASK-M11-11_CLOSEOUT_REPORT_20260415T053856Z.md`
- `docs/archive/reports/TASK-M11-12_CLOSEOUT_REPORT_20260415T065244Z.md`

Verified live files / symbols:

- `src-rust/crates/api/src/provider_types.rs`
  - `TrustDomain`
  - `TrustDomain::for_provider()`
- `src-rust/crates/api/src/model_registry.rs`
  - `find_provider_for_model()`
  - `list_by_provider()`
  - `best_model_for_provider()`
- `src-rust/crates/query/src/provider_resolution.rs`
  - `ProviderResolutionError`
  - `resolve_provider_identity()`
  - `materialize_provider()`
  - `resolve_provider_with_fallback()`
- `src-rust/crates/query/src/health_cache.rs`
  - `HealthCache`
  - `HealthCache::probe_if_stale()`
- `src-rust/crates/query/src/session_budget.rs`
  - `SessionBudget`
  - `SessionBudget::child_scope()`
  - `register_session_budget()`
  - `session_budget_for_session()`
  - `with_registered_session_budget()`
- `src-rust/crates/query/src/agent_tool.rs`
  - `child_session_budget()`
  - `worker_provider_resolved_event()`
  - `worker_budget_exceeded_event()`
  - `AgentInput.allow_fallback`
  - `AgentInput.budget_usd`
- `src-rust/crates/tools/src/team_tool.rs`
  - `AgentRunParams.allow_fallback`
  - `AgentRunParams.budget_usd`
  - `AgentSpec.allow_fallback`
  - `AgentSpec.budget_usd`
- `src-rust/crates/query/src/lib.rs`
  - `QueryConfig.session_budget`
  - `QueryEvent::{WorkerProviderResolved, WorkerBudgetExceeded, SessionBudgetExceeded}`
  - `emit_tool_observability_events()`
  - `emit_session_budget_exceeded()`
  - root registry no-legacy-fallback test

Commands/checks run:

- git branch / HEAD / status / commit-presence checks
- `rg` over `docs/Current`, `docs/archive`, `src-rust/crates/api/src/`, `src-rust/crates/query/src/`, `src-rust/crates/tools/src/`
- direct file inspections with `nl -ba ... | sed -n ...`
- targeted test execution:
  - `cargo test -p claurst-api trust_domain`
  - `cargo test -p claurst-query fallback`
  - `cargo test -p claurst-query session_budget`
  - `cargo test -p claurst-query health_cache`
  - `cargo test -p claurst-query teamcreate_mixed_providers_per_agent_dispatch`
  - `cargo test -p claurst-query teamcreate_observability_is_sanitized_and_emitted`
  - `cargo test -p claurst-query explicit_provider_conflicts`

Drift found:

- The deleted split-era authority file still appears as a tracked deletion in worktree state, but the live pack explicitly demotes it and treats it as deleted/non-controlling in this checkout.
- The archive-only surrogate docs are present as untracked worktree material, but the live pack explicitly references them as historical evidence only.

Blockers:

- none

## 6. Recast-M12 authority confirmation

Confirmed:

- `docs/Current/MPWO_WORK_ORDER_PACK.md` is the live active authority artifact in this checkout.
- The pack explicitly states it is the single / sole active authority artifact in `docs/Current/`.
- The pack contains the recast M12 wording:
  - surrogate is archive-only
  - no live tracked surrogate D2 test exists
  - M12 is now a live D2 coverage-audit / closure-decision milestone
  - any real remaining delta must become a separate follow-on ticket

Confirmed no newer conflicting current-authority artifact in `docs/Current/`:

- `docs/Current/IMPLEMENTATION_PLAN_MPWO.md` explicitly marks itself historical / non-active
- `docs/Current/D1_REVIEW_REPORT_20260413T233604Z.md` and `docs/Current/M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md` are historical evidence only per the live pack
- `docs/Current/TASK-M11-10_SPLIT_PLAN_AND_TEMPORARY_AUTHORITY_AMENDMENT.md` is deleted in the current worktree and explicitly demoted by the live pack
- no other inspected `docs/Current/*` file asserts controlling authority over the pack

Authority contradiction / sequencing blocker check:

- none found

## 7. Reconstructed old surrogate intent

The old surrogate was not trying to reopen architecture. It was trying to preserve one narrow D2 behavioral claim before a real query-layer D2 seam existed:

- D2 fallback candidate scope must be determined from real registered providers, not from a hardcoded cloud-only allowlist.
- Trust-domain classification must treat built-in local providers as `Local` and everything else, including unknown/custom provider IDs, as `Cloud`.
- Same-domain candidate scope must therefore include healthy cloud providers even when the provider ID is custom/unknown.
- Local providers must not enter the cloud fallback candidate set.
- Unhealthy providers must be excluded from fallback candidacy.
- Providers that do not satisfy the required capability floor, especially tool-calling, must be excluded from fallback candidacy.

That surrogate intent is grounded by:

- the live pack’s statement that the surrogate is archive-only historical context
- `docs/archive/provider_orchestrator/d2_test_micro_patch_report.md`, which shows the exact surrogate assertions
- `docs/archive/provider_orchestrator/provider_reconciliation_report.md`, which explains why the surrogate existed at all: real D2 production seams were absent at that earlier point
- the accepted M10 D2 plan and M11 closeout trail, which define the intended replacement surfaces:
  - `TrustDomain`
  - `resolve_provider_with_fallback()`
  - `HealthCache`
  - child `allow_fallback`
  - child / session budget seams
  - D2 validation and observability

In concrete terms, the old surrogate was trying to prove:

- "when fallback exists, a custom provider should default to cloud-domain candidacy rather than being dropped from the D2 routing set,"
- "same-domain fallback must be health- and capability-filtered,"
- "local-to-cloud escape must not happen by accident."

## 8. Live D2 coverage audit

### API surfaces

- Trust-domain classification is live in `src-rust/crates/api/src/provider_types.rs:241-253`.
- Direct trust-domain tests are live in `src-rust/crates/api/src/provider_types.rs:256-287`.
- Fallback model selection prerequisites are live in `src-rust/crates/api/src/model_registry.rs:281-339` and `:342-380` via:
  - `find_provider_for_model()`
  - `list_by_provider()`
  - `best_model_for_provider()`

### Query provider-resolution surfaces

- Provider/model conflict handling is live in `src-rust/crates/query/src/provider_resolution.rs:121-139` and `:146-158`.
- Root resolution identity seam is live in `src-rust/crates/query/src/provider_resolution.rs:141-195`.
- Materialization seam is live in `src-rust/crates/query/src/provider_resolution.rs:197-255`.
- D2 fallback seam is live in `src-rust/crates/query/src/provider_resolution.rs:306-389`.
  - same-domain filter: `:328-339`
  - health-aware candidate ordering: `:329-355`
  - capability-aware model selection: `:257-304`, `:366-374`
  - fallback-disabled error path: `:322-325`
  - cross-domain exclusion: `:337-338`
- Root registry-backed no-legacy-fallback behavior remains live in `src-rust/crates/query/src/lib.rs:1095-1125`, with direct regression at `src-rust/crates/query/src/lib.rs:2701-2732`.

### Health / budget surfaces

- `HealthCache` is live in `src-rust/crates/query/src/health_cache.rs:11-68`.
- Direct health-cache tests are live in `src-rust/crates/query/src/health_cache.rs:187-262`.
- `SessionBudget` is live in `src-rust/crates/query/src/session_budget.rs:18-86`.
- Shared registration / inheritance seam is live in `src-rust/crates/query/src/session_budget.rs:110-205`.
- Direct session-budget tests are live in `src-rust/crates/query/src/session_budget.rs:215-355`.
- Query-loop session-budget registration is live in `src-rust/crates/query/src/lib.rs:867-894`.
- Query-loop shared-budget accounting and event emission are live at:
  - `src-rust/crates/query/src/lib.rs:1356-1366`
  - `src-rust/crates/query/src/lib.rs:1638-1645`
  - `src-rust/crates/query/src/lib.rs:598-613`

### Child / TeamCreate D2 surfaces

- Child `allow_fallback` and `budget_usd` schema surfaces are live in `src-rust/crates/query/src/agent_tool.rs:226-255` and `:284-343`.
- Child fallback runtime wiring is live in `src-rust/crates/query/src/agent_tool.rs:402-417`.
- Child/session budget layering is live in `src-rust/crates/query/src/agent_tool.rs:149-160` and `:492-517`.
- Background child inheritance of the same seam is live in `src-rust/crates/query/src/agent_tool.rs:541-555`.
- Team-runner fallback / budget carriage is live in `src-rust/crates/query/src/agent_tool.rs:699-833`.
- TeamCreate schema / dispatch carriage is live in `src-rust/crates/tools/src/team_tool.rs:36-49`, `:157-183`, `:225-288`, and `:403-452`.

### Observability / event surfaces

- Child/team worker provider-resolution event creation is live in `src-rust/crates/query/src/agent_tool.rs:163-177`.
- Child/team worker budget-exceeded event creation is live in `src-rust/crates/query/src/agent_tool.rs:179-197`.
- Query event shapes are live in `src-rust/crates/query/src/lib.rs:400-470`.
- TeamCreate observability extraction / re-emission is live in `src-rust/crates/query/src/lib.rs:478-595`.
- Session-budget exceeded event emission is live in `src-rust/crates/query/src/lib.rs:598-613`.

### Direct live tests covering the D2 audit question

Provider / fallback / trust domain:

- `provider_types::tests::trust_domain_marks_local_provider_aliases_as_local`
- `provider_types::tests::trust_domain_marks_non_local_providers_as_cloud`
- `provider_resolution::tests::fallback_disabled_returns_suggestion_text`
- `provider_resolution::tests::fallback_same_domain_returns_healthy_cloud_candidate`
- `provider_resolution::tests::fallback_cross_domain_is_prohibited`
- `provider_resolution::tests::p3_explicit_provider_conflicts_with_model_prefix`
- `provider_resolution::tests::p5_explicit_provider_conflicts_with_reverse_model_prefix`
- `tests::provider_registry_some_resolution_failure_does_not_fallback_to_legacy_anthropic`

Child / team / budget / observability:

- `agent_tool::tests::agent_tool_allow_fallback_uses_same_domain_provider`
- `agent_tool::tests::child_session_budget_reuses_inherited_budget_when_child_limit_absent`
- `agent_tool::tests::child_session_budget_wraps_parent_when_child_limit_present`
- `agent_tool::tests::worker_budget_exceeded_event_reports_child_limit`
- `agent_tool::tests::teamcreate_mixed_providers_per_agent_dispatch`
- `tests::teamcreate_observability_is_sanitized_and_emitted`
- `tests::session_budget_exceeded_event_emits_only_on_new_cancellation`
- all direct `session_budget` unit tests
- all direct `health_cache` unit tests

Targeted test execution results in this preflight:

- `cargo test -p claurst-api trust_domain` -> passed
- `cargo test -p claurst-query fallback` -> passed
- `cargo test -p claurst-query session_budget` -> passed
- `cargo test -p claurst-query health_cache` -> passed
- `cargo test -p claurst-query teamcreate_mixed_providers_per_agent_dispatch` -> passed
- `cargo test -p claurst-query teamcreate_observability_is_sanitized_and_emitted` -> passed
- `cargo test -p claurst-query explicit_provider_conflicts` -> passed

## 9. Gap analysis matrix

| Old surrogate / material D2 behavior | Current live runtime/test status | Evidence | Audit judgment |
|---|---|---|---|
| Built-in local providers classify as `Local` | Live runtime + direct tests | `provider_types.rs:247-253`, tests at `:260-287` | `COVERED` |
| Unknown/custom provider IDs default to `Cloud` | Live runtime + direct trust-domain test of non-local default | `provider_types.rs:247-253`, test case includes `unknown-provider` at `:272-285` | `COVERED` |
| Same-domain fallback enumerates registered providers rather than a surrogate-only list | Live runtime + direct fallback test | `provider_resolution.rs:332-389`, `fallback_same_domain_returns_healthy_cloud_candidate` | `COVERED` |
| Cross-domain fallback is prohibited | Live runtime + direct test | `provider_resolution.rs:337-338`, `fallback_cross_domain_is_prohibited` | `COVERED` |
| Fallback-disabled path fails with descriptive suggestion | Live runtime + direct test | `provider_resolution.rs:322-325`, `fallback_disabled_returns_suggestion_text` | `COVERED` |
| Health-aware candidate filtering exists | Live runtime + direct health-cache tests + fallback ordering test | `health_cache.rs:47-68`, `provider_resolution.rs:329-355`, targeted `health_cache` / `fallback` test runs | `COVERED` |
| Capability floor participates in fallback eligibility | Live runtime + capability helper tests + fallback seam | `provider_resolution.rs:257-304`, capability tests in `provider_resolution_tests.rs:428-499` | `COVERED` |
| Provider/model conflict is rejected explicitly | Live runtime + direct tests | `provider_resolution.rs:129-134`, `:151-158`, tests `p3`, `p5` | `COVERED` |
| Child `allow_fallback` reaches real child/team runtime seams | Live runtime + direct tests | `agent_tool.rs:250-255`, `:402-417`, `:758-767`; `team_tool.rs:44`, `:178-180`, `:416-448`; `agent_tool_allow_fallback_uses_same_domain_provider` | `COVERED` |
| Child `budget_usd` is distinct from `max_budget_usd` and layers onto shared budget | Live runtime + direct tests | `agent_tool.rs:149-160`, `:492-517`, `session_budget.rs:36-86`, targeted `session_budget` test run | `COVERED` |
| Shared session budget is inherited and emits session-level event only on new cancellation | Live runtime + direct tests | `query/lib.rs:867-894`, `:1356-1366`, `:1638-1645`, `:598-613`, `session_budget_exceeded_event_emits_only_on_new_cancellation` | `COVERED` |
| Team mixed-provider dispatch is live, not surrogate-only | Live runtime + direct test | `team_tool.rs:403-452`, `agent_tool.rs:699-833`, `teamcreate_mixed_providers_per_agent_dispatch` | `COVERED` |
| Worker / team observability is live on the real seam | Live runtime + direct test | `agent_tool.rs:163-197`, `query/lib.rs:478-595`, `teamcreate_observability_is_sanitized_and_emitted` | `COVERED` |

Audit conclusion from the matrix:

- I did not find a remaining live D2 behavior that still depends on the old surrogate/spec-proxy to be proven.
- The surrogate’s intent is now represented by live runtime seams plus live unit/integration-style tests on the real D2 surfaces.

## 10. Recommended next-step decision

`M12-SATISFIED-BY-AUDIT`

Reason:

- The recast authority asks whether the current live D2 seams/tests subsume the old surrogate intent.
- They do.
- The old surrogate existed because trust-domain classification, fallback candidate enumeration, health/capability filtering, child fallback control, child/shared budgets, and D2 observability were not yet live.
- Those surfaces are now live in source and backed by direct tests, including targeted test execution in this preflight.
- I found no concrete remaining live implementation delta that still belongs to M12.

## 11. Exact follow-on boundary

Not applicable. No M12 delta was identified.

## 12. Explicit out-of-scope list

This preflight found no need to reopen:

- deferred architecture items:
  - shared/global `HealthCache` decision
  - `ToolContext` budget/cache carriage reconsideration
  - TeamCreate outer-cancellation redesign
- accepted M11 runtime tickets
- code outside the D2 surfaces reviewed above
- unrelated docs cleanup

Also explicitly out of scope for this preflight:

- rewriting or deleting archive-only surrogate documentation
- cleaning noisy untracked `docs/Current/*` or `docs/archive/*`
- reconciling `.gitignore` or broader worktree hygiene

## 13. Risks / notes

- The worktree is not clean. Later closure/review prompts must continue to declare the noisy basis explicitly.
- `docs/archive/provider_orchestrator/` is untracked in the current worktree. I used it only as historical evidence because the live pack explicitly points to it for surrogate reconstruction.
- The deleted split-era authority file is still visible as a tracked deletion in `git status`. That is a scope-attribution risk for future review passes, not an M12 blocker.
- The current decision is based on live repo reality at `HEAD = 63595c387ac8fd2f5adbf9cf75d45a724153c3db`, not on earlier split-era assumptions.

## 14. Final recommendation

Mark M12 as satisfied by audit against the accepted post-M11 baseline.

There is no remaining real M12 implementation ticket to execute on the current live D2 path. Any future work in this area would need to be justified as a new ticket with a new scope boundary, not as unfinished M12.
