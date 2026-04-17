# TASK-M11-10B2 Preflight Report

## ticket id

`TASK-M11-10B2`

## verdict

`PASS-WITH-NOTES`

## timestamp UTC

`2026-04-15T03:53:05Z`

## branch / HEAD observed

- Branch: `feature/provider-resolution-seam`
- HEAD: `3812df04ec87ce0e96f851da2d18ab38b24f0b99`
- Expected accepted latest HEAD from prompt: `3812df04ec87ce0e96f851da2d18ab38b24f0b99`
- Match result: exact match
- Working tree state: noisy / dirty from unrelated artifacts (`.gitignore`, `.codex`, many untracked report/doc paths, `src-rust/target/`), but no evidence of structural drift in the ticket-owned Rust surfaces inspected for this preflight

## authority reviewed

- `AGENTS.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md`
- `docs/Current/TASK-M11-10_SPLIT_PLAN_AND_TEMPORARY_AUTHORITY_AMENDMENT.md`
- `docs/archive/reports/M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md`
- `docs/archive/reports/MPWO_REVISION_REPORT_20260414T003218Z.md`
- `docs/archive/reports/TASK-M11-08R_CLOSEOUT_REPORT_20260415T004401Z.md`
- `docs/archive/reports/TASK-M11-08B_CLOSEOUT_REPORT_20260415T011952Z.md`
- `docs/archive/reports/TASK-M11-09_CLOSEOUT_REPORT_20260415T014220Z.md`
- `docs/archive/reports/TASK-M11-10A_CLOSEOUT_REPORT_20260415T024141Z.md`
- `docs/archive/reports/TASK-M11-10B_AUTHORITY_REPORT_20260415T030449Z.md`
- `docs/archive/reports/TASK-M11-10B_SPLIT_AUTHORITY_REPORT_20260415T031240Z.md`
- `docs/archive/reports/TASK-M11-10B1_CLOSEOUT_REPORT_20260415T034453Z.md`

## accepted-baseline comparison

Accepted corrected-path assumptions remain present in current repo reality and do not need reopening:

- `08R` root `SessionBudget` wiring is present in `src-rust/crates/cli/src/main.rs:734-735` where root `budget_usd` creates `QueryConfig.session_budget = Some(Arc::new(SessionBudget::new(usd)))`.
- `08B` inherited parent shared-session accounting is present in:
  - `src-rust/crates/query/src/agent_tool.rs:134-137` via `inherited_session_budget(&ctx.session_id)`
  - `src-rust/crates/query/src/lib.rs:694-705` via `with_registered_session_budget(...)`
- `09` child `max_tokens` override wiring is present in:
  - `src-rust/crates/query/src/agent_tool.rs:403-405`
  - `src-rust/crates/query/src/agent_tool.rs:658-659`
  - `src-rust/crates/tools/src/team_tool.rs:43, 407, 437`
- `10A` child `allow_fallback` wiring is present in:
  - `src-rust/crates/query/src/agent_tool.rs:174, 312, 321`
  - `src-rust/crates/query/src/agent_tool.rs:594, 635`
  - `src-rust/crates/tools/src/team_tool.rs:44, 179, 408, 438`
- `10B1` accepted layered child-budget seam is present in:
  - `src-rust/crates/query/src/session_budget.rs:36-66` (`SessionBudget::child_scope`, parent propagation, `shared_budget`)
  - `src-rust/crates/query/src/session_budget.rs:98-155` (task-local stack aware registration via `register_session_budget` / `with_registered_session_budget`)
  - `src-rust/crates/query/src/lib.rs:694-705` (loop registration wrapper)

Rejected assumptions remain rejected in live code:

- no `ToolContext.session_budget`
- no `ToolContext.health_cache`
- no concrete query-owned budget/cache fields added to `ToolContext`

## verified target files / symbols / commands

Verified primary target files:

- `src-rust/crates/query/src/agent_tool.rs`
- `src-rust/crates/tools/src/team_tool.rs`

Verified read-only supporting files:

- `src-rust/crates/query/src/session_budget.rs`
- `src-rust/crates/query/src/lib.rs`
- `src-rust/crates/tools/src/lib.rs`
- `src-rust/crates/query/src/provider_resolution.rs`
- `src-rust/crates/query/src/health_cache.rs`
- `src-rust/crates/cli/src/main.rs`

Verified current symbol reality:

- `AgentInput` is defined in `src-rust/crates/query/src/agent_tool.rs:149` and currently has `max_tokens`, `model`, `provider`, `allow_fallback`, `isolation`, `run_in_background`; it does not currently have `budget_usd`.
- `AgentSpec` is defined in `src-rust/crates/tools/src/team_tool.rs:158` and currently has `provider`, `model`, `max_tokens`, `allow_fallback`; it does not currently have `budget_usd`.
- `AgentRunParams` is defined in `src-rust/crates/tools/src/team_tool.rs:37` and currently has `max_tokens_override` and `allow_fallback`; it does not currently have `budget_usd`.
- All three child/team spawn paths were verified:
  - foreground `AgentTool` child run in `src-rust/crates/query/src/agent_tool.rs:396-416` plus synchronous `run_query_loop(...)` call at `493-503`
  - background `AgentTool` child run uses the same `query_config` clone and launches `run_query_loop(...)` at `447-457`
  - cc-query-backed team runner created by `init_team_swarm_runner()` in `src-rust/crates/query/src/agent_tool.rs:581-678`, with runner `QueryConfig` assembled at `654-668`

Verified commands:

- `git branch --show-current`
- `git rev-parse HEAD`
- `git status --short --branch`
- `rg -n "struct AgentInput|struct AgentSpec|struct AgentRunParams|budget_usd|max_tokens_override|allow_fallback|session_budget|child_budget|register_agent_runner|inherited_session_budget|session_budget_for_session|register_session_budget|begin_child_scope" ...`
- `sed -n` inspections of the target/supporting files listed above
- `cd src-rust && cargo check --workspace`

## exact scope confirmation in current repo reality

This preflight is for `10B2 = child/team schema and runtime carriage for budget_usd` only. It is not a seam-redesign ticket, and accepted `10B1` must not be reopened.

Current repo reality supports that narrow interpretation:

- the query-owned layered child-budget seam already exists from accepted `10B1`
- child/team entry surfaces still lack `budget_usd` fields, so the missing work is carriage rather than seam invention
- omitted `budget_usd` can continue to preserve the accepted current behavior: inherit the parent shared-session `SessionBudget` from `08B`, with no extra child-local cap
- `max_budget_usd` remains separate and untouched in `src-rust/crates/query/src/lib.rs:1446-1455`
- `ToolContext` remains clean of query-owned budget/cache state in `src-rust/crates/tools/src/lib.rs:216-233`

Conclusion on scope: on live repo reality, `TASK-M11-10B2` remains a single narrow ticket on the corrected accepted path.

## child budget carriage findings

1. The accepted `10B1` seam is present and usable.
   - `SessionBudget::child_scope(parent, budget_usd)` exists in `session_budget.rs:36-43`.
   - `record_cost()` and `check_and_cancel()` cascade to the parent in `session_budget.rs:45-57`.
   - `shared_budget()` preserves the first shared root budget for registry visibility in `session_budget.rs:63-66`.
   - `with_registered_session_budget(...)` uses a task-local stack, so nested child scopes no longer overwrite the visible parent shared-session registration in `session_budget.rs:131-155`.

2. Current child/team runtime still carries only inherited shared-session budget and no child-local cap.
   - `AgentTool` foreground/background path currently does:
     - `let session_budget = inherited_session_budget(&ctx.session_id);`
     - `session_budget: session_budget.clone()`
     - `max_budget_usd: None`
   - team runner currently does the same at `agent_tool.rs:654-668`
   - `team_tool.rs` does not expose `budget_usd` in schema or `AgentRunParams`

3. `HealthCache` remains unrelated.
   - it is runtime-local fallback infrastructure in `src-rust/crates/query/src/health_cache.rs`
   - current child budget semantics do not require `HealthCache`
   - no inspected budget path depends on adding `ToolContext.health_cache`

4. Risk to already-closed tickets appears low if `10B2` stays within carriage.
   - `08R` root `budget_usd` meaning stays `SessionBudget`, not `max_budget_usd`
   - `08B` parent shared-session accounting remains preserved if child-local caps are created through `SessionBudget::child_scope(...)`
   - `09` `max_tokens` stays separate
   - `10A` `allow_fallback` stays separate
   - same-domain fallback behavior from `TASK-M11-05` is untouched if provider-resolution code is not redesigned

## anticipated implementation shape

Expected narrow implementation shape on current live code:

- add `budget_usd: Option<f64>` to:
  - `AgentInput`
  - `AgentSpec`
  - `AgentRunParams`
- extend the `AgentTool` and `TeamCreateTool` JSON schemas to expose `budget_usd`
- in `agent_tool.rs`, compute child runtime budget as:
  - omitted `budget_usd` => keep `inherited_session_budget(&ctx.session_id)` exactly as today
  - provided `budget_usd` with inherited parent budget => wrap the inherited parent with `Arc::new(SessionBudget::child_scope(parent_budget, usd))`
- reuse that computed `session_budget` for both:
  - foreground `AgentTool` child runs
  - background `AgentTool` child runs
- pass `budget_usd` through `TeamCreateTool` into `AgentRunParams`
- in `init_team_swarm_runner()`, construct the team-runner child scope against the accepted `10B1` seam in the same way as the foreground/background paths
- keep `QueryConfig.max_budget_usd = None` in these child/team paths

This shape targets the accepted `10B1` seam narrowly and does not require redesigning that seam.

## anticipated compile-fallout scope

Likely primary edit scope:

- `src-rust/crates/query/src/agent_tool.rs`
- `src-rust/crates/tools/src/team_tool.rs`

Likely query-side fallout:

- no additional structural query-side seam change appears required in `src-rust/crates/query/src/session_budget.rs`
- no structural query-side seam change appears required in `src-rust/crates/query/src/lib.rs`
- if any query-side adjustment is needed, it is most likely narrow constructor/API fallout only, not structural drift

Reason:

- `QueryConfig.session_budget` already carries the runtime object needed
- `run_query_loop()` already registers and charges the active budget
- `SessionBudget::child_scope(...)` already provides the additional child-local cap while preserving parent accounting

## validation command run and result

- Command: `cd src-rust && cargo check --workspace`
- Result: `PASS`
- Output summary:
  - `Compiling claurst v0.0.8 (/home/jordi/claurst/src-rust/crates/cli)`
  - `Finished dev profile [unoptimized + debuginfo] target(s) in 0.31s`

Classification:

- no compile failure occurred
- no ticket-relevant compile blocker was found in this preflight

## drift found

- No structural drift was found against the accepted `10B1` seam or the corrected `08R -> 08B -> 09 -> 10A -> 10B1` path.
- Current project authority is split across the MPWO and the later `TASK-M11-10` split/amendment reports; the live prompt explicitly supplied the later controlling documents, so this is an authority note rather than a blocker.
- The worktree is noisy with unrelated modified/untracked files, which should be kept out of any later `10B2` implementation/review basis.
- Current live code does not yet express behavior for `child budget_usd` because `10B2` carriage has not landed; that is expected missing scope, not drift.

## blockers, if any

No hard blocker was found for a narrow `TASK-M11-10B2` execution pass.

Notes that should remain explicit when implementation begins:

- `10B2` must remain carriage-only and must not reopen the accepted `10B1` seam
- omitted `budget_usd` must preserve current inherited parent shared-session behavior exactly
- `budget_usd` must remain distinct from `max_budget_usd`, shared inherited parent `SessionBudget`, and `max_tokens`
- unrelated worktree noise should be excluded from the later ticket diff/review basis

## hosted Ollama invariant assessment

`preserved`

Basis:

- current accepted `10B1` seam changes are confined to `session_budget.rs` and `query::lib.rs`
- hosted-Ollama-sensitive provider-resolution / materialization / auth / request-shaping paths were not changed by `10B1`
- live `10B2` target work is carriage in `agent_tool.rs` and `team_tool.rs`, not provider-resolution redesign
- `HealthCache` remains runtime-local fallback infrastructure and unrelated to budget semantics

## exact recommendation for next step

Proceed to `TASK-M11-10B2` execution as a single narrow ticket on the accepted corrected path.

Execution guardrails for the next pass:

- treat this as `10B2 = child/team schema and runtime carriage for budget_usd`, not seam redesign
- treat accepted `10B1` as fixed baseline and do not reopen it
- add and wire only:
  - `AgentInput.budget_usd: Option<f64>`
  - `AgentSpec.budget_usd: Option<f64>`
  - `AgentRunParams.budget_usd: Option<f64>`
- cover exactly the three required child/team paths:
  - foreground `AgentTool`
  - background `AgentTool`
  - cc-query-backed team runner via `register_agent_runner()`
- preserve omitted behavior as inherited parent shared-session accounting only
- preserve `08R`, `08B`, `09`, `10A`, hosted Ollama, and same-domain fallback behavior
- do not add `ToolContext.session_budget`
- do not add `ToolContext.health_cache`
- do not reinterpret child `budget_usd` as `max_budget_usd`
