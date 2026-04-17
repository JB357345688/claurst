# TASK-M8-04 Review Report

## Ticket
`TASK-M8-04`

## Timestamp UTC
`20260412T103951Z`

## Branch
`feature/provider-resolution-seam`

## Working Tree Summary
- `git branch --show-current`: `feature/provider-resolution-seam`
- Tracked working-tree status:
  - `git diff --name-only`: `src-rust/crates/query/src/agent_tool.rs`
  - `git diff --cached --name-only`: empty
- The active tracked source diff is limited to `src-rust/crates/query/src/agent_tool.rs` only.
- No other tracked source files are modified.
- Pre-existing untracked noise remains under tolerated locations, including `.codex/`, `docs/Current/`, `docs/archive/reports/`, `docs/Orchestrator_planning/`, `docs/archive/provider_orchestrator/`, and `src-rust/target/`.

## Authority Criteria Reviewed
- `AGENTS.md` requirements reviewed:
  - verify referenced files/symbols/commands before judging
  - keep review basis limited to the active ticket diff
  - report exact violations and minimal corrective actions
  - stop on failed validation
- `docs/Current/MPWO_WORK_ORDER_PACK.md` `TASK-M8-04` criteria reviewed:
  - foreground `AgentTool::execute()` must stop using the hardcoded `ANTHROPIC_API_KEY` + direct Anthropic path
  - foreground path must use `resolve_provider_identity()` and `materialize_provider()`
  - model must be resolved before provider resolution
  - child `QueryConfig` must carry propagated `provider_registry` and `model_registry`
  - child `QueryConfig.model` must be set from the resolved target model
  - `run_query_loop()` signature must remain unchanged
  - background block must remain for `TASK-M8-05`
  - `init_team_swarm_runner()` must remain for `TASK-M8-08`
  - validation command is `cd /home/jordi/claurst/src-rust && cargo check -p claurst-query`

## Files Reviewed
- Diff reviewed:
  - `src-rust/crates/query/src/agent_tool.rs`
- Unchanged-by-diff scope-checked:
  - `src-rust/crates/query/src/lib.rs`
  - `src-rust/crates/query/src/provider_resolution.rs`
  - `src-rust/crates/tools/src/team_tool.rs`
  - `src-rust/crates/tools/src/lib.rs`
  - `src-rust/crates/cli/src/main.rs`
  - tests
- Behavior cross-check files read for review:
  - `src-rust/crates/query/src/compact.rs`
  - `src-rust/crates/api/src/lib.rs`

## Foreground Provider-Resolution Review
- The foreground execute path no longer reads `ANTHROPIC_API_KEY` directly for this path.
- The foreground execute path now uses `resolve_provider_identity()` and `materialize_provider()`.
- The child `QueryConfig` now carries propagated `provider_registry` and `model_registry` values.
- The child `QueryConfig.model` now reflects the resolved target model.
- The selected provider is preserved into the nested synchronous `run_query_loop()` call by cloning `ToolContext` and setting `foreground_ctx.config.provider = Some(target.provider_id.clone())`.
- `run_query_loop()` signature was not changed.
- Review note: the synchronous foreground execution path is wired ticket-compatibly for ordinary registry-backed turns.

## Scope / Non-Regression Review
- Only `src-rust/crates/query/src/agent_tool.rs` is part of the active tracked source diff for this ticket.
- The patch stayed in `agent_tool.rs` only.
- The background block in `agent_tool.rs` was not modified as a block of logic for `TASK-M8-05`; the diff only inserts foreground setup immediately before it.
- `init_team_swarm_runner()` was not modified.
- No changes were found in:
  - `src-rust/crates/query/src/lib.rs`
  - `src-rust/crates/query/src/provider_resolution.rs`
  - `src-rust/crates/tools/src/team_tool.rs`
  - `src-rust/crates/tools/src/lib.rs`
  - `src-rust/crates/cli/src/main.rs`
  - tests
- No fallback behavior changes were found in the active diff.
- No new registries were created.
- Scope cleanliness assessment: the active diff is scope-clean for `TASK-M8-04` in file footprint, but not review-acceptable yet because of the functional issue listed in Findings.

## Validation Commands Run
```bash
cd /home/jordi/claurst/src-rust && cargo check -p claurst-query
```

## Validation Results
- Result: pass
- Output:
  - `Finished 'dev' profile [unoptimized + debuginfo] target(s) in 0.09s`

## Drift Versus Execution Report
- No factual drift found in diff scope:
  - the active tracked source diff is still only `src-rust/crates/query/src/agent_tool.rs`
  - the reported validation command still passes
  - the foreground path changes described in the execution report are present
- Review found one material omission from the execution report:
  - the execution report did not call out that non-Anthropic foreground agents still depend on a dummy `AnthropicClient` in compaction/context-collapse paths outside the normal registry-backed turn dispatch

## Findings
- `FAIL` `src-rust/crates/query/src/agent_tool.rs:290-316`, `src-rust/crates/query/src/compact.rs:633`, `src-rust/crates/query/src/compact.rs:1050`, `src-rust/crates/api/src/lib.rs:613-651`
  - For non-Anthropic foreground agents, the patch constructs `ClientConfig::default()` and then `AnthropicClient::new(...)` as a dummy client. Normal turns are routed through the provider registry, but compaction and emergency context-collapse still call `client.create_message_stream(...)` directly. That API client rejects an empty-key Anthropic client at call time. Result: sufficiently long non-Anthropic foreground sub-agent sessions can lose compaction behavior or fail that path instead of remaining provider-agnostic. This is exactly the stop/escalate caveat the ticket called out for dummy-client safety, so `TASK-M8-04` is not review-accepted yet.
  - Minimal corrective action: perform a corrective patch that either proves and enforces a real Anthropic-capable client for all compaction-capable foreground runs, or routes compaction/context-collapse through the same registry-aware provider path when `config.provider_registry` is `Some(...)`. If that requires changing `src-rust/crates/query/src/lib.rs`, report the scope expansion explicitly before doing it.

## Hosted Ollama Invariant Outcome
Hosted Ollama compatibility baseline preserved

## Verdict
- Verdict: `FAIL`
- `TASK-M8-04` is not review-accepted.
- `TASK-M8-04` is not ready for closeout/commit.
- The diff is file-scope-clean, but it has one unresolved functional defect in the non-Anthropic foreground path.
- `M8-05` may be next only after `M8-04` closeout is complete.
