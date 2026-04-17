# TASK-M9-07 Execution Report

## Ticket
- `TASK-M9-07 — Root missing registry -> legacy path test`

## Timestamp UTC
- `20260413T095809Z`

## Branch
- `feature/provider-resolution-seam`

## Repo State Summary
- `git branch --show-current` matched the expected branch: `feature/provider-resolution-seam`
- `git status --short --branch` showed one tracked ticket-local modification after execution: `src-rust/crates/query/src/lib.rs`
- `git diff --name-only` after execution showed only `src-rust/crates/query/src/lib.rs`
- `git diff --cached --name-only` remained empty
- substantial unrelated untracked workspace noise remained under `docs/archive/reports/` and `src-rust/target/`; it stayed outside the tracked review basis

## Authority Reviewed
- `AGENTS.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md`
- `docs/archive/reports/TASK-M9-07_PREFLIGHT_REPORT_20260413T093641Z.md`
- `docs/archive/reports/TASK-M9-06_EXECUTION_REPORT_20260413T085936Z.md`
- `docs/archive/reports/TASK-M9-04_COMMIT_VERIFICATION_REPORT_20260413T075907Z.md`

## Preflight Input Used
- preflight verdict carried into execution: `READY-WITH-NOTES`
- preflight-established execution surface was respected:
  - `src-rust/crates/query/src/lib.rs`
  - local `#[cfg(test)]` module only
- preflight-established root-path reality remained current:
  - `run_query_loop(...)` uses registry-backed dispatch only inside `if let Some(ref registry) = config.provider_registry`
  - `provider_registry: None` falls through to `client.create_message_stream(...)`

## Current Code Reality Re-confirmed
- `QueryConfig::provider_registry` still defaults to `None`
- the root branch in `run_query_loop(...)` remained unchanged:
  - registry-backed resolution/materialization branch at `src-rust/crates/query/src/lib.rs:874`
  - legacy Anthropic client call at `src-rust/crates/query/src/lib.rs:1291`
- `src-rust/crates/query/tests/` still does not exist and was not created
- worker-path missing-registry coverage from `TASK-M9-06` remained untouched

## Implemented Test Changes
- added local test-only helpers in `src-rust/crates/query/src/lib.rs` for:
  - isolated auth environment setup
  - minimal `ToolContext` construction with `provider_registry: None`
  - current-thread runtime wrapper for `run_query_loop(...)`
- added root closure test:
  - `provider_registry_none_uses_legacy_anthropic_client_path`
- no production logic was changed

## Exact Legacy-Path / Assertion Strategy Used
- invoked the root query path through `run_query_loop(...)`
- constructed `ToolContext.config.provider = Some("openai")`
- set `QueryConfig.model = "gpt-4o"`
- left `QueryConfig.provider_registry = None`
- passed an `AnthropicClient` built from `ClientConfig::default()` with an empty API key
- used the deterministic `AnthropicClient::create_message_stream(...)` empty-key auth path as the proof signal
- asserted the outcome was `QueryOutcome::Error(...)`
- asserted the error text matched the Anthropic-client-specific auth signature:
  - `Authentication error: No API key for the selected model.`
  - `Model 'gpt-4o' is an OpenAI model.`
  - `Use --provider openai or set OPENAI_API_KEY.`
- this proves the root path used the legacy Anthropic client even though the parent config requested `openai`; if registry-backed routing had been taken, the result shape would not be this Anthropic empty-key auth hint

## Corrective Patch During Execution
- first narrow validation failed on a test-harness-only import path:
  - expected: local test compiles with the new helper
  - found: `ClientConfig` is not re-exported at `claurst_api` crate root
  - smallest fix applied: imported `claurst_api::client::ClientConfig` in the local test module and retried
- no scope widening was required

## Commands Run
- `git branch --show-current`
- `git status --short --branch`
- `git diff --name-only`
- `git diff --cached --name-only`
- `git log --oneline --decorate -n 20`
- `sed -n '1,260p' AGENTS.md`
- `rg -n "TASK-M9-07|M9-07" docs/Current/MPWO_WORK_ORDER_PACK.md`
- `nl -ba docs/Current/MPWO_WORK_ORDER_PACK.md | sed -n '1656,1668p'`
- `sed -n '1,260p' docs/archive/reports/TASK-M9-07_PREFLIGHT_REPORT_20260413T093641Z.md`
- `sed -n '1,260p' docs/archive/reports/TASK-M9-06_EXECUTION_REPORT_20260413T085936Z.md`
- `sed -n '1,260p' docs/archive/reports/TASK-M9-04_COMMIT_VERIFICATION_REPORT_20260413T075907Z.md`
- `rg -n "provider_registry|run_query_loop|make_config|cfg\\(test\\)|provider_registry_none|legacy|create_message_stream|resolve_provider_identity|materialize_provider" src-rust/crates/query/src/lib.rs`
- `nl -ba src-rust/crates/query/src/lib.rs | sed -n '90,220p'`
- `nl -ba src-rust/crates/query/src/lib.rs | sed -n '660,740p'`
- `nl -ba src-rust/crates/query/src/lib.rs | sed -n '840,920p'`
- `nl -ba src-rust/crates/query/src/lib.rs | sed -n '1270,1320p'`
- `nl -ba src-rust/crates/query/src/lib.rs | sed -n '1880,2160p'`
- `nl -ba src-rust/crates/query/src/lib.rs | sed -n '2160,2405p'`
- `nl -ba src-rust/crates/api/src/lib.rs | sed -n '392,455p'`
- `nl -ba src-rust/crates/api/src/lib.rs | sed -n '600,660p'`
- `nl -ba src-rust/crates/tools/src/lib.rs | sed -n '216,280p'`
- `nl -ba src-rust/crates/core/src/lib.rs | sed -n '706,780p'`
- `nl -ba src-rust/crates/query/src/agent_tool.rs | sed -n '730,930p'`
- `nl -ba src-rust/crates/query/src/agent_tool.rs | sed -n '900,980p'`
- `cargo test -p claurst-query -- provider_registry_none`
- `cargo test -p claurst-query -- provider_registry_none`
- `cargo test -p claurst-query -- query`
- `date -u +%Y%m%dT%H%M%SZ`
- `git status --short --branch src-rust/crates/query/src/lib.rs docs/archive/reports`
- `git diff -- src-rust/crates/query/src/lib.rs`
- `rg -n "provider_registry_none_uses_legacy_anthropic_client_path|run_root_query|with_isolated_provider_auth|make_tool_context" src-rust/crates/query/src/lib.rs`

## Validation Result
- narrow validation:
  - first run `cargo test -p claurst-query -- provider_registry_none` -> `FAIL`
  - cause: local test module referenced `claurst_api::ClientConfig` instead of `claurst_api::client::ClientConfig`
  - second run after the smallest corrective patch -> `PASS`
  - result summary: `1 passed; 0 failed; 0 ignored; 0 measured; 107 filtered out`
- broader nearby validation:
  - `cargo test -p claurst-query -- query` -> `PASS`
  - result summary: `2 passed; 0 failed; 0 ignored; 0 measured; 106 filtered out`
- non-blocking out-of-scope warning on validation runs:
  - unused import `Role` in `src-rust/crates/query/src/compact.rs`

## Files Changed
- `src-rust/crates/query/src/lib.rs`

## Verdict
- `PASS`

## Notes
- execution stayed inside the ticket-owned `lib.rs` `#[cfg(test)]` surface
- no production files were changed
- no new integration test harness was created
- review basis is the active unstaged diff for `src-rust/crates/query/src/lib.rs`; unrelated untracked workspace/report noise remained reported but untouched
