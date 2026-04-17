# TASK-M7-06 Execution Report

- Ticket ID: `TASK-M7-06`
- Verdict: `DONE`
- Branch: `feature/provider-resolution-seam`

## Files Edited

- `src-rust/crates/query/src/provider_resolution.rs`

## What Was Implemented

Added the required `materialize_provider()` unit coverage inside the existing `#[cfg(test)] mod tests` in `src-rust/crates/query/src/provider_resolution.rs`.

Exact test names added:

- `materialize_provider_returns_openai_target_from_happy_path`
- `materialize_provider_returns_no_credentials_for_unknown_provider`
- `materialize_provider_accepts_ollama_api_base_override`

Supporting test-only helper added in the same test module:

- `provider_identity(...)`

## MPWO Behavior Mapping

- Registry provider found / happy path:
  - `materialize_provider_returns_openai_target_from_happy_path`
- Missing credentials error path:
  - `materialize_provider_returns_no_credentials_for_unknown_provider`
- Ollama `api_base` override happy path:
  - `materialize_provider_accepts_ollama_api_base_override`

## Validation Commands Run

```bash
cd /home/jordi/claurst/src-rust && cargo test -p claurst-query -- provider_resolution
```

## Validation Result

Validation passed.

- `cargo test -p claurst-query -- provider_resolution` succeeded
- Provider-resolution test run result: `18 passed; 0 failed; 0 ignored`
- A pre-existing unrelated warning was emitted from `crates/query/src/compact.rs:1193` for an unused import; this did not affect the ticket result and was not modified

## Assumptions And Notes

- The OpenAI happy-path test keeps its assertion surface narrow and aligned to MPWO: it asserts successful materialization and `target.provider_id == "openai"` without trying to distinguish runtime-credential materialization from injected-registry materialization.
- The missing-credentials assertion uses `matches!(...)` against `ProviderResolutionError::NoCredentials(...)`, per repo reality.
- The Ollama override test intentionally asserts only successful construction and target identity. The current public API does not expose a non-invasive way to inspect the applied base URL from `Arc<dyn LlmProvider>` without adding production seams, which is out of scope.
- Existing untracked repo noise under `docs/` and `src-rust/target/` remained untouched.

## Production Code

Production code was not modified.

- Only the existing test module in `src-rust/crates/query/src/provider_resolution.rs` was extended.
- No integration tests were added.
- No production behavior, provider construction logic, auth-store logic, or normalization logic was changed.

## Hosted Ollama Non-Regression

Hosted Ollama compatibility baseline preserved

Explicit verification performed during execution:

- `normalize_ollama_api_base(...)` remains present in `src-rust/crates/query/src/provider_resolution.rs`
- Ollama materialization still consults `AuthStore::load().api_key_for(ProviderId::OLLAMA)`
- Environment-first precedence for `AuthStore::api_key_for("ollama")` remains intact in `src-rust/crates/core/src/auth_store.rs`
- Existing hosted Ollama normalization tests remain present and unweakened
