# POST-M11-08 Preflight Report

## 1. ticket id

`POST-M11-08`

## 2. verdict

`PASS-WITH-NOTES`

## 3. timestamp UTC

`20260415T232456Z`

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
- Accepted evidence reviewed:
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
- `docs/Current/` reality checked with `find`:
  - `docs/Current/MPWO_WORK_ORDER_PACK.md` is still the sole active current-authority artifact
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
  - `cargo build -p claurst-api`
  - `cargo test -p claurst-api`
- Verified live surfaces inspected:
  - `src-rust/crates/query/`
  - `src-rust/crates/api/`
  - `src-rust/crates/query/src/lib.rs`
  - `src-rust/crates/query/src/agent_tool.rs`
  - `src-rust/crates/query/src/compact.rs`
  - `src-rust/crates/query/src/provider_resolution.rs`
  - `src-rust/crates/api/src/providers/openai.rs`
  - `src-rust/crates/api/src/providers/google.rs`
  - `src-rust/crates/api/src/lib.rs`
  - `src-rust/crates/api/src/providers/openai_compat.rs`
  - `src-rust/crates/api/src/transform.rs`

## 6. current test-organization findings

- `src-rust/crates/query/` has no crate-level `tests/` tree.
- `src-rust/crates/api/` has no crate-level `tests/` tree.
- Live test placement remains predominantly inline:
  - `claurst-query`: 11 source files with inline `mod tests`
  - `claurst-api`: 10 source files with inline `mod tests`
- The originally identified seam still exists, but it has narrowed:
  - oversized inline test modules are still materially present in `claurst-query`
  - the earlier lint-driven `items_after_test_module` problem is no longer materially present in the inspected candidate files
- Live `items_after_test_module` reality:
  - `src-rust/crates/query/src/lib.rs` now ends with its test module; the earlier `items_after_test_module` debt called out in `POST_M11-06B` was cleaned in `POST_M11-06C`
  - `src-rust/crates/api/src/providers/openai.rs` now ends with its test module; the earlier `items_after_test_module` debt called out in `POST_M11_REPO_ASSESSMENT` was cleaned in `POST_M11-07B`
  - no inspected query/API candidate file currently shows obvious live `items_after_test_module`-style layout debt
- Strongest live cleanup candidates are query-side, not API-side:
  - `src-rust/crates/query/src/provider_resolution.rs`
    - file length: `1192`
    - test module starts at line `430`
    - inferred trailing test block size: about `763` lines
    - tests include substantial internal harnessing around fake providers, auth-store setup, and fallback-resolution behavior
  - `src-rust/crates/query/src/agent_tool.rs`
    - file length: `1425`
    - test module starts at line `841`
    - inferred trailing test block size: about `585` lines
    - tests include internal runner/provider scaffolding and team/child-agent dispatch coverage
  - `src-rust/crates/query/src/lib.rs`
    - file length: `2884`
    - test-only helper starts at line `2360`
    - test module starts at line `2413`
    - inferred trailing test block size: about `472` lines after `mod tests`
    - still carries substantial crate-root behavior coverage inline, but the earlier lint-specific layout debt is already resolved
  - `src-rust/crates/query/src/compact.rs`
    - file length: `1829`
    - test module starts at line `1609`
    - inferred trailing test block size: about `221` lines
    - mostly local unit coverage; lower-value relocation target than the three files above
- Secondary query candidates exist but are less urgent:
  - `src-rust/crates/query/src/session_memory.rs`
    - file length: `674`
    - test module starts at line `424`
    - inferred trailing test block size: about `251` lines
    - mostly parser/persistence-local tests
- API-side live reality is comparatively light:
  - `src-rust/crates/api/src/providers/openai.rs`
    - file length: `1035`
    - test module starts at line `977`
    - inferred trailing test block size: about `59` lines
  - `src-rust/crates/api/src/providers/google.rs`
    - file length: `1141`
    - test module starts at line `1037`
    - inferred trailing test block size: about `105` lines
  - `src-rust/crates/api/src/lib.rs`
    - file length: `1293`
    - test module starts at line `1230`
    - inferred trailing test block size: about `64` lines
  - `src-rust/crates/api/src/providers/openai_compat.rs`
    - file length: `801`
    - test module starts at line `771`
    - inferred trailing test block size: about `31` lines
  - `src-rust/crates/api/src/transform.rs`
    - no tests
- Conclusion from live file shape:
  - inline tests still predominate across both crates
  - a meaningful cleanup seam still exists
  - that seam is now overwhelmingly query-side maintainability debt, not balanced query/API debt and not a remaining lint-failure cleanup

## 7. recommended cleanup boundary

- Do not keep the next ticket as one combined `query/API test-organization cleanup` pass.
- The smallest safe next ticket is `query`-only.
- Recommended immediate scope:
  - `src-rust/crates/query/src/provider_resolution.rs`
  - `src-rust/crates/query/src/agent_tool.rs`
- Recommended defer / optional follow-on only if separately approved:
  - `src-rust/crates/query/src/lib.rs`
- Recommended exclude from the immediate next ticket:
  - all of `src-rust/crates/api/`
  - `src-rust/crates/query/src/compact.rs`
  - other smaller query test modules unless they become necessary for consistency inside the same file-local reorg pattern
- Recommended approach: `mixed`, but biased toward source-local reorganization rather than crate-level integration tests
  - move only the bulky query test modules out of the production files into sibling `#[cfg(test)]` unit-test modules/files under `src/` so private/internal access remains intact
  - keep genuinely local unit tests inline where the module is already small and readable
  - do not force a new crate-level `tests/` tree just for consistency
- This should not be treated as docs-only or housekeeping-only:
  - a real cleanup ticket will need meaningful test-helper/import/module movement inside query-owned files
  - API-side files do not currently justify a comparable move

## 8. risk / non-regression findings

- Current runtime/test baseline is stable:
  - `cd src-rust && cargo build -p claurst-query` -> `PASS`
  - `cd src-rust && cargo test -p claurst-query` -> `PASS`
  - `cd src-rust && cargo build -p claurst-api` -> `PASS`
  - `cd src-rust && cargo test -p claurst-api` -> `PASS`
- `claurst-query` still has high-value maintainability upside from test reorganization, but a full move to integration-style `crates/query/tests/` would be higher risk than value:
  - the strongest candidates rely heavily on `super::*`, crate-private helpers, internal fake providers, and local harness scaffolding
  - moving those tests to external integration-style files would likely require new public exposure, helper duplication, or broader harness plumbing
- `src-rust/crates/query/src/lib.rs` is a higher-risk cleanup target than `provider_resolution.rs` or `agent_tool.rs`:
  - it is the crate root
  - it already had accepted local lint cleanup in `POST_M11-06C`
  - reopening it too early risks mixing maintainability work with runtime-adjacent seams
- API-side relocation now has weak payoff:
  - `openai.rs`, `google.rs`, `lib.rs`, and `openai_compat.rs` each hold comparatively small inline test modules
  - moving them would add churn to accepted `POST_M11-07B` / `POST_M11-07C` surfaces without a strong maintainability gain
- Net non-regression conclusion:
  - a query-only, source-local test reorg is low-to-medium risk if limited to the two strongest candidates
  - a combined query/API pass or forced integration-test migration is unnecessary risk

## 9. recommended validation gate

- For the immediate next ticket, if scoped query-only as recommended, blocking gates should be:
  - `cd src-rust && cargo build -p claurst-query`
  - `cd src-rust && cargo test -p claurst-query`
- If a later separate API test-organization ticket is ever explicitly approved, its blocking gates should be:
  - `cd src-rust && cargo build -p claurst-api`
  - `cd src-rust && cargo test -p claurst-api`
- This preflight reran the likely future gates as a live baseline check:
  - query build -> `PASS`
  - query test -> `PASS` (`138 passed; 0 failed`)
  - api build -> `PASS`
  - api test -> `PASS` (`32 passed; 0 failed`)

## 10. drift / blockers, if any

- Branch / HEAD drift: `none`
- Authority drift: `none`
- Structural repo-reality drift versus the original combined ticket idea: `yes, but not blocking`
  - the post-assessment recommendation for combined `query/API` cleanup is now too broad for live reality
  - the meaningful seam that remains is mostly query-side
- Worktree / review-basis note:
  - the repo is still noisy from unrelated modified/untracked files
  - this is not a blocker for preflight
  - it is a patch-hygiene note for any later implementation ticket, which should use explicit path-scoped review basis
- Blockers:
  - no blocker prevents a narrow next ticket
  - combined query/API scope should not be used as-is

## 11. exact recommendation for next step

- Open the next ticket as `POST-M11-08A` and keep it `query`-only.
- Scope `POST-M11-08A` to test-organization cleanup in:
  - `src-rust/crates/query/src/provider_resolution.rs`
  - `src-rust/crates/query/src/agent_tool.rs`
- Use a source-local mixed approach:
  - extract bulky inline tests into sibling test modules/files under `src/`
  - preserve private/internal access
  - leave small local tests inline
- Do not include `src-rust/crates/api/` in that ticket.
- Do not treat `src-rust/crates/query/src/lib.rs` as in-scope unless a second, explicitly approved follow-on ticket is opened after the first query-only pass lands cleanly.
- Do not frame the next ticket as feature work, lint cleanup, or repo-wide test-strategy redesign.
