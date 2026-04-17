# POST-M11-09 Preflight Report

## 1. ticket id

`POST-M11-09`

## 2. verdict

`PASS-WITH-NOTES`

## 3. timestamp UTC

`20260416T002821Z`

## 4. branch / HEAD observed

- Branch: `feature/provider-resolution-seam`
- HEAD: `2def737b4a723184db22b791f6527609db7abc8e`
- Expected accepted HEAD: `2def737b4a723184db22b791f6527609db7abc8e`
- HEAD match: `yes`
- Worktree state: `dirty / noisy`

## 5. authority reviewed

- Governing authority:
  - `AGENTS.md`
  - `docs/Current/MPWO_WORK_ORDER_PACK.md`
- Accepted evidence reviewed:
  - `docs/archive/reports/POST_M11_REPO_ASSESSMENT_20260415T071321Z.md`
  - `docs/archive/reports/POST_M11_08_PREFLIGHT_REPORT_20260415T232456Z.md`
  - `docs/archive/reports/POST_M11_08A_CLOSEOUT_REPORT_20260416T001346Z.md`
- Verified current-authority reality:
  - `docs/Current/MPWO_WORK_ORDER_PACK.md` still states it is the sole active current-authority artifact
  - `docs/Current/` still contains historical/non-controlling files alongside it:
    - `D1_REVIEW_REPORT_20260413T233604Z.md`
    - `IMPLEMENTATION_PLAN_MPWO.md`
    - `M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md`
    - `TASK-M11-10_SPLIT_PLAN_AND_TEMPORARY_AUTHORITY_AMENDMENT.md`
- Verified commands used in this preflight:
  - `git branch --show-current`
  - `git rev-parse HEAD`
  - `git status --short --branch`
  - `find`
  - `rg`
  - `sed`
  - `git diff -- .gitignore`
  - `git ls-files --others --exclude-standard`

## 6. current worktree-noise findings

- Tracked modification:
  - `.gitignore` is modified
  - live diff is only:
    - add `.envrc`
    - add `.env`
  - this does not yet address the main recurring noise surfaces
- Generated/build artifacts:
  - `src-rust/target/` is untracked
  - visible subtrees include `debug/` and `tmp/`
  - this is standard generated build output, not source drift
- Local tooling / scratch paths:
  - `.codex` is untracked
  - `src-rust/.codex` is untracked
  - both currently appear as directory markers only, with no tracked baseline
  - these look intentional/local, not runtime repo content
- Report artifacts:
  - `docs/archive/reports/` currently contains `289` files at top level
  - only `12` of those files are tracked by git
  - the post-M11 authority reports named in this prompt are themselves part of the untracked report set
  - this is the largest recurring source of review-basis noise
- Intentional planning / historical docs:
  - `docs/Orchestrator_planning/` has `5` untracked files
  - `docs/archive/provider_orchestrator/` has `25` untracked files
  - these read as intentional retained planning/history material, not generated trash
- Recurring-noise summary:
  - mostly report artifacts and generated build artifacts
  - secondarily intentional local planning/tooling directories
  - not evidence of reopened M11 runtime drift

## 7. recommended housekeeping boundary

- The next housekeeping work can remain `docs/config/repo-hygiene only`
- Live repo reality does **not** force touching runtime source files
- `.gitignore` is part of the honest next scope
  - it is already modified
  - current ignore policy does not cover the main local/generated noise surfaces
- Narrowest realistic boundary:
  - `local artifact ignore-policy cleanup`
  - scope:
    - `.gitignore`
    - `.codex`
    - `src-rust/.codex`
    - `src-rust/target/`
  - objective:
    - reduce accidental broad staging risk from purely local/generated paths
- Separate follow-on boundary:
  - `report/planning path policy formalization`
  - scope:
    - `docs/archive/reports/`
    - `docs/Orchestrator_planning/`
    - `docs/archive/provider_orchestrator/`
  - objective:
    - decide what is intentionally preserved local evidence versus what should become tracked repo history versus what should be explicitly ignored
- Recommendation on one ticket vs split:
  - split
  - reason:
    - local/generated artifact cleanup is low-risk and mechanically clear
    - docs/report/planning paths are intentional content and need policy clarification, not blanket deletion or blind ignore rules

## 8. risk / non-regression findings

- No branch drift found
- No HEAD drift found
- No evidence that accepted M11 runtime behavior must be reopened
- The main risk is process/review risk, not runtime risk:
  - accidental broad staging
  - ambiguous review basis
  - repeated future tickets starting from a visibly dirty worktree
- Paths that should be preserved and documented rather than blindly cleaned:
  - `docs/archive/reports/`
  - `docs/Orchestrator_planning/`
  - `docs/archive/provider_orchestrator/`
  - `.codex`
  - `src-rust/.codex`
- Preservation note:
  - `src-rust/target/` should be treated as generated output, not preserved as reviewable repo content
  - the docs/planning/report directories appear intentional enough that deletion would be unjustified in a housekeeping pass without separate authority

## 9. recommended validation/review gate

- Do not use cargo build/test as the primary gate for this housekeeping lane
- Recommended blocking checks:
  - `git status --short --branch`
  - `git diff -- .gitignore`
  - `git status --short -- .gitignore .codex src-rust/.codex src-rust/target docs/archive/reports docs/Orchestrator_planning docs/archive/provider_orchestrator`
  - `git ls-files --others --exclude-standard -- .codex src-rust/.codex src-rust/target docs/archive/reports docs/Orchestrator_planning docs/archive/provider_orchestrator`
- Review basis recommendation:
  - use a path-scoped unstaged diff/status review
  - explicitly exclude runtime crates from the review basis

## 10. drift / blockers, if any

- Structural drift: `none`
- Authority drift: `none`
- Worktree drift from accepted baseline:
  - persistent housekeeping noise remains exactly in the areas repeatedly called out post-M11
- Honest-scope note:
  - a single small `.gitignore` ticket will reduce noise but will not by itself resolve the large untracked docs/report/planning surface
- Blockers:
  - no blocker prevents a docs/config-only housekeeping ticket
  - no blocker forces runtime-file changes

## 11. exact recommendation for next step

- Open the next ticket as a narrow housekeeping implementation pass for local/generated artifacts first
- Recommended next ticket boundary:
  - `.gitignore`
  - `.codex`
  - `src-rust/.codex`
  - `src-rust/target/`
- Keep that ticket limited to ignore-policy / patch-hygiene cleanup only
- Do not include runtime crates
- Do not include broad deletion of docs paths
- After that lands, open a separate docs/report-policy housekeeping ticket for:
  - `docs/archive/reports/`
  - `docs/Orchestrator_planning/`
  - `docs/archive/provider_orchestrator/`
- If only one immediate next ticket is allowed, the honest choice is the `.gitignore` / local-artifact ticket above, with an explicit note that report/planning noise remains as a documented follow-on rather than pretending the whole worktree-noise problem is solved
