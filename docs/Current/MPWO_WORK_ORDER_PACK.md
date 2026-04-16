# Post-M11 Authority Pack

## Status

- This file is the single active authority artifact in `docs/Current/` for post-M11 work.
- `AGENTS.md` remains the repo-level governing document. Within `docs/Current/`, use this file as the controlling authority path.
- M11 is complete.
- Accepted M11 closeout hash: `b157924e130fdf71c09a3787b47dd5eb1f31d542`

Accepted runtime chain:
- `08R`
- `08B`
- `09`
- `10A`
- `10B1`
- `10B2`
- `11`
- `12`

## Active Authority Rules

- Treat `docs/Current/MPWO_WORK_ORDER_PACK.md` as the sole active current-authority artifact.
- Do not casually reopen runtime semantics from accepted M11 tickets on the split path.
- Use the archive reports for traceability to accepted history; do not restate or rewrite that history here.
- `docs/Current/TASK-M11-10_SPLIT_PLAN_AND_TEMPORARY_AUTHORITY_AMENDMENT.md` is deleted in this checkout and must not be treated as live authority or live retained context.
- Future prompts must not treat deleted or demoted split-era docs as active authority context in this checkout.

## Active Vs Historical In `docs/Current/`

Active:
- `docs/Current/MPWO_WORK_ORDER_PACK.md`

Historical / non-controlling:
- `docs/Current/TASK-M11-10_SPLIT_PLAN_AND_TEMPORARY_AUTHORITY_AMENDMENT.md` — deleted in this checkout; superseded split-era authority only, not live current-doc context
- `docs/Current/D1_REVIEW_REPORT_20260413T233604Z.md` — historical evidence only
- `docs/Current/M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md` — historical planning evidence only
- `docs/Current/IMPLEMENTATION_PLAN_MPWO.md` — historical planning context only

## M12 Authority Reconciliation

- The historical surrogate artifact exists only as archive documentation at `docs/archive/provider_orchestrator/d2_test_micro_patch_report.md`.
- No live tracked surrogate D2 test exists in source in the current checkout.
- `POST-M11-M12-AUTH-RECON` removes no live surrogate test because none exists.
- M12 is a live D2 coverage-audit and closure-decision milestone against the accepted post-M11 baseline at `b157924e130fdf71c09a3787b47dd5eb1f31d542`, plus explicit authority acknowledgment that the surrogate is archive-only historical context.
- If the current live D2 seams/tests fully subsume the old surrogate intent, M12 may be declared satisfied by audit outcome.
- If a real uncovered live delta remains, handle that delta in a separate follow-on ticket; do not fold it into this authority-reconciliation ticket.

## Archive Traceability

Accepted authority and planning trail:
- `docs/archive/reports/M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md`
- `docs/archive/reports/MPWO_REVISION_REPORT_20260414T003218Z.md`
- `docs/archive/reports/TASK-M11-08_AUTHORITY_RECONCILIATION_REPORT_20260415T002331Z.md`
- `docs/archive/reports/TASK-M11-08B_AUTHORITY_REPORT_20260415T005148Z.md`
- `docs/archive/reports/TASK-M11-10B_SPLIT_AUTHORITY_REPORT_20260415T031240Z.md`

Accepted closeout trail for the post-M11 state:
- `docs/archive/reports/TASK-M11-08R_CLOSEOUT_REPORT_20260415T004401Z.md`
- `docs/archive/reports/TASK-M11-08B_CLOSEOUT_REPORT_20260415T011952Z.md`
- `docs/archive/reports/TASK-M11-09_CLOSEOUT_REPORT_20260415T014220Z.md`
- `docs/archive/reports/TASK-M11-10A_CLOSEOUT_REPORT_20260415T024141Z.md`
- `docs/archive/reports/TASK-M11-10B1_CLOSEOUT_REPORT_20260415T034453Z.md`
- `docs/archive/reports/TASK-M11-10B2_CLOSEOUT_REPORT_20260415T041344Z.md`
- `docs/archive/reports/TASK-M11-11_CLOSEOUT_REPORT_20260415T053856Z.md`
- `docs/archive/reports/TASK-M11-12_GATE_CLARIFICATION_REPORT_20260415T060015Z.md`
- `docs/archive/reports/TASK-M11-12_CLOSEOUT_REPORT_20260415T065244Z.md`

Supporting post-closeout context:
- `docs/archive/reports/M11_CONVERGENCE_REVIEW_20260415T042825Z.md` — convergence review context; predates final closeout
- `docs/archive/reports/POST_M11_REPO_ASSESSMENT_20260415T071321Z.md`
- `docs/archive/reports/POST_M11_01_PREFLIGHT_REPORT_20260415T072257Z.md`

## Future Prompt Guidance

Use this guidance for future post-M11 prompts:

- Read and obey `AGENTS.md`.
- Read and obey `docs/Current/MPWO_WORK_ORDER_PACK.md`.
- Do not use the demoted `docs/Current/` files as controlling authority unless the prompt explicitly asks for historical traceability.
- Do not treat deleted or demoted split-era docs as live authority context or as evidence of a live surrogate D2 test in this checkout.
- Preserve the accepted post-M11 baseline at `b157924e130fdf71c09a3787b47dd5eb1f31d542` and the accepted runtime chain `08R -> 08B -> 09 -> 10A -> 10B1 -> 10B2 -> 11 -> 12`.
