# TASK-M8-11 Preflight Report

## Ticket
`TASK-M8-11`

## Timestamp UTC
`20260413T000622Z`

## Branch
`feature/provider-resolution-seam`

## Verdict
`READY-WITH-NOTES`

## Repo State Summary
- Current branch verified: `feature/provider-resolution-seam`
- `git diff --name-only`: empty
- `git diff --cached --name-only`: empty
- `git status --short`: no tracked unstaged or staged drift
- Untracked workspace noise is present under `.codex/`, `docs/Current/`, `docs/Orchestrator_planning/`, `docs/archive/provider_orchestrator/`, `docs/archive/reports/`, and `src-rust/target/`
- Tracked drift found: none

## Verified Files, Symbols, And Commands
- Authority files reviewed:
  - `AGENTS.md`
  - `docs/Current/MPWO_WORK_ORDER_PACK.md`
  - `docs/Orchestrator_planning/IMPLEMENTATION_PLAN_MPWO.md`
  - `docs/archive/reports/TASK-M8-09_COMMIT_VERIFICATION_REPORT_20260412T232152Z.md`
  - `docs/archive/reports/TASK-M8-10_COMMIT_VERIFICATION_REPORT_20260412T235810Z.md`
- Repo-state commands verified:
  - `git branch --show-current`
  - `git status --short`
  - `git diff --name-only`
  - `git diff --cached --name-only`
  - `git log --oneline --decorate -n 20`
- Current source reality inspected:
  - `src-rust/crates/tools/src/lib.rs`
  - `src-rust/crates/tools/src/team_tool.rs`
  - `src-rust/crates/query/src/agent_tool.rs`
  - `src-rust/crates/query/src/lib.rs`
  - `src-rust/crates/cli/src/main.rs`

## Authority Reviewed
- `AGENTS.md` confirmed repo-local authority over `~/.codex/AGENTS.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md` confirmed as controlling ticket authority
- `docs/Orchestrator_planning/IMPLEMENTATION_PLAN_MPWO.md` reviewed as secondary only; no M8-11 instruction found that overrides MPWO

## Dependency Baseline Confirmed
- Required closed M8 baseline commits are present on current history:
  - `ea9da37` `TASK-M8-06 replace AgentRunFn positional seam with AgentRunParams`
  - `1d531da` `TASK-M8-08 wire team runner producer through shared provider seam`
  - `5d472cf` `TASK-M8-07 add provider/model fields to team spec schema`
  - `2fd7732` `TASK-M8-09 wire team spec provider/model into override fields`
  - `1056eb3` `TASK-M8-10 add agent tool provider seam coverage tests`
- M8-09 verification report confirms `TeamCreateTool::execute()` now passes `spec.provider` and `spec.model` into `AgentRunParams`
- M8-10 verification report confirms `agent_tool.rs` carries test-only additions for:
  - missing `provider_registry` hard error
  - explicit provider resolution
  - parent-provider inheritance in `AgentTool`
- Hosted Ollama compatibility baseline preserved

## Exact MPWO Contract For TASK-M8-11
- Objective: verify the entire workspace compiles, tests pass, and clippy is clean after M8
- Exact code targets: none; validation-only ticket
- Explicit dependencies:
  - all M8 tickets complete
  - dependency graph shows direct edge `M8-10 -> M8-11`
- Owned behaviors:
  - run the four workspace validation commands
  - fix M8-related failures only if validation later reveals them
- Explicit out-of-scope items:
  - no unrelated clippy cleanup
  - no new features
  - no surrounding-code cleanup
  - no file changes outside the already-M8-touched surface if fixups later become necessary
- Validation commands required later:
  - `cd src-rust && cargo fmt --all -- --check`
  - `cd src-rust && cargo build --workspace`
  - `cd src-rust && cargo test --workspace`
  - `cd src-rust && cargo clippy --workspace --all-targets`
- Stop / escalate conditions inherited from M7-07:
  - if workspace test failure is outside M8-related scope, stop and investigate/report rather than fix unrelated code
  - if more than 3 files need fixes, escalate

## M8-11 Scope Confirmation
- M8-11 owns:
  - workspace validation execution and M8-local failure triage
- M8-11 does not own:
  - new production behavior
  - schema changes
  - D2/fallback/budget/trust-domain work
  - unrelated test cleanup
  - baseline refactors outside the M8 propagation seam
- Smallest edit surface required by contract:
  - none in preflight
  - if later validation fails for an M8-local reason, the smallest plausible fix surface stays within already-M8-touched seam files such as:
    - `src-rust/crates/query/src/agent_tool.rs`
    - `src-rust/crates/tools/src/team_tool.rs`
    - `src-rust/crates/tools/src/lib.rs`
    - `src-rust/crates/cli/src/main.rs`
    - `src-rust/crates/query/src/lib.rs`

## Current Code Reality
- `ToolContext` currently carries `provider_registry` and `model_registry` in `src-rust/crates/tools/src/lib.rs`
- CLI startup currently populates both registries into both `QueryConfig` and `ToolContext` in `src-rust/crates/cli/src/main.rs`
- `AgentTool` currently:
  - accepts optional `provider`
  - resolves provider/model via `resolve_provider_identity()` and `materialize_provider()`
  - fails loudly when `provider_registry` is absent
  - propagates registries into child `QueryConfig`
- `init_team_swarm_runner()` currently:
  - accepts `AgentRunParams`
  - consumes `provider_override` and `model_override`
  - resolves/materializes via the shared seam
  - propagates registries into child `QueryConfig`
- `TeamCreateTool::execute()` currently:
  - captures `spec.provider.clone()` and `spec.model.clone()`
  - forwards them through `AgentRunParams`

## Dependency And Interface Assessment
- Does M8-11 depend on M8-08/M8-09/M8-10 behavior being present on current `HEAD`:
  - yes
  - current `HEAD` includes the required commits and the inspected source reality matches those seam changes
- Latent interface mismatch at current `HEAD`:
  - no compile-shape mismatch was found across `ToolContext`, `QueryConfig`, `AgentRunParams`, `TeamCreateTool::execute()`, and `init_team_swarm_runner()`
  - all inspected constructors/call sites currently carry the new registry and override fields
- Semantic note:
  - in `init_team_swarm_runner()`, the shared-seam call visibly uses `provider_override` plus model-derived resolution and does not itself consult `ctx.config.provider` when the override is absent
  - this is not a type/interface mismatch and the current MPWO M8-08 snippet uses the same shape, so preflight does not treat it as structural drift
  - however, it is worth keeping in mind during later M8-11 validation because the broader Milestone 8 narrative speaks about worker inheritance from the parent session

## Likely Edit Surface
- Preflight pass: no source edit surface required
- Validation pass later:
  - can remain ticket-local if all four workspace commands pass
  - if a failure occurs inside the inspected M8 seam, likely fix surface stays limited to the existing M8 propagation files listed above
  - widening beyond that would require explicit evidence from validation output

## Blockers Or Notes
- Blockers: none for preflight
- Notes:
  - untracked workspace noise exists but tracked review basis is clean
  - no cargo commands were run in this preflight pass, per instruction
  - hosted-Ollama non-regression invariant remains the baseline and must stay explicit during later M8-11 execution/review
