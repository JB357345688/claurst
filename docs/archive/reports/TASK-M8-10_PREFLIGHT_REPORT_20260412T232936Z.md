# TASK-M8-10 Preflight Report

## Ticket
`TASK-M8-10`

## Timestamp UTC
`20260412T232936Z`

## Branch
`feature/provider-resolution-seam`

## Verdict
`READY-WITH-NOTES`

## Repo State Summary
- `git branch --show-current`: `feature/provider-resolution-seam`
- `git diff --name-only`: empty
- `git diff --cached --name-only`: empty
- `git status --short`: no tracked unstaged or staged drift; untracked workspace noise is present under `.codex/`, `docs/Current/`, `docs/Orchestrator_planning/`, `docs/archive/`, and `src-rust/target/`
- Required history confirmed on current branch:
  - `ea9da37` `TASK-M8-06 replace AgentRunFn positional seam with AgentRunParams`
  - `1d531da` `TASK-M8-08 wire team runner producer through shared provider seam`
  - `5d472cf` `TASK-M8-07 add provider/model fields to team spec schema`
  - `2fd7732` `TASK-M8-09 wire team spec provider/model into override fields`
- Additional M8 preconditions visible in history:
  - `b5249a3` `TASK-M8-04 wire foreground provider resolution through shared seam`
  - `5d246b2` `TASK-M8-05 wire background provider resolution through shared seam`
- Drift found: none in tracked state
- Blockers: none from repo cleanliness

## Authority Reviewed
- `AGENTS.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md` (controlling authority)
- `docs/Orchestrator_planning/IMPLEMENTATION_PLAN_MPWO.md` (secondary only)
- `docs/archive/reports/TASK-M8-08_POST_M8-07_NONREGRESSION_REPORT_20260412T230523Z.md`
- `docs/archive/reports/TASK-M8-09_COMMIT_VERIFICATION_REPORT_20260412T232152Z.md`

## Verified Files / Symbols / Commands
- Files:
  - `src-rust/crates/query/src/agent_tool.rs`
  - `src-rust/crates/tools/src/team_tool.rs`
  - `src-rust/crates/tools/src/lib.rs`
  - `src-rust/crates/query/src/lib.rs`
  - `src-rust/crates/query/src/provider_resolution.rs`
- Symbols / behaviors:
  - `AgentTool::execute()`
  - `init_team_swarm_runner()`
  - `AgentRunParams`
  - `TeamCreateTool::execute()`
  - `ToolContext`
  - `run_query_loop()`
- Commands:
  - `git branch --show-current`
  - `git status --short`
  - `git diff --name-only`
  - `git diff --cached --name-only`
  - `git log --oneline --decorate -n 20`

## Dependency Baseline Confirmed
- MPWO preconditions for `TASK-M8-10` require `TASK-M8-04`, `TASK-M8-05`, and `TASK-M8-08` complete; all are present in current branch history.
- MPWO dependency graph also includes `M8-09 -> M8-10`; current `HEAD` is `2fd7732`, so the override transport dependency is present.
- Prior report baseline confirms:
  - accepted `M8-08` provider-seam behavior remains preserved on current `HEAD`
  - accepted `M8-09` commit scope is limited to `team_tool.rs` plus its report
- Hosted Ollama non-regression invariant remains the controlling baseline for any later implementation work touching provider resolution/materialization behavior.

## Exact M8-10 Scope Confirmation
- Owns:
  - new tests only
  - preferred current-reality location: `src-rust/crates/query/src/agent_tool.rs` in a new `#[cfg(test)]` module
- Does not own:
  - `src-rust/crates/tools/src/team_tool.rs`
  - `src-rust/crates/query/src/provider_resolution.rs`
  - `run_query_loop()` production behavior
  - TeamCreate execution coverage
  - fallback/D2 behavior
  - new mock frameworks or broad test utilities
- MPWO-owned behaviors for this ticket:
  - verify missing `provider_registry` hard error
  - verify explicit provider behavior where feasible
  - verify parent inheritance where feasible
  - mark only network-dependent tests as `#[ignore]`
- Validation command required later:
  - `cd src-rust && cargo test -p claurst-query -- agent_tool`
- MPWO stop / escalate condition:
  - if constructing a `ToolContext` in tests requires too many dependencies, focus on the missing-registry test and document the limitation

## Current Code Reality
- `src-rust/crates/query/src/agent_tool.rs` has no existing `#[cfg(test)]` module.
- `src-rust/crates/query/tests/` does not exist.
- `AgentTool::execute()` already exposes a narrow, testable seam:
  - parent-provider hint derivation at `src-rust/crates/query/src/agent_tool.rs:256-268`
  - missing-registry hard error at `src-rust/crates/query/src/agent_tool.rs:270-277`
  - provider resolution/materialization at `src-rust/crates/query/src/agent_tool.rs:280-287`
- Non-network success-path testing is feasible because `run_query_loop()` returns before any provider/API call when `max_turns` is `0` at `src-rust/crates/query/src/lib.rs:702-726`.
- Post-M8-09 override transport is present on current `HEAD`:
  - `TeamCreateTool::execute()` passes `provider_override` / `model_override` into `AgentRunParams` at `src-rust/crates/tools/src/team_tool.rs:415-424`
  - `init_team_swarm_runner()` consumes those fields at `src-rust/crates/query/src/agent_tool.rs:575-617`

## Latent Interface Mismatch At Current HEAD
- `AgentTool::execute()` inherits the parent provider when no explicit provider is supplied and the model has no provider prefix:
  - `src-rust/crates/query/src/agent_tool.rs:256-268`
- `init_team_swarm_runner()` does not derive any parent-provider hint from `ctx.config.provider`; it resolves only from `provider_override`, model prefix, and `model_registry`:
  - `src-rust/crates/query/src/agent_tool.rs:609-617`
- Result:
  - explicit provider transport from `M8-09` is present
  - parent inheritance for the runner path is not presently implemented on current `HEAD`
- Additional testability constraint:
  - direct runner invocation is not exposed publicly from the query crate
  - `AGENT_RUNNER` and `run_agent(...)` are private in `src-rust/crates/tools/src/team_tool.rs:55-75`
  - MPWO marks TeamCreate execution out of scope for this ticket

## Likely Smallest Edit Surface
- Single-file test-only delta in `src-rust/crates/query/src/agent_tool.rs`
- Likely additions:
  - one local test helper to construct `ToolContext`
  - one local helper to build valid `AgentTool` input JSON
  - ticket-local `#[tokio::test]` coverage for:
    - missing registry -> error
    - explicit provider success path using in-memory registry plus `max_turns: 0`
    - parent inheritance success path on `AgentTool` using parent `config.provider` plus `max_turns: 0`
- Risk of widening:
  - low if the ticket is kept to `AgentTool` test coverage
  - real if the ticket is interpreted as requiring direct runner-path parent-inheritance verification, because current `HEAD` would expose an out-of-scope behavior gap

## Validation Commands Required Later
```bash
cd src-rust && cargo test -p claurst-query -- agent_tool
```

## Blockers Or Notes
- No tracked-state blocker.
- Note: current `HEAD` is compatible with a narrow, ticket-local `AgentTool` test pass.
- Note: current `HEAD` is not aligned with the broader “both worker paths inherit parent provider” objective because the runner path does not currently use `ctx.config.provider` as a fallback hint.
- Note: hosted Ollama baseline must remain untouched; no preflight evidence suggests regression in that accepted behavior.
