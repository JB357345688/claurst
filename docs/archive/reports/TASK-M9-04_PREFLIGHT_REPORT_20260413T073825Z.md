# TASK-M9-04 Preflight Report

- Ticket: `TASK-M9-04`
- Timestamp UTC: `2026-04-13T07:38:25Z`
- Branch: `feature/provider-resolution-seam`
- Verdict: `READY-WITH-NOTES`

## Repo State Summary

- `git branch --show-current` returned `feature/provider-resolution-seam`, matching branch expectation.
- `git diff --name-only` returned no tracked unstaged file paths.
- `git diff --cached --name-only` returned no staged file paths.
- `git status --short --branch` showed a clean tracked diff with substantial untracked repo noise, mainly under `docs/Current/`, `docs/archive/reports/`, and `src-rust/target/`.
- `git log --oneline --decorate -n 20` shows:
  - `c28ef22` `TASK-M9-03 prove agent explicit provider routes to openai`
  - `662b29a` `TASK-M9-02 add materialize provider coverage tests`
  - `b5b6dd4` `TASK-M8-11 reconcile M8 workspace validation and formatting`
- Review-basis note: active tracked patch surface is clean for preflight, but later closure reports must acknowledge the large untracked-file background.

## Authority Reviewed

- Repo-local `AGENTS.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md`
- Reviewed M9 dependency graph and the `TASK-M9-04` contract section.
- Reviewed hosted-Ollama standing invariant section.

## Dependency Baseline Confirmed

- `M8-11` is present on the current branch as commit `b5b6dd4`, which satisfies the stated dependency for M9 work.
- `M9-02` and `M9-03` are already present on the branch as commits `662b29a` and `c28ef22`.
- `M9-01` was provided as accepted/complete baseline by the ticket prompt; nothing in inspected repo state suggests reopening it.
- `M9-01` through `M9-03` do not need reopening for `TASK-M9-04`.
- Hosted Ollama compatibility baseline preserved.
- Hosted Ollama remains a standing background invariant only; nothing in this preflight indicates `TASK-M9-04` requires reopening or altering that behavior.

## Exact M9-04 Contract

- Objective: integration test proving an agent without `provider` or `model` inherits the parent provider.
- Later execution intent:
  1. Parent on OpenAI.
  2. Spawn agent with no provider/model.
  3. Agent resolves to OpenAI.
  4. Assert via mock or logging.
- Do not change production code.
- Validation target later: test passes.
- Dependency: `M8-11`.

## Verified Files / Symbols / Commands

- Commands run:
  - `git branch --show-current`
  - `git status --short --branch`
  - `git diff --name-only`
  - `git diff --cached --name-only`
  - `git log --oneline --decorate -n 20`
  - `rg`, `sed`, `find`, `nl -ba`, `date -u +%Y%m%dT%H%M%SZ`
- Files inspected:
  - `AGENTS.md`
  - `docs/Current/MPWO_WORK_ORDER_PACK.md`
  - `src-rust/crates/query/src/agent_tool.rs`
  - `src-rust/crates/query/src/provider_resolution.rs`
  - `src-rust/crates/query/src/lib.rs`
  - `src-rust/crates/tools/src/lib.rs`
  - `src-rust/crates/tools/src/team_tool.rs`
  - `src-rust/crates/core/src/lib.rs`
- Verified symbols / seams:
  - `AgentTool`
  - `AgentTool::execute(...)`
  - `ToolContext`
  - `QueryConfig`
  - `resolve_provider_identity(...)`
  - `materialize_provider(...)`
  - `Config.provider`
  - `TrackingOpenAiProvider`
  - `make_tracking_openai_registry(...)`
  - `agent_explicit_provider_routes_to_openai_provider()`
  - `agent_tool_inherits_parent_provider_without_network()`

## Current Code Reality

- Authority path hint drift exists: `src-rust/crates/query/tests/` does not exist in current repo reality.
- The active agent-provider integration tests already live in the local test module inside `src-rust/crates/query/src/agent_tool.rs`.
- Parent provider state lives in `ToolContext.config.provider` (`src-rust/crates/tools/src/lib.rs:216-235`, `src-rust/crates/core/src/lib.rs:710-746`).
- `AgentTool::execute(...)` computes child model as `params.model.unwrap_or(DEFAULT_MODEL)` and then derives `provider_hint` from:
  - explicit child provider first
  - model prefix second
  - parent `ctx.config.provider` last
  - see `src-rust/crates/query/src/agent_tool.rs:252-269`
- `AgentTool::execute(...)` then resolves and materializes the provider through the shared seam:
  - `resolve_provider_identity(...)`
  - `materialize_provider(...)`
  - see `src-rust/crates/query/src/agent_tool.rs:282-290`
- Once resolved, child context propagation sets `foreground_ctx.config.provider = Some(target.provider_id.clone())` before running the nested query loop (`src-rust/crates/query/src/agent_tool.rs:390-414`).
- Important repo-reality detail: omitting child `model` does not inherit the parent model. It uses `DEFAULT_MODEL` (`claude-opus-4-6`) and only inherits the parent provider.

## Existing Test Surface Audit

- Existing M9-04-equivalent test already exists in some form: yes.
- Existing candidate files:
  - `src-rust/crates/query/src/agent_tool.rs`
  - `src-rust/crates/query/src/provider_resolution.rs`
  - missing authority-hinted path: `src-rust/crates/query/tests/`
- Relevant symbols:
  - `agent_explicit_provider_routes_to_openai_provider()` at `src-rust/crates/query/src/agent_tool.rs:960-981`
  - `agent_tool_inherits_parent_provider_without_network()` at `src-rust/crates/query/src/agent_tool.rs:984-1000`
  - `TrackingOpenAiProvider` / `make_tracking_openai_registry(...)` at `src-rust/crates/query/src/agent_tool.rs:776-879`
  - provider inheritance logic at `src-rust/crates/query/src/agent_tool.rs:252-269`
  - shared resolution seam at `src-rust/crates/query/src/provider_resolution.rs:101-214`
- Reusable patterns already present:
  - agent spawn test harness in the local `agent_tool.rs` test module
  - fake OpenAI provider implementing `LlmProvider`
  - invocation counter via `AtomicUsize`
  - sentinel response text assertion
  - isolated env/auth helper avoiding real credentials
- Current repo already has reusable agent spawn and fake-provider patterns from M9-03.
- Current repo does not have a standalone integration-test directory for `claurst-query`.

## Parent Inheritance Feasibility

- Parent provider/model state relevant to this ticket:
  - parent provider lives in `ctx.config.provider`
  - child model comes from `params.model` or falls back to `DEFAULT_MODEL`
  - parent model is not inherited in the current `AgentTool` path
- Child agent request construction in `AgentTool::execute(...)`:
  - omitted child `provider` becomes `explicit_provider = None`
  - omitted child `model` becomes `DEFAULT_MODEL`
  - because `DEFAULT_MODEL` has no known provider prefix, `provider_hint` falls through to `parent_provider`
- The shared seam then treats the inherited `parent_provider` as the explicit provider argument to `resolve_provider_identity(...)`, which yields the inherited provider instead of the default anthropic path.
- This means omitting both `provider` and `model` in the child request should inherit the parent provider exactly as the ticket states.
- This can be proven directly with a fake OpenAI provider, not merely inferred from success-path behavior.

## Mocking / Assertion Surface Feasibility

- A fake OpenAI provider can be injected cleanly through the current interfaces by registering an `Arc<dyn LlmProvider>` in `ProviderRegistry`.
- The M9-03-style fake/counter/sentinel pattern is already implemented and reusable in `src-rust/crates/query/src/agent_tool.rs`.
- Most stable assertion strategy for later execution:
  - parent `ToolContext.config.provider = Some("openai")`
  - child input omits both `provider` and `model`
  - use `make_tracking_openai_registry(...)`
  - set `max_turns = 1`
  - assert invocation counter is `1`
  - assert result content equals the sentinel response
- This is stronger and less brittle than logs-only evidence.
- Real credentials are avoidable with the existing fake provider and `with_isolated_provider_auth(...)`.
- `#[ignore]` appears unnecessary based on current repo reality.

## Likely Smallest Edit Surface For Execution

- `existing file src-rust/crates/query/src/agent_tool.rs local test module`

## Validation Readiness

- Production code changes do not appear necessary.
- Existing test harness support is already local to `agent_tool.rs`.
- Current parent-inheritance test name exists, but the present implementation is weaker than the ticket contract because it does not force provider dispatch.
- Later execution likely needs either:
  - strengthening the existing `agent_tool_inherits_parent_provider_without_network()` test, or
  - replacing it with a more explicit name tied to the inherited-provider behavior.
- A stable later filter appears feasible without `#[ignore]`; if kept local, a name containing `agent_parent_inherits_provider` would provide a deterministic filter surface.
- No special external setup or real provider credentials appear necessary.

## Drift Found

- Path drift: authority names `crates/query/tests/`, but `src-rust/crates/query/tests/` does not exist.
- Existing local test surface in `src-rust/crates/query/src/agent_tool.rs` is the smaller and more repo-accurate execution surface.
- Existing `agent_tool_inherits_parent_provider_without_network()` test is only a partial equivalent:
  - it uses `max_turns: 0`
  - `run_query_loop(...)` returns immediately when `turn > effective_max_turns`, returning the last message instead of dispatching to a provider (`src-rust/crates/query/src/lib.rs:702-721`)
  - it therefore does not assert actual OpenAI provider invocation, only that execution did not error before the loop
- This is drift in assertion strength, not a structural blocker.

## Blockers

- None.

## Notes

- Deterministic preflight verdict: `READY-WITH-NOTES`.
- Smallest correct future execution surface is local to `src-rust/crates/query/src/agent_tool.rs`, not a new integration-test directory.
- The existing parent-inheritance test means the ticket surface is already partially present, but it should be tightened to match the contract’s preferred evidence surface.
- The exact ticket wording says the child omits both `provider` and `model`; current repo reality supports that shape cleanly.
- The adjacent team-runner path in `init_team_swarm_runner()` currently resolves from `provider_override` / `model_override` and does not read `ctx.config.provider`; that is outside `TASK-M9-04` scope and does not block this ticket as written.
