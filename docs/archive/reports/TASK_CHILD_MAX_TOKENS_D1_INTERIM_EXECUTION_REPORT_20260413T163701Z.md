# TASK_CHILD_MAX_TOKENS_D1_INTERIM_EXECUTION_REPORT

- Task: `D1-safe child max_tokens fallback for spawned agents`
- Timestamp UTC: `2026-04-13T16:37:01Z`
- Branch: `feature/provider-resolution-seam`

## Repo State Summary

- Mandatory preflight commands run:
  - `git branch --show-current`
  - `git status --short --branch`
  - `git diff --name-only`
  - `git diff --cached --name-only`
  - `git log --oneline --decorate -n 20`
- Preflight verdict: target branch matched expectation and the worktree was already dirty before this task.
- Unstaged tracked files already present before remediation:
  - `src-rust/crates/cli/src/main.rs`
  - `src-rust/crates/query/src/agent_tool.rs`
  - `src-rust/crates/query/src/lib.rs`
  - `src-rust/crates/query/src/provider_resolution.rs`
- Cached diff at preflight: none.
- Review basis for this remediation: the narrow delta in `src-rust/crates/query/src/agent_tool.rs` for child-agent `max_tokens` fallback only.

## Exact Bug Being Addressed

- Spawned child agents in `src-rust/crates/query/src/agent_tool.rs` were building `QueryConfig` with `claurst_core::constants::DEFAULT_MAX_TOKENS`.
- `claurst_core::constants::DEFAULT_MAX_TOKENS` remains `32000`.
- Manual OpenAI smoke had already reached provider resolution and agent spawning, then failed because the child request still used `max_tokens=32000`.

## Exact Remediation Performed

- Added a local constant in `src-rust/crates/query/src/agent_tool.rs`:
  - `CHILD_AGENT_FALLBACK_MAX_TOKENS: u32 = 4_096`
- Added a short comment marking that constant as a D1-safe interim fallback for spawned child agents and not the final parent/child token policy.
- Replaced the child-agent `QueryConfig.max_tokens` assignments in `agent_tool.rs` to use the local `4096` fallback instead of the global `32000` default.
- Kept the change local to child-agent construction in `agent_tool.rs`.
- Did not change `claurst_core::constants::DEFAULT_MAX_TOKENS`.
- Did not change root query defaults, provider resolution, parent token behavior, or config schema.

## Files Changed

- `src-rust/crates/query/src/agent_tool.rs`

## Commands Run

- `git branch --show-current`
- `git status --short --branch`
- `git diff --name-only`
- `git diff --cached --name-only`
- `git log --oneline --decorate -n 20`
- `rg -n "DEFAULT_MAX_TOKENS|max_tokens|spawn|child" src-rust/crates/query/src/agent_tool.rs`
- `sed -n '1,260p' src-rust/crates/query/src/agent_tool.rs`
- `sed -n '261,520p' src-rust/crates/query/src/agent_tool.rs`
- `sed -n '520,700p' src-rust/crates/query/src/agent_tool.rs`
- `rg -n "pub const DEFAULT_MAX_TOKENS|DEFAULT_MAX_TOKENS" src-rust/crates/core/src -g '*.rs'`
- `git diff -- src-rust/crates/query/src/agent_tool.rs`
- `cargo test -p claurst-query -- agent_tool`
- `test -n "$OPENAI_API_KEY"`
- `cargo run -q -p claurst -- --help`
- `cargo run -q -p claurst -- --provider openai --model gpt-4o-mini --max-tokens 1024 --verbose --allowed-tools Agent --max-turns 3 -p "You are running a smoke test. Do not answer directly. Your first and only tool call must be Agent. Spawn one child agent with description 'smoke test' and prompt 'Reply with exactly CHILD_OK and nothing else.' Wait for it to finish, then reply with exactly PARENT_OK: CHILD_OK."`
- `cargo run -q -p claurst -- --provider openai --model gpt-4o-mini --max-tokens 1024 --verbose --allowed-tools Agent --max-turns 4 -p "You are running a smoke test. Do not answer directly. Your first and only tool call must be Agent. Spawn exactly one child agent with description 'smoke test', provider 'openai', and model 'gpt-4o-mini'. Use the child prompt 'Reply with exactly CHILD_OK and nothing else.' Wait for it to finish, then reply with exactly PARENT_OK: CHILD_OK."`

## Test Results

- `cargo test -p claurst-query -- agent_tool`
- Result: passed.
- Observed summary: `4 passed; 0 failed`.

## Manual Smoke Retry Result

- Environment assumption check: `OPENAI_API_KEY` was present in the shell.
- First smoke attempt inside sandbox failed on network egress before provider completion.
- First network-enabled retry reached `provider=openai`, executed `Agent`, spawned the child, and no longer failed with `max_tokens is too large: 32000`.
- That first network-enabled retry exposed a separate existing issue where the child used `provider=openai` with default model `claude-opus-4-6`, which OpenAI rejected as `Model not found: unknown`.
- Second network-enabled retry explicitly instructed the parent to set child `provider=openai` and child `model=gpt-4o-mini`.
- Final smoke result: success with terminal output `PARENT_OK: CHILD_OK`.

## Whether The 32000 Child max_tokens Failure Is Eliminated

- Yes.
- The child path no longer failed with `max_tokens is too large: 32000` in the OpenAI smoke retry.
- The successful retry completed the parent and child flow with the child running on `gpt-4o-mini`.

## Verdict

- `REMEDIATED`

## Notes

- Scope stayed narrow to child-agent `max_tokens` fallback logic in `agent_tool.rs`.
- Parent and child token allowance remain conceptually separate; this is an interim D1-safe local fallback only.
- A fuller parent/child token policy remains future work and was not started here.
- The worktree still contains unrelated pre-existing modifications outside this remediation surface.
