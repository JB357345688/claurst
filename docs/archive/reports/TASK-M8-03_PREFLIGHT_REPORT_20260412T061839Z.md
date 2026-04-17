# TASK-M8-03 Preflight Report

**Ticket:** `TASK-M8-03`
**Timestamp UTC:** `2026-04-12T06:18:39Z`
**Branch:** `feature/provider-resolution-seam`

## Working Tree Summary

- `HEAD` is `fe060af` with subject `TASK-M8-02 wire root registries into startup ToolContext`; `TASK-M8-02` is already committed at `HEAD`.
- `git diff --name-only` is empty and `git diff --cached --name-only` is empty. There are no staged changes and no tracked unstaged changes.
- The worktree is not clean because of untracked noise: `.codex/`, 5 untracked files under `docs/Current/`, 49 existing report files under `docs/archive/reports/`, and about 25,101 untracked files under `src-rust/target/`.
- Baseline clean enough to begin `TASK-M8-03` preflight: yes.
- Review-basis note: a future execution pass should keep the active diff limited to `src-rust/crates/query/src/agent_tool.rs` plus the new report file, and should explicitly ignore the existing untracked noise.

## Authority Reviewed

- `/home/jordi/claurst/AGENTS.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md:66-110` for the M8 dependency graph and the standing hosted Ollama non-regression invariant.
- `docs/Current/MPWO_WORK_ORDER_PACK.md:917-974` for the exact `TASK-M8-03` instructions.
- Hosted Ollama compatibility baseline preserved.

## Verified Targets / Symbols / Commands

| Kind | Item | Verified Reality |
|---|---|---|
| Repo state | Branch | `feature/provider-resolution-seam` |
| Repo state | `HEAD` | `fe060af` = `TASK-M8-02 wire root registries into startup ToolContext` |
| Repo state | Tracked diffs | `git diff --name-only` empty; `git diff --cached --name-only` empty |
| Authority | Dependency edges | `M8-03 -> M8-04` and `M8-03 -> M8-05` at `docs/Current/MPWO_WORK_ORDER_PACK.md:77-82` |
| Authority | Ticket scope | `docs/Current/MPWO_WORK_ORDER_PACK.md:917-974` |
| Authority | Hosted Ollama invariant | `docs/Current/MPWO_WORK_ORDER_PACK.md:108-110` |
| Target file | `AgentInput` + `input_schema()` | `src-rust/crates/query/src/agent_tool.rs:130-218` |
| Current root seam | `ToolContext` registry fields | `src-rust/crates/tools/src/lib.rs:216-234` |
| Current root seam | Root startup populates registries | `src-rust/crates/cli/src/main.rs:696-741` |
| Current query seam | `QueryConfig` registry fields | `src-rust/crates/query/src/lib.rs:78-128` |
| Current foreground agent path | Hardcoded Anthropic remains | `src-rust/crates/query/src/agent_tool.rs:229-248` |
| Current foreground agent path | Sub-agent `QueryConfig` drops registries | `src-rust/crates/query/src/agent_tool.rs:337-357` |
| Current in-repo caller | AutoDream `AgentTool` input omits `provider` today | `src-rust/crates/query/src/lib.rs:1568-1574` |
| Current worker path | `AgentRunFn` signature still has no provider/model params | `src-rust/crates/tools/src/team_tool.rs:47-58` |
| Current worker path | Injected team runner still hardcodes Anthropic | `src-rust/crates/query/src/agent_tool.rs:523-584` |
| Validation | MPWO command | `cd src-rust && cargo check -p claurst-query` |

## Exact M8-03 Scope Confirmation

- **Objective:** allow callers to specify an explicit provider when spawning an agent through `AgentTool`.
- **MPWO framing:** this is a narrow field/wiring change. It is not a consumer propagation change, not a routing/materialization change, and not broader worker-fabric work.
- **Exact code targets:** only `src-rust/crates/query/src/agent_tool.rs`, specifically the private `AgentInput` struct and `AgentTool::input_schema()`.
- **Required behavior:** add `provider: Option<String>` immediately after `model`, with `#[serde(default)]`, and add a `"provider"` property to the JSON schema with the MPWO description.
- **Strict constraints:** do not add `allow_fallback`, `budget_usd`, or any D2 fields; do not modify `AgentTool::execute()`; do not change existing `AgentInput` fields.
- **Definition of done:** `AgentInput` has `provider: Option<String>`, the schema includes `"provider"`, and `cargo check -p claurst-query` succeeds.
- **Validation command:** `cd src-rust && cargo check -p claurst-query`.
- **Stop/escalate conditions:** MPWO lists none expected for this ticket.

## Current Post-M8-02 State

- The M8-01/M8-02 root seam is present at `HEAD`: `ToolContext` now carries `provider_registry` and `model_registry` fields in `src-rust/crates/tools/src/lib.rs:216-234`, and root startup populates both as `Some(...)` in `src-rust/crates/cli/src/main.rs:729-741`.
- `QueryConfig` already exposes `provider_registry` and `model_registry` fields in `src-rust/crates/query/src/lib.rs:117-128`.
- `AgentInput` currently contains `description`, `prompt`, `tools`, `system_prompt`, `max_turns`, `model`, `isolation`, and `run_in_background`; there is no `provider` field in `src-rust/crates/query/src/agent_tool.rs:130-155`.
- `AgentTool::input_schema()` currently exposes the same properties and does not publish a `"provider"` property in `src-rust/crates/query/src/agent_tool.rs:175-218`.
- The foreground `AgentTool::execute()` path still resolves `ANTHROPIC_API_KEY` and constructs `AnthropicClient` directly in `src-rust/crates/query/src/agent_tool.rs:229-248`.
- The foreground sub-agent `QueryConfig` currently sets `provider_registry: None` and `model_registry: None` in `src-rust/crates/query/src/agent_tool.rs:337-357`.
- The in-repo AutoDream caller builds `AgentTool` JSON without a `provider` field in `src-rust/crates/query/src/lib.rs:1568-1574`; adding `#[serde(default)] provider: Option<String>` would keep this caller valid without any companion edit.
- The separate team/worker path remains on the later-ticket seam: `AgentRunFn` still has no provider/model parameters in `src-rust/crates/tools/src/team_tool.rs:47-58`, and `init_team_swarm_runner()` still hardcodes Anthropic and builds `QueryConfig` from defaulted fields in `src-rust/crates/query/src/agent_tool.rs:523-584`.
- Test/helper construction sites still initialize `ToolContext` with `provider_registry: None` and `model_registry: None` in `src-rust/crates/tools/src/lib.rs:557-588`. These are unrelated to `TASK-M8-03` and should remain untouched.

## Dependency / Interface Shape Notes

- **MPWO dependency shape:** `TASK-M8-03` feeds `TASK-M8-04` and `TASK-M8-05`, but has no prerequisite M8 ticket. Current repo reality matches that: the field to add is private and local to `agent_tool.rs`.
- **Current interface shape:** `AgentInput` is a private deserialize-only struct, and `input_schema()` is manually authored JSON. There is no public type-export or cross-crate interface update required for this ticket.
- **Likely challenge classification:** simple wiring. There is no construction-order issue, no `Arc` or clone ownership issue, no import exposure issue, and no routing/materialization change required for `TASK-M8-03`.
- **Proven non-goal:** `provider` will be inert until later tickets because `AgentTool::execute()` currently ignores it by design; that matches MPWO's instruction not to touch `execute()` until `TASK-M8-04`.
- **Proven widening risk:** touching `AgentTool::execute()`, `QueryConfig`, `ToolContext`, registry propagation, the team runner signature, or the hardcoded Anthropic paths would spill into `TASK-M8-04`, `TASK-M8-05`, `TASK-M8-06`, or `TASK-M8-08`.
- **Still uncertain:** only fresh compile status. I did not rerun the validation command during this read-only preflight.

## Likely Edit Surface

| Classification | File | Why |
|---|---|---|
| Definitely in scope | `src-rust/crates/query/src/agent_tool.rs` | Add one `Option<String>` field to `AgentInput` and one `"provider"` property to `input_schema()` |
| Maybe in scope | None proven | Current repo reality does not show any second file that must change for `TASK-M8-03` |
| Should remain untouched if the ticket stays tight | `src-rust/crates/query/src/lib.rs` | Existing JSON call sites can omit `provider`; propagation belongs to later tickets |
| Should remain untouched if the ticket stays tight | `src-rust/crates/query/src/agent_tool.rs` outside the struct/schema sites | `execute()` and the injected runner are reserved for later M8 tickets |
| Should remain untouched if the ticket stays tight | `src-rust/crates/tools/src/team_tool.rs` | Worker/team runner signature changes belong to `TASK-M8-06` and later |
| Should remain untouched if the ticket stays tight | `src-rust/crates/tools/src/lib.rs` and `src-rust/crates/cli/src/main.rs` | Root registry fields/population were handled by `TASK-M8-01` and `TASK-M8-02` |

- Current repo reality is not broader than MPWO. If anything, it is slightly narrower: no proven consumer call site needs to be updated because omitted `provider` will deserialize as `None`.

## Validation Command

- MPWO validation command: `cd src-rust && cargo check -p claurst-query`.
- I did **not** run the command during this preflight. Reason: the pass was explicitly read-only with the worktree to remain unchanged except for this report, and a cargo build would further mutate the already-large untracked `src-rust/target/` tree.
- This preflight therefore relies on current source inspection and the committed `HEAD` baseline, not on fresh compile evidence.

## Drift Found

- **Structural drift:** none found in the `TASK-M8-03` seam. The exact target file and symbol locations still match MPWO.
- **Repo-state drift / hygiene notes:** the authority files under `docs/Current/` are present but untracked in git, and the repository has substantial untracked noise under `docs/archive/reports/` and `src-rust/target/`. This affects review hygiene, not ticket scope.
- **Ticket narrowness:** `TASK-M8-03` still looks narrow enough for a single tight implementation pass.

## Blockers

- None for preflight.
- Non-blocking note: a later execution pass must keep patch hygiene explicit because the repo is status-noisy even though tracked diffs are clean.

## Verdict

- **Verdict:** `READY-WITH-NOTES`
- **Structural drift exists:** `no`
- **Ready for execution:** `yes`, provided the implementation stays limited to the struct/schema edits in `src-rust/crates/query/src/agent_tool.rs`
- **Still narrow enough for one tight pass:** `yes`
- **Why not plain READY:** the tracked baseline is clean, but the repo contains substantial untracked noise that should be called out in the review basis before closure.
