# Post-M11 Repo Assessment

## 1. timestamp UTC

`2026-04-15T07:13:21Z`

## 2. branch / HEAD observed

- Branch: `feature/provider-resolution-seam`
- HEAD: `b157924e130fdf71c09a3787b47dd5eb1f31d542`
- Worktree state: dirty / noisy
- Observed noise:
  - modified `.gitignore`
  - untracked docs/report artifacts under `docs/Current/` and `docs/archive/reports/`
  - untracked `.codex`
  - untracked `src-rust/target/`

## 3. baseline confirmation

- Supplied accepted M11-12 closeout hash: `b157924e130fdf71c09a3787b47dd5eb1f31d542`
- Live `HEAD` match: `yes`
- Accepted split baseline checked against live repo and closeout trail:
  - `08R`
  - `08B`
  - `09`
  - `10A`
  - `10B1`
  - `10B2`
  - `11`
  - `12`
- Basis:
  - live `HEAD` matches the accepted closeout commit exactly
  - `TASK-M11-12` is closed at that hash per `docs/archive/reports/TASK-M11-12_CLOSEOUT_REPORT_20260415T065244Z.md`
  - live runtime surfaces still show the accepted split-path implementation

## 4. M11 completion assessment

**A. Is M11 functionally complete on the accepted split path?**

`yes`

Why:
- The accepted closeout hash is live at `HEAD`.
- The accepted M11-12 closeout already records blocking validation success:
  - `cargo build --workspace` `PASS`
  - `cargo test -p claurst-api` `PASS`
  - `cargo test -p claurst-query` `PASS`
- The live code still contains the expected split-path runtime seams:
  - layered `SessionBudget` behavior in `src-rust/crates/query/src/session_budget.rs`
  - child `max_tokens`, `allow_fallback`, and `budget_usd` carriage in `src-rust/crates/query/src/agent_tool.rs` and `src-rust/crates/tools/src/team_tool.rs`
  - same-domain fallback and capability matching in `src-rust/crates/query/src/provider_resolution.rs`
  - `HealthCache` implementation in `src-rust/crates/query/src/health_cache.rs`
  - M11-11 event variants are live in `src-rust/crates/query/src/lib.rs:400-427`
- The required read-only build probe rerun in this assessment still passes:
  - `cd src-rust && cargo build --workspace` -> `PASS`

What is not materially missing:
- No confirmed post-M11 functional gap remains on the accepted runtime path.
- Remaining issues are cleanup debt, authority-shape drift, and deliberate architectural deferrals.

## 5. technical debt register

Functional-gap status:
- No open M11 functional gap is confirmed at current `HEAD`.

### 5.1 Workspace rustfmt drift

- Title: `Workspace rustfmt drift in query crate`
- Category: `repo hygiene / tooling debt`
- Evidence:
  - `cd src-rust && cargo fmt --all -- --check` -> `FAIL`
  - Current failure output is limited to:
    - `src-rust/crates/query/src/health_cache.rs`
    - `src-rust/crates/query/src/provider_resolution.rs`
  - This matches the non-blocking debt recorded in the accepted M11-12 closeout.
- Impact:
  - Keeps workspace fmt gate red.
  - Adds avoidable diff noise around M11-adjacent files.
- Urgency: `medium`
- Blocks next work: `no`
- Recommended follow-up ticket shape:
  - one narrow rustfmt-baseline cleanup ticket touching only the two reported files

### 5.2 Workspace clippy debt in `claurst-core`

- Title: `Workspace clippy gate still blocked by claurst-core baseline debt`
- Category: `repo hygiene / tooling debt`
- Evidence:
  - `cd src-rust && cargo clippy --workspace --all-targets -- -D warnings` -> `FAIL`
  - First failing crate is still `claurst-core`
  - Current live failures remain in files such as:
    - `crates/core/src/session_storage.rs`
    - `crates/core/src/attachments.rs`
    - `crates/core/src/feature_flags.rs`
    - `crates/core/src/skill_discovery.rs`
    - `crates/core/src/bash_classifier.rs`
    - `crates/core/src/system_prompt.rs`
- Impact:
  - Prevents a trustworthy workspace-wide `clippy -D warnings` acceptance gate.
  - Reduces signal for later tickets because failures stop before downstream crates.
- Urgency: `high`
- Blocks next work: `not a runtime blocker`, but `yes` if the next major work wants a blocking workspace clippy gate
- Recommended follow-up ticket shape:
  - begin a dedicated `claurst-core` lint-remediation ladder in small tranches by file cluster, not one broad cleanup blob

### 5.3 Remaining warning debt in `claurst-query`

- Title: `claurst-query is still not no-deps clippy clean`
- Category: `repo hygiene / tooling debt`
- Evidence:
  - `cd src-rust && cargo clippy -p claurst-query --all-targets --no-deps -- -D warnings` -> `FAIL`
  - Representative live failures:
    - `crates/query/src/compact.rs` (`unused import`, `single_match`, `redundant_closure`, `if_same_then_else`)
    - `crates/query/src/agent_tool.rs` (`unwrap_or_default`, `unnecessary_map_or`, test-only `field_reassign_with_default`, `type_complexity`)
    - `crates/query/src/coordinator.rs` (`needless_lifetimes`, `borrowed_box`)
    - `crates/query/src/provider_resolution.rs` (`needless_borrow`)
    - `crates/query/src/skill_prefetch.rs` (`unnecessary_map_or`, `manual_strip`)
    - `crates/query/src/lib.rs` (`too_many_arguments`, `unnecessary_map_or`, `items_after_test_module`)
- Impact:
  - M11-owning crate still cannot participate in a blocking lint gate.
  - Some debt is test-organization related rather than runtime-risk related.
- Urgency: `high`
- Blocks next work: `no`
- Recommended follow-up ticket shape:
  - a dedicated query lint-cleanup ticket, ideally split into:
    - runtime/style fixes
    - test-layout fixes

### 5.4 Remaining warning debt in `claurst-api`

- Title: `claurst-api is still not no-deps clippy clean`
- Category: `repo hygiene / tooling debt`
- Evidence:
  - `cd src-rust && cargo clippy -p claurst-api --all-targets --no-deps -- -D warnings` -> `FAIL`
  - Representative live failures:
    - `crates/api/src/provider_types.rs` (`derivable_impls`)
    - `crates/api/src/transform.rs` (`wrong_self_convention`)
    - `crates/api/src/registry.rs` (`for_kv_map`)
    - `crates/api/src/providers/google.rs` (`manual_map`, `collapsible_match`)
    - `crates/api/src/providers/openai_compat.rs` (`ptr_arg`)
    - `crates/api/src/providers/openai.rs` (`items_after_test_module`)
    - `crates/api/src/model_registry.rs` (`needless_borrow`, `explicit_auto_deref`)
    - `crates/api/src/lib.rs` (`derivable_impls`, `new_without_default`)
- Impact:
  - The API crate also fails a meaningful crate-local lint gate.
  - Some findings touch M11-adjacent files such as `provider_types.rs`.
- Urgency: `medium`
- Blocks next work: `no`
- Recommended follow-up ticket shape:
  - one crate-local lint cleanup ticket after `claurst-query`, or split provider-adapter cleanup from shared API-surface cleanup

### 5.5 Authority and documentation fragmentation after the split path

- Title: `Current authority is fragmented and partially stale`
- Category: `process / documentation debt`
- Evidence:
  - `docs/Current/MPWO_WORK_ORDER_PACK.md` is labeled canonical, but in repo reality it is a revision summary rather than a clean executable authority artifact.
  - `docs/Current/TASK-M11-10_SPLIT_PLAN_AND_TEMPORARY_AUTHORITY_AMENDMENT.md` is explicitly temporary and still lives in `docs/Current/`.
  - `docs/Current/` also duplicates accepted reports that already exist under `docs/archive/reports/`.
  - The earlier `M11_CONVERGENCE_REVIEW_20260415T042825Z.md` is now stale on at least two points:
    - it says no M11-11 event variants are present, but live `src-rust/crates/query/src/lib.rs:400-427` contains them
    - it says no dedicated M11-12 test landing is present, but accepted M11-12 closeout committed test updates in `provider_types.rs`, `agent_tool.rs`, and `lib.rs`
- Impact:
  - High process risk for future ticketing and reviews.
  - Encourages re-reading superseded assumptions from old planning text.
- Urgency: `high`
- Blocks next work: `yes` for clean authority-driven execution discipline; `no` for raw coding ability
- Recommended follow-up ticket shape:
  - a docs-only consolidation ticket that writes one post-M11 authority artifact, retires or clearly demotes stale `docs/Current` files, and records the accepted post-M11 baseline in one place

### 5.6 Test placement and test-layout cleanup seam

- Title: `Post-M11 tests are coherent enough, but organization now deserves cleanup`
- Category: `repo hygiene / tooling debt`
- Evidence:
  - `src-rust/crates/query`, `src-rust/crates/api`, and `src-rust/crates/tools` have no crate-level `tests/` directories in current layout; tests are overwhelmingly inline in source files.
  - `src-rust/crates/core` does have a `tests/` directory, so the repo is inconsistent by crate.
  - Current clippy failures include `items_after_test_module` in:
    - `crates/query/src/lib.rs`
    - `crates/api/src/providers/openai.rs`
- Impact:
  - Not a correctness problem.
  - Makes large acceptance-style tests harder to isolate from production-file lint/layout rules.
- Urgency: `medium`
- Blocks next work: `no`
- Recommended follow-up ticket shape:
  - one targeted test-organization cleanup ticket moving larger scenario/acceptance tests out of long source files where practical, while leaving genuinely local unit tests inline

### 5.7 Review-basis noise in the worktree

- Title: `Noisy worktree complicates future patch isolation`
- Category: `repo hygiene / tooling debt`
- Evidence:
  - Current `git status --short --branch` shows modified `.gitignore` and many unrelated untracked docs artifacts plus `src-rust/target/`
- Impact:
  - Raises patch-hygiene and review-basis ambiguity for later tickets
  - Especially risky in this repo because closeout/review discipline depends on explicit, narrow diffs
- Urgency: `medium`
- Blocks next work: `no`
- Recommended follow-up ticket shape:
  - a narrow housekeeping pass or explicit baseline-cleanup decision before the next long ticket ladder

### 5.8 Deliberate architectural deferrals still parked outside M11

- Title: `Accepted architectural deferrals remain deferred, not incomplete`
- Category: `deliberate architectural deferral`
- Evidence:
  - `ToolContext` still contains `provider_registry` and `model_registry` only at `src-rust/crates/tools/src/lib.rs:216-234`
  - No `ToolContext.session_budget` or `ToolContext.health_cache` field exists
  - The split amendment explicitly forbids assuming those fields and calls its own `10B` authority temporary
  - Fallback still uses runtime-local `HealthCache::new()` in `agent_tool.rs`
  - TeamCreate outer-cancellation was not redesigned as part of the accepted M11 path
- Impact:
  - No current runtime hole is proven.
  - These are future architecture choices, not post-M11 defects.
- Urgency: `low`
- Blocks next work: `no`
- Recommended follow-up ticket shape:
  - only open these when a later milestone has a concrete need for shared cache ownership, cross-crate budget carriage, or TeamCreate cancellation redesign

## 6. blocking vs non-blocking follow-up items

Blocking next major work:
- No confirmed runtime blocker remains.
- One process blocker should be treated as first priority if the repo wants to keep authority-driven execution clean:
  - consolidate post-M11 authority and retire stale `docs/Current` artifacts

Non-blocking but should be cleaned up soon:
- workspace rustfmt drift in `provider_resolution.rs` and `health_cache.rs`
- `claurst-core` clippy baseline debt
- `claurst-query` crate-local clippy debt
- `claurst-api` crate-local clippy debt
- test-layout cleanup around large inline test modules
- noisy worktree / review-basis housekeeping

Safe to leave as deferred backlog:
- `ToolContext.session_budget`
- `ToolContext.health_cache`
- global/shared `HealthCache` plumbing
- TeamCreate outer-cancellation redesign

## 7. recommended post-M11 ticket ladder

1. `POST-M11-01` — Post-M11 authority consolidation
   - docs-only
   - produce one current authority artifact that states M11 is complete at `b157924e130fdf71c09a3787b47dd5eb1f31d542`
   - retire, archive, or explicitly demote stale `docs/Current` artifacts

2. `POST-M11-02` — Workspace rustfmt baseline cleanup
   - scope only the currently failing files:
     - `src-rust/crates/query/src/provider_resolution.rs`
     - `src-rust/crates/query/src/health_cache.rs`

3. `POST-M11-03` — `claurst-core` clippy cleanup tranche 1
   - start with the first failing file cluster from `session_storage.rs`, `attachments.rs`, `feature_flags.rs`, `skill_discovery.rs`, `bash_classifier.rs`, and `system_prompt.rs`
   - keep tranches narrow until workspace clippy reaches downstream crates cleanly

4. `POST-M11-04` — `claurst-query` lint cleanup
   - fix crate-local no-deps clippy debt in query-owned files
   - keep acceptance behavior unchanged

5. `POST-M11-05` — `claurst-api` lint cleanup
   - fix crate-local no-deps clippy debt in API-owned files
   - include `provider_types.rs` cleanup only as lint/style work, not behavior redesign

6. `POST-M11-06` — Query/API test-organization cleanup
   - move larger acceptance-style tests out of oversized source files where useful
   - resolve `items_after_test_module` findings cleanly

7. `POST-M11-07` — Worktree / patch-hygiene housekeeping
   - isolate or formalize `.gitignore`, docs noise, and generated-path expectations before the next long implementation ladder

Deferred backlog only when justified by future feature work:
- `POST-M11-ARCH-01` — shared/global `HealthCache` decision
- `POST-M11-ARCH-02` — `ToolContext` budget/cache carriage reconsideration
- `POST-M11-ARCH-03` — TeamCreate outer-cancellation redesign

First ticket recommendation:
- `POST-M11-01` should be first.
- Reason: this repo is authority-driven, and the current `docs/Current` shape is now the clearest source of future execution mistakes.

## 8. whether repo is ready to move beyond M11

`yes`

Qualification:
- ready to move beyond M11 feature work: `yes`
- ready to claim a clean repo baseline: `no`

Meaning:
- M11 is complete.
- The repo still carries cleanup debt that should be handled deliberately rather than mislabeled as unfinished M11 work.

## 9. notes / risks

- Do not reopen accepted M11 tickets to absorb generic lint or formatting cleanup.
- The strongest current risk is documentation/process confusion, not missing runtime behavior.
- Any future ticket that reinstates old planning assumptions about `ToolContext.session_budget` or `ToolContext.health_cache` would be reopening rejected seams, not cleaning up M11.
- If a future milestone requires blocking `clippy -D warnings`, it needs explicit baseline-cleanup authority first.
- The earlier convergence review should not be reused as the latest factual state without correction, because live repo reality has advanced beyond parts of that report.
