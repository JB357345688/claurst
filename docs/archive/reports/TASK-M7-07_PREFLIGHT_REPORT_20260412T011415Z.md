# TASK-M7-07 Preflight Report

- Ticket ID: `TASK-M7-07`
- Verdict: `READY-WITH-NOTES`
- Preflight timestamp (UTC): `2026-04-12T01:14:15Z`
- Current branch: `feature/provider-resolution-seam`
- No source files were edited and no git state was altered during this preflight. Only this report file was created under `docs/archive/reports/`.

## Git Working Tree Summary

- Verified from `git status --short --branch`:
  - `## feature/provider-resolution-seam`
  - `?? docs/`
  - `?? src-rust/target/`
- Verified from `git diff --cached --name-status`: no staged tracked changes.
- Verified from `git diff --name-status`: no unstaged tracked changes.
- Untracked working-tree noise is present under `docs/` and `src-rust/target/`.
- Authority file git state:
  - `AGENTS.md` is ignored, not tracked: `git check-ignore -v AGENTS.md` returned `.gitignore:1:/AGENTS.md AGENTS.md`
  - `docs/Current/MPWO_WORK_ORDER_PACK.md` is untracked: `git status --short --ignored AGENTS.md docs/Current/MPWO_WORK_ORDER_PACK.md` returned `?? docs/Current/MPWO_WORK_ORDER_PACK.md`

## Baseline Commit Verification Results

All required baseline commits are present in current branch history as ancestors of `HEAD`.

| Commit | Subject | Present in current branch history |
|---|---|---|
| `a09b3da` | `Establish provider resolution seam baseline` | Yes |
| `5881983` | `TASK-M7-04 wire run_query_loop through provider resolution seam` | Yes |
| `255e3c7` | `Cleanup remove obsolete provider worker fabric RFCs` | Yes |
| `5f8dfe1` | `Fix hosted Ollama compatibility on provider seam` | Yes |
| `d76e8fb731473b5abf09c05ce885a0c4721233b9` | `test(provider_resolution): add P1-P12 resolve_provider_identity coverage` | Yes |
| `73e9104d96cc7d12a7000285268522d326ce9956` | `test(provider_resolution): add materialize_provider coverage` | Yes |
| `865767881c683bb70c5bd253740ae0c5107b3752` | `chore(gitignore): ignore local root prompt files` | Yes |

## Verified Authority Sources

- Verified `/home/jordi/claurst/AGENTS.md` from the current working tree.
- Verified `/home/jordi/claurst/docs/Current/MPWO_WORK_ORDER_PACK.md` from the current working tree.
- Exact `TASK-M7-07` section located at `docs/Current/MPWO_WORK_ORDER_PACK.md:737`.
- Standing hosted-Ollama non-regression invariant located at `docs/Current/MPWO_WORK_ORDER_PACK.md:108`.

## TASK-M7-07 Contract Verified From MPWO

- Objective: verify the entire workspace compiles, tests pass, and clippy is clean after M7 changes.
- Preconditions: all M7 tickets `M7-01` through `M7-06` complete.
- Exact code targets: none. The ticket is validation-only.
- Strict constraints:
  - do not fix pre-existing clippy warnings in unrelated files
  - do not modify any file not already modified in `M7-01` through `M7-06`
  - do not add features or surrounding cleanup
- Definition of done:
  - `cargo build --workspace` succeeds
  - `cargo test --workspace` passes
  - `cargo clippy --workspace --all-targets` has no new warnings from M7 code
  - `cargo fmt --all -- --check` passes
- Stop or escalate conditions:
  - if `cargo test --workspace` fails outside `provider_resolution.rs` and outside M7-modified behavior, stop and investigate causality
  - if more than 3 files need fixes, escalate
- Hosted-Ollama reporting implication from section 2A:
  - any preflight, execution, or review report for this ticket must explicitly state either `Hosted Ollama compatibility baseline preserved` or `Hosted Ollama compatibility baseline intentionally changed by explicit ticket scope`

## Preconditions Status

- Overall precondition verdict: satisfied in committed repo state.

### M7-01 complete

- Baseline commit `a09b3daefe887f2794c9fc2154afd8ebc8b3ec27` is present in branch history.
- `docs/archive/reports/TASK-M7-BASELINE_BRANCH_REPORT.md` states that commit `a09b3da...` established the real git baseline for `src-rust/crates/query/src/provider_resolution.rs` and `src-rust/crates/query/src/lib.rs`.
- Current committed `lib.rs` still exports the seam module at `src-rust/crates/query/src/lib.rs:19` and `:25`.

### M7-02 complete

- `docs/archive/reports/TASK-M7-BASELINE_BRANCH_REPORT.md` explicitly states commit `a09b3da...` includes accepted content through `M7-03`, including `resolve_provider_identity()`.
- Current committed repo state contains `resolve_provider_identity(...)` at `src-rust/crates/query/src/provider_resolution.rs:101` with full resolution logic, not a stub.
- There are no staged or unstaged tracked diffs, so this function is committed repo state, not a working-tree-only change.

### M7-03 complete

- `docs/archive/reports/TASK-M7-BASELINE_BRANCH_REPORT.md` explicitly states commit `a09b3da...` includes accepted content through `M7-03`, including `materialize_provider()`.
- `docs/archive/reports/TASK-M7-03_COMPLETION_REPORT.md` records `TASK-M7-03` completion with `cargo check -p claurst-query` passing.
- Current committed repo state contains `materialize_provider(...)` at `src-rust/crates/query/src/provider_resolution.rs:157`.

### M7-04 complete

- Commit `58819832c1385d64d0e8f9c4d68ff18f5a96fd05` is present in branch history.
- `docs/archive/reports/TASK-M7-04_ACCEPTANCE_AND_RFC_CLEANUP_REPORT.md` records `5881983` as the accepted standalone `M7-04` commit touching only `src-rust/crates/query/src/lib.rs`.
- Current committed `lib.rs` calls the seam functions at `src-rust/crates/query/src/lib.rs:860` and `:872`.

### M7-05 complete

- Commit `d76e8fb731473b5abf09c05ce885a0c4721233b9` is present in branch history.
- `docs/archive/reports/TASK-M7-05_CLOSEOUT_REPORT_20260411T232128Z.md` records:
  - verdict `CLOSED`
  - committed file `src-rust/crates/query/src/provider_resolution.rs`
  - validation result `15 passed, 0 failed`
- Current committed `provider_resolution.rs` contains the `#[cfg(test)]` module and the P1-P12 `resolve_provider_identity()` coverage.
- `git diff --name-status` and `git diff --cached --name-status` are empty, confirming `M7-05` is committed, not sitting as unstaged or staged diff.

### M7-06 complete

- Commit `73e9104d96cc7d12a7000285268522d326ce9956` is present in branch history and is the current `HEAD`.
- `docs/archive/reports/TASK-M7-06_CLOSEOUT_REPORT_20260412T005128Z.md` records:
  - verdict `CLOSED`
  - committed file `src-rust/crates/query/src/provider_resolution.rs`
  - validation result `18 passed; 0 failed; 0 ignored`
- Current committed `provider_resolution.rs` contains the `materialize_provider_*` tests beginning at line `489`.
- `git diff --name-status` and `git diff --cached --name-status` are empty, confirming `M7-06` is committed, not sitting as unstaged or staged diff.

## Workspace-Validation Scope Assessment

- `TASK-M7-07` is validation-only and does not authorize broad cleanup.
- Based on committed `M7` history, the only legitimate ticket-local fix targets if validation later finds M7 regressions are:
  - `src-rust/crates/query/src/provider_resolution.rs`
  - `src-rust/crates/query/src/lib.rs`
- Evidence from committed history:
  - `a09b3da` touched `src-rust/crates/query/src/lib.rs` and `src-rust/crates/query/src/provider_resolution.rs`
  - `5881983` touched `src-rust/crates/query/src/lib.rs`
  - `d76e8fb...` touched `src-rust/crates/query/src/provider_resolution.rs`
  - `73e9104...` touched `src-rust/crates/query/src/provider_resolution.rs`
- Files not authorized for incidental cleanup under `TASK-M7-07` include:
  - `src-rust/crates/api/src/error_handling.rs`
  - `src-rust/crates/core/src/auth_store.rs`
  - `.gitignore`
  - `src-rust/crates/query/src/compact.rs`
  - docs and RFC files
  - any other unrelated workspace file
- Important scope nuance:
  - commit `5f8dfe1` touched `src-rust/crates/api/src/error_handling.rs`, `src-rust/crates/core/src/auth_store.rs`, and `src-rust/crates/query/src/provider_resolution.rs`
  - the hosted-Ollama invariant makes that behavior authoritative baseline
  - `TASK-M7-07` still does not authorize editing `error_handling.rs` or `auth_store.rs`, because they were not part of `M7-01` through `M7-06`
- Known unrelated warning/noise:
  - recent `M7-05` and `M7-06` reports record an unrelated existing unused-import warning in `crates/query/src/compact.rs`
  - latest cited location is `crates/query/src/compact.rs:1193`
  - preflight classification: pre-existing and unrelated unless current validation output proves the M7 seam changed its behavior or warning surface

## Hosted Ollama Invariant Applicability

- `TASK-M7-07` is an in-scope seam-validation ticket under MPWO section 2A.
- Current committed hosted-Ollama baseline touchpoints remain present in `src-rust/crates/query/src/provider_resolution.rs`:
  - `build_ollama_provider(...)` at line `218`
  - `normalize_ollama_api_base(...)` at line `237`
  - `AuthStore::load().api_key_for(ProviderId::OLLAMA)` at line `230`
- If any M7-related fixup becomes necessary during execution, validation, review, and closeout must explicitly state:
  - `Hosted Ollama compatibility baseline preserved`
  - or `Hosted Ollama compatibility baseline intentionally changed by explicit ticket scope`
- Expected outcome for `TASK-M7-07` if validation passes without code changes:
  - `Hosted Ollama compatibility baseline preserved`

## Verified Commands

- Verified `src-rust/` exists.
- Verified workspace manifest exists at `src-rust/Cargo.toml`.
- Verified the manifest contains `[workspace]` with these members:
  - `crates/core`
  - `crates/api`
  - `crates/tools`
  - `crates/query`
  - `crates/tui`
  - `crates/commands`
  - `crates/mcp`
  - `crates/bridge`
  - `crates/cli`
  - `crates/buddy`
  - `crates/plugins`
  - `crates/acp`
- Verified member manifests exist for all listed workspace members.
- Verified current query manifest still includes `thiserror = { workspace = true }` in `src-rust/crates/query/Cargo.toml:17`.
- Required command shapes are plausible for the current workspace layout:
  - `cd src-rust && cargo fmt --all -- --check`
  - `cd src-rust && cargo build --workspace`
  - `cd src-rust && cargo test --workspace`
  - `cd src-rust && cargo clippy --workspace --all-targets`
- No repo-local `rust-toolchain` or `rust-toolchain.toml` file was found at `src-rust/`; preflight did not execute cargo and therefore did not verify toolchain component installation.

## Drift Found

- Classification: `line drift only`
- Non-blocking drift observed:
  - the MPWO numbered steps list `build`, `test`, `clippy`, then `fmt`
  - the MPWO `Validation Commands` block and the active prompt list `fmt`, `build`, `test`, `clippy`
  - command set is identical; only ordering differs
- No structural drift was found in ticket scope, preconditions, file targets, or command shapes.

## Blockers

- None.

## Notes For Execution Phase

- `M7-07` is the Milestone 7 workspace-validation gate required before Milestone 8.
- Use the current working-tree MPWO and `AGENTS.md` as authority even though they are not committed.
- Keep scope strict:
  - do not fix unrelated failures
  - do not clean up `compact.rs`
  - do not widen changes beyond `provider_resolution.rs` and the already-modified seam section of `lib.rs` unless the ticket explicitly escalates
- If validation implicates files outside that scope, report causality analysis and stop rather than broadening the patch.
- If more than 3 files would need changes, escalate per ticket instructions.
- Preserve and explicitly report the hosted-Ollama baseline outcome.
