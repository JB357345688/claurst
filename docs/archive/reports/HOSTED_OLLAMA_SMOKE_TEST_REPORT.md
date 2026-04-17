# HOSTED_OLLAMA_SMOKE_TEST_REPORT

## Objective
Run the smallest safe manual hosted-Ollama smoke test against the live provider-resolution seam on `feature/provider-resolution-seam` without printing the secret, writing it into git-tracked files, or committing temporary smoke-test config.

## Branch
`feature/provider-resolution-seam`

## Baseline commit reference
`255e3c7391eb1b02e79188bdf37792ccc86a7544`

## Direct hosted API sanity check result
- Result: `PASS`
- Command succeeded with `HTTP_STATUS:200` against `https://ollama.com/api/tags`.
- Returned a non-empty model catalog (`model_count: 36`).
- Sample returned models:
  - `cogito-2.1:671b`
  - `qwen3-vl:235b-instruct`
  - `devstral-small-2:24b`
  - `gemma3:4b`
  - `gemma3:27b`

## Claurst smoke test method
- Method: headless `cargo run -q -p claurst -- --print ...` so the request goes through the live `run_query_loop()` provider-resolution seam in [src-rust/crates/query/src/lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:860).
- Provider selection path exercised:
  - explicit CLI provider `--provider ollama`
  - CLI `--api-base https://ollama.com/api`, which writes to `config.provider_configs[provider].api_base` in [src-rust/crates/cli/src/main.rs](/home/jordi/claurst/src-rust/crates/cli/src/main.rs:488)
  - materialization via [src-rust/crates/query/src/provider_resolution.rs](/home/jordi/claurst/src-rust/crates/query/src/provider_resolution.rs:157)
- Request used:
  - model: `gemma3:4b`
  - prompt: `Reply with exactly OK`
  - max turns: `1`
- Note: the current Claurst headless path uses streaming `run_query_loop()` rather than a non-streaming one-shot path; see [src-rust/crates/cli/src/main.rs](/home/jordi/claurst/src-rust/crates/cli/src/main.rs:1113).

## Temporary config/env changes used
- No persistent config files were created or modified.
- No `~/.claurst/auth.json` or `~/.claurst/settings.json` was present before the test.
- The only temporary state used was shell-local environment:
  - `OLLAMA_API_KEY` loaded from `/home/jordi/claurst/docs/Current/ollamafreekey.txt`
  - `RUST_LOG=debug` for smoke-test evidence
- The hosted Ollama base was supplied transiently via CLI:
  - `--api-base https://ollama.com/api`

## Whether provider resolution selected ollama
Yes.

Evidence from the live Claurst run:
- `DEBUG Dispatching to registry-backed provider provider=ollama model=gemma3:4b`

Relevant code:
- resolution call in [src-rust/crates/query/src/lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:860)
- materialization call in [src-rust/crates/query/src/lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:872)

## Whether materialization succeeded
Yes, to the point of dispatch.

Evidence:
- Claurst reached `Dispatching to registry-backed provider provider=ollama model=gemma3:4b`.
- A real HTTPS connection was then opened to `ollama.com`.

Relevant code:
- `materialize_provider()` in [src-rust/crates/query/src/provider_resolution.rs](/home/jordi/claurst/src-rust/crates/query/src/provider_resolution.rs:157)
- `runtime_provider_for()` in [src-rust/crates/api/src/registry.rs](/home/jordi/claurst/src-rust/crates/api/src/registry.rs:73)

## Whether the request hit hosted Ollama
Yes.

Evidence from the live Claurst run:
- `DEBUG starting new connection: https://ollama.com/`
- `DEBUG connecting to 34.36.133.15:443`
- `DEBUG connected to 34.36.133.15:443`

Direct diagnostic follow-up also confirmed the hosted endpoint behavior:
- `POST https://ollama.com/api/v1/chat/completions` returned:
  - `HTTP_STATUS:404`
  - `{"error":"path \"/api/v1/chat/completions\" not found"}`

## Response outcome
- Direct hosted sanity check:
  - success
- Claurst hosted smoke test:
  - failed
  - surfaced as:
    - `ERROR Provider stream failed provider=ollama error=[ollama] Model not found: unknown`
    - `Error: API error: [ollama] Model not found: unknown`

## Failure point, if any
Yes.

Primary failure point:
- `base URL wiring`

Reason:
- The live Ollama provider is an OpenAI-compatible provider that posts to `.../chat/completions`; see [src-rust/crates/api/src/providers/openai_compat.rs](/home/jordi/claurst/src-rust/crates/api/src/providers/openai_compat.rs:370).
- `materialize_provider()` appends `/v1` to the configured `api_base`; see [src-rust/crates/query/src/provider_resolution.rs](/home/jordi/claurst/src-rust/crates/query/src/provider_resolution.rs:176).
- With `--api-base https://ollama.com/api`, Claurst therefore targets:
  - `https://ollama.com/api/v1/chat/completions`
- A direct probe to that exact hosted path returned:
  - `404`
  - `{"error":"path \"/api/v1/chat/completions\" not found"}`

Why Claurst reported `Model not found: unknown` instead of the raw 404 path error:
- HTTP 404 is generically mapped to `ProviderError::ModelNotFound { model: "unknown" }` in [src-rust/crates/api/src/error_handling.rs](/home/jordi/claurst/src-rust/crates/api/src/error_handling.rs:138).

Secondary limitation found during inspection:
- The current Ollama credential path is not first-class for hosted env-key usage:
  - `AuthStore::api_key_for()` has no `ollama -> OLLAMA_API_KEY` env fallback; see [src-rust/crates/core/src/auth_store.rs](/home/jordi/claurst/src-rust/crates/core/src/auth_store.rs:107).
  - `provider_from_key()` has no `ollama` arm; see [src-rust/crates/api/src/registry.rs](/home/jordi/claurst/src-rust/crates/api/src/registry.rs:26).
  - the default Ollama provider factory itself is local-only and does not read `OLLAMA_API_KEY`; see [src-rust/crates/api/src/providers/openai_compat_providers.rs](/home/jordi/claurst/src-rust/crates/api/src/providers/openai_compat_providers.rs:17).

Failure classification against the requested buckets:
- base URL wiring: `FAILED`
- credential injection: `not conclusively exercised end-to-end because the hosted chat-completions path itself 404s first, but the current code path is not properly wired for ollama env-key materialization`
- model name mismatch: `not primary`; `gemma3:4b` is present in `/api/tags`
- provider config mismatch: `no`; the temporary CLI config behaved as intended and routed to `ollama`
- runtime provider materialization path: `resolution/materialization reached dispatch`, but the keyed hosted-ollama materialization path is incomplete in code

## Commands run (redacted)
```bash
command -v jq

test -s /home/jordi/claurst/docs/Current/ollamafreekey.txt && echo PRESENT

find "$HOME/.claurst" -maxdepth 1 -type f \( -name 'auth.json' -o -name 'settings.json' \) -printf '%f\n'

OLLAMA_API_KEY="$(tr -d '\r\n' < /home/jordi/claurst/docs/Current/ollamafreekey.txt)"
curl -sS -o "$TMP" -w '%{http_code}' \
  -H "Authorization: Bearer $OLLAMA_API_KEY" \
  https://ollama.com/api/tags

RUST_LOG=debug OLLAMA_API_KEY="$OLLAMA_API_KEY" \
  cargo run -q -p claurst -- \
  --print \
  --provider ollama \
  --model gemma3:4b \
  --api-base https://ollama.com/api \
  --max-turns 1 \
  "Reply with exactly OK"

OLLAMA_API_KEY="$(tr -d '\r\n' < /home/jordi/claurst/docs/Current/ollamafreekey.txt)"
curl -sS -o "$TMP" -w '%{http_code}' \
  -H "Authorization: Bearer $OLLAMA_API_KEY" \
  -H 'Content-Type: application/json' \
  -d '{"model":"gemma3:4b","messages":[{"role":"user","content":"Reply with exactly OK"}],"stream":false}' \
  https://ollama.com/api/v1/chat/completions
```

## Revert steps
- No persistent smoke-test config was written, so no config revert is required.
- The shell-local environment variables were only set inside single commands.
- Temporary response files were created with `mktemp` and removed immediately.
- If you repeat the test manually in an interactive shell, revert by running:
  - `unset OLLAMA_API_KEY`
  - `unset RUST_LOG`

## Whether any source files were changed
- No source files were changed.
- No config files were changed.
- Only this report file was created:
  - `docs/archive/reports/HOSTED_OLLAMA_SMOKE_TEST_REPORT.md`

## Ready for a permanent hosted-Ollama config fix: yes/no
Yes.

Reason:
- The seam itself is live and exercised.
- The direct hosted key is valid.
- The live Claurst path already proves provider resolution and dispatch to `ollama`.
- The blocking issue is now sharply localized:
  - hosted endpoint/base-URL compatibility for the Ollama provider path
  - plus missing first-class Ollama env-key materialization for hosted usage
