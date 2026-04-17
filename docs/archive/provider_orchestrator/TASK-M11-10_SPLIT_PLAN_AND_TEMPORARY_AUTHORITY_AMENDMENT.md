# M11-10 Split Plan and Temporary Authority Amendment

> Status: historical / superseded / non-controlling for active use.
>
> This file records the split-era authority used during the M11-10A / M11-10B transition.
> Active current authority now lives in `docs/Current/MPWO_WORK_ORDER_PACK.md`.
> Use this file only for historical traceability.
>
> Traceability:
> - `docs/archive/reports/TASK-M11-10B_SPLIT_AUTHORITY_REPORT_20260415T031240Z.md`
> - `docs/archive/reports/TASK-M11-12_CLOSEOUT_REPORT_20260415T065244Z.md`

---

## Status

This document is a temporary authority amendment for the M11 D2 ladder.

It is intended to resolve the `TASK-M11-10` halt and to preserve alignment with the accepted corrected path established by:
- corrected `TASK-M11-08R` root session-budget wiring
- `TASK-M11-08B` child/team session-budget propagation
- revised `TASK-M11-09` child `max_tokens` override wiring.

This amendment supersedes any stale project-file wording that still treats `TASK-M11-10` as one narrow executable ticket containing both `allow_fallback` and `budget_usd` on the current repo seam. The accepted `TASK-M11-10` preflight concluded that this combined ticket must halt because `allow_fallback` remains a narrow fit, but `budget_usd` does not remain narrow on the accepted `08B` session-budget seam.

## Decision

`TASK-M11-10` is split into two tickets:

- `TASK-M11-10A` — child `allow_fallback` schema and runtime wiring
- `TASK-M11-10B` — child `budget_usd` semantics and seam resolution

This split is mandatory on the current accepted path. `TASK-M11-10A` may proceed as a normal narrow ticket. `TASK-M11-10B` requires its own authority and preflight because child-specific budget behavior is not a narrow wiring change on the currently accepted `08B` session-id keyed propagation seam.

## Accepted baseline this split must preserve

All future M11 work must treat the following as fixed accepted baseline:

- `TASK-M11-08R` closed at commit `25518cac29d34353cb58c8811da1040a3da69247`
- `TASK-M11-08B` closed at commit `b4ad28ee9eb1e434f935fd2de70c4b402a5c74da`
- `TASK-M11-09` closed at commit `4ef9547dab51959f7b39c473f929b81f05ee1134`

The corrected path is therefore:

- root session-budget wiring exists
- child/team session-budget propagation exists through a query-owned session-id keyed seam
- child `max_tokens` override wiring exists across foreground, background, and query-backed team-runner paths.

Any future ticket that assumes the older broad pre-correction M11-08 shape is out of date. The rejected assumptions remain rejected, including concrete `ToolContext.session_budget` and `ToolContext.health_cache` additions.

## Why the split is required

The `TASK-M11-10` preflight found:

- `allow_fallback` is still implementable as a narrow schema/runtime wiring ticket
- `budget_usd` is not narrow on the accepted current seam
- the problem is that the accepted `08B` propagation seam registers `SessionBudget` by shared `session_id`
- a distinct child `SessionBudget` under that same session key would overwrite the parent registration
- the ticket also explicitly forbids conflating child `budget_usd` with `max_budget_usd`.

So the blocker is not fallback. The blocker is child-specific budget semantics on the current seam. That requires separate authority.

## Temporary authority for `TASK-M11-10A`

### Ticket name
`TASK-M11-10A`

### Purpose
Add child `allow_fallback` schema and runtime wiring on the accepted corrected path.

### In scope
Add and wire:

- `AgentInput.allow_fallback: Option<bool>`
- `AgentSpec.allow_fallback: Option<bool>`
- `AgentRunParams.allow_fallback: bool`

Apply this across all three child spawn paths:

- foreground AgentTool child runs
- background AgentTool child runs
- cc-query-backed team-runner child loops invoked through `register_agent_runner()`.

### Required behavior
When `allow_fallback` is omitted, it defaults to `false`.

When `allow_fallback` is true, child provider resolution may use the already-existing fallback-aware provider resolution seam.

This ticket may wire child paths to `resolve_provider_with_fallback(...)` narrowly, without redesigning provider policy. The `TASK-M11-10` preflight found that this can be done without reopening the rejected broad M11-08 assumptions. It also found that runtime-local `HealthCache::new()` use is plausible without adding `ToolContext.health_cache`.

### Allowed implementation shape
Default primary file scope:

- `src-rust/crates/query/src/agent_tool.rs`
- `src-rust/crates/tools/src/team_tool.rs`

Supporting query-side imports or narrow local fallback helper use are allowed if preflight confirms they remain narrow.

### Explicitly excluded
`TASK-M11-10A` must not include:

- child `budget_usd`
- any `ToolContext.session_budget`
- any `ToolContext.health_cache`
- TeamCreate outer-cancellation redesign
- root query redesign
- event expansion
- hosted Ollama behavior change
- `allow_cross_domain_fallback`
- `max_tokens` redesign
- any budget-mechanism conflation.

## Temporary authority for `TASK-M11-10B`

### Ticket name
`TASK-M11-10B`

### Purpose
Resolve child `budget_usd` semantics on the accepted corrected path.

### Reason this is separate
On the current accepted path, child/team session-budget propagation is keyed by shared `session_id`. The preflight found that a child-specific `SessionBudget` would overwrite the parent mapping under the current registry behavior. That makes child `budget_usd` a seam-definition problem, not a narrow field-wiring problem.

### Required first step
`TASK-M11-10B` must begin with authority clarification and preflight before any implementation prompt.

### Questions `TASK-M11-10B` must answer explicitly
1. What does child `budget_usd` mean on the corrected path?
2. Does it create a child-specific `SessionBudget`, or something else?
3. How does that interact with inherited parent `SessionBudget` from `08B`?
4. How are parent and child registrations separated without breaking the accepted session-id keyed propagation seam?
5. How is budget behavior kept distinct from:
   - `max_tokens`
   - `max_budget_usd`
   - root shared `SessionBudget`?

### Explicitly excluded
Until `TASK-M11-10B` authority is accepted, no ticket may silently:
- reinterpret child `budget_usd` as `max_budget_usd`
- reopen or redesign the `08B` query-owned seam without explicit authority
- add concrete query-owned budget/cache fields to `ToolContext`
- claim that child budget behavior is already settled.

## Global guardrails for both 10A and 10B

These are binding guardrails for Codex and for future prompts:

1. Do not assume `ToolContext.session_budget` exists or should be added.
2. Do not assume `ToolContext.health_cache` exists or should be added.
3. Do not silently revert to the older broad pre-correction M11-08 assumptions.
4. Do not silently redesign the accepted `08B` query-owned session-id keyed propagation seam.
5. Do not silently redesign TeamCreate outer per-agent cancellation.
6. Do not conflate:
   - `max_tokens`
   - `max_budget_usd`
   - shared `SessionBudget`
   - child `budget_usd`.

## Sequencing rule

The revised ladder from this point is:

- `TASK-M11-10A` preflight
- `TASK-M11-10A` execution / verification / closeout
- `TASK-M11-10B` authority clarification
- `TASK-M11-10B` preflight
- `TASK-M11-10B` execution / verification / closeout
- then continue to `TASK-M11-11`

Until `TASK-M11-10B` is resolved and closed, downstream work must not assume child `budget_usd` semantics are settled.

## Controlling interpretation for future prompts

Use this wording in future orchestration prompts:

> `TASK-M11-10` is split. `TASK-M11-10A` is child `allow_fallback` schema/runtime wiring only. `TASK-M11-10B` is child `budget_usd` semantics and seam resolution. Do not combine them. Treat accepted `08R`, `08B`, and `09` as fixed baseline. Do not assume ToolContext carries concrete query-owned budget/cache fields. Do not redesign the accepted query-owned session-id keyed propagation seam unless `TASK-M11-10B` authority explicitly allows it.`

## Operational conclusion

This split is the authoritative corrected path for M11 from this point forward.

`TASK-M11-10A` is the next normal executable ticket.

`TASK-M11-10B` is deferred pending explicit authority and separate preflight.
