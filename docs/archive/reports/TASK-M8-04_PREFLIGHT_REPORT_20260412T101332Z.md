# TASK-M8-04 Preflight Report

## Ticket
`TASK-M8-04`

## Timestamp UTC
`20260412T101332Z`

## Branch
`feature/provider-resolution-seam`

## Working Tree Summary
- `git diff --name-only`: empty.
- `git diff --cached --name-only`: empty.
- `git status --short`: 56 untracked entries; no staged changes; no unstaged tracked changes.
- Notable untracked noise affecting later review basis: `docs/Current/MPWO_WORK_ORDER_PACK.md`, `src-rust/target/`, `.codex`, and many existing report files under `docs/archive/reports/`.
- `TASK-M8-03` is already committed at `HEAD`: `f4dc962270c2f804ed09c071777efec75d4abb73` (`TASK-M8-03 add provider field to AgentTool input schema`).
- Baseline verdict for this pass: clean enough for read-only preflight, but not scope-clean for a later unstaged-diff review unless existing untracked noise is explicitly accounted for.

## Authority Reviewed
- Repo-local `AGENTS.md`.
- `docs/Current/MPWO_WORK_ORDER_PACK.md`.
- Current `HEAD` commit and current source state under `src-rust/`.

| Type | Verified Item | Location / Command | Result |
|---|---|---|---|
| Repo | Current branch | `git branch --show-current` | `feature/provider-resolution-seam` |
| Repo | Tracked worktree state | `git diff --name-only`, `git diff --cached --name-only` | No tracked diffs |
| Repo | Head commit | `git log --oneline --decorate -n 12`, `git rev-parse HEAD` | `f4dc962` is `TASK-M8-03` |
| Authority | Ticket section | `rg -n "TASK-M8-04" docs/Current/MPWO_WORK_ORDER_PACK.md` | Located at `978` |
| Authority | MPWO tracking status | `git ls-files --error-unmatch docs/Current/MPWO_WORK_ORDER_PACK.md` | Untracked authority file |
| M8-03 seam | `AgentInput.provider` and schema | `src-rust/crates/query/src/agent_tool.rs` | Present at `HEAD` |
| M8-01 seam | `ToolContext.provider_registry`, `model_registry` | `src-rust/crates/tools/src/lib.rs:216-234` | Present |
| Startup wiring | Root context/query registries populated | `src-rust/crates/cli/src/main.rs:725-741` | Present |
| Shared seam | `resolve_provider_identity()`, `materialize_provider()` | `src-rust/crates/query/src/provider_resolution.rs:101-214` | Present |
| Query loop | Registry-backed dispatch path | `src-rust/crates/query/src/lib.rs:874-1234` | Present; bypasses raw Anthropic request path when registry is set |
| Foreground target | Hardcoded env/client + child `QueryConfig` | `src-rust/crates/query/src/agent_tool.rs:236-364` | Still hardcoded; registries still `None` |
| Worker/team split | Injected runner remains separate | `src-rust/crates/query/src/agent_tool.rs:523-610`, `src-rust/crates/tools/src/team_tool.rs:47-85` | Separate path; later tickets |
| Validation | Ticket validation command | MPWO `TASK-M8-04` section | `cd src-rust && cargo check -p claurst-query` |

## Exact M8-04 Scope Confirmation
- **Objective verified from MPWO:** replace the foreground `AgentTool::execute()` hardcoded `ANTHROPIC_API_KEY` + `AnthropicClient::new()` path with the shared provider-resolution seam.
- **MPWO framing:** this is a narrow foreground execute-path wiring change with provider-selection/provider-routing behavior inside that path. It is not framed as a broader registry propagation ticket. The only propagation explicitly in scope is setting the child `QueryConfig` registry fields in the same foreground method.
- **Exact code targets verified against current `HEAD`:**
  - API key resolution and `AnthropicClient::new()`: `src-rust/crates/query/src/agent_tool.rs:236-255` (`MPWO` line references drifted from `229-248`; drift is line-number only).
  - Foreground model resolution: `src-rust/crates/query/src/agent_tool.rs:271-275`.
  - Foreground child `QueryConfig` construction: `src-rust/crates/query/src/agent_tool.rs:344-364`.
  - Foreground synchronous/background call sites consuming that config: `src-rust/crates/query/src/agent_tool.rs:369-456`.
- **Preconditions verified:**
  - `TASK-M8-01` complete in current repo reality: `ToolContext` has registry fields and CLI startup populates them.
  - `TASK-M8-03` complete at `HEAD`: `AgentInput` now has `provider: Option<String>` and `input_schema()` includes `"provider"`.
  - Milestone 7 seam exists: `resolve_provider_identity()` and `materialize_provider()` are implemented in `provider_resolution.rs`.
- **Required behavior verified from MPWO:**
  - Foreground path must resolve provider via `resolve_provider_identity()` and `materialize_provider()`.
  - Child `QueryConfig` must carry `provider_registry: Some(...)` and `model_registry: ctx.model_registry.clone()`.
  - Child `QueryConfig.model` must be updated from the resolved target rather than left as the pre-resolution string.
  - `run_query_loop()` signature must remain unchanged in this ticket.
- **Strict constraints verified from MPWO:**
  - Do not remove the `client` parameter from `run_query_loop()`.
  - Do not add fallback behavior.
  - Do not modify the background agent block; that is `TASK-M8-05`.
  - Do not modify `init_team_swarm_runner()`; that is `TASK-M8-08`.
  - Do not touch tool-list or system-prompt logic.
- **Definition of done verified from MPWO:** foreground path stops reading `ANTHROPIC_API_KEY` directly, uses the shared seam, passes registries into child `QueryConfig`, and `cargo check -p claurst-query` compiles.

## Current Post-M8-03 State
- `HEAD` is the `TASK-M8-03` commit; no later tracked commits are present on this branch tip.
- `AgentInput` now contains `provider: Option<String>` and the JSON schema exposes `"provider"` in `input_schema()`.
- The foreground sub-agent path in `AgentTool::execute()` still does all of the following directly:
  - reads `ANTHROPIC_API_KEY`,
  - constructs `AnthropicClient::new(ClientConfig { ... })`,
  - resolves only a model string,
  - builds a child `QueryConfig` with `provider_registry: None` and `model_registry: None`.
- The background sub-agent path does not create its own provider-aware config; it clones the foreground `client` and `query_config`. That confirms `TASK-M8-05` depends on `TASK-M8-04` output but remains a separate execution block.
- The team/worker path is still distinct and still hardcodes Anthropic inside `init_team_swarm_runner()`. That remains later-ticket work (`M8-08`, then `M8-09` after `M8-06`).
- Root session wiring from CLI startup is already present:
  - `query_config.provider_registry = Some(provider_registry.clone())`
  - `tool_ctx.provider_registry = Some(provider_registry.clone())`
  - `tool_ctx.model_registry = Some(model_registry.clone())`

Hosted Ollama compatibility baseline preserved

## Dependency / Interface Shape Notes
- **Proven:** `ToolContext` and `claurst_core::Config` are both `Clone`, so `agent_tool.rs` can make a local child context/config adjustment without widening into a broader ownership refactor.
- **Proven:** registry ownership shape is already compatible with a tight local patch:
  - `ctx.provider_registry` and `ctx.model_registry` are `Option<Arc<...>>`.
  - `QueryConfig.provider_registry` and `QueryConfig.model_registry` are also `Option<Arc<...>>`.
  - Local `.clone()` on the `Arc` is sufficient; no new ownership type is needed.
- **Proven:** `run_query_loop()` already has the shared registry-backed dispatch path at `src-rust/crates/query/src/lib.rs:874-1234`. When `config.provider_registry` is `Some(...)`, request dispatch goes through `resolve_provider_identity()` and `materialize_provider()` and does not fall through to `client.create_message_stream(...)` for that turn.
- **Proven:** the foreground `AgentInput.provider` seam added in `TASK-M8-03` is not yet consumed anywhere in `agent_tool.rs`; `TASK-M8-04` is the first ticket that can use it.
- **Proven:** the relevant dependency chain in both MPWO and current code is `M8-01` registry plumbing -> `M8-03` provider field seam -> `M8-04` foreground path wiring -> `M8-05` background reuse -> later worker/team propagation.
- **Most likely challenge:** provider resolution / routing behavior inside the foreground execute path, not `Arc` ownership, imports, or type exposure.
- **Still uncertain but important:** `run_query_loop()` resolves registry-backed provider identity from `tool_ctx.config.provider.as_deref()` plus `effective_model`, not from `AgentInput.provider` directly. Current repo reality suggests `agent_tool.rs` may need to preserve the resolved provider through a cloned child `ToolContext.config.provider` (or an equivalent local mechanism) so an explicit sub-agent provider override survives the nested call. This is a real interface note, but it does not yet prove a second-file change is required.
- **Not proven necessary:** edits to `src-rust/crates/query/src/lib.rs`, `src-rust/crates/query/src/provider_resolution.rs`, `src-rust/crates/tools/src/team_tool.rs`, or CLI startup code.

## Likely Edit Surface
| File / Area | Classification | Why |
|---|---|---|
| `src-rust/crates/query/src/agent_tool.rs` foreground `execute()` path | Definitely in scope | All MPWO target blocks are here; current hardcoded Anthropic path is here |
| `src-rust/crates/query/src/agent_tool.rs` child `QueryConfig` construction | Definitely in scope | Registries and resolved target model must be carried here |
| `src-rust/crates/query/src/agent_tool.rs` local child context/config handling | Maybe in scope, still same file | Needed only if explicit provider must be preserved into `run_query_loop()` via cloned context |
| `src-rust/crates/query/src/lib.rs` | Maybe in scope only as contingency | Only if the current nested-call interface cannot be satisfied from `agent_tool.rs`; preflight did not prove this is necessary |
| `src-rust/crates/query/src/agent_tool.rs` background block (`M8-05`) | Should remain untouched if ticket stays tight | Later ticket owns the background-specific block |
| `src-rust/crates/query/src/agent_tool.rs` `init_team_swarm_runner()` (`M8-08`) | Should remain untouched if ticket stays tight | Later team/worker ticket |
| `src-rust/crates/tools/src/team_tool.rs` (`M8-06`/`M8-09`) | Should remain untouched if ticket stays tight | Different milestone seam |
| `src-rust/crates/query/src/provider_resolution.rs` | Should remain untouched if ticket stays tight | Shared seam already exists; this ticket should consume it, not rewrite it |

- Current repo reality suggests the patch should stay in `src-rust/crates/query/src/agent_tool.rs` only.
- A second file is not presently required by the evidence gathered in this preflight.
- Widening risk to call out explicitly:
  - `M8-05`: touching the background block directly.
  - `M8-06`: changing `AgentRunFn` / worker signatures early.
  - `M8-08` and later: touching `init_team_swarm_runner()` or team/worker propagation.

## Validation Command
- Verified from MPWO: `cd src-rust && cargo check -p claurst-query`
- Not run during this preflight pass.
- Reason not run: the pass was kept read-only except for the required report artifact, and `cargo check` would mutate build artifacts under the already-untracked `src-rust/target/` tree.
- Baseline confidence for this preflight comes from current source inspection plus the fact that `HEAD` is the committed `TASK-M8-03` change, not from a fresh build in this pass.

## Drift Found
- **Target seam structural drift:** none found. The ticket’s target file and symbols still exist exactly where expected; only line numbers have drifted.
- **Repo-state drift / review-basis note:** present.
  - `docs/Current/MPWO_WORK_ORDER_PACK.md` is untracked even though it is the ticket authority for this pass.
  - The worktree contains 56 untracked entries, including `src-rust/target/` and many report artifacts.
- **Impact:** this does not block read-only preflight, but it should be made explicit before any future closure review that uses the unstaged diff as its basis.

## Blockers
- No code-level blocker was proven for starting `TASK-M8-04`.
- Review-basis note only: the current worktree is not clean because of existing untracked noise and an untracked MPWO authority file.

## Verdict
- **Verdict:** `READY-WITH-NOTES`
- `TASK-M8-04` still looks narrow enough for a single tight implementation pass.
- MPWO and current repo reality both point to a foreground execute-path provider-selection/provider-routing change, not a broader registry propagation change.
- Smallest plausible edit surface remains `src-rust/crates/query/src/agent_tool.rs` only.
- Structural drift exists only in repo-state/review-basis form, not in the code seam itself.
