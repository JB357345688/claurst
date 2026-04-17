# TASK-M8-05 Preflight Report

## Ticket
`TASK-M8-05`

## Timestamp UTC
`20260412T115058Z`

## Branch
`feature/provider-resolution-seam`

## Working Tree Summary
- `git diff --name-only` is empty.
- `git diff --cached --name-only` is empty.
- `git status --short` shows no staged changes and no unstaged tracked changes.
- The worktree is not clean: there is substantial untracked noise, including `docs/Current/MPWO_WORK_ORDER_PACK.md`, many `docs/archive/reports/*.md` files, multiple untracked doc directories, and `src-rust/target/`.
- Baseline clean enough to begin `TASK-M8-05` preflight: `YES`, with review-basis notes.
- `TASK-M8-04` is already committed at `HEAD`: `YES` (`b5249a3`, `TASK-M8-04 wire foreground provider resolution through shared seam`).

## Authority Reviewed
- Repo-local authority read: `AGENTS.md`
- Ticket authority read: `docs/Current/MPWO_WORK_ORDER_PACK.md`
- Git note: `AGENTS.md` is present in the worktree but git-ignored (`.gitignore:1:/AGENTS.md`).
- Git note: `docs/Current/MPWO_WORK_ORDER_PACK.md` is present in the worktree but untracked.

| Item | Type | Verified Reality |
|---|---|---|
| `git branch --show-current` | command | `feature/provider-resolution-seam` |
| `git log --oneline --decorate -n 12` | command | `HEAD` is `b5249a3` = `TASK-M8-04 wire foreground provider resolution through shared seam` |
| `git status --short` | command | No staged or tracked-file modifications; extensive untracked noise |
| `git diff --name-only` | command | Empty |
| `git diff --cached --name-only` | command | Empty |
| `AGENTS.md` | authority file | Present; read successfully; git-ignored, not tracked |
| `docs/Current/MPWO_WORK_ORDER_PACK.md` | authority file | Present; read successfully; untracked |
| `src-rust/crates/query/src/agent_tool.rs` | target file | Tracked at `HEAD`; contains foreground seam and background block |
| `AgentTool::execute()` | symbol | Verified in current `HEAD` state |
| `run_query_loop()` registry branch | symbol | Uses `tool_ctx.config.provider` plus `config.provider_registry` / `config.model_registry` |
| `ToolContext` | symbol | Has `config`, `provider_registry`, `model_registry`; `#[derive(Clone)]` |
| `cd src-rust && cargo check -p claurst-query` | validation command | Confirmed from MPWO; not run in this read-only preflight |

## Exact M8-05 Scope Confirmation
- Objective: apply the same provider-resolution seam used by `TASK-M8-04` to the background agent path.
- Exact MPWO target: background path inside `crates/query/src/agent_tool.rs`, specifically the `if params.run_in_background { ... }` block and its `client_bg` / `config_bg` captures.
- Current line drift only:
  - MPWO target block `362-420`
  - Current block `416-462`
  - `client_bg` now at line `426`
  - `config_bg` now at line `428`
- Preconditions from MPWO:
  - `TASK-M8-04` complete. Verified at `HEAD`.
  - Resolved `target` and provider-aware child `QueryConfig` are available before the background branch. Verified in current code.
- Required behavior from MPWO:
  - Background path must use the same provider-aware seam as foreground.
  - No background-path `ANTHROPIC_API_KEY` reads or background-path `AnthropicClient::new()` should remain.
  - `tokio::spawn` structure, worktree cleanup, and closure capture set must stay tight.
- Definition of done from MPWO:
  - Background agent path uses provider-aware client/config from `M8-04`.
  - No Anthropic-only background path remains.
  - `cargo check -p claurst-query` compiles.
- Stop / escalate condition from MPWO:
  - If the background block has a fundamentally different client-creation path from the foreground block, stop and investigate before editing.
- MPWO framing classification:
  - `TASK-M8-05` is still framed as a narrow background execute-path wiring change.
  - It is specifically a provider-selection / provider-routing fix for background agents.
  - It is not framed as a broader registry propagation change.
  - It is not framed as team-runner or worker-signature work.

## Current Post-M8-04 State
- `TASK-M8-04` is committed at `HEAD` and changed `src-rust/crates/query/src/agent_tool.rs`.
- In current `AgentTool::execute()`:
  - model resolves before provider materialization.
  - provider hint resolves via explicit provider override, model prefix, or parent provider inheritance.
  - `registry`, `identity`, and `target` are materialized before `QueryConfig` construction.
  - child `QueryConfig` already uses:
    - `model: target.model_id.clone()`
    - `provider_registry: Some(registry.clone())`
    - `model_registry: ctx.model_registry.clone()`
- Foreground path is already wired through the shared seam:
  - `foreground_ctx` is cloned from `ctx`
  - `foreground_ctx.config.provider = Some(target.provider_id.clone())`
  - foreground `run_query_loop()` receives `&foreground_ctx`
- Background path is not yet fully aligned with that seam:
  - it clones `client` into `client_bg`
  - it clones the original parent context into `ctx_bg`
  - it clones the provider-aware `QueryConfig` into `config_bg`
  - background `run_query_loop()` receives `&ctx_bg`, not the provider-mutated `foreground_ctx`
- `run_query_loop()` current registry-backed branch proves why that matters:
  - it only enters registry-backed dispatch when `config.provider_registry` is `Some(...)`
  - it resolves provider identity using `tool_ctx.config.provider.as_deref()`
  - it uses `config.model_registry.as_deref()`
  - it materializes the provider with `tool_ctx.config.provider_configs`
- Proven consequence:
  - background already carries registry/model registry through `config_bg`
  - background does not yet prove correct provider routing when the child provider differs from the parent provider and the model is bare
  - the foreground path is fixed because it mutates `ToolContext.config.provider`; the background path currently does not
- Important distinction:
  - there is no separate `AnthropicClient::new()` call inside the background branch itself anymore
  - the remaining seam issue is the captured `ToolContext`, not a second background-local client-construction block
- Out-of-scope paths confirmed separate:
  - foreground path is already closed by `TASK-M8-04`
  - `init_team_swarm_runner()` in the same file still hardcodes Anthropic, but MPWO assigns that to `TASK-M8-08`
  - `team_tool.rs` / `AgentRunFn` work belongs to `TASK-M8-06`
  - tests/helpers are not named by `TASK-M8-05`

## Dependency / Interface Shape Notes
- MPWO dependency chain is consistent with current repo reality:
  - `TASK-M8-01` introduced registry fields on `ToolContext`
  - `TASK-M8-03` introduced `AgentInput.provider`
  - `TASK-M8-04` consumed those seams for the foreground path
  - `TASK-M8-05` now consumes the same seam for background execution
- Proven:
  - `ToolContext` is `Clone`
  - `QueryConfig` is `Clone`
  - background closure already owns cloned `ToolContext`, `QueryConfig`, `client`, and `cost_tracker`
  - `run_query_loop()` derives provider identity from `tool_ctx.config.provider`, not from a provider field on `QueryConfig`
- Proven likely challenge:
  - simple background-path reuse of the foreground provider-aware context/config seam
  - not registry propagation into `QueryConfig` itself, because that is already present
  - not ownership / `Arc` / cloning shape, because the needed types already clone
  - not `init_team_swarm_runner()` or worker signature propagation
- Still uncertain:
  - whether the execution patch can be exactly one-line-local in spirit (`ctx_bg` capture change) or needs a tiny same-file variable reshuffle for borrow/move hygiene
  - no second-file change is currently proven necessary
- Hosted Ollama compatibility baseline preserved

## Likely Edit Surface
- Definitely in scope:
  - `src-rust/crates/query/src/agent_tool.rs`
  - specifically the background branch around current lines `416-462`
- Maybe in scope:
  - no second file is currently justified by repo reality
  - at most, a same-file local variable/capture adjustment if the compiler requires it
- Should remain untouched if the ticket stays tight:
  - `src-rust/crates/query/src/lib.rs`
  - `src-rust/crates/tools/src/lib.rs`
  - `src-rust/crates/tools/src/team_tool.rs`
  - `init_team_swarm_runner()` in `src-rust/crates/query/src/agent_tool.rs`
  - tests/helpers not explicitly named by MPWO
- Current repo reality suggests the execution patch should stay in `src-rust/crates/query/src/agent_tool.rs` only.
- Widening risks to avoid:
  - `TASK-M8-06` (`AgentRunFn` / `AgentRunParams`)
  - `TASK-M8-08` (`init_team_swarm_runner()` provider seam)
  - later worker-team propagation tickets

## Validation Command
- MPWO validation command: `cd src-rust && cargo check -p claurst-query`
- This command was not run during preflight.
- Reason for skipping:
  - this pass was required to remain read-only except for the report file
  - `cargo check` can write under `src-rust/target/`, which would alter the worktree beyond the allowed report artifact
- Baseline confidence source:
  - current `HEAD` commit inspection
  - direct source inspection of `agent_tool.rs`, `run_query_loop()`, and `ToolContext`
  - not a fresh compile rerun in this pass

## Drift Found
- Structural drift in the target code seam: `NO`
- Line-number drift: `YES`, but only line drift
- Authority/review-basis notes:
  - `docs/Current/MPWO_WORK_ORDER_PACK.md` is untracked in the worktree
  - `AGENTS.md` is present but git-ignored
  - extensive unrelated untracked files exist and should be called out in any later execution/review basis

## Blockers
- None for preflight.
- Execution note only: later implementation should keep the review basis explicit because the worktree contains substantial unrelated untracked noise.

## Verdict
- Ticket id: `TASK-M8-05`
- Verdict: `READY-WITH-NOTES`
- Structural drift exists: `NO`
- Ticket still looks narrow enough for a single tight implementation pass: `YES`
- Smallest plausible edit surface: reuse the already provider-mutated child context in the background `run_query_loop()` path so the background agent resolves provider identity against `target.provider_id`, while leaving `tokio::spawn`, cleanup, and later team-runner work untouched.
