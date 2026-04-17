# TASK-M8-08 Post-M8-07 Non-Regression Report

## Ticket
`TASK-M8-08`

## Timestamp UTC
`20260412T230523Z`

## Branch
`feature/provider-resolution-seam`

## Repo State Observed
- Current branch: `feature/provider-resolution-seam`
- `git diff --name-only`: empty
- `git diff --cached --name-only`: empty
- `git status --short` showed no tracked unstaged or staged drift
- Untracked workspace noise remains present under:
  - `.codex/`
  - `docs/Current/`
  - `docs/Orchestrator_planning/`
  - `docs/archive/provider_orchestrator/`
  - `docs/archive/reports/`
  - `src-rust/target/`
- Required commits present in current history:
  - `5d472cf` `TASK-M8-07 add provider/model fields to team spec schema`
  - `1d531da` `TASK-M8-08 wire team runner producer through shared provider seam`

## Authority Reviewed
- `AGENTS.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md`
- `docs/Orchestrator_planning/IMPLEMENTATION_PLAN_MPWO.md`

## Prior Reports Reviewed
- `docs/archive/reports/TASK-M8-08_CLOSEOUT_REPORT_20260412T152403Z.md`
- `docs/archive/reports/TASK-M8-08_COMMIT_VERIFICATION_REPORT_20260412T152849Z.md`
- `docs/archive/reports/TASK-M8-07_COMMIT_VERIFICATION_REPORT_20260412T225608Z.md`

## Reconfirmed Accepted M8-08 Contract
- Owned file/symbol scope:
  - `src-rust/crates/query/src/agent_tool.rs`
  - `init_team_swarm_runner()`
- Accepted behavior contract from MPWO and closeout basis:
  - consume `AgentRunParams`
  - consume `provider_override` and `model_override`
  - remove hardcoded producer-path `ANTHROPIC_API_KEY` lookup
  - route provider/model resolution through `resolve_provider_identity(...)` and `materialize_provider(...)`
  - propagate `provider_registry`, `model_registry`, and resolved target model into child `QueryConfig`
  - preserve selected provider into `run_query_loop()` through cloned `ToolContext.config.provider`
  - preserve `run_query_loop()` call shape, including the client parameter
- Out of scope for M8-08:
  - `src-rust/crates/tools/src/team_tool.rs` runtime wiring beyond the `AgentRunParams` transport seam
  - `TASK-M8-09` override activation in `TeamCreateTool::execute()`
  - fallback/D2 behavior

## Current HEAD Contract Check
- `init_team_swarm_runner()` still consumes `claurst_tools::team_tool::AgentRunParams`: yes
  - verified at `src-rust/crates/query/src/agent_tool.rs:570-584`
- `provider_override` and `model_override` are still consumed in the producer seam: yes
  - `model_override` selects the model at `src-rust/crates/query/src/agent_tool.rs:609-611`
  - `provider_override` is passed into `resolve_provider_identity(...)` at `src-rust/crates/query/src/agent_tool.rs:613-616`
- Hardcoded producer-path `ANTHROPIC_API_KEY` lookup remains removed there: yes
  - no such lookup remains in `init_team_swarm_runner()`
- Provider/model resolution still flows through shared seam helpers: yes
  - `resolve_provider_identity(...)` at `src-rust/crates/query/src/agent_tool.rs:613-617`
  - `materialize_provider(...)` at `src-rust/crates/query/src/agent_tool.rs:627-635`
- Child `QueryConfig` still carries provider/model seam state: yes
  - `model: target.model_id.clone()` at `src-rust/crates/query/src/agent_tool.rs:680-681`
  - `provider_registry: Some(registry.clone())` at `src-rust/crates/query/src/agent_tool.rs:688`
  - `model_registry: ctx.model_registry.clone()` at `src-rust/crates/query/src/agent_tool.rs:689`
- Selected provider is still preserved into `run_query_loop()` through cloned `ToolContext.config.provider`: yes
  - `runner_ctx.config.provider = Some(target.provider_id.clone())` at `src-rust/crates/query/src/agent_tool.rs:693-694`
- `run_query_loop()` call shape, including the client parameter, remains preserved: yes
  - `run_query_loop(client.as_ref(), ...)` at `src-rust/crates/query/src/agent_tool.rs:698-707`
- No M8-07 change widened into query crate/runtime behavior: yes
  - `M8-07` additions remain confined to `src-rust/crates/tools/src/team_tool.rs`
  - current `AgentSpec.provider` / `AgentSpec.model` are schema-only fields at `src-rust/crates/tools/src/team_tool.rs:159-174`
  - current schema exposure remains in `src-rust/crates/tools/src/team_tool.rs:216-252`
- `TeamCreateTool::execute()` still only wires `None` overrides, leaving M8-09 as the first activation ticket: yes
  - `provider_override: None` at `src-rust/crates/tools/src/team_tool.rs:421`
  - `model_override: None` at `src-rust/crates/tools/src/team_tool.rs:422`

## Validation Commands Run
1. `cd /home/jordi/claurst/src-rust && cargo check -p claurst-query`
2. `cd /home/jordi/claurst/src-rust && cargo check --workspace`

## Validation Results
- `cargo check -p claurst-query`: PASS
  - finished successfully on current HEAD
  - warning observed from `claurst-tools`:
    - `fields 'provider' and 'model' are never read` in `crates/tools/src/team_tool.rs`
  - note:
    - this warning is consistent with current milestone state because `M8-07` added the fields and `M8-09` has not yet wired them into execution
- `cargo check --workspace`: PASS
  - finished successfully on current HEAD
  - same `claurst-tools` warning observed

## Current-HEAD Validity Assessment
- Historical `M8-08` commit correctness remains intact: yes
- Current `HEAD` still preserves the accepted `M8-08` contract after retroactive `M8-07` closure: yes
- `M8-08` remains fully valid on current HEAD: yes
- `M8-09` can proceed without revisiting `M8-08`: yes
- Reason:
  - producer seam remains correct and unchanged in behavior
  - `M8-07` stayed within schema/transport boundary
  - `TeamCreateTool::execute()` still passes `None` overrides, so `M8-09` remains the intended connector ticket

## Hosted Ollama Invariant Outcome
Hosted Ollama compatibility baseline preserved

## Notes
- The previously accepted MPWO wording inconsistency still exists:
  - step-level instructions preserve the `run_query_loop()` client parameter
  - definition-of-done language says no `AnthropicClient::new()` calls remain
- Current HEAD still matches the accepted preserved-client-parameter interpretation used for `TASK-M8-08` closeout and verification.

## Verdict
- Verdict: `VERIFIED-WITH-NOTES`
- No regression detected in the accepted `TASK-M8-08` contract on current HEAD
