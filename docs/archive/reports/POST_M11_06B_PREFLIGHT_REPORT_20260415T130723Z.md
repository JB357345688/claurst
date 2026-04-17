# POST-M11-06B Preflight Report

## 1. ticket id

`POST-M11-06B`

## 2. verdict

`PASS-WITH-NOTES`

## 3. timestamp UTC

`2026-04-15T13:07:23Z`

## 4. branch / HEAD observed

- Branch: `feature/provider-resolution-seam`
- HEAD: `4a9a97f225390a280fb7f3ad934584812ec817b0`
- Expected accepted HEAD: `4a9a97f225390a280fb7f3ad934584812ec817b0`
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

- `AGENTS.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md`
- `docs/archive/reports/POST_M11_REPO_ASSESSMENT_20260415T071321Z.md`
- `docs/archive/reports/POST_M11_03_CLOSEOUT_REPORT_20260415T090527Z.md`
- `docs/archive/reports/POST_M11_04_CLOSEOUT_REPORT_20260415T102652Z.md`
- `docs/archive/reports/POST_M11_05_CLOSEOUT_REPORT_20260415T122710Z.md`
- `docs/archive/reports/POST_M11_06_PREFLIGHT_REPORT_20260415T124322Z.md`
- `docs/archive/reports/POST_M11_06A_CLOSEOUT_REPORT_20260415T130143Z.md`

Authority verification:
- `docs/Current/MPWO_WORK_ORDER_PACK.md` still declares itself the single active authority artifact in `docs/Current/`.
- `docs/Current/` still contains only one live controlling artifact:
  - active: `docs/Current/MPWO_WORK_ORDER_PACK.md`
  - historical/non-controlling: `TASK-M11-10_SPLIT_PLAN_AND_TEMPORARY_AUTHORITY_AMENDMENT.md`, `D1_REVIEW_REPORT_20260413T233604Z.md`, `M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md`, `IMPLEMENTATION_PLAN_MPWO.md`
- No conflicting live current-authority artifact was found.

Verified commands:
- `git branch --show-current`
- `git rev-parse HEAD`
- `git status --short --branch`
- `rg --files docs/Current`
- `rg -n "sole active current-authority artifact|single active authority artifact|Active:|Historical / non-controlling:" docs/Current/...`
- `cd src-rust && cargo build -p claurst-query`
- `cd src-rust && cargo test -p claurst-query`
- `cd src-rust && cargo clippy -p claurst-query --all-targets --no-deps -- -D warnings`
- targeted `sed` / `rg` inspection of:
  - `src-rust/crates/query/src/provider_resolution.rs`
  - `src-rust/crates/query/src/agent_tool.rs`
  - `src-rust/crates/query/src/lib.rs`

## 6. current claurst-query clippy findings

Live validation probes:
- `cd src-rust && cargo build -p claurst-query` -> `PASS`
- `cd src-rust && cargo test -p claurst-query` -> `PASS`
- `cd src-rust && cargo clippy -p claurst-query --all-targets --no-deps -- -D warnings` -> `FAIL`

`cargo test -p claurst-query` result:
- `138 passed; 0 failed`

Exact live clippy failure set:
- `src-rust/crates/query/src/agent_tool.rs`
  - `unwrap_or_default`
  - `unnecessary_map_or`
  - `field_reassign_with_default` in tests
  - `type_complexity` in test helper return type
- `src-rust/crates/query/src/provider_resolution.rs`
  - `needless_borrow`
- `src-rust/crates/query/src/lib.rs`
  - `too_many_arguments` x2
  - `unnecessary_map_or`
  - `items_after_test_module`
  - `field_reassign_with_default` in tests

Failure-set confinement check:
- The live remaining failure set is still confined exactly to:
  - `src-rust/crates/query/src/agent_tool.rs`
  - `src-rust/crates/query/src/provider_resolution.rs`
  - `src-rust/crates/query/src/lib.rs`
- No other `claurst-query` file remains in the failure set after `POST-M11-06A`.
- No non-query crate file appears in this command's failure set.

## 7. recommended tranche boundary

Recommended `POST-M11-06B` boundary:
- include:
  - `src-rust/crates/query/src/provider_resolution.rs`
  - `src-rust/crates/query/src/agent_tool.rs`
- exclude:
  - `src-rust/crates/query/src/lib.rs`
  - any non-query crate

Boundary verdict:
- `provider_resolution.rs` and `agent_tool.rs` can stay together in one tranche.
- `lib.rs` can remain excluded in this tranche.
- the next tranche can remain `claurst-query`-only.

Why this boundary is still viable:
- `provider_resolution.rs` has one remaining lint only, and it is a mechanical `needless_borrow`.
- `agent_tool.rs` still has four lints, but they do not force `lib.rs` API-shape changes.
- the only remaining clearly semantic/API-shape cleanup is in `lib.rs`, especially:
  - `too_many_arguments` on `run_query_loop`
  - `too_many_arguments` on `run_query_loop_inner`
  - `items_after_test_module`
- `run_query_loop` has external callsites in:
  - `src-rust/crates/cli/src/main.rs`
  - `src-rust/crates/query/src/cron_scheduler.rs`
  - `src-rust/crates/query/src/agent_tool.rs`
- that makes `lib.rs` the widening trigger, not `agent_tool.rs` or `provider_resolution.rs`.

Low-risk mechanical vs higher-sensitivity findings:
- `src-rust/crates/query/src/provider_resolution.rs`
  - `needless_borrow` at the fallback-model family sort is low-risk mechanical cleanup.
- `src-rust/crates/query/src/agent_tool.rs`
  - `unwrap_or_default` on inherited child cancel token is mechanically equivalent cleanup.
  - `unnecessary_map_or` on plugin agent-definition file filtering is low-risk mechanical cleanup.
  - `field_reassign_with_default` is test-only cleanup.
  - `type_complexity` is test-helper shape cleanup.
- `src-rust/crates/query/src/lib.rs`
  - `too_many_arguments` is higher-sensitivity API-shape work.
  - `items_after_test_module` implies file reordering.
  - this file should remain deferred.

## 8. risk / non-regression findings

Accepted M11 runtime behaviors reviewed for risk:
- same-domain fallback behavior
- hosted Ollama compatibility
- child/team session-budget propagation
- child `max_tokens`
- child `allow_fallback`
- child `budget_usd`
- `QueryEvent` observability

Current risk assessment:
- `provider_resolution.rs` is M11-sensitive because it owns same-domain fallback selection and hosted Ollama API-base normalization.
- `agent_tool.rs` is M11-sensitive because it owns child session-budget carriage, child cancel propagation, child `max_tokens`, child `allow_fallback`, child `budget_usd`, and worker/team execution flow.
- despite that sensitivity, the live remaining lints in those two files are still narrow enough to keep the tranche safe if edits stay local and mechanical.

Concrete evidence reviewed:
- same-domain fallback remains directly covered by:
  - `provider_resolution::tests::fallback_same_domain_returns_healthy_cloud_candidate`
  - `provider_resolution::tests::fallback_cross_domain_is_prohibited`
  - `agent_tool::tests::agent_tool_allow_fallback_uses_same_domain_provider`
- hosted Ollama compatibility remains directly covered by:
  - `provider_resolution::tests::normalize_ollama_api_base_rewrites_hosted_api_root`
  - `provider_resolution::tests::normalize_ollama_api_base_rewrites_hosted_api_v1_root`
  - `provider_resolution::tests::normalize_ollama_api_base_appends_v1_for_plain_roots`
- child/team budget and token carriage remain directly covered by:
  - `agent_tool::tests::child_session_budget_reuses_inherited_budget_when_child_limit_absent`
  - `agent_tool::tests::child_session_budget_wraps_parent_when_child_limit_present`
  - `agent_tool::tests::worker_budget_exceeded_event_reports_child_limit`
  - `agent_tool::tests::agent_tool_respects_max_tokens_override`
  - `agent_tool::tests::teamcreate_mixed_providers_per_agent_dispatch`
- `QueryEvent` observability remains live in `src-rust/crates/query/src/lib.rs` and covered by:
  - `tests::teamcreate_observability_is_sanitized_and_emitted`
  - `tests::session_budget_exceeded_event_emits_only_on_new_cancellation`

Non-regression conclusion:
- No live evidence forces reopening accepted M11 runtime behavior for `POST-M11-06B`.
- The risk is not that `provider_resolution.rs` and `agent_tool.rs` inherently require widening.
- The real regression risk is accidental over-editing inside M11-sensitive files; keeping the tranche strictly mechanical avoids that.

## 9. recommended validation gate

For `POST-M11-06B`, use these as blocking gates:
- `cd src-rust && cargo build -p claurst-query`
- `cd src-rust && cargo test -p claurst-query`

Use this only as a progress probe for `POST-M11-06B`:
- `cd src-rust && cargo clippy -p claurst-query --all-targets --no-deps -- -D warnings`

Blocking/non-blocking conclusion:
- full query clippy should still be non-blocking for this tranche
- it cannot credibly be made blocking while `src-rust/crates/query/src/lib.rs` stays excluded
- it could only become a blocking gate in a later tranche that explicitly includes the remaining `lib.rs` findings

## 10. drift / blockers, if any

Drift found:
- no branch drift
- no HEAD drift
- no active-authority drift
- no structural drift in the expected post-`06A` query failure set

Notes:
- worktree noise remains significant, so any later execution pass must keep staging explicit and review basis explicit
- a transient cargo file-lock wait appeared during build/test but did not affect results

Blockers:
- no blocker prevents a narrow `POST-M11-06B` execution ticket limited to `provider_resolution.rs` and `agent_tool.rs`
- there is a blocker to making full crate `clippy -D warnings` a blocking gate in that same tranche:
  - known out-of-scope `lib.rs` failures would still remain

## 11. exact recommendation for next step

Execute `POST-M11-06B` as a higher-sensitivity but still narrow `claurst-query` lint cleanup tranche with this exact scope:
- `src-rust/crates/query/src/provider_resolution.rs`
- `src-rust/crates/query/src/agent_tool.rs`

Keep explicitly out of scope:
- `src-rust/crates/query/src/lib.rs`
- any non-query crate
- repo-wide lint cleanup
- accepted M11 runtime redesign

Execution guidance for the next ticket:
- keep edits mechanical and local to the reported lint sites
- treat `provider_resolution.rs` and `agent_tool.rs` as one tranche
- preserve existing fallback, hosted Ollama, budget, `max_tokens`, `allow_fallback`, `budget_usd`, and observability behavior
- use `cargo build -p claurst-query` and `cargo test -p claurst-query` as blocking gates
- use full crate `cargo clippy -p claurst-query --all-targets --no-deps -- -D warnings` only as a progress probe

Bottom line:
- `POST-M11-06B` may proceed
- it can stay limited to `provider_resolution.rs` plus `agent_tool.rs`
- `lib.rs` does not need to be pulled in yet
- full query clippy should remain non-blocking until a later `lib.rs` tranche

## 12. preflight reporting additions

Verified files/symbols:
- files:
  - `src-rust/crates/query/src/provider_resolution.rs`
  - `src-rust/crates/query/src/agent_tool.rs`
  - `src-rust/crates/query/src/lib.rs`
- symbols / seams:
  - `resolve_provider_with_fallback`
  - `normalize_ollama_api_base`
  - `child_session_budget`
  - inherited child cancel-token path
  - `run_query_loop`
  - `run_query_loop_inner`
  - `QueryEvent`

Preflight verdict summary:
- ticket id: `POST-M11-06B`
- verdict: `PASS-WITH-NOTES`
- verified commands, files, and symbols: `yes`
- drift found: worktree noise only; no structural drift affecting tranche scope
- blockers: no execution blocker for the narrow tranche; `lib.rs` remains the blocker for blocking full crate clippy
