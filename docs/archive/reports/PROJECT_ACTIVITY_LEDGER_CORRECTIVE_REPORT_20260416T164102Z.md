# PROJECT_ACTIVITY_LEDGER_CORRECTIVE_REPORT

## 1. timestamp UTC

- `20260416T164102Z`

## 2. branch / HEAD / worktree summary

- Branch: `feature/provider-resolution-seam`
- `HEAD`: `038f3c20e01a96eec6397d506b477a461166f762`
- Worktree during corrective pass pre-edit verification: `6` modified tracked paths, `1` deleted tracked path, `312` untracked paths
- Scope note: the worktree was already dirty outside this docs-only pass; that noise was treated as review-basis context, not as a reason to widen scope

## 3. authority reviewed

- `AGENTS.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md`
- accepted archive evidence under `docs/archive/reports/`, with emphasis on:
  - `POST_M11_01_CLOSEOUT_REPORT_20260415T074353Z.md`
  - `POST_M11_M12_AUTH_RECON_VERIFICATION_REPORT_20260416T062420Z.md`
  - `M12_CLOSEOUT_REPORT_20260416T065308Z.md`
  - `POST_M11_ARCH_01A_CLOSEOUT_REPORT_20260416T081839Z.md`
  - `POST_M11_MILESTONE_ACCEPTANCE_REPLAY_PLAN_REPORT_20260416T100911Z.md`
  - `POST_M11_MILESTONE_ACCEPTANCE_REPLAY_PASS_A_REPORT_20260416T135153Z.md`
  - `POST_M11_MILESTONE_ACCEPTANCE_REPLAY_PASS_B_REPORT_20260416T135621Z.md`

## 4. exact commands run

```text
git branch --show-current
git rev-parse HEAD
git status --short --branch
sed -n '1,260p' docs/Current/MPWO_WORK_ORDER_PACK.md
sed -n '1,260p' docs/Current/PROJECT_ACTIVITY_LEDGER.md
rg -n "PROJECT_ACTIVITY_LEDGER|grouped context only|recommended reading order|external review|PASS-WITH-NOTES|YES, WITH NOTES|YES-WITH-NOTES|clippy|dirty worktree" docs/Current/PROJECT_ACTIVITY_LEDGER.md docs/archive/reports/*.md
git log --oneline --decorate --graph --max-count=250 --all
rg -n "grouped context only|dc772aa|0942e4a|POST-M11-01|POST_M11_ARCH_01A|targeted replay|broad build|broad test|dirty worktree|workspace clippy|credential|smoke|external review|with notes|YES-WITH-NOTES|PASS-WITH-NOTES" docs/Current/PROJECT_ACTIVITY_LEDGER.md docs/archive/reports/*.md
sed -n '1,220p' docs/archive/reports/POST_M11_MILESTONE_ACCEPTANCE_REPLAY_PLAN_REPORT_20260416T100911Z.md
sed -n '1,260p' docs/archive/reports/POST_M11_ARCH_01A_CLOSEOUT_REPORT_20260416T081839Z.md
sed -n '1,260p' docs/archive/reports/M12_CLOSEOUT_REPORT_20260416T065308Z.md
date -u +%Y%m%dT%H%M%SZ
sed -n '40,80p' docs/archive/reports/M11_CONVERGENCE_REVIEW_20260415T042825Z.md
sed -n '1,240p' docs/archive/reports/POST_M11_MILESTONE_ACCEPTANCE_REPLAY_PASS_A_REPORT_20260416T135153Z.md
sed -n '1,260p' docs/archive/reports/POST_M11_MILESTONE_ACCEPTANCE_REPLAY_PASS_B_REPORT_20260416T135621Z.md
nl -ba docs/Current/PROJECT_ACTIVITY_LEDGER.md | sed -n '1,120p'
git show --stat --oneline dc772aa
git show --stat --oneline fe21969
git show --stat --oneline 828b08e
git show --stat --oneline eb26773
git show --stat --oneline 1472024
git show --stat --oneline cf8201f
git show --stat --oneline 0942e4a
rg --files docs/archive/reports | rg 'POST_M11_MILESTONE_ACCEPTANCE_REPLAY_PASS'
git status --short | awk 'BEGIN{m=0;d=0;u=0} /^\\?\\? /{u++} /^ D /{d++} /^[ MARCUD][MDARCUT?] /{if (substr($0,1,3) != "?? ") m++} END{printf("modified_or_other_tracked=%d\\ndeleted_tracked=%d\\nuntracked=%d\\n", m-d, d, u)}'
git status --short --branch
rg -n "Recommended reading order for external review|External review readiness|YES, WITH NOTES|grouped context only; exact per-item hash not asserted|Provider-resolution-lane evidence|Broader repo/worktree noise" docs/Current/PROJECT_ACTIVITY_LEDGER.md docs/archive/reports/PROJECT_ACTIVITY_LEDGER_CORRECTIVE_REPORT_20260416T164102Z.md
sed -n '1,170p' docs/Current/PROJECT_ACTIVITY_LEDGER.md
sed -n '1,220p' docs/archive/reports/PROJECT_ACTIVITY_LEDGER_CORRECTIVE_REPORT_20260416T164102Z.md
```

## 5. files inspected

- `docs/Current/MPWO_WORK_ORDER_PACK.md`
- `docs/Current/PROJECT_ACTIVITY_LEDGER.md`
- `docs/archive/reports/M11_CONVERGENCE_REVIEW_20260415T042825Z.md`
- `docs/archive/reports/POST_M11_01_CLOSEOUT_REPORT_20260415T074353Z.md`
- `docs/archive/reports/POST_M11_M12_AUTH_RECON_VERIFICATION_REPORT_20260416T062420Z.md`
- `docs/archive/reports/M12_CLOSEOUT_REPORT_20260416T065308Z.md`
- `docs/archive/reports/POST_M11_ARCH_01A_CLOSEOUT_REPORT_20260416T081839Z.md`
- `docs/archive/reports/POST_M11_MILESTONE_ACCEPTANCE_REPLAY_PLAN_REPORT_20260416T100911Z.md`
- `docs/archive/reports/POST_M11_MILESTONE_ACCEPTANCE_REPLAY_PASS_A_REPORT_20260416T135153Z.md`
- `docs/archive/reports/POST_M11_MILESTONE_ACCEPTANCE_REPLAY_PASS_B_REPORT_20260416T135621Z.md`

## 6. ledger corrections applied

- Added a compact `Recommended reading order for external review` section near the top, anchored on the live authority pack, replay plan, final replay passes, key authority-recast reports, and the latest accepted runtime extension
- Added a concise `External review readiness` section stating `YES, WITH NOTES` for the provider-resolution lane, with explicit separation between lane evidence and broader repo hygiene
- Replaced the `M11-01..07` grouped range notation with exact verified hashes:
  - `dc772aa`
  - `fe21969`
  - `828b08e`
  - `eb26773`
  - `1472024`
  - `cf8201f`
  - `0942e4a`
- Downgraded the post-M11 cleanup ladder commit cell from grouped range notation to `grouped context only; exact per-item hash not asserted`
- Tightened worktree-noise wording so the ledger distinguishes provider-resolution replay evidence from broader dirty-worktree and historical-gate noise

## 7. any places where exact hashes could NOT be safely asserted

- `POST-M11 cleanup ladder (02..08A, grouped)` remains grouped historical context only
- Reason: the archive evidence and `git log` support the lane as a grouped accepted sequence, but this corrective pass did not reopen each cleanup closeout deeply enough to assert exact per-item hashes without over-claiming precision

## 8. readiness wording chosen and exact evidence basis

- Chosen wording: `YES, WITH NOTES`
- Exact evidence basis:
  - targeted replay passed on current `HEAD` in `POST_M11_MILESTONE_ACCEPTANCE_REPLAY_PASS_A_REPORT_20260416T135153Z.md`
  - broad build/test replay passed on current `HEAD` in `POST_M11_MILESTONE_ACCEPTANCE_REPLAY_PASS_B_REPORT_20260416T135621Z.md`
  - dirty worktree remained a review-basis caveat in the replay plan and both replay passes
  - workspace `clippy` remained red outside the provider-resolution feature lane in the final Pass B report
- Smoke note:
  - this refined ledger cites the later Pass B report `POST_M11_MILESTONE_ACCEPTANCE_REPLAY_PASS_B_REPORT_20260416T135621Z.md`, where the OpenAI conditional smoke passed with `PARENT_OK: CHILD_OK`
  - an earlier Pass B report in the archive recorded `invalid_api_key`, but that credential-limited attempt was not used as the primary readiness basis for the refined ledger

## 9. final verdict

- `READY-WITH-NAMED-LIMITATIONS`

## 10. explicit confirmation

- no source code changed
- no tests changed
- no commit created
