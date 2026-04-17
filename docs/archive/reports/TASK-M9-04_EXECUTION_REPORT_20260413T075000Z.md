# TASK-M9-04 Execution Report

## Ticket
`TASK-M9-04`

## Timestamp UTC
`20260413T075000Z`

## Branch
`feature/provider-resolution-seam`

## Repo State Summary
- Required repo-state checks matched the ticket expectation before edits:
  - `git branch --show-current` -> `feature/provider-resolution-seam`
  - `git diff --name-only` -> empty
  - `git diff --cached --name-only` -> empty
- `git status --short --branch` showed a clean tracked baseline plus substantial pre-existing untracked workspace/report/build noise under `docs/Current/`, `docs/archive/reports/`, and `src-rust/target/`.
- After execution, the only tracked source diff is `src-rust/crates/query/src/agent_tool.rs`.
- Review basis for this ticket remains the active unstaged diff in `src-rust/crates/query/src/agent_tool.rs`; unrelated untracked noise remains present and untouched.

## Authority Reviewed
- `AGENTS.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md`
- `docs/archive/reports/TASK-M9-04_PREFLIGHT_REPORT_20260413T073825Z.md`
- `docs/archive/reports/TASK-M9-03_COMMIT_VERIFICATION_REPORT_20260413T072303Z.md`
- `docs/archive/reports/TASK-M9-03_EXECUTION_REPORT_20260413T064215Z.md`
- `docs/archive/reports/TASK-M9-03_CLOSEOUT_REPORT_20260413T070716Z.md`

## Preflight Input Used
- Preflight verdict re-used as authority input: `READY-WITH-NOTES`.
- Reconfirmed repo reality still matches the preflight findings:
  - local test surface is `src-rust/crates/query/src/agent_tool.rs`
  - `src-rust/crates/query/tests/` still does not exist and was not needed
  - parent provider still lives in `ToolContext.config.provider`
  - child input can omit both `provider` and `model`
  - the prior local inheritance test shape still existed and remained too weak because it used `max_turns: 0`
- Hosted Ollama compatibility baseline preserved.

## Current Code Reality Re-Confirmed
- `AgentTool::execute(...)` still resolves child model as explicit model or `DEFAULT_MODEL`.
- Provider hint ordering still matches preflight:
  - explicit child provider
  - model prefix
  - parent `ctx.config.provider`
- Shared resolution/materialization seam still runs through:
  - `resolve_provider_identity(...)`
  - `materialize_provider(...)`
- The existing M9-03 fake provider seam remained available and reusable in the local `#[cfg(test)]` module.

## Implemented Test Changes
- Edited only `src-rust/crates/query/src/agent_tool.rs`.
- Replaced the weak inheritance closure test `agent_tool_inherits_parent_provider_without_network` with the stronger, filter-friendly test `agent_parent_inherits_provider_openai_dispatch`.
- Removed the now-unused local `make_openai_registry()` helper and its `OpenAiProvider` test import after switching the inheritance test to the tracking fake provider path.
- No production code was changed.

## Exact Inheritance / Fake / Assertion Strategy
- Parent `ToolContext.config.provider` is set to `Some("openai")`.
- Child agent input omits both `provider` and `model`.
- The test uses `make_tracking_openai_registry(...)` to register the fake `openai` provider in `ProviderRegistry`.
- `max_turns` is set to `1` so the nested query path reaches `create_message_stream(...)`.
- The fake provider emits a deterministic sentinel response: `inherited openai provider sentinel`.
- Assertions:
  - invocation counter equals `1`
  - final `ToolResult.content` equals the sentinel response
  - result is not an error
- This proves inherited OpenAI dispatch directly rather than inferring success from a no-error path.

## Commands Run
- `git branch --show-current`
- `git status --short --branch`
- `git diff --name-only`
- `git diff --cached --name-only`
- `git log --oneline --decorate -n 20`
- `sed -n '1,220p' AGENTS.md`
- `sed -n '1,260p' docs/Current/MPWO_WORK_ORDER_PACK.md`
- `sed -n '1,240p' docs/archive/reports/TASK-M9-04_PREFLIGHT_REPORT_20260413T073825Z.md`
- `sed -n '1,220p' docs/archive/reports/TASK-M9-03_COMMIT_VERIFICATION_REPORT_20260413T072303Z.md`
- `rg -n "TrackingOpenAiProvider|make_tracking_openai_registry|make_tool_context|run_agent_tool|with_isolated_provider_auth|agent_explicit_provider_routes_to_openai_provider|agent_tool_inherits_parent_provider_without_network" src-rust/crates/query/src/agent_tool.rs`
- `sed -n '232,320p' src-rust/crates/query/src/agent_tool.rs`
- `sed -n '680,735p' src-rust/crates/query/src/lib.rs`
- `sed -n '700,790p' src-rust/crates/query/src/agent_tool.rs`
- `git diff -- src-rust/crates/query/src/agent_tool.rs`
- `cargo test -p claurst-query -- agent_parent_inherits_provider`
- `cargo test -p claurst-query -- agent_tool`

## Validation Result
- Required narrow validation: `PASS`
  - Command: `cargo test -p claurst-query -- agent_parent_inherits_provider`
  - Result: `1 passed; 0 failed; 0 ignored; 106 filtered out`
- Optional nearby regression: `PASS`
  - Command: `cargo test -p claurst-query -- agent_tool`
  - Result: `3 passed; 0 failed; 0 ignored; 104 filtered out`
- Non-blocking out-of-scope warning remains present on both runs:
  - unused import `Role` in `src-rust/crates/query/src/compact.rs`

## Files Changed
- `src-rust/crates/query/src/agent_tool.rs`

## Verdict
`PASS`

## Notes
- Scope stayed inside the preflight-approved local test module only.
- No new integration-test harness was created.
- No production logic, team-runner path, or hosted-Ollama behavior was reopened.
