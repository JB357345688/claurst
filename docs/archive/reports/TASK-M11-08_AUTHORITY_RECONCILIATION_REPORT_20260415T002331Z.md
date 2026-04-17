# TASK-M11-08 Authority Reconciliation Report

## ticket id

`TASK-M11-08-AUTH`

## timestamp UTC

`2026-04-15T00:23:31Z`

## authority inputs reviewed

- `AGENTS.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md`
- `docs/archive/reports/M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md`
- `docs/archive/reports/TASK-M11-08_PREFLIGHT_REPORT_20260414T153327Z.md`
- `docs/archive/reports/TASK-M11-08_EXECUTION_REPORT_20260414T223857Z.md`
- `docs/archive/reports/TASK-M11-08_CORRECTIVE_PREFLIGHT_REPORT_20260414T230240Z.md`

## blocker summary

- `ToolContext` is defined in `claurst-tools`.
- `SessionBudget` and `HealthCache` are defined in `claurst-query`.
- `claurst-query` already depends on `claurst-tools`.
- Therefore the M10/M11-08 wording that implies adding concrete `Option<Arc<SessionBudget>>` and `Option<Arc<HealthCache>>` fields directly to `ToolContext` is not executable as written, because it would require an invalid reverse dependency from `claurst-tools` to `claurst-query`.
- The current child/team seams only receive `ToolContext`, not the parent `QueryConfig`, so child/team `SessionBudget` propagation is not currently reachable through existing live repo seams.
- `HealthCache` is not on the current M11-08 runtime path; it is only consumed by `resolve_provider_with_fallback()`, which is not yet the active runtime path for this ticket.

## why current M11-08 is not executable as written

- The current plan text for M11-08 requires both:
  - `QueryConfig.session_budget`
  - `ToolContext.session_budget` and `ToolContext.health_cache`
- `QueryConfig.session_budget` is structurally valid inside `claurst-query`.
- `ToolContext.session_budget` and `ToolContext.health_cache` are not structurally valid with the current crate ownership.
- Child/team propagation in the current plan depends on those invalid `ToolContext` additions:
  - `agent_tool.rs` child query loops build fresh child `QueryConfig` values from `ToolContext`
  - `team_tool.rs` outer team cancellation and the injected query-side team runner also only receive `ToolContext`
- Because the required handoff seam is invalid, current M11-08 cannot be executed faithfully as written without redesign or widened authority.
- The smallest valid correction is therefore to reduce M11-08 to the root-session behavior that is already directly reachable, and to defer child/team propagation plus `health_cache` plumbing.

## minimal corrected authority for M11-08

Decision:
- Yes, M11-08 should be split in effect into:
  - a reduced executable corrected M11-08 now
  - a later follow-up ticket for child/team session-budget propagation

Corrected M11-08 should be limited to:
- root CLI `--budget-usd`
- root `SessionBudget` creation
- `QueryConfig.session_budget`
- root `run_query_loop()` cost recording and `check_and_cancel()`
- root cancellation-token wiring where directly reachable in existing root invocation paths
- no `ToolContext.session_budget`
- no `ToolContext.health_cache`
- no child/team propagation in this ticket

Corrected M11-08 remains a budget-and-root-cancellation ticket only.

What remains in corrected M11-08:
- add the root session-budget CLI surface as a separate cross-session USD budget, without conflating it with existing `max_budget_usd`
- create `SessionBudget` at the root session entrypoint
- carry it on `QueryConfig`
- record/query-loop spend against it in `run_query_loop()`
- call `check_and_cancel()` in `run_query_loop()`
- wire root query-loop cancellation to observe the root session budget where directly reachable from existing root call sites

What is explicitly removed from corrected M11-08:
- any requirement to place `SessionBudget` on `ToolContext`
- any requirement to place `HealthCache` on `ToolContext`
- any requirement to change `team_tool.rs`
- any requirement to modify child/team `CancellationToken` creation
- any requirement to propagate session budget into `AgentTool` or team-runner child loops

## explicitly deferred items

- child/team session-budget propagation
- any `ToolContext` carriage of `SessionBudget`
- any `ToolContext` carriage of `HealthCache`
- `agent_tool.rs` child-token changes
- `team_tool.rs` per-agent cancellation-token changes
- any authority that assumes child/team loops inherit session-budget cancellation automatically
- `health_cache` runtime plumbing for M11-08

Deferral decision:
- `health_cache` should be explicitly deferred. It is not on the current M11-08 runtime path and should not remain implied inside corrected M11-08.

Recommended deferred follow-up scope:
- a later authority-approved follow-up ticket, inserted after corrected M11-08, for child/team session-budget propagation and any associated seam reconciliation

## recommended sequencing impact

- A corrected M11-08 execution pass may proceed immediately after authority acceptance.
- Downstream work must not assume that corrected M11-08 provides child/team session-budget propagation.
- A new inserted follow-up ticket is required before any downstream ticket that depends on:
  - child/team session-budget propagation
  - `ToolContext.session_budget`
  - `ToolContext.health_cache`
  - worker/session budget events that depend on child propagation

Recommended sequencing consequence:
- corrected `TASK-M11-08` executes now as root-only
- insert a new follow-up authority ticket after corrected `TASK-M11-08`
- do not treat corrected `TASK-M11-08` as satisfying the old child/team propagation expectation
- resume later downstream tickets only against the corrected dependency understanding

Practical interpretation:
- corrected M11-08 can run next
- but downstream execution order must be treated as patched: a follow-up ticket is required before downstream work that relied on the old full-scope M11-08 assumption

## exact proposed wording snippet for GPT/WebUI to adopt as temporary corrected authority

`TEMPORARY AUTHORITY CORRECTION FOR TASK-M11-08`

`TASK-M11-08 is reduced to root-session budget and cancellation wiring only. Implement only: (1) root CLI --budget-usd, (2) root SessionBudget creation, (3) QueryConfig.session_budget, (4) run_query_loop() session-budget cost recording and check_and_cancel(), and (5) root cancellation-token observation where directly reachable from existing root invocation paths.`

`Do NOT add SessionBudget or HealthCache fields to ToolContext. Do NOT modify team_tool.rs. Do NOT implement child/team session-budget propagation in this ticket. Do NOT add ToolContext.health_cache in this ticket.`

`Child/team session-budget propagation is deferred to a later follow-up ticket. HealthCache runtime plumbing is also deferred because it is not on the current M11-08 runtime path.`

## whether execution may resume after this correction

`yes`

With the following boundary:
- execution may resume for corrected M11-08 only
- execution may not assume the old full-scope M11-08 semantics
- downstream work that needs child/team propagation still requires a later follow-up authority ticket

## notes / risks

- Risk: if the temporary correction is not stated explicitly, a later implementation pass may reintroduce the invalid `ToolContext` concrete-field assumption.
- Risk: `max_budget_usd` and `budget_usd` remain distinct mechanisms. Corrected M11-08 must keep the new root session budget separate from existing per-loop `max_budget_usd`.
- Risk: current M10 planning text and the MPWO revision summary still describe the broader M11-08 shape. This report should be treated as the controlling temporary interpretation for GPT/WebUI until canonical authority is formally revised.
- Risk: `QueryEvent` or worker-budget event work must not assume child/team propagation has already landed.
- Hosted Ollama non-regression remains binding; corrected M11-08 should stay bookkeeping/cancellation-only and must not alter provider resolution or materialization behavior.
