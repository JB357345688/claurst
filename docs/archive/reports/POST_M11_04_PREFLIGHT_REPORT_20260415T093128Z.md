# POST-M11-04 Preflight Report

## 1. ticket id

`POST-M11-04`

## 2. verdict

`PASS-WITH-NOTES`

## 3. timestamp UTC

`2026-04-15T09:31:28Z`

## 4. branch / HEAD observed

- Branch: `feature/provider-resolution-seam`
- HEAD: `0f66f7f3c35b4f8232eb8795627b4e8dfb0b2083`
- Expected accepted HEAD: `0f66f7f3c35b4f8232eb8795627b4e8dfb0b2083`
- HEAD match: `yes`
- Working-tree state: dirty / noisy
- Observed out-of-scope noise:
  - modified `.gitignore`
  - untracked `.codex`
  - untracked report/archive artifacts under `docs/archive/reports/`
  - untracked `docs/Orchestrator_planning/`
  - untracked `docs/archive/provider_orchestrator/`
  - untracked `src-rust/.codex`
  - untracked `src-rust/target/`

## 5. authority reviewed

- Reviewed authority files requested by prompt:
  - `AGENTS.md`
  - `docs/Current/MPWO_WORK_ORDER_PACK.md`
  - `docs/archive/reports/POST_M11_REPO_ASSESSMENT_20260415T071321Z.md`
  - `docs/archive/reports/POST_M11_01_CLOSEOUT_REPORT_20260415T074353Z.md`
  - `docs/archive/reports/POST_M11_02_CLOSEOUT_REPORT_20260415T082535Z.md`
  - `docs/archive/reports/POST_M11_03_CLOSEOUT_REPORT_20260415T090527Z.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md` still states it is the single active authority artifact in `docs/Current/`.
- No conflicting live current-authority artifact was found in the reviewed post-M11 chain.
- Prompt-narrowed scope is controlling for this pass: preflight only, no implementation, no staging, no commit.

Verified commands run:

- `git branch --show-current`
- `git rev-parse HEAD`
- `git status --short --branch`
- `cargo clippy -p claurst-core --all-targets -- -D warnings`
- `cargo build -p claurst-core`
- `cargo test -p claurst-core`
- targeted `rg` / `sed` inspection of:
  - `src-rust/crates/core/src/lib.rs`
  - `src-rust/crates/core/src/effort.rs`
  - `src-rust/crates/core/src/system_prompt.rs`
  - `src-rust/crates/core/tests/parity_smoke.rs`
  - `src-rust/crates/cli/src/main.rs`

## 6. current claurst-core clippy findings

Live command result:

- `cd src-rust && cargo clippy -p claurst-core --all-targets -- -D warnings` -> `FAIL`

Exact remaining failing files and lint classes in live repo reality:

- `src-rust/crates/core/src/lib.rs`
  - `derivable_impls` at `FormatterConfig` default impl
  - `doc_lazy_continuation` in `resolve_auth_async` docs
  - `manual_map` in `resolve_auth_async`
  - `single_match` in `list_sessions`
  - `field_reassign_with_default` in tests at three config setup sites
- `src-rust/crates/core/src/effort.rs`
  - `should_implement_trait` on `EffortLevel::from_str`
- `src-rust/crates/core/src/system_prompt.rs`
  - `should_implement_trait` on `OutputStyle::from_str`
  - `vec_init_then_push` in `build_system_prompt`

Non-clippy warning surface from live test probe:

- `cd src-rust && cargo test -p claurst-core` -> `PASS`
- Remaining warning:
  - `src-rust/crates/core/tests/parity_smoke.rs:9`
  - `unused import: TranscriptEntry`

Boundary confirmation against accepted `POST-M11-03` closeout:

- Remaining clippy hard failures are still confined to:
  - `src-rust/crates/core/src/lib.rs`
  - `src-rust/crates/core/src/effort.rs`
  - `src-rust/crates/core/src/system_prompt.rs`
- The expected test-warning surface is still:
  - `src-rust/crates/core/tests/parity_smoke.rs`
- No additional `claurst-core` file appeared in the live failure set.

## 7. recommended tranche boundary

Recommended narrowest realistic `POST-M11-04` tranche:

- Keep this ticket limited to `claurst-core`.
- Scope the execution ticket to:
  - `src-rust/crates/core/src/lib.rs`
  - `src-rust/crates/core/src/system_prompt.rs`
  - `src-rust/crates/core/tests/parity_smoke.rs`
- Split out:
  - `src-rust/crates/core/src/effort.rs`

Reasoning:

- `lib.rs` findings are mechanical or test-only cleanup and do not require cross-crate API changes.
- `system_prompt.rs` includes one API-shape lint, but repo-local evidence shows its current `from_str` usage is contained inside `claurst-core` (`src-rust/crates/core/src/lib.rs`), so the adjustment can still stay crate-local.
- `effort.rs` is different: repo-local evidence shows `EffortLevel::from_str` is called from `src-rust/crates/cli/src/main.rs` for CLI effort parsing and `/effort` handling. A clean fix there is not safely reviewable as `claurst-core`-only unless ticket authority explicitly widens to coordinated cross-crate changes and broader validation.

Conclusion:

- The candidate scope can remain inside `claurst-core` only if `effort.rs` is excluded from `POST-M11-04`.
- All four remaining files should not stay together in one low-risk tranche.
- Recommended split:
  - `POST-M11-04`: low-risk `claurst-core` cleanup tranche
  - later ticket: semantic/API-shape tranche for `effort.rs` and any required downstream callsite updates

## 8. risk / non-regression findings

Low-risk mechanical or docs-only cleanup:

- `lib.rs`
  - `derivable_impls`
  - `doc_lazy_continuation`
  - `manual_map`
  - `single_match`
  - `field_reassign_with_default` in tests
- `system_prompt.rs`
  - `vec_init_then_push`
- `tests/parity_smoke.rs`
  - unused import warning

Semantic/API-shape decisions:

- `effort.rs`
  - `should_implement_trait` on `EffortLevel::from_str`
  - risk: changing this cleanly can affect accepted CLI effort parsing behavior outside `claurst-core`
  - repo-local evidence:
    - `src-rust/crates/cli/src/main.rs:722`
    - `src-rust/crates/cli/src/main.rs:1958`
- `system_prompt.rs`
  - `should_implement_trait` on `OutputStyle::from_str`
  - lower risk than `effort.rs` in repo-local reality because current usage is contained inside `claurst-core`
  - still requires care to preserve current fallback-to-`Default` behavior

Already-accepted runtime behavior at risk if touched incautiously:

- `lib.rs`
  - `resolve_auth_async` is runtime auth-path code
  - `list_sessions` is runtime session enumeration code
- `system_prompt.rs`
  - output-style parsing and prompt assembly affect prompt construction behavior
- `effort.rs`
  - effort parsing feeds user-facing CLI behavior outside crate-local validation scope

Non-regression assessment:

- No evidence of drift reopening accepted M11 runtime behavior in this preflight.
- Risk is concentrated in API-shape cleanup, not in newly discovered functional defects.

## 9. recommended validation gate

For the recommended low-risk `POST-M11-04` scope (`lib.rs` + `system_prompt.rs` + `tests/parity_smoke.rs`):

- Recommended blocking gates:
  - `cd src-rust && cargo build -p claurst-core`
  - `cd src-rust && cargo test -p claurst-core`
- Recommended non-blocking progress probe:
  - `cd src-rust && cargo clippy -p claurst-core --all-targets -- -D warnings`

Explicit answer on full clippy as the blocking gate for this next ticket:

- `no` for the recommended narrow core-only tranche, because `effort.rs` would remain intentionally excluded and would keep the crate-level clippy gate red.
- `yes` only if the next ticket absorbs the `effort.rs` API-shape work and also accepts widened downstream validation beyond `claurst-core` to protect CLI behavior.

## 10. drift / blockers, if any

Drift:

- No structural drift was found in the remaining `claurst-core` clippy failure boundary relative to `POST-M11-03`.
- The live failure set still matches the expected confined surfaces.

Notes / blockers:

- Worktree remains noisy, so later execution/review should keep the review basis explicit.
- The only scope-defining blocker is `effort.rs`:
  - a clean lint remediation there is not safely `claurst-core`-only in current repo reality
  - core-only validation would not cover the known `claurst-cli` callsites

## 11. exact recommendation for next step

Open `POST-M11-04` as a low-risk `claurst-core` cleanup ticket limited to:

- `src-rust/crates/core/src/lib.rs`
- `src-rust/crates/core/src/system_prompt.rs`
- `src-rust/crates/core/tests/parity_smoke.rs`

Execution guidance for that next ticket:

- preserve existing runtime semantics
- treat `cargo build -p claurst-core` and `cargo test -p claurst-core` as blocking gates
- treat full `cargo clippy -p claurst-core --all-targets -- -D warnings` as a progress probe, not the closure gate

Do not include `src-rust/crates/core/src/effort.rs` in that low-risk tranche unless authority is explicitly widened to:

- allow coordinated downstream callsite updates outside `claurst-core`
- and require broader validation to protect accepted CLI effort behavior
