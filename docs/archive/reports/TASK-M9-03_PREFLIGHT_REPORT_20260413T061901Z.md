# TASK-M9-03 Preflight Report

## Ticket
- `TASK-M9-03`
- Title: `Agent explicit provider routing integration test`

## Timestamp UTC
- `2026-04-13T06:19:01Z`

## Branch
- Expected: `feature/provider-resolution-seam`
- Actual: `feature/provider-resolution-seam`

## Verdict
- `READY-WITH-NOTES`

## Repo State Summary
- `git branch --show-current` returned `feature/provider-resolution-seam`.
- `git status --short --branch` shows one tracked unstaged source diff: `src-rust/crates/query/src/provider_resolution.rs`.
- `git diff --name-only` returned only `src-rust/crates/query/src/provider_resolution.rs`.
- `git diff --cached --name-only` returned no staged paths.
- `git log --oneline --decorate -n 20` shows `HEAD` at `b5b6dd4 (HEAD -> feature/provider-resolution-seam) TASK-M8-11 reconcile M8 workspace validation and formatting`.
- The worktree also contains many untracked docs/build artifacts, including `docs/Current/`, `docs/archive/reports/*`, and `src-rust/target/`.
- Repo state is compatible with starting M9-03, but later patch hygiene/closure must account for the pre-existing unstaged `provider_resolution.rs` diff.

## Authority Reviewed
- Repo-local `AGENTS.md` reviewed and treated as controlling authority.
- `docs/Current/MPWO_WORK_ORDER_PACK.md` reviewed and treated as ticket authority.
- Relevant MPWO sections verified:
  - M9 dependency rule: `M9 Validation (requires M8-11 complete)`.
  - Standing invariant `2A. Hosted Ollama Compatibility`.
  - Ticket section `TASK-M9-03 — Agent explicit provider routing integration test`.

## Dependency Baseline Confirmed
- Branch/head reality is compatible with starting M9-03 after accepted M8-11:
  - branch matches expectation
  - `HEAD` is the M8-11 commit
- M9-01 does not need reopening:
  - no source changes for M9-01 were required by contract
  - the repo contains `docs/archive/reports/TASK-M9-01_*` artifacts only
- M9-02 does not need reopening:
  - the existing unstaged diff in `src-rust/crates/query/src/provider_resolution.rs` is test-only and matches the M9-02 contract surface
  - current `provider_resolution.rs` test body already contains the M9-02-style cases for auth-store precedence, `lm-studio`/`llama-cpp` `api_base`, and missing credentials
- Hosted Ollama compatibility baseline preserved.
- Hosted Ollama remains a preserved background invariant only; M9-03 does not need to reopen or widen that baseline.

## Exact M9-03 Contract
- Objective: integration test proving that an agent invoked with `provider: "openai"` dispatches to OpenAI.
- Candidate file surfaces from ticket authority:
  - `src-rust/crates/query/tests/`
  - `src-rust/crates/query/src/agent_tool.rs`
- Later execution target:
  1. construct `ToolContext` with a mock/fake `ProviderRegistry` containing an OpenAI provider
  2. call `AgentTool::execute()` with `provider: "openai"`
  3. assert OpenAI was selected using the least-brittle available signal
- Constraints:
  - no real OpenAI credentials
  - use mocks/fakes if feasible
  - use `#[ignore]` only if mocking is genuinely infeasible
- Later validation command:
  - `cargo test -p claurst-query -- agent_explicit_provider`

## Verified Files / Symbols / Commands
- Files reviewed:
  - `AGENTS.md`
  - `docs/Current/MPWO_WORK_ORDER_PACK.md`
  - `src-rust/crates/query/src/agent_tool.rs`
  - `src-rust/crates/query/src/provider_resolution.rs`
  - `src-rust/crates/query/src/lib.rs`
  - `src-rust/crates/tools/src/lib.rs`
  - `src-rust/crates/api/src/registry.rs`
  - `src-rust/crates/api/src/provider.rs`
  - `src-rust/crates/api/src/provider_types.rs`
  - `src-rust/crates/core/src/lib.rs`
- Symbols/path reality verified:
  - `AgentTool`
  - `AgentTool::execute(...)`
  - `ToolContext`
  - `QueryConfig.provider_registry`
  - `resolve_provider_identity(...)`
  - `materialize_provider(...)`
  - `ProviderRegistry::register(...)`
  - `runtime_provider_for(...)`
  - local `agent_tool.rs` test helpers `make_tool_context`, `make_openai_registry`, `run_agent_tool`
- Commands executed:
  - `git branch --show-current`
  - `git status --short --branch`
  - `git diff --name-only`
  - `git diff --cached --name-only`
  - `git log --oneline --decorate -n 20`
  - `git diff -- src-rust/crates/query/src/provider_resolution.rs`
  - targeted `rg`/`sed` inspections on the files above

## Current Code Reality
- `src-rust/crates/query/src/agent_tool.rs` contains the active agent provider-routing path:
  - provider hint selection at `agent_tool.rs:264`
  - missing registry hard-error at `agent_tool.rs:276`
  - provider resolution call at `agent_tool.rs:283`
  - provider materialization call at `agent_tool.rs:288`
  - resolved provider copied into child context at `agent_tool.rs:414`
- `ToolContext` already exposes `provider_registry: Option<Arc<ProviderRegistry>>` and `model_registry: Option<Arc<ModelRegistry>>` in `src-rust/crates/tools/src/lib.rs:216`.
- Root query-loop dispatch already routes registry-backed providers through the shared seam in `src-rust/crates/query/src/lib.rs:871-910`, then calls `provider.create_message_stream(...)` at `lib.rs:989`.
- `src-rust/crates/query/src/provider_resolution.rs` already supports the needed agent path:
  - explicit provider resolution via `resolve_provider_identity(...)`
  - provider materialization via `materialize_provider(...)`
  - registry fallback via `registry.get(&pid).cloned()` at `provider_resolution.rs:178`
  - auth-store runtime override via `runtime_provider_for(...)` at `provider_resolution.rs:173`
- `ProviderRegistry` accepts arbitrary `Arc<dyn LlmProvider>` registrations in `src-rust/crates/api/src/registry.rs:93`.

## Existing Test Surface Audit
- M9-03-equivalent test already exists: `yes`
- Existing equivalent candidate:
  - `src-rust/crates/query/src/agent_tool.rs:849`
  - test name: `agent_tool_resolves_explicit_provider_without_network`
- What that existing test does:
  - builds a `ToolContext` with an OpenAI registry entry
  - calls `AgentTool.execute(...)` with `"provider": "openai"` and `"model": "gpt-4o"`
  - isolates provider auth env
  - asserts success and echoed result without network
- Why it is only partial/equivalent rather than closure-ready for this ticket:
  - it relies on `max_turns: 0`, so `run_query_loop` exits before any provider stream call
  - the assertion is indirect: success/result shape implies the explicit provider path avoided the anthropic/no-credentials failure path
  - it does not directly prove a fake OpenAI provider was invoked
- Candidate files in current repo:
  - `src-rust/crates/query/src/agent_tool.rs`
  - `src-rust/crates/query/src/provider_resolution.rs`
  - `src-rust/crates/query/tests/` does not currently exist
- Relevant symbols/patterns worth reusing:
  - `agent_tool.rs` helpers:
    - `make_tool_context`
    - `make_openai_registry`
    - `run_agent_tool`
    - `with_isolated_provider_auth`
  - provider-routing seam:
    - `resolve_provider_identity`
    - `materialize_provider`
  - provider fake pattern already present:
    - local `TestProvider` implementing `LlmProvider` in `provider_resolution.rs` tests
- Provider-routing / agent dispatch / fake-provider patterns already reusable: `yes`

## Mocking / Assertion Surface Feasibility
- Mock/fake OpenAI injection through current interfaces: `feasible`
  - `ProviderRegistry::register(...)` already accepts `Arc<dyn LlmProvider>`
  - existing repo test code already defines a local fake `LlmProvider` implementation pattern
- Strongest stable assertion surface currently available:
  - preferred: fake provider with `id = "openai"` plus an invocation counter on `create_message_stream(...)`
  - have the fake emit a deterministic one-message `end_turn` stream
  - assert both:
    - invocation count incremented
    - `ToolResult.content` equals the fake provider sentinel response
- Why this is the least-brittle option:
  - it proves actual OpenAI dispatch, not just successful resolution/materialization
  - it avoids brittle structured-log matching
  - it avoids dependence on real network clients or credentials
- Current weaker but already-present assertion surface:
  - result-shape/success path with `max_turns: 0`
  - viable as evidence of provider selection, but weaker than direct fake invocation
- Real credentials avoidable with current surfaces: `yes`
- `#[ignore]` necessary: `no`
  - current seams are sufficient for a pure local fake-based test
  - no production seam changes appear necessary
- Important implementation detail for later execution:
  - `run_query_loop` uses `create_message_stream(...)`, not `create_message(...)`
  - a fake used for a stronger dispatch proof must implement stream events, not only a non-streaming response

## Likely Smallest Edit Surface For Execution
- `existing file src-rust/crates/query/src/agent_tool.rs local test module`
- Reason:
  - `src-rust/crates/query/tests/` is absent
  - `agent_tool.rs` already contains the exact helper scaffolding needed for `ToolContext` construction and isolated auth setup
  - adding a narrow local test there is smaller than creating a new integration-test harness directory/file
  - no production-code changes appear necessary

## Validation Filter Readiness
- Current readiness: `not ready as-is`
- Reason:
  - the existing equivalent test is named `agent_tool_resolves_explicit_provider_without_network`
  - the later validation filter is `agent_explicit_provider`
  - that filter string would not match the current test name
- Naming adaptation needed later: `yes`
  - the eventual closure test should include the literal substring `agent_explicit_provider`
  - example shape: `agent_explicit_provider_routes_to_openai`
- This is naming-only drift, not a structural blocker.

## Drift Found
- Minor path drift:
  - `src-rust/crates/query/tests/` does not exist in the current repo
- Minor validation drift:
  - current equivalent test name will not match the required later filter string `agent_explicit_provider`
- Patch hygiene drift:
  - one pre-existing unstaged tracked diff exists in `src-rust/crates/query/src/provider_resolution.rs`
  - many untracked docs/build artifacts also exist in the worktree
- No structural drift found in:
  - `AgentTool`
  - `ToolContext`
  - `ProviderRegistry`
  - provider-resolution/materialization seam
- Hosted Ollama compatibility baseline preserved.

## Blockers
- None structural for M9-03.
- Not blocked on:
  - file availability
  - interface shape
  - provider fake injection
  - stable assertion surface
  - real credentials
- Caution for later closure only:
  - pre-existing `provider_resolution.rs` diff will need explicit review-basis handling so M9-03’s patch stays scope-clean

## Notes
- The repo already contains a practical starting point for M9-03 in `agent_tool.rs`; later implementation can tighten that existing coverage rather than widening into production code.
- The strongest later execution shape is a local `agent_tool.rs` test using a fake OpenAI provider with a stream-call counter and deterministic response text.
- Because `HEAD` is still the M8-11 commit while M9-02-style test changes remain unstaged, M9-03 is ready to start but not on a clean accepted baseline.
- M9-01 and M9-02 should remain closed/not reopened during M9-03 execution.
