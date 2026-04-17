# POST-M11-05 Preflight Report

## 1. ticket id

`POST-M11-05`

## 2. verdict

`PASS-WITH-NOTES`

## 3. timestamp UTC

`2026-04-15T11:24:25Z`

## 4. branch / HEAD observed

- Branch: `feature/provider-resolution-seam`
- HEAD: `d802b379e2133827d928ab9ba4b7f9de35a5a6f0`
- Expected accepted HEAD: `d802b379e2133827d928ab9ba4b7f9de35a5a6f0`
- HEAD match: `yes`
- Working-tree state: dirty / noisy
- Observed out-of-scope noise:
  - modified `.gitignore`
  - untracked `.codex`
  - untracked docs/report artifacts under `docs/Orchestrator_planning/`, `docs/archive/provider_orchestrator/`, and `docs/archive/reports/`
  - untracked `src-rust/.codex`
  - untracked `src-rust/target/`

## 5. authority reviewed

- Reviewed authority artifacts requested by prompt:
  - `AGENTS.md`
  - `docs/Current/MPWO_WORK_ORDER_PACK.md`
  - `docs/archive/reports/POST_M11_REPO_ASSESSMENT_20260415T071321Z.md`
  - `docs/archive/reports/POST_M11_03_CLOSEOUT_REPORT_20260415T090527Z.md`
  - `docs/archive/reports/POST_M11_04_PREFLIGHT_REPORT_20260415T093128Z.md`
  - `docs/archive/reports/POST_M11_04_CLOSEOUT_REPORT_20260415T102652Z.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md` still states it is the sole active current-authority artifact in `docs/Current/`.
- No conflicting live current-authority artifact was found in the reviewed post-M11 chain.
- Verified files and symbols:
  - `src-rust/crates/core/src/effort.rs`
  - `src-rust/crates/cli/src/main.rs`
  - `src-rust/crates/commands/src/lib.rs`
  - `src-rust/crates/tui/src/model_picker.rs`
  - symbol surface: `EffortLevel`, `EffortLevel::from_str`, `current_effort`, `/effort`
- Verified commands run:
  - `git branch --show-current`
  - `git rev-parse HEAD`
  - `git status --short --branch`
  - `date -u +%Y%m%dT%H%M%SZ`
  - `rg -n "EffortLevel::from_str|parse::<EffortLevel>|EffortLevel" src-rust`
  - `rg -n "parse::<EffortLevel>|EffortLevel::from_str|/effort|cmd_name == \"effort\"|effort_level|effort:" src-rust/crates/cli src-rust/crates/commands src-rust/crates/core src-rust/crates/query`
  - `cd src-rust && cargo clippy -p claurst-core --all-targets -- -D warnings`
  - `cd src-rust && cargo build -p claurst-core`
  - `cd src-rust && cargo test -p claurst-core`
  - `cd src-rust && cargo build -p claurst`

## 6. current effort-related clippy findings

Live command result:

- `cd src-rust && cargo clippy -p claurst-core --all-targets -- -D warnings` -> `FAIL`

Current live failure reality is still confined to one file and one lint family only:

- `src-rust/crates/core/src/effort.rs:35`
  - `clippy::should_implement_trait`
  - finding text: inherent `EffortLevel::from_str` can be confused with `std::str::FromStr::from_str`

Explicit boundary confirmation:

- `effort.rs` is still the only remaining `claurst-core` clippy blocker.
- No other `claurst-core` file appears in the current live `-D warnings` failure set.
- The failure is still specifically the `should_implement_trait` finding on `EffortLevel::from_str`.

Live supporting baseline probes:

- `cd src-rust && cargo build -p claurst-core` -> `PASS`
- `cd src-rust && cargo test -p claurst-core` -> `PASS`
- `cd src-rust && cargo build -p claurst` -> `PASS`

## 7. live callsite findings

Current live parser API shape in `src-rust/crates/core/src/effort.rs`:

- `EffortLevel` variants: `Low`, `Medium`, `High`, `Max`
- inherent parser:
  - `src-rust/crates/core/src/effort.rs:35`
  - `pub fn from_str(s: &str) -> Option<Self>`
  - accepted literals: `low`, `medium`, `high`, `max`
- related round-trip/display surface:
  - `as_str()` returns `low|medium|high|max`
  - `Display` delegates to `as_str()`

Current repo-local `EffortLevel::from_str` callsites found by live search:

- production callsites:
  - `src-rust/crates/cli/src/main.rs:722`
    - parses `--effort`
  - `src-rust/crates/cli/src/main.rs:1958`
    - parses explicit `/effort <arg>` after command handling
- test callsites:
  - `src-rust/crates/core/src/effort.rs:138`
  - `src-rust/crates/core/src/effort.rs:150`
  - `src-rust/crates/core/src/effort.rs:158`

No additional live parser callsites were found for:

- `parse::<EffortLevel>`
- other core/cli/query/tests code paths outside `effort.rs` tests and the two CLI production callsites above

Relevant surrounding effort-flow evidence:

- `src-rust/crates/cli/src/main.rs:1360-1366`
  - maps core `Medium` to TUI `Normal`
- `src-rust/crates/cli/src/main.rs:1717-1729`
  - no-args `/effort` cycling maps TUI `Normal` back to core `Medium`
- `src-rust/crates/cli/src/main.rs:2087-2089`
  - `current_effort` is what actually flows into `qcfg.effort_level`

Important live semantic note:

- `src-rust/crates/commands/src/lib.rs:4666-4694`
  - `/effort` help and command parsing use `low|normal|high`
- `src-rust/crates/tui/src/model_picker.rs:24-28`
  - TUI effort enum uses `Low|Normal|High|Max`
- `src-rust/crates/core/src/effort.rs:35-40`
  - core parser uses `low|medium|high|max`

This means repo reality currently contains a naming split:

- core parser token: `medium`
- slash-command / TUI label token: `normal`

## 8. recommended tranche boundary

The next ticket can remain one narrow cross-crate tranche.

Recommended exact owned files:

- `src-rust/crates/core/src/effort.rs`
- `src-rust/crates/cli/src/main.rs`

Why this remains narrow:

- the only production parser callsites are the two CLI sites in `main.rs`
- no live evidence forces changes in `query`, `commands`, `tui`, or tests outside `effort.rs`
- the remaining blocker is API-shape cleanup, not broader runtime or lint churn

Recommended excluded scope for this next ticket:

- `src-rust/crates/commands/src/lib.rs`
- `src-rust/crates/tui/src/model_picker.rs`
- any non-effort-related clippy cleanup
- any repo-wide or broader `claurst-core` cleanup

Split recommendation:

- do **not** split into separate API-shape and downstream-adaptation tickets
- the safe downstream adaptation is only the exact two CLI callsites, so keeping them with the `effort.rs` change is the smallest reviewable unit

## 9. risk / non-regression findings

Safest remediation shape:

- implement `std::str::FromStr` for `EffortLevel`
- remove the inherent `EffortLevel::from_str`
- update the exact CLI callsites to trait-based parsing, for example `level_str.parse::<EffortLevel>().ok()`
- update only the `effort.rs` unit tests to assert trait-based parsing behavior

Compatibility-shim assessment:

- no repo-local compatibility shim is required
- keeping the inherent `from_str` would preserve the ambiguous API surface and would not resolve the clippy blocker cleanly
- repo-local production usage is limited to two CLI callsites, so direct callsite adaptation is narrow and low-churn

Behavioral-risk assessment:

- `--effort` CLI parsing is affected only at the parser callsite in `src-rust/crates/cli/src/main.rs:722`
- explicit `/effort <arg>` parsing is affected only at `src-rust/crates/cli/src/main.rs:1958`
- both can preserve current accepted behavior if the new `FromStr` implementation accepts the same literals the current inherent parser accepts: `low|medium|high|max`

Slash-command / config / test impact:

- slash-command parsing:
  - there is live semantic mismatch between command-layer `normal` and core-layer `medium`
  - adding `"normal"` as a new `EffortLevel` parse alias in this ticket would change current slash-command behavior
  - safest no-drift choice is to **not** broaden accepted literals in this ticket
- config parsing:
  - no live evidence was found of config-string parsing through `EffortLevel::from_str`
  - `QueryConfig::from_config_with_registry` does not introduce a separate effort-string parse path
- tests:
  - `claurst-core` unit tests in `effort.rs` will need narrow updates
  - no additional live repo-local parser tests were found outside `effort.rs`

Risk summary:

- no evidence that the next ticket needs to reopen accepted M11 runtime behavior
- the real non-regression risk is accidental normalization of `normal -> Medium`
- treat that aliasing decision as out of scope unless the next ticket is explicitly authorized to adjust slash-command semantics

## 10. recommended validation gate

Recommended blocking gates for the next ticket:

- `cd src-rust && cargo build -p claurst-core`
- `cd src-rust && cargo test -p claurst-core`
- `cd src-rust && cargo build -p claurst`
- `cd src-rust && cargo clippy -p claurst-core --all-targets -- -D warnings`

Recommended not-required gates for this narrow ticket:

- `cargo test -p claurst`
  - no live evidence of parser-specific CLI tests that justify making this mandatory
- workspace-wide clippy or unrelated crate gates
  - not needed for this isolated effort/API-shape tranche

Gate rationale:

- `claurst-core` build/test covers the parser implementation and its local tests
- `claurst` build covers the only live downstream production callsites
- `claurst-core` clippy `-D warnings` is the actual closure gate this ticket is meant to clear

## 11. drift / blockers, if any

Drift:

- no structural drift was found against the accepted `POST-M11-04` evidence
- `effort.rs` is still the only remaining `claurst-core` clippy blocker
- branch and HEAD still match the accepted baseline exactly
- `docs/Current/MPWO_WORK_ORDER_PACK.md` is still the sole active current-authority artifact

Notes / blockers:

- working tree remains noisy, so later execution/review must keep review basis explicit
- there is live semantic mismatch between:
  - core token `medium`
  - slash-command / TUI token `normal`
- this mismatch is a note, not a blocker, as long as the next ticket preserves current parser literals and avoids alias expansion

## 12. exact recommendation for next step

Open one narrow execution ticket for the remaining effort/API-shape cleanup with this scope only:

- `src-rust/crates/core/src/effort.rs`
- `src-rust/crates/cli/src/main.rs`

Implementation target for that next ticket:

- replace the inherent `EffortLevel::from_str` with a `std::str::FromStr` impl
- update only the two CLI production callsites and the `effort.rs` tests
- preserve current accepted parse literals: `low|medium|high|max`
- do not add `normal` as a parse alias in this ticket
- do not touch `commands` or `tui` unless authority is explicitly widened to change slash-command semantics

Closure expectation for that next ticket:

- it should be able to clear the remaining `claurst-core` clippy blocker without widening into a broader repo cleanup
- if a future ticket wants to reconcile `normal` versus `medium`, that should be opened separately as an explicit behavior-facing cleanup, not folded silently into this clippy/API-shape tranche
