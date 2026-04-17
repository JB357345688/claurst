# PROJECT_ACTIVITY_LEDGER

## 1. Status and usage note

This document is a non-authoritative evidence ledger for review/navigation only. `docs/Current/MPWO_WORK_ORDER_PACK.md` remains the sole live authority artifact. This ledger is built from archived reports, verified git history, and current repo inspection; it must not be used to replace current authority text or to infer unstated status. [Sources: docs/Current/MPWO_WORK_ORDER_PACK.md; docs/archive/reports/POST_M11_01_CLOSEOUT_REPORT_20260415T074353Z.md; git rev-parse HEAD]

## 2. Recommended reading order for external review

1. `docs/Current/MPWO_WORK_ORDER_PACK.md` for the sole live authority and accepted post-M11 chain. [Sources: docs/Current/MPWO_WORK_ORDER_PACK.md]
2. `docs/Current/PROJECT_ACTIVITY_LEDGER.md` for the non-authoritative navigation map across authority corrections, accepted baselines, replay evidence, and deferred items. [Sources: docs/Current/PROJECT_ACTIVITY_LEDGER.md]
3. `docs/archive/reports/POST_M11_MILESTONE_ACCEPTANCE_REPLAY_PLAN_REPORT_20260416T100911Z.md` for the replay split, review-basis caveats, and command mapping. [Sources: docs/archive/reports/POST_M11_MILESTONE_ACCEPTANCE_REPLAY_PLAN_REPORT_20260416T100911Z.md]
4. `docs/archive/reports/POST_M11_MILESTONE_ACCEPTANCE_REPLAY_PASS_A_REPORT_20260416T135153Z.md` for targeted provider-resolution seam replay on current `HEAD`. [Sources: docs/archive/reports/POST_M11_MILESTONE_ACCEPTANCE_REPLAY_PASS_A_REPORT_20260416T135153Z.md]
5. `docs/archive/reports/POST_M11_MILESTONE_ACCEPTANCE_REPLAY_PASS_B_REPORT_20260416T135621Z.md` for broad build/test replay, smoke result, dirty-worktree caveat, and current external-review recommendation. [Sources: docs/archive/reports/POST_M11_MILESTONE_ACCEPTANCE_REPLAY_PASS_B_REPORT_20260416T135621Z.md]
6. `docs/archive/reports/POST_M11_01_CLOSEOUT_REPORT_20260415T074353Z.md`, `docs/archive/reports/POST_M11_M12_AUTH_RECON_VERIFICATION_REPORT_20260416T062420Z.md`, and `docs/archive/reports/M12_CLOSEOUT_REPORT_20260416T065308Z.md` for the current-authority recast and the audit-only M12 conclusion. [Sources: docs/archive/reports/POST_M11_01_CLOSEOUT_REPORT_20260415T074353Z.md; docs/archive/reports/POST_M11_M12_AUTH_RECON_VERIFICATION_REPORT_20260416T062420Z.md; docs/archive/reports/M12_CLOSEOUT_REPORT_20260416T065308Z.md]
7. `docs/archive/reports/POST_M11_ARCH_01A_CLOSEOUT_REPORT_20260416T081839Z.md` for the latest accepted runtime extension beyond the M11 closeout baseline. [Sources: docs/archive/reports/POST_M11_ARCH_01A_CLOSEOUT_REPORT_20260416T081839Z.md]

## 3. External review readiness

- Provider-resolution lane external-review recommendation: `YES, WITH NOTES`. [Sources: docs/archive/reports/POST_M11_MILESTONE_ACCEPTANCE_REPLAY_PASS_A_REPORT_20260416T135153Z.md; docs/archive/reports/POST_M11_MILESTONE_ACCEPTANCE_REPLAY_PASS_B_REPORT_20260416T135621Z.md]
- Targeted replay passed on current `HEAD`, covering provider resolution, worker propagation seams, split-path D2 runtime seams, and `ARCH-01A` session-scoped `HealthCache` reuse. [Sources: docs/archive/reports/POST_M11_MILESTONE_ACCEPTANCE_REPLAY_PASS_A_REPORT_20260416T135153Z.md]
- Broad replay passed on current `HEAD` for workspace build, `claurst-api` tests, `claurst-query` tests, workspace tests, `cargo fmt --all -- --check`, and the OpenAI conditional smoke. [Sources: docs/archive/reports/POST_M11_MILESTONE_ACCEPTANCE_REPLAY_PASS_B_REPORT_20260416T135621Z.md]
- Review notes that remain live: the worktree is still dirty, so review-basis hygiene is imperfect, and `cargo clippy --workspace --all-targets -- -D warnings` is still red in `acp`, `buddy`, `plugins`, and `mcp`, which the replay report classifies as broader historical-gate debt outside the validated provider-resolution feature lane. [Sources: docs/archive/reports/POST_M11_MILESTONE_ACCEPTANCE_REPLAY_PLAN_REPORT_20260416T100911Z.md; docs/archive/reports/POST_M11_MILESTONE_ACCEPTANCE_REPLAY_PASS_B_REPORT_20260416T135621Z.md]
- This readiness note does not claim a clean worktree or solved broad repo hygiene; it claims current provider-resolution-lane replay support with explicit notes carried into review. [Sources: docs/archive/reports/POST_M11_MILESTONE_ACCEPTANCE_REPLAY_PASS_B_REPORT_20260416T135621Z.md]

## 4. Current repo snapshot

- Branch: `feature/provider-resolution-seam`. [Sources: git branch --show-current]
- HEAD: `038f3c20e01a96eec6397d506b477a461166f762` (`feat(query): add session-scoped HealthCache reuse`). [Sources: git rev-parse HEAD; git show --stat --oneline 038f3c2]
- Worktree summary at ledger generation: `6` modified tracked paths, `1` deleted tracked path, `313` untracked paths; representative noise includes `.gitignore`, deleted `docs/Current/TASK-M11-10_SPLIT_PLAN_AND_TEMPORARY_AUTHORITY_AMENDMENT.md`, multiple `src-rust/crates/core/*` files, and many untracked report/archive paths. [Sources: git status --short --branch; awk count probe]
- Provider-resolution-lane evidence is stronger than repo-hygiene evidence: replay Pass A and Pass B support the lane on current `HEAD`, while broader worktree and historical-gate noise remain separate review notes. [Sources: docs/archive/reports/POST_M11_MILESTONE_ACCEPTANCE_REPLAY_PASS_A_REPORT_20260416T135153Z.md; docs/archive/reports/POST_M11_MILESTONE_ACCEPTANCE_REPLAY_PASS_B_REPORT_20260416T135621Z.md]
- Broader repo/worktree noise is still unresolved: this ledger does not claim a clean branch, patch-isolated review basis, or fully green workspace historical gates outside the provider-resolution lane. [Sources: git status --short --branch; docs/archive/reports/POST_M11_MILESTONE_ACCEPTANCE_REPLAY_PLAN_REPORT_20260416T100911Z.md; docs/archive/reports/POST_M11_MILESTONE_ACCEPTANCE_REPLAY_PASS_B_REPORT_20260416T135621Z.md]
- Current authority artifact: `docs/Current/MPWO_WORK_ORDER_PACK.md`. [Sources: docs/Current/MPWO_WORK_ORDER_PACK.md; find docs/Current -maxdepth 1 -type f | sort]
- Ledger generated at UTC `20260416T164102Z`. [Sources: date -u +%Y%m%dT%H%M%SZ]

## 5. Current authority summary

- `docs/Current/MPWO_WORK_ORDER_PACK.md` says it is the single active authority artifact in `docs/Current/`, records `M11` complete, and names accepted M11 closeout hash `b157924e130fdf71c09a3787b47dd5eb1f31d542`. [Sources: docs/Current/MPWO_WORK_ORDER_PACK.md; docs/archive/reports/POST_M11_01_CLOSEOUT_REPORT_20260415T074353Z.md; commit b157924e130fdf71c09a3787b47dd5eb1f31d542]
- The same pack records the accepted post-M11 runtime chain as `08R -> 08B -> 09 -> 10A -> 10B1 -> 10B2 -> 11 -> 12`. [Sources: docs/Current/MPWO_WORK_ORDER_PACK.md; docs/archive/reports/POST_M11_01_CLOSEOUT_REPORT_20260415T074353Z.md]
- The deleted split-era amendment file is explicitly non-live authority in this checkout, and the other `docs/Current/` files are historical/non-controlling only. [Sources: docs/Current/MPWO_WORK_ORDER_PACK.md; docs/archive/reports/POST_M11_01_CLOSEOUT_REPORT_20260415T074353Z.md; find docs/Current -maxdepth 1 -type f | sort; git status --short --branch]
- Current authority recasts `M12` as a D2 coverage-audit / closure-decision milestone, states that the old surrogate exists only as archive documentation, and says any uncovered live delta must become a separate follow-on ticket. The later accepted M12 audit found no such live delta. [Sources: docs/Current/MPWO_WORK_ORDER_PACK.md; docs/archive/reports/POST_M11_M12_AUTH_RECON_VERIFICATION_REPORT_20260416T062420Z.md; docs/archive/reports/M12_RECUT_PREFLIGHT_REPORT_20260416T064259Z.md; docs/archive/reports/M12_CLOSEOUT_REPORT_20260416T065308Z.md]

## 6. Milestone / ticket ledger table

| ID | Category | Derived status | Short scope summary | Evidence | Commit hash(es), if proven | Notes |
| --- | --- | --- | --- | --- | --- | --- |
| `docs/Current/MPWO_WORK_ORDER_PACK.md` | authority pack | current-authority | Sole live authority pack; says `M11` is complete and `M12` is audit/closure-oriented. | `docs/Current/MPWO_WORK_ORDER_PACK.md`; `POST_M11_01_CLOSEOUT_REPORT_20260415T074353Z.md`; `M12_CLOSEOUT_REPORT_20260416T065308Z.md` | `b157924e130fdf71c09a3787b47dd5eb1f31d542` baseline named; current HEAD `038f3c20e01a96eec6397d506b477a461166f762` separately verified | Live authority, not a restatement target. |
| `D1 (M7-M9)` | milestone | accepted-baseline | Provider-resolution seam, worker propagation, and D1 validation/smoke accepted. | `D1_REVIEW_REPORT_20260413T233604Z.md`; `MILESTONE_M8_WRAPUP_CHECK_20260413T004403Z.md` | `b5b6dd4`; `6b362a0` | Ledger keeps D1 mostly at milestone granularity, not per-ticket replay. |
| `M10 D2 plan` | integration planning | docs-only | Planning-only D2 ladder grounded on accepted D1 baseline. | `M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md` |  | Later authority corrections narrowed several M11 assumptions. |
| `MPWO revision` | authority correction | superseded | Revised MPWO from 11-ticket to 12-ticket M11 ladder, inserting `M11-09`; later split/correction work still superseded parts of this wording. | `MPWO_REVISION_REPORT_20260414T003218Z.md`; `TASK-M11-10B_SPLIT_AUTHORITY_REPORT_20260415T031240Z.md`; `docs/Current/MPWO_WORK_ORDER_PACK.md` |  | Historical context only; not the current live authority form. |
| `M11-01..07` | ticket chain | accepted-baseline | D2 substrate landed: trust-domain, model capability data, fallback substrate, cost tracking, and session-budget substrate. | `M11_CONVERGENCE_REVIEW_20260415T042825Z.md`; `docs/Current/MPWO_WORK_ORDER_PACK.md`; git show | `dc772aa`; `fe21969`; `828b08e`; `eb26773`; `1472024`; `cf8201f`; `0942e4a` | Exact per-ticket closeout hashes are listed in the convergence review and were rechecked against git history in this corrective pass. |
| `TASK-M11-08 authority reconciliation` | authority correction | superseded | Found original `M11-08` structurally invalid as written; deferred child/team propagation and `HealthCache` assumptions out of corrected `08R`. | `TASK-M11-08_AUTHORITY_RECONCILIATION_REPORT_20260415T002331Z.md` |  | Historical authority-correction step; later replaced by accepted split path. |
| `TASK-M11-08R` | ticket | accepted-baseline | Root `SessionBudget` wiring and root cancel-token observation only. | `TASK-M11-08R_CLOSEOUT_REPORT_20260415T004401Z.md`; git show | `25518cac29d34353cb58c8811da1040a3da69247` | Explicitly excluded `ToolContext.session_budget`, `ToolContext.health_cache`, and child/team propagation. |
| `TASK-M11-08B` | ticket | accepted-baseline | Query-owned inherited parent `SessionBudget` propagation into child/team query loops. | `TASK-M11-08B_CLOSEOUT_REPORT_20260415T011952Z.md`; git show | `b4ad28ee9eb1e434f935fd2de70c4b402a5c74da` | Explicitly excluded TeamCreate outer-cancellation redesign and `HealthCache` plumbing. |
| `TASK-M11-09` | ticket | accepted-baseline | Child `max_tokens` override wiring across foreground, background, and team-runner spawn paths. | `TASK-M11-09_CLOSEOUT_REPORT_20260415T014220Z.md`; git show | `4ef9547dab51959f7b39c473f929b81f05ee1134` | Retained `4096` fallback as backward-compatible default. |
| `TASK-M11-10 split authority` | authority correction | superseded | Replaced unsplit `10B` assumption with `10A`, `10B1`, and `10B2`; kept `ToolContext.*` carriage and TeamCreate cancellation redesign out of scope. | `TASK-M11-10B_SPLIT_AUTHORITY_REPORT_20260415T031240Z.md`; `docs/Current/MPWO_WORK_ORDER_PACK.md` |  | Historical split/correction step; live authority now references the accepted chain, not this doc directly. |
| `TASK-M11-10A` | ticket | accepted-baseline | Child `allow_fallback` schema/runtime wiring. | `TASK-M11-10A_CLOSEOUT_REPORT_20260415T024141Z.md`; git show | `ea046c52da82dfd9778f4065bd36b36e28d73c8a` | Preserved accepted `08B`/`09` behavior. |
| `TASK-M11-10B1` | ticket | accepted-baseline | Query-owned layered child budget seam before any child/team `budget_usd` carriage. | `TASK-M11-10B1_CLOSEOUT_REPORT_20260415T034453Z.md`; git show | `3812df04ec87ce0e96f851da2d18ab38b24f0b99` | Explicitly kept `budget_usd` schema/runtime carriage blocked until acceptance. |
| `TASK-M11-10B2` | ticket | accepted-baseline | Child/team `budget_usd` schema and runtime carriage on top of accepted `10B1` seam. | `TASK-M11-10B2_CLOSEOUT_REPORT_20260415T041344Z.md`; git show | `bfabfd5934b0afe801f2e7de9f15a7f6258c563b` | Explicitly preserved distinction from `max_budget_usd` and rejected `ToolContext.*` carriage. |
| `TASK-M11-11` | ticket | accepted-baseline | Query-event / observability expansion for worker-provider and budget events. | `TASK-M11-11_CLOSEOUT_REPORT_20260415T053856Z.md`; git show | `0c9dac407e82fccdfe16337bc2c05a6aeb816ca5` | Treated as revised `M11-11`, not stale numbering. |
| `TASK-M11-12` | ticket | accepted-baseline | Split-path D2 validation coverage landed and closed on clarified crate-level gate. | `TASK-M11-12_GATE_CLARIFICATION_REPORT_20260415T060015Z.md`; `TASK-M11-12_CLOSEOUT_REPORT_20260415T065244Z.md`; git show | `b157924e130fdf71c09a3787b47dd5eb1f31d542` | `cargo fmt` / workspace `clippy` were recorded as informational only at closeout. |
| `POST-M11 repo assessments` | post-milestone assessment | historical-context | Post-M11 analysis identified authority drift, cleanup debt, and safe architectural deferrals without finding a functional hole in accepted M11 runtime behavior. | `POST_M11_REPO_ASSESSMENT_20260415T071321Z.md`; `POST_M11_FULL_REPO_EVALUATION_DEFERRED_ITEMS_AND_M12_20260416T053356Z.md` |  | Assessment/context only; not authority. |
| `POST-M11-01` | post-ticket docs consolidation | docs-only | Consolidated `docs/Current/` so the pack is the sole live authority artifact and other current docs are demoted/historical. | `POST_M11_01_CLOSEOUT_REPORT_20260415T074353Z.md`; git show | `560b54f` | Important authority hygiene baseline. |
| `POST-M11 cleanup ladder (02..08A, grouped)` | post-ticket cleanup | historical-context | Accepted formatter/lint/API/query-test-organization cleanup advanced HEAD beyond `b157924` before M12 recast and `ARCH-01A`. | `POST_M11_FULL_REPO_EVALUATION_DEFERRED_ITEMS_AND_M12_20260416T053356Z.md`; `POST_M11_MILESTONE_ACCEPTANCE_REPLAY_PLAN_REPORT_20260416T100911Z.md`; git log | grouped context only; exact per-item hash not asserted | Reports and git history support the grouped historical lane, but this corrective pass did not safely re-ledger each cleanup ticket one by one. |
| `POST-M11-M12-AUTH-RECON` | authority correction | docs-only | Recast active authority so M12 no longer assumes a live surrogate test in source. | `POST_M11_M12_AUTH_RECON_VERIFICATION_REPORT_20260416T062420Z.md`; `docs/Current/MPWO_WORK_ORDER_PACK.md`; git show | `63595c387ac8fd2f5adbf9cf75d45a724153c3db` | Docs-only correction; no runtime delta. |
| `M12` | post-ticket audit | closed-no-commit | Accepted audit concluded old surrogate intent is already covered by live D2 seams/tests; no implementation pass or commit required. | `M12_RECUT_PREFLIGHT_REPORT_20260416T064259Z.md`; `M12_CLOSEOUT_REPORT_20260416T065308Z.md` |  | Closed by audit, not by new code. |
| `POST-M11-ARCH-01A` | architecture follow-on | accepted-baseline | Added query-owned session-scoped `HealthCache` reuse for child/team fallback paths. | `POST_M11_ARCH_01_PREFLIGHT_REPORT_20260416T072504Z.md`; `POST_M11_ARCH_01A_VERIFICATION_REPORT_20260416T080436Z.md`; `POST_M11_ARCH_01A_CLOSEOUT_REPORT_20260416T081839Z.md`; git show | `038f3c20e01a96eec6397d506b477a461166f762` | Explicitly excluded `ToolContext.health_cache`, process-global cache semantics, and ARCH-02/03 redesigns. |
| `POST-M11-ARCH-02` | architecture deferral | deferred | `ToolContext` budget/cache carriage reconsideration remains deferred; current query-owned seams are treated as valid and sufficient. | `POST_M11_ARCH_02_PREFLIGHT_REPORT_20260416T084418Z.md`; `POST_M11_REPO_ASSESSMENT_20260415T071321Z.md` |  | Not a blocker for current baseline. |
| `POST-M11-ARCH-03` | architecture deferral | deferred | TeamCreate outer-cancellation redesign remains deferred; current two-layer cancellation ownership is treated as good enough for accepted runtime path. | `POST_M11_ARCH_03_PREFLIGHT_REPORT_20260416T091621Z.md`; `POST_M11_REPO_ASSESSMENT_20260415T071321Z.md` |  | Not a blocker for current baseline. |

## 7. Accepted baseline chain

- `D1` established the accepted provider-resolution seam baseline: provider/model routing landed, workers inherited parent provider/model choices, and D1 validation/smoke was accepted. Important exclusion: D1 explicitly did not start D2 work. [Sources: docs/archive/reports/D1_REVIEW_REPORT_20260413T233604Z.md; docs/archive/reports/MILESTONE_M8_WRAPUP_CHECK_20260413T004403Z.md; commit b5b6dd4; commit 6b362a0]
- `M11-01..07` plus corrected split runtime chain `08R -> 08B -> 09 -> 10A -> 10B1 -> 10B2 -> 11 -> 12` form the accepted D2 baseline named by the current pack and closed at `b157924e130fdf71c09a3787b47dd5eb1f31d542`. Important exclusions: `ToolContext.session_budget`, `ToolContext.health_cache`, shared/global `HealthCache`, and TeamCreate outer-cancellation redesign were not accepted as part of this chain. [Sources: docs/Current/MPWO_WORK_ORDER_PACK.md; docs/archive/reports/M11_CONVERGENCE_REVIEW_20260415T042825Z.md; docs/archive/reports/TASK-M11-08R_CLOSEOUT_REPORT_20260415T004401Z.md; docs/archive/reports/TASK-M11-08B_CLOSEOUT_REPORT_20260415T011952Z.md; docs/archive/reports/TASK-M11-09_CLOSEOUT_REPORT_20260415T014220Z.md; docs/archive/reports/TASK-M11-10A_CLOSEOUT_REPORT_20260415T024141Z.md; docs/archive/reports/TASK-M11-10B1_CLOSEOUT_REPORT_20260415T034453Z.md; docs/archive/reports/TASK-M11-10B2_CLOSEOUT_REPORT_20260415T041344Z.md; docs/archive/reports/TASK-M11-11_CLOSEOUT_REPORT_20260415T053856Z.md; docs/archive/reports/TASK-M11-12_CLOSEOUT_REPORT_20260415T065244Z.md; commit b157924e130fdf71c09a3787b47dd5eb1f31d542]
- `POST-M11-01` established the current authority baseline: one live current pack, other `docs/Current/` files demoted to historical/non-controlling roles. [Sources: docs/archive/reports/POST_M11_01_CLOSEOUT_REPORT_20260415T074353Z.md; commit 560b54f]
- `POST-M11-M12-AUTH-RECON` plus `M12` established that M12 is audit-only on the accepted D2 baseline and required no implementation delta or commit. [Sources: docs/archive/reports/POST_M11_M12_AUTH_RECON_VERIFICATION_REPORT_20260416T062420Z.md; docs/archive/reports/M12_RECUT_PREFLIGHT_REPORT_20260416T064259Z.md; docs/archive/reports/M12_CLOSEOUT_REPORT_20260416T065308Z.md; commit 63595c387ac8fd2f5adbf9cf75d45a724153c3db]
- `POST-M11-ARCH-01A` is the latest accepted runtime extension in the lane: session-scoped `HealthCache` reuse is query-owned and session-keyed, while broader carriage/redesign questions remain excluded. [Sources: docs/archive/reports/POST_M11_ARCH_01A_VERIFICATION_REPORT_20260416T080436Z.md; docs/archive/reports/POST_M11_ARCH_01A_CLOSEOUT_REPORT_20260416T081839Z.md; commit 038f3c20e01a96eec6397d506b477a461166f762]

## 8. Recast / superseded / authority-correction chain

- Original M10 planning and the first revised MPWO described a broader M11 ladder, including assumptions later found structurally invalid around `ToolContext` carriage and unsplit budget/fallback work. Those documents remain historical context only. [Sources: docs/archive/reports/M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md; docs/archive/reports/MPWO_REVISION_REPORT_20260414T003218Z.md]
- `TASK-M11-08_AUTHORITY_RECONCILIATION_REPORT_20260415T002331Z.md` corrected original `M11-08` by explicitly deferring child/team propagation and `HealthCache` assumptions out of the root-only `08R` scope. [Sources: docs/archive/reports/TASK-M11-08_AUTHORITY_RECONCILIATION_REPORT_20260415T002331Z.md]
- `TASK-M11-10B_SPLIT_AUTHORITY_REPORT_20260415T031240Z.md` corrected unsplit `10B` by splitting it into `10B1` and `10B2`, and it explicitly kept `ToolContext.session_budget`, `ToolContext.health_cache`, and TeamCreate outer-cancellation redesign out of scope. [Sources: docs/archive/reports/TASK-M11-10B_SPLIT_AUTHORITY_REPORT_20260415T031240Z.md]
- `POST-M11-01` then demoted stale/current mixed docs in `docs/Current/` so the pack became the single live authority artifact; older current-doc copies became historical evidence only. [Sources: docs/archive/reports/POST_M11_01_CLOSEOUT_REPORT_20260415T074353Z.md; docs/Current/MPWO_WORK_ORDER_PACK.md]
- `POST-M11-M12-AUTH-RECON` and `M12_RECUT_PREFLIGHT` corrected stale M12 assumptions by recognizing that the supposed surrogate test exists only in archive docs, not as a live tracked source artifact. That recast changed M12 from a presumed implementation/removal ticket into an audit/closure decision. [Sources: docs/archive/reports/POST_M11_M12_AUTH_RECON_VERIFICATION_REPORT_20260416T062420Z.md; docs/archive/reports/M12_RECUT_PREFLIGHT_REPORT_20260416T064259Z.md; docs/archive/reports/M12_CLOSEOUT_REPORT_20260416T065308Z.md]

## 9. Deferred / backlog architecture items

| ID / label | What it concerns | Why deferred | What evidence says about sequencing risk | Currently a blocker? |
| --- | --- | --- | --- | --- |
| `Residual shared/global HealthCache question after ARCH-01A` | Whether to go beyond accepted session-scoped query-owned reuse toward broader shared/global cache semantics. | `ARCH-01A` addressed the concrete same-session reuse gap narrowly; broader process/global semantics were not justified and remained explicitly out of scope. [Sources: docs/archive/reports/POST_M11_ARCH_01_PREFLIGHT_REPORT_20260416T072504Z.md; docs/archive/reports/POST_M11_ARCH_01A_VERIFICATION_REPORT_20260416T080436Z.md; docs/archive/reports/POST_M11_FULL_REPO_EVALUATION_DEFERRED_ITEMS_AND_M12_20260416T053356Z.md] | Reopening broader ownership now would change fallback runtime assumptions beyond the accepted D2 baseline; later reports preferred M12 audit before broader architecture changes. [Sources: docs/archive/reports/POST_M11_FULL_REPO_EVALUATION_DEFERRED_ITEMS_AND_M12_20260416T053356Z.md] | `no` |
| `POST-M11-ARCH-02` | Reconsidering `ToolContext.session_budget` / `ToolContext.health_cache` or some unified carriage abstraction. | The original assumption was declared structurally invalid, and current query-owned seams are treated as sufficient after `ARCH-01A`. [Sources: docs/archive/reports/TASK-M11-08_AUTHORITY_RECONCILIATION_REPORT_20260415T002331Z.md; docs/archive/reports/POST_M11_ARCH_02_PREFLIGHT_REPORT_20260416T084418Z.md] | Later reports say reopening it would pressure broader crate/API redesign and is not needed for current accepted runtime behavior. [Sources: docs/archive/reports/POST_M11_ARCH_02_PREFLIGHT_REPORT_20260416T084418Z.md; docs/archive/reports/POST_M11_REPO_ASSESSMENT_20260415T071321Z.md] | `no` |
| `POST-M11-ARCH-03` | TeamCreate outer-cancellation redesign. | Current evidence shows a coherent direct TeamDelete stop path plus separate query/session-budget cancellation ownership; visible issues are test/observability gaps, not proof of wrong ownership. [Sources: docs/archive/reports/POST_M11_ARCH_03_PREFLIGHT_REPORT_20260416T091621Z.md] | Reopening it would touch tools/query boundary seams and risk regressions in accepted cache/budget/provider behavior. [Sources: docs/archive/reports/POST_M11_ARCH_03_PREFLIGHT_REPORT_20260416T091621Z.md] | `no` |

## 10. Open questions / evidence gaps

- The accepted post-M11 cleanup ladder between `POST-M11-01` and `POST-M11-ARCH-01A` remains conservatively grouped historical context in this ledger. Reports indicate those commits are accepted cleanup/test-organization work, but this corrective pass did not safely re-open each closeout individually enough to assert exact per-item hashes here. [Sources: docs/archive/reports/POST_M11_FULL_REPO_EVALUATION_DEFERRED_ITEMS_AND_M12_20260416T053356Z.md; docs/archive/reports/POST_M11_MILESTONE_ACCEPTANCE_REPLAY_PLAN_REPORT_20260416T100911Z.md; git log --oneline --decorate --graph --max-count=250 --all]
- Early D1 detail before milestone wrap-up was summarized from milestone-level accepted reports instead of re-reading every M7/M9 ticket artifact in this build. No contradiction was found, but a fully per-ticket D1 ledger would require a separate pass. [Sources: docs/archive/reports/D1_REVIEW_REPORT_20260413T233604Z.md; docs/archive/reports/MILESTONE_M8_WRAPUP_CHECK_20260413T004403Z.md]

## 11. Source index

### Current authority

- `docs/Current/MPWO_WORK_ORDER_PACK.md`

### Milestone closeout / verification

- `docs/archive/reports/D1_REVIEW_REPORT_20260413T233604Z.md`
- `docs/archive/reports/MILESTONE_M8_WRAPUP_CHECK_20260413T004403Z.md`
- `docs/archive/reports/TASK-M11-08R_CLOSEOUT_REPORT_20260415T004401Z.md`
- `docs/archive/reports/TASK-M11-08B_CLOSEOUT_REPORT_20260415T011952Z.md`
- `docs/archive/reports/TASK-M11-09_CLOSEOUT_REPORT_20260415T014220Z.md`
- `docs/archive/reports/TASK-M11-10A_CLOSEOUT_REPORT_20260415T024141Z.md`
- `docs/archive/reports/TASK-M11-10B1_CLOSEOUT_REPORT_20260415T034453Z.md`
- `docs/archive/reports/TASK-M11-10B2_CLOSEOUT_REPORT_20260415T041344Z.md`
- `docs/archive/reports/TASK-M11-11_CLOSEOUT_REPORT_20260415T053856Z.md`
- `docs/archive/reports/TASK-M11-12_GATE_CLARIFICATION_REPORT_20260415T060015Z.md`
- `docs/archive/reports/TASK-M11-12_CLOSEOUT_REPORT_20260415T065244Z.md`
- `docs/archive/reports/POST_M11_ARCH_01A_CLOSEOUT_REPORT_20260416T081839Z.md`

### Post-milestone assessments

- `docs/archive/reports/M11_CONVERGENCE_REVIEW_20260415T042825Z.md`
- `docs/archive/reports/POST_M11_REPO_ASSESSMENT_20260415T071321Z.md`
- `docs/archive/reports/POST_M11_FULL_REPO_EVALUATION_DEFERRED_ITEMS_AND_M12_20260416T053356Z.md`
- `docs/archive/reports/M12_RECUT_PREFLIGHT_REPORT_20260416T064259Z.md`
- `docs/archive/reports/M12_CLOSEOUT_REPORT_20260416T065308Z.md`
- `docs/archive/reports/POST_M11_MILESTONE_ACCEPTANCE_REPLAY_PASS_A_REPORT_20260416T135153Z.md`
- `docs/archive/reports/POST_M11_MILESTONE_ACCEPTANCE_REPLAY_PASS_B_REPORT_20260416T135621Z.md`

### Architecture deferrals / authority corrections

- `docs/archive/reports/TASK-M11-08_AUTHORITY_RECONCILIATION_REPORT_20260415T002331Z.md`
- `docs/archive/reports/TASK-M11-10B_SPLIT_AUTHORITY_REPORT_20260415T031240Z.md`
- `docs/archive/reports/POST_M11_M12_AUTH_RECON_VERIFICATION_REPORT_20260416T062420Z.md`
- `docs/archive/reports/POST_M11_ARCH_01_PREFLIGHT_REPORT_20260416T072504Z.md`
- `docs/archive/reports/POST_M11_ARCH_01A_VERIFICATION_REPORT_20260416T080436Z.md`
- `docs/archive/reports/POST_M11_ARCH_02_PREFLIGHT_REPORT_20260416T084418Z.md`
- `docs/archive/reports/POST_M11_ARCH_03_PREFLIGHT_REPORT_20260416T091621Z.md`

### Integration planning

- `docs/archive/reports/M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md`
- `docs/archive/reports/MPWO_REVISION_REPORT_20260414T003218Z.md`
- `docs/archive/reports/POST_M11_MILESTONE_ACCEPTANCE_REPLAY_PLAN_REPORT_20260416T100911Z.md`
