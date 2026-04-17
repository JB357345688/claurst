# TASK-M9-08 Execution Report

## Ticket
- `TASK-M9-08 — Root registry + resolution failure -> hard error test`

## Timestamp UTC
- `2026-04-13T12:45:07Z`

## Branch
- `feature/provider-resolution-seam`

## Repo State Summary
- `git branch --show-current` -> `feature/provider-resolution-seam`
- `git diff --name-only` before report writes -> `src-rust/crates/query/src/lib.rs`
- `git diff --cached --name-only` before report writes -> empty
- `git status --short --branch` shows no tracked baseline drift beyond the active ticket file and substantial unrelated untracked workspace/report/build noise already present in the repo
- Review basis for this execution is the active unstaged diff in `src-rust/crates/query/src/lib.rs`, with report artifacts written separately under `docs/archive/reports/`

## Authority Reviewed
- `AGENTS.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md`
- `docs/archive/reports/TASK-M9-08_PREFLIGHT_REPORT_20260413T123418Z.md`
- `docs/archive/reports/TASK-M9-07_COMMIT_VERIFICATION_REPORT_20260413T115639Z.md`
- `docs/archive/reports/TASK-M9-07_EXECUTION_REPORT_20260413T095809Z.md`

## Preflight Input Used
- Preflight verdict re-used: `READY-WITH-NOTES`
- Preflight-owned execution surface re-confirmed:
  - `src-rust/crates/query/src/lib.rs` local `#[cfg(test)]` module only
  - no new `src-rust/crates/query/tests/` harness
  - no production code changes indicated
- Hosted Ollama compatibility baseline preserved

## Current Code Reality Re-Confirmed
- `run_query_loop(...)` still enters the registry-backed branch when `config.provider_registry` is `Some(...)`
- failure from `resolve_provider_identity(...)` still returns early as `QueryOutcome::Error(ClaudeError::Api(...))`
- failure from `materialize_provider(...)` still returns early as `QueryOutcome::Error(ClaudeError::Api(...))`
- the legacy Anthropic client path remains below that branch and is not used on registry-backed seam failure
- root-path test helpers already present in `lib.rs` were reused:
  - `with_isolated_provider_auth(...)`
  - `make_tool_context(...)`
  - `run_root_query(...)`

## Implemented Test Changes
- Added one root-path test in `src-rust/crates/query/src/lib.rs` local test module:
  - `provider_registry_some_resolution_failure_does_not_fallback_to_legacy_anthropic`
- Test setup:
  - `config.provider_registry = Some(Arc::new(claurst_api::ProviderRegistry::new()))`
  - `tool_ctx.config.provider` supplied through `run_root_query(config, Some("openai"))`
  - `config.model = "gpt-4o"`
  - isolated auth with both `ANTHROPIC_API_KEY` and `OPENAI_API_KEY` absent

## Exact No-Fallback / Assertion Strategy Used
- Positive proof of seam failure:
  - asserted returned root error contains `No credentials available for provider 'openai'`
- Negative proof of no legacy Anthropic fallthrough:
  - asserted returned root error does not contain `Authentication error: No API key for the selected model.`
  - asserted returned root error does not contain `Model 'gpt-4o' is an OpenAI model.`
  - asserted returned root error does not contain `Use \`--provider openai\` or set OPENAI_API_KEY.`
- This proves the registry-backed root branch was used and the call did not silently fall through to the legacy Anthropic client path

## Commands Run
- `git branch --show-current`
- `git status --short --branch`
- `git diff --name-only`
- `git diff --cached --name-only`
- `git log --oneline --decorate -n 20`
- `sed -n '1,260p' AGENTS.md`
- `sed -n '1,260p' docs/Current/MPWO_WORK_ORDER_PACK.md`
- `sed -n '1,260p' docs/archive/reports/TASK-M9-08_PREFLIGHT_REPORT_20260413T123418Z.md`
- `sed -n '1,260p' docs/archive/reports/TASK-M9-07_COMMIT_VERIFICATION_REPORT_20260413T115639Z.md`
- `sed -n '1,260p' docs/archive/reports/TASK-M9-07_EXECUTION_REPORT_20260413T095809Z.md`
- `rg -n ... src-rust/crates/query/src/lib.rs`
- `sed -n '840,930p' src-rust/crates/query/src/lib.rs`
- `sed -n '150,230p' src-rust/crates/query/src/provider_resolution.rs`
- `sed -n '2060,2425p' src-rust/crates/query/src/lib.rs`
- `sed -n '1,170p' src-rust/crates/api/src/registry.rs`
- `cargo test -p claurst-query -- provider_registry_some_resolution_failure`
- `cargo test -p claurst-query -- query`

## Validation Result
- Narrow validation: `PASS`
  - `cargo test -p claurst-query -- provider_registry_some_resolution_failure`
  - result: `1 passed; 0 failed; 0 ignored; 108 filtered out`
- Broader nearby regression: `PASS`
  - `cargo test -p claurst-query -- query`
  - result: `2 passed; 0 failed; 0 ignored; 107 filtered out`
- Non-blocking out-of-scope warning remained present:
  - unused import `Role` in `src-rust/crates/query/src/compact.rs`

## Files Changed
- `src-rust/crates/query/src/lib.rs`
- `docs/archive/reports/TASK-M9-08_EXECUTION_REPORT_20260413T124507Z.md`
- `docs/archive/reports/TASK-M9-08_CLOSEOUT_REPORT_20260413T124507Z.md`

## Verdict
- `IMPLEMENTED AND VALIDATED`

## Notes
- Scope stayed inside the active ticket and did not reopen `M9-01` through `M9-07`
- No worker-path tests were added
- No provider-resolution seam-only tests were used as closure proof
- No production code was changed
- No new integration-test harness was created
