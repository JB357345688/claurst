# TASK-M9-11 Preflight Report

- Ticket: `TASK-M9-11`
- Timestamp UTC: `20260413T142100Z`
- Branch: `feature/provider-resolution-seam`
- Verdict: `BLOCKED`

## Repo State Summary

- `git branch --show-current` -> `feature/provider-resolution-seam`
- `git status --short --branch` -> branch matches expectation; no tracked staged or unstaged diffs; untracked repo noise is present under `docs/` and `src-rust/target/`
- `git diff --name-only` -> no output
- `git diff --cached --name-only` -> no output
- `git log --oneline --decorate -n 20` -> `HEAD` is `5e77652 (TASK-M9-09 prove TeamCreate mixed providers dispatch per agent)` and includes accepted baseline commit `b5b6dd4 (TASK-M8-11 reconcile M8 workspace validation and formatting)`

## Authority Reviewed

- Repo authority reviewed: `AGENTS.md`
- Ticket authority reviewed: `docs/Current/MPWO_WORK_ORDER_PACK.md`
- Confirmed from work order:
  - `TASK-M9-11` is a verification-only ticket
  - `M9` validation requires `M8-11` complete
  - `M9-01` through `M9-11` may run in parallel
- Hosted Ollama compatibility baseline preserved as background invariant only; not reopened or widened in this preflight

## Dependency Baseline Confirmed

- Accepted baseline `TASK-M8-11` is present in branch history
- Current tracked baseline is compatible with the user-stated sequence:
  - `M9-01` through `M9-09` complete on branch history
  - `M9-10` treated per prompt as closed on audit/validation basis without source change
- No prior tickets were reopened in this session

## Exact M9-11 Contract

- Objective: verify the three original hardcoded `AnthropicClient::new()` sites no longer exist
- Authority file path: `crates/query/src/agent_tool.rs`
- Repo reality path used for audit: `src-rust/crates/query/src/agent_tool.rs`
- Required checks:
  - zero `AnthropicClient::new()` matches in `agent_tool.rs`
  - zero direct `ANTHROPIC_API_KEY` env-var reads in `agent_tool.rs`
  - verify current production paths route through provider seam behavior instead of hardcoded anthropic construction

## Verified Files / Symbols / Commands

- Files reviewed:
  - `AGENTS.md`
  - `docs/Current/MPWO_WORK_ORDER_PACK.md`
  - `src-rust/crates/query/src/agent_tool.rs`
  - `src-rust/crates/query/src/provider_resolution.rs`
- Symbols / mechanisms reviewed:
  - `AnthropicClient::new`
  - `ANTHROPIC_API_KEY`
  - `resolve_provider_identity`
  - `materialize_provider`
  - `init_team_swarm_runner`
  - `run_in_background`
- Commands reviewed:
  - `git branch --show-current`
  - `git status --short --branch`
  - `git diff --name-only`
  - `git diff --cached --name-only`
  - `git log --oneline --decorate -n 20`
  - `grep -c 'AnthropicClient::new' src-rust/crates/query/src/agent_tool.rs`
  - `grep -n 'AnthropicClient::new' src-rust/crates/query/src/agent_tool.rs`
  - `grep -n 'ANTHROPIC_API_KEY' src-rust/crates/query/src/agent_tool.rs`
  - `rg -n 'std::env::var\("ANTHROPIC_API_KEY"\)|std::env::remove_var\("ANTHROPIC_API_KEY"\)' src-rust/crates/query/src/agent_tool.rs`
  - `rg -n 'AnthropicClient::new|ANTHROPIC_API_KEY|resolve_provider_identity|materialize_provider|init_team_swarm_runner|run_in_background|api_key_for\("anthropic"\)' src-rust/crates/query/src/agent_tool.rs`

## Current Code Reality

- `src-rust/crates/query/src/agent_tool.rs` exists and is the meaningful audit target for the authority path.
- `AgentTool::execute()` now calls `resolve_provider_identity(...)` and `materialize_provider(...)` before launching the sub-agent.
- The background agent path is not a separate constructor path anymore; `run_in_background` reuses the same `AgentTool::execute()` client-creation path.
- `init_team_swarm_runner()` also calls `resolve_provider_identity(...)` and `materialize_provider(...)`.
- Despite the seam calls, both live production paths still branch on `target.provider_id == "anthropic"`, build anthropic-specific `ClientConfig`, and then call `AnthropicClient::new(...)`.
- No live direct `std::env::var("ANTHROPIC_API_KEY")` or `std::env::remove_var("ANTHROPIC_API_KEY")` reads remain in `agent_tool.rs`.
- The only `ANTHROPIC_API_KEY` string occurrence in `agent_tool.rs` is test-only: `EnvGuard::set("ANTHROPIC_API_KEY", None)` in the test helper.

## Hardcoded-Removal Audit

- `AnthropicClient::new()` live-code matches in `src-rust/crates/query/src/agent_tool.rs`: `2`
  - line `318` in `AgentTool::execute()`
  - line `672` in `init_team_swarm_runner()`
- `AnthropicClient::new()` matches in comments/tests/docs only: `0`
- `ANTHROPIC_API_KEY` direct env-var reads in live code: `0`
  - `rg -n 'std::env::var\("ANTHROPIC_API_KEY"\)|std::env::remove_var\("ANTHROPIC_API_KEY"\)' src-rust/crates/query/src/agent_tool.rs` returned no matches
- `ANTHROPIC_API_KEY` string occurrences in comments/tests/docs only: `1`
  - line `965` in test helper `with_isolated_provider_auth()`
- Equivalent anthropic-specific production-path logic remains after seam resolution:
  - line `294` branches on `target.provider_id == "anthropic"`
  - line `298` uses `AuthStore::load().api_key_for("anthropic")`
  - line `643` branches on `target.provider_id == "anthropic"`
  - line `650` uses `AuthStore::load().api_key_for("anthropic")`
- Classification:
  - direct env-var reads are gone from live code
  - hardcoded anthropic client construction is not gone from live code
  - seam routing is partial, not complete; current paths resolve provider identity through the seam, but still instantiate `AnthropicClient` directly instead of consuming the materialized provider object

## M9-11 Verification Matrix

| Required case | Evidence source / command | Status | Basis for classification | Likely follow-up action if execution is required |
|---|---|---|---|---|
| `AnthropicClient::new()` no longer exists in `agent_tool.rs` | `grep -c 'AnthropicClient::new' src-rust/crates/query/src/agent_tool.rs` -> `2`; `grep -n ...` -> lines `318`, `672` | `MISSING` | Two live production call sites remain in active code; current validation target would fail immediately | Verification can run as-is and should report failure; any corrective implementation would need to be limited to `src-rust/crates/query/src/agent_tool.rs` |
| `ANTHROPIC_API_KEY` env var reads no longer exist in `agent_tool.rs` | `rg -n 'std::env::var\("ANTHROPIC_API_KEY"\)|std::env::remove_var\("ANTHROPIC_API_KEY"\)' src-rust/crates/query/src/agent_tool.rs` -> no matches; `grep -n 'ANTHROPIC_API_KEY' ...` -> line `965` test helper only | `VERIFIED` | No live env-var read sites remain; sole file-level occurrence is test-only setup, not production logic | Use a read-specific `rg` check during execution and separately note the test-only string occurrence so raw string grep is not misclassified |

## Likely Smallest Edit Surface For Execution

- Execution surface for `TASK-M9-11` remains pure verification on `src-rust/crates/query/src/agent_tool.rs`
- No source edit belongs inside this verification-only ticket
- If the goal is to make `TASK-M9-11` pass rather than merely execute its verification, the smallest corrective source surface would still be `src-rust/crates/query/src/agent_tool.rs`

## Validation Readiness

- Stable direct validation command for the exact ticket target:
  - `grep -c 'AnthropicClient::new' src-rust/crates/query/src/agent_tool.rs`
- Current result:
  - returns `2`, not `0`
- Stable companion audit commands for env-var-read classification:
  - `rg -n 'std::env::var\("ANTHROPIC_API_KEY"\)|std::env::remove_var\("ANTHROPIC_API_KEY"\)' src-rust/crates/query/src/agent_tool.rs`
  - `grep -n 'ANTHROPIC_API_KEY' src-rust/crates/query/src/agent_tool.rs`
- Current env-read audit result:
  - no live read matches
  - one test-only string occurrence at line `965`

## Drift Found

- Minor path drift only:
  - authority says `crates/query/src/agent_tool.rs`
  - repo reality is `src-rust/crates/query/src/agent_tool.rs`
- Minor implementation-shape drift:
  - the original three hardcoded sites are no longer represented as three separate exact call sites
  - foreground and background agent execution now share one `AgentTool::execute()` client-construction path
  - this drift does not invalidate the verification target; it makes the remaining exact-match count `2` rather than the original `3`
- No blocking file move or rename beyond the path-prefix drift above

## Blockers

- Yes: remaining live hardcoded anthropic construction in production code blocks `TASK-M9-11` from passing as written
- Expected:
  - zero live `AnthropicClient::new()` matches in `agent_tool.rs`
- Found:
  - two live matches at lines `318` and `672`
- Why it blocks:
  - `TASK-M9-11` is verification-only and does not permit source changes, but the current repository state does not satisfy the verification objective
- Blocker type:
  - remaining live hardcoded logic
  - not path drift
  - not verification-target obsolescence

## Notes

- This preflight stayed within `TASK-M9-11` only and did not reopen `M9-01` through `M9-10`
- Current branch state is still suitable for a scope-clean verification pass
- The repo is not structurally ambiguous for `M9-11`; the failure is substantive, not procedural
- Hosted Ollama compatibility baseline preserved
