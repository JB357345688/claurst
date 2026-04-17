# TASK-M9-07 Preflight Report

## Ticket
- `TASK-M9-07 — Root missing registry → legacy path test`

## Timestamp UTC
- `2026-04-13T09:36:41Z`

## Branch
- Expected: `feature/provider-resolution-seam`
- Actual: `feature/provider-resolution-seam`

## Verdict
- `READY-WITH-NOTES`

## Repo State Summary
- `git branch --show-current` matched the expected branch.
- `git diff --name-only` returned no tracked unstaged file diffs.
- `git diff --cached --name-only` returned no staged file diffs.
- `git status --short --branch` showed substantial untracked background noise only, primarily under `docs/` report paths plus `src-rust/target/`.
- `git log --oneline --decorate -n 20` shows `M8-11` and source-changing `M9-02` through `M9-04` on the current branch. This is compatible with the stated baseline that `M9-05` and `M9-06` completed without source change.

## Authority Reviewed
- Repo-local `AGENTS.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md`
- `MPWO` entry for `TASK-M9-07` at `docs/Current/MPWO_WORK_ORDER_PACK.md:1656-1666`

## Dependency Baseline Confirmed
- `M8-11` is present in branch history as commit `b5b6dd4`.
- `M9-01` through `M9-06` were treated as accepted baseline per prompt and not reopened.
- Branch reality does not contradict starting `M9-07` after accepted `M8-11`.
- `M9-05` and `M9-06` are not visible as source-changing commits in the last 20 commits, which is consistent with the prompt’s statement that they completed without source change.
- Hosted Ollama compatibility baseline preserved. No read-only evidence indicates that `M9-07` would need to reopen or alter that background invariant.

## Exact M9-07 Contract
- Verify that `QueryConfig` with `provider_registry: None` uses the `client: &AnthropicClient` parameter in the root query path.
- Verify the root path, not the worker `AgentTool` path already covered by `M9-06`.
- Do not remove the legacy path.
- Do not change production code.
- Later validation target: the test passes.

## Verified Files / Symbols / Commands
- Commands:
  - `git branch --show-current`
  - `git status --short --branch`
  - `git diff --name-only`
  - `git diff --cached --name-only`
  - `git log --oneline --decorate -n 20`
- Authority files:
  - `AGENTS.md`
  - `docs/Current/MPWO_WORK_ORDER_PACK.md`
- Active ticket code surfaces:
  - `src-rust/crates/query/src/lib.rs`
  - `src-rust/crates/query/src/agent_tool.rs`
  - `src-rust/crates/query/src/provider_resolution.rs`
  - `src-rust/crates/tools/src/lib.rs`
  - `src-rust/crates/api/src/lib.rs`
- Verified symbols / branch points:
  - `QueryConfig::provider_registry`
  - `run_query_loop(...)`
  - `provider_resolution::resolve_provider_identity(...)`
  - `provider_resolution::materialize_provider(...)`
  - `AnthropicClient::create_message_stream(...)`
  - `agent_tool_errors_when_provider_registry_missing()`
  - `p10_no_provider_without_model_registry_defaults()`

## Current Code Reality
- `QueryConfig` exposes `provider_registry: Option<Arc<ProviderRegistry>>`, and its default is `None` at `src-rust/crates/query/src/lib.rs:117-121` and `:131-153`.
- The root query path is `run_query_loop(...)` at `src-rust/crates/query/src/lib.rs:675`.
- The root branch between registry-backed routing and legacy client routing is explicit:
  - `if let Some(ref registry) = config.provider_registry { ... }` at `src-rust/crates/query/src/lib.rs:874`
  - Inside that `Some(...)` branch, the code performs both:
    - `resolve_provider_identity(...)` at `:875-879`
    - `materialize_provider(...)` at `:887-890`
  - On either seam failure, it returns immediately and does not fall through at `:881-900`.
- The legacy root path is the fallthrough after that `if let Some(...)` block:
  - direct call to `client.create_message_stream(request, handler).await` at `src-rust/crates/query/src/lib.rs:1291`
- Therefore the branch to the legacy path occurs before any registry-backed resolution or materialization logic when `provider_registry` is `None`.
- `AgentTool` is intentionally different:
  - worker path requires a registry and hard-errors if missing at `src-rust/crates/query/src/agent_tool.rs:272-278`
  - this is the `M9-06` behavior and is not the root-path behavior.
- `src-rust/crates/query/tests/` does not exist in current repo reality. The hinted alternate authority surface is absent, so the likely execution surface is `src-rust/crates/query/src/lib.rs #[cfg(test)]`.

## Existing Legacy-Path Test Audit
- `src-rust/crates/query/src/lib.rs #[cfg(test)]`
  - Current test names:
    - `test_system_prompt_default_when_empty`
    - `test_system_prompt_with_custom`
    - `test_system_prompt_with_append`
    - `test_system_prompt_append_only`
    - `test_system_prompt_with_custom_output_style_prompt`
    - `test_query_config_clone`
    - `test_query_outcome_debug`
    - `test_build_provider_options_for_google_gemini_3`
    - `test_build_provider_options_for_openrouter_gpt5`
    - `test_build_provider_options_for_bedrock_anthropic`
  - Coverage assessment:
    - none of these call `run_query_loop(...)`
    - none assert root-path behavior under `provider_registry: None`
    - none prove `client: &AnthropicClient` is used
  - Exact `M9-07`-equivalent coverage already exists: `no`
- `src-rust/crates/query/src/provider_resolution.rs`
  - Relevant overlapping test:
    - `p10_no_provider_without_model_registry_defaults()`
  - What it covers:
    - proves the resolution seam defaults provider identity to `anthropic` when there is no explicit provider and no model registry
  - What it does not cover:
    - does not call `run_query_loop(...)`
    - does not exercise `provider_registry: None`
    - does not prove root-path dispatch used `client: &AnthropicClient`
    - does not prove absence of registry-backed routing in the root path
  - Classification:
    - overlapping but indirect and weaker than `M9-07`
- `src-rust/crates/query/src/agent_tool.rs`
  - Relevant overlapping tests:
    - `agent_tool_errors_when_provider_registry_missing()`
    - `agent_explicit_provider_routes_to_openai_provider()`
    - `agent_parent_inherits_provider_openai_dispatch()`
  - What they cover:
    - worker missing-registry hard error
    - worker registry-backed openai dispatch
  - What they do not cover:
    - root `run_query_loop(...)` legacy path
    - `QueryConfig.provider_registry = None` in the root path
  - Classification:
    - adjacent / contrast-only; not `M9-07` coverage
- Weak / indirect / overlapping evidence summary:
  - `p10_no_provider_without_model_registry_defaults()` proves identity defaulting, not root dispatch.
  - `agent_tool_errors_when_provider_registry_missing()` proves the opposite behavior in the worker path.
  - Current root-path tests accidentally prove only prompt assembly, config cloning, and provider-options helpers; they do not prove any legacy dispatch behavior.

## M9-07 Coverage Matrix

| Required case | Existing test name(s) | Status | Basis for classification | Likely assertion needed if follow-up execution is required |
|---|---|---|---|---|
| `QueryConfig` with `provider_registry: None` in root path -> legacy Anthropic client path taken | None | `MISSING` | No current test in `lib.rs` or elsewhere calls `run_query_loop(...)` with `provider_registry: None`. The code branch exists at `lib.rs:874` and the legacy call exists at `lib.rs:1291`, but this is untested. | Call `run_query_loop(...)` with `provider_registry: None` and an `AnthropicClient` configured with an empty key so `AnthropicClient::create_message_stream(...)` returns its own auth error. Assert on the Anthropic-client-specific error signature from `src-rust/crates/api/src/lib.rs:614-645`. |
| no registry-backed routing path used when `provider_registry` is `None` in root path | None | `MISSING` | There is no current assertion proving that `resolve_provider_identity(...)` / `materialize_provider(...)` are skipped in the root path when registry is absent, even though the code structure makes that clear. | Use a setup that would behave differently if registry-backed routing were attempted, e.g. `tool_ctx.config.provider = Some("openai")` plus `model = "gpt-4o"` and `provider_registry: None`, then assert the result is the Anthropic-client auth/hint path rather than any provider-resolution/materialization path. |

## Likely Smallest Edit Surface For Execution
- Smallest correct edit surface: `src-rust/crates/query/src/lib.rs #[cfg(test)]`
- Why:
  - `src-rust/crates/query/tests/` is absent
  - the branch under test is local to `run_query_loop(...)` in `lib.rs`
  - existing `lib.rs` tests already have a `make_config(...)` helper with `provider_registry: None`
  - `agent_tool.rs` tests already show reusable patterns for constructing `ToolContext` and isolated auth state without touching production code
- Production code changes appear unnecessary.
- Likely execution shape:
  - add one narrow async test or one sync test with a current-thread runtime
  - construct `ToolContext` locally in the `lib.rs` test module
  - use `provider_registry: None`
  - provide an `AnthropicClient` with empty key and a model whose auth-hint text is deterministic
  - assert `QueryOutcome::Error(...)` contains the Anthropic-client-specific error text

## Validation Readiness
- Later validation should be test-only.
- Recommended future test naming/filter strategy:
  - preferred test name shape: `provider_registry_none_uses_legacy_anthropic_client_path`
  - stable validation filter: `provider_registry_none`
- Validation command was intentionally not run in this preflight session.

## Drift Found
- Minor drift only:
  - authority allowed `src-rust/crates/query/src/lib.rs #[cfg(test)]` or `src-rust/crates/query/tests/`
  - current repo has no `src-rust/crates/query/tests/` directory
  - current `lib.rs` test module exists but contains no async/root-path dispatch coverage
- No structural drift found in:
  - `QueryConfig.provider_registry`
  - `run_query_loop(...)`
  - root `Some` vs `None` branch behavior
  - existence of the legacy `client: &AnthropicClient` path

## Blockers
- No structural blockers.
- No production seam blocker found.
- The only gap is missing test coverage for the exact root-path legacy behavior.

## Notes
- This ticket is not audit-only on current repo evidence.
- Exact `M9-07` coverage does not already exist.
- The strongest current observable signal for follow-up execution is the direct legacy call to `AnthropicClient::create_message_stream(...)` at `src-rust/crates/query/src/lib.rs:1291`, combined with the client’s deterministic empty-key auth error path at `src-rust/crates/api/src/lib.rs:614-645`.
- That signal is stronger than generic success because it ties the result to the legacy client path itself, not merely to a successful request.
- The worker-path missing-registry test from `M9-06` should remain untouched and should only be cited as contrast.
