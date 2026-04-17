# POST-M11-06C Preflight Report

## 2. ticket id

`POST-M11-06C`

## 3. verdict

`PASS-WITH-NOTES`

## 4. timestamp UTC

`2026-04-15T13:30:59Z`

## 5. branch / HEAD observed

- Branch: `feature/provider-resolution-seam`
- HEAD: `20c3c275021737c3018d199d2739c40471b1753e`
- Expected accepted HEAD: `20c3c275021737c3018d199d2739c40471b1753e`
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

## 6. authority reviewed

- Reviewed prompt-required authority artifacts:
  - `AGENTS.md`
  - `docs/Current/MPWO_WORK_ORDER_PACK.md`
  - `docs/archive/reports/POST_M11_REPO_ASSESSMENT_20260415T071321Z.md`
  - `docs/archive/reports/POST_M11_03_CLOSEOUT_REPORT_20260415T090527Z.md`
  - `docs/archive/reports/POST_M11_04_CLOSEOUT_REPORT_20260415T102652Z.md`
  - `docs/archive/reports/POST_M11_05_CLOSEOUT_REPORT_20260415T122710Z.md`
  - `docs/archive/reports/POST_M11_06_PREFLIGHT_REPORT_20260415T124322Z.md`
  - `docs/archive/reports/POST_M11_06A_CLOSEOUT_REPORT_20260415T130143Z.md`
  - `docs/archive/reports/POST_M11_06B_PREFLIGHT_REPORT_20260415T130723Z.md`
  - `docs/archive/reports/POST_M11_06B_CLOSEOUT_REPORT_20260415T132352Z.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md` still states that it is the single active authority artifact in `docs/Current/` and the sole active current-authority artifact.
- Live `docs/Current/` listing still supports that claim:
  - active: `docs/Current/MPWO_WORK_ORDER_PACK.md`
  - historical / non-controlling: `TASK-M11-10_SPLIT_PLAN_AND_TEMPORARY_AUTHORITY_AMENDMENT.md`, `D1_REVIEW_REPORT_20260413T233604Z.md`, `M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md`, `IMPLEMENTATION_PLAN_MPWO.md`
- No conflicting live current-authority artifact was found.

Verified commands:
- `git branch --show-current`
- `git rev-parse HEAD`
- `git status --short --branch`
- `date -u +%Y%m%dT%H%M%SZ`
- `rg --files docs/Current`
- `rg -n "sole active current-authority artifact|single active authority artifact|Active:|Historical / non-controlling:" docs/Current`
- `cd src-rust && cargo build -p claurst-query`
- `cd src-rust && cargo test -p claurst-query`
- `cd src-rust && cargo build -p claurst`
- `cd src-rust && cargo clippy -p claurst-query --all-targets --no-deps -- -D warnings`
- targeted `rg`, `sed`, and `nl -ba` inspection of:
  - `src-rust/crates/query/src/lib.rs`
  - `src-rust/crates/query/src/agent_tool.rs`
  - `src-rust/crates/query/src/cron_scheduler.rs`
  - `src-rust/crates/cli/src/main.rs`

Verified symbols / seams:
- `run_query_loop`
- `run_query_loop_inner`
- `build_todo_nudge`
- `ChannelStreamHandler`
- `run_single_query`
- `QueryEvent`

## 7. current claurst-query lib.rs clippy findings

Live validation probes:
- `cd src-rust && cargo build -p claurst-query` -> `PASS`
- `cd src-rust && cargo test -p claurst-query` -> `PASS`
- `cd src-rust && cargo build -p claurst` -> `PASS`
- `cd src-rust && cargo clippy -p claurst-query --all-targets --no-deps -- -D warnings` -> `FAIL`

Test note:
- `cargo test -p claurst-query` completed with `138 passed; 0 failed`

Remaining failure-set confinement:
- The live remaining `claurst-query` clippy failure set is confined only to `src-rust/crates/query/src/lib.rs`.
- No other `claurst-query` file appears in the current failure set.

Exact remaining `lib.rs` findings:
- `too_many_arguments` on `run_query_loop` at `src-rust/crates/query/src/lib.rs:864`
- `too_many_arguments` on `run_query_loop_inner` at `src-rust/crates/query/src/lib.rs:893`
- `unnecessary_map_or` in `build_todo_nudge` at `src-rust/crates/query/src/lib.rs:2206`
- `items_after_test_module` because `mod tests` starts at `src-rust/crates/query/src/lib.rs:2362` while `ChannelStreamHandler` and `run_single_query` remain after it at `2833-2853`
- `field_reassign_with_default` in test helper `make_tool_context` at `src-rust/crates/query/src/lib.rs:2443-2444`

Local-fix assessment by finding:
- `unnecessary_map_or`: local one-line cleanup only
- `items_after_test_module`: local file-order cleanup only
- `field_reassign_with_default`: local test-only initializer cleanup only
- `too_many_arguments`: not a file-order or test-only cleanup; this is the public query-loop seam and is the only remaining issue that can widen the tranche

## 8. live callsite findings

`run_query_loop_inner`:
- no external repo-local callsites found
- only local invocation is `src-rust/crates/query/src/lib.rs:878`

`run_query_loop` repo-local callsites:
- `src-rust/crates/cli/src/main.rs:1139`
- `src-rust/crates/cli/src/main.rs:2099`
- `src-rust/crates/cli/src/main.rs:2305`
- `src-rust/crates/cli/src/main.rs:2458`
- `src-rust/crates/cli/src/main.rs:2570`
- `src-rust/crates/query/src/cron_scheduler.rs:85`
- `src-rust/crates/query/src/agent_tool.rs:544`
- `src-rust/crates/query/src/agent_tool.rs:593`
- `src-rust/crates/query/src/agent_tool.rs:813`
- `src-rust/crates/query/src/lib.rs:2477` (test helper path)

Callsite summary:
- total repo-local `run_query_loop` callsites found: `10`
- outside `lib.rs`: `9`
- minimum prompt-required files were confirmed:
  - `src-rust/crates/cli/src/main.rs`
  - `src-rust/crates/query/src/agent_tool.rs`
  - `src-rust/crates/query/src/cron_scheduler.rs`
- every current repo-local callsite passes `None` for `pending_messages`

Impact on boundary:
- a structural signature change to `run_query_loop` cannot be treated as `lib.rs`-only
- exact repo-local adaptation files would be:
  - `src-rust/crates/query/src/lib.rs`
  - `src-rust/crates/cli/src/main.rs`
  - `src-rust/crates/query/src/agent_tool.rs`
  - `src-rust/crates/query/src/cron_scheduler.rs`

## 9. recommended tranche boundary

Boundary conclusions:
- The next tranche can remain `claurst-query`-only: `yes`
- The next tranche can remain `lib.rs`-only: `yes`, but only if the `too_many_arguments` issue is handled as a local lint-policy decision instead of a structural API-shape refactor
- A no-allow structural fix for `too_many_arguments` would require a narrow cross-file tranche, not `lib.rs` alone

Safest remediation shape:
- preferred: local cleanup in `src-rust/crates/query/src/lib.rs` plus targeted `#[allow(clippy::too_many_arguments)]` on `run_query_loop` and `run_query_loop_inner`

Why this is the narrowest honest choice:
- it resolves the remaining crate-local blocker without changing the established query-loop API seam
- it leaves accepted M11 runtime behavior untouched
- it keeps the ticket reviewable as one-file scope
- a helper wrapper alone is not sufficient, because the public `run_query_loop` signature would still trip clippy
- a parameter-object refactor is technically viable, but it would force cross-file callsite edits through the root query, cron, and agent paths

If lint allows are rejected by policy:
- use a parameter-object / context-struct refactor rather than ad hoc wrapper churn
- treat that as a separate cross-file API-shape tranche with the exact files listed in section 8

## 10. risk / non-regression findings

Accepted M11-sensitive behaviors that run through `run_query_loop` / `run_query_loop_inner`:
- root session-budget registration via `with_registered_session_budget` at `src-rust/crates/query/src/lib.rs:875-889`
- inherited parent session-budget accounting and child/local budget layering through `config.session_budget`, `cost_tracker`, and caller-provided cancellation tokens
- worker/query observability via `QueryEvent` and `event_tx`
- same-session tool-loop behavior and turn accounting
- fallback routing through `effective_model`, registry-backed dispatch, and legacy fallback handling inside `run_query_loop_inner`

Risk assessment:
- local cleanup for `unnecessary_map_or`, `items_after_test_module`, and test `field_reassign_with_default` does not meaningfully threaten accepted M11 runtime behavior
- targeted `too_many_arguments` allows also do not alter runtime behavior
- structural API-shape work is behavior-preserving in principle, but it increases regression surface because it touches all root-query, worker-agent, and cron entrypoints

Non-regression conclusion:
- the safest path is the one-file local-cleanup plus targeted-allow route
- the real risk is not the remaining local lint sites; it is widening the central query-loop seam without a strong need

## 11. recommended validation gate

For the recommended one-file `POST-M11-06C` execution ticket, use these as blocking gates:
- `cd src-rust && cargo build -p claurst-query`
- `cd src-rust && cargo test -p claurst-query`
- `cd src-rust && cargo build -p claurst`
- `cd src-rust && cargo clippy -p claurst-query --all-targets --no-deps -- -D warnings`

Why these gates are appropriate:
- `claurst-query` build/test validate the file-local changes directly
- `claurst` build guards the public `run_query_loop` seam used by the CLI crate
- crate-local clippy should become a blocking gate here because the live remaining failure set is already confined entirely to `lib.rs`

If the ticket is widened into a structural API-shape refactor:
- keep the same four blocking gates
- no workspace-wide lint gate is required for this tranche

## 12. drift / blockers, if any

Drift found:
- no branch drift
- no HEAD drift
- no active-authority drift
- no structural drift versus the intended narrow post-`06B` tranche
- live repo reality still matches the accepted claim that `src-rust/crates/query/src/lib.rs` is the only remaining `claurst-query` clippy blocker

Patch-basis note:
- the worktree remains noisy with unrelated modified and untracked paths
- this does not block preflight
- it does require any later execution ticket to keep staging and review basis explicit by exact path

Blockers:
- no blocker prevents a narrow `POST-M11-06C` execution ticket
- one decision gate remains:
  - if policy requires removing `too_many_arguments` structurally rather than allowing it locally, the ticket must widen to the exact callsite files listed in section 8

## 13. exact recommendation for next step

Execute `POST-M11-06C` as one narrow `src-rust/crates/query/src/lib.rs` ticket only.

Recommended implementation shape:
- fix `unnecessary_map_or` in `build_todo_nudge`
- move `ChannelStreamHandler` and `run_single_query` above `mod tests` or otherwise reorder locally to satisfy `items_after_test_module`
- rewrite the test helper initializer to remove `field_reassign_with_default`
- add targeted `#[allow(clippy::too_many_arguments)]` to `run_query_loop` and `run_query_loop_inner`, with a brief comment that this is the established query-loop seam used by CLI, agent-tool, and cron entrypoints

Do not widen this ticket unless the maintainers explicitly reject the local lint-allow decision.

If the allow is rejected, stop and open a separate cross-file API-shape ticket with this exact scope:
- `src-rust/crates/query/src/lib.rs`
- `src-rust/crates/cli/src/main.rs`
- `src-rust/crates/query/src/agent_tool.rs`
- `src-rust/crates/query/src/cron_scheduler.rs`
