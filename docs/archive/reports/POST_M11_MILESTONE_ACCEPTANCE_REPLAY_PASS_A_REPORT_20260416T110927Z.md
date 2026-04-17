# 1. Replay scope

- Ticket ID: `POST-M11-MILESTONE-ACCEPTANCE-REPLAY - PASS A`
- Replay mode: targeted seam replay only on current `HEAD`; no code patching, no commit, no Pass B execution
- Preflight verdict: `READY-FOR-SPLIT-REPLAY`
- Governing authority verified:
  - `AGENTS.md`
  - `docs/Current/MPWO_WORK_ORDER_PACK.md`
  - historical mapping basis: `docs/archive/reports/POST_M11_MILESTONE_ACCEPTANCE_REPLAY_PLAN_REPORT_20260416T100911Z.md`
- Verified files/symbols/commands before execution:
  - files present: `src-rust/crates/tools/src/lib.rs`, `src-rust/crates/cli/src/main.rs`, `src-rust/crates/tools/src/team_tool.rs`, `src-rust/crates/query/src/agent_tool.rs`, `src-rust/crates/query/src/lib.rs`
  - ordered Pass A commands matched the accepted replay plan
  - report destination exists under `docs/archive/reports/`
- Drift found:
  - no structural drift in the live authority path
  - dirty worktree present on this branch; treated as a reporting note, not a blocker to targeted replay
- Blockers: none for Pass A execution

# 2. Timestamp UTC

`20260416T110927Z`

# 3. Branch / HEAD / worktree summary

- Branch: `feature/provider-resolution-seam`
- `HEAD`: `038f3c20e01a96eec6397d506b477a461166f762`
- `git status --short --branch` showed:
  - `6` modified tracked files
  - `1` deleted tracked file
  - extensive untracked paths, including archive reports, planning docs, `.codex`, and `src-rust/target/`
- Representative tracked worktree noise:
  - `.gitignore`
  - `docs/Current/TASK-M11-10_SPLIT_PLAN_AND_TEMPORARY_AUTHORITY_AMENDMENT.md` (`D`)
  - `src-rust/crates/api/src/providers/google.rs`
  - `src-rust/crates/core/src/effort.rs`
  - `src-rust/crates/core/src/lib.rs`
  - `src-rust/crates/core/src/remote_settings.rs`
  - `src-rust/crates/core/src/system_prompt.rs`
- Scope attribution note:
  - the deleted split-plan doc is consistent with the live authority pack, which explicitly says it is deleted/superseded and not active authority

# 4. Commands run

1. `git branch --show-current` -> exit `0`
2. `git rev-parse HEAD` -> exit `0`
3. `git status --short --branch` -> exit `0`
4. `rg -n "provider_registry|model_registry" src-rust/crates/tools/src/lib.rs src-rust/crates/cli/src/main.rs` -> exit `0`
5. `rg -n "provider_override|model_override|max_tokens_override|allow_fallback|budget_usd" src-rust/crates/tools/src/team_tool.rs src-rust/crates/query/src/agent_tool.rs` -> exit `0`
6. `rg -n "AnthropicClient::new|provider_registry not available in ToolContext|resolve_provider_identity\(|materialize_provider\(|resolve_provider_with_fallback\(" src-rust/crates/query/src/agent_tool.rs src-rust/crates/query/src/lib.rs` -> exit `0`
7. `rg -n "WorkerProviderResolved|WorkerBudgetExceeded|SessionBudgetExceeded|teamcreate_observability_is_sanitized_and_emitted|session_budget_exceeded_event_emits_only_on_new_cancellation" src-rust/crates/query/src/lib.rs src-rust/crates/query/src/agent_tool.rs` -> exit `0`
8. `cd src-rust && cargo test -p claurst-query -- provider_resolution` -> exit `0`
9. `cd src-rust && cargo test -p claurst-query -- explicit_provider_conflicts` -> exit `0`
10. `cd src-rust && cargo test -p claurst-query -- agent_tool` -> exit `0`
11. `cd src-rust && cargo test -p claurst-query -- provider_registry_none` -> exit `0`
12. `cd src-rust && cargo test -p claurst-query -- provider_registry_some_resolution_failure` -> exit `0`
13. `cd src-rust && cargo test -p claurst-query -- teamcreate_observability_is_sanitized_and_emitted` -> exit `0`
14. `cd src-rust && cargo test -p claurst-query -- session_budget_exceeded_event_emits_only_on_new_cancellation` -> exit `0`
15. `cd src-rust && cargo test -p claurst-api -- trust_domain` -> exit `0`
16. `cd src-rust && cargo test -p claurst-core -- cost_tracker` -> exit `0`
17. `cd src-rust && cargo test -p claurst-query -- health_cache` -> exit `0`
18. `cd src-rust && cargo test -p claurst-query -- session_budget` -> exit `0`

# 5. Structural probe results

- Registry carriage remains live in `ToolContext` and root CLI wiring:
  - `src-rust/crates/tools/src/lib.rs:232-234` still define `provider_registry` and `model_registry`
  - `src-rust/crates/cli/src/main.rs:741-756` still attach both registries into `query_config` and `ToolContext`
  - additional refresh/update wiring remains present at `src-rust/crates/cli/src/main.rs:1841-1860`
- Worker/team override carriage remains live:
  - `src-rust/crates/tools/src/team_tool.rs:43-48` still define `max_tokens_override`, `allow_fallback`, `budget_usd`, `provider_override`, and `model_override`
  - `src-rust/crates/tools/src/team_tool.rs:413-451` still pass those values into worker execution
  - `src-rust/crates/query/src/agent_tool.rs:717-835` still carry those values through child resolution, child budgets, fallback, and budget events
- Shared provider-resolution seam remains live on both root and worker paths:
  - root query path still calls `resolve_provider_identity()` at `src-rust/crates/query/src/lib.rs:1102`
  - root query path still calls `materialize_provider()` at `src-rust/crates/query/src/lib.rs:1114`
  - worker path still calls `resolve_provider_identity()` at `src-rust/crates/query/src/agent_tool.rs:394` and `:751`
  - worker path still calls `resolve_provider_with_fallback()` at `src-rust/crates/query/src/agent_tool.rs:411` and `:766`
  - missing-registry guard text remains live at `src-rust/crates/query/src/agent_tool.rs:403` and `:729`
- Legacy Anthropic construction note:
  - `AnthropicClient::new` was still found in `src-rust/crates/query/src/lib.rs:1857` and `:2527`
  - no `AnthropicClient::new` hit was found in `src-rust/crates/query/src/agent_tool.rs`
  - this matches the targeted replay results: the legacy path is still intentionally exercised only for the `provider_registry_none` root case, while the registry-present failure case does not fall back to legacy
- Observability seam remains live:
  - `QueryEvent` variants `WorkerProviderResolved`, `WorkerBudgetExceeded`, and `SessionBudgetExceeded` remain present in `src-rust/crates/query/src/lib.rs:416-427`
  - event translation remains present at `src-rust/crates/query/src/lib.rs:573-608`
  - direct observability tests remain present at `src-rust/crates/query/src/lib.rs:2784-2871`

# 6. Targeted seam test results

- Stop rule outcome: no targeted seam replay command failed; Pass A ran through command `18`
- Total targeted tests passed in this pass: `75`
- `cargo test -p claurst-query -- provider_resolution`
  - result: `31 passed, 0 failed`
  - direct coverage observed:
    - `p1` through `p12` precedence matrix tests
    - `materialize_provider_*` materialization/auth/api-base/no-credentials cases
    - fallback behavior, including same-domain selection and cross-domain prohibition
    - `fallback_same_session_reuses_registered_health_cache`
    - `fallback_session_scopes_do_not_share_cached_health`
- `cargo test -p claurst-query -- explicit_provider_conflicts`
  - result: `2 passed, 0 failed`
  - direct coverage observed:
    - `p3_explicit_provider_conflicts_with_model_prefix`
    - `p5_explicit_provider_conflicts_with_reverse_model_prefix`
- `cargo test -p claurst-query -- agent_tool`
  - result: `10 passed, 0 failed`
  - direct coverage observed:
    - `agent_tool_errors_when_provider_registry_missing`
    - `agent_explicit_provider_routes_to_openai_provider`
    - `agent_parent_inherits_provider_openai_dispatch`
    - `agent_tool_allow_fallback_uses_same_domain_provider`
    - `agent_tool_respects_max_tokens_override`
    - `child_session_budget_reuses_inherited_budget_when_child_limit_absent`
    - `child_session_budget_wraps_parent_when_child_limit_present`
    - `worker_budget_exceeded_event_reports_child_limit`
    - `teamcreate_mixed_providers_per_agent_dispatch`
    - `child_and_team_fallback_share_session_health_cache`
- `cargo test -p claurst-query -- provider_registry_none`
  - result: `1 passed, 0 failed`
  - direct coverage observed:
    - `provider_registry_none_uses_legacy_anthropic_client_path`
- `cargo test -p claurst-query -- provider_registry_some_resolution_failure`
  - result: `1 passed, 0 failed`
  - direct coverage observed:
    - `provider_registry_some_resolution_failure_does_not_fallback_to_legacy_anthropic`
- `cargo test -p claurst-query -- teamcreate_observability_is_sanitized_and_emitted`
  - result: `1 passed, 0 failed`
- `cargo test -p claurst-query -- session_budget_exceeded_event_emits_only_on_new_cancellation`
  - result: `1 passed, 0 failed`
- `cargo test -p claurst-api -- trust_domain`
  - result: `2 passed, 0 failed`
  - direct coverage observed:
    - local-provider aliases marked local
    - non-local providers marked cloud
- `cargo test -p claurst-core -- cost_tracker`
  - result: `3 passed, 0 failed`
  - direct coverage observed:
    - `test_cost_tracker`
    - `test_cost_tracker_initial_zero`
    - `test_cost_tracker_cumulative`
- `cargo test -p claurst-query -- health_cache`
  - result: `12 passed, 0 failed`
  - direct coverage observed:
    - cache registration/lifetime probes
    - stale probe success/error/timeout behavior
    - session-scoped reuse coverage surfaced again via provider-resolution and agent-tool tests
- `cargo test -p claurst-query -- session_budget`
  - result: `11 passed, 0 failed`
  - direct coverage observed:
    - root threshold cancellation
    - child-local and parent budget accounting
    - descendant chained caps
    - root-child cancel propagation
    - session registration/lifetime
    - budget-exceeded event emission behavior

# 7. Historical milestone coverage summary

| Bucket | Current evidence in Pass A | Basis |
| --- | --- | --- |
| M7 seam extraction / precedence / materialization | `indirect` | `provider_resolution` directly re-proved precedence and materialization, but the original seam-extraction claim is historical and only indirectly supported on current `HEAD` by structural probes plus live tests |
| M8 worker propagation | `indirect` | structural probes re-verified registry and override carriage, and `agent_tool`/TeamCreate tests passed, but the original propagation landing is historical rather than replayed as a fresh code delta |
| M9 D1 validation cases | `direct` | `provider_resolution`, `explicit_provider_conflicts`, `agent_tool`, `provider_registry_none`, and `provider_registry_some_resolution_failure` directly exercised the accepted D1 validation claims |
| M11 split-path runtime seams | `indirect` | trust-domain, cost-tracker, session-budget, child-budget, fallback, observability, and worker-budget tests all passed directly, but current Pass A evidence for the full root split-path seam remains partly structural rather than a broad gate replay |
| POST-M11-ARCH-01A | `direct` | `health_cache`, `provider_resolution`, and `agent_tool` directly re-proved session-scoped HealthCache reuse and non-sharing semantics on current `HEAD` |
| M12 and deferred ARCH-02 / ARCH-03 | `record-only` | not runtime replay targets in this pass by design; carry as historical/audit context only |

# 8. Failures / warnings / scope-attribution notes

- No targeted seam replay command failed.
- No Pass B command was run in this pass.
- `cargo build --workspace`, `cargo test --workspace`, `cargo fmt`, and `cargo clippy` were intentionally not run, per pass scope.
- Dirty worktree noise remains material for later interpretation:
  - broad Pass B failures, if any, will need attribution against the existing modified/deleted/untracked state rather than being assumed to be replay regressions
- The `AnthropicClient::new` probe result is a note, not a Pass A failure:
  - current evidence shows it remains on the root/legacy path in `query/src/lib.rs`
  - worker/agent-tool routing continues to use the shared seam and missing-registry guardrails
  - the targeted legacy/non-legacy regression tests both passed, which is the relevant accepted behavior for this pass
- M12 closeout and deferred `POST-M11-ARCH-02` / `POST-M11-ARCH-03` remained record-only context and were not treated as runtime replay work

# 9. Overall verdict

`PASS-WITH-NOTES`

- Reason:
  - all ordered Pass A commands completed successfully
  - all targeted seam tests passed
  - current-head evidence is sufficient to preserve targeted milestone confidence for M7/M8/M9/M11/ARCH-01A without showing a targeted replay blocker
  - notes remain for dirty worktree attribution and for milestone buckets that are only indirectly proven on current `HEAD`

# 10. Recommendation on whether Pass B should proceed

- Recommendation: `YES`
- Basis:
  - Pass A found no targeted seam failure that would block continuation
  - the accepted split replay plan's gate condition for proceeding to Pass B is satisfied
  - when Pass B is run, broad-workspace failures should be attributed carefully against the current dirty branch rather than treated as automatic milestone regressions
