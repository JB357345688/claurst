# TASK-M11-10 Preflight Report

## ticket id

`TASK-M11-10`

## verdict

`HALT`

## timestamp UTC

`2026-04-15T02:10:06Z`

## branch / HEAD observed

- Branch: `feature/provider-resolution-seam`
- HEAD: `4ef9547dab51959f7b39c473f929b81f05ee1134`
- HEAD matches the accepted latest baseline named in the task prompt: `yes`
- Working tree state: dirty and noisy, but not structurally conflicting with this preflight
- Observed repo noise includes:
  - modified `.gitignore`
  - many untracked docs/report files under `docs/`
  - untracked `src-rust/target/`

## authority reviewed

- `AGENTS.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md`
- `docs/archive/reports/M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md`
- `docs/archive/reports/MPWO_REVISION_REPORT_20260414T003218Z.md`
- `docs/archive/reports/TASK-M11-08R_CLOSEOUT_REPORT_20260415T004401Z.md`
- `docs/archive/reports/TASK-M11-08B_CLOSEOUT_REPORT_20260415T011952Z.md`
- `docs/archive/reports/TASK-M11-09_CLOSEOUT_REPORT_20260415T014220Z.md`
- User prompt authority for revised `TASK-M11-10 = child allow_fallback + budget_usd`

## accepted-baseline comparison

- Accepted corrected path is present in current repo reality.
- `TASK-M11-08R` is present as root `SessionBudget` wiring:
  - `QueryConfig.session_budget` exists in [lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:82)
  - CLI root budget wiring exists in [main.rs](/home/jordi/claurst/src-rust/crates/cli/src/main.rs:731)
- `TASK-M11-08B` is present as query-owned session-id keyed child/team propagation:
  - registry and session lookup exist in [session_budget.rs](/home/jordi/claurst/src-rust/crates/query/src/session_budget.rs:8)
  - `run_query_loop()` registers `config.session_budget` by `tool_ctx.session_id` in [lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:696)
  - child/team paths inherit via `session_budget_for_session(session_id)` in [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:134)
- Revised `TASK-M11-09` is present as child `max_tokens` override wiring:
  - `AgentInput.max_tokens` exists in [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:148)
  - `AgentSpec.max_tokens` and `AgentRunParams.max_tokens_override` exist in [team_tool.rs](/home/jordi/claurst/src-rust/crates/tools/src/team_tool.rs:37) and [team_tool.rs](/home/jordi/claurst/src-rust/crates/tools/src/team_tool.rs:157)

## verified target files / symbols / commands

- Primary ticket surfaces:
  - [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:148)
  - [team_tool.rs](/home/jordi/claurst/src-rust/crates/tools/src/team_tool.rs:37)
- Supporting seams:
  - [provider_resolution.rs](/home/jordi/claurst/src-rust/crates/query/src/provider_resolution.rs:302)
  - [lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:82)
  - [session_budget.rs](/home/jordi/claurst/src-rust/crates/query/src/session_budget.rs:8)
  - [health_cache.rs](/home/jordi/claurst/src-rust/crates/query/src/health_cache.rs:10)
  - [tools lib.rs](/home/jordi/claurst/src-rust/crates/tools/src/lib.rs:216)
  - [cli main.rs](/home/jordi/claurst/src-rust/crates/cli/src/main.rs:731)
- Verified symbols:
  - `AgentInput`
  - `AgentSpec`
  - `AgentRunParams`
  - `resolve_provider_with_fallback()`
  - `SessionBudget`
  - `register_session_budget()`
  - `session_budget_for_session()`
  - `register_agent_runner()`
- Verified commands:
  - `git rev-parse --abbrev-ref HEAD`
  - `git rev-parse HEAD`
  - `git status --short --branch`
  - `cd src-rust && cargo check --workspace`

## exact scope confirmation in current repo reality

- This preflight is for revised `TASK-M11-10 = allow_fallback + budget_usd`, after accepted `08R`, `08B`, and `09`.
- `allow_fallback` remains a narrow fit for the accepted path:
  - add `AgentInput.allow_fallback: Option<bool>`
  - add `AgentSpec.allow_fallback: Option<bool>`
  - add `AgentRunParams.allow_fallback: bool`
  - extend `AgentTool` and `TeamCreateTool` schemas
  - replace direct child `resolve_provider_identity() + materialize_provider()` calls with `resolve_provider_with_fallback(..., allow_fallback)`
- `budget_usd` does not remain a narrow fit on current repo reality if it is meant to create a child-specific `SessionBudget`.
- Reason:
  - all child/team query loops keep the same `ToolContext.session_id`
  - `run_query_loop()` registers the active `SessionBudget` by that shared `session_id`
  - `register_session_budget()` overwrites the stored budget for an occupied session key instead of stacking/restoring the parent budget
- Result:
  - creating a distinct child `SessionBudget` would overwrite the accepted `08B` parent-budget mapping for that session
  - that conflicts with the corrected accepted path and cannot be treated as narrow constructor/schema fallout

## child fallback/budget seam findings

1. Foreground `AgentTool` child run:
   - Provider resolution currently happens before the foreground/background split via `resolve_provider_identity()` and `materialize_provider()` in [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:281).
   - Session-budget inheritance currently comes from `inherited_session_budget(&ctx.session_id)` in [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:383).
   - Child cancellation currently derives from `session_budget.child_cancel_token()` when a budget exists, otherwise a fresh token, in [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:140) and [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:480).
   - Child `max_tokens` currently uses `params.max_tokens.unwrap_or(4096)` in [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:385).
   - Child `max_budget_usd` is currently hardcoded `None` in [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:403).

2. Background `AgentTool` child run:
   - Provider resolution is identical to the foreground path because both use the same `target` computed before branching in [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:281).
   - Session-budget inheritance uses the same shared `query_config.session_budget` built from `inherited_session_budget(&ctx.session_id)` in [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:383).
   - Child cancellation derives from that inherited budget in [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:433).
   - Child `max_tokens` is the same `params.max_tokens.unwrap_or(4096)` path in [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:387).
   - Child `max_budget_usd` is also `None` through the cloned `query_config`.

3. cc-query-backed team-runner child loop via `register_agent_runner()`:
   - `TeamCreateTool` currently passes `provider_override`, `model_override`, and `max_tokens_override` into `AgentRunParams` in [team_tool.rs](/home/jordi/claurst/src-rust/crates/tools/src/team_tool.rs:421).
   - Provider resolution currently happens inside the registered runner closure via `resolve_provider_identity()` and `materialize_provider()` in [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:613).
   - Session-budget inheritance currently comes from `inherited_session_budget(&ctx.session_id)` in [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:647).
   - Child cancellation currently derives from the inherited budget in [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:667).
   - Child `max_tokens` currently uses `max_tokens_override.unwrap_or(4096)` in [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:649).
   - Child `max_budget_usd` is currently the default `None` because the `QueryConfig` literal uses `..Default::default()` in [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:649).

- `resolve_provider_with_fallback()` already exists in [provider_resolution.rs](/home/jordi/claurst/src-rust/crates/query/src/provider_resolution.rs:302).
- Current runtime does not use it yet:
  - root query-loop registry dispatch still uses `resolve_provider_identity()` + `materialize_provider()` in [lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:886)
  - child/team paths also still use the direct pair in [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:305) and [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:613)
- `HealthCache` remains deferred at the runtime plumbing level:
  - `ToolContext` has no `health_cache` field in [tools lib.rs](/home/jordi/claurst/src-rust/crates/tools/src/lib.rs:216)
  - `resolve_provider_with_fallback()` only requires `&HealthCache`, and `HealthCache::new()` exists in [health_cache.rs](/home/jordi/claurst/src-rust/crates/query/src/health_cache.rs:17)
  - narrow child fallback wiring could therefore use an ephemeral cache without adding `ToolContext.health_cache`

## anticipated implementation shape

- Narrow and ticket-compatible for `allow_fallback`:
  - add optional schema fields on `AgentInput` and `AgentSpec`
  - resolve `Option<bool>` into `AgentRunParams.allow_fallback: bool`
  - preserve omitted default as `false`
  - call `resolve_provider_with_fallback()` at the two current child provider-resolution sites in `agent_tool.rs`
  - use an inline `HealthCache::new()` or similar local cache object to avoid `ToolContext.health_cache`

- Not narrow on current accepted path for `budget_usd`:
  - adding `AgentInput.budget_usd`, `AgentSpec.budget_usd`, and `AgentRunParams.budget_usd` is straightforward
  - wiring that field into a distinct child `SessionBudget` is not straightforward because the accepted `08B` seam is keyed only by shared `session_id`
  - wiring `budget_usd` into `QueryConfig.max_budget_usd` is explicitly forbidden by ticket authority because `budget_usd`, `max_budget_usd`, and `SessionBudget` must remain distinct
  - changing session IDs per child or changing session-budget registry semantics would widen scope into `lib.rs` and `session_budget.rs` and would effectively reopen the accepted `08B` seam

- Current-repo interpretation of child `budget_usd`:
  - `budget_usd absent` can continue to mean inherit the existing parent `SessionBudget`
  - `budget_usd present` cannot safely mean "create a child-specific SessionBudget on the current shared session-id seam" without additional authority and runtime changes

## anticipated compile-fallout scope

- If this ticket were reduced to `allow_fallback` only, anticipated compile fallout is narrow:
  - `agent_tool.rs`
  - `team_tool.rs`
  - import/use-site updates for `HealthCache` and `resolve_provider_with_fallback`
  - small test/schema fallout local to those modules

- For full revised `TASK-M11-10` including functional `budget_usd`, current repo reality suggests broader churn:
  - likely `query/src/lib.rs`
  - likely `query/src/session_budget.rs`
  - possibly additional tests around registration semantics and child nesting
- That is structural seam work, not mere constructor fallout.

## validation command run and result

- Command run: `cd src-rust && cargo check --workspace`
- Result: `PASS`
- Observed output: workspace check completed successfully on the accepted baseline

## drift found

- No branch drift: observed branch and HEAD exactly match the accepted baseline.
- No dependency-ticket drift: corrected `08R`, `08B`, and revised `09` are present in code and commit history.
- Repo-state noise exists:
  - modified `.gitignore`
  - many untracked docs/report files
  - untracked `src-rust/target/`
- Structural ticket-relevant drift found:
  - the current accepted query-owned session-budget seam does not support child-specific `SessionBudget` replacement under the same `session_id`
  - therefore revised `budget_usd` behavior is not implementable as a narrow schema/runtime wiring change on current repo reality

## blockers, if any

- Blocker 1:
  - revised `budget_usd` semantics conflict with the accepted `08B` session-id keyed propagation seam
  - a child-specific `SessionBudget` would overwrite the parent registration for the same session and break inherited shared-budget behavior

- Blocker 2:
  - the ticket explicitly forbids budget/limit conflation
  - therefore `budget_usd` cannot be reinterpreted as `max_budget_usd`

- Non-blocker note:
  - `allow_fallback` itself is not blocked; it can be wired narrowly with runtime-local `HealthCache`

## hosted Ollama invariant assessment

- Current baseline preserves hosted Ollama behavior.
- Revised `TASK-M11-10` preflight does not require any hosted-Ollama-specific change.
- `allow_fallback` can stay compatible with accepted same-domain fallback behavior because `resolve_provider_with_fallback()` already enforces trust-domain matching in [provider_resolution.rs](/home/jordi/claurst/src-rust/crates/query/src/provider_resolution.rs:324).
- The main hosted-Ollama risk is not fallback-domain policy; it is accidental widening of the child budget seam. No such change was made in this preflight.

## exact recommendation for next step

- Do not implement revised `TASK-M11-10` as currently framed.
- First resolve the child `budget_usd` interpretation explicitly with authority.
- The cleanest next step is one of:
  - split the ticket so `allow_fallback` proceeds now and child-specific budget semantics are handled in a follow-up ticket that explicitly changes the query-owned session-budget registry behavior, or
  - amend authority for `TASK-M11-10` to authorize the required `session_budget.rs` / `lib.rs` seam change while preserving accepted `08B` behavior.
- Until that decision is made, the correct preflight disposition is `HALT`.
