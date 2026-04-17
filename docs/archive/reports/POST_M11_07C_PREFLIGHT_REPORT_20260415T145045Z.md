# POST-M11-07C Preflight Report

## 1. ticket id

`POST-M11-07C`

## 2. verdict

`PASS-WITH-NOTES`

## 3. timestamp UTC

`2026-04-15T14:50:45Z`

## 4. branch / HEAD observed

- Branch observed: `feature/provider-resolution-seam`
- HEAD observed: `d07600e57f85928752b381f2ccf5057496f026a5`
- Accepted HEAD expected by authority: `d07600e57f85928752b381f2ccf5057496f026a5`
- HEAD match verdict: `yes`
- Working tree status: `dirty`
- Read-only status notes:
  - tracked modification: `.gitignore`
  - untracked repo noise under `.codex/`, `docs/Orchestrator_planning/`, `docs/archive/provider_orchestrator/`, many `docs/archive/reports/*.md`, and `src-rust/target/`
- Staged changes observed: `none`

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
  - `docs/archive/reports/POST_M11_07B_PREFLIGHT_REPORT_20260415T142531Z.md`
  - `docs/archive/reports/POST_M11_07B_CLOSEOUT_REPORT_20260415T144345Z.md`
- Sole active current-authority artifact check:
  - `docs/Current/MPWO_WORK_ORDER_PACK.md` still explicitly states it is the single active authority artifact in `docs/Current/`
  - `tree -L 2 docs/Current` still shows historical files present there, but the pack demotes them from controlling authority
  - verdict: `yes`
- Commands verified in this preflight:
  - `git branch --show-current`
  - `git rev-parse HEAD`
  - `git status --short --branch`
  - `cargo build -p claurst-api`
  - `cargo test -p claurst-api`
  - `cargo clippy -p claurst-api --all-targets --no-deps -- -D warnings`
  - `rg`
  - `sed`
  - `nl -ba`

## 6. current claurst-api transform clippy findings

- `cd src-rust && cargo build -p claurst-api` -> `PASS`
- `cd src-rust && cargo test -p claurst-api` -> `PASS`
- Test result summary: `32 passed; 0 failed`
- `cd src-rust && cargo clippy -p claurst-api --all-targets --no-deps -- -D warnings` -> `FAIL`
- Live failure set confinement verdict: `yes`
- The remaining full-crate API clippy failure is still confined only to:
  - `src-rust/crates/api/src/transform.rs`
- Exact live lint:
  - `clippy::wrong_self_convention`
  - site: `src-rust/crates/api/src/transform.rs:35-38`
  - symbol: `MessageTransformer::from_provider(&self, response: &serde_json::Value)`
- Clippy guidance emitted live:
  - methods called `from_*` usually take no `self`
  - consider choosing a less ambiguous name
  - local override is possible via `#[allow(clippy::wrong_self_convention)]`

## 7. live implementer/callsite findings

- Verified trait definition:
  - `src-rust/crates/api/src/transform.rs:21-48`
  - `MessageTransformer` exposes:
    - `to_provider(&self, ...)`
    - `from_provider(&self, ...)`
    - `apply_caching(&self, ...)`
- Verified in-crate implementers:
  - `src-rust/crates/api/src/transformers/anthropic.rs:30-112`
  - `src-rust/crates/api/src/transformers/openai_chat.rs:26-68`
- Verified exact impl method sites:
  - `src-rust/crates/api/src/transformers/anthropic.rs:109-112`
  - `src-rust/crates/api/src/transformers/openai_chat.rs:63-66`
- Verified direct in-crate callsites:
  - `none found`
- Live search result:
  - `rg -n "from_provider\\(" src-rust` returned only:
    - `src-rust/crates/api/src/transform.rs`
    - `src-rust/crates/api/src/transformers/anthropic.rs`
    - `src-rust/crates/api/src/transformers/openai_chat.rs`
- Public surface note:
  - `src-rust/crates/api/src/lib.rs:69` re-exports `MessageTransformer`
  - `src-rust/crates/api/src/lib.rs:99` re-exports `AnthropicTransformer` and `OpenAiChatTransformer`
- Behavioral adjacency note:
  - no provider-resolution or adapter-selection path in `src-rust/crates/query` references `MessageTransformer` or these concrete transformer types

## 8. recommended tranche boundary

- Is `transform.rs` still the only remaining API clippy blocker? `yes`
- Can the next ticket honestly stay `transform.rs` only? `no`
- Can the next ticket remain `claurst-api` only? `yes`

Smallest safe implementation boundary:

- `src-rust/crates/api/src/transform.rs`
- `src-rust/crates/api/src/transformers/anthropic.rs`
- `src-rust/crates/api/src/transformers/openai_chat.rs`

Why this is the narrowest honest boundary:

- The lint is reported on the trait declaration only, but the trait method shape is duplicated in both in-crate implementers.
- A real API-shape fix has to keep the trait and its impls aligned.
- No additional in-repo callsite edits are required because no direct callsites were found.
- No non-api crate needs to change for the in-repo codebase to compile after the rename.

Optional but non-required follow-on touch:

- `src-rust/crates/api/src/transformers/mod.rs` only if comment text is kept consistent with the renamed method

## 9. risk / non-regression findings

- Safest remediation shape: `real trait/API-shape rename`
  - Preferred pattern: rename `from_provider` to a non-`from_*` verb while keeping `&self`
  - Recommended name: `parse_provider_response`
- Why rename is safer than changing receiver semantics:
  - changing to an associated function would alter trait ergonomics more than needed
  - it would introduce avoidable public API shape churn beyond the clippy complaint
  - it offers no demonstrated in-repo runtime benefit because the implementers are stateless unit structs and there are no internal callsites
- Why local lint allow is not the preferred choice:
  - this is a genuine public API naming inconsistency, not a false positive on complex logic
  - `#[allow(clippy::wrong_self_convention)]` would preserve the ambiguity and merely defer the cleanup
  - a local allow is only defensible if preserving the existing exported method name for downstream compatibility is explicitly more important than cleaning the API shape
- Provider transformation / request-shaping behavior risk:
  - `low` for a rename-only same-signature fix
  - `medium` if semantics are widened beyond renaming and impl synchronization
- Adapter selection behavior risk:
  - `no live in-repo impact found`
  - `ProviderRegistry`, `ModelRegistry`, and `query::provider_resolution` do not reference `MessageTransformer`
- Test impact:
  - `cargo test -p claurst-api` passes today
  - no direct transformer-specific test or in-repo callsite was found by live search
  - the next ticket should therefore expect compile-surface adaptation in the three API files, not test-behavior redesign

## 10. recommended validation gate

Recommended blocking gates for the next tranche:

- `cd src-rust && cargo build -p claurst-api`
- `cd src-rust && cargo test -p claurst-api`
- `cd src-rust && cargo clippy -p claurst-api --all-targets --no-deps -- -D warnings`

Why full crate-local clippy should now be blocking:

- the live failure set is down to one remaining semantic/API-shape issue
- the recommended next ticket fully owns that remaining failure
- using clippy only as a progress probe would no longer buy narrower scope; it would simply defer the actual closure gate

## 11. drift / blockers, if any

Drift / notes:

- The repo worktree remains noisy and non-clean outside ticket scope.
- The intended `transform.rs`-only framing is narrower than live code reality.
- Live code reality still supports a narrow ticket, but the honest scope is the three-file same-crate API tranche above, not one file.

Blocker verdict:

- `none for preflight`

Execution caution for the next ticket:

- do not widen into provider files already closed in `POST-M11-07B`
- do not widen into non-api crates
- do not use a local lint allow unless an explicit public-compatibility decision is made first

## 12. exact recommendation for next step

- Open the next execution pass as `POST-M11-07C` semantic/API-shape cleanup for `claurst-api` only.
- Scope it exactly to:
  - `src-rust/crates/api/src/transform.rs`
  - `src-rust/crates/api/src/transformers/anthropic.rs`
  - `src-rust/crates/api/src/transformers/openai_chat.rs`
- Implement the smallest real fix:
  - rename `MessageTransformer::from_provider` to `parse_provider_response`
  - update the two in-crate implementations to match
- Keep receiver semantics unchanged unless a stronger design reason is found during execution.
- Treat all three validation commands as blocking gates:
  - `cd src-rust && cargo build -p claurst-api`
  - `cd src-rust && cargo test -p claurst-api`
  - `cd src-rust && cargo clippy -p claurst-api --all-targets --no-deps -- -D warnings`
- Do not reopen accepted M11 runtime behavior, provider-runtime files, or any non-api crate in this tranche.
