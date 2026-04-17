# TASK-M11-08B Preflight Report

## ticket id

`TASK-M11-08B`

## verdict

`PASS-WITH-NOTES`

## timestamp UTC

`2026-04-15T00:57:53Z`

## branch / HEAD observed

- Branch: `feature/provider-resolution-seam`
- HEAD: `25518cac29d34353cb58c8811da1040a3da69247`
- Working tree state:
  - modified `.gitignore`
  - untracked docs/report artifacts
  - untracked `.codex`
  - untracked `src-rust/target/`

## authority reviewed

- `AGENTS.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md`
- `docs/archive/reports/M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md`
- `docs/archive/reports/TASK-M11-08_AUTHORITY_RECONCILIATION_REPORT_20260415T002331Z.md`
- `docs/archive/reports/TASK-M11-08R_CLOSEOUT_REPORT_20260415T004401Z.md`
- `docs/archive/reports/TASK-M11-08B_AUTHORITY_REPORT_20260415T005148Z.md`

## accepted-baseline comparison

- Expected accepted branch: `feature/provider-resolution-seam`
- Expected accepted latest HEAD: `25518cac29d34353cb58c8811da1040a3da69247`
- Observed branch matches expected: `yes`
- Observed HEAD matches expected: `yes`
- Corrected `TASK-M11-08R` is present at the accepted head and remains the active root-only baseline.
- I did not independently re-verify every earlier closed-ticket commit hash listed in the prompt; I treated that accepted baseline list as controlling evidence and directly verified the latest accepted head only.

## verified target files / symbols / commands

Verified files:

- `src-rust/crates/query/src/agent_tool.rs`
- `src-rust/crates/query/src/lib.rs`
- `src-rust/crates/query/src/session_budget.rs`
- `src-rust/crates/query/src/health_cache.rs`
- `src-rust/crates/tools/src/team_tool.rs`
- `src-rust/crates/tools/src/lib.rs`
- `src-rust/crates/query/Cargo.toml`
- `src-rust/crates/tools/Cargo.toml`
- `src-rust/crates/cli/src/main.rs`
- `src-rust/crates/core/src/lib.rs`

Verified symbols / seams:

- `QueryConfig.session_budget`
- `run_query_loop()`
- `SessionBudget::child_cancel_token()`
- `root_query_cancel_token()`
- `ToolContext.session_id`
- `AgentRunParams`
- `register_agent_runner()`
- `ACTIVE_TEAMS`
- `CostTracker::set_agent_id()` / `set_provider_id()`
- `resolve_provider_with_fallback()`

Commands run:

- `git branch --show-current`
- `git rev-parse HEAD`
- `git status --short`
- `rg`
- `sed`
- `nl -ba`
- `cat`
- `cd src-rust && cargo check --workspace`

## exact scope confirmation in current repo reality

The temporary `TASK-M11-08B` authority is structurally valid against the live repo.

Confirmed in current code:

- root `SessionBudget` wiring already exists and is active only at root scope
- child/team query loops still do not inherit shared session-budget state
- no concrete query-owned types currently exist on `ToolContext`
- `team_tool.rs` still owns only outer TeamDelete cancellation, not inner query-loop budget propagation
- `HealthCache` is present in `claurst-query` but is not on the active child/team runtime path for this ticket

There is no live-repo conflict that forces scope widening beyond the temporary authority.

## live child/team seam findings

### Root baseline after corrected `TASK-M11-08R`

- `QueryConfig` carries `session_budget: Option<Arc<SessionBudget>>` at `src-rust/crates/query/src/lib.rs:115-117`.
- `run_query_loop()` records cost and calls `check_and_cancel()` when `session_budget` is present at:
  - `src-rust/crates/query/src/lib.rs:1140-1143`
  - `src-rust/crates/query/src/lib.rs:1410-1413`
- `run_query_loop()` observes cancellation only through its `cancel_token` parameter at `src-rust/crates/query/src/lib.rs:741-743`.
- root CLI currently derives the root cancel token from the session budget via `root_query_cancel_token()` at `src-rust/crates/cli/src/main.rs:104-110`.

This confirms corrected `TASK-M11-08R` landed and remains root-only.

### Foreground AgentTool child path

- Child `QueryConfig` is built in `src-rust/crates/query/src/agent_tool.rs:362-384`.
- Current child config explicitly sets `session_budget: None` at `src-rust/crates/query/src/agent_tool.rs:377`.
- Foreground child run creates a fresh unrelated cancellation token at `src-rust/crates/query/src/agent_tool.rs:454-465`.

### Background AgentTool child path

- Background path reuses the same child `QueryConfig` built at `src-rust/crates/query/src/agent_tool.rs:362-384`.
- It also therefore carries `session_budget: None`.
- Background child run creates a fresh unrelated cancellation token at `src-rust/crates/query/src/agent_tool.rs:408-419`.

### Query-backed team-runner path

- `claurst-tools` exposes `AgentRunParams` containing only crate-neutral data plus `ctx: Arc<ToolContext>` at `src-rust/crates/tools/src/team_tool.rs:36-46`.
- `TeamCreateTool` fans out per-agent futures and calls `run_agent(AgentRunParams { ... })` at `src-rust/crates/tools/src/team_tool.rs:376-423`.
- The real runner is injected from `claurst-query` via `register_agent_runner()` and the query-backed closure builds the child `QueryConfig` at `src-rust/crates/query/src/agent_tool.rs:621-633`.
- That team-runner child config also explicitly sets `session_budget: None` at `src-rust/crates/query/src/agent_tool.rs:629`.
- The inner team-runner child loop creates a fresh unrelated cancellation token at `src-rust/crates/query/src/agent_tool.rs:638-648`.

### Current available handoff

- `ToolContext` carries `session_id` but not `QueryConfig` or `SessionBudget`: `src-rust/crates/tools/src/lib.rs:216-235`.
- `SessionBudget` already exposes `child_cancel_token()` at `src-rust/crates/query/src/session_budget.rs:30-32`.
- `agent_tool.rs` and the injected team-runner closure both already live in `claurst-query`, where `SessionBudget` is owned.

Conclusion:

- the currently reachable child/team seam remains query-owned
- there is no existing narrower shared-budget carrier already flowing through child/team paths
- `ToolContext.session_id` is the only already-carried cross-path key that can support a query-owned lookup without invalid crate coupling

## anticipated implementation shape

Most likely narrow implementation shape:

- keep `team_tool.rs` read-only
- keep `ToolContext` read-only with no concrete query-owned additions
- add or reuse a small query-owned lookup keyed by `ToolContext.session_id`
- use that lookup in `src-rust/crates/query/src/agent_tool.rs` at both child `QueryConfig` construction points
- replace fresh child `CancellationToken::new()` calls with `SessionBudget::child_cancel_token()` when a parent session budget is available
- continue using plain fresh tokens when no parent session budget exists

Preflight judgment on seam options:

- existing narrower seam already available without helper: `no`
- session-id keyed query-owned registry/helper likely required: `yes`

Reason:

- current child/team paths receive only `ToolContext`, not parent `QueryConfig` or a session-budget handle
- `ToolContext.session_id` already crosses all three child/team paths
- adding only a neutral `CancellationToken` carrier would not solve shared spend recording on child `QueryConfig`

## anticipated compile-fallout scope

Expected fallout is narrow.

Current explicit `session_budget` construction sites found by grep:

- `src-rust/crates/query/src/lib.rs:155` (`Default` for `QueryConfig`)
- `src-rust/crates/query/src/lib.rs:2162` (test helper `make_config()`)
- `src-rust/crates/query/src/agent_tool.rs:377` (foreground/background child config)
- `src-rust/crates/query/src/agent_tool.rs:629` (team-runner child config)
- `src-rust/crates/cli/src/main.rs:105` (root cancel-token helper takes `SessionBudget`)

Expected implementation-touch set:

- definitely:
  - `src-rust/crates/query/src/agent_tool.rs`
- likely:
  - `src-rust/crates/query/src/session_budget.rs` or a new small query-owned helper module
- possibly:
  - `src-rust/crates/query/src/lib.rs` only if helper exposure or tests need adjustment
- not currently indicated:
  - `src-rust/crates/tools/src/team_tool.rs`
  - `src-rust/crates/tools/src/lib.rs`
  - `src-rust/crates/cli/src/main.rs`

Preflight expectation:

- compile fallout should stay localized to query-owned code plus query tests
- there is no current evidence of required cross-crate signature churn

## validation command run and result

- Command run: `cd src-rust && cargo check --workspace`
- Result: `PASS`
- Output summary:
  - `Compiling claurst v0.0.8 (/home/jordi/claurst/src-rust/crates/cli)`
  - `Finished dev profile [unoptimized + debuginfo] target(s) in 0.30s`

## drift found

- Worktree noise exists outside the intended ticket scope:
  - modified `.gitignore`
  - many untracked docs/report artifacts
  - untracked `.codex`
  - untracked `src-rust/target/`
- No structural drift was found against the temporary `TASK-M11-08B` authority.
- One planning drift remains historical only:
  - older M10/MPWO wording implied `ToolContext.session_budget` / `ToolContext.health_cache`
  - live repo reality and accepted authority reports supersede that for this ticket

## blockers, if any

`none`

Notes:

- The ticket is implementable as a single narrow follow-up.
- The only notable preflight note is that a query-owned session-id keyed helper is likely needed because no even narrower seam already exists.

## hosted Ollama invariant assessment

`preserved if ticket stays in scope`

Assessment:

- hosted Ollama compatibility is not on the active runtime path for this ticket
- `TASK-M11-08B` can stay entirely away from provider resolution and provider materialization
- `resolve_provider_with_fallback()` from `TASK-M11-05` can remain untouched
- `CostTracker` attribution from `TASK-M11-06` is already landed in `src-rust/crates/core/src/lib.rs:2850-2890` and does not need redesign for this ticket
- corrected root-only `TASK-M11-08R` can remain intact if root wiring is not altered

Risk review against already-closed work:

- hosted Ollama compatibility risk: low if no provider-resolution edits are made
- corrected `TASK-M11-08R` risk: low if root entrypoints remain unchanged and only child/team seams are extended
- same-domain fallback behavior from `TASK-M11-05` risk: low if `provider_resolution.rs` remains untouched
- CostTracker attribution from `TASK-M11-06` risk: low if this ticket only reuses the existing shared tracker/session budget path and does not alter attribution APIs

## exact recommendation for next step

Proceed to `TASK-M11-08B` implementation with the following strict boundary:

- edit only query-owned seams
- keep `src-rust/crates/tools/src/team_tool.rs` read-only unless implementation proves a minimal interface adjustment is truly unavoidable
- do not add concrete query-owned fields to `ToolContext`
- implement shared `SessionBudget` propagation for:
  - foreground AgentTool child runs
  - background AgentTool child runs
  - query-backed team-runner child loops
- use a query-owned session-id keyed lookup/helper unless implementation discovers an even narrower query-owned seam
- preserve root-only `TASK-M11-08R`, hosted Ollama behavior, same-domain fallback behavior, and CostTracker attribution behavior unchanged
