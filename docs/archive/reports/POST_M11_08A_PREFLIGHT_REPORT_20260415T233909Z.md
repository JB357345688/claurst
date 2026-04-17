# POST-M11-08A Preflight Report

## 1. ticket id

`POST-M11-08A`

## 2. verdict

`GO`

## 3. timestamp UTC

`20260415T233909Z`

## 4. branch / HEAD observed

- Branch: `feature/provider-resolution-seam`
- HEAD: `f841967d42663a6f7af410832634c886dc68ef99`
- Expected accepted HEAD: `f841967d42663a6f7af410832634c886dc68ef99`
- HEAD match: `yes`
- Worktree state: dirty / noisy
- Observed unrelated worktree noise:
  - modified `.gitignore`
  - untracked docs/report artifacts under `docs/archive/reports/`
  - untracked `.codex`
  - untracked `src-rust/target/`

## 5. authority reviewed

- Governing authority:
  - `AGENTS.md`
  - `docs/Current/MPWO_WORK_ORDER_PACK.md`
- Historical / accepted evidence reviewed per prompt:
  - `docs/archive/reports/POST_M11_REPO_ASSESSMENT_20260415T071321Z.md`
  - `docs/archive/reports/POST_M11_03_CLOSEOUT_REPORT_20260415T090527Z.md`
  - `docs/archive/reports/POST_M11_04_CLOSEOUT_REPORT_20260415T102652Z.md`
  - `docs/archive/reports/POST_M11_05_CLOSEOUT_REPORT_20260415T122710Z.md`
  - `docs/archive/reports/POST_M11_06A_CLOSEOUT_REPORT_20260415T130143Z.md`
  - `docs/archive/reports/POST_M11_06B_CLOSEOUT_REPORT_20260415T132352Z.md`
  - `docs/archive/reports/POST_M11_06C_CLOSEOUT_REPORT_20260415T134926Z.md`
  - `docs/archive/reports/POST_M11_07A_CLOSEOUT_REPORT_20260415T141904Z.md`
  - `docs/archive/reports/POST_M11_07B_CLOSEOUT_REPORT_20260415T144345Z.md`
  - `docs/archive/reports/POST_M11_07C_CLOSEOUT_REPORT_20260415T150817Z.md`
  - `docs/archive/reports/POST_M11_08_PREFLIGHT_REPORT_20260415T232456Z.md`
- `docs/Current/` reality checked with `find`:
  - `docs/Current/MPWO_WORK_ORDER_PACK.md` remains the sole active current-authority artifact
  - other files in `docs/Current/` remain historical / non-controlling, matching the pack
- Verified commands used in this preflight:
  - `git branch --show-current`
  - `git rev-parse HEAD`
  - `git status --short --branch`
  - `find`
  - `rg`
  - `sed`
  - `cargo build -p claurst-query`
  - `cargo test -p claurst-query`
- Verified live surfaces inspected:
  - `src-rust/crates/query/src/provider_resolution.rs`
  - `src-rust/crates/query/src/agent_tool.rs`
  - `src-rust/crates/query/src/lib.rs`
  - current crate tree under `src-rust/crates/query/src/`
  - query-side test helper usage, especially `crate::provider_auth_test_lock()`

## 6. current query test-organization findings

- `src-rust/crates/query/` still has no crate-level `tests/` tree.
- Live query test placement remains predominantly inline.
- The two strongest query-only cleanup candidates remain unchanged from `POST-M11-08`:
  - `src-rust/crates/query/src/provider_resolution.rs`
  - `src-rust/crates/query/src/agent_tool.rs`
- `src-rust/crates/query/src/lib.rs` still does not need to be reopened for this ticket.

Current file shape:

- `src-rust/crates/query/src/provider_resolution.rs`
  - file length: `1192`
  - inline `mod tests` begins at line `430`
  - test count: `29`
  - inferred trailing inline test block: about `763` lines
  - test module includes substantial local harnessing:
    - `TestProvider`
    - `EnvGuard`
    - `with_isolated_provider_auth`
    - `run_async`
    - assertion helpers and provider/model identity helpers
  - coverage focus remains:
    - same-domain fallback behavior
    - fallback-disabled and cross-domain rejection behavior
    - hosted Ollama API-base normalization and override materialization
    - provider/model resolution and capability-selection behavior

- `src-rust/crates/query/src/agent_tool.rs`
  - file length: `1425`
  - inline `mod tests` begins at line `841`
  - test count: `9`
  - inferred trailing inline test block: about `585` lines
  - test module includes substantial local harnessing:
    - `TrackingStreamingProvider`
    - registry builders for tracked providers
    - `EnvGuard`
    - `with_isolated_provider_auth`
    - `make_tool_context*`
    - `init_team_swarm_runner_once`
    - encoded team-output parsing helpers
  - coverage focus remains:
    - child `max_tokens` override behavior
    - child `allow_fallback` plumbing
    - child `budget_usd` / session-budget propagation
    - team mixed-provider dispatch
    - team observability payload structure

- `src-rust/crates/query/src/lib.rs`
  - file length: `2884`
  - test-only helper `provider_auth_test_lock()` at line `2360`
  - inline `mod tests` begins at line `2413`
  - test count: `14`
  - still contains accepted root-level coverage for:
    - legacy root-query seam behavior
    - QueryEvent observability sanitization / emission
    - root session-budget emission behavior
  - live reality does not show `lib.rs` implementation changes are needed for `POST-M11-08A`

Coupling / helper reality:

- `provider_resolution.rs` and `agent_tool.rs` both depend on the existing `crate::provider_auth_test_lock()` in `lib.rs`.
- That dependency already exists and is `pub(crate)` under `#[cfg(test)]`.
- No additional shared query-side helper module is currently required.
- The two candidate files otherwise use distinct local harnesses and distinct runtime seams.

Layout / lint reality:

- `provider_resolution.rs` and `agent_tool.rs` no longer carry active clippy debt from the accepted lint ladder.
- `lib.rs` no longer has the earlier `items_after_test_module` layout debt; `POST-M11-06C` already resolved that.
- The remaining problem is maintainability / concentration of bulky inline tests, not active runtime or lint breakage.

## 7. recommended cleanup boundary

- The ticket can safely stay `query`-only.
- `provider_resolution.rs` and `agent_tool.rs` should stay together in one narrow implementation ticket.
- `lib.rs` can remain excluded for now.

Why the two candidate files can stay together:

- They were already the accepted paired query scope in `POST-M11-06B`, so the review basis is historically coherent.
- Both need the same class of change:
  - source-local test organization only
  - no feature edits
  - no runtime seam edits
  - same validation gate
- Their only meaningful cross-file dependency is the already-existing `crate::provider_auth_test_lock()`.
- Keeping them together avoids opening a second near-identical ticket just to repeat the same extraction pattern and validation.

Why `lib.rs` should remain excluded:

- `lib.rs` already had accepted local cleanup in `POST-M11-06C`.
- Its current role in this seam is limited to:
  - the existing shared auth-test lock
  - accepted root-level observability tests
- Reopening `lib.rs` would widen risk into the crate root and mix this test-organization pass with already-accepted runtime-adjacent seams.
- Live repo reality does not show any unavoidable need to modify `lib.rs` implementation for the planned extraction.

Safest cleanup shape:

- Best route remains the source-local mixed approach identified in `POST-M11-08`.
- For this ticket specifically, that means:
  - extract the bulky inline test modules in `provider_resolution.rs` and `agent_tool.rs` into sibling `#[cfg(test)]` source-local modules/files under `src/`
  - preserve private-item access through child-module structure
  - leave `lib.rs` and other smaller query test modules inline
- Do not convert these tests into crate-level integration tests under `crates/query/tests/`.
- Do not create a broader shared harness unless implementation proves it is strictly necessary.

## 8. risk / non-regression findings

- Current query baseline is stable:
  - `cd src-rust && cargo build -p claurst-query` -> `PASS`
  - `cd src-rust && cargo test -p claurst-query` -> `PASS`
  - `cargo test -p claurst-query` summary -> `138 passed; 0 failed`

Private/internal access risk:

- Both candidate test modules exercise private/internal functions:
  - `provider_resolution.rs` tests call private helpers such as `normalize_ollama_api_base`
  - `agent_tool.rs` tests call private helpers such as `child_session_budget`, `worker_budget_exceeded_event`, and `init_team_swarm_runner`
- Moving them to external integration tests would require exposing internals or building a larger harness layer.
- Source-local child-module extraction avoids that widening.

Harness-plumbing risk:

- No live evidence suggests broader harness plumbing is required.
- The two files already keep their test harnesses mostly self-contained.
- Existing duplication such as `EnvGuard` / `with_isolated_provider_auth` is preferable to introducing a new shared test utility layer in this ticket.

Accepted M11 runtime behavior risk:

- Same-domain fallback behavior and fallback coverage:
  - primary coverage lives in `provider_resolution.rs`
  - one plumbing/dispatch check also lives in `agent_tool.rs`
  - source-local extraction preserves this without changing runtime code
- Hosted Ollama compatibility coverage:
  - lives in `provider_resolution.rs`
  - extraction is low-risk if tests remain source-local
- Child/team session-budget propagation coverage:
  - lives in `agent_tool.rs`
  - extraction is low-risk if helper access remains local
- Child `max_tokens`, `allow_fallback`, and `budget_usd` coverage:
  - lives in `agent_tool.rs`
  - extraction is low-risk if tests remain in a child module with access to local helpers
- QueryEvent observability coverage:
  - root emission/sanitization coverage remains in `lib.rs`
  - team payload construction checks remain in `agent_tool.rs`
  - this argues for keeping `lib.rs` excluded rather than reopening it

Net non-regression assessment:

- A query-only ticket covering `provider_resolution.rs` and `agent_tool.rs` together is low-to-medium risk.
- Splitting by file is not required by live repo reality.
- Widening into `lib.rs`, a shared harness layer, or integration-style tests would add risk without clear payoff.

## 9. recommended validation gate

- Blocking gates for the later implementation ticket should be:
  - `cd src-rust && cargo build -p claurst-query`
  - `cd src-rust && cargo test -p claurst-query`
- No API or workspace-wide gate is needed for this ticket.

## 10. drift / blockers, if any

- Branch / HEAD drift: `none`
- Authority drift: `none`
- Structural drift against intended narrow query-only direction: `none`
- Worktree / review-basis note:
  - the repo remains noisy from unrelated modified/untracked files
  - this is not a blocker for preflight
  - later implementation must keep an explicit path-scoped review basis
- Blockers:
  - no blocker prevents proceeding with `POST-M11-08A` as a narrow implementation ticket

## 11. exact recommendation for next step

- Proceed with `POST-M11-08A` as one narrow query-only implementation ticket.
- Keep the in-scope production files to:
  - `src-rust/crates/query/src/provider_resolution.rs`
  - `src-rust/crates/query/src/agent_tool.rs`
- Add only the minimal source-local sibling test-module files needed for those two modules.
- Keep `src-rust/crates/query/src/lib.rs` excluded.
- Use source-local extraction, not integration-test migration and not broad inline pruning.
- Preserve existing test-local harness duplication unless implementation proves a tiny shared helper is unavoidable.
- Validate only with:
  - `cd src-rust && cargo build -p claurst-query`
  - `cd src-rust && cargo test -p claurst-query`
- If a later follow-on is ever needed, reserve it for `lib.rs` only after `POST-M11-08A` lands cleanly and is accepted.
