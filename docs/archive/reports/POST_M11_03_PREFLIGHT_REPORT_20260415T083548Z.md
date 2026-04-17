# POST-M11-03 Preflight Report

## 1. ticket id

`POST-M11-03`

## 2. verdict

`PASS-WITH-NOTES`

## 3. timestamp UTC

`2026-04-15T08:35:48Z`

## 4. branch / HEAD observed

- Branch: `feature/provider-resolution-seam`
- HEAD: `7fef4a3b9610a91963d474c1d61ab736299447d8`
- Matches accepted latest HEAD from `POST-M11-02`: `yes`
- Working-tree state: dirty / noisy
- Observed repo noise:
  - modified `.gitignore`
  - many untracked artifacts under `docs/archive/reports/`
  - untracked `.codex`
  - untracked `docs/Orchestrator_planning/`
  - untracked `docs/archive/provider_orchestrator/`
  - untracked `src-rust/.codex`
  - untracked `src-rust/target/`

## 5. authority reviewed

- `AGENTS.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md`
- `docs/archive/reports/POST_M11_REPO_ASSESSMENT_20260415T071321Z.md`
- `docs/archive/reports/POST_M11_01_CLOSEOUT_REPORT_20260415T074353Z.md`
- `docs/archive/reports/POST_M11_02_CLOSEOUT_REPORT_20260415T082535Z.md`

Verified authority state:

- `docs/Current/MPWO_WORK_ORDER_PACK.md` still states that it is the single active authority artifact in `docs/Current/`.
- `rg --files docs/Current` shows five files in `docs/Current/`, with the other four retained as historical/non-controlling artifacts.
- No conflicting newer current-authority artifact was found in `docs/Current/`.

Verified commands and inspection surfaces:

- `git branch --show-current`
- `git rev-parse HEAD`
- `git status --short --branch`
- `cd src-rust && cargo clippy -p claurst-core --all-targets -- -D warnings`
- `cd src-rust && cargo build -p claurst-core`
- `cd src-rust && cargo test -p claurst-core`
- targeted `sed` inspection of the live failing files named below

## 6. current claurst-core clippy findings

Live command result:

- `cd src-rust && cargo clippy -p claurst-core --all-targets -- -D warnings` -> `FAIL`

Exact current failing files and lint classes from the live run, in observed order:

1. `src-rust/crates/core/src/session_storage.rs`
   - `collapsible_match` at lines `310`, `470`, `481`
   - `redundant_pattern_matching` at lines `447`, `450`
2. `src-rust/crates/core/src/attachments.rs`
   - `unnecessary_map_or` at line `101`
3. `src-rust/crates/core/src/status_notices.rs`
   - `ptr_arg` at line `104`
4. `src-rust/crates/core/src/cloud_session.rs`
   - `unnecessary_lazy_evaluations` at line `73`
5. `src-rust/crates/core/src/claudemd.rs`
   - `unnecessary_map_or` at line `233`
6. `src-rust/crates/core/src/feature_flags.rs`
   - `new_without_default` at line `57`
7. `src-rust/crates/core/src/skill_discovery.rs`
   - `manual_strip` at line `46`
   - `double_ended_iterator_last` at line `219`
8. `src-rust/crates/core/src/lib.rs`
   - `derivable_impls` at line `898`
   - `doc_lazy_continuation` at line `1034`
   - `manual_map` at line `1108`
   - `single_match` at line `2659`
   - `field_reassign_with_default` in tests at lines `3514`, `3530`, `3542`
9. `src-rust/crates/core/src/bash_classifier.rs`
   - `collapsible_if` at line `131`
   - `manual_strip` at lines `149`, `175`, `283`
10. `src-rust/crates/core/src/effort.rs`
   - `should_implement_trait` at line `35`
11. `src-rust/crates/core/src/lsp.rs`
   - `single_match` at line `549`
12. `src-rust/crates/core/src/remote_settings.rs`
   - `manual_ok_err` at line `275`
   - `redundant_closure` at line `285`
13. `src-rust/crates/core/src/system_prompt.rs`
   - `should_implement_trait` at line `113`
   - `vec_init_then_push` at line `249`

Assessment-cluster accuracy vs live repo reality:

- The repo-assessment six-file cluster is still accurate as a subset:
  - `session_storage.rs`
  - `attachments.rs`
  - `feature_flags.rs`
  - `skill_discovery.rs`
  - `bash_classifier.rs`
  - `system_prompt.rs`
- Live failures have shifted broader than that assessment. Additional currently failing `claurst-core` files are:
  - `status_notices.rs`
  - `cloud_session.rs`
  - `claudemd.rs`
  - `lib.rs`
  - `effort.rs`
  - `lsp.rs`
  - `remote_settings.rs`

Additional likely follow-on all-targets blocker:

- `cd src-rust && cargo test -p claurst-core` passes, but emits `unused_import` in `src-rust/crates/core/tests/parity_smoke.rs:9`.
- That warning is likely to become a later `clippy --all-targets -D warnings` failure once the current library blockers are removed.

## 7. recommended tranche boundary

Recommended boundary:

- `small file cluster`

Recommended `POST-M11-03` tranche-1 file set:

- `src-rust/crates/core/src/session_storage.rs`
- `src-rust/crates/core/src/attachments.rs`
- `src-rust/crates/core/src/status_notices.rs`
- `src-rust/crates/core/src/cloud_session.rs`
- `src-rust/crates/core/src/claudemd.rs`
- `src-rust/crates/core/src/feature_flags.rs`
- `src-rust/crates/core/src/skill_discovery.rs`
- `src-rust/crates/core/src/bash_classifier.rs`
- `src-rust/crates/core/src/lsp.rs`
- `src-rust/crates/core/src/remote_settings.rs`

Recommended exclusions from tranche 1:

- `src-rust/crates/core/src/lib.rs`
- `src-rust/crates/core/src/effort.rs`
- `src-rust/crates/core/src/system_prompt.rs`
- `src-rust/crates/core/tests/parity_smoke.rs`

Boundary rationale:

- A single-file ticket is too small to materially advance the crate toward a meaningful clippy gate.
- The ten included files are dominated by local, semantics-preserving hygiene lints:
  - `collapsible_match`
  - `redundant_pattern_matching`
  - `unnecessary_map_or`
  - `ptr_arg`
  - `unnecessary_lazy_evaluations`
  - `manual_strip`
  - `double_ended_iterator_last`
  - `single_match`
  - `manual_ok_err`
  - `redundant_closure`
  - `new_without_default`
- Including `lib.rs`, `effort.rs`, and `system_prompt.rs` would widen the ticket into broader shared-glue and API-shape cleanup rather than a narrow first tranche.

Scope containment:

- This ticket can remain limited to `claurst-core` only.
- No current live evidence requires widening into `claurst-api`, `claurst-query`, `claurst-tools`, or repo-wide cleanup.

## 8. risk / non-regression findings

- `session_storage.rs` is runtime-sensitive because it touches transcript tombstone handling and tail metadata extraction. The named lints are still local/mechanical, but this file needs careful review.
- `bash_classifier.rs` affects command risk classification. The reported lints are syntactic, but this file is behavior-sensitive and should be treated as non-regression-sensitive.
- `skill_discovery.rs` touches frontmatter parsing and git URL cache naming. The current lints are small, but parsing behavior should not be casually rewritten.
- `feature_flags.rs` is low risk for this tranche; `new_without_default` can be addressed without changing runtime semantics.
- `status_notices.rs`, `cloud_session.rs`, `claudemd.rs`, `lsp.rs`, and `remote_settings.rs` are straightforward hygiene surfaces with low semantic risk.
- `lib.rs` is high-spread shared glue and test code. Touching it in tranche 1 risks turning the ticket into a broad cleanup blob.
- `effort.rs` and `system_prompt.rs` both carry `should_implement_trait`, which requires semantic judgment about public parsing API shape (`rename`, trait implementation strategy, compatibility expectations). These should stay out of tranche 1.
- No accepted M11 runtime behavior needs to be reopened if the ticket stays inside the recommended boundary.

## 9. recommended validation gate

Current supporting gate results:

- `cd src-rust && cargo build -p claurst-core` -> `PASS`
- `cd src-rust && cargo test -p claurst-core` -> `PASS`
- Test note: `cargo test` emitted `unused_import` in `src-rust/crates/core/tests/parity_smoke.rs:9`

Recommended later execution/verification gates for this tranche:

- blocking:
  - `cd src-rust && cargo build -p claurst-core`
  - `cd src-rust && cargo test -p claurst-core`
- non-blocking progress probe:
  - `cd src-rust && cargo clippy -p claurst-core --all-targets -- -D warnings`

Explicit answer on the proposed clippy gate:

- `POST-M11-03` should **not** use `cd src-rust && cargo clippy -p claurst-core --all-targets -- -D warnings` as its blocking validation gate if it remains the narrow tranche recommended above.
- Reason: that full crate gate will still fail on intentionally excluded files (`lib.rs`, `effort.rs`, `system_prompt.rs`) and likely later on `tests/parity_smoke.rs`.
- That full clippy command becomes an appropriate blocking gate only for the final `claurst-core` cleanup tranche that absorbs the remaining crate-local lint set.

## 10. drift / blockers, if any

- No structural drift was found in branch, HEAD, or accepted baseline:
  - expected branch `feature/provider-resolution-seam` -> matched
  - expected HEAD `7fef4a3b9610a91963d474c1d61ab736299447d8` -> matched
- No authority drift was found in `docs/Current/`:
  - `docs/Current/MPWO_WORK_ORDER_PACK.md` remains the sole active current-authority artifact
- Live lint-set drift was found relative to the earlier repo assessment:
  - the earlier six-file cluster is still valid, but it is no longer exhaustive
- Pre-existing worktree noise remains a patch-hygiene note for later execution:
  - modified `.gitignore`
  - many untracked report and generated paths
- No blocker forces widening beyond `claurst-core`
- The only material blocker to using a full crate clippy pass as a blocking gate is the intentionally narrow tranche boundary itself

## 11. exact recommendation for next step

- Proceed with `POST-M11-03` as a `claurst-core`-only execution ticket.
- Keep it explicitly framed as `claurst-core clippy remediation tranche 1: low-risk mechanical hygiene cluster`.
- Include exactly these tranche-1 files:
  - `session_storage.rs`
  - `attachments.rs`
  - `status_notices.rs`
  - `cloud_session.rs`
  - `claudemd.rs`
  - `feature_flags.rs`
  - `skill_discovery.rs`
  - `bash_classifier.rs`
  - `lsp.rs`
  - `remote_settings.rs`
- Explicitly exclude:
  - `lib.rs`
  - `effort.rs`
  - `system_prompt.rs`
  - any non-core crate
  - repo-wide lint cleanup
- Use `cargo build -p claurst-core` and `cargo test -p claurst-core` as blocking execution-time validation gates for this tranche.
- Re-run full `cargo clippy -p claurst-core --all-targets -- -D warnings` after the tranche as a progress probe, then define the next `claurst-core` tranche from the remaining failures.
