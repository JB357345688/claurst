# TASK-M9-02 Execution Report

## Ticket
`TASK-M9-02`

## Timestamp UTC
`20260413T060834Z`

## Branch
`feature/provider-resolution-seam`

## Repo State Summary
- Initial mandatory repo-state checks matched the expected branch `feature/provider-resolution-seam`.
- Initial tracked worktree was clean; `git diff --name-only` and `git diff --cached --name-only` were empty before execution.
- Pre-existing untracked workspace noise remained present under `.codex/`, `docs/Current/`, `docs/Orchestrator_planning/`, `docs/archive/provider_orchestrator/`, `docs/archive/reports/`, and `src-rust/target/`; it was left untouched and kept outside the tracked ticket patch basis.
- Final tracked diff after execution is limited to [src-rust/crates/query/src/provider_resolution.rs](/home/jordi/claurst/src-rust/crates/query/src/provider_resolution.rs:255).
- Final staged diff remained empty.

## Authority Reviewed
- `AGENTS.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md`
- `docs/archive/reports/TASK-M9-02_PREFLIGHT_REPORT_20260413T054655Z.md`
- `docs/archive/reports/TASK-M9-01_EXECUTION_REPORT_20260413T050804Z.md`

## Preflight Input Used
- Preflight verdict carried into execution: `READY-WITH-NOTES`
- Preflight-established execution surface respected:
  - owned edit surface stayed inside `src-rust/crates/query/src/provider_resolution.rs`
  - edits stayed inside the local `#[cfg(test)]` module only
  - no production code changes were made
  - no blockers were found
- Hosted Ollama compatibility baseline preserved.

## Current Code Reality Re-confirmed
- `materialize_provider(...)` still prioritizes `runtime_provider_for(...)` over registry fallback for non-ollama providers.
- `materialize_provider(...)` still contains explicit `api_base` override branches for:
  - `"lmstudio" | "lm-studio"`
  - `"llamacpp" | "llama-cpp"`
- Direct `base_url` inspection is still unavailable through `Arc<dyn LlmProvider>`.
- Existing materialize coverage still lacked explicit tests for:
  - auth-store precedence over registry
  - `lm-studio` override branch
  - `llama-cpp` override branch
  - known-provider no-credentials path

## Implemented Test Additions
- Added local deterministic test helpers in [src-rust/crates/query/src/provider_resolution.rs](/home/jordi/claurst/src-rust/crates/query/src/provider_resolution.rs:275):
  - `TestProvider`
  - `EnvGuard`
  - `with_isolated_provider_auth(...)`
  - `run_async(...)`
  - `assert_unavailable_reason(...)`
- Added the required materialize coverage tests in the same local test module:
  - `materialize_provider_prefers_auth_store_provider_over_registry`
  - `materialize_provider_applies_lm_studio_api_base_override`
  - `materialize_provider_applies_llama_cpp_api_base_override`
  - `materialize_provider_returns_no_credentials_for_known_provider_without_auth`

## Exact Assertion Strategy
- Auth-store precedence over registry:
  - isolated `HOME` and provider env state
  - wrote an `openai` API key through `AuthStore::set(...)` so `runtime_provider_for(...)` had to reload from disk
  - registered a local `TestProvider` fallback with the same provider id but distinct `name() == "Registry OpenAI"`
  - asserted the returned provider had `id() == "openai"` and `name() == "OpenAI"`, proving the auth-backed runtime provider won instead of the registry double
- `api_base` override for `lm-studio`:
  - used canonical provider id `lm-studio`
  - forced `LM_STUDIO_HOST` to a local-looking invalid host (`http://localhost:bad`) so the non-overridden path would not satisfy the remote no-key behavior
  - supplied `provider_configs["lm-studio"].api_base = "https://example.invalid/lm-studio"`
  - asserted `health_check()` returned `ProviderStatus::Unavailable { reason: "No API key configured" }`, which is the observable remote/no-key effect of the override branch
- `api_base` override for `llama-cpp`:
  - used canonical provider id `llama-cpp`
  - forced `LLAMA_CPP_HOST` to `http://localhost:bad`
  - supplied `provider_configs["llama-cpp"].api_base = "https://example.invalid/llama-cpp"`
  - asserted the same remote/no-key `health_check()` result
- Known-provider no-credentials:
  - isolated `HOME` and cleared `OPENAI_API_KEY`
  - used an empty registry
  - asserted `Err(ProviderResolutionError::NoCredentials("openai"))`

## Commands Run
- `git branch --show-current`
- `git status --short --branch`
- `git diff --name-only`
- `git diff --cached --name-only`
- `git log --oneline --decorate -n 20`
- `sed -n '1,260p' AGENTS.md`
- `sed -n '1,260p' docs/Current/MPWO_WORK_ORDER_PACK.md`
- `sed -n '1,260p' docs/archive/reports/TASK-M9-02_PREFLIGHT_REPORT_20260413T054655Z.md`
- `sed -n '1,260p' docs/archive/reports/TASK-M9-01_EXECUTION_REPORT_20260413T050804Z.md`
- `sed -n '1,420p' src-rust/crates/query/src/provider_resolution.rs`
- `sed -n '420,620p' src-rust/crates/query/src/provider_resolution.rs`
- `sed -n '1,220p' src-rust/crates/api/src/registry.rs`
- `sed -n '1,260p' src-rust/crates/core/src/auth_store.rs`
- `sed -n '1,220p' src-rust/crates/api/src/provider.rs`
- `sed -n '680,820p' src-rust/crates/api/src/providers/openai_compat.rs`
- `sed -n '740,860p' src-rust/crates/query/src/agent_tool.rs`
- `sed -n '1,220p' src-rust/crates/api/src/providers/openai_compat_providers.rs`
- `sed -n '1,320p' src-rust/crates/api/src/provider_types.rs`
- `cargo test -p claurst-query -- materialize_provider_`
- `cargo test -p claurst-query -- provider_resolution`

## Validation Result
- `cargo test -p claurst-query -- materialize_provider_` -> `PASS`
  - result: `7 passed; 0 failed; 0 ignored; 0 measured; 100 filtered out`
- `cargo test -p claurst-query -- provider_resolution` -> `PASS`
  - result: `22 passed; 0 failed; 0 ignored; 0 measured; 85 filtered out`
- Non-blocking out-of-scope warning remained in `crates/query/src/compact.rs` for unused import `Role`; no action taken because it is outside `TASK-M9-02`.

## Files Changed
- `src-rust/crates/query/src/provider_resolution.rs`

## Verdict
`PASS`

## Notes
- The execution stayed inside the preflight-owned local test surface.
- No production seams were widened.
- The tracked review basis is scope-clean for `TASK-M9-02`; only pre-existing untracked workspace/report noise remains around it.
