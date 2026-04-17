# TASK-M9-10 Preflight Report

- Ticket: `TASK-M9-10`
- Timestamp UTC: `2026-04-13T13:42:57Z`
- Branch: `feature/provider-resolution-seam`
- Verdict: `READY-WITH-NOTES`

## Repo State Summary

- `git branch --show-current` -> `feature/provider-resolution-seam`
- `git status --short --branch` -> branch matches expectation; tracked worktree appears clean; there is substantial unrelated untracked noise under `docs/`, `.codex`, and `src-rust/target/`
- `git diff --name-only` -> no unstaged tracked-file diff
- `git diff --cached --name-only` -> no staged diff
- `git log --oneline --decorate -n 20` -> HEAD is `5e77652` (`TASK-M9-09 prove TeamCreate mixed providers dispatch per agent`); accepted dependency baseline `b5b6dd4` (`TASK-M8-11`) is present in history
- Patch hygiene note: active tracked diff basis is clean, but untracked-report noise should be kept out of any later closure/review basis

## Authority Reviewed

- Repo-local `AGENTS.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md`
- `TASK-M9-10` ticket section reviewed directly
- Hosted Ollama invariant reviewed in MPWO section `2A`
- Hosted Ollama compatibility baseline preserved

## Dependency Baseline Confirmed

- `TASK-M8-11` is present in branch history and is compatible with starting `M9-10`
- `M9-01` through `M9-09` are treated as accepted baseline per prompt and do not require reopening
- Current branch head already includes later accepted M9 integration-test work (`M9-03`, `M9-04`, `M9-07`, `M9-08`, `M9-09`) aligned with the shared provider seam
- No read-only evidence indicates the hosted-Ollama compatibility baseline has been reopened; it remains a preserved background constraint only

## Exact M9-10 Contract

- Objective: prove that agent spawn succeeds when `ANTHROPIC_API_KEY` is absent and the spawned agent explicitly requests `provider: "openai"`
- Required setup:
  - `ANTHROPIC_API_KEY` absent in test environment
  - explicit provider `openai` on agent spawn
  - mock/fake OpenAI provider available in registry
  - success result, not an Anthropic-key-missing failure
- Do not:
  - call real OpenAI
  - change production code

## Verified Files / Symbols / Commands

- Verified commands:
  - `git branch --show-current`
  - `git status --short --branch`
  - `git diff --name-only`
  - `git diff --cached --name-only`
  - `git log --oneline --decorate -n 20`
  - `rg -n "TASK-M9-10|M8-11|hosted-Ollama" docs/Current/MPWO_WORK_ORDER_PACK.md`
  - targeted `rg`, `sed`, and `nl` reads under `src-rust/crates/query/src/`, `src-rust/crates/tools/src/`, and `src-rust/crates/api/src/`
- Verified files:
  - `AGENTS.md`
  - `docs/Current/MPWO_WORK_ORDER_PACK.md`
  - `src-rust/crates/query/src/agent_tool.rs`
  - `src-rust/crates/query/src/lib.rs`
  - `src-rust/crates/query/src/provider_resolution.rs`
  - `src-rust/crates/tools/src/lib.rs`
  - `src-rust/crates/api/src/registry.rs`
- Verified symbols / helpers:
  - `AgentTool::execute`
  - `resolve_provider_identity(...)`
  - `materialize_provider(...)`
  - `runtime_provider_for(...)`
  - `TrackingStreamingProvider`
  - `make_tracking_openai_registry(...)`
  - `with_isolated_provider_auth(...)`
  - `agent_explicit_provider_routes_to_openai_provider`

## Current Code Reality

- Authority-hinted path drift exists: `src-rust/crates/query/tests/` does not exist in current repo reality
- The smallest correct local test surface is the existing in-file test module in `src-rust/crates/query/src/agent_tool.rs`
- Agent spawn path currently resolves provider through the shared seam:
  - `agent_tool.rs:282-290` resolves identity and materializes provider from registry-backed seam
  - `agent_tool.rs:293-315` only attempts Anthropic credential lookup when `target.provider_id == "anthropic"`
  - for explicit `openai`, that Anthropic-auth branch is bypassed
- The query loop registry-backed dispatch path is hard separated from the legacy raw-Anthropic path:
  - `lib.rs:874-905` resolves/materializes provider when `config.provider_registry` is present
  - `lib.rs:905+` dispatches directly to `target.provider.create_message_stream(...)`
  - failures in this path return immediately and do not fall through to the legacy root Anthropic path
- Provider materialization supports the required mock injection shape:
  - `provider_resolution.rs:173-179` prefers runtime credential-backed provider only if available
  - otherwise it uses `registry.get(...)`
  - `api/src/registry.rs:74-80` shows runtime provider creation depends on stored credentials being present

## Existing No-Key Agent Spawn Test Audit

- `agent_explicit_provider_routes_to_openai_provider`
  - Appears to cover the exact functional requirement
  - Uses `with_isolated_provider_auth(...)` to set `HOME` to a temp dir and clear `ANTHROPIC_API_KEY`, `OPENAI_API_KEY`, and `GOOGLE_API_KEY`
  - Injects a mock OpenAI provider via `make_tracking_openai_registry(...)`
  - Spawns `AgentTool` with `"provider": "openai"` and `"model": "gpt-4o"`
  - Asserts success (`!result.is_error`), exact sentinel output, and one mock invocation
  - Exact M9-10-equivalent coverage already exists in substance, despite the generic test name
- `agent_parent_inherits_provider_openai_dispatch`
  - Overlapping but weaker for this ticket
  - Proves inherited parent-provider dispatch to OpenAI mock succeeds under no-key isolation
  - Does not use explicit provider on the spawned agent input, so it is not the ticket’s exact case
- `teamcreate_mixed_providers_per_agent_dispatch`
  - Overlapping integration coverage
  - Proves per-agent dispatch to mock OpenAI/Google providers under no-key isolation
  - Stronger on multi-agent dispatch, weaker on explicit single-agent no-Anthropic-key framing
- `provider_registry_none_uses_legacy_anthropic_client_path`
  - Negative-control root-path test
  - Shows missing registry can still surface legacy Anthropic/OpenAI auth guidance
  - Relevant only to distinguish the path that `M9-10` must avoid
- `provider_registry_some_resolution_failure_does_not_fallback_to_legacy_anthropic`
  - Negative-control root-path seam test
  - Proves registry-backed root resolution failure does not fall back to legacy Anthropic path
  - Helps support the no-fallback claim, but is not an agent-spawn success test
- `materialize_provider_returns_no_credentials_for_known_provider_without_auth`
  - Provider seam unit test
  - Shows `openai` without runtime creds errors when no mock registry provider is present
  - Supports the claim that the agent-spawn success case depends on the injected mock registry provider

Assessment:

- Relevant explicit-openai / no-Anthropic-key / agent-spawn test names currently present:
  - `agent_explicit_provider_routes_to_openai_provider`
  - `agent_parent_inherits_provider_openai_dispatch`
  - `teamcreate_mixed_providers_per_agent_dispatch`
- Exact `M9-10`-equivalent coverage already exists: `yes`, via `agent_explicit_provider_routes_to_openai_provider`
- Weak / indirect / overlapping coverage also exists: `yes`
- Some current tests prove routing or no-fallback properties without independently proving the full no-key spawn-success contract; those remain supportive, not primary

## M9-10 Coverage Matrix

| Required case | Existing test name(s) | Status | Basis for classification | Likely assertion needed if follow-up execution is required |
|---|---|---|---|---|
| `ANTHROPIC_API_KEY` absent + explicit `openai` + mock registry provider -> agent succeeds | `agent_explicit_provider_routes_to_openai_provider` | `COVERED` | Test helper clears Anthropic/OpenAI env, isolates `HOME`, injects tracking OpenAI provider, executes `AgentTool` with explicit `provider: "openai"`, and asserts success plus sentinel output | None required for behavior; only optional rename if execution wants ticket-specific naming |
| no Anthropic-key-related failure appears on that path | `agent_explicit_provider_routes_to_openai_provider`; supported by `agent_tool.rs:293-315` | `COVERED` | Success under isolated no-key env is incompatible with hitting the `"No Anthropic credentials available..."` branch; resolved provider is mocked OpenAI and the test finishes successfully | Optional explicit negative-string assertion on `result.content` only if a more literal ticket-style assertion is desired |
| no real OpenAI call required / mocks feasible | `agent_explicit_provider_routes_to_openai_provider`; `materialize_provider_returns_no_credentials_for_known_provider_without_auth` | `COVERED` | Tracking provider is registry-injected, invocation count is asserted, `OPENAI_API_KEY` is cleared, temporary `HOME` prevents auth-store reuse, and seam unit test shows OpenAI materialization fails without creds when no registry mock exists | None required; current counter/sentinel assertion surface is already stable |

## Mocking / Assertion Surface Feasibility

- Existing fake OpenAI provider pattern can be reused directly: `yes`
  - `TrackingStreamingProvider` + `make_tracking_openai_registry(...)` is already in the exact local test module
- `ANTHROPIC_API_KEY` can be deterministically cleared in current helpers: `yes`
  - `with_isolated_provider_auth(...)` clears env keys and points `HOME` at a temp directory, which also suppresses persisted auth-store reuse
- Most stable assertion strategy:
  - keep `with_isolated_provider_auth(...)`
  - use explicit `"provider": "openai"`
  - inject tracking registry provider
  - assert `!result.is_error`
  - assert exact sentinel output
  - assert mock invocation counter is `1`
- Real credentials are avoidable: `yes`
- `#[ignore]` appears necessary: `no`
  - current matching tests are plain `#[test]` and use only local mocks

## Likely Smallest Edit Surface For Execution

- If ticket execution is treated as audit-only / validation-only: no code edit required
- If execution insists on a new ticket-specific test name despite existing exact coverage:
  - smallest correct edit surface is `src-rust/crates/query/src/agent_tool.rs`
  - not `src-rust/crates/query/tests/`, because that directory is absent
- Production code changes appear unnecessary: `yes`

## Validation Readiness

- Likely execution shape: `audit-only / validation-only`
- Current narrow validation target supported by repo reality:
  - `agent_explicit_provider_routes_to_openai_provider`
- Alternative substrings named in the prompt are not currently present:
  - `no_key_agent_spawn` -> not present
  - `anthropic_key_absent_openai` -> not present
- If follow-up execution chooses not to rename the existing test, the stable filter should target the current exact test name rather than an imagined new substring

## Drift Found

- Minor path drift:
  - authority-hinted `src-rust/crates/query/tests/` does not exist
  - equivalent and stronger-than-minimum local test surface exists in `src-rust/crates/query/src/agent_tool.rs`
- Repo-noise drift:
  - many unrelated untracked docs/report artifacts and `src-rust/target/`
  - not a blocker for preflight, but should be called out in later patch-hygiene / review basis
- No structural drift found in:
  - agent spawn flow name/location
  - provider seam symbols
  - fake OpenAI injection path
  - assertion surface needed for this ticket

## Blockers

- No structural blocker found
- No agent-path behavior blocker found
- No fake-provider seam blocker found
- No assertion-surface blocker found

## Notes

- `TASK-M9-10` appears already satisfied by existing read-only evidence
- The strongest evidence is the existing test `agent_explicit_provider_routes_to_openai_provider` in `src-rust/crates/query/src/agent_tool.rs`
- That test is not merely abstract explicit-provider routing:
  - it explicitly clears `ANTHROPIC_API_KEY`
  - it avoids real OpenAI credentials
  - it uses a mock registry provider
  - it proves successful agent spawn completion
- Because the exact behavior is already covered, the most likely next-step after this preflight is a narrow validation-only pass against the existing test
