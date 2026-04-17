# POST-M11-07 Preflight Report

## 1. ticket id

`POST-M11-07`

## 2. verdict

`PASS-WITH-NOTES`

## 3. timestamp UTC

`2026-04-15T13:59:50Z`

## 4. branch / HEAD observed

- Branch: `feature/provider-resolution-seam`
- HEAD: `8b20182177f6d3689ce133114245c8ff7d587791`
- HEAD matches accepted `POST-M11-06C` baseline: `yes`
- Working tree state: dirty / noisy
- Observed repo noise:
  - modified `.gitignore`
  - untracked `.codex`
  - untracked docs/report artifacts under `docs/archive/reports/` and `docs/Orchestrator_planning/`
  - untracked `docs/archive/provider_orchestrator/`
  - untracked `src-rust/target/`
- Staged changes observed: `none`

## 5. authority reviewed

- `AGENTS.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md`
- `docs/archive/reports/POST_M11_REPO_ASSESSMENT_20260415T071321Z.md`
- `docs/archive/reports/POST_M11_03_CLOSEOUT_REPORT_20260415T090527Z.md`
- `docs/archive/reports/POST_M11_04_CLOSEOUT_REPORT_20260415T102652Z.md`
- `docs/archive/reports/POST_M11_05_CLOSEOUT_REPORT_20260415T122710Z.md`
- `docs/archive/reports/POST_M11_06A_CLOSEOUT_REPORT_20260415T130143Z.md`
- `docs/archive/reports/POST_M11_06B_CLOSEOUT_REPORT_20260415T132352Z.md`
- `docs/archive/reports/POST_M11_06C_CLOSEOUT_REPORT_20260415T134926Z.md`
- Verified commands run:
  - `git branch --show-current`
  - `git rev-parse HEAD`
  - `git status --short --branch`
  - `rg --files docs/Current`
  - `cargo build -p claurst-api`
  - `cargo test -p claurst-api`
  - `cargo clippy -p claurst-api --all-targets --no-deps -- -D warnings`
- Sole active current-authority artifact verdict: `yes`
- Basis:
  - `docs/Current/MPWO_WORK_ORDER_PACK.md` explicitly states it is the single active authority artifact in `docs/Current/`
  - `rg --files docs/Current` confirms historical files still exist there, but the pack explicitly demotes them from controlling authority

## 6. current claurst-api clippy findings

Baseline probes:

- `cd src-rust && cargo build -p claurst-api` -> `PASS`
- `cd src-rust && cargo test -p claurst-api` -> `PASS`
- Test result summary: `32 passed; 0 failed`
- `cd src-rust && cargo clippy -p claurst-api --all-targets --no-deps -- -D warnings` -> `FAIL`

Current live failing files and lint classes:

- `src-rust/crates/api/src/provider_types.rs`
  - `derivable_impls`
- `src-rust/crates/api/src/transform.rs`
  - `wrong_self_convention`
- `src-rust/crates/api/src/registry.rs`
  - `for_kv_map`
- `src-rust/crates/api/src/providers/google.rs`
  - `manual_map` x2
  - `collapsible_match`
- `src-rust/crates/api/src/providers/openai_compat.rs`
  - `ptr_arg`
- `src-rust/crates/api/src/providers/openai.rs`
  - `items_after_test_module`
- `src-rust/crates/api/src/providers/bedrock.rs`
  - `let_and_return`
  - `only_used_in_recursion`
  - `while_let_loop`
- `src-rust/crates/api/src/providers/copilot.rs`
  - `redundant_closure`
- `src-rust/crates/api/src/model_registry.rs`
  - `needless_borrow`
  - `explicit_auto_deref`
- `src-rust/crates/api/src/lib.rs`
  - `derivable_impls`
  - `new_without_default`

Assessment drift versus prior evidence:

- The earlier post-M11 assessment remains directionally correct for:
  - `provider_types.rs`
  - `transform.rs`
  - `registry.rs`
  - `providers/google.rs`
  - `providers/openai_compat.rs`
  - `providers/openai.rs`
  - `model_registry.rs`
  - `lib.rs`
- The live failure set has shifted broader than that representative list:
  - `src-rust/crates/api/src/providers/bedrock.rs` now fails live `clippy -D warnings`
  - `src-rust/crates/api/src/providers/copilot.rs` now fails live `clippy -D warnings`
- Conclusion: the prior representative list is `partially accurate but no longer complete`

## 7. recommended tranche boundary

- The next cleanup can remain limited to `claurst-api` only: `yes`
- No live failure currently requires editing outside `src-rust/crates/api/`
- Recommended first tranche type: `small mechanical file cluster`

Recommended `POST-M11-07` tranche-1 file set:

- `src-rust/crates/api/src/provider_types.rs`
- `src-rust/crates/api/src/registry.rs`
- `src-rust/crates/api/src/model_registry.rs`
- `src-rust/crates/api/src/providers/openai_compat.rs`
- `src-rust/crates/api/src/providers/copilot.rs`
- `src-rust/crates/api/src/lib.rs`

Why this is the narrowest realistic material tranche:

- All six files have local, mechanical lint fixes only
- None of these six require request/stream control-flow restructuring
- None of these six require test-module relocation across large provider files
- None of these six require renaming a re-exported trait method
- This removes a meaningful portion of the crate-local failure set without turning into a provider-runtime cleanup blob

Files that should stay out of tranche 1:

- `src-rust/crates/api/src/transform.rs`
- `src-rust/crates/api/src/providers/google.rs`
- `src-rust/crates/api/src/providers/openai.rs`
- `src-rust/crates/api/src/providers/bedrock.rs`

Single-tranche vs split verdict:

- `POST-M11-07` should already split into:
  - low-risk mechanical cleanup first
  - later semantic / API-shape and provider-runtime cleanup
- It should not be executed as one broad all-api clippy sweep

## 8. risk / non-regression findings

- `src-rust/crates/api/src/provider_types.rs`
  - Live lint is limited to `StopReason` default derivation
  - Risk level: `low` if edits stay local to the enum derive/default marker
  - Note: this file also contains accepted provider trust-domain behavior and tests, so the ticket must avoid unrelated edits there
- `src-rust/crates/api/src/transform.rs`
  - Risk level: `high`
  - `from_provider(&self, ...)` is part of the `MessageTransformer` trait, implemented in multiple transformer modules and re-exported from crate `lib.rs`
  - This is semantic/API-shape cleanup, not a first-tranche mechanical fix
- `src-rust/crates/api/src/registry.rs`
  - Risk level: `low`
  - The live lint is a local map-iteration cleanup only
- `src-rust/crates/api/src/providers/google.rs`
  - Risk level: `medium`
  - Reported lints are mechanically fixable, but they sit in request-body/schema transformation paths
  - Better held for a provider-runtime tranche rather than mixed into the first cleanup
- `src-rust/crates/api/src/providers/openai_compat.rs`
  - Risk level: `low`
  - `&mut Vec<Value>` to `&mut [Value]` is a contained signature cleanup with local callsites
- `src-rust/crates/api/src/providers/openai.rs`
  - Risk level: `medium`
  - `items_after_test_module` requires reordering a large file that mixes production code and tests
- `src-rust/crates/api/src/providers/bedrock.rs`
  - Risk level: `medium`
  - Lints are individually mechanical, but they sit in signing and streaming/parser logic
  - Better handled as a separate provider-runtime tranche
- `src-rust/crates/api/src/providers/copilot.rs`
  - Risk level: `low`
  - Single redundant-closure fix only
- `src-rust/crates/api/src/model_registry.rs`
  - Risk level: `low`
  - Live lint is a single expression cleanup with no evident API-shape change
- `src-rust/crates/api/src/lib.rs`
  - Risk level: `low-to-medium`
  - Live lints are still mechanical, but this is crate-central public surface
  - Safe for tranche 1 only if restricted to derive/default additions and no broader refactor

Accepted M11 runtime behavior risk:

- Risk to accepted M11 runtime behavior from the recommended tranche-1 file set is `low`
- Risk rises to `medium/high` if the tranche expands into:
  - `transform.rs`
  - `providers/google.rs`
  - `providers/openai.rs`
  - `providers/bedrock.rs`

## 9. recommended validation gate

Recommended blocking gates for tranche 1:

- `cd src-rust && cargo build -p claurst-api`
- `cd src-rust && cargo test -p claurst-api`

Recommended `clippy` gate policy for tranche 1:

- `cd src-rust && cargo clippy -p claurst-api --all-targets --no-deps -- -D warnings`
  should be `progress probe only`, not a blocking gate, for the first API tranche

Why:

- Live out-of-scope failures will remain in `transform.rs`, `providers/google.rs`, `providers/openai.rs`, and `providers/bedrock.rs`
- A blocking full-crate `clippy -D warnings` gate would force tranche-1 scope expansion
- That would conflict with the requested narrow `claurst-api` cleanup direction

Promotion condition:

- Promote full crate-local `clippy -D warnings` to a blocking gate only after the later provider-runtime and semantic/API-shape tranches are explicitly opened and completed

## 10. drift / blockers, if any

Drift:

- The live `claurst-api` failure set is broader than the earlier representative assessment
- Newly surfaced live failing files:
  - `src-rust/crates/api/src/providers/bedrock.rs`
  - `src-rust/crates/api/src/providers/copilot.rs`

Patch-hygiene note:

- The repo worktree remains noisy outside the ticket scope
- Later execution will need explicit path staging and an explicit review basis

Blockers:

- No blocker prevents a narrow `claurst-api`-only tranche from proceeding
- There is a blocker to using full crate-local `clippy -D warnings` as the first-tranche blocking gate:
  - remaining failures are intentionally outside the recommended tranche-1 scope

## 11. exact recommendation for next step

- Open the next execution ticket as a narrow mechanical tranche limited to:
  - `src-rust/crates/api/src/provider_types.rs`
  - `src-rust/crates/api/src/registry.rs`
  - `src-rust/crates/api/src/model_registry.rs`
  - `src-rust/crates/api/src/providers/openai_compat.rs`
  - `src-rust/crates/api/src/providers/copilot.rs`
  - `src-rust/crates/api/src/lib.rs`
- Keep `src-rust/crates/api/src/transform.rs` out of that ticket
- Keep `src-rust/crates/api/src/providers/google.rs`, `openai.rs`, and `bedrock.rs` out of that ticket
- Use:
  - `cd src-rust && cargo build -p claurst-api`
  - `cd src-rust && cargo test -p claurst-api`
  as blocking gates
- Use:
  - `cd src-rust && cargo clippy -p claurst-api --all-targets --no-deps -- -D warnings`
  as a progress probe only for that first tranche
- After that tranche lands, rerun the same crate-local clippy command and open a follow-on ticket split into:
  - provider-runtime cleanup (`google.rs`, `openai.rs`, `bedrock.rs`)
  - semantic/API-shape cleanup (`transform.rs`)
