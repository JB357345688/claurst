# TASK-M9-09 Execution Report

## Ticket
- `TASK-M9-09 — TeamCreate mixed providers integration test`

## Timestamp UTC
- `20260413T132046Z`

## Branch
- `feature/provider-resolution-seam`

## Repo State Summary
- Required repo-state commands were run first.
- `git branch --show-current` -> `feature/provider-resolution-seam`
- `git status --short --branch` before edits -> no tracked staged/unstaged diffs; substantial unrelated untracked workspace/report/build noise was present and left untouched.
- `git diff --name-only` before edits -> empty
- `git diff --cached --name-only` before edits -> empty
- `git log --oneline --decorate -n 20` confirmed `HEAD` at `63a8485` (`TASK-M9-08 prove root registry failure does not fallback to legacy anthropic`).
- Repo state after implementation remained scope-clean on tracked diffs:
  - `git diff --name-only` -> `src-rust/crates/query/src/agent_tool.rs`
  - `git diff --cached --name-only` -> empty

## Authority Reviewed
- `AGENTS.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md`

## Preflight Input Used
- `docs/archive/reports/TASK-M9-09_PREFLIGHT_REPORT_20260413T130702Z.md`
- `docs/archive/reports/TASK-M9-08_COMMIT_VERIFICATION_REPORT_20260413T130029Z.md`
- `docs/archive/reports/TASK-M9-03_EXECUTION_REPORT_20260413T064215Z.md`
- `docs/archive/reports/TASK-M9-04_COMMIT_VERIFICATION_REPORT_20260413T075907Z.md`

## Current Code Reality Re-Confirmed
- The authoritative preflight findings still matched repo reality.
- `src-rust/crates/tools/tests/` and `src-rust/crates/query/tests/` are absent and were not created.
- `TeamCreateTool` still lives in `src-rust/crates/tools/src/team_tool.rs`.
- Team agent provider/model overrides are still forwarded through `AgentRunParams.provider_override` and `model_override`.
- The actual team-agent provider resolution/materialization still occurs in the injected query runner in `src-rust/crates/query/src/agent_tool.rs`.
- The smallest correct edit surface remained the local `#[cfg(test)]` module in `src-rust/crates/query/src/agent_tool.rs`.
- Hosted Ollama compatibility baseline preserved.

## Implemented Test Changes
- Edited only `src-rust/crates/query/src/agent_tool.rs`.
- Kept all source edits inside the existing local `#[cfg(test)]` module.
- Replaced the test-local OpenAI-only tracking fake with a local generic tracking streaming provider so the same seam can register both `openai` and `google` without widening production code.
- Kept the existing OpenAI helper pattern in place via `make_tracking_openai_registry(...)`.
- Added a mixed registry helper that registers:
  - fake `openai` streaming provider
  - fake `google` streaming provider
- Added local test-only runner helpers:
  - one-time `init_team_swarm_runner_once()` mirroring the repo’s `OnceLock` init pattern
  - `run_team_create_tool(...)`
- Extended the isolated auth helper to clear `GOOGLE_API_KEY` in addition to the existing local auth isolation.
- Added closure test:
  - `teamcreate_mixed_providers_per_agent_dispatch`

## Exact Mixed-Provider / Assertion Strategy Used
- Used the real `TeamCreateTool` path, not a forwarding-only stub.
- Initialized the team swarm runner exactly once through a local `OnceLock` helper to avoid `register_agent_runner(...)` double-registration panic.
- Constructed one team with two agents:
  - `agent-a` -> `provider: "openai"`, `model: "gpt-4o"`
  - `agent-b` -> `provider: "google"`, `model: "gemini-2.5-flash"`
- Registered fake OpenAI and fake Google streaming providers through `ProviderRegistry`.
- Each fake increments its own invocation counter in `create_message_stream(...)`.
- Each fake emits a deterministic provider-specific sentinel text.
- Parsed the returned structured `TeamCreate` JSON payload and asserted:
  - `agent-a` output equals the OpenAI sentinel
  - `agent-b` output equals the Google sentinel
  - `agent-a` output is not the Google sentinel
  - `agent-b` output is not the OpenAI sentinel
  - OpenAI invocation counter == `1`
  - Google invocation counter == `1`
- This proves per-agent provider selection is honored and that no single-provider bleed-through occurred.

## Commands Run
- `git branch --show-current`
- `git status --short --branch`
- `git diff --name-only`
- `git diff --cached --name-only`
- `git log --oneline --decorate -n 20`
- `sed -n '1,240p' AGENTS.md`
- `sed -n '1,260p' docs/Current/MPWO_WORK_ORDER_PACK.md`
- `sed -n '1,260p' docs/archive/reports/TASK-M9-09_PREFLIGHT_REPORT_20260413T130702Z.md`
- `sed -n '1,260p' docs/archive/reports/TASK-M9-08_COMMIT_VERIFICATION_REPORT_20260413T130029Z.md`
- `sed -n '1,260p' docs/archive/reports/TASK-M9-03_EXECUTION_REPORT_20260413T064215Z.md`
- `sed -n '1,260p' docs/archive/reports/TASK-M9-04_COMMIT_VERIFICATION_REPORT_20260413T075907Z.md`
- local `rg` / `sed` inspection over:
  - `src-rust/crates/query/src/agent_tool.rs`
  - `src-rust/crates/tools/src/team_tool.rs`
  - `src-rust/crates/query/src/provider_resolution.rs`
  - `src-rust/crates/cli/src/main.rs`
- Validation:
  - `cargo test -p claurst-query -- mixed_providers_per_agent`
  - `cargo test -p claurst-query -- agent_tool`

## Validation Result
- Narrow validation -> `PASS`
  - `test agent_tool::tests::teamcreate_mixed_providers_per_agent_dispatch ... ok`
  - `1 passed; 0 failed; 0 ignored; 109 filtered out`
- Broader nearby regression -> `PASS`
  - `4 passed; 0 failed; 0 ignored; 106 filtered out`
- Observed non-blocking out-of-scope warning on both runs:
  - unused import `Role` in `src-rust/crates/query/src/compact.rs`

## Files Changed
- `src-rust/crates/query/src/agent_tool.rs`
- `docs/archive/reports/TASK-M9-09_EXECUTION_REPORT_20260413T132046Z.md`
- `docs/archive/reports/TASK-M9-09_CLOSEOUT_REPORT_20260413T132046Z.md`

## Verdict
- `IMPLEMENTED_AND_VALIDATED`

## Notes
- Scope stayed inside the single active ticket.
- No production code was changed.
- No new tests directories were created.
- No M9-01 through M9-08 logic was reopened.
- No M9-10 through M9-12 work was started.
- Review basis for this execution session is the active unstaged diff.
- Unrelated untracked workspace/report/build noise remains present and outside the active tracked ticket diff.
