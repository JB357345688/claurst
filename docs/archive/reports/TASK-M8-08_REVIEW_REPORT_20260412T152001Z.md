# TASK-M8-08 Review Report

## Ticket
`TASK-M8-08`

## Timestamp UTC
`2026-04-12T15:20:01Z`

## Branch
`feature/provider-resolution-seam`

## Working Tree Summary
- Current branch: `feature/provider-resolution-seam`
- Current tracked working-tree status:
  - unstaged tracked diff: `src-rust/crates/query/src/agent_tool.rs`
  - staged tracked diff: none
- Active tracked source diff appears limited to `src-rust/crates/query/src/agent_tool.rs` only.
- Tolerated untracked noise remains present under:
  - `.codex/`
  - `docs/Current/`
  - `docs/archive/reports/`
  - `docs/Orchestrator_planning/`
  - `docs/archive/provider_orchestrator/`
  - `src-rust/target/`
- Review basis is scope-clean enough for `TASK-M8-08`: yes

## Authority Criteria Reviewed
- `AGENTS.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md` `TASK-M8-08`
- Acceptance criteria reviewed against current repo reality:
  - `init_team_swarm_runner()` must consume `AgentRunParams`
  - producer closure must consume `provider_override` and `model_override`
  - producer must resolve provider/model through `resolve_provider_identity(...)` and `materialize_provider(...)`
  - direct producer-path `ANTHROPIC_API_KEY` hardcoding must be removed
  - child `QueryConfig` must propagate `provider_registry` and `model_registry`
  - child `QueryConfig.model` must use the resolved target model
  - selected provider must be preserved into `run_query_loop()`
  - `run_query_loop()` call shape, including the client parameter, must be preserved
  - no fallback behavior should be added
  - `team_tool.rs`, shared query-loop code, provider-resolution code, and post-M8-05 compaction/context-collapse hardening remain out of scope
- Post-M8-05 compaction/context-collapse hardening is branch baseline and out of scope for this ticket.
- Review note on authority wording:
  - MPWO step 4b explicitly requires constructing the `AnthropicClient` argument for `run_query_loop()`.
  - MPWO definition-of-done also says no `AnthropicClient::new()` calls remain.
  - Those two statements are internally inconsistent once the client parameter must be preserved.
  - Review therefore evaluates the patch against the more specific behavioral contract: remove hardcoded producer-path Anthropic routing and mirror the established provider-aware client pattern.

## Files Reviewed
- Diff-reviewed:
  - `src-rust/crates/query/src/agent_tool.rs`
- Explicitly checked as unmodified tracked source files:
  - `src-rust/crates/tools/src/team_tool.rs`
  - `src-rust/crates/query/src/lib.rs`
  - `src-rust/crates/query/src/compact.rs`
  - `src-rust/crates/query/src/provider_resolution.rs`
- Execution-report cross-check:
  - `docs/archive/reports/TASK-M8-08_EXECUTION_REPORT_20260412T151649Z.md`

## Producer-Seam Review
- `init_team_swarm_runner()` now consumes `claurst_tools::team_tool::AgentRunParams` at current line `572`.
- The producer closure destructures and consumes:
  - `description`
  - `prompt`
  - `tools`
  - `system_prompt`
  - `max_turns`
  - `ctx`
  - `provider_override`
  - `model_override`
- `provider_override` and `model_override` are consumed inside the producer closure: yes
- Direct producer-path `ANTHROPIC_API_KEY` lookup was removed: yes
- Direct producer-path hardcoded Anthropic-only routing was removed: yes
- Producer now uses:
  - `resolve_provider_identity(...)` at current lines `613-617`
  - `materialize_provider(...)` at current lines `627-636`
- Child `QueryConfig` now carries:
  - `provider_registry: Some(registry.clone())`
  - `model_registry: ctx.model_registry.clone()`
  - `model: target.model_id.clone()`
- Selected provider is preserved into `run_query_loop()` by cloning the incoming `ToolContext` and setting:
  - `runner_ctx.config.provider = Some(target.provider_id.clone())`
- `run_query_loop()` call shape was preserved, including the `client` parameter: yes
- Review note:
  - `AnthropicClient::new(client_config)` still exists in the producer seam at current line `667`.
  - This is ticket-compatible with the more specific MPWO step 4b instructions and the preserved client-parameter constraint.
  - It is no longer hardcoded to `ANTHROPIC_API_KEY` or unconditional default-model Anthropic routing.

## Scope / Non-Regression Review
- Only `src-rust/crates/query/src/agent_tool.rs` is part of the active tracked source diff for this ticket: yes
- Patch stayed in `agent_tool.rs` only: yes
- A second tracked source file was actually required: no
- Foreground `AgentTool::execute()` path was modified: no
- Background `AgentTool::execute()` path was modified: no
- `team_tool.rs` transport seam was modified in this ticket: no
- Tracked changes to `src-rust/crates/query/src/lib.rs`: no
- Tracked changes to `src-rust/crates/query/src/compact.rs`: no
- Tracked changes to `src-rust/crates/query/src/provider_resolution.rs`: no
- Tracked changes to unrelated tests/helpers: no
- Fallback behavior changes detected: no
- Post-M8-05 compaction/context-collapse hardening baseline modified: no
- Active diff is scope-clean for `TASK-M8-08`: yes

## Validation Commands Run
1. `cd /home/jordi/claurst/src-rust && cargo check -p claurst-query`
2. `cd /home/jordi/claurst/src-rust && cargo check --workspace`

## Validation Results
- `cargo check -p claurst-query`: PASS
  - Output ended with: `Finished 'dev' profile [unoptimized + debuginfo] target(s) in 0.10s`
- `cargo check --workspace`: PASS
  - Output ended with: `Finished 'dev' profile [unoptimized + debuginfo] target(s) in 0.30s`

## Drift Versus Execution Report
- No code-scope drift found versus the execution report.
- No validation-result drift found versus the execution report.
- The execution report’s statements about scope, provider/model propagation, registry propagation, and preserved `run_query_loop()` shape match the actual current diff.
- Clarifying note only:
  - the execution report correctly described removal of producer-path hardcoding
  - the current code still contains `AnthropicClient::new(client_config)`, which matches the established provider-aware pattern and the MPWO step-level instructions

## Findings
- No blocking implementation findings.
- Note: MPWO contains an internal inconsistency between:
  - the step-level instruction to keep the `client` parameter and construct the `AnthropicClient` argument
  - the literal definition-of-done text saying no `AnthropicClient::new()` calls remain
- Minimal corrective action: none required for this ticket review pass, because the implemented seam matches the more specific behavioral contract and passes both required validations.

## Hosted Ollama Invariant Outcome
Hosted Ollama compatibility baseline preserved

## Verdict
- Verdict: `PASS-WITH-NOTES`
- `TASK-M8-08` is review-accepted: yes
- Ready for closeout/commit: yes
- Ready to close: yes
- M8-09 may be next only after M8-08 closeout is complete.
