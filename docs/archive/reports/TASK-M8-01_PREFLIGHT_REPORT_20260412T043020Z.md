# TASK-M8-01 Preflight Report

- Ticket ID: `TASK-M8-01`
- Verdict: `READY-WITH-NOTES`
- Timestamp: `2026-04-12T04:30:20Z`
- Current branch: `feature/provider-resolution-seam`
- Source-edit status: no source files were edited in this preflight; the only write was creation of this required report artifact under `docs/archive/reports/`.

## Git Working Tree Summary

- `git branch --show-current` verified the current branch as `feature/provider-resolution-seam`.
- `git status --short --branch` showed no staged changes and no unstaged tracked changes.
- `git diff --name-only --cached` was empty.
- `git diff --name-only` was empty.
- Untracked noise is present under:
  - `.codex/`
  - `docs/`
  - `src-rust/target/`
- Preflight verdict on baseline cleanliness: tracked-source baseline is clean enough to begin `TASK-M8-01`; untracked artifact noise should remain explicitly noted in later review basis.

## Baseline Commit Verification Results

All required baseline commits from the prompt were verified as ancestors of `HEAD` on the current branch:

- `a09b3daefe887f2794c9fc2154afd8ebc8b3ec27` — `Establish provider resolution seam baseline`
- `58819832c1385d64d0e8f9c4d68ff18f5a96fd05` — `TASK-M7-04 wire run_query_loop through provider resolution seam`
- `5f8dfe1edd3b0b2c3c064b463948080bcc5b188b` — `Fix hosted Ollama compatibility on provider seam`
- `d76e8fb731473b5abf09c05ce885a0c4721233b9` — `test(provider_resolution): add P1-P12 resolve_provider_identity coverage`
- `73e9104d96cc7d12a7000285268522d326ce9956` — `test(provider_resolution): add materialize_provider coverage`
- `780cb725297d26ede1c858b47c9a371b8e098339` — `style(rust): apply workspace rustfmt baseline`
- `fc6d5a9bf86d29cb5527a997bd245fe8dda337d1` — `test(codex_adapter): make request assertion robust to numeric representation`
- `f8eb1300676937e07ad7ead65c94498b1bd0e7df` — `test(onboarding): align default-page assertion with current behavior`
- `7f57749feb005df209166d7f8f1205644de328fb` — `test(tui): align collapsed-thinking expectation with current rendering`

Accepted `TASK-M7-07` closeout status was also verified in current repo reality:

- `docs/archive/reports/TASK-M7-07_CLOSEOUT_REPORT_20260412T040059Z.md` records verdict `CLOSED`.
- That closeout report names commit `b8cc827c19eb29cde106b08dca0262d4c8dd66d8` with subject `refactor(query): resolve seam-local clippy regressions in run_query_loop`.
- `git rev-parse HEAD` returned the same commit hash, so the current branch tip is anchored at the accepted `TASK-M7-07` closeout commit.
- `docs/archive/reports/TASK-M7-07_REVIEW_REPORT_20260412T035625Z.md` records `Pass/Fail: PASS` and `Ready to close: yes`.

Result: Milestone 7 is complete and accepted on the current branch for purposes of starting Milestone 8.

## Verified Authority Sources

- `/home/jordi/claurst/AGENTS.md`
- `/home/jordi/claurst/docs/Current/MPWO_WORK_ORDER_PACK.md`

Exact authority locations verified:

- `TASK-M8-01` section at `docs/Current/MPWO_WORK_ORDER_PACK.md:794`
- standing hosted-Ollama invariant at `docs/Current/MPWO_WORK_ORDER_PACK.md:108`
- M8 dependency gate `M8 Worker Propagation (all require M7-07 complete)` at `docs/Current/MPWO_WORK_ORDER_PACK.md:77`

## TASK-M8-01 Contract Verified From MPWO

- Objective:
  - extend `ToolContext` with optional provider/model registry fields so tools can access the parent session registries
- Preconditions:
  - Milestone 7 complete
  - `claurst-tools` already depends on `claurst-api`
- Exact code target:
  - `crates/tools/src/lib.rs` `ToolContext` struct
- Strict constraints:
  - do not add `session_budget`, `health_cache`, or other D2 fields
  - do not change existing `ToolContext` fields
  - do not add provider-resolution methods to `ToolContext`
  - do not make the new fields non-optional
- Definition of done:
  - `ToolContext` has `provider_registry: Option<Arc<ProviderRegistry>>`
  - `ToolContext` has `model_registry: Option<Arc<ModelRegistry>>`
  - existing construction sites compile with the new fields
  - `cargo check --workspace` succeeds
- Validation command:
  - `cd src-rust && cargo check --workspace`
- Stop / escalate conditions:
  - if there are more than 10 `ToolContext` construction sites, escalate
  - if importing `Arc` creates conflicts, investigate before proceeding

## Preconditions Status

- Milestone 7 complete and accepted on current branch: `PASS`
  - evidence: all listed M7 baseline commits are present; `TASK-M7-07` closeout report is `CLOSED`; `HEAD` is the accepted `TASK-M7-07` closeout commit
- Workspace baseline healthy enough to begin `TASK-M8-01`: `PASS`
  - evidence: no staged changes, no unstaged tracked changes, only untracked artifact noise
- `claurst-tools` already depends on `claurst-api`: `PASS`
  - evidence: `src-rust/crates/tools/Cargo.toml:8` contains `claurst-api = { workspace = true }`

## ToolContext Construction-Site Inventory

- Exact `ToolContext` struct location:
  - `src-rust/crates/tools/src/lib.rs:216-229`
- `std::sync::Arc` already imported in `crates/tools/src/lib.rs`:
  - `YES` at `src-rust/crates/tools/src/lib.rs:13`
- Repo-wide `ToolContext` construction-site count:
  - `3`
- Repo-wide file count containing constructors:
  - `2`
- Escalation threshold check (`>10` sites):
  - `NOT EXCEEDED`

Construction-site classification:

| File | Approx line | Classification | Notes |
|---|---:|---|---|
| `src-rust/crates/cli/src/main.rs` | 651 | production site | root startup `tool_ctx` literal |
| `src-rust/crates/tools/src/lib.rs` | 539 | test-only site | `test_resolve_path_absolute` fixture |
| `src-rust/crates/tools/src/lib.rs` | 567 | test-only site | `test_resolve_path_relative` fixture |

Repo reality matches the recon finding:

- exactly `3` construction sites
- exactly `2` files

Files and approximate line locations that the current repo shape would force an execution pass to consider:

- `src-rust/crates/tools/src/lib.rs`
  - import block near line `13`
  - `ToolContext` definition at lines `216-229`
  - test constructor sites near lines `539-552` and `567-580`
- `src-rust/crates/cli/src/main.rs`
  - production constructor site at lines `651-662`

Execution note:

- The MPWO assigns registry population in `main.rs` to `TASK-M8-02`.
- In current repo reality, there is only one production `ToolContext` literal, so constructor-surface coordination remains small.
- The later startup-order wrinkle belongs to `TASK-M8-02`, not `TASK-M8-01`: `provider_registry` is built before `tool_ctx`, but `model_registry` is created later in startup.

## Dependency / Interface Verification

- `claurst_api::ProviderRegistry` availability from `crates/tools`: `PASS`
  - `src-rust/crates/tools/Cargo.toml:8` already depends on `claurst-api`
  - `src-rust/crates/api/src/lib.rs:72` re-exports `ProviderRegistry`
- `claurst_api::ModelRegistry` availability from `crates/tools`: `PASS`
  - `src-rust/crates/tools/Cargo.toml:8` already depends on `claurst-api`
  - `src-rust/crates/api/src/lib.rs:80` re-exports `ModelRegistry`
- Proposed field additions:
  - `provider_registry: Option<Arc<claurst_api::ProviderRegistry>>`
  - `model_registry: Option<Arc<claurst_api::ModelRegistry>>`
- Mechanical safety assessment: `LOW RISK`
  - `ToolContext` already imports `Arc`
  - `ToolContext` derives `Clone` only; `Option<Arc<T>>` preserves cloneability cleanly
  - no `Default` impl, builder pattern, or other constructor indirection was found for `ToolContext`
  - existing test fixtures are straightforward to patch with `provider_registry: None` and `model_registry: None`

## Validation-Command Plausibility Check

- `src-rust` exists: `YES`
- workspace manifest exists at `src-rust/Cargo.toml`: `YES`
- `crates/tools` is a workspace member: `YES` (`src-rust/Cargo.toml:3-16`)
- MPWO validation command shape:

```bash
cd src-rust && cargo check --workspace
```

- Plausibility verdict: `VALID FOR CURRENT WORKSPACE LAYOUT`
- Command execution status in this preflight: not run by design; inspection was sufficient to confirm the command shape is valid as written

## Hosted Ollama Invariant Applicability

- `TASK-M8-01` is primarily a plumbing ticket:
  - it changes `ToolContext` shape and constructor payloads
  - it does not directly change provider resolution, provider materialization, auth-store lookup, or runtime provider selection
- Direct hosted-Ollama regression risk for `TASK-M8-01`: `LOW`
- Higher-risk later M8 work remains:
  - `TASK-M8-02` for startup wiring/order
  - `TASK-M8-04`, `TASK-M8-05`, `TASK-M8-08`, and `TASK-M8-09` if they bypass shared seam helpers
- Required reporting outcome for `TASK-M8-01` if production behavior stays unchanged:
  - `Hosted Ollama compatibility baseline preserved`

Hosted Ollama compatibility baseline preserved

## Verified Commands

Commands executed for this preflight:

- `git -C /home/jordi/claurst branch --show-current`
- `git -C /home/jordi/claurst status --short --branch`
- `git -C /home/jordi/claurst diff --name-only --cached`
- `git -C /home/jordi/claurst diff --name-only`
- `git -C /home/jordi/claurst ls-files --others --exclude-standard`
- `git -C /home/jordi/claurst merge-base --is-ancestor <required-commit> HEAD` for each required baseline commit
- `git -C /home/jordi/claurst log --no-walk --format='%h %H %s' ...`
- `git -C /home/jordi/claurst rev-parse HEAD`
- `sed`, `nl -ba`, and `rg` reads against:
  - `AGENTS.md`
  - `docs/Current/MPWO_WORK_ORDER_PACK.md`
  - `src-rust/Cargo.toml`
  - `src-rust/crates/tools/Cargo.toml`
  - `src-rust/crates/tools/src/lib.rs`
  - `src-rust/crates/cli/src/main.rs`
  - `src-rust/crates/api/src/lib.rs`
- validation-command plausibility probe:
  - `test -d /home/jordi/claurst/src-rust && test -f /home/jordi/claurst/src-rust/Cargo.toml`

Command verified by inspection, not executed:

- `cd src-rust && cargo check --workspace`

## Drift Found

- Drift classification: `LINE DRIFT ONLY`
- Verified line drift:
  - MPWO lists `ToolContext` around `crates/tools/src/lib.rs:209-223`; actual current location is `216-229`
  - MPWO cites `main.rs` constructor around `~647-658`; actual current location is `651-662`
- No structural drift was found that materially changes `TASK-M8-01` execution.
- Confirmed downstream note for `TASK-M8-02`:
  - `provider_registry` is built at `src-rust/crates/cli/src/main.rs:612-613`
  - `model_registry` is built later at `src-rust/crates/cli/src/main.rs:709`
  - this is a later startup-ordering wrinkle, not a blocker for `TASK-M8-01` preflight

## Blockers

- None for `TASK-M8-01` preflight

## Notes For Execution Phase

- Keep scope tight to `ToolContext` and constructor updates only.
- Re-run the repo-wide constructor search immediately before editing; current verified count is `3` across `2` files.
- Update the two test-only fixtures in `crates/tools/src/lib.rs` with `provider_registry: None` and `model_registry: None`.
- Keep the production-site note explicit in the execution report:
  - `main.rs` contains the only production `ToolContext` literal
  - the actual registry-population wrinkle belongs to `TASK-M8-02`, because `model_registry` is created later in startup
- Preserve the standing invariant language in execution/review output:
  - `Hosted Ollama compatibility baseline preserved`
