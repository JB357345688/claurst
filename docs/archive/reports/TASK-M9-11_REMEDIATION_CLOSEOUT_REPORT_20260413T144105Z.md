# TASK-M9-11 Remediation Closeout Report

## Task
`TASK-M9-11 — Remediate remaining hardcoded Anthropic client construction`

## Timestamp UTC
`20260413T144105Z`

## Final Changed File List
- `src-rust/crates/query/src/agent_tool.rs`
- `src-rust/crates/query/src/lib.rs`
- `src-rust/crates/query/src/cron_scheduler.rs`

## Summary of Remediation
- Removed the remaining live `AnthropicClient::new()` construction from `agent_tool.rs`
- Kept the foreground/background/shared sub-agent path and `init_team_swarm_runner()` path on the already-materialized provider seam by calling `run_query_loop(None, ...)` when `provider_registry` is present
- Added the smallest supporting query-loop change so legacy Anthropic client access is optional and still required only at legacy non-registry callers

## Validation Commands and Outcomes
- `grep -c 'AnthropicClient::new' src-rust/crates/query/src/agent_tool.rs` -> `0`
- `grep -n 'AnthropicClient::new' src-rust/crates/query/src/agent_tool.rs` -> no matches
- `rg -n 'std::env::var\("ANTHROPIC_API_KEY"\)|std::env::remove_var\("ANTHROPIC_API_KEY"\)' src-rust/crates/query/src/agent_tool.rs` -> no matches
- `cargo test -p claurst-query -- agent_tool` -> `PASS`
  - `4 passed; 0 failed; 0 ignored; 106 filtered out`
- `cargo test -p claurst-query -- provider_registry_none` -> `PASS`
  - `1 passed; 0 failed; 0 ignored; 109 filtered out`
- `cargo test -p claurst-query -- mixed_providers_per_agent` -> `PASS`
  - `1 passed; 0 failed; 0 ignored; 109 filtered out`

## Scope Confirmation
- Active review basis: unstaged diff only
- Remediation behavior change is confined to `agent_tool.rs`
- Minimal compile-fix deviation was required:
  - `src-rust/crates/query/src/lib.rs` changed only to make the legacy Anthropic client parameter optional for registry-backed callers
  - `src-rust/crates/query/src/cron_scheduler.rs` changed only to pass `Some(...)` at an existing legacy caller
- No unrelated source, docs, tests, or workspace noise were modified

## Outstanding Notes
- The worktree still contains substantial unrelated untracked `docs/` and `src-rust/target/` noise that predates this task and remains untouched
- Test runs still emit a pre-existing unused-import warning in `src-rust/crates/query/src/compact.rs`
