# TASK-M8-11 Format Scope Reconciliation Report

## Ticket
`TASK-M8-11`

## Timestamp UTC
`20260413T001700Z`

## Branch
`feature/provider-resolution-seam`

## Repo State Observed
- `git branch --show-current`: `feature/provider-resolution-seam`
- `git diff --name-only`: empty
- `git diff --cached --name-only`: empty
- `git status --short`: no tracked unstaged or staged drift; untracked noise remains under `.codex/`, `docs/Current/`, `docs/Orchestrator_planning/`, `docs/archive/provider_orchestrator/`, `docs/archive/reports/`, and `src-rust/target/`
- `git log --oneline --decorate -n 30` confirms current `HEAD` is `1056eb3 TASK-M8-10 add agent tool provider seam coverage tests`
- Tracked state is still clean: yes

## Authority Reviewed
- `AGENTS.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md`
- `docs/Orchestrator_planning/IMPLEMENTATION_PLAN_MPWO.md` present and reviewed as secondary only

## Reconfirmed M8-11 Authority
- `TASK-M8-11` is a validation-only ticket: yes
- MPWO `TASK-M8-11` says to run `cd src-rust && cargo fmt --all -- --check` and fix formatting, then continue with build, test, and clippy
- MPWO `TASK-M8-11` also says: "Same rules as M7-07: fix M8-related failures, report unrelated ones."
- Inherited M7-07 rule set remains controlling for stop/escalate behavior:
  - do not fix unrelated files
  - if more than 3 files need fixes, escalate

## Prior Reports Reviewed
- `docs/archive/reports/TASK-M8-08_PREFLIGHT_REPORT_20260412T150803Z.md`
- `docs/archive/reports/TASK-M8-08_COMMIT_VERIFICATION_REPORT_20260412T152849Z.md`
- `docs/archive/reports/TASK-M8-09_COMMIT_VERIFICATION_REPORT_20260412T232152Z.md`
- `docs/archive/reports/TASK-M8-10_COMMIT_VERIFICATION_REPORT_20260412T235810Z.md`
- `docs/archive/reports/TASK-M8-11_PREFLIGHT_REPORT_20260413T000622Z.md`
- `docs/archive/reports/TASK-M8-11_EXECUTION_REPORT_20260413T001110Z.md`

## Formatting Command Re-Run
- Command re-run exactly: `cd src-rust && cargo fmt --all -- --check`
- Result: failed
- Exit status: `1`

## Exact Failing File List
- `src-rust/crates/query/src/agent_tool.rs`
- `src-rust/crates/query/src/compact.rs`
- `src-rust/crates/tools/src/team_tool.rs`

## Per-File Ownership / History Assessment
- `src-rust/crates/query/src/agent_tool.rs`
  - File-level M8 history on this branch: `b5249a3 TASK-M8-04`, `5d246b2 TASK-M8-05`, `1d531da TASK-M8-08`, `1056eb3 TASK-M8-10`
  - The exact formatting-failing hunks are blamed to M8 commits:
    - import and foreground-resolution/client block: `b5249a3 TASK-M8-04 wire foreground provider resolution through shared seam`
    - team-runner materialization call shape: `1d531da TASK-M8-08 wire team runner producer through shared provider seam`
  - Assessment: direct M8 surface, direct M8-origin formatting debt

- `src-rust/crates/tools/src/team_tool.rs`
  - File-level M8 history on this branch: `ea9da37 TASK-M8-06`, `5d472cf TASK-M8-07`, `2fd7732 TASK-M8-09`
  - The formatting-failing `AgentRunFn` type alias hunk is blamed to `ea9da37 TASK-M8-06 replace AgentRunFn positional seam with AgentRunParams`
  - `2fd7732 TASK-M8-09` is the latest M8 commit touching the file overall, but not the specific formatting hunk
  - Assessment: direct M8 surface, direct M8-origin formatting debt

- `src-rust/crates/query/src/compact.rs`
  - File-level branch history after the rustfmt baseline is:
    - `780cb72 style(rust): apply workspace rustfmt baseline`
    - `ced6005 Harden provider-aware compaction for post-M8-05 registry-backed runs`
  - Both formatting-failing hunks are blamed to `ced6005`
  - No later M8-06 through M8-10 commit touched `compact.rs`
  - MPWO Milestone 8 ticket authority does not name `compact.rs` as a numbered M8 target
  - However, the commit subject itself ties the file directly to post-M8-05 provider-seam hardening, not unrelated repo cleanup

## Specific Assessment For `compact.rs`
- The current formatting drift is attributable to the provider-aware compaction hardening baseline in `ced6005`: yes
- That baseline is part of this branch's accepted Milestone 8 reality: yes
  - Evidence: branch history places `ced6005` between `TASK-M8-05` and `TASK-M8-06`
  - Evidence: the earlier `TASK-M8-08` preflight explicitly identified `ced6005` as "later hardening baseline" present on this branch
- The file is not clearly part of an explicit numbered M8 ticket surface
- The file is also not unrelated pre-existing repo debt
- Best classification: `arguably M8-local`
- A formatting-only adjustment there would remain consistent with M8-11's "fix M8-related failures only" rule because:
  - the originating commit is provider-seam hardening on the active M8 branch
  - the failure is style-only, not functional
  - the fix would not require reopening non-M8 concepts or adjacent runtime design

## Policy Reconciliation
- The previous `BLOCKED-OUT-OF-SCOPE` call depended on the narrower assumption that only the already-listed M8 propagation files counted as recoverable M8 surface
- Direct MPWO authority for `TASK-M8-11` is broader than that: it allows fixing M8-related failures and does not require the failing file to appear in a numbered M8 ticket table if the failure is still part of Milestone 8 branch work
- On that stricter reading:
  - `agent_tool.rs` is M8-local formatting debt
  - `team_tool.rs` is M8-local formatting debt
  - `compact.rs` is branch-local, post-M8-05 provider-seam formatting debt and is better treated as arguable M8-local rather than unrelated

## Required Judgment
- Is `agent_tool.rs` formatting drift clearly M8-local? `yes`
- Is `team_tool.rs` formatting drift clearly M8-local? `yes`
- Is `compact.rs` formatting drift clearly M8-local, arguably M8-local, or out-of-scope? `arguably M8-local`
- Is the current M8-11 block correctly classified as `BLOCKED-OUT-OF-SCOPE`, or is it better classified as `RECONCILE-AND-PROCEED`? `RECONCILE-AND-PROCEED`
- If a fix pass is justified, can it stay within 3 files and formatting-only changes? `yes`

## Whether A Formatting-Only Recovery Pass Would Stay Within Policy
- File count remains exactly `3`
- M7-07 / M8-11 escalate threshold is `more than 3 files`
- A recovery pass confined to:
  - `src-rust/crates/query/src/agent_tool.rs`
  - `src-rust/crates/query/src/compact.rs`
  - `src-rust/crates/tools/src/team_tool.rs`
  would stay within the numeric threshold
- Because all three failures are branch-local to Milestone 8 work or its accepted post-M8-05 hardening baseline, a formatting-only pass on these three files would remain within policy

## Final Recommendation
`allow a narrow formatting-only recovery pass on the 3 reported files, then resume M8-11 validation`

## Hosted-Ollama Invariant Outcome
Hosted Ollama compatibility baseline preserved

## Verdict
`RECOVERABLE-WITHIN-SCOPE`
