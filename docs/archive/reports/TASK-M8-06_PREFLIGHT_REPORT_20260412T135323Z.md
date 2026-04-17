# TASK-M8-06 Preflight Report

## Ticket
`TASK-M8-06`

## Timestamp UTC
`20260412T135323Z`

## Branch
`feature/provider-resolution-seam`

## Working Tree Summary
- `git branch --show-current` verified branch `feature/provider-resolution-seam`.
- `git diff --name-only` and `git diff --cached --name-only` were empty: no unstaged tracked changes and no staged changes.
- `git status --short` showed substantial untracked noise: `.codex`, `docs/Current/`, `docs/Orchestrator_planning/`, `docs/archive/provider_orchestrator/`, many untracked report files, and `src-rust/target/`.
- Baseline clean enough to begin read-only `TASK-M8-06` preflight: `yes`.
- Baseline clean enough for a scope-clean execution/closure review without an explicit review basis: `no`; untracked authority/docs/report noise must be called out during execution.
- `TASK-M8-05` is already committed: `5d246b233604195f4d1a0a7780fa15b54aedce2b TASK-M8-05 wire background provider resolution through shared seam`.
- `TASK-M8-05` is not `HEAD`; it is an ancestor of `HEAD`, and there is exactly one later commit on top of it:
  `ced600545fb3517c9995f022d7772ec5fe5f514d Harden provider-aware compaction for post-M8-05 registry-backed runs`.
- That single later commit should be treated as fixed branch baseline before `TASK-M8-06` begins.
- Repo-local `AGENTS.md` exists and is gitignored by `.gitignore`.
- `docs/Current/MPWO_WORK_ORDER_PACK.md` exists and is untracked.

## Authority Reviewed
- `/home/jordi/claurst/AGENTS.md`
- `/home/jordi/claurst/docs/Current/MPWO_WORK_ORDER_PACK.md`

| Verified item | File / command | Current reality |
|---|---|---|
| Branch | `git branch --show-current` | `feature/provider-resolution-seam` |
| Worktree tracked state | `git diff --name-only`; `git diff --cached --name-only` | both empty |
| Worktree noise | `git status --short` | many untracked docs/reports plus `src-rust/target/` |
| Active ticket authority | `rg -n "TASK-M8-06" docs/Current/MPWO_WORK_ORDER_PACK.md` | section present at line `1126` |
| `AgentRunFn` seam | [team_tool.rs](/home/jordi/claurst/src-rust/crates/tools/src/team_tool.rs:47) | still 6 positional params |
| `run_agent()` seam | [team_tool.rs](/home/jordi/claurst/src-rust/crates/tools/src/team_tool.rs:76) | still accepts 6 positional params |
| `run_agent()` call sites | `rg -n "run_agent\\(" src-rust/crates/tools/src/team_tool.rs` | exactly 1 real call site |
| Team runner producer | [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:570) | still 6-arg closure; hardcoded Anthropic/env path |
| Foreground provider seam baseline | [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:270) | uses `resolve_provider_identity()` + `materialize_provider()` |
| Background provider seam baseline | [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:416) | clones resolved `query_config` / `foreground_ctx` into `tokio::spawn` |
| Public export shape | [lib.rs](/home/jordi/claurst/src-rust/crates/tools/src/lib.rs:88) | re-exports `AgentRunFn`, not `AgentRunParams` |
| Post-M8-05 hardening baseline | [lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:874), [compact.rs](/home/jordi/claurst/src-rust/crates/query/src/compact.rs:1174) | registry-backed dispatch and provider-aware compaction/context-collapse already present |
| Hosted Ollama baseline | [provider_resolution.rs](/home/jordi/claurst/src-rust/crates/query/src/provider_resolution.rs:157) | `materialize_provider()` and Ollama API-base normalization already present |

## Exact M8-06 Scope Confirmation
- **Objective:** replace the 6-positional-parameter `AgentRunFn` alias with a struct-based `AgentRunParams` parameter.
- **MPWO framing:** this is an `AgentRunFn` / `AgentRunParams` interface change with one local call-site migration in `team_tool.rs`. It is **not** framed as broader team-runner wiring.
- **Exact code targets named by MPWO:** `AgentRunFn`, `AGENT_RUNNER`, `register_agent_runner()`, `run_agent()`, and the `run_agent()` call inside `TeamCreateTool::execute()`, all in `src-rust/crates/tools/src/team_tool.rs`.
- **Preconditions / dependencies:** MPWO says none within M8; `TASK-M8-08` and `TASK-M8-09` depend on this change.
- **Required behavior:** add `AgentRunParams` with 8 fields (`description`, `prompt`, `tools`, `system_prompt`, `max_turns`, `ctx`, `provider_override`, `model_override`); change `AgentRunFn` to `Fn(AgentRunParams) -> ...`; change `run_agent()` to accept `AgentRunParams`; change the existing `TeamCreateTool::execute()` call site to pass an `AgentRunParams` with `provider_override: None` and `model_override: None`.
- **Strict constraints:** do not add M11/D2 fields; do not modify `register_agent_runner()` or `AGENT_RUNNER` beyond the type change; do not modify `init_team_swarm_runner()`; stop if there are more local `run_agent()` call sites than expected or if the boxed future type needs adaptation.
- **Definition of done:** `AgentRunParams` exists with all 8 fields; `AgentRunFn` uses it; `run_agent()` uses it; the one `TeamCreateTool::execute()` call site passes it.
- **Validation command from MPWO:** `cd src-rust && cargo check -p claurst-tools  # may fail until M8-08`
- **Stop / escalate conditions from MPWO:** update all `run_agent()` call sites if more exist; adapt boxed future return shape only if the signature change requires it.
- **Compaction / context-collapse parity note:** MPWO says nothing about compaction or context-collapse parity in `TASK-M8-06`.

## Current Post-M8-05 State
- [team_tool.rs](/home/jordi/claurst/src-rust/crates/tools/src/team_tool.rs:47) still defines `AgentRunFn` as a 6-positional-argument callback.
- [team_tool.rs](/home/jordi/claurst/src-rust/crates/tools/src/team_tool.rs:76) still defines `run_agent()` with the same 6 positional parameters.
- [team_tool.rs](/home/jordi/claurst/src-rust/crates/tools/src/team_tool.rs:412) is the only real `run_agent()` call site in this file.
- [team_tool.rs](/home/jordi/claurst/src-rust/crates/tools/src/team_tool.rs:171) shows `AgentSpec` still contains only `name`, `role`, `tools`, and `task`; it does not yet contain `provider` or `model`. That keeps the M8-06 call-site placeholders (`None` overrides) aligned with MPWO.
- [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:270) and [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:285) confirm the already-closed foreground provider path: `resolve_provider_identity()` and `materialize_provider()` run before building the sub-agent config.
- [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:405) and [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:408) confirm the current foreground `QueryConfig` already carries `provider_registry` and `model_registry`.
- [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:416) through [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:447) confirm the already-closed background seam: the spawned path clones the resolved `query_config` and `foreground_ctx` into `run_query_loop()`.
- [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:570) through [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:606) confirm `init_team_swarm_runner()` still produces the old 6-argument closure and still contains the hardcoded `ANTHROPIC_API_KEY` / `AnthropicClient::new()` logic. That remains `TASK-M8-08` work, not `TASK-M8-06`.
- Later post-M8-05 hardening is present at `HEAD` and should be treated as baseline, not ticket scope:
  - [lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:874) adds registry-backed provider dispatch in the shared query loop.
  - [lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:1134) routes provider-aware reactive compaction/context-collapse through provider-specific options.
  - [compact.rs](/home/jordi/claurst/src-rust/crates/query/src/compact.rs:1174) and [compact.rs](/home/jordi/claurst/src-rust/crates/query/src/compact.rs:1368) add provider-aware `reactive_compact_with_provider()` and `context_collapse_with_provider()`.
- Already-baseline behavior that should **not** be misattributed to `TASK-M8-06`:
  - foreground provider/model resolution in `AgentTool`
  - background provider/model resolution in `AgentTool`
  - registry-backed query-loop dispatch
  - provider-aware compaction/context-collapse hardening
- Hosted Ollama compatibility baseline preserved.

## Dependency / Interface Shape Notes
- **MPWO dependency shape:** `TASK-M8-06` has no within-M8 prerequisite; it is the interface seam that `TASK-M8-08` and later `TASK-M8-09` consume.
- **Current repo dependency shape:** earlier M8 work already established registries in `ToolContext` and provider/model resolution in `AgentTool`; `TASK-M8-06` is the missing transport interface between `TeamCreateTool` and the later runner-producer update.
- **Proven now:**
  - no `AgentRunParams` type exists anywhere in `src-rust`
  - `run_agent()` has exactly one local call site in `team_tool.rs`
  - `init_team_swarm_runner()` is the only current producer of `claurst_tools::AgentRunFn`
  - the provider/model seam from earlier M8 tickets is already baseline in `AgentTool`, not in `TeamCreateTool`
- **Likely challenge for M8-06 itself:** a local callback-signature migration in `team_tool.rs`, plus updating the local call site to package owned values into a struct.
- **Likely challenge deferred to M8-08:** the producer closure/callback signature change in `init_team_swarm_runner()`, not the `run_query_loop` implementation itself.
- **Ownership / async / Arc shape:** no broad async-coupling drift is proven. The existing seam already passes owned `String`s and `Arc<ToolContext>`; wrapping them into `AgentRunParams` should remain a narrow transport refactor.
- **Import / type exposure note:** `claurst_tools` currently re-exports `AgentRunFn` but not `AgentRunParams`. Because `pub mod team_tool;` already exists, `claurst_tools::team_tool::AgentRunParams` would be publicly reachable once added. It is still uncertain whether `TASK-M8-06` should also update `src-rust/crates/tools/src/lib.rs` to make the top-level `claurst_tools::AgentRunParams` path available immediately.
- **Post-M8-05 hardening impact on edit surface:** none proven. The hardening changed shared query-loop and compaction internals, not the `team_tool.rs` worker-signature seam. `TASK-M8-06` remains orthogonal to that hardening.

## Likely Edit Surface
- **Definitely in scope:** `src-rust/crates/tools/src/team_tool.rs` for `AgentRunParams`, `AgentRunFn`, `run_agent()`, and the single `run_agent()` call site in `TeamCreateTool::execute()`.
- **Maybe in scope:** `src-rust/crates/tools/src/lib.rs` only if the execution pass decides `AgentRunParams` must be re-exported at crate root to support the later `claurst_tools::AgentRunParams` path shown in MPWO examples.
- **Should remain untouched if the ticket stays tight:** `src-rust/crates/query/src/agent_tool.rs` including `init_team_swarm_runner()` (`TASK-M8-08`).
- **Should remain untouched if the ticket stays tight:** `src-rust/crates/query/src/lib.rs`.
- **Should remain untouched if the ticket stays tight:** `src-rust/crates/query/src/compact.rs`.
- **Should remain untouched if the ticket stays tight:** `src-rust/crates/query/src/provider_resolution.rs`.
- **Should remain untouched if the ticket stays tight:** tests/helpers unrelated to the `team_tool.rs` interface seam.
- **Tightness assessment:** current repo reality still supports a single tight implementation pass.
- **Stay-in-one-file or likely second file?** The smallest plausible execution patch still looks like `src-rust/crates/tools/src/team_tool.rs` only. A second file is **not proven required** today, but `src-rust/crates/tools/src/lib.rs` is a credible maybe-scope follow-on if top-level re-export ergonomics are required immediately.
- **Widening risks to avoid:**
  - `TASK-M8-08` / `init_team_swarm_runner()`
  - `TASK-M8-09` per-agent provider/model propagation
  - shared query-loop dispatch
  - compaction / context-collapse layers
- Any later post-M8-05 hardening baseline should remain untouched during `TASK-M8-06`.

## Validation Command
- MPWO validation command: `cd src-rust && cargo check -p claurst-tools`
- MPWO note: this may fail until `TASK-M8-08` updates the producer closure.
- Validation was **not run** during this preflight.
- Reason skipped: the user requested worktree preservation except for the report file, and `cargo check` would mutate `src-rust/target/`. This preflight therefore relies on current source inspection, commit history, and current seam verification rather than a fresh build.

## Drift Found
- **Structural drift:** `none found`.
- **Line drift:** minor/expected only. The MPWO target seam still maps directly to the current source locations.
- **Repo-state drift relevant to review basis:** local authority files are not git-tracked baseline (`AGENTS.md` is gitignored; `MPWO_WORK_ORDER_PACK.md` is untracked), and the worktree contains substantial untracked noise. This does not invalidate the ticket seam, but it must be made explicit during execution/review.

## Blockers
- No blocker prevents a narrow `TASK-M8-06` execution pass.
- Note: execution should declare the review basis explicitly because of the untracked worktree noise.
- Note: if exact top-level `claurst_tools::AgentRunParams` exposure is required immediately, `src-rust/crates/tools/src/lib.rs` may need to be included intentionally rather than discovered mid-pass.

## Verdict
- **Ticket id:** `TASK-M8-06`
- **Verdict:** `READY-WITH-NOTES`
- **Verified files/symbols/commands:** listed above in the verification table.
- **Structural drift exists:** `no`
- **Ticket still narrow enough for one tight implementation pass:** `yes`
- **Later post-M8-05 hardening detected:** `yes`; one later commit (`ced6005`) is already on branch and should be treated as fixed branch baseline outside `TASK-M8-06` scope.
- **Hosted Ollama compatibility baseline preserved**
