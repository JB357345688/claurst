# TASK-M9-09 Preflight Report

## Ticket
- `TASK-M9-09 — TeamCreate mixed providers integration test`

## Timestamp UTC
- `2026-04-13T13:07:02Z`

## Branch
- Expected branch: `feature/provider-resolution-seam`
- Verified branch: `feature/provider-resolution-seam`

## Verdict
- `READY-WITH-NOTES`

## Repo State Summary
- `git branch --show-current` returned `feature/provider-resolution-seam`.
- `git status --short --branch` shows no staged or unstaged tracked-file diffs, but the worktree is noisy with many unrelated untracked docs/report paths plus `src-rust/target/`.
- `git diff --name-only` returned no paths.
- `git diff --cached --name-only` returned no paths.
- `git log --oneline --decorate -n 20` shows `HEAD` at `63a8485` (`TASK-M9-08 prove root registry failure does not fallback to legacy anthropic`), with `b5b6dd4` (`TASK-M8-11`) and the later M9 provider-seam commits present in recent history.
- Patch hygiene note for later execution: unrelated untracked repo noise exists and should be called out again during execution/review, but it does not block this preflight.

## Authority Reviewed
- Repo-local `AGENTS.md` is controlling for this repository and requires one-ticket scope, read-first verification, and report creation under `docs/archive/reports/`.
- `docs/Current/MPWO_WORK_ORDER_PACK.md` is the controlling ticket authority.
- Verified ticket section:
  - `docs/Current/MPWO_WORK_ORDER_PACK.md:1684` defines `TASK-M9-09 — TeamCreate mixed providers integration test`.
  - Contract states: create a team with agent A on `"openai"` and agent B on `"google"`, verify each resolves to its specified provider, do not require real API keys, use mocks, and allow `#[ignore]` only if genuinely necessary.
- Verified standing invariant:
  - `docs/Current/MPWO_WORK_ORDER_PACK.md:108-134` treats hosted-Ollama compatibility as preserved baseline for later provider-seam work.
- Preflight conclusion on authority:
  - Hosted-Ollama remains a preserved background constraint only.
  - `M9-01` through `M9-08` are acknowledged as already complete and are not reopened in this session.

## Dependency Baseline Confirmed
- `M8-11` dependency is satisfied by branch history: `b5b6dd4 TASK-M8-11 reconcile M8 workspace validation and formatting`.
- Recent branch history also contains:
  - `662b29a TASK-M9-02 add materialize provider coverage tests`
  - `c28ef22 TASK-M9-03 prove agent explicit provider routes to openai`
  - `2f1f169 TASK-M9-04 prove agent inherits parent provider on openai dispatch`
  - `dfc4be4 TASK-M9-07 prove root missing registry uses legacy anthropic path`
  - `63a8485 TASK-M9-08 prove root registry failure does not fallback to legacy anthropic`
- `M9-05` and `M9-06` were provided as accepted no-source-change tickets in the session contract and are treated as baseline.
- `M9-01` was provided as complete in the session contract and is treated as baseline without reopening.
- Branch reality is compatible with starting `M9-09` after accepted `M8-11`.

## Exact M9-09 Contract
- Objective: integration test that `TeamCreate` spawns agents with different providers per agent.
- Required scenario:
  - agent A configured for `"openai"`
  - agent B configured for `"google"`
  - each agent resolves to its own specified provider
- Constraints:
  - no real API keys
  - use mocks/fakes if current seams allow it
  - `#[ignore]` only if genuinely necessary with explanation
- Later validation target:
  - test passes, or
  - test is `#[ignore]`-gated with explanation

## Verified Files / Symbols / Commands
- Commands verified:
  - `git branch --show-current`
  - `git status --short --branch`
  - `git diff --name-only`
  - `git diff --cached --name-only`
  - `git log --oneline --decorate -n 20`
  - `rg`/`sed`/`nl` inspections over the active ticket surfaces
- Files verified:
  - `AGENTS.md`
  - `docs/Current/MPWO_WORK_ORDER_PACK.md`
  - `src-rust/crates/tools/src/team_tool.rs`
  - `src-rust/crates/query/src/agent_tool.rs`
  - `src-rust/crates/query/src/provider_resolution.rs`
  - `src-rust/crates/query/src/lib.rs`
  - `src-rust/crates/tools/src/lib.rs`
  - `src-rust/crates/cli/src/main.rs`
- Symbols verified:
  - `TeamCreateTool`
  - `AgentSpec.provider`
  - `AgentRunParams.provider_override`
  - `init_team_swarm_runner()`
  - `resolve_provider_identity(...)`
  - `materialize_provider(...)`
  - existing tests:
    - `agent_explicit_provider_routes_to_openai_provider`
    - `agent_parent_inherits_provider_openai_dispatch`
    - `p4_no_provider_with_known_model_prefix`
    - `p8_no_provider_bare_model_registry_resolves`
    - `materialize_provider_returns_openai_target_from_happy_path`
    - `provider_registry_none_uses_legacy_anthropic_client_path`
    - `provider_registry_some_resolution_failure_does_not_fallback_to_legacy_anthropic`

## Current Code Reality
- Authority-hinted directories `src-rust/crates/tools/tests/` and `src-rust/crates/query/tests/` are both absent in current repo reality.
- The current team-creation implementation lives in [src-rust/crates/tools/src/team_tool.rs](/home/jordi/claurst/src-rust/crates/tools/src/team_tool.rs:152).
- `TeamCreateTool` accepts per-agent `provider` and `model` fields in `AgentSpec` and the tool input schema:
  - provider field stored in `AgentSpec`: [team_tool.rs](/home/jordi/claurst/src-rust/crates/tools/src/team_tool.rs:166)
  - provider exposed in schema: [team_tool.rs](/home/jordi/claurst/src-rust/crates/tools/src/team_tool.rs:242)
- `TeamCreateTool` forwards the per-agent provider to the runner via `AgentRunParams.provider_override`:
  - param definition: [team_tool.rs](/home/jordi/claurst/src-rust/crates/tools/src/team_tool.rs:37)
  - handoff from team agent spec: [team_tool.rs](/home/jordi/claurst/src-rust/crates/tools/src/team_tool.rs:389)
  - dispatch into `run_agent(...)`: [team_tool.rs](/home/jordi/claurst/src-rust/crates/tools/src/team_tool.rs:413)
- The actual provider resolution/materialization for team-spawned agents happens in the injected query runner, not in `claurst-tools` alone:
  - runner registration entrypoint: [src-rust/crates/query/src/agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:572)
  - runner resolves provider identity from `provider_override`: [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:615)
  - runner materializes the execution target: [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:629)
  - runner sets `runner_ctx.config.provider` to the resolved provider: [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:698)
  - runner executes `run_query_loop(...)`: [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:703)
- Provider recognition already includes both `"openai"` and `"google"` in the shared seam:
  - known providers list: [src-rust/crates/query/src/provider_resolution.rs](/home/jordi/claurst/src-rust/crates/query/src/provider_resolution.rs:6)
- Team result aggregation already returns structured per-agent outputs, which is a stable assertion surface for a future mixed-provider test:
  - `results` JSON shape and `aggregated_output`: [team_tool.rs](/home/jordi/claurst/src-rust/crates/tools/src/team_tool.rs:446)

## Existing Mixed-Provider Test Audit
- `agent_explicit_provider_routes_to_openai_provider`
  - Location: [src-rust/crates/query/src/agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:955)
  - Covers: single-agent explicit provider routing to a fake OpenAI provider, with invocation count and sentinel output asserted.
  - Strength: proves real foreground agent dispatch can route through the registry-backed provider seam for `openai` without real credentials.
  - Limitation: no `TeamCreate`, no second provider, no multi-agent interaction.
- `agent_parent_inherits_provider_openai_dispatch`
  - Location: [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:979)
  - Covers: single-agent inheritance from parent provider (`openai`) through actual dispatch.
  - Strength: proves inherited provider flow works for one agent.
  - Limitation: does not prove per-agent override inside a team and does not involve `google`.
- `p4_no_provider_with_known_model_prefix`
  - Location: [src-rust/crates/query/src/provider_resolution.rs](/home/jordi/claurst/src-rust/crates/query/src/provider_resolution.rs:528)
  - Covers: shared resolution logic recognizes a `google/...` model prefix.
  - Strength: proves the resolution seam understands `google`.
  - Limitation: identity-only; no team path, no dispatch, no provider invocation.
- `p8_no_provider_bare_model_registry_resolves`
  - Location: [provider_resolution.rs](/home/jordi/claurst/src-rust/crates/query/src/provider_resolution.rs:569)
  - Covers: model-registry-based resolution for a Google Gemini model.
  - Strength: additional indirect evidence that `google` is wired into shared resolution.
  - Limitation: still identity-only; no spawned agent or team execution.
- `materialize_provider_returns_openai_target_from_happy_path`
  - Location: [provider_resolution.rs](/home/jordi/claurst/src-rust/crates/query/src/provider_resolution.rs:633)
  - Covers: registry materialization for `openai`.
  - Strength: proves the materialization layer can produce an `openai` execution target.
  - Limitation: no team path, no `google`, no live dispatch.
- `provider_registry_none_uses_legacy_anthropic_client_path`
  - Location: [src-rust/crates/query/src/lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:2359)
  - Covers: root fallback behavior when no registry is present.
  - Limitation: not a team or mixed-provider test; relevant only as surrounding seam baseline.
- `provider_registry_some_resolution_failure_does_not_fallback_to_legacy_anthropic`
  - Location: [lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:2388)
  - Covers: hard-error behavior when registry-backed resolution fails.
  - Limitation: not a team or mixed-provider test; relevant only as surrounding seam baseline.
- Team / TeamCreate-specific tests currently present:
  - None found in `src-rust/crates/tools/src/team_tool.rs`
  - None found under `src-rust/crates/tools/tests/` because the directory is absent
  - None found under `src-rust/crates/query/tests/` because the directory is absent
- Exact `M9-09`-equivalent coverage already exists:
  - No
- Weak / indirect / overlapping coverage:
  - The current tests are all single-agent or seam-layer tests.
  - They overlap on openai-only routing and generic provider resolution behavior.
  - None exercise the `TeamCreate` multi-agent path.
- Tests that prove only generic team success rather than per-agent provider resolution:
  - None found, because there are currently no `TeamCreate` tests at all.

## M9-09 Coverage Matrix

| Required case | Existing test name(s) | Status | Basis for classification | Likely assertion needed if follow-up execution is required |
| --- | --- | --- | --- | --- |
| agent A configured for openai -> resolves to openai | `agent_explicit_provider_routes_to_openai_provider`; `materialize_provider_returns_openai_target_from_happy_path` | PARTIAL | OpenAI explicit-provider routing is proven for a single agent and at seam level, but not through `TeamCreate` | Assert agent A result maps to the OpenAI sentinel and OpenAI invocation counter increments exactly once |
| agent B configured for google -> resolves to google | `p4_no_provider_with_known_model_prefix`; `p8_no_provider_bare_model_registry_resolves` | PARTIAL | Google is recognized in shared resolution logic, but there is no agent or team execution test using a fake Google provider | Add a fake streaming Google provider, assert agent B result maps to the Google sentinel and Google invocation counter increments exactly once |
| mixed team run does not collapse to a single provider | none | MISSING | No current test invokes `TeamCreate` with two agents on different providers | Assert both provider counters are exactly `1`, results JSON contains both agents with distinct sentinels, and neither provider services both agents |
| no real API keys required / mocks feasible | `agent_explicit_provider_routes_to_openai_provider`; env-isolation helpers in `agent_tool.rs`; registry-backed seam in `provider_resolution.rs` | PARTIAL | OpenAI mock/no-key execution is already proven and the registry seam supports fake providers, but there is no existing mixed-team fake-provider test and no reusable fake Google streamer yet | Reuse the isolated `HOME`/env guard pattern and a registry containing fake OpenAI + fake Google streaming providers; no real credentials should be required |

## Mocking / Assertion Surface Feasibility
- Fake OpenAI provider injection through current interfaces:
  - Yes.
  - Existing evidence: `TrackingOpenAiProvider` in [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:770) is a streaming fake registered into `ProviderRegistry`.
- Fake Google provider injection through current interfaces:
  - Yes, by current interface design.
  - Basis: `materialize_provider(...)` accepts any registry provider keyed by `ProviderId` when no runtime provider overrides it: [provider_resolution.rs](/home/jordi/claurst/src-rust/crates/query/src/provider_resolution.rs:171).
  - Gap: there is no existing reusable streaming fake for `"google"` in current repo reality.
- Reusable fake-provider or stream-counter patterns:
  - Present for OpenAI only: `TrackingOpenAiProvider` plus `make_tracking_openai_registry(...)`.
  - Present as a generic non-streaming seam pattern: `TestProvider` in `provider_resolution.rs`.
  - Missing: a current multi-provider streaming harness or a generic tracking streaming provider parameterized by provider id.
- Exact assertion strategy that appears most stable:
  - Register a fake streaming OpenAI provider and a fake streaming Google provider in one `ProviderRegistry`.
  - Initialize the team swarm runner once in test scope.
  - Invoke `TeamCreateTool.execute(...)` with two agents, one `provider: "openai"` and one `provider: "google"`.
  - Parse returned JSON `results` and assert:
    - agent A output equals an OpenAI-only sentinel
    - agent B output equals a Google-only sentinel
    - OpenAI invocation counter == `1`
    - Google invocation counter == `1`
  - This directly proves provider dispatch and guards against single-provider bleed-through.
- Real credentials avoidable:
  - Yes.
  - Basis: current single-agent tests already clear `ANTHROPIC_API_KEY` and `OPENAI_API_KEY` and run against fake providers only.
- `#[ignore]` appears necessary or avoidable:
  - Avoidable on current read-only evidence.
  - Reason: the required seams already exist, and the missing piece is test coverage, not runtime capability.
- Additional execution note:
  - `register_agent_runner(...)` uses `OnceCell` and panics on double registration: [team_tool.rs](/home/jordi/claurst/src-rust/crates/tools/src/team_tool.rs:52).
  - Current CLI guards this with `OnceLock`: [src-rust/crates/cli/src/main.rs](/home/jordi/claurst/src-rust/crates/cli/src/main.rs:651).
  - A future test should mirror that one-time init pattern to keep execution stable.

## Likely Smallest Edit Surface For Execution
- Smallest correct test surface in current repo reality:
  - `src-rust/crates/query/src/agent_tool.rs` `#[cfg(test)]` module
- Why this is the best surface:
  - The actual provider resolution/materialization seam for team-spawned agents is implemented in `claurst-query`’s injected runner, not solely in `claurst-tools`.
  - Existing no-key env isolation and streaming fake-provider helpers already live there.
  - `src-rust/crates/tools/tests/` and `src-rust/crates/query/tests/` are absent, so using them would introduce new test directories rather than the smallest local surface.
- Smaller-but-inferior alternative:
  - `src-rust/crates/tools/src/team_tool.rs` local tests with a custom stub runner could prove `provider_override` forwarding only.
  - It would not directly prove the actual `query` provider-resolution/materialization seam required by `M9-09`.
- Production code changes appear necessary:
  - No, based on current read-only evidence.
- Likely future implementation shape:
  - Extend the current `agent_tool.rs` test harness with a generic tracking streaming provider or a second Google-specific tracking provider.
  - Add one `TeamCreate` integration-style test that uses the real injected runner.

## Validation Readiness
- `M9-09` is not audit-only on current evidence.
- Exact mixed-provider-per-agent coverage does not already exist strongly enough to close the ticket without a new test.
- A real test addition is needed.
- Narrow validation filter recommendation for the later execution phase:
  - `mixed_providers_per_agent`
- Reason:
  - It is narrower and less collision-prone than `team_create`.
  - It is more idiomatic and stable than relying on the unidiomatic single token `teamcreate`.
- Expected validation posture after implementation:
  - Test should be able to run normally without `#[ignore]`.

## Drift Found
- Minor drift:
  - Authority-hinted directories `src-rust/crates/tools/tests/` and `src-rust/crates/query/tests/` are absent.
  - Current repo keeps the relevant integration-style tests inline under `src-rust/crates/query/src/*.rs`.
- Minor drift:
  - There is no existing `TeamCreate` test surface; team-path coverage is currently missing entirely.
- Minor drift:
  - There is no current reusable streaming fake Google provider, only an OpenAI one.
- No structural drift found in the team path itself:
  - `TeamCreate` still exists.
  - Per-agent provider selection still exists.
  - The query runner still resolves and materializes provider overrides per agent.

## Blockers
- None.

## Notes
- Deterministic preflight conclusion:
  - `READY-WITH-NOTES`
- Covered cases today:
  - single-agent explicit OpenAI provider dispatch
  - single-agent parent-provider OpenAI inheritance
  - Google recognition in the shared provider-resolution seam
  - registry-backed no-real-key testing patterns
- Missing cases today:
  - actual `TeamCreate` mixed-provider execution
  - proof that one team can dispatch one agent to `openai` and another to `google`
  - proof that no single-provider bleed-through occurs across team members
- Session constraints honored:
  - no source files edited
  - no tests added
  - no staging
  - no commit
  - no ticket validation command run
  - exactly one preflight report created
