# TASK-M9-12 Execution Rerun Report

## Ticket
`TASK-M9-12 — Full regression + D1 completion declaration (rerun after formatting remediation)`

## Timestamp UTC
`20260413T153943Z`

## Branch
`feature/provider-resolution-seam`

## Repo State Summary
- `git branch --show-current` -> `feature/provider-resolution-seam`
- `git status --short --branch` at rerun start showed only the formatting-remediation tracked source diff:
  - `src-rust/crates/query/src/agent_tool.rs`
  - `src-rust/crates/query/src/provider_resolution.rs`
- `git diff --cached --name-only` at rerun start -> no output
- `git log --oneline --decorate -n 20` at rerun start -> `HEAD` was `af97a87 (HEAD -> feature/provider-resolution-seam) TASK-M9-11 remove hardcoded anthropic construction from agent tool paths`
- Broad unrelated untracked workspace/report/build noise remained present under `.codex/`, `docs/`, and `src-rust/target/`; left untouched and outside the review basis
- Final tracked diff after corrective rerun work:
  - `src-rust/crates/cli/src/main.rs`
  - `src-rust/crates/query/src/agent_tool.rs`
  - `src-rust/crates/query/src/lib.rs`
  - `src-rust/crates/query/src/provider_resolution.rs`

## Authority Reviewed
- `AGENTS.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md`
- `docs/archive/reports/TASK-M9-12_PREFLIGHT_REPORT_20260413T151224Z.md`
- `docs/archive/reports/TASK-M9-12_EXECUTION_REPORT_20260413T152113Z.md`
- `docs/archive/reports/TASK-M9-12_FORMAT_REMEDIATION_EXECUTION_REPORT_20260413T153001Z.md`
- `docs/archive/reports/TASK-M9-12_FORMAT_REMEDIATION_CLOSEOUT_REPORT_20260413T153001Z.md`
- `docs/archive/reports/TASK-M9-11_COMMIT_VERIFICATION_REPORT_20260413T150533Z.md`

## Preflight Input Used
- `TASK-M9-12_PREFLIGHT_REPORT_20260413T151224Z.md`
- Preflight verdict reused: `READY-WITH-NOTES`
- Preflight reality confirmed during rerun:
  - workspace root is `src-rust/`
  - branch matches expectation
  - hosted Ollama compatibility remains a preserved background invariant only
  - live OpenAI smoke remains conditional on credentials and network

## Formatting Remediation Input Used
- `TASK-M9-12_EXECUTION_REPORT_20260413T152113Z.md`
- `TASK-M9-12_FORMAT_REMEDIATION_EXECUTION_REPORT_20260413T153001Z.md`
- `TASK-M9-12_FORMAT_REMEDIATION_CLOSEOUT_REPORT_20260413T153001Z.md`
- Confirmed rerun started from the expected formatting-only tracked diff in:
  - `src-rust/crates/query/src/agent_tool.rs`
  - `src-rust/crates/query/src/provider_resolution.rs`

## Workspace Root Used
- All cargo validation commands were run from `/home/jordi/claurst/src-rust`
- `src-rust/Cargo.toml` was verified present before execution

## Corrective Patch Summary
- First rerun attempt failed at `cargo build --workspace` because `claurst_query::run_query_loop(...)` now takes `Option<&AnthropicClient>`, while five CLI call sites still passed `&AnthropicClient`
- Smallest fix applied:
  - wrapped five `client_clone.as_ref()` arguments in `Some(...)` in `src-rust/crates/cli/src/main.rs`
- Second rerun attempt passed `build` but failed `cargo test --workspace` on `provider_registry_some_resolution_failure_does_not_fallback_to_legacy_anthropic`
- Root cause:
  - `claurst-query` had three separate test-only env/auth locks in `agent_tool.rs`, `lib.rs`, and `provider_resolution.rs`
  - parallel tests could race on `HOME` and `.claurst/auth.json`, allowing one test's stored OpenAI credential to leak into another
- Smallest fix applied:
  - added one crate-wide test-only provider-auth lock in `src-rust/crates/query/src/lib.rs`
  - updated the three `with_isolated_provider_auth(...)` helpers to use that shared lock
  - retained the pre-existing test-only `OnceLock` import still needed by `agent_tool.rs`
- After those minimal fixes, the full validation sequence was rerun from the top

## Command Results

### `cargo fmt --all -- --check`
- Status: `PASS`
- Summary: final rerun formatting gate passed with exit code `0`
- Caveats: none

### `cargo build --workspace`
- Status: `PASS`
- Summary: final rerun build completed successfully with exit code `0`
- Caveats:
  - an earlier rerun build failed on five `run_query_loop(...)` call sites in `src-rust/crates/cli/src/main.rs`
  - that failure was corrected by the minimal `Some(client_clone.as_ref())` patch described above

### `cargo test --workspace`
- Status: `PASS`
- Summary: final rerun workspace tests passed with exit code `0`
- Non-blocking warnings observed:
  - `unused import: Role` in `crates/query/src/compact.rs`
  - `unused variable: ctx` in `crates/commands/src/named_commands.rs`
  - `unused import: TranscriptEntry` in `crates/core/tests/parity_smoke.rs`
  - several non-snake-case test names in `crates/tui/src/prompt_input.rs`
- Caveats:
  - an earlier rerun test pass failed on `tests::provider_registry_some_resolution_failure_does_not_fallback_to_legacy_anthropic`
  - the failure was caused by cross-test auth-store leakage and was corrected by the shared test-only lock described above

### `cargo clippy --workspace --all-targets`
- Status: `PASS`
- Summary: final rerun clippy completed with exit code `0`
- Non-blocking warnings / caveats:
  - clippy reported broad existing warning-only lint debt across multiple crates (`core`, `api`, `mcp`, `tools`, `query`, `tui`, `commands`, `cli`, `bridge`, `acp`, `plugins`, `buddy`)
  - no clippy error blocked this ticket
  - the command passed without requiring additional source changes

## Manual Smoke-Test Feasibility Assessment
- CLI/provider surface for `--provider openai` is present:
  - yes; `src-rust/crates/cli/src/main.rs` still exposes the provider flag
- Agent spawn surface is present:
  - yes; `build_tools_with_mcp(...)` still includes `claurst_query::AgentTool`, and the team swarm runner remains initialized
- Automated repo proof of the OpenAI path is present:
  - `agent_explicit_provider_routes_to_openai_provider`
  - `agent_parent_inherits_provider_openai_dispatch`
  - `teamcreate_mixed_providers_per_agent_dispatch`
- Live smoke prerequisites in this environment:
  - `OPENAI_API_KEY` in environment -> `no`
  - `$HOME/.claurst/auth.json` present -> `no`
  - sandbox provides unrestricted outbound network access -> `no`
- Feasibility classification:
  - not feasible in this environment

## Manual Smoke-Test Result
- Status: `NOT RUN`
- Reason:
  - no usable OpenAI credential was available in the current environment
  - no stored auth file was present at `$HOME/.claurst/auth.json`
  - the active sandbox is network-restricted, so a real outbound OpenAI validation could not be performed here
- MPWO alignment:
  - this matches the work-order clause `if possible in the test environment`

## D1 Completion Assessment
- Automated validation outcome:
  - `cargo fmt --all -- --check` -> pass
  - `cargo build --workspace` -> pass
  - `cargo test --workspace` -> pass
  - `cargo clippy --workspace --all-targets` -> pass
- Manual smoke outcome:
  - environment-blocked for concrete credential and network reasons
- Grounded conclusion:
  - `D1 is complete. Provider-resolution seam is landed. Workers inherit parent providers.`
  - That conclusion is supported by the passing full automated validation suite and the existing automated OpenAI/agent-dispatch coverage, but it does not claim a live OpenAI smoke was executed in this sandbox

## Files Changed
- `src-rust/crates/cli/src/main.rs`
- `src-rust/crates/query/src/agent_tool.rs`
- `src-rust/crates/query/src/lib.rs`
- `src-rust/crates/query/src/provider_resolution.rs`
- `docs/archive/reports/TASK-M9-12_EXECUTION_RERUN_REPORT_20260413T153943Z.md`

## Verdict
`PASS / D1 COMPLETE WITH SMOKE-TEST ENV LIMITATION`

## Notes
- This rerun stayed within `TASK-M9-12` only
- No D2 / M10 / M11 / M12 work was started
- No hosted-Ollama work was reopened
- No unrelated cleanup was performed
- Review basis is explicit:
  - active unstaged diff for the four tracked source files above plus the required report artifacts
- Because source edits became strictly necessary to clear validation, a closeout report accompanies this rerun
