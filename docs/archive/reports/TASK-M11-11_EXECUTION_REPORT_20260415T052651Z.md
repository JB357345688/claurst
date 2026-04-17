# TASK-M11-11 Execution Report

## 1. ticket id

`TASK-M11-11`

This execution is for revised `TASK-M11-11 = QueryEvent expansion / observability`, not the stale MPWO numbering where QueryEvent work appeared as `M11-10`.

## 2. execution verdict

`PASS-WITH-NOTES`

## 3. timestamp UTC

`2026-04-15T05:26:51Z`

## 4. branch / HEAD before execution

- Branch: `feature/provider-resolution-seam`
- HEAD: `bfabfd5934b0afe801f2e7de9f15a7f6258c563b`
- Match to required baseline: `yes`

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
- `docs/archive/reports/TASK-M11-10B_SPLIT_AUTHORITY_REPORT_20260415T031240Z.md`
- `docs/archive/reports/TASK-M11-10B1_CLOSEOUT_REPORT_20260415T034453Z.md`
- `docs/archive/reports/TASK-M11-10B2_CLOSEOUT_REPORT_20260415T041344Z.md`
- `docs/archive/reports/M11_CONVERGENCE_REVIEW_20260415T042825Z.md`
- `docs/archive/reports/TASK-M11-11_PREFLIGHT_REPORT_20260415T045550Z.md`

## 6. files changed

- `src-rust/crates/query/src/lib.rs`
- `src-rust/crates/query/src/agent_tool.rs`
- `src-rust/crates/query/src/session_budget.rs`
- `src-rust/crates/tui/src/app.rs`

## 7. exact changes made

- Added these `QueryEvent` variants in `src-rust/crates/query/src/lib.rs` exactly as required:
  - `WorkerProviderResolved { agent_id: String, provider_id: String, model_id: String, was_fallback: bool }`
  - `WorkerBudgetExceeded { agent_id: String, cost_usd: f64, limit_usd: f64 }`
  - `SessionBudgetExceeded { cost_usd: f64, limit_usd: f64 }`
- Added narrow query-side event extraction helpers in `lib.rs` so the parent loop can emit worker events from existing result-carried data without adding new channels or broadening `ToolContext`.
- Emitted `SessionBudgetExceeded` in `lib.rs` at both existing shared-session `record_cost(...)` plus `check_and_cancel()` sites, using the shared root budget object and only on the transition where the shared session budget first becomes cancelled.
- Added narrow read-only `SessionBudget` accessors in `src-rust/crates/query/src/session_budget.rs`:
  - `limit_usd()`
  - `spent_usd()`
  - `is_limit_exceeded()`
- In `src-rust/crates/query/src/agent_tool.rs`, attached structured observability data for direct AgentTool child runs via `ToolResult.metadata`.
- In `src-rust/crates/query/src/agent_tool.rs`, attached structured observability data for the cc-query-backed team-runner path via a narrow encoded runner-result suffix, then stripped and decoded that suffix in `lib.rs` before the parent emitted events or returned tool content to the model.
- Added only no-op exhaustive-match handling in `src-rust/crates/tui/src/app.rs`; no rendering/UI behavior was introduced for the new events.

## 8. event transport seam summary

- Direct AgentTool foreground/background path:
  - provider-resolution observability is returned to the parent via `ToolResult.metadata`
  - synchronous AgentTool completions also return child-local budget-exceeded observability through that same metadata seam when applicable
- Team-runner path used by `TeamCreate`:
  - the injected runner in `agent_tool.rs` appends a narrow encoded observability payload to each per-agent result string
  - `query::lib.rs` decodes and strips that payload from the final `TeamCreate` tool result, rebuilds clean aggregated output, and emits parent-side `QueryEvent`s
- Shared session-budget path:
  - `query::lib.rs` emits `SessionBudgetExceeded` directly after the existing shared-budget `record_cost(...)` plus `check_and_cancel()` flow
- No new global channels were introduced.
- `ToolContext.session_budget` was not added.
- `ToolContext.health_cache` was not added.

## 9. provider/budget event mapping summary

- `WorkerProviderResolved`
  - implemented on the existing fallback-aware child/team provider-resolution seam only
  - emitted from:
    - direct foreground AgentTool child runs
    - direct background AgentTool child starts
    - cc-query-backed team-runner child runs
  - `was_fallback` is computed by comparing the requested provider/model identity against the resolved execution target
  - `agent_id` source used:
    - direct AgentTool path: existing generated UUID `agent_id`
    - team-runner path: existing live description identifier `team_name/agent_name`
- `WorkerBudgetExceeded`
  - implemented against child-local layered `budget_usd` semantics only
  - not emitted for `max_budget_usd`
  - payload is emitted only when a child run had `budget_usd` configured and that child scope's own `SessionBudget` limit was exceeded
- `SessionBudgetExceeded`
  - implemented against shared-session `SessionBudget` semantics only
  - not emitted for child-local `budget_usd`
  - not emitted for `max_budget_usd`

## 10. validation commands run

- `cd src-rust && cargo check --workspace`

## 11. validation results

- Result: `PASS`
- Output summary:
  - `Checking claurst-query v0.0.8 (/home/jordi/claurst/src-rust/crates/query)`
  - `Compiling claurst v0.0.8 (/home/jordi/claurst/src-rust/crates/cli)`
  - `Checking claurst-bridge v0.0.8 (/home/jordi/claurst/src-rust/crates/bridge)`
  - `Checking claurst-tui v0.0.8 (/home/jordi/claurst/src-rust/crates/tui)`
  - `Checking claurst-commands v0.0.8 (/home/jordi/claurst/src-rust/crates/commands)`
  - `Finished dev profile [unoptimized + debuginfo] target(s) in 2.91s`

## 12. deviations from ticket, if any

- None in authority scope.
- Note:
  - `src-rust/crates/tui/src/app.rs` had to be touched for compile fallout because `QueryEvent` matching there is exhaustive. The change was limited to no-op branches only, exactly per ticket allowance.
  - Detached background AgentTool runs surface `WorkerProviderResolved` at launch on the parent event stream, but post-launch child-runtime observability still depends on the existing detached background result path. This execution did not widen that path beyond the narrow ticket seam.

## 13. blockers, if any

- None.

## 14. hosted Ollama invariant assessment

`preserved`

Basis:

- no hosted-Ollama-specific request-shaping or auth logic was touched
- no provider-resolution policy changes were made
- `src-rust/crates/query/src/provider_resolution.rs` was not edited
- `src-rust/crates/query/src/health_cache.rs` was not edited
- fallback behavior remains on the already-landed same-domain resolution seam

## 15. ready for verification

`yes`
