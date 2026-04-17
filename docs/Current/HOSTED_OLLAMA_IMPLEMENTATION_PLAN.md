# Hosted Ollama Implementation Plan

Branch: `feature/provider-resolution-seam`

## Plan Summary

- The live seam is already correct in `src-rust/crates/query/src/lib.rs:860-876`; the hosted Ollama failure is localized below it.
- `api_base` normalization should live in `materialize_provider()`, not in the generic `OpenAiCompatProvider` builder layer. The normalization is a seam-level interpretation of `ProviderConfig.api_base` for one provider, and putting it in `with_base_url()` would hide an Ollama-specific rewrite inside a generic adapter.
- `OLLAMA_API_KEY` support should live in a combination of `AuthStore::api_key_for()` and `materialize_provider()`. `AuthStore` should own the precedence rule `env -> auth store`, while `materialize_provider()` should own the Ollama-specific rebuild that preserves the key while applying the normalized hosted base. `registry.rs` does not need to change for this narrow seam fix.
- The fix stays Ollama-specific by branching only on `provider_id == "ollama"` for base normalization and 404 classification. No generic cross-provider URL rewriting, no D2 behavior, no UI, no refactor of the broader provider stack.

## Files To Change

- `src-rust/crates/query/src/provider_resolution.rs`
  Current hotspot: `materialize_provider()` at lines `157-207`, especially the runtime-provider selection at `164-170`, the `if let Some(override_base)` branch at `172-195`, and the `"ollama"` match arm at `177-181`.
  Planned change: add a private Ollama-only base normalizer and an Ollama-specific materialization path that applies normalized hosted base plus optional key, while preserving unauthenticated local fallback.
- `src-rust/crates/core/src/auth_store.rs`
  Current hotspot: `api_key_for()` at lines `82-126`, including the stored-first branch and the env-var match table.
  Planned change: add `ollama -> OLLAMA_API_KEY` support and make Ollama use env-first precedence, with stored-key fallback when the env var is absent or empty.
- `src-rust/crates/api/src/error_handling.rs`
  Current hotspot: `parse_error_response()` at lines `77-145`, specifically the generic `404 => ModelNotFound { model: "unknown" }` branch at `138-142`.
  Planned change: add an Ollama-specific endpoint-shape 404 exception so `{"error":"path \"/api/v1/chat/completions\" not found"}` surfaces as a raw 404/configuration-style error instead of `ModelNotFound("unknown")`.

## Proposed Implementation Steps

1. In `provider_resolution.rs`, add a small private helper that normalizes Ollama `api_base` values to the strict OpenAI-compatible root:
   - `https://ollama.com/api` -> `https://ollama.com/v1`
   - `https://ollama.com/api/v1` -> `https://ollama.com/v1`
   - `http://localhost:11434` -> `http://localhost:11434/v1`
   - never emit `/api/v1/...`
2. Update `materialize_provider()` so the Ollama path is handled before the current generic override replacement logic:
   - load the effective Ollama key through `AuthStore::load().api_key_for("ollama")`
   - if `provider_id == "ollama"` and an `api_base` override exists, rebuild the provider with `openai_compat_providers::ollama()`, then apply `.with_base_url(normalized_base)` and `.with_api_key(key)` when a key exists
   - if `provider_id == "ollama"` and no key exists, still allow the local unauthenticated provider path
   - leave `lmstudio` and `llamacpp` on the existing logic
3. Update `AuthStore::api_key_for()` so Ollama is the only provider with `env -> store` precedence:
   - check `OLLAMA_API_KEY` first
   - if absent or empty, fall back to stored `ApiKey`
   - if neither exists, return `None`
   - keep all other providers on their existing precedence rules
4. Update `parse_error_response()` so Ollama 404 endpoint-shape failures are not classified as model lookup failures:
   - if `provider == ollama`, `status == 404`, and the body/message matches the path-shape signature (`path ... not found`, including the hosted `/api/v1/...` case), return `ProviderError::Other { status: Some(404), ... }` with the raw path message preserved
   - keep all non-Ollama 404s on the existing `ModelNotFound` path
5. Add focused unit coverage in the touched files only:
   - `provider_resolution.rs`: pure normalization helper tests proving `/api` and `/api/v1` collapse to `/v1`
   - `auth_store.rs`: Ollama env-precedence tests and stored-key fallback tests
   - `error_handling.rs`: Ollama endpoint-shape 404 test proving it no longer returns `ModelNotFound { model: "unknown" }`

## Validation Plan

- Cargo checks from `src-rust`:

```bash
cargo check -p claurst-core
cargo check -p claurst-api
cargo check -p claurst-query
cargo check --workspace
```

- Direct hosted sanity check:

```bash
OLLAMA_API_KEY="$(tr -d '\r\n' < /home/jordi/claurst/docs/Current/ollamafreekey.txt)"
curl -sS -o /tmp/ollama-tags.out -w '%{http_code}\n' \
  -H "Authorization: Bearer $OLLAMA_API_KEY" \
  https://ollama.com/api/tags

curl -sS -o /tmp/ollama-chat.out -w '%{http_code}\n' \
  -H "Authorization: Bearer $OLLAMA_API_KEY" \
  -H 'Content-Type: application/json' \
  -d '{"model":"gemma3:4b","messages":[{"role":"user","content":"Reply with exactly OK"}],"stream":false}' \
  https://ollama.com/v1/chat/completions
```

Success criteria: both return `200`; no probe uses `/api/v1/chat/completions`.

- Live Claurst smoke test through the seam:

```bash
OLLAMA_API_KEY="$(tr -d '\r\n' < /home/jordi/claurst/docs/Current/ollamafreekey.txt)"
RUST_LOG=debug OLLAMA_API_KEY="$OLLAMA_API_KEY" \
  cargo run -q -p claurst -- \
  --print \
  --provider ollama \
  --model gemma3:4b \
  --api-base https://ollama.com/api \
  --max-turns 1 \
  "Reply with exactly OK"
```

Success criteria: request still resolves through `provider=ollama`, returns `OK`, and does not fail with `Model not found: unknown`.

## Risks / Ambiguities

- A seam-local fix in `materialize_provider()` will not normalize `OLLAMA_HOST=https://ollama.com/api` in unrelated code paths that rely only on the default factory. That is a separate scope expansion if needed.
- The startup registry and provider health listing will remain local-leaning for Ollama; this plan only fixes live request materialization through the seam.
- `auth_store` env-var tests will need serialized env handling to avoid cross-test interference.
- If the hosted Ollama API returns a different 404 body shape than the currently observed `path "... not found"` payload, the Ollama-specific classifier may need one more pattern, but that still belongs in the same narrow file/function.

## Explicit Stop Points For Approval Before Editing

1. Approve the narrow file scope before editing: `provider_resolution.rs`, `auth_store.rs`, and `error_handling.rs` only.
2. Approve the placement decisions before editing:
   - Ollama `api_base` normalization in `materialize_provider()`
   - Ollama key precedence in `AuthStore`
   - no generic builder-layer rewrite
   - no `registry.rs` change unless implementation proves it is strictly necessary
3. If implementation shows `registry.rs`, `openai_compat.rs`, or `openai_compat_providers.rs` must change after all, stop and get explicit approval before widening scope.
4. After code changes compile, stop for approval before running the live hosted sanity check and the live Claurst smoke test with the Ollama secret.
