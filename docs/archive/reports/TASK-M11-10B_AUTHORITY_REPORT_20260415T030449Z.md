# TASK-M11-10B Authority Report

## ticket id

`TASK-M11-10B`

## timestamp UTC

`2026-04-15T03:04:49Z`

## authority inputs reviewed

- `AGENTS.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md`
- `docs/Current/TASK-M11-10_SPLIT_PLAN_AND_TEMPORARY_AUTHORITY_AMENDMENT.md`
- `docs/archive/reports/M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md`
- `docs/archive/reports/MPWO_REVISION_REPORT_20260414T003218Z.md`
- `docs/archive/reports/TASK-M11-08R_CLOSEOUT_REPORT_20260415T004401Z.md`
- `docs/archive/reports/TASK-M11-08B_CLOSEOUT_REPORT_20260415T011952Z.md`
- `docs/archive/reports/TASK-M11-09_CLOSEOUT_REPORT_20260415T014220Z.md`
- `docs/archive/reports/TASK-M11-10_PREFLIGHT_REPORT_20260415T021006Z.md`
- `docs/archive/reports/TASK-M11-10A_CLOSEOUT_REPORT_20260415T024141Z.md`
- Live repo surfaces inspected:
  - `src-rust/crates/query/src/session_budget.rs`
  - `src-rust/crates/query/src/lib.rs`
  - `src-rust/crates/query/src/agent_tool.rs`
  - `src-rust/crates/tools/src/team_tool.rs`
  - `src-rust/crates/tools/src/lib.rs`
  - `src-rust/crates/query/src/provider_resolution.rs`
  - `src-rust/crates/query/src/health_cache.rs`
  - `src-rust/crates/cli/src/main.rs`

## current baseline checked

- Branch observed: `feature/provider-resolution-seam`
- HEAD observed: `ea046c52da82dfd9778f4065bd36b36e28d73c8a`
- Expected branch / HEAD from prompt: matched exactly
- Worktree state: dirty and noisy from unrelated docs artifacts, `.gitignore`, `.codex`, and `src-rust/target/`; no structural blocker for this docs-only pass
- Accepted corrected path verified in live code:
  - `08R`: root `budget_usd` still creates `SessionBudget` in `src-rust/crates/cli/src/main.rs:734-735`
  - `08B`: child/team paths still inherit session budget by shared `session_id` and `run_query_loop()` still re-registers that budget under `tool_ctx.session_id`
  - `09`: child `max_tokens` override remains in `agent_tool.rs` / `team_tool.rs`
  - `10A`: child `allow_fallback` wiring remains in `agent_tool.rs` / `team_tool.rs`
- Review basis for this pass: one new docs report file only

## missing capability after `TASK-M11-10A`

The missing capability is still exactly this:

- child and team child runs have no child-specific `budget_usd` field or runtime behavior
- foreground AgentTool, background AgentTool, and the cc-query-backed team runner only inherit the already-registered parent `SessionBudget`
- those child paths still set `max_budget_usd: None`, so no child-local budget cap exists at all

After `10A`, child execution can override:

- provider
- model
- `max_tokens`
- `allow_fallback`

But child execution still cannot express:

- a child-local cumulative USD budget that is distinct from `max_budget_usd`
- and distinct from the inherited shared parent `SessionBudget`

## live child-budget seam findings

1. Root `budget_usd` semantics are already concrete and query-owned.
   - `--budget-usd` maps to `QueryConfig.session_budget = Some(Arc::new(SessionBudget::new(usd)))` in `src-rust/crates/cli/src/main.rs:734-735`.
   - This is the accepted meaning of root `budget_usd`: cumulative USD spend tracked through `SessionBudget`, not `max_budget_usd`.

2. The current child/team runtime only carries one live session budget.
   - `QueryConfig` has one field: `session_budget: Option<Arc<SessionBudget>>` in `src-rust/crates/query/src/lib.rs:117`.
   - `run_query_loop()` registers only that one budget under `tool_ctx.session_id` in `src-rust/crates/query/src/lib.rs:696-699`.
   - `run_query_loop()` records spend and checks cancellation only on that one budget in `src-rust/crates/query/src/lib.rs:1414-1417`.
   - `max_budget_usd` remains a separate per-loop guard in `src-rust/crates/query/src/lib.rs:1421-1428`.

3. The accepted `08B` seam is session-id keyed and overwrite-prone for distinct child budgets.
   - `SESSION_BUDGET_REGISTRY` is keyed only by `String session_id` in `src-rust/crates/query/src/session_budget.rs:8-9`.
   - `register_session_budget()` replaces the stored `Arc<SessionBudget>` for an occupied key in `src-rust/crates/query/src/session_budget.rs:70-75`.
   - `session_budget_for_session()` returns only the currently stored budget for that session in `src-rust/crates/query/src/session_budget.rs:86-90`.

4. All live child/team paths inherit budget by the shared parent `session_id`.
   - AgentTool uses `inherited_session_budget(&ctx.session_id)` in `src-rust/crates/query/src/agent_tool.rs:134-137` and `396`.
   - AgentTool foreground/background `QueryConfig` then sets `session_budget: session_budget.clone()` and `max_budget_usd: None` in `src-rust/crates/query/src/agent_tool.rs:398-416`.
   - The cc-query-backed team runner also uses `inherited_session_budget(&ctx.session_id)` and sets only one `session_budget` in `src-rust/crates/query/src/agent_tool.rs:654-668`.
   - `TeamCreateTool` currently carries only `max_tokens_override` and `allow_fallback` through `AgentRunParams` in `src-rust/crates/tools/src/team_tool.rs:37-47` and `431-442`.

5. What specifically breaks if child `budget_usd` creates a distinct child `SessionBudget` on the current seam:
   - the child loop would register its own budget under the same shared `session_id`
   - that registration would replace the parent budget pointer for the duration of the child run
   - nested descendants of that child would then inherit the child budget instead of the parent budget
   - when the loop records spend, it would record only into the currently attached budget
   - therefore parent shared-session accounting from accepted `08B` would stop receiving that child spend unless the seam is expanded beyond the current single-budget model

6. Inference from accepted `08R` + `08B` baseline:
   - root `budget_usd` already means `SessionBudget`, not `max_budget_usd`
   - accepted `08B` established that child spend should count against the inherited parent session budget
   - therefore the faithful child meaning is not "replace parent budget"
   - the faithful child meaning is "add a child-local cumulative USD cap for that child subtree while preserving parent shared-session accounting"

## crate-boundary / seam constraints

- `SessionBudget` and its registry are query-owned in `src-rust/crates/query/src/session_budget.rs`
- the active runtime budget semantics are query-owned in `src-rust/crates/query/src/lib.rs`
- child runtime construction points live in `src-rust/crates/query/src/agent_tool.rs`
- team schema/runtime carriage lives in `src-rust/crates/tools/src/team_tool.rs`
- `ToolContext` in `src-rust/crates/tools/src/lib.rs:216-233` still has no `session_budget` or `health_cache`, and the accepted corrected path says it must stay that way
- `HealthCache` is query-owned fallback infrastructure in `src-rust/crates/query/src/health_cache.rs`; it is unrelated to child budget semantics after `10A`

Constraint summary:

- a faithful `10B` implementation cannot be query-only if it also exposes the child/team schema
- but the seam-definition part is query-owned first, because the current single-budget seam is the blocker
- `ToolContext` is not a valid place to solve this

## evaluated authority options

### Option A: child `budget_usd` creates a child-specific `SessionBudget` and therefore requires seam change

Assessment:

- This is the only option that matches the existing root meaning of `budget_usd`.
- It stays distinct from `max_budget_usd`.
- It also matches the split-plan blocker language: the unresolved problem is child-specific budget semantics on the accepted `08B` seam.

But on live repo reality, Option A is not just "register another budget":

- if the child budget simply replaces the parent budget, accepted `08B` parent shared-session accounting is broken
- if the child budget is intended as an additional cap, the current runtime still cannot express that because `QueryConfig` and `run_query_loop()` only handle one `SessionBudget`
- so Option A requires a query-owned seam expansion such as layered, nested, or composite child-budget handling before schema/runtime carriage can be correct

Verdict:

- valid
- not narrow field-only wiring
- requires explicit seam authority

### Option B: interpret child `budget_usd` some other narrow way that remains distinct from `max_budget_usd`

Realistic sub-cases:

- map it to `QueryConfig.max_budget_usd`
  - invalid: explicit budget-mechanism conflation
- treat it as "replace inherited parent budget just for this child"
  - invalid: breaks accepted `08B` parent shared-session accounting
- treat it as metadata / no-op
  - invalid: does not supply the missing capability
- treat it as a child-local spend delta against the shared parent budget
  - still requires new query-owned seam/API beyond current single-budget model
  - not materially narrower than Option A in implementation terms

Verdict:

- no valid narrow Option B was found that is both faithful to the accepted path and distinct from `max_budget_usd`

## recommended `TASK-M11-10B` scope

Recommended authority outcome:

- `TASK-M11-10B` should not remain a single executable wiring ticket
- the narrowest valid next boundary is a second split

Recommended split:

1. `TASK-M11-10B1` — query-owned child budget seam resolution
   - define the accepted meaning of child `budget_usd` as an additional child-local cumulative USD cap for the child subtree
   - preserve parent shared-session accounting from `08B`
   - implement the query-owned layered/nested budget seam needed to support that meaning
   - file scope: `src-rust/crates/query/src/session_budget.rs`, `src-rust/crates/query/src/lib.rs`, and only the minimal query-side construction fallout needed to exercise the seam

2. `TASK-M11-10B2` — child/team schema and runtime carriage
   - add child/team `budget_usd` fields
   - wire foreground AgentTool, background AgentTool, and the team runner into the accepted `10B1` seam
   - file scope: `src-rust/crates/query/src/agent_tool.rs` and `src-rust/crates/tools/src/team_tool.rs`, plus any narrow fallout from the accepted `10B1` seam

Why this is the narrowest valid boundary:

- the blocker is the single-budget query seam, not the absence of schema fields
- solving schema carriage before the seam exists would force speculative semantics
- solving the seam first keeps the first executable ticket query-owned and authority-clean

## explicitly excluded scope

- do not reinterpret child `budget_usd` as `max_budget_usd`
- do not treat child `budget_usd` as a synonym for the inherited parent shared `SessionBudget`
- do not add `ToolContext.session_budget`
- do not add `ToolContext.health_cache`
- do not pull `HealthCache` into `10B`
- do not reopen `10A`, `09`, `08B`, or root CLI `budget_usd`
- do not redesign TeamCreate outer per-agent cancellation; current repo reality does not prove that redesign is inseparable from child `budget_usd`
- do not expand QueryEvent work here; `M11-11` remains downstream
- do not widen into provider fallback or provider-resolution policy

## whether `team_tool.rs` belongs in this ticket

For an unsplit `TASK-M11-10B`, `team_tool.rs` would necessarily belong because child/team schema and `AgentRunParams` carriage live there.

For the recommended narrower path, `team_tool.rs` does **not** belong in the first executable split (`10B1`). It belongs only in the follow-on carriage split (`10B2`).

Recommendation:

- keep `team_tool.rs` out of the seam-resolution step
- include it only once the query-owned child-budget seam is accepted

## whether `HealthCache` belongs in this ticket

`no`

Basis:

- `HealthCache` is only used by `resolve_provider_with_fallback(...)`
- child `allow_fallback` already landed in `10A`
- live child budget behavior does not touch provider health, provider fallback, or provider selection
- no inspected budget seam requires `HealthCache`

## sequencing impact on `TASK-M11-11` and later

- `TASK-M11-11` should not proceed on the assumption that child `budget_usd` semantics are already settled
- if `10B` is split again, downstream sequencing should become:
  - `TASK-M11-10B1` seam resolution
  - `TASK-M11-10B1` closeout / acceptance
  - `TASK-M11-10B2` schema/runtime carriage
  - `TASK-M11-10B2` closeout / acceptance
  - then `TASK-M11-11`
- `TASK-M11-12` test authority should also wait for whichever final child-budget ticket actually lands
- no already-closed ticket needs reopening if the new seam preserves:
  - root `budget_usd` semantics from `08R`
  - inherited parent session-budget accounting from `08B`
  - child `max_tokens` from `09`
  - child `allow_fallback` from `10A`

## exact proposed wording snippet for GPT/WebUI to adopt as temporary authority for `TASK-M11-10B`

> `TASK-M11-10B` is not a narrow field-wiring ticket on the accepted corrected path. Child `budget_usd` means an additional child-local cumulative USD cap for the spawned child subtree, distinct from both `max_budget_usd` and the inherited parent shared `SessionBudget`. The accepted `08B` path must be preserved: child and descendant spend must still count against the parent session budget. Because the current query-owned seam supports only one `SessionBudget` per shared `session_id`, no implementation may map child `budget_usd` to `QueryConfig.max_budget_usd`, replace the parent registration, or add `ToolContext.session_budget` / `ToolContext.health_cache`.`
>
> `Split this work again if needed. First execute a query-owned seam ticket that adds the minimal layered/nested child-budget mechanism required in `session_budget.rs` and `query::lib.rs` while preserving parent accounting and nested child inheritance. Only after that ticket is accepted may a follow-on child/team carriage ticket add and wire `AgentInput.budget_usd`, `AgentSpec.budget_usd`, and `AgentRunParams.budget_usd` through `agent_tool.rs` and `team_tool.rs`. HealthCache, provider fallback policy, QueryEvent expansion, and TeamCreate outer-cancellation redesign remain out of scope.`

## whether `TASK-M11-10B` is sufficient as a single ticket

`no`

## notes / risks

- The live blocker is larger than simple registry overwrite. The current runtime budget model is single-budget end to end:
  - one `QueryConfig.session_budget`
  - one registry entry per `session_id`
  - one spend-record/check path in `run_query_loop()`
- Because `08B` is accepted baseline, any child budget seam that suppresses parent shared-session accounting would be a regression, not a valid interpretation.
- If future authority insists on keeping `10B` as one code ticket anyway, it should explicitly authorize edits in:
  - `src-rust/crates/query/src/session_budget.rs`
  - `src-rust/crates/query/src/lib.rs`
  - `src-rust/crates/query/src/agent_tool.rs`
  - `src-rust/crates/tools/src/team_tool.rs`
  and it should still preserve the exclusions listed above.
- No live evidence in this pass tied `HealthCache` or TeamCreate outer cancellation to child `budget_usd` inseparably.
