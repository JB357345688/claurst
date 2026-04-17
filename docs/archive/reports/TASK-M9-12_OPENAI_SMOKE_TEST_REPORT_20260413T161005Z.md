# M9-12 OpenAI Smoke Test Report

- Report timestamp: `2026-04-13T16:10:05Z`
- Repo: `/home/jordi/claurst`
- Scope: manual smoke test only

## Commands Run

```bash
cd /home/jordi/claurst/src-rust && cargo run -q -p claurst -- --help
cd /home/jordi/claurst/src-rust && cargo run -q -p claurst -- --provider openai --verbose --allowed-tools Agent --max-turns 3 -p "You are running a smoke test. Do not answer directly. Your first and only tool call must be Agent. Spawn one child agent with description 'smoke test' and prompt 'Reply with exactly CHILD_OK and nothing else.' Wait for it to finish, then reply with exactly PARENT_OK: CHILD_OK."
cd /home/jordi/claurst/src-rust && cargo run -q -p claurst -- --provider openai --max-tokens 1024 --verbose --allowed-tools Agent --max-turns 3 -p "You are running a smoke test. Do not answer directly. Your first and only tool call must be Agent. Spawn one child agent with description 'smoke test' and prompt 'Reply with exactly CHILD_OK and nothing else.' Wait for it to finish, then reply with exactly PARENT_OK: CHILD_OK."
```

## Outcome

- Agent spawn happened: `No`
- OpenAI path succeeded: `No`

## Blocker

- Run 1 reached OpenAI and selected `provider=openai` with model `gpt-4o-mini`, but failed before agent spawn with:
  - `Error 400: max_tokens is too large: 32000. This model supports at most 16384 completion tokens`
- Run 2 reached OpenAI and selected `provider=openai` with model `gpt-4o-mini`, but failed before agent spawn with:
  - `Invalid request: We could not parse the JSON body of your request.`

## Minimal CLI Command Identified

```bash
cd /home/jordi/claurst/src-rust && cargo run -q -p claurst -- --provider openai -p "<prompt>"
```
