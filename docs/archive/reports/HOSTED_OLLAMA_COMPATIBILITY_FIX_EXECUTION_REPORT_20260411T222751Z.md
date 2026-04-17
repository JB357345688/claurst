# Ticket

`HOSTED-OLLAMA-COMPATIBILITY-FIX`

# Branch

`feature/provider-resolution-seam`

# Baseline commit

`255e3c7391eb1b02e79188bdf37792ccc86a7544`

# Files changed

- `src-rust/crates/query/src/provider_resolution.rs`
- `src-rust/crates/core/src/auth_store.rs`
- `src-rust/crates/api/src/error_handling.rs`
- `docs/archive/reports/HOSTED_OLLAMA_COMPATIBILITY_FIX_EXECUTION_REPORT_20260411T222751Z.md`

# What was implemented

- Verified before editing that stored-key fallback for provider id `ollama` could be completed without adding an `ollama` arm to `provider_from_key()` and without touching `registry.rs`.
- Kept Ollama-specific `api_base` normalization in `materialize_provider()` and added a narrow helper that normalizes hosted roots to the strict OpenAI-compatible root:
  - `.../api` -> `.../v1`
  - `.../api/v1` -> `.../v1`
  - never emits `/api/v1/...`
- Changed the seam so `materialize_provider()` builds the Ollama provider directly from `AuthStore::api_key_for("ollama")`, preserving:
  - `OLLAMA_API_KEY` env precedence
  - auth-store fallback
  - unauthenticated local fallback when no key is present
- Kept the fix Ollama-specific and did not add any generic cross-provider URL rewriting or builder-layer rewrite.
- Updated `AuthStore::api_key_for()` so Ollama now uses the approved precedence:
  - env
  - auth store
  - none
- Updated HTTP error classification so Ollama endpoint-shape 404s such as `path "/api/v1/chat/completions" not found` no longer surface as `ModelNotFound { model: "unknown" }`.
- Added narrow unit coverage for:
  - Ollama base normalization
  - Ollama env-vs-store precedence
  - Ollama endpoint-shape 404 handling

# Validation commands run

- `cd src-rust && cargo check -p claurst-core`
- `cd src-rust && cargo check -p claurst-api`
- `cd src-rust && cargo check -p claurst-query`
- `cd src-rust && cargo check --workspace`
- `cd src-rust && cargo test -p claurst-core auth_store`
- `cd src-rust && cargo test -p claurst-api error_handling`
- `cd src-rust && cargo test -p claurst-query provider_resolution`

# Validation results

- `cargo check -p claurst-core` passed.
- `cargo check -p claurst-api` passed.
- `cargo check -p claurst-query` passed.
- `cargo check --workspace` passed.
- `cargo test -p claurst-core auth_store` passed.
- `cargo test -p claurst-api error_handling` passed.
- `cargo test -p claurst-query provider_resolution` passed.
- Two pre-existing warnings were observed during test builds and did not fail validation:
  - unused import in `crates/core/tests/parity_smoke.rs`
  - unused import in `crates/query/src/compact.rs`

# Deviations, if any

- Stopped at the approved validation gate after compile and targeted unit-test validation.
- Did not run the direct hosted sanity check or the live Claurst smoke test because those were explicitly gated for approval before using the Ollama secret.

# Blockers, if any

- Approval is still required before running:
  - the direct hosted sanity check
  - the live Claurst smoke test with `OLLAMA_API_KEY`
