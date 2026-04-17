# POST-M11-06 Preflight Report

## 1. ticket id

`POST-M11-06`

## 2. verdict

`PASS-WITH-NOTES`

## 3. timestamp UTC

`2026-04-15T12:43:22Z`

## 4. branch / HEAD observed

- Branch: `feature/provider-resolution-seam`
- HEAD: `7c979f558243ff8014dbe68ac398c37e863a820c`
- Expected accepted HEAD: `7c979f558243ff8014dbe68ac398c37e863a820c`
- HEAD match: `yes`
- Working-tree state: dirty / noisy
- Observed out-of-scope noise:
  - modified `.gitignore`
  - untracked `.codex`
  - untracked `docs/Orchestrator_planning/`
  - untracked `docs/archive/provider_orchestrator/`
  - many untracked report artifacts under `docs/archive/reports/`
  - untracked `src-rust/.codex`
  - untracked `src-rust/target/`

## 5. authority reviewed

- Reviewed prompt-required authority artifacts:
  - `AGENTS.md`
  - `docs/Current/MPWO_WORK_ORDER_PACK.md`
  - `docs/archive/reports/POST_M11_REPO_ASSESSMENT_20260415T071321Z.md`
  - `docs/archive/reports/POST_M11_03_CLOSEOUT_REPORT_20260415T090527Z.md`
  - `docs/archive/reports/POST_M11_04_CLOSEOUT_REPORT_20260415T102652Z.md`
  - `docs/archive/reports/POST_M11_05_CLOSEOUT_REPORT_20260415T122710Z.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md` still states that it is the single active authority artifact in `docs/Current/` and the sole active current-authority artifact.
- `docs/Current/` file listing still matches that claim:
  - active: `docs/Current/MPWO_WORK_ORDER_PACK.md`
  - historical/non-controlling files remain present but demoted
- No conflicting live current-authority artifact was found in the reviewed post-M11 chain.

Verified commands:
- `git branch --show-current`
- `git rev-parse HEAD`
- `git status --short --branch`
- `date -u +%Y%m%dT%H%M%SZ`
- `rg --files docs/Current`
- `rg -n "sole active current-authority artifact|single active authority artifact|Active:" docs/Current`
- `cd src-rust && cargo build -p claurst-query`
- `cd src-rust && cargo test -p claurst-query`
- `cd src-rust && cargo clippy -p claurst-query --all-targets --no-deps -- -D warnings`
- targeted `rg` / `sed` inspection of:
  - `src-rust/crates/query/src/compact.rs`
  - `src-rust/crates/query/src/agent_tool.rs`
  - `src-rust/crates/query/src/coordinator.rs`
  - `src-rust/crates/query/src/provider_resolution.rs`
  - `src-rust/crates/query/src/skill_prefetch.rs`
  - `src-rust/crates/query/src/lib.rs`

## 6. current claurst-query clippy findings

Live baseline probes:
- `cd src-rust && cargo build -p claurst-query` -> `PASS`
- `cd src-rust && cargo test -p claurst-query` -> `PASS`
- `cd src-rust && cargo clippy -p claurst-query --all-targets --no-deps -- -D warnings` -> `FAIL`

`cargo test -p claurst-query` note:
- test pass included one plain compiler warning already reflected in clippy:
  - `src-rust/crates/query/src/compact.rs:1611`
  - `unused import: Role`

Current live clippy failure set:
- `src-rust/crates/query/src/compact.rs`
  - `unused_imports`
  - `single_match`
  - `redundant_closure` x2
  - `if_same_then_else`
  - `field_reassign_with_default` in tests
- `src-rust/crates/query/src/agent_tool.rs`
  - `unwrap_or_default`
  - `unnecessary_map_or`
  - `field_reassign_with_default` in tests
  - `type_complexity` in test helper return type
- `src-rust/crates/query/src/coordinator.rs`
  - `needless_lifetimes`
  - `borrowed_box`
- `src-rust/crates/query/src/provider_resolution.rs`
  - `needless_borrow`
- `src-rust/crates/query/src/skill_prefetch.rs`
  - `unnecessary_map_or` x2
  - `manual_strip`
- `src-rust/crates/query/src/lib.rs`
  - `too_many_arguments` x2
  - `unnecessary_map_or`
  - `items_after_test_module`
  - `field_reassign_with_default` in tests

Observed total:
- 21 clippy errors across 6 `claurst-query` files
- all failures remain inside `claurst-query`; no non-query crate file appears in this command’s failure set

Comparison with `POST_M11_REPO_ASSESSMENT_20260415T071321Z`:
- earlier representative file list is still accurate
- earlier representative lint-family list is still directionally accurate
- live output is more specific than the assessment and adds two omitted test-only findings:
  - `src-rust/crates/query/src/compact.rs` also has `field_reassign_with_default` in tests
  - `src-rust/crates/query/src/lib.rs` also has `field_reassign_with_default` in tests
- no structural drift was found that would invalidate the earlier assessment’s query-lint picture

## 7. recommended tranche boundary

Recommended first tranche: a narrow low-risk mechanical `claurst-query`-only cluster.

Exact tranche-1 files:
- `src-rust/crates/query/src/compact.rs`
- `src-rust/crates/query/src/coordinator.rs`
- `src-rust/crates/query/src/skill_prefetch.rs`

Why this is the narrowest realistic first tranche:
- it removes 11 of the 21 current query clippy errors without entering the highest-risk M11-owned runtime surfaces
- all three files can be cleaned by local mechanical edits rather than behavior redesign
- it stays fully inside `claurst-query`
- it avoids turning `POST-M11-06` into a broad cleanup blob

Explicitly not in tranche 1:
- `src-rust/crates/query/src/provider_resolution.rs`
- `src-rust/crates/query/src/agent_tool.rs`
- `src-rust/crates/query/src/lib.rs`

Why those stay out of tranche 1:
- `provider_resolution.rs` has only one `needless_borrow`, but it sits directly on accepted M11 provider/fallback routing logic
- `agent_tool.rs` lint classes are mostly mechanical, but the file owns accepted child-budget, child-cancel, provider dispatch, and TeamCreate execution behavior
- `lib.rs` contains API-shape and layout findings rather than pure mechanical cleanup

Scope conclusion:
- the next tranche can remain limited to `claurst-query` only
- a full crate-clean ticket should not be forced into this first tranche

## 8. risk / non-regression findings

Low-risk mechanical group:
- `compact.rs`
  - findings are local control-flow / iterator cleanup plus one test-only default-init cleanup
  - no live evidence ties these findings to accepted M11 runtime behavior
- `coordinator.rs`
  - findings are signature-shape cleanup only
  - repo-local search found no in-repo callsite for `filter_tools_for_mode`, which reduces regression risk
  - note: it is still public API surface, so signature changes should stay minimal
- `skill_prefetch.rs`
  - findings are local filesystem / front-matter parsing cleanup only
  - no accepted M11 runtime seam depends on these exact code paths

Higher-sensitivity files:
- `provider_resolution.rs`
  - current finding is mechanically trivial
  - the file also contains accepted M11 same-domain fallback and provider-resolution behavior
  - touching it is still low code-change risk but higher non-regression sensitivity than the three-file tranche above
- `agent_tool.rs`
  - current lints are mechanical or test-helper-shaped
  - the file is M11-sensitive because it carries inherited session budget, child cancel token, worker provider dispatch, and TeamCreate execution paths
- `lib.rs`
  - `too_many_arguments` on `run_query_loop` and `run_query_loop_inner` is API-shape work, not just style
  - `run_query_loop` has live callsites outside this file, including `src-rust/crates/cli/src/main.rs`
  - `items_after_test_module` implies file reordering around test/module boundaries
  - this is not a good first tranche if the goal is to minimize risk

M11 non-regression assessment:
- accepted M11 runtime behavior is most at risk if tranche 1 touches `provider_resolution.rs`, `agent_tool.rs`, or `lib.rs`
- accepted M11 runtime behavior is not meaningfully threatened by limiting tranche 1 to `compact.rs`, `coordinator.rs`, and `skill_prefetch.rs`

## 9. recommended validation gate

For the recommended tranche-1 mechanical query-only ticket, use these as blocking gates:
- `cd src-rust && cargo build -p claurst-query`
- `cd src-rust && cargo test -p claurst-query`

For the same tranche, use this only as a progress probe:
- `cd src-rust && cargo clippy -p claurst-query --all-targets --no-deps -- -D warnings`

Why clippy should not be blocking in tranche 1:
- the recommended tranche intentionally leaves known failures in:
  - `src-rust/crates/query/src/provider_resolution.rs`
  - `src-rust/crates/query/src/agent_tool.rs`
  - `src-rust/crates/query/src/lib.rs`
- therefore full crate clippy will still fail even if tranche 1 is implemented correctly

Later-gate note:
- once `lib.rs` API-shape work is in scope, add a cross-crate build check such as `cd src-rust && cargo build -p claurst`
- that later step is not required for the recommended tranche-1 mechanical cleanup

## 10. drift / blockers, if any

Drift found:
- no branch drift
- no HEAD drift
- no authority drift
- no structural drift in the expected query lint file set
- minor evidence drift versus the earlier repo assessment:
  - live clippy confirms two extra test-only `field_reassign_with_default` findings omitted from the assessment summary

Patch-basis / hygiene note:
- the worktree is noisy with unrelated modified and untracked files
- this does not block preflight
- it does mean any later execution ticket must state review basis explicitly and stage by exact path

Current blocker assessment:
- no blocker prevents a narrow `claurst-query` tranche-1 execution ticket
- there is a blocker to making full crate clippy a tranche-1 blocking gate: known out-of-scope failures would remain

## 11. exact recommendation for next step

Execute `POST-M11-06` as a split cleanup ladder, not as one crate-wide clippy-clean ticket.

Recommended immediate next ticket:
- `POST-M11-06A`
- scope exactly:
  - `src-rust/crates/query/src/compact.rs`
  - `src-rust/crates/query/src/coordinator.rs`
  - `src-rust/crates/query/src/skill_prefetch.rs`
- keep out of scope:
  - `src-rust/crates/query/src/provider_resolution.rs`
  - `src-rust/crates/query/src/agent_tool.rs`
  - `src-rust/crates/query/src/lib.rs`
- validation:
  - blocking: `cargo build -p claurst-query`, `cargo test -p claurst-query`
  - progress probe only: `cargo clippy -p claurst-query --all-targets --no-deps -- -D warnings`

Recommended follow-on split after that:
- `POST-M11-06B`
  - higher-sensitivity but still query-local cleanup in `provider_resolution.rs` and `agent_tool.rs`
- later semantic/API-shape ticket
  - `src-rust/crates/query/src/lib.rs`
  - decide explicitly whether to refactor `run_query_loop` signature or accept a lint suppression strategy

Bottom-line recommendation:
- `POST-M11-06` should already be split into:
  - low-risk mechanical cleanup first
  - later semantic/API-shape cleanup second
- do not attempt full `claurst-query` `-D warnings` closure in the first tranche
