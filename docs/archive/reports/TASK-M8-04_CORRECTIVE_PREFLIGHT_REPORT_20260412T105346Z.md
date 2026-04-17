# TASK-M8-04 Corrective Preflight Report

Ticket: `TASK-M8-04 — foreground AgentTool::execute() provider-resolution wiring`
Timestamp UTC: `20260412T105346Z`
Branch: `feature/provider-resolution-seam`

## Working Tree Summary
- `git diff --cached --name-only`: no staged tracked changes.
- `git diff --name-only`: only tracked source diff is `src-rust/crates/query/src/agent_tool.rs`.
- `git status --short`: untracked noise includes `.codex`, `docs/Current/`, `docs/Orchestrator_planning/`, many `docs/archive/reports/*.md`, and `src-rust/target/`.
- `TASK-M8-04` remains the only active tracked ticket diff.
- Review basis note: repo-local authority files `AGENTS.md` and `docs/Current/MPWO_WORK_ORDER_PACK.md` are present locally but untracked, so the tracked source patch is isolated but the overall worktree is noisy.

## Authority Reviewed
- `AGENTS.md`: verified repo-local authority. One-ticket-only, report under `docs/archive/reports`, read-only preflight, explicit drift/blocker reporting.
- `docs/Current/MPWO_WORK_ORDER_PACK.md` `TASK-M8-04`:
  - Objective: replace foreground hardcoded `ANTHROPIC_API_KEY` + `AnthropicClient::new()` with `resolve_provider_identity()` + `materialize_provider()`.
  - In-scope foreground behavior: resolve provider/model in `agent_tool.rs`, set child `QueryConfig.model` to the resolved model, set `provider_registry: Some(...)`, carry `model_registry`, and preserve the existing `run_query_loop(client, ...)` signature.
  - Strict constraints: no fallback behavior, no `run_query_loop()` signature removal, no background-path work (`TASK-M8-05`), no `init_team_swarm_runner()` work (`TASK-M8-08`), no tool-list or system-prompt redesign.
  - Stop/escalate condition: if the dummy `AnthropicClient` is not actually safe when `provider_registry` is `Some(...)`, inspect the `run_query_loop()` control flow before editing.
  - Validation command: `cd src-rust && cargo check -p claurst-query`.
- MPWO implication check: step 5 explicitly assumes dummy-client safety because registry-backed dispatch will bypass raw Anthropic client usage. Current repo reality only partially matches that rationale: registry-backed foreground runs do bypass the raw client, but by returning before the compaction block rather than by making compaction/context-collapse registry-aware.

## Verified Files / Symbols / Commands

| Kind | Item | Verified note |
|---|---|---|
| Command | `git branch --show-current` | `feature/provider-resolution-seam` |
| Command | `git diff --name-only` | only `src-rust/crates/query/src/agent_tool.rs` |
| Command | `git diff --cached --name-only` | empty |
| File | `AGENTS.md` | present locally, untracked |
| File | `docs/Current/MPWO_WORK_ORDER_PACK.md` | present locally, untracked; `TASK-M8-04` at `978+` |
| Symbol | `AgentTool::execute()` | provider resolution at `src-rust/crates/query/src/agent_tool.rs:250-316`; child config at `:388-412`; foreground call at `:483-489` |
| Symbol | Registry dispatch | `src-rust/crates/query/src/lib.rs:871-999` |
| Symbol | Provider-branch exit | `src-rust/crates/query/src/lib.rs:1184` `continue`; `:1196-1199` `return QueryOutcome::EndTurn` |
| Symbol | Raw compaction calls | `src-rust/crates/query/src/compact.rs:633`, `:1050`; invoked only from `src-rust/crates/query/src/lib.rs:1392-1442` |
| Symbol | Dummy-client safety | `src-rust/crates/api/src/lib.rs:454-462` defers validation; `:613-651` fails first stream call on empty key |

## Failed-Review Finding Recheck
- Status: `partially confirmed`
- Confirmed:
  - `src-rust/crates/query/src/compact.rs:633` and `:1050` still call `client.create_message_stream(...)` directly.
  - `AnthropicClient::new(ClientConfig::default())` succeeds because validation is deferred (`src-rust/crates/api/src/lib.rs:454-462`), but the first `create_message_stream()` call fails immediately when `api_key` is empty and provider is still Anthropic (`src-rust/crates/api/src/lib.rs:613-651`).
- Not confirmed as written:
  - In current foreground M8-04 wiring, `src-rust/crates/query/src/agent_tool.rs:405-412` and `:483-489` pass `provider_registry: Some(...)` plus `foreground_ctx.config.provider = Some(target.provider_id.clone())`.
  - `run_query_loop()` then takes the registry-backed branch at `src-rust/crates/query/src/lib.rs:874-989`.
  - That branch handles tool turns and exits at `src-rust/crates/query/src/lib.rs:1184` or `:1196-1199`, before the later compaction/context-collapse block at `src-rust/crates/query/src/lib.rs:1378-1442`.
- Conclusion: the raw-client compaction seam remains real and unsafe if reached, but the exact claim that current foreground registry-backed sub-agents still reach it is not reproduced against current repo control flow.

## Compaction / Context-Collapse Path Analysis
- Current foreground causal chain:
  1. `src-rust/crates/query/src/agent_tool.rs:270-316` resolves `identity` and `target`, then constructs a default/dummy `AnthropicClient` for non-Anthropic targets.
  2. `src-rust/crates/query/src/agent_tool.rs:388-412` sets `QueryConfig.model = target.model_id`, `provider_registry: Some(registry.clone())`, `model_registry: ctx.model_registry.clone()`, and `foreground_ctx.config.provider = Some(target.provider_id.clone())`.
  3. `src-rust/crates/query/src/agent_tool.rs:483-489` calls `run_query_loop(client.as_ref(), ..., &foreground_ctx, &query_config, ...)`.
  4. `run_query_loop()` sees `config.provider_registry.is_some()` and dispatches through `resolve_provider_identity()` + `materialize_provider()` + `provider.create_message_stream(...)` at `src-rust/crates/query/src/lib.rs:874-989`.
  5. That registry branch then `continue`s on tool-use or `return`s end-turn at `src-rust/crates/query/src/lib.rs:1184` / `:1196-1199`.
  6. The later reactive/proactive compaction block at `src-rust/crates/query/src/lib.rs:1378-1442` is therefore not entered for the current registry-backed foreground path.
- Separate seam check:
  - If `compact.rs` were ever invoked with the dummy client, `summarise_head()` and `context_collapse()` would hit raw `client.create_message_stream(...)` at `src-rust/crates/query/src/compact.rs:633` and `:1050`.
  - For a non-Anthropic foreground dummy client, that call would fail auth before any network request because the client keeps an empty API key and Anthropic provider defaults (`src-rust/crates/api/src/lib.rs:613-651`).
- Hosted Ollama compatibility baseline preserved

## Corrective Options Compared

| Option | Viable | Likely files | Ticket-semantic impact | Later-ticket risk | Hosted Ollama baseline |
|---|---|---|---|---|---|
| A. Keep fix inside `agent_tool.rs` by manufacturing a "safe real client" for compaction-capable non-Anthropic runs | No | `src-rust/crates/query/src/agent_tool.rs` | Would force Anthropic-specific client semantics onto non-Anthropic compaction or require an undocumented fallback provider/model | High | Not reliably preserved |
| B. Widen so compaction/context-collapse use the registry-aware provider path when `provider_registry` is `Some(...)` | Yes, but broader than the reviewed defect | `src-rust/crates/query/src/lib.rs`, `src-rust/crates/query/src/compact.rs` | Extends shared query-loop behavior beyond M8-04 foreground wiring; useful hardening, but not required by the rechecked reachability claim | Medium | Preserved if implemented through existing `resolve_provider_identity()` + `materialize_provider()` seam |
| C. No source corrective edit for this finding; re-review against current control flow | Yes | none | No behavior change; treats the failed review finding as not reproduced for the current foreground path | Low | Preserved |

## Recommended Corrective Scope
- Recommendation: `corrective fix can remain in src-rust/crates/query/src/agent_tool.rs only`
- Rationale: the specific failed-review finding is not reproduced for current registry-backed foreground runs; no minimal safe widening into shared `query/src/lib.rs` or `query/src/compact.rs` is justified by current repo reality.
- File classification:
  - Definitely required: none beyond the existing `src-rust/crates/query/src/agent_tool.rs` tracked diff if the ticket is simply re-reviewed against actual control flow.
  - Maybe required: `src-rust/crates/query/src/lib.rs` and `src-rust/crates/query/src/compact.rs` only if the orchestrator intentionally widens scope from "foreground provider wiring" to "registry-backed compaction parity".
  - Should remain untouched: `src-rust/crates/api/src/lib.rs`; background-agent path in `src-rust/crates/query/src/agent_tool.rs:416-447` remains `TASK-M8-05`.
- Corrective fix likely: `single-file` for the reviewed defect; `two-file` only if scope is deliberately widened to compaction parity.
- Corrective execution pass state: `NOT READY` until the orchestrator decides whether to close/re-review `TASK-M8-04` as-is or explicitly authorize the broader `query/src/lib.rs` + `query/src/compact.rs` parity change.

## Validation Command
- Confirmed MPWO validation: `cd src-rust && cargo check -p claurst-query`.
- Not run in this preflight, to honor the "read-only except report file" constraint and avoid mutating the existing untracked `src-rust/target/`.

## Drift Found
- Local authority files `AGENTS.md` and `docs/Current/MPWO_WORK_ORDER_PACK.md` are untracked in git.
- The MPWO dummy-client rationale assumes registry-backed safety generally; current implementation achieves foreground safety only because the registry-backed branch exits before the compaction block, not because compaction itself is registry-aware.
- Extensive untracked docs/report noise remains in the worktree.

## Blockers
- The specific corrective edit requested by the failed review is not justified by current foreground reachability.
- Any attempt to "fix" registry-backed compaction now would widen into shared `query/src/lib.rs` + `query/src/compact.rs` behavior and should be explicitly re-authorized as broader scope.

## Verdict
- Ticket: `TASK-M8-04`
- Failed-review finding: `partially confirmed`
- Smallest safe corrective scope: no additional widening justified beyond current `agent_tool.rs` foreground ticket scope
- Corrective fix likely: `single-file`
- Corrective execution pass: `NOT READY`
- Hosted Ollama compatibility baseline preserved
