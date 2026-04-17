# TASK-M11-08 Preflight Report

## ticket id

`TASK-M11-08`

## verdict

`PASS-WITH-NOTES`

## timestamp UTC

`2026-04-14T15:33:27Z`

## branch / HEAD observed

- Branch: `feature/provider-resolution-seam`
- HEAD: `0942e4aefe99184b8caf4259d9cf9006616d6c6c`

## authority reviewed

- `AGENTS.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md`
- `docs/archive/reports/M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md`
- Accepted baseline supplied in prompt for `TASK-M11-01` through `TASK-M11-07`

## accepted-baseline comparison

- Branch matches expected accepted branch: `feature/provider-resolution-seam`
- HEAD matches the accepted latest baseline exactly: `0942e4aefe99184b8caf4259d9cf9006616d6c6c`
- Target implementation files and prerequisite read-only files are clean against `HEAD`:
  - `src-rust/crates/cli/src/main.rs`
  - `src-rust/crates/query/src/agent_tool.rs`
  - `src-rust/crates/query/src/lib.rs`
  - `src-rust/crates/tools/src/lib.rs`
  - `src-rust/crates/tools/src/team_tool.rs`
  - `src-rust/crates/query/src/session_budget.rs`
  - `src-rust/crates/query/src/health_cache.rs`
- Worktree is not clean. Observed unrelated drift/noise:
  - modified `.gitignore`
  - many untracked `docs/` report files
  - untracked `.codex`
  - untracked `src-rust/target/`
- This noise affects review-basis hygiene, but not the M11-08 target-file baseline.

## verified target files / symbols / commands

- Verified `SessionBudget` landed and is exported:
  - `src-rust/crates/query/src/session_budget.rs:5-36`
  - `src-rust/crates/query/src/lib.rs:19-21`
  - `src-rust/crates/query/src/lib.rs:34-36`
- Verified `HealthCache` landed and is exported:
  - `src-rust/crates/query/src/health_cache.rs:12-68`
  - `src-rust/crates/query/src/lib.rs:19`
  - `src-rust/crates/query/src/lib.rs:34`
- Verified CLI budget handling live reality:
  - `src-rust/crates/cli/src/main.rs:243-245` defines `--max-budget-usd`
  - `src-rust/crates/cli/src/main.rs:719-720` copies `cli.max_budget_usd` into `query_config.max_budget_usd`
  - No current `--budget-usd` flag exists
- Verified root `QueryConfig` construction:
  - `src-rust/crates/cli/src/main.rs:698-727`
  - `QueryConfig::from_config_with_registry()` at `src-rust/crates/query/src/lib.rs:178-188`
- Verified root `ToolContext` construction:
  - `src-rust/crates/cli/src/main.rs:729-742`
- Verified current root run cancel-token creation:
  - headless path: `src-rust/crates/cli/src/main.rs:1115-1135`
  - interactive submit path: `src-rust/crates/cli/src/main.rs:2055-2095`
  - auto-compact dispatch path: `src-rust/crates/cli/src/main.rs:2275-2300`
  - bridge inbound prompt path: `src-rust/crates/cli/src/main.rs:2428-2453`
  - polled remote prompt path: `src-rust/crates/cli/src/main.rs:2542-2565`
- Verified `QueryConfig` struct/default/helper reality:
  - struct: `src-rust/crates/query/src/lib.rs:82-133`
  - default: `src-rust/crates/query/src/lib.rs:135-159`
  - helper constructors: `src-rust/crates/query/src/lib.rs:163-188`
  - test helper literal: `src-rust/crates/query/src/lib.rs:2130-2152`
- Verified `ToolContext` struct/test/helper reality:
  - struct: `src-rust/crates/tools/src/lib.rs:216-235`
  - tests: `src-rust/crates/tools/src/lib.rs:544-559`, `:574-589`
  - query test helper: `src-rust/crates/query/src/lib.rs:2198-2217`
  - agent-tool test helper: `src-rust/crates/query/src/agent_tool.rs:679-701`
- Verified `run_query_loop()` budget/accounting locations:
  - signature: `src-rust/crates/query/src/lib.rs:679-688`
  - registry-backed provider path adds usage at `src-rust/crates/query/src/lib.rs:1129-1134`
  - legacy Anthropic path adds usage at `src-rust/crates/query/src/lib.rs:1393-1398`
  - existing per-loop budget guard exists only at `src-rust/crates/query/src/lib.rs:1400-1415`
- Verified spawn-path locations:
  - foreground child path: `src-rust/crates/query/src/agent_tool.rs:362-466`
  - background child path: `src-rust/crates/query/src/agent_tool.rs:390-435`
  - team runner child path: `src-rust/crates/query/src/agent_tool.rs:543-649`
  - TeamCreate per-agent token path: `src-rust/crates/tools/src/team_tool.rs:367-424`
- Verified accepted invariants still present:
  - same-domain fallback: `src-rust/crates/query/src/provider_resolution.rs:302-345`, test at `:1141-1154`
  - CostTracker attribution fields: `src-rust/crates/core/src/lib.rs:2850-2889`
  - Hosted Ollama special casing: `src-rust/crates/query/src/provider_resolution.rs:197-208`, `:390-422`
- Validation command run:
  - `cd src-rust && cargo check --workspace`

## exact M11-08 scope confirmation in current repo reality

- The five intended implementation files exist and match the ticket’s ownership boundaries.
- The two prerequisite/read-only files exist and are exported exactly as expected for M11-07 / M11-04:
  - `SessionBudget` is available via `claurst_query::*`
  - `HealthCache` is available via `claurst_query::*`
- M11-08 can remain M11-08-only. No live repo evidence requires pulling in M11-09 fields or schema work:
  - `AgentInput` currently has no `max_tokens`, `allow_fallback`, or `budget_usd` field
  - `AgentSpec` currently has no `max_tokens`, `allow_fallback`, or `budget_usd` field
  - `AgentRunParams` currently has no `max_tokens_override`, `allow_fallback`, or `budget_usd` field
  - `CHILD_AGENT_FALLBACK_MAX_TOKENS` remains the active D1 fallback at `src-rust/crates/query/src/agent_tool.rs:130-132`
- The one ticket-local interface note is that current CLI reality has only `--max-budget-usd`; M11-08 would need to add a separate root-session budget input if the contract remains `--budget-usd`.
- The one ticket-local query-loop note is that current provider-registry dispatch does not share the legacy branch’s budget-guard location. That is a `lib.rs` integration note, not a reason to widen into later tickets.

## spawn-path findings

- foreground path
  - `src-rust/crates/query/src/agent_tool.rs:362-383` builds child `QueryConfig`
  - `src-rust/crates/query/src/agent_tool.rs:364` hardcodes `max_tokens: CHILD_AGENT_FALLBACK_MAX_TOKENS`
  - `src-rust/crates/query/src/agent_tool.rs:377` hardcodes `max_budget_usd: None`
  - `src-rust/crates/query/src/agent_tool.rs:379-382` carries provider/model registries only
  - `src-rust/crates/query/src/agent_tool.rs:454` creates a fresh `CancellationToken::new()`
  - `src-rust/crates/query/src/agent_tool.rs:456-466` passes that fresh token into `run_query_loop()`
- background path
  - Reuses the same `query_config` built at `src-rust/crates/query/src/agent_tool.rs:362-383`
  - `src-rust/crates/query/src/agent_tool.rs:408` creates a fresh `CancellationToken::new()`
  - `src-rust/crates/query/src/agent_tool.rs:410-419` passes that fresh token into `run_query_loop()`
- team runner path
  - `src-rust/crates/query/src/agent_tool.rs:620-631` builds a separate child `QueryConfig`
  - `src-rust/crates/query/src/agent_tool.rs:622` hardcodes `max_tokens: CHILD_AGENT_FALLBACK_MAX_TOKENS`
  - `src-rust/crates/query/src/agent_tool.rs:628-629` carries provider/model registries
  - `src-rust/crates/query/src/agent_tool.rs:636` creates a fresh `CancellationToken::new()`
  - `src-rust/crates/query/src/agent_tool.rs:638-647` passes that fresh token into `run_query_loop()`
- TeamCreate per-agent token path
  - `src-rust/crates/tools/src/team_tool.rs:367-372` creates one fresh `CancellationToken::new()` per team agent
  - `src-rust/crates/tools/src/team_tool.rs:374` stores them in `ACTIVE_TEAMS`
  - `src-rust/crates/tools/src/team_tool.rs:405-424` uses `tokio::select!` between `run_agent(...)` and `cancel.cancelled()`
  - This is a distinct cancellation layer from the query-loop token inside the team runner. Both layers need M11-08 wiring:
    - query-loop child cancellation tokens in `agent_tool.rs`
    - TeamCreate per-agent cancellation tokens in `team_tool.rs`

## constructor / fan-out findings

- `QueryConfig`
  - Struct fan-out owner: `src-rust/crates/query/src/lib.rs:82-133`
  - Default fan-out owner: `src-rust/crates/query/src/lib.rs:135-159`
  - Helper constructors that inherit new fields through `Default`: `src-rust/crates/query/src/lib.rs:163-188`
  - Explicit helper literal needing update: `src-rust/crates/query/src/lib.rs:2130-2152`
  - Explicit production literals needing update: `src-rust/crates/query/src/agent_tool.rs:362-383`, `:620-631`
  - No other `QueryConfig { ... }` literals exist in `src-rust`
- `ToolContext`
  - Struct fan-out owner: `src-rust/crates/tools/src/lib.rs:216-235`
  - Explicit production literal needing update: `src-rust/crates/cli/src/main.rs:729-742`
  - Explicit test/helper literals needing update:
    - `src-rust/crates/tools/src/lib.rs:544-559`
    - `src-rust/crates/tools/src/lib.rs:574-589`
    - `src-rust/crates/query/src/lib.rs:2202-2217`
    - `src-rust/crates/query/src/agent_tool.rs:686-701`
  - No other `ToolContext { ... }` literals exist in `src-rust`
- test/helper literals that will need later compile fallout updates
  - `QueryConfig` helper literal in `src-rust/crates/query/src/lib.rs:2130-2152`
  - `ToolContext` test literals in `src-rust/crates/tools/src/lib.rs:544-559` and `:574-589`
  - `ToolContext` helper literals in `src-rust/crates/query/src/lib.rs:2202-2217` and `src-rust/crates/query/src/agent_tool.rs:686-701`
  - This is narrow constructor/test fallout, not structural drift
- additional non-constructor touchpoint check
  - `src-rust/crates/query/src/cron_scheduler.rs:74-95` is an extra `run_query_loop()` caller, but it clones already-built `QueryConfig` / `ToolContext`
  - No extra constructor edits are implied there; it is not a structural blocker for M11-08-only execution

## validation command run and result

- Command: `cd src-rust && cargo check --workspace`
- Result: `PASS`
- Output summary:
  - `Checking claurst-query`
  - `Compiling claurst`
  - `Checking claurst-bridge`
  - `Checking claurst-tui`
  - `Checking claurst-commands`
  - `Finished dev profile [unoptimized + debuginfo] target(s) in 2.08s`

## drift found

- Worktree drift/noise exists outside the target files:
  - modified `.gitignore`
  - many untracked report/doc files
  - untracked `.codex`
  - untracked `src-rust/target/`
- Ticket-contract drift:
  - current CLI has `--max-budget-usd` only at `src-rust/crates/cli/src/main.rs:243-245`
  - no live `--budget-usd` flag currently exists
- Query-loop integration drift relative to the plan wording:
  - existing per-loop budget guard is only on the legacy Anthropic branch at `src-rust/crates/query/src/lib.rs:1400-1415`
  - registry-backed provider flow adds usage at `src-rust/crates/query/src/lib.rs:1129-1134` and returns via `:1287-1290` without passing that guard
- Root-cancellation wiring note:
  - all root run paths in `main.rs` currently create fresh cancel tokens
  - `SessionBudget::check_and_cancel()` only cancels its own root token at `src-rust/crates/query/src/session_budget.rs:24-31`
  - live repo reality therefore requires an explicit M11-08 decision for how root query-loop cancellation observes `SessionBudget`
- No structural drift was found in the five owned files that would force entry into M11-09

## blockers, if any

- No hard blocker for a later M11-08 implementation pass
- No compile blocker
- No structural blocker requiring ticket halt
- Notes to resolve during implementation:
  - add root-session budget input without conflating it with existing `max_budget_usd`
  - bind root query-loop cancellation to `SessionBudget`
  - wire session-budget accounting at the live provider-execution site(s) in `run_query_loop()`
  - wire both team cancellation layers

## hosted Ollama invariant assessment

- Hosted Ollama compatibility surfaces remain intact and outside the M11-08 target logic:
  - `materialize_provider()` special-cases Ollama at `src-rust/crates/query/src/provider_resolution.rs:197-208`
  - `build_ollama_provider()` preserves provider-config API-base override and auth-store key lookup at `src-rust/crates/query/src/provider_resolution.rs:390-404`
  - `normalize_ollama_api_base()` still rewrites hosted `/api` and `/api/v1` roots to `/v1` at `src-rust/crates/query/src/provider_resolution.rs:409-422`
- M11-08 does not need to touch provider selection, provider materialization, or provider request shaping.
- Risk surface is indirect only:
  - root and child sessions already use the registry-backed provider path
  - any new session-budget logic added in `run_query_loop()` must remain bookkeeping/cancellation-only and must not alter provider resolution/materialization behavior
- Assessment: hosted Ollama non-regression is maintainable within M11-08-only scope

## exact recommendation for next step

- Proceed to the implementation pass for `TASK-M11-08` only.
- Keep the patch confined to:
  - `src-rust/crates/cli/src/main.rs`
  - `src-rust/crates/query/src/agent_tool.rs`
  - `src-rust/crates/query/src/lib.rs`
  - `src-rust/crates/tools/src/lib.rs`
  - `src-rust/crates/tools/src/team_tool.rs`
- During implementation, explicitly cover these live-reality points:
  - root CLI budget input is currently absent as `--budget-usd`
  - `QueryConfig` needs `session_budget: Option<Arc<SessionBudget>>`
  - `ToolContext` needs `session_budget: Option<Arc<SessionBudget>>` and `health_cache: Option<Arc<HealthCache>>`
  - root query-loop cancellation must actually observe `SessionBudget`
  - child query-loop tokens must use session-budget children when available
  - TeamCreate per-agent tokens must also use session-budget children
  - do not touch `AgentInput.max_tokens`, `AgentSpec.max_tokens`, `AgentRunParams.max_tokens_override`, `allow_fallback`, `budget_usd` child schema, or `CHILD_AGENT_FALLBACK_MAX_TOKENS`
