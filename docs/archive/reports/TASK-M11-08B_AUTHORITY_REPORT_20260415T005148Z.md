# TASK-M11-08B Authority Report

## ticket id

`TASK-M11-08B`

## timestamp UTC

`2026-04-15T00:51:48Z`

## authority inputs reviewed

- `AGENTS.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md`
- `docs/archive/reports/M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md`
- `docs/archive/reports/TASK-M11-08_AUTHORITY_RECONCILIATION_REPORT_20260415T002331Z.md`
- `docs/archive/reports/TASK-M11-08R_CLOSEOUT_REPORT_20260415T004401Z.md`
- live repo files:
  - `src-rust/crates/query/src/agent_tool.rs`
  - `src-rust/crates/tools/src/team_tool.rs`
  - `src-rust/crates/tools/src/lib.rs`
  - `src-rust/crates/query/src/lib.rs`
  - `src-rust/crates/query/src/session_budget.rs`
  - `src-rust/crates/query/src/health_cache.rs`
  - `src-rust/crates/query/Cargo.toml`
  - `src-rust/crates/tools/Cargo.toml`

## current baseline checked

- Branch: `feature/provider-resolution-seam`
- HEAD: `25518cac29d34353cb58c8811da1040a3da69247`
- `TASK-M11-08R` closeout says corrected root-only scope is closed on this commit.
- Worktree is not clean:
  - modified `.gitignore`
  - many untracked docs/report paths
  - untracked `.codex`
  - untracked `src-rust/target/`
- Repo noise does not block this docs-only authority pass, but it means the review basis for any later code ticket must stay explicit.

## missing capability after corrected M11-08R

The missing capability is still exactly this: the already-existing root `SessionBudget` does not propagate into child query loops or team-runner child loops, so child/team work neither records spend against the shared session budget nor observes session-budget cancellation through the live child runtime path.

Live proof:

- `QueryConfig` already has `session_budget: Option<Arc<SessionBudget>>` at `src-rust/crates/query/src/lib.rs:115-117`.
- `run_query_loop()` already records spend and calls `check_and_cancel()` when `config.session_budget` is present at `src-rust/crates/query/src/lib.rs:1140-1143` and `1409-1413`.
- `run_query_loop()` only exits on cancellation by observing the passed `cancel_token` at `src-rust/crates/query/src/lib.rs:741-743`.
- `SessionBudget` already exposes `child_cancel_token()` at `src-rust/crates/query/src/session_budget.rs:30-32`.
- But both child-query builders in `agent_tool.rs` explicitly set `session_budget: None`:
  - foreground/background AgentTool path at `src-rust/crates/query/src/agent_tool.rs:362-384`
  - injected team-runner path at `src-rust/crates/query/src/agent_tool.rs:621-633`
- All three child/team query-loop launches still create fresh unrelated cancellation tokens:
  - background AgentTool at `src-rust/crates/query/src/agent_tool.rs:408-419`
  - foreground AgentTool at `src-rust/crates/query/src/agent_tool.rs:454-465`
  - query-backed team runner at `src-rust/crates/query/src/agent_tool.rs:638-648`

Result:

- corrected `TASK-M11-08R` solved only root-session budget wiring
- foreground child agents do not inherit shared session-budget spend/cancellation
- background child agents do not inherit shared session-budget spend/cancellation
- team-runner child loops do not inherit shared session-budget spend/cancellation

## live seam findings

### 1. Live structures that currently carry data into child runs and team runs

- `AgentInput` in `agent_tool.rs` carries only:
  - `description`
  - `prompt`
  - `tools`
  - `system_prompt`
  - `max_turns`
  - `model`
  - `provider`
  - `isolation`
  - `run_in_background`
  - reference: `src-rust/crates/query/src/agent_tool.rs:134-163`
- `ToolContext` carries only crate-neutral session/runtime data:
  - `cost_tracker`
  - `session_id`
  - `config`
  - `provider_registry`
  - `model_registry`
  - reference: `src-rust/crates/tools/src/lib.rs:214-235`
- `AgentRunParams` carries only:
  - `description`
  - `prompt`
  - `tools`
  - `system_prompt`
  - `max_turns`
  - `ctx: Arc<ToolContext>`
  - `provider_override`
  - `model_override`
  - reference: `src-rust/crates/tools/src/team_tool.rs:36-46`
- `TeamCreateTool` clones the same parent `ToolContext` into each agent future and drives the query-backed runner through `run_agent(AgentRunParams { ... })`:
  - `src-rust/crates/tools/src/team_tool.rs:376-423`
- Inside `claurst-query`, both child execution sites already construct the child `QueryConfig` and already own the call to `run_query_loop()`:
  - AgentTool path at `src-rust/crates/query/src/agent_tool.rs:362-465`
  - team-runner injection path at `src-rust/crates/query/src/agent_tool.rs:621-648`

### 2. Plausible narrow handoff points that avoid the invalid concrete dependency

The narrow structurally valid seam is query-owned propagation keyed by the already-carried `ToolContext.session_id`.

Basis:

- `ToolContext.session_id` already crosses all child/team paths without introducing new crate coupling: `src-rust/crates/tools/src/lib.rs:220-235`
- `AgentTool` and the team-runner injection closure both live in `claurst-query`, so both can consult query-owned state without adding `claurst-tools -> claurst-query` references.
- The repo already uses process-global registries keyed by session id or similar ownership boundaries:
  - `session_shell_state()` / `clear_session_shell_state()` in `claurst-tools`: `src-rust/crates/tools/src/lib.rs:185-196`
  - `session_snapshot()` / `clear_session_snapshot()` in `claurst-tools`: `src-rust/crates/tools/src/lib.rs:199-211`
  - background-agent registry in `claurst-query`: `src-rust/crates/query/src/agent_tool.rs:38-64`

Inference from current repo patterns:

- a query-side registry keyed by `session_id`, or an equivalent query-owned lookup keyed off the same carried context, is the narrowest live seam for shared `SessionBudget` propagation
- this avoids putting concrete `SessionBudget` or `HealthCache` types into `ToolContext` or `AgentRunParams`

Less suitable seam:

- adding only a crate-neutral `CancellationToken` to `AgentRunParams` would not solve shared spend accounting, because the child loop also needs to see the shared `SessionBudget` in `QueryConfig` so that child spend records into the same budget

### 3. TeamCreate outer cancellation seam status

- `TeamCreateTool` already has its own per-agent outer cancellation mechanism for `TeamDelete`:
  - token creation at `src-rust/crates/tools/src/team_tool.rs:367-374`
  - outer `tokio::select!` at `src-rust/crates/tools/src/team_tool.rs:412-423`
- That mechanism is separate from shared session-budget propagation.
- Current repo reality does not prove that `TASK-M11-08B` must redesign or widen that outer cancellation mechanism in order to solve shared session-budget propagation.

## crate-boundary constraints

- `claurst-query` depends on `claurst-tools`:
  - `src-rust/crates/query/Cargo.toml`
- `claurst-tools` does not depend on `claurst-query`:
  - `src-rust/crates/tools/Cargo.toml`
- `ToolContext` is owned by `claurst-tools`: `src-rust/crates/tools/src/lib.rs:214-235`
- `SessionBudget` and `HealthCache` are owned by `claurst-query`:
  - `src-rust/crates/query/src/session_budget.rs:4-37`
  - `src-rust/crates/query/src/health_cache.rs:11-68`

Therefore:

- adding concrete `Option<Arc<SessionBudget>>` or `Option<Arc<HealthCache>>` to `ToolContext` is still structurally invalid
- adding those same concrete query types directly to `AgentRunParams` would create the same invalid reverse dependency problem
- the follow-up ticket must stay inside a query-owned seam or use only crate-neutral carriers

## recommended inserted-ticket scope

Recommended scope for `TASK-M11-08B`:

- propagate the already-existing root `SessionBudget` into child/team query loops only
- cover exactly these live paths:
  - foreground AgentTool child runs
  - background AgentTool child runs
  - query-backed team-runner child loops invoked through `register_agent_runner()`
- use a query-owned handoff that does not add a concrete query type to `ToolContext` or `AgentRunParams`
- ensure child loops both:
  - receive the shared `SessionBudget` on child `QueryConfig`
  - observe shared-budget cancellation through the cancellation path used by `run_query_loop()`

Narrowest valid file boundary, based on current repo structure:

- primary:
  - `src-rust/crates/query/src/agent_tool.rs`
  - `src-rust/crates/query/src/lib.rs` or `src-rust/crates/query/src/session_budget.rs` if a small query-owned registry/helper is needed
- only if execution proves unavoidable:
  - a new small query-owned helper module under `src-rust/crates/query/src/`
- not required by current evidence:
  - `src-rust/crates/tools/src/lib.rs`
  - `src-rust/crates/tools/src/team_tool.rs`

Recommended ticket objective sentence:

- Establish shared `SessionBudget` propagation from the root session into all existing child/team query-loop entrypoints, without adding concrete query-owned budget/cache fields to `ToolContext`.

## explicitly excluded scope

- no `ToolContext.session_budget`
- no `ToolContext.health_cache`
- no `HealthCache` propagation or runtime plumbing
- no `team_tool.rs` redesign unless execution proves an unavoidable narrow interface adjustment
- no TeamCreate / TeamDelete feature redesign
- no child `max_tokens` override work
- no `allow_fallback` or child `budget_usd` schema work
- no worker/query event expansion except what a later ticket explicitly owns
- no provider-resolution fallback redesign

## whether TeamCreate outer cancellation belongs in this ticket

`no`

Reason:

- The missing capability after corrected `TASK-M11-08R` is shared session-budget propagation into child/team query loops.
- `TeamCreateTool` already has an outer per-agent cancellation path for `TeamDelete` at `src-rust/crates/tools/src/team_tool.rs:367-423`.
- That outer cancellation machinery is a distinct tools-layer concern and is not the same as shared session-budget spend/cancellation propagation.
- Current live repo evidence does not require widening `TASK-M11-08B` to redesign or unify TeamCreate outer cancellation.

Practical authority decision:

- `TASK-M11-08B` should cover team-runner child-loop inheritance of shared session-budget state
- it should not also become a TeamCreate outer-cancellation redesign ticket
- if a later ticket wants explicit unification of TeamDelete per-agent cancellation with inner query-loop tokens, that should be a separate follow-up

## whether "HealthCache" belongs in this ticket

`no`

Reason:

- `HealthCache` exists in `claurst-query` as a provider-health TTL cache: `src-rust/crates/query/src/health_cache.rs:11-68`
- corrected `TASK-M11-08R` did not put it on the active runtime path
- child/team session-budget propagation still does not require it
- `HealthCache` only becomes relevant when provider fallback / health-aware routing work is actually introduced on the runtime path

Decision:

- `HealthCache` remains deferred
- `TASK-M11-08B` should not mention `ToolContext.health_cache`
- downstream work must not assume `TASK-M11-08B` solves any `HealthCache` plumbing

## sequencing impact on "TASK-M11-09" and later

Sequencing consequence:

- insert `TASK-M11-08B` immediately after corrected `TASK-M11-08R`
- treat `TASK-M11-08B` as the accepted baseline for child/team shared-budget inheritance before reopening the same spawn seams in later tickets

Impact by downstream ticket:

- `TASK-M11-09`:
  - should wait for `TASK-M11-08B`
  - rationale: it also edits the same child/team query-loop construction sites, and the child inheritance baseline should be corrected first rather than mixed with later `max_tokens` work
- `TASK-M11-10`:
  - should wait for `TASK-M11-08B`
  - rationale: child `budget_usd` schema work should not proceed while shared session-budget propagation remains undefined
- `TASK-M11-11` and any worker/session-budget event work:
  - blocked on `TASK-M11-08B`
  - rationale: event semantics that depend on child propagation are not valid until child propagation exists
- later tickets generally:
  - must not assume child/team loops inherit root session-budget cancellation until `TASK-M11-08B` is complete and accepted

## exact proposed wording snippet for GPT/WebUI to adopt as temporary authority for "TASK-M11-08B"

`TEMPORARY AUTHORITY FOR TASK-M11-08B`

`TASK-M11-08B is limited to propagating the already-existing root SessionBudget into spawned child query loops only. Implement only the narrow seam needed so foreground AgentTool runs, background AgentTool runs, and the cc-query-backed team runner inherit the parent SessionBudget and observe its cancellation. Use a query-owned handoff that does not introduce a claurst-tools -> claurst-query dependency.`

`Do NOT add SessionBudget or HealthCache as concrete fields on ToolContext. Do NOT add ToolContext.health_cache. Do NOT broaden this ticket into HealthCache plumbing, TeamCreate/TeamDelete redesign, child max_tokens override wiring, allow_fallback work, or child budget_usd schema work.`

`TeamCreate outer per-agent cancellation is not part of TASK-M11-08B unless a minimal interface adjustment is proven unavoidable during implementation. HealthCache remains deferred.`

## whether "TASK-M11-08B" is sufficient as a single ticket

`yes`

Condition:

- yes, if and only if the ticket is kept narrow to query-owned shared `SessionBudget` propagation into existing child/team query loops and does not absorb TeamCreate outer-cancellation redesign or `HealthCache`

## notes / risks

- Risk: if `TASK-M11-08B` is written too broadly, it will recreate the already-rejected `ToolContext -> concrete query type` problem.
- Risk: if execution chooses a session-id keyed query registry, lifecycle cleanup for stale entries and background-agent completion must be handled explicitly, but that remains a local query-owned concern.
- Risk: `TASK-M11-09` and `TASK-M11-10` also touch child `QueryConfig` builders; patch isolation will matter after `TASK-M11-08B` closes.
- Risk: later event tickets must not describe child/team budget behavior as if it were already present before `TASK-M11-08B` lands.
