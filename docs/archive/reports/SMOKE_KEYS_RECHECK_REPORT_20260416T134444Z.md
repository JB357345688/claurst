# Smoke Keys Recheck Report

1. Branch / HEAD
   - `feature/provider-resolution-seam`
   - `038f3c20e01a96eec6397d506b477a461166f762`

2. OpenAI key source found
   - `OPENAI_API_KEY`: present in env

3. OpenAI smoke result
   - Connectivity probe to `https://api.openai.com/v1/models`: `HTTP 200`
   - Claurst smoke: `PASS`
   - Expected text observed: `PARENT_OK: CHILD_OK`

4. Ollama key source found
   - `OLLAMA_API_KEY`: present in env
   - Fallback file `/home/jordi/claurst/docs/Current/ollamafreekey.txt`: missing, not needed

5. Ollama direct hosted sanity result
   - `GET https://ollama.com/api/tags`: `HTTP 200`
   - `POST https://ollama.com/v1/chat/completions`: `HTTP 200`
   - Expected hosted reply observed: `OK`

6. Ollama Claurst smoke result
   - Claurst hosted-Ollama smoke: `PASS`
   - Expected text observed: `OK`
   - Log evidence: `provider=ollama` selected
   - Log evidence: outbound HTTPS reached `https://ollama.com/` and connected to `34.36.133.15:443`
   - Materialization: no explicit success log line observed; inferred from successful dispatch and final `OK`

7. Failure attribution for each path
   - OpenAI: no credential/auth/runtime failure in the successful unrestricted run; initial sandbox-only DNS restriction prevented outbound resolution before rerun
   - Hosted Ollama: no credential/auth/runtime failure in the successful final run; initial sandbox-only DNS restriction prevented outbound resolution before rerun, and one first-pass direct POST failed due to shell-quoting malformed JSON before corrected rerun

8. Final summary
   - `OPENAI: PASS`
   - `HOSTED_OLLAMA: PASS`
