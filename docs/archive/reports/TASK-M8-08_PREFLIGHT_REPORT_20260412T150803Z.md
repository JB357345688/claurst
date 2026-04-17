# TASK-M8-08 Preflight Report

## Ticket
`TASK-M8-08`

## Timestamp UTC
`2026-04-12T15:08:03Z`

## Branch
`feature/provider-resolution-seam`

## Working Tree Summary
- `git branch --show-current` confirms branch `feature/provider-resolution-seam`.
- `git log --oneline --decorate -n 20` confirms `HEAD` is `ea9da37 TASK-M8-06 replace AgentRunFn positional seam with AgentRunParams`.
- `TASK-M8-06` is already committed at `HEAD`.
- There are no later commits on top of `TASK-M8-06`; the immediate branch baseline under `HEAD` is `ced6005 Harden provider-aware compaction for post-M8-05 registry-backed runs`.
- `git diff --name-only` and `git diff --cached --name-only` are empty: no unstaged tracked changes and no staged changes.
- `git status --short` shows substantial untracked noise, notably `.codex`, `docs/Current/`, many `docs/archive/reports/*.md`, and `src-rust/target/**`.
- `docs/Current/MPWO_WORK_ORDER_PACK.md` itself is present but untracked (`?? docs/Current/MPWO_WORK_ORDER_PACK.md`).
- Baseline is clean enough for preflight and likely execution planning, but not fully clean for later closure review unless the untracked noise is explicitly excluded from the review basis.

## Authority Reviewed
- `AGENTS.md` at repo root.
- `docs/Current/MPWO_WORK_ORDER_PACK.md`, specifically:
  - dependency graph references for `M8-06 -> M8-08`
  - `TASK-M8-06` section
  - `TASK-M8-08` section
  - `TASK-M8-09` section for boundary confirmation only

## Verified Targets / Files / Symbols / Commands

| Type | Verified Item | Current Reality / Evidence |
|---|---|---|
| Command | `git branch --show-current` | `feature/provider-resolution-seam` |
| Command | `git status --short` | no tracked diffs; large untracked noise including authority pack and `src-rust/target/**` |
| Command | `git log --oneline --decorate -n 20` | `HEAD=ea9da37` (`TASK-M8-06`), prior baseline includes `ced6005` hardening and `5f8dfe1` hosted Ollama fix |
| Authority | `docs/Current/MPWO_WORK_ORDER_PACK.md` | `TASK-M8-08` is defined as replacing `init_team_swarm_runner()` hardcoded Anthropic producer with shared seam |
| File / Symbol | `src-rust/crates/tools/src/team_tool.rs` | `AgentRunParams` exists at line 37; `AgentRunFn` takes `AgentRunParams`; `run_agent()` takes `AgentRunParams`; call site still passes `provider_override: None` and `model_override: None` |
| File / Symbol | `src-rust/crates/query/src/agent_tool.rs` | `AgentTool::execute()` already resolves provider/model and populates child `QueryConfig`; `init_team_swarm_runner()` at line 570 still uses old 6-arg closure and hardcoded Anthropic setup |
| File / Symbol | `src-rust/crates/query/src/lib.rs` | `QueryConfig` already has `provider_registry` and `model_registry`; `run_query_loop()` still requires `client: &AnthropicClient` and dispatches through `provider_registry` when present |
| File / Symbol | `src-rust/crates/query/src/provider_resolution.rs` | `resolve_provider_identity()` at line 101 and `materialize_provider()` at line 157 exist and are usable by the producer seam |
| Command | MPWO validation commands | Step text says `cd src-rust && cargo check -p claurst-query`; validation block says `cd src-rust && cargo check --workspace` |

## Exact M8-08 Scope Confirmation
- Objective from MPWO: update `init_team_swarm_runner()` so the producer closure accepts `AgentRunParams` and uses `resolve_provider_identity()` plus `materialize_provider()` instead of `ANTHROPIC_API_KEY` plus `AnthropicClient::new()` hardcoding.
- Exact code target from MPWO: `crates/query/src/agent_tool.rs`, specifically `init_team_swarm_runner()`, its API-key/client creation block, and the child `QueryConfig` construction.
- Preconditions from MPWO: `TASK-M8-06` complete; Milestone 7 provider-resolution functions available. Current repo reality satisfies those preconditions.
- Required behavior from MPWO: consume `params.provider_override` and `params.model_override`, preserve `run_query_loop()` call shape, keep the `client` parameter, and propagate `provider_registry` plus `model_registry` into the child `QueryConfig`.
- Strict constraints from MPWO: no fallback behavior, no broader `register_agent_runner()` rewrite, no `run_query_loop()` call-structure rewrite beyond the config/client handoff.
- Definition of done from MPWO: `init_team_swarm_runner()` accepts `AgentRunParams`, no direct `ANTHROPIC_API_KEY` reads or `AnthropicClient::new()` hardcoding remain there, child `QueryConfig` carries both registries, and the query crate compiles with the M8-06 transport seam.
- Stop / escalate conditions from MPWO: boxed-future lifetime/`Send` issues in the closure, or evidence that `run_query_loop()` still validates Anthropic credentials before the registry-backed path can short-circuit.
- MPWO frames `TASK-M8-08` as a narrow producer-closure / `init_team_swarm_runner()` update that implements provider/model propagation inside the runner producer seam.
- MPWO does not frame `TASK-M8-08` as a broader team-runner rewrite.
- MPWO does not mention compaction/context-collapse parity in `TASK-M8-08`; no compaction/context-collapse work is pulled into this ticket by authority.

## Current Post-M8-06 State
- `team_tool.rs` is already on the struct transport seam introduced by `TASK-M8-06`: `AgentRunParams` contains `description`, `prompt`, `tools`, `system_prompt`, `max_turns`, `ctx`, `provider_override`, and `model_override`.
- `run_agent()` in `team_tool.rs` already consumes `AgentRunParams`, so `TASK-M8-08` is the producer-side seam that must now match it.
- `TeamCreateTool::execute()` currently builds `AgentRunParams` with `provider_override: None` and `model_override: None`. That proves the transport seam exists, but later per-agent propagation is not wired yet on this branch.
- `AgentSpec` in current `team_tool.rs` does not yet carry `provider` or `model`; that reinforces that `M8-09` remains separate and should not be pulled into `TASK-M8-08`.
- `AgentTool::execute()` in `agent_tool.rs` already resolves provider identity, materializes the provider, uses a real Anthropic client only for Anthropic targets, uses `ClientConfig::default()` for non-Anthropic targets, and clones `provider_registry` plus `model_registry` into child `QueryConfig`.
- The current foreground and background `AgentTool::execute()` paths are branch baseline from the already-closed foreground/background seams and must not be misattributed to `TASK-M8-08`.
- `run_query_loop()` already has the registry-backed dispatch gate in `src-rust/crates/query/src/lib.rs`: when `config.provider_registry` is `Some`, it resolves provider identity from `tool_ctx.config.provider` plus effective model, materializes the provider, and returns early on failure without falling through to the raw Anthropic path.
- `init_team_swarm_runner()` is the outstanding seam. It still:
  - uses the old 6-positional closure signature
  - reads `ANTHROPIC_API_KEY`
  - constructs `AnthropicClient::new(...)` directly
  - fixes the child model to `DEFAULT_MODEL`
  - does not propagate `provider_registry` or `model_registry` into its child `QueryConfig`
  - does not consume `provider_override` or `model_override`
- Detected later hardening baseline: `ced6005` changed `src-rust/crates/query/src/compact.rs` and `src-rust/crates/query/src/lib.rs` for provider-aware compaction/context-collapse behavior. That is present on this branch and should be treated as fixed baseline outside `TASK-M8-08`.
- Hosted Ollama compatibility baseline preserved

## Dependency / Interface Shape Notes
- Proven: MPWO and current code agree that `TASK-M8-06` is the transport seam `TASK-M8-08` must consume.
- Proven: the producer seam is isolated to `init_team_swarm_runner()`; the foreground path and background path are already using the shared provider-resolution flow and are not the active ticket.
- Proven: `run_query_loop()` still takes `client: &AnthropicClient`, so `TASK-M8-08` must preserve that parameter and mirror the existing "default client for non-Anthropic targets" pattern already used in `AgentTool::execute()`.
- Proven: `ToolContext` is `Clone`, and `provider_registry` / `model_registry` already exist on both `ToolContext` and `QueryConfig`.
- Proven: `claurst_tools` re-exports `AgentRunFn` but does not currently re-export `AgentRunParams`. A tight implementation can avoid widening scope by referring to `claurst_tools::team_tool::AgentRunParams` directly from `agent_tool.rs`.
- Likely implementation challenge: simple producer closure signature migration to `AgentRunParams` plus owned-value packing inside the boxed async closure. That is a narrow adaptation, not a broader architecture change.
- Likely implementation challenge: wire `provider_override` / `model_override` into provider resolution and child config construction in the producer closure. Current consumers still pass `None`, but the seam must be ready for later callers.
- Low-uncertainty note: the MPWO stop condition about dummy-client compatibility looks manageable because current `AgentTool::execute()` already proves the non-Anthropic default-client pattern exists in this codebase.
- No evidence of broader structural drift in the producer seam was found.
- Post-M8-05 hardening does not change the likely M8-08 edit surface; it remains orthogonal to the producer closure update.

## Likely Edit Surface
- Definitely in scope: `src-rust/crates/query/src/agent_tool.rs`
  - `init_team_swarm_runner()`
  - local imports needed for `AgentRunParams` access
  - local provider-resolution / client-selection / `QueryConfig` wiring inside that function only
- Maybe in scope: `src-rust/crates/tools/src/lib.rs`
  - only if an ergonomic re-export of `AgentRunParams` is chosen
  - not technically required, because `claurst_tools::team_tool::AgentRunParams` is already public
- Should remain untouched if the ticket stays tight:
  - `src-rust/crates/tools/src/team_tool.rs`
  - `src-rust/crates/query/src/lib.rs`
  - `src-rust/crates/query/src/compact.rs`
  - `src-rust/crates/query/src/provider_resolution.rs`
  - tests/helpers not explicitly required by `TASK-M8-08`
- Current repo reality suggests the execution patch should stay in `src-rust/crates/query/src/agent_tool.rs` only.
- A second file is not likely required unless the implementation elects to add a convenience re-export.
- Widening risks to avoid:
  - `M8-09` / later per-agent provider/model propagation in `team_tool.rs`
  - reopening the already-closed `AgentRunParams` transport seam in `team_tool.rs`
  - touching shared query-loop or compaction/context-collapse layers
  - touching hosted Ollama/provider-resolution baseline outside the producer seam

## Validation Command
- MPWO contains two validation directives:
  - narrow step-specific check: `cd src-rust && cargo check -p claurst-query`
  - broader validation block: `cd src-rust && cargo check --workspace`
- I did not run either command during this preflight.
- Reason for skipping: the user requested a read-only pass that keeps the worktree unchanged except for this report file, and running Cargo in this repo would write additional artifacts under the already-untracked `src-rust/target/**`.
- Validation conclusions in this report rely on current source inspection, current git history, and the explicit M8-06/M8-08 MPWO relationship rather than a fresh build.

## Drift Found
- No structural drift found in the code seam itself.
- Line-number drift only: MPWO cites `init_team_swarm_runner()` around `agent_tool.rs:517-607`; current function is at approximately `570-660`.
- Path-prefix drift only: MPWO uses crate-relative paths; actual repo paths are under `src-rust/crates/...`.
- MPWO contains an internal validation-command inconsistency: step text targets `-p claurst-query`, while the validation block says `--workspace`.
- Repo-state note: the authority file `docs/Current/MPWO_WORK_ORDER_PACK.md` is untracked in the current worktree, so the authority basis is explicit but not committed baseline.

## Blockers
- No execution blocker found for preflight.
- Review-basis note only: untracked `docs/Current/**`, many archived reports, and `src-rust/target/**` should be treated as branch noise and kept out of any later ticket closure decision.

## Verdict
- Verdict: `READY-WITH-NOTES`
- Ticket still looks narrow enough for a single tight implementation pass: `yes`
- Structural drift exists: `no`
- Smallest plausible edit surface: `src-rust/crates/query/src/agent_tool.rs` only
- Later post-M8-05 hardening detected: `yes`; treat it as fixed branch baseline outside `TASK-M8-08` scope
- `TASK-M8-06` is committed at `HEAD`, and no later commits sit on top of it
- `TASK-M8-08` should consume the existing `AgentRunParams` seam, update only the producer closure, and avoid widening into `M8-09`, `team_tool.rs`, or shared compaction/query-loop layers
