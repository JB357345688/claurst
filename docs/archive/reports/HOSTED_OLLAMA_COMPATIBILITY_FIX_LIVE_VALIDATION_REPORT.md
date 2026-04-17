# Objective

Run the remaining live validation for `HOSTED-OLLAMA-COMPATIBILITY-FIX` using shell-local environment only, without printing the Ollama API key or writing it into any tracked file, and confirm:

- direct hosted auth and endpoint success
- live Claurst seam routing through provider `ollama`
- materialization success
- request delivery to hosted Ollama
- end-to-end response success

# Branch

`feature/provider-resolution-seam`

# Baseline commit

`255e3c7391eb1b02e79188bdf37792ccc86a7544`

# Direct hosted sanity check result

Result: `PASS`

Checks run:

- `GET https://ollama.com/api/tags`
  - auth header supplied from shell-local `OLLAMA_API_KEY`
  - `HTTP_STATUS:200`
  - returned a non-empty hosted model catalog
  - confirmed `gemma3:4b` is present in the hosted catalog
- `POST https://ollama.com/v1/chat/completions`
  - auth header supplied from shell-local `OLLAMA_API_KEY`
  - request body used model `gemma3:4b`
  - `HTTP_STATUS:200`
  - response content returned `OK`

Direct hosted chat response excerpt:

```json
{"model":"gemma3:4b","choices":[{"message":{"role":"assistant","content":"OK\n"}}]}
```

# Claurst smoke test command used

```bash
OLLAMA_API_KEY="$(tr -d '\r\n' < /home/jordi/claurst/docs/Current/ollamafreekey.txt)" \
RUST_LOG=debug \
cargo run -q -p claurst -- \
  --print \
  --provider ollama \
  --model gemma3:4b \
  --api-base https://ollama.com/api \
  --max-turns 1 \
  "Reply with exactly OK" \
  > /tmp/hosted_ollama_claurst.log 2>&1
```

Exit status: `0`

# Whether provider resolution selected ollama

Yes.

Evidence from `/tmp/hosted_ollama_claurst.log`:

- `DEBUG Dispatching to registry-backed provider provider=ollama model=gemma3:4b`

# Whether materialization succeeded

Yes.

Evidence:

- The run reached registry-backed dispatch for `provider=ollama`.
- No `Provider materialization failed` log line appeared.
- The process exited with status `0`.

# Whether the request hit hosted Ollama

Yes.

Evidence from `/tmp/hosted_ollama_claurst.log`:

- `DEBUG starting new connection: https://ollama.com/`
- `DEBUG connecting to 34.36.133.15:443`
- `DEBUG connected to 34.36.133.15:443`

This confirms the live Claurst smoke test reached hosted Ollama over HTTPS rather than a local Ollama server.

# Response outcome

End-to-end result: `PASS`

Observed outcome:

- direct hosted sanity check succeeded
- live Claurst seam smoke test succeeded
- final Claurst output was:

```text
OK
```

# Failure classification, if any

None.

- hosted auth: `PASS`
- endpoint shape: `PASS`
- provider resolution to `ollama`: `PASS`
- materialization: `PASS`
- hosted request delivery: `PASS`
- end-to-end response: `PASS`

# Whether any source files changed

No source files changed during live validation.

Only this report file was created:

- `docs/archive/reports/HOSTED_OLLAMA_COMPATIBILITY_FIX_LIVE_VALIDATION_REPORT.md`

# Ready to close: yes/no

Yes.
