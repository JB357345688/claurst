# TASK-M9-06 Preflight Report

## Ticket
- `TASK-M9-06 — Worker missing registry -> hard error test`

## Timestamp UTC
- `2026-04-13T08:46:27Z`

## Branch
- Expected: `feature/provider-resolution-seam`
- Actual: `feature/provider-resolution-seam`

## Verdict
- `READY-WITH-NOTES`

## Repo State Summary
- `git branch --show-current` returned `feature/provider-resolution-seam`.
- `git diff --name-only` returned no tracked unstaged file changes.
- `git diff --cached --name-only` returned no staged file changes.
- `git log --oneline --decorate -n 20` shows `TASK-M8-11` (`b5b6dd4`) in current history, followed by `TASK-M9-02`, `TASK-M9-03`, and `TASK-M9-04`; `HEAD` is `2f1f169` (`TASK-M9-04 prove agent inherits parent provider on openai dispatch`).
- The worktree has substantial untracked noise, including `.codex`, `docs/Current/`, many `docs/archive/reports/*.md`, and `src-rust/target/`.
- `AGENTS.md` and `docs/Current/MPWO_WORK_ORDER_PACK.md` are present on disk but currently untracked in git.

## Authority Reviewed
- Repo-local `AGENTS.md` present and reviewed as controlling repo authority.
- `docs/Current/MPWO_WORK_ORDER_PACK.md` present and reviewed as controlling ticket authority.
- `MPWO` section reviewed for `TASK-M9-06` at `docs/Current/MPWO_WORK_ORDER_PACK.md:1641`.

## Dependency Baseline Confirmed
- `TASK-M8-11` is present in branch history (`b5b6dd4`) and is compatible with starting `M9-06`.
- `TASK-M9-02`, `TASK-M9-03`, and `TASK-M9-04` are present in branch history and do not need reopening.
- User-provided accepted baseline states `TASK-M9-01` and `TASK-M9-05` are complete; nothing in the inspected code area suggests either ticket must be reopened.
- The standing hosted-Ollama compatibility baseline remains a preserved background invariant only. This ticket is confined to `agent_tool.rs` local tests and does not require reopening or revalidating hosted-Ollama behavior.

## Exact M9-06 Contract
- Verify that `ToolContext` with `provider_registry: None` in a worker path produces a hard error.
- Scope file from authority: `src-rust/crates/query/src/agent_tool.rs` local `#[cfg(test)]` only.
- Required later execution shape:
  1. Construct `ToolContext` with `provider_registry: None`.
  2. Call `AgentTool::execute()`.
  3. Assert error result.
- Explicit prohibition: do not allow fallback to Anthropic.
- Validation target later: test passes.

## Verified Files / Symbols / Commands
- Files verified:
  - `AGENTS.md`
  - `docs/Current/MPWO_WORK_ORDER_PACK.md`
  - `src-rust/crates/query/src/agent_tool.rs`
  - `src-rust/crates/tools/src/lib.rs`
  - `src-rust/crates/core/src/lib.rs`
  - `src-rust/crates/query/src/lib.rs`
  - `src-rust/crates/query/src/provider_resolution.rs`
- Symbols / behaviors verified:
  - `AgentTool::execute(...)`
  - `ToolContext.provider_registry`
  - local `#[cfg(test)]` module in `agent_tool.rs`
  - worker-path provider registry guard
  - `ToolResult::error(...)` error shape
  - root query-loop registry-vs-legacy split for contrast only
- Commands run:
  - `git branch --show-current`
  - `git status --short --branch`
  - `git diff --name-only`
  - `git diff --cached --name-only`
  - `git log --oneline --decorate -n 20`
  - targeted `rg`, `sed`, and `nl -ba` reads against the files above

## Current Code Reality
- `AgentTool::execute(...)` currently computes provider hint inputs first, then checks `ctx.provider_registry` before any provider resolution or client materialization work:
  - `src-rust/crates/query/src/agent_tool.rs:272-280`
- If `ctx.provider_registry` is `None`, it returns:
  - `ToolResult::error("Cannot spawn sub-agent: provider_registry not available in ToolContext")`
- `ToolResult::error(...)` sets:
  - `is_error = true`
  - `content = <provided string>`
  - `metadata = None`
  - basis: `src-rust/crates/tools/src/lib.rs:99-122`
- The worker-path hard error occurs before:
  - `resolve_provider_identity(...)` at `src-rust/crates/query/src/agent_tool.rs:282-286`
  - `materialize_provider(...)` at `src-rust/crates/query/src/agent_tool.rs:288-290`
  - any Anthropic credential/client branch at `src-rust/crates/query/src/agent_tool.rs:293-305`
- For root-path contrast only, `run_query_loop` in `src-rust/crates/query/src/lib.rs:871-873` explicitly documents that registry-backed failures do not fall through to the raw Anthropic path below. `M9-07` remains the later ticket for the root legacy path when `provider_registry` is `None`.

## Existing Missing-Registry Test Audit
- Relevant current test name:
  - `agent_tool_errors_when_provider_registry_missing` in `src-rust/crates/query/src/agent_tool.rs:932-952`
- What it covers:
  - Constructs `ToolContext` via `make_tool_context(None, None)`, so `provider_registry: None` is set in the worker-path test harness.
  - Calls `AgentTool.execute(...)` through `run_agent_tool(...)`.
  - Asserts `result.is_error`.
  - Asserts `result.content` contains `provider_registry not available in ToolContext`.
- Whether exact `M9-06`-equivalent coverage already exists:
  - Yes. This test already matches the ticket's required setup and required `AgentTool::execute()` error assertion path.
- Weak / indirect / overlapping tests:
  - `agent_explicit_provider_routes_to_openai_provider` and `agent_parent_inherits_provider_openai_dispatch` are adjacent worker-path routing tests, but they are positive-path OpenAI dispatch tests, not missing-registry tests.
  - No separate local test explicitly injects Anthropics credentials to prove the non-fallback guarantee under credential-present conditions.
- Whether any current test accidentally proves only generic error behavior:
  - No. `agent_tool_errors_when_provider_registry_missing` is not merely generic-error coverage; it checks the specific missing-registry message.
- Coverage-strength note:
  - The current test directly proves `hard error result` plus `missing-registry cause`.
  - The `no Anthropic fallback` requirement is proved by the current guard ordering plus the observed missing-registry error result, not by a dedicated credential-present fallback-negation test.

## M9-06 Coverage Matrix

| Required case | Existing test name(s) | Status | Basis for classification | Likely assertion needed if follow-up execution is required |
| --- | --- | --- | --- | --- |
| `ToolContext` with `provider_registry: None` in worker path -> hard error result | `agent_tool_errors_when_provider_registry_missing` | `COVERED` | Test uses `make_tool_context(None, None)`, calls `AgentTool::execute()`, asserts `result.is_error`, and asserts the missing-registry error text. | None required; existing assertions already satisfy the ticket contract. |
| No fallback to Anthropic when `provider_registry` is `None` in worker path | `agent_tool_errors_when_provider_registry_missing` plus code ordering in `AgentTool::execute()` | `COVERED` | `AgentTool::execute()` returns at `agent_tool.rs:272-280`, before provider resolution, provider materialization, or Anthropic credential/client code at `agent_tool.rs:293-305`. The existing test observes the missing-registry hard error, which is incompatible with any successful or masked Anthropic fallback path. | Optional tightening only: assert exact error string or isolate auth env, but no production-path edit is indicated by current evidence. |

## Likely Smallest Edit Surface For Execution
- Smallest correct edit surface remains `src-rust/crates/query/src/agent_tool.rs` local `#[cfg(test)]` module only, per the ticket authority.
- Based on current read-only evidence, no production-code changes appear necessary.
- Based on current read-only evidence, no test edit appears necessary either; this looks like an audit-only / validation-only ticket unless a stricter explicit anti-fallback assertion is required by reviewer preference.

## Validation Readiness
- Validation was not run in this preflight session, per instruction.
- The narrowest stable validation filter suggested by current test names is:
  - `agent_tool_errors_when_provider_registry_missing`
- A broader but still local fallback filter would be:
  - `agent_tool`
- `missing_registry` is also plausible as a substring filter, but the exact current test-name substring is `provider_registry_missing`.

## Drift Found
- No structural drift found in active ticket scope:
  - file path `src-rust/crates/query/src/agent_tool.rs` exists
  - `AgentTool::execute(...)` exists
  - local `#[cfg(test)]` module exists
  - worker-path missing-registry hard-error behavior exists
- Minor repo-state notes only:
  - `AGENTS.md` and `docs/Current/MPWO_WORK_ORDER_PACK.md` are present but untracked
  - repo contains extensive unrelated untracked files and reports
- Ticket wording vs current code reality:
  - Current repo reality already contains the exact local worker-path missing-registry hard-error test, so `M9-06` appears effectively audit-only / validation-only as of this preflight.

## Blockers
- None for preflight.
- No blocker found in worker-path behavior, assertion shape, or production/test seam.

## Notes
- This session stayed read-only except for creating this required preflight report.
- No source files were edited.
- No tests were added or modified.
- No validation command was run.
- No staging, commit, cleanup, or scope expansion was performed.
