# TASK-M8-04 Adjudication Report

Ticket: `TASK-M8-04 — wire the foreground AgentTool::execute() path through the shared provider-resolution seam`
Timestamp UTC: `20260412T111215Z`
Branch: `feature/provider-resolution-seam`

## Working Tree Summary
- `git branch --show-current`: `feature/provider-resolution-seam`
- `git diff --cached --name-only`: no staged tracked changes.
- `git diff --name-only`: active tracked source diff remains limited to `src-rust/crates/query/src/agent_tool.rs`.
- `git status --short`: tolerated pre-existing untracked noise is present under `.codex/`, `docs/Current/`, `docs/Orchestrator_planning/`, `docs/archive/provider_orchestrator/`, `docs/archive/reports/`, and `src-rust/target/`.
- Review basis for this adjudication: current unstaged tracked diff only.

## Authority Criteria Reviewed
- `AGENTS.md` requires one-ticket-only scope, explicit review basis, validation before closure, and no ticket expansion without authority.
- `docs/Current/MPWO_WORK_ORDER_PACK.md` `TASK-M8-04` acceptance criteria adjudicated here:
  - Foreground path no longer reads `ANTHROPIC_API_KEY` directly.
  - Foreground path no longer hardcodes `AnthropicClient::new()` from raw env-only resolution.
  - Foreground path uses `resolve_provider_identity()` + `materialize_provider()`.
  - Child `QueryConfig` carries `provider_registry: Some(...)`.
  - Child `QueryConfig` carries `model_registry: ctx.model_registry.clone()`.
  - Child `QueryConfig.model` is the resolved `target.model_id`.
  - `cargo check -p claurst-query` compiles.
- MPWO scope determination:
  - `TASK-M8-04` requires foreground provider-resolution wiring only.
  - MPWO does not require compaction/context-collapse parity as acceptance criteria for this ticket.
  - The dummy-client/compaction concern is explicitly a stop/escalate investigation condition, not a separate definition-of-done item for `TASK-M8-04`.

## Original Failed Finding
- Adjudicated status: `partially confirmed but not reachable in current TASK-M8-04 foreground flow`
- Confirmed portion:
  - `src-rust/crates/query/src/compact.rs:633` and `:1050` still call `client.create_message_stream(...)` directly.
  - A dummy `AnthropicClient` with empty/default config is theoretically unsafe if those raw compaction paths are reached, because first streaming use fails deferred auth validation in `src-rust/crates/api/src/lib.rs:613-651`.
- Not confirmed as a live `TASK-M8-04` foreground defect:
  - Current registry-backed foreground sub-agent flow does not reproduce reachability from the active foreground ticket path into that raw compaction block.

## Current Control-Flow Recheck
- Foreground provider/model materialization:
  - `src-rust/crates/query/src/agent_tool.rs:250-288` resolves the foreground model, derives provider identity, and materializes the provider through the shared seam.
  - `src-rust/crates/query/src/agent_tool.rs:290-316` constructs an `AnthropicClient`; for non-Anthropic targets this can be default/dummy because the registry-backed path is intended to own execution.
- Child `QueryConfig` construction:
  - `src-rust/crates/query/src/agent_tool.rs:388-409` sets `model: target.model_id.clone()`, `provider_registry: Some(registry.clone())`, and `model_registry: ctx.model_registry.clone()`.
  - `src-rust/crates/query/src/agent_tool.rs:411-412` sets `foreground_ctx.config.provider = Some(target.provider_id.clone())`.
- Registry-backed `run_query_loop()` branch:
  - `src-rust/crates/query/src/lib.rs:874-989` resolves provider identity again from the propagated foreground provider/model context, materializes the provider, builds a provider request, and dispatches through `provider.create_message_stream(...)`.
- Registry-backed branch exit points:
  - `src-rust/crates/query/src/lib.rs:1184` exits back to the top of the loop with `continue` after tool-use handling.
  - `src-rust/crates/query/src/lib.rs:1196-1199` returns `QueryOutcome::EndTurn` on normal turn completion.
- Raw compaction/context-collapse block:
  - Begins later at `src-rust/crates/query/src/lib.rs:1378-1442`.
  - Uses raw Anthropic-client-based helpers in `src-rust/crates/query/src/compact.rs:633`, `:715`, `:937`, and `:1050`.
- Reachability adjudication:
  - For the current registry-backed foreground sub-agent path created by `TASK-M8-04`, the branch at `src-rust/crates/query/src/lib.rs:874-1199` exits before the later compaction/context-collapse block.
  - Result: the dummy Anthropic client is theoretically unsafe in shared compaction code, but not actually reachable in current foreground `TASK-M8-04` execution.

## Ticket-Scope Adjudication
- In-scope live foreground defect check:
  - No live defect found in the current `TASK-M8-04` foreground registry-backed path.
- Out-of-scope hardening opportunity:
  - Shared compaction/context-collapse still rely on raw `AnthropicClient` calls and are not registry-aware.
  - That is a theoretical/shared-seam hardening opportunity outside this ticket’s acceptance contract unless separately authorized.
- Acceptance decision:
  - The current patch satisfies the `TASK-M8-04` MPWO contract as written.
  - `TASK-M8-04` is review-accepted.
  - No further code patch is required before closeout for this ticket.
  - `M8-05` may be next only after `M8-04` closeout is complete.

## Scope / Non-Regression Review
- Active tracked source diff remains limited to `src-rust/crates/query/src/agent_tool.rs`.
- Background block remains untouched for ticket purposes:
  - `src-rust/crates/query/src/agent_tool.rs:416-447` is still the separate background path targeted by `TASK-M8-05`.
- `init_team_swarm_runner()` remains untouched:
  - symbol still present at `src-rust/crates/query/src/agent_tool.rs:570`; no tracked diff outside the foreground hunk.
- Verified unchanged in tracked diff:
  - `src-rust/crates/query/src/lib.rs`
  - `src-rust/crates/query/src/provider_resolution.rs`
  - `src-rust/crates/query/src/compact.rs`
  - `src-rust/crates/tools/src/team_tool.rs`
  - `src-rust/crates/tools/src/lib.rs`
  - `src-rust/crates/cli/src/main.rs`
  - `tests`
- No fallback behavior change detected.
- No new registries were created.

## Validation Commands Run
```bash
cd /home/jordi/claurst/src-rust && cargo check -p claurst-query
```

## Validation Results
- Result: pass
- Output:
  - `Finished 'dev' profile [unoptimized + debuginfo] target(s) in 0.13s`

## Final Findings
- Original failed-review finding is `partially confirmed but not reachable in current TASK-M8-04 foreground flow`.
- The compaction issue is not reachable for current foreground registry-backed runs.
- `TASK-M8-04` is review-accepted.
- Ready for closeout/commit: `yes`, on the current explicit review basis of the active unstaged tracked diff in `src-rust/crates/query/src/agent_tool.rs`.
- Further code patch required before closeout: `no`.

## Hosted Ollama Invariant Outcome
- Hosted Ollama compatibility baseline preserved

## Verdict
- `PASS-WITH-NOTES`
