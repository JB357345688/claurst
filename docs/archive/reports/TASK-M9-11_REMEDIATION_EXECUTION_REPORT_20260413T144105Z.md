# TASK-M9-11 Remediation Execution Report

## Task
`TASK-M9-11 — Remediate remaining hardcoded Anthropic client construction`

## Timestamp UTC
`20260413T144105Z`

## Branch
`feature/provider-resolution-seam`

## Repo State Summary
- `git branch --show-current` -> `feature/provider-resolution-seam`
- `git status --short --branch` before remediation -> branch matched expectation; no tracked staged or unstaged diffs; substantial unrelated untracked workspace/report/build noise remained under `.codex`, `docs/`, and `src-rust/target/`
- `git diff --name-only` before remediation -> empty
- `git diff --cached --name-only` before remediation -> empty
- `git log --oneline --decorate -n 20` before remediation -> `HEAD` was `5e77652 (HEAD -> feature/provider-resolution-seam) TASK-M9-09 prove TeamCreate mixed providers dispatch per agent`
- Review basis for this remediation: active unstaged diff only
- Unrelated untracked noise was left untouched throughout

## Authority Reviewed
- `AGENTS.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md`
- `docs/archive/reports/TASK-M9-11_PREFLIGHT_REPORT_20260413T142100Z.md`
- `docs/archive/reports/TASK-M9-10_EXECUTION_REPORT_20260413T140203Z.md`
- `docs/archive/reports/TASK-M9-09_COMMIT_VERIFICATION_REPORT_20260413T132904Z.md`

## Preflight Blocker Being Addressed
- `TASK-M9-11` preflight was `BLOCKED`
- Blocking defect:
  - live hardcoded `AnthropicClient::new()` in `src-rust/crates/query/src/agent_tool.rs`
  - foreground/background shared sub-agent path
  - `init_team_swarm_runner()` path
- Preflight-confirmed non-blocker preserved:
  - live direct `ANTHROPIC_API_KEY` env-var reads were already absent in `agent_tool.rs`

## Current Code Reality Re-Confirmed
- `AgentTool::execute()` still resolved provider identity via `resolve_provider_identity(...)` and `materialize_provider(...)`
- `init_team_swarm_runner()` still resolved provider identity via the same seam
- Remaining problem before remediation:
  - both paths still branched on `target.provider_id == "anthropic"`
  - both paths still built anthropic-specific `ClientConfig`
  - both paths still directly called `AnthropicClient::new(...)`
- `run_query_loop(...)` still preferred the registry-backed provider path whenever `provider_registry` was present

## Exact Remediation Performed
- Removed the remaining anthropic-specific client-construction branches from [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:280)
- Updated the foreground/background/shared sub-agent path and `init_team_swarm_runner()` path to call `run_query_loop(None, ...)` because these paths already pass:
  - resolved `target.provider_id`
  - resolved `target.model_id`
  - populated `provider_registry`
- Applied the smallest compile-required support change in [lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:675) so `run_query_loop(...)` accepts `Option<&AnthropicClient>` and only requires a legacy client when the non-registry path is used
- Updated the existing legacy caller in [cron_scheduler.rs](/home/jordi/claurst/src-rust/crates/query/src/cron_scheduler.rs:85) to pass `Some(client.as_ref())`

## Commands Run
- `git branch --show-current`
- `git status --short --branch`
- `git diff --name-only`
- `git diff --cached --name-only`
- `git log --oneline --decorate -n 20`
- `sed -n '1,240p' AGENTS.md`
- `sed -n '1713,1778p' docs/Current/MPWO_WORK_ORDER_PACK.md`
- `sed -n '1,240p' docs/archive/reports/TASK-M9-11_PREFLIGHT_REPORT_20260413T142100Z.md`
- `sed -n '1,240p' docs/archive/reports/TASK-M9-10_EXECUTION_REPORT_20260413T140203Z.md`
- `sed -n '1,240p' docs/archive/reports/TASK-M9-09_COMMIT_VERIFICATION_REPORT_20260413T132904Z.md`
- `rg -n "AnthropicClient::new|resolve_provider_identity|materialize_provider|init_team_swarm_runner|provider_id == \"anthropic\"|api_key_for\\(\"anthropic\"\\)" src-rust/crates/query/src/agent_tool.rs`
- `sed -n '1,260p' src-rust/crates/query/src/provider_resolution.rs`
- `nl -ba src-rust/crates/query/src/lib.rs | sed -n '850,930p'`
- `sed -n '1,240p' src-rust/crates/api/src/provider.rs`
- `sed -n '1,260p' src-rust/crates/api/src/registry.rs`
- `grep -c 'AnthropicClient::new' src-rust/crates/query/src/agent_tool.rs`
- `grep -n 'AnthropicClient::new' src-rust/crates/query/src/agent_tool.rs`
- `rg -n 'std::env::var\("ANTHROPIC_API_KEY"\)|std::env::remove_var\("ANTHROPIC_API_KEY"\)' src-rust/crates/query/src/agent_tool.rs`
- `cargo test -p claurst-query -- agent_tool`
- `cargo test -p claurst-query -- provider_registry_none`
- `cargo test -p claurst-query -- mixed_providers_per_agent`
- `date -u +%Y%m%dT%H%M%SZ`
- `git diff --stat -- src-rust/crates/query/src/agent_tool.rs src-rust/crates/query/src/lib.rs src-rust/crates/query/src/cron_scheduler.rs`

## Validation Result
- Hardcoded-removal checks:
  - `grep -c 'AnthropicClient::new' src-rust/crates/query/src/agent_tool.rs` -> `0`
  - `grep -n 'AnthropicClient::new' src-rust/crates/query/src/agent_tool.rs` -> no matches
  - `rg -n 'std::env::var\("ANTHROPIC_API_KEY"\)|std::env::remove_var\("ANTHROPIC_API_KEY"\)' src-rust/crates/query/src/agent_tool.rs` -> no matches
- Relevant nearby tests:
  - `cargo test -p claurst-query -- agent_tool` -> `PASS`
    - `4 passed; 0 failed; 0 ignored; 106 filtered out`
  - `cargo test -p claurst-query -- provider_registry_none` -> `PASS`
    - `1 passed; 0 failed; 0 ignored; 109 filtered out`
  - `cargo test -p claurst-query -- mixed_providers_per_agent` -> `PASS`
    - `1 passed; 0 failed; 0 ignored; 109 filtered out`
- Non-blocking pre-existing warning remained in test runs:
  - unused import `Role` in `src-rust/crates/query/src/compact.rs`
- Non-blocking environment observation:
  - transient cargo file-lock waits occurred and resolved without intervention

## Files Changed
- `src-rust/crates/query/src/agent_tool.rs`
- `src-rust/crates/query/src/lib.rs`
- `src-rust/crates/query/src/cron_scheduler.rs`

## Remaining `AnthropicClient::new()` Count
- In `src-rust/crates/query/src/agent_tool.rs`: `0`

## Remaining Live `ANTHROPIC_API_KEY` Read Count
- In `src-rust/crates/query/src/agent_tool.rs`: `0`

## Verdict
`REMEDIATED`

## Notes
- The intended remediation logic stayed confined to `agent_tool.rs`
- Deviation from single-file ownership:
  - [lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:675) changed only for the minimal compile-required optional legacy-client parameter
  - [cron_scheduler.rs](/home/jordi/claurst/src-rust/crates/query/src/cron_scheduler.rs:85) changed only to satisfy that compile fix at an existing legacy caller
- No final `TASK-M9-11` verification ticket was started in this session
- No `M9-12` work was started
