# POST-M11-07B Preflight Report

## 1. ticket id

`POST-M11-07B`

## 2. verdict

`PASS-WITH-NOTES`

## 3. timestamp UTC

`2026-04-15T14:25:31Z`

## 4. branch / HEAD observed

- Branch observed: `feature/provider-resolution-seam`
- HEAD observed: `03a03573f8183783047f564f259319a7b53fc0b5`
- Accepted HEAD expected by authority: `03a03573f8183783047f564f259319a7b53fc0b5`
- HEAD match verdict: `yes`
- Working tree status: `dirty`
- Read-only status notes:
  - tracked modification: `.gitignore`
  - untracked repo-noise present under `.codex/`, `docs/Orchestrator_planning/`, `docs/archive/provider_orchestrator/`, many `docs/archive/reports/*.md`, and `src-rust/target/`
- Preflight command verification run:
  - `git branch --show-current`
  - `git rev-parse HEAD`
  - `git status --short --branch`

## 5. authority reviewed

- Repo authority reviewed:
  - `AGENTS.md`
  - `docs/Current/MPWO_WORK_ORDER_PACK.md`
- Historical/accepted evidence reviewed:
  - `docs/archive/reports/POST_M11_REPO_ASSESSMENT_20260415T071321Z.md`
  - `docs/archive/reports/POST_M11_03_CLOSEOUT_REPORT_20260415T090527Z.md`
  - `docs/archive/reports/POST_M11_04_CLOSEOUT_REPORT_20260415T102652Z.md`
  - `docs/archive/reports/POST_M11_05_CLOSEOUT_REPORT_20260415T122710Z.md`
  - `docs/archive/reports/POST_M11_06A_CLOSEOUT_REPORT_20260415T130143Z.md`
  - `docs/archive/reports/POST_M11_06B_CLOSEOUT_REPORT_20260415T132352Z.md`
  - `docs/archive/reports/POST_M11_06C_CLOSEOUT_REPORT_20260415T134926Z.md`
  - `docs/archive/reports/POST_M11_07_PREFLIGHT_REPORT_20260415T135950Z.md`
  - `docs/archive/reports/POST_M11_07A_CLOSEOUT_REPORT_20260415T141904Z.md`
- Sole active current-authority artifact check:
  - `docs/Current/MPWO_WORK_ORDER_PACK.md` still states it is the single active authority artifact in `docs/Current/`
  - no conflicting current-authority artifact was found
  - verdict: `yes`

## 6. current claurst-api clippy findings

- Validation commands run in this preflight:
  - `cd src-rust && cargo build -p claurst-api` -> `PASS`
  - `cd src-rust && cargo test -p claurst-api` -> `PASS`
  - `cd src-rust && cargo clippy -p claurst-api --all-targets --no-deps -- -D warnings` -> `FAIL`
- Build/test result detail:
  - `cargo test -p claurst-api` passed with `32 passed; 0 failed`
- Live clippy failure set is still confined only to the four expected files:
  - `src-rust/crates/api/src/transform.rs`
  - `src-rust/crates/api/src/providers/google.rs`
  - `src-rust/crates/api/src/providers/openai.rs`
  - `src-rust/crates/api/src/providers/bedrock.rs`
- No additional live `claurst-api` file appeared in the failure set.

Exact live lint sites inspected:

- `src-rust/crates/api/src/transform.rs:35-38`
  - `wrong_self_convention`
  - site: `MessageTransformer::from_provider(&self, ...)`
  - note: this trait method is also implemented in `src-rust/crates/api/src/transformers/anthropic.rs` and `src-rust/crates/api/src/transformers/openai_chat.rs`
- `src-rust/crates/api/src/providers/google.rs:147-165`
  - `manual_map`
  - image URL fallback in `content_block_to_part`
- `src-rust/crates/api/src/providers/google.rs:180-197`
  - `manual_map`
  - document URL fallback in `content_block_to_part`
- `src-rust/crates/api/src/providers/google.rs:297-312`
  - `collapsible_match`
  - schema `required` filtering in `sanitize_schema`
- `src-rust/crates/api/src/providers/openai.rs:597-663`
  - `items_after_test_module`
  - `mod tests` precedes `impl LlmProvider for OpenAiProvider`
- `src-rust/crates/api/src/providers/bedrock.rs:221-226`
  - `let_and_return`
  - signing-key HMAC chain local return
- `src-rust/crates/api/src/providers/bedrock.rs:335-378`
  - `only_used_in_recursion`
  - `role: &Role` is only passed through recursive `ToolResultContent::Blocks` conversion
- `src-rust/crates/api/src/providers/bedrock.rs:721-739`
  - `while_let_loop`
  - streaming JSON drain loop

## 7. recommended tranche boundary

- Can the remaining four files stay together as one next tranche? `no`
- Should `transform.rs` stay with the provider files? `no`
- Can the next tranche remain `claurst-api` only? `yes`

Recommended split:

- Next tranche for `POST-M11-07B`: provider-runtime cleanup only
  - `src-rust/crates/api/src/providers/google.rs`
  - `src-rust/crates/api/src/providers/openai.rs`
  - `src-rust/crates/api/src/providers/bedrock.rs`
- Later separate tranche: semantic/API-shape cleanup
  - `src-rust/crates/api/src/transform.rs`
  - plus any same-crate transformer implementation touchpoints required by the trait rename or equivalent fix

Reasoning:

- `transform.rs` is not a mechanical single-file lint. The live warning is reported only in `transform.rs`, but a correct fix likely reaches the `MessageTransformer` implementers in `src-rust/crates/api/src/transformers/*`. That makes it semantic/API-shape work, not a narrow follow-on lint pass.
- `google.rs`, `openai.rs`, and `bedrock.rs` are still medium-sensitivity files, but the reported lints are file-local runtime/layout cleanups rather than shared trait-surface changes.
- Keeping all four together would mix:
  - shared trait/API-shape cleanup
  - provider request-shaping/runtime cleanup
  - test-layout reordering
- That would widen review risk and make regression attribution harder.

Smallest safe next tranche verdict:

- The narrowest realistic safe `POST-M11-07B` tranche is the three-file provider-runtime set:
  - `google.rs`
  - `openai.rs`
  - `bedrock.rs`

## 8. risk / non-regression findings

- `src-rust/crates/api/src/transform.rs`
  - Risk level: `high`
  - Reason: `wrong_self_convention` is attached to a public trait method shape, not an isolated local expression
  - Non-regression concern: request/response transformation contract for provider adapters
- `src-rust/crates/api/src/providers/google.rs`
  - Risk level: `medium`
  - Reason: lints sit inside request-body and schema transformation code
  - Non-regression concern: provider transformation / request-shaping behavior
- `src-rust/crates/api/src/providers/openai.rs`
  - Risk level: `medium`
  - Reason: lint is layout-only, but the file is the generic OpenAI-compatible chat adapter used for OpenAI-style request/message shaping
  - Non-regression concern: generic OpenAI-compatible message conversion and Responses API fallback messaging
- `src-rust/crates/api/src/providers/bedrock.rs`
  - Risk level: `medium`
  - Reason: lints sit inside signing and streaming/parser logic
  - Non-regression concern: provider request-shaping and Bedrock stream parsing

Accepted M11 runtime behavior exposure:

- Provider trust-domain behavior:
  - accepted behavior lives primarily in `src-rust/crates/api/src/provider_types.rs`
  - the recommended provider-runtime tranche does not need to reopen it
- Same-domain fallback assumptions:
  - accepted behavior lives primarily in `src-rust/crates/query/src/provider_resolution.rs`
  - the recommended provider-runtime tranche does not need to reopen it
- Hosted Ollama compatibility expectations:
  - accepted behavior lives primarily in `src-rust/crates/query/src/provider_resolution.rs`, `src-rust/crates/api/src/error_handling.rs`, and `src-rust/crates/api/src/providers/openai_compat.rs`
  - `openai.rs` is adjacent OpenAI-compatible adapter infrastructure, so edits should remain minimal and layout-only where possible
- Provider transformation / request-shaping behavior:
  - directly exposed in `google.rs`, `openai.rs`, `bedrock.rs`, and most strongly in `transform.rs`
  - this is the main reason to keep `transform.rs` out of the next provider-runtime tranche

## 9. recommended validation gate

For the recommended next tranche (`google.rs` + `openai.rs` + `bedrock.rs`):

- Blocking gates:
  - `cd src-rust && cargo build -p claurst-api`
  - `cd src-rust && cargo test -p claurst-api`
- Full crate-local clippy gate:
  - `cd src-rust && cargo clippy -p claurst-api --all-targets --no-deps -- -D warnings`
  - status for the next tranche: `progress probe only`

Why full crate-local clippy should not yet be blocking:

- `transform.rs` will remain a known out-of-scope live failure if the next tranche is correctly split
- making full crate-local clippy blocking for `07B` would force the semantic/API-shape work back into the provider-runtime ticket
- that would violate the narrowest-safe split identified by this preflight

Promotion condition:

- Full crate-local `clippy -D warnings` can become the blocking gate only on the later ticket that closes the final remaining split segment
- If provider-runtime goes first, clippy remains a progress probe until the follow-on `transform.rs` semantic/API-shape ticket lands

## 10. drift / blockers, if any

- Drift found:
  - working tree is not clean; unrelated tracked/untracked repo noise is present
  - this does not block read-only preflight, but it does mean the next implementation ticket must state review basis and patch isolation explicitly
- Structural note:
  - live clippy output is confined to the expected four files
  - however, the `transform.rs` fix path is not truly four-files-only in practice because the trait method shape is implemented elsewhere in `claurst-api`
- Blocker verdict: `none for preflight`
- Implementation caution for the next ticket:
  - do not let a `transform.rs` fix silently expand into a mixed provider-runtime/API-shape ticket

## 11. exact recommendation for next step

- Open `POST-M11-07B` as a `claurst-api` provider-runtime lint tranche only.
- Scope it to:
  - `src-rust/crates/api/src/providers/google.rs`
  - `src-rust/crates/api/src/providers/openai.rs`
  - `src-rust/crates/api/src/providers/bedrock.rs`
- Keep `src-rust/crates/api/src/transform.rs` out of `07B`.
- Treat blocking validation for `07B` as:
  - `cd src-rust && cargo build -p claurst-api`
  - `cd src-rust && cargo test -p claurst-api`
- Treat full crate-local clippy as:
  - `cd src-rust && cargo clippy -p claurst-api --all-targets --no-deps -- -D warnings`
  - `progress probe only` until the separate `transform.rs` semantic/API-shape tranche is completed
- After the provider-runtime tranche closes, open a separate `claurst-api` semantic/API-shape lint ticket for `transform.rs` and its in-crate transformer implementations.
