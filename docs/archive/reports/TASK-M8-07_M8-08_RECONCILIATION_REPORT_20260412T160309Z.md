# TASK-M8-07 / M8-08 Reconciliation Report

## Repo state
- Branch: `feature/provider-resolution-seam`; `HEAD` is `1d531da TASK-M8-08 wire team runner producer through shared provider seam`.
- No tracked unstaged or staged drift: `git diff --name-only` and `git diff --cached --name-only` were empty.
- No `TASK-M8-07` commit was found in `git log --grep`, and no `TASK-M8-07*` report exists under `docs/archive/reports`.
- Ambiguity: `docs/Current/` and most supplemental M8 reports are untracked working-tree artifacts. They were treated as current authority because the prompt required that, but they are not commit-anchored. The tracked M8 report artifacts at `HEAD` are the closeout reports only.

## Authority used
- Primary authority: `AGENTS.md` and `docs/Current/MPWO_WORK_ORDER_PACK.md`.
- `docs/Current/IMPLEMENTATION_PLAN_MPWO.md` is absent. The implementation-plan equivalent used was `docs/Orchestrator_planning/IMPLEMENTATION_PLAN_MPWO.md`.
- Supplemental evidence read:
  - tracked closeout reports:
    - `docs/archive/reports/TASK-M8-06_CLOSEOUT_REPORT_20260412T145114Z.md`
    - `docs/archive/reports/TASK-M8-08_CLOSEOUT_REPORT_20260412T152403Z.md`
  - untracked corroborating reports:
    - `docs/archive/reports/TASK-M8-06_EXECUTION_REPORT_20260412T141929Z.md`
    - `docs/archive/reports/TASK-M8-06_REVIEW_REPORT_20260412T144536Z.md`
    - `docs/archive/reports/TASK-M8-08_EXECUTION_REPORT_20260412T151649Z.md`
    - `docs/archive/reports/TASK-M8-08_REVIEW_REPORT_20260412T152001Z.md`
    - `docs/archive/reports/TASK-M8-08_COMMIT_VERIFICATION_REPORT_20260412T152849Z.md`
- MPWO vs implementation plan:
  - the implementation plan is milestone-level and too coarse to police ticket boundaries
  - MPWO is the controlling ticket contract
- Material contradiction called out:
  - the implementation plan says `init_team_swarm_runner()` should no longer construct `AnthropicClient` directly
  - the same plan's risk section and MPWO step text preserve the `run_query_loop()` client parameter and require constructing a client for Anthropic targets
  - current M8-08 follows the narrower MPWO step-level contract

## Exact M8-07 scope
- MPWO assigns M8-07 only to `src-rust/crates/tools/src/team_tool.rs`:
  - add `provider: Option<String>` and `model: Option<String>` to `AgentSpec`
  - add `agents[].provider` and `agents[].model` to `TeamCreateTool::input_schema()`
- Exact symbols/behaviors owned by M8-07:
  - `AgentSpec.provider`
  - `AgentSpec.model`
  - TeamCreate JSON schema exposure of those fields
- M8-07 explicitly must not touch:
  - `TeamCreateTool::execute()`; MPWO says: `Do NOT modify TeamCreateTool::execute() — that is M8-09.`
  - `allow_fallback`
  - `budget_usd`
  - existing `AgentSpec` fields
- Current repo reality confirms M8-07 has not landed:
  - `AgentSpec` still has only `name`, `role`, `tools`, and `task`
  - the TeamCreate schema still lacks `provider` and `model`
- Separate M8-09 scope, per MPWO:
  - only `TeamCreateTool::execute()` wiring from `spec.provider` / `spec.model` into `AgentRunParams`

## Exact M8-08 scope
- MPWO assigns M8-08 only to `src-rust/crates/query/src/agent_tool.rs`:
  - `init_team_swarm_runner()`
- Exact symbols/behaviors owned by M8-08:
  - consume `AgentRunParams`
  - honor `provider_override` and `model_override`
  - route through `resolve_provider_identity()` and `materialize_provider()`
  - propagate `provider_registry`, `model_registry`, and resolved model into child `QueryConfig`
  - preserve the selected provider into the runner context
- M8-08 explicitly must not:
  - remove the `run_query_loop()` client parameter
  - add fallback behavior
  - modify `register_agent_runner()` beyond type fallout
  - change `run_query_loop()` call shape beyond the new config/client handling
- By file scope, M8-08 has no authority to modify `src-rust/crates/tools/src/team_tool.rs`.

## Dependency graph finding
- The controlling MPWO graph says:
  - `M8-06 -> M8-08`
  - `M8-07 -> M8-09`
  - `M8-06 -> M8-09`
  - `M8-08 -> M8-10`
- There is no `M8-07 -> M8-08` edge and no `M8-08 -> M8-09` edge in MPWO.
- M8-06 explicitly sets:
  - `provider_override: None`
  - `model_override: None`
  - and marks them as placeholders to be wired in M8-09
- Result:
  - M8-07 and M8-08 are sibling tickets
  - M8-09 is the intended connector
  - numeric order does not create a dependency edge that MPWO does not declare

## M8-08 actual touched files and symbols
- The M8-08 closeout commit is present at `HEAD`:
  - `1d531daa6ef46196fecfbde6627edd073ac252f1`
- `git show --name-only 1d531da` shows tracked source change only to:
  - `src-rust/crates/query/src/agent_tool.rs`
  - plus the tracked closeout report
- No `team_tool.rs` source file was touched by M8-08.
- The diff is confined to `init_team_swarm_runner()`:
  - destructures `AgentRunParams`
  - resolves provider with `resolve_provider_identity(...)`
  - materializes provider with `materialize_provider(...)`
  - propagates registries/model into child `QueryConfig`
  - preserves provider into `runner_ctx.config.provider`
- Current `team_tool.rs` still passes:
  - `provider_override: None`
  - `model_override: None`
  - exactly as M8-06 said it should until M8-09
- Current `AgentSpec` and TeamCreate schema remain untouched.
- M8-08 consumed `AgentRunParams` only at the producer seam, which is exactly what MPWO assigns to it.
- M8-08 did not assume M8-07’s `AgentSpec` or schema fields already existed.
- M8-08 did not introduce M8-07 behavior or M8-09 behavior.
- Hidden coupling assessment:
  - low
  - retroactive M8-07 is additive in `team_tool.rs`
  - M8-09 remains the first ticket that will actually source non-`None` override values

## Contamination classification
- File-scope contamination: no
- Interface/sequencing contamination: no
- Benign out-of-order but composable: yes
- Rollback-worthy entanglement: no
- Reason:
  - current `HEAD` is numerically out of order because M8-08 landed without M8-07
  - MPWO does not make M8-07 a prerequisite for M8-08
  - the code stayed inside the owned M8-08 file/symbol boundary
  - this is not a statement that overall M8 is complete; only that M8-08 did not trespass into M8-07 territory

## Safest recovery path
- retroactive M8-07 on top of current HEAD
- Narrow justification:
  - M8-07 is a clean additive change in `src-rust/crates/tools/src/team_tool.rs` only
  - there is no overlap with M8-08’s `src-rust/crates/query/src/agent_tool.rs` producer seam
  - rolling back M8-08 buys nothing because the producer is already correctly prepared to consume override fields while current callers still send `None`

## Recommended next Codex action
- Implement `TASK-M8-07` only in `src-rust/crates/tools/src/team_tool.rs`:
  - add `provider` / `model` to `AgentSpec`
  - add `provider` / `model` to the TeamCreate schema
  - do not touch `TeamCreateTool::execute()`
