# POST-M11-M12-AUTH-RECON Preflight Report

## 1. Ticket ID

`POST-M11-M12-AUTH-RECON`

## 2. Verdict

`PASS-WITH-NOTES`

## 3. Timestamp UTC

`20260416T060529Z`

## 4. Branch / HEAD / Worktree Summary

- Branch: `feature/provider-resolution-seam`
- HEAD: `2def737b4a723184db22b791f6527609db7abc8e`
- Accepted post-M11 closeout hash verified present: `b157924e130fdf71c09a3787b47dd5eb1f31d542`
- Worktree state: `dirty / noisy`
- Relevant tracked diff in likely target docs:
  - deleted `docs/Current/TASK-M11-10_SPLIT_PLAN_AND_TEMPORARY_AUTHORITY_AMENDMENT.md`
- Relevant clean likely-target docs:
  - `docs/Current/MPWO_WORK_ORDER_PACK.md`
  - `docs/Current/IMPLEMENTATION_PLAN_MPWO.md`
  - `docs/Current/M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md`
  - `docs/Current/D1_REVIEW_REPORT_20260413T233604Z.md`
- Broader worktree noise present:
  - modified `.gitignore`
  - untracked `docs/archive/reports/`
  - untracked `docs/archive/provider_orchestrator/`
  - untracked `docs/Current/HOSTED_OLLAMA_IMPLEMENTATION_PLAN.md`
  - untracked `docs/Current/MPWO_WORK_ORDER_PACK_pre_M10_revision.md`
  - untracked `docs/Current/RFC_PROVIDER_AWARE_WORKER_FABRIC_v3.1.md`
  - untracked `src-rust/target/`

## 5. Authority Reviewed

- Governing authority reviewed:
  - `AGENTS.md`
  - `docs/Current/MPWO_WORK_ORDER_PACK.md`
- Verified current-authority reality:
  - `docs/Current/MPWO_WORK_ORDER_PACK_AUTH.md` does not exist in this checkout
  - `docs/Current/MPWO_WORK_ORDER_PACK.md` is the only live current pack claiming active authority
  - `AGENTS.md` and `docs/Current/MPWO_WORK_ORDER_PACK.md` agree that the active authority path is `docs/Current/MPWO_WORK_ORDER_PACK.md`
- Conflicting current-authority artifact check:
  - no second live tracked authority pack exists
  - untracked `docs/Current/MPWO_WORK_ORDER_PACK_pre_M10_revision.md` exists and is authority-ambiguous because it contains actionable M12 wording while the active pack claims sole authority
- Verified files / symbols / commands used in this preflight:
  - files:
    - `AGENTS.md`
    - `docs/Current/MPWO_WORK_ORDER_PACK.md`
    - `docs/Current/IMPLEMENTATION_PLAN_MPWO.md`
    - `docs/Current/MPWO_WORK_ORDER_PACK_pre_M10_revision.md`
    - `docs/Current/M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md`
    - `docs/Current/D1_REVIEW_REPORT_20260413T233604Z.md`
    - `docs/archive/provider_orchestrator/d2_test_micro_patch_report.md`
    - `src-rust/crates/api/src/provider_types.rs`
    - `src-rust/crates/query/src/provider_resolution.rs`
    - `src-rust/crates/query/src/provider_resolution_tests.rs`
    - `src-rust/crates/query/src/health_cache.rs`
    - `src-rust/crates/query/src/session_budget.rs`
    - `src-rust/crates/query/src/agent_tool.rs`
    - `src-rust/crates/query/src/agent_tool_tests.rs`
    - `src-rust/crates/tools/src/team_tool.rs`
    - `src-rust/crates/query/src/lib.rs`
  - symbols:
    - `M12`
    - `surrogate`
    - `d2_test_micro_patch_report`
    - `TASK-M11-10_SPLIT_PLAN_AND_TEMPORARY_AUTHORITY_AMENDMENT`
    - `TrustDomain`
    - `resolve_provider_with_fallback`
    - `allow_fallback`
    - `HealthCache`
    - `SessionBudget`
    - `ProviderModelConflict`
    - `WorkerProviderResolved`
    - `WorkerBudgetExceeded`
    - `SessionBudgetExceeded`
  - commands:
    - `git branch --show-current`
    - `git rev-parse HEAD`
    - `git rev-parse --verify b157924e130fdf71c09a3787b47dd5eb1f31d542`
    - `git status --short`
    - `git diff --name-status -- ...`
    - `rg --files docs/Current`
    - `find docs/Current -maxdepth 1 -type f | sort`
    - repo-wide `rg` for the required search terms
    - targeted `sed` / `nl` inspection of docs and source files

## 6. Live `docs/Current/` Inventory and Classification

Live files under `docs/Current/` at preflight time:

- `docs/Current/MPWO_WORK_ORDER_PACK.md`
  - classification: `active controlling`
  - reason: explicitly declares itself the single active authority artifact
- `docs/Current/D1_REVIEW_REPORT_20260413T233604Z.md`
  - classification: `historical/contextual`
  - reason: top banner explicitly says historical evidence only / non-controlling
- `docs/Current/IMPLEMENTATION_PLAN_MPWO.md`
  - classification: `historical/contextual`
  - reason: top banner explicitly says historical planning context only / non-controlling
- `docs/Current/M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md`
  - classification: `historical/contextual`
  - reason: top banner explicitly says historical planning evidence only / non-controlling
- `docs/Current/HOSTED_OLLAMA_IMPLEMENTATION_PLAN.md`
  - classification: `stale/ambiguous`
  - reason: current live file under `docs/Current/`, untracked, not inventoried by the active pack, and not part of this authority lane
- `docs/Current/MPWO_WORK_ORDER_PACK_pre_M10_revision.md`
  - classification: `stale/ambiguous`
  - reason: old pack-shaped file, untracked, carries actionable M12 wording, conflicts with sole-pack authority if treated casually
- `docs/Current/RFC_PROVIDER_AWARE_WORKER_FABRIC_v3.1.md`
  - classification: `historical/contextual`
  - reason: design intent / RFC context, not current controlling authority, and not inventoried by the active pack

Pack references pointing to deleted, renamed, or demoted files:

- `docs/Current/MPWO_WORK_ORDER_PACK.md` still points to deleted `docs/Current/TASK-M11-10_SPLIT_PLAN_AND_TEMPORARY_AUTHORITY_AMENDMENT.md` as retained historical context
- `docs/Current/M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md` still references demoted / moved `docs/Orchestrator_planning/IMPLEMENTATION_PLAN_MPWO.md`

## 7. Live M12 Wording Findings

- The active pack does not currently define M12 in detail; it only preserves the accepted runtime chain:
  - `08R -> 08B -> 09 -> 10A -> 10B1 -> 10B2 -> 11 -> 12`
- The still-live contextual wording under `docs/Current/` is stale:
  - `docs/Current/IMPLEMENTATION_PLAN_MPWO.md` describes Milestone 12 as `Surrogate Test Retirement / Replacement`
  - `docs/Current/MPWO_WORK_ORDER_PACK_pre_M10_revision.md` breaks M12 into:
    - locate and audit surrogate test
    - remove/replace surrogate test
    - D2 coverage completeness verification
- Current stale assumptions still present in contextual docs:
  - a live surrogate D2 test exists
  - locating that live surrogate is an M12 prerequisite
  - removal/replacement of that live surrogate is still required
- Live repo reality does not match those assumptions

## 8. Surrogate Reality Findings

- No live tracked surrogate D2 test was found in source
- No current `crates/api/tests/` or `crates/query/tests/` trees were found for the old M12 search path
- Repo-wide search found the surrogate-specific identifiers only in archive/planning material, not in live tracked source:
  - `unknown_custom_providers_default_to_cloud_and_enter_cloud_candidate_scope`
  - `cloud_fallback_candidate_ids`
  - `TestTrustDomain`
- Historical surrogate path inspected:
  - `docs/archive/provider_orchestrator/d2_test_micro_patch_report.md`
- What that archive doc shows:
  - it documents a past test-only patch to `src-rust/crates/api/src/registry.rs`
  - it is not evidence of a current live tracked surrogate file in this checkout
- Important live-repo reality note:
  - `docs/archive/provider_orchestrator/d2_test_micro_patch_report.md` exists on disk but is not tracked by git in the current worktree
- Conclusion:
  - the surrogate exists only as archived documentation in current repo reality
  - there is no live tracked surrogate test to remove

## 9. Live D2 Coverage / Test-Surface Findings

Without widening into M12 execution, the current live D2-related surfaces that make the old surrogate framing stale are:

- `src-rust/crates/api/src/provider_types.rs`
  - live `TrustDomain`
  - hardcoded local/cloud classification exists in source now
- `src-rust/crates/query/src/provider_resolution.rs`
  - live `ProviderModelConflict`
  - live `resolve_provider_with_fallback`
  - live same-domain fallback boundary using `TrustDomain`
  - live capability-aware fallback model selection
- `src-rust/crates/query/src/provider_resolution_tests.rs`
  - fallback-disabled coverage
  - same-domain fallback coverage
  - cross-domain prohibition coverage
- `src-rust/crates/query/src/health_cache.rs`
  - live `HealthCache`
  - TTL and probe behavior covered in unit tests
- `src-rust/crates/query/src/session_budget.rs`
  - live `SessionBudget`
  - shared/root/child budget behavior covered in unit tests
- `src-rust/crates/query/src/agent_tool.rs`
  - live child-agent `allow_fallback`
  - live child `budget_usd`
  - live query-tool plumbing into provider resolution
- `src-rust/crates/query/src/agent_tool_tests.rs`
  - live same-domain fallback test at agent layer
  - live child-budget tests
  - live TeamCreate mixed-provider dispatch coverage
- `src-rust/crates/tools/src/team_tool.rs`
  - live TeamCreate carrier for provider/model/fallback/budget fields
- `src-rust/crates/query/src/lib.rs`
  - live query observability events:
    - `WorkerProviderResolved`
    - `WorkerBudgetExceeded`
    - `SessionBudgetExceeded`

Assessment:

- The repo now has real live D2-era seams and tests that did not exist when the surrogate/spec-proxy framing was written
- The old M12 wording is therefore stale because it still treats the surrogate as a live prerequisite target instead of archive-only historical context

## 10. Recommended Exact Execution Boundary

Narrowest valid execution boundary for `POST-M11-M12-AUTH-RECON`:

- `docs-only rewrite of the active pack only`

Why this is the narrowest correct boundary:

- the authority drift is concentrated in `docs/Current/MPWO_WORK_ORDER_PACK.md`
- no code or test change is required to reconcile the stale M12 assumption
- no contextual-doc banner sweep is strictly required to make the active authority safe
- the later pass should:
  - correct the live `docs/Current/` inventory inside the active pack
  - stop treating the deleted split-plan file as live retained context
  - explicitly state the surrogate is archive-only
  - recast M12 as a live D2 coverage-audit / closure-decision milestone instead of “find and delete live surrogate test”

## 11. Exact File Touch Set for Execution

Later execution pass should touch exactly:

- `docs/Current/MPWO_WORK_ORDER_PACK.md`

Later execution pass should not touch:

- `AGENTS.md`
- `docs/Current/IMPLEMENTATION_PLAN_MPWO.md`
- `docs/Current/M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md`
- `docs/Current/D1_REVIEW_REPORT_20260413T233604Z.md`
- `docs/Current/MPWO_WORK_ORDER_PACK_pre_M10_revision.md`
- `docs/Current/HOSTED_OLLAMA_IMPLEMENTATION_PLAN.md`
- `docs/Current/RFC_PROVIDER_AWARE_WORKER_FABRIC_v3.1.md`
- deleted `docs/Current/TASK-M11-10_SPLIT_PLAN_AND_TEMPORARY_AUTHORITY_AMENDMENT.md`
- `docs/archive/provider_orchestrator/d2_test_micro_patch_report.md`
- any `src-rust/**` file

## 12. Explicit Out-of-Scope List

This ticket should avoid:

- code changes
- runtime behavior changes
- test changes
- deferred architecture items:
  - shared/global `HealthCache` decision
  - `ToolContext` budget/cache carriage reconsideration
  - `TeamCreate` outer-cancellation redesign
- reopening accepted M11 tickets
- `AGENTS.md` edits
- archive cleanup/removal work

Nothing inspected in live repo reality makes any of those strictly unavoidable for this ticket.

## 13. Risks / Drift / Blockers

- Drift found:
  - active pack points to a deleted split-plan file as if it were live retained historical context
  - active pack inventory does not reflect the current live `docs/Current/` surface
  - still-live contextual docs under `docs/Current/` retain stale M12 surrogate-removal wording
  - untracked old/current docs in `docs/Current/` create authority-ambiguity risk if a later prompt ignores the active pack
- Worktree hygiene risk:
  - later closure review will need an explicit path-scoped review basis because the overall worktree is noisy
- Structural drift:
  - not severe enough to block this ticket
- Blockers:
  - none for a one-file docs-only authority reconciliation pass

## 14. Exact Wording Recommendations for the Later Execution Pass

The active pack should gain explicit M12 authority wording with the following substance:

- M12 is not to be executed from the legacy “locate and remove a live surrogate test” framing
- the historical surrogate artifact exists only as archive documentation at `docs/archive/provider_orchestrator/d2_test_micro_patch_report.md`
- no live tracked surrogate D2 test exists in source in the current checkout
- therefore there is no live surrogate test to remove in `POST-M11-M12-AUTH-RECON`
- M12 is recast as:
  - a live D2 coverage-audit milestone against the accepted post-M11 baseline, plus
  - authority acknowledgment that the surrogate is archive-only historical context
- the audit question for later M12 work should be:
  - do current live D2 seams/tests fully subsume the old surrogate intent, or
  - is there still a concrete uncovered live delta that needs a separate follow-on ticket
- if no remaining live delta exists, M12 may be declared satisfied by audit outcome
- if a real uncovered live delta exists, that delta should be executed as a separate follow-on implementation ticket, not silently folded into this authority-reconciliation ticket

The active pack should also state:

- the split-plan file is not a live current-authority artifact in this checkout
- future prompts must not treat deleted/demoted split-era docs as retained live authority context

## 15. Final Recommendation

- Proceed with `POST-M11-M12-AUTH-RECON` as a narrow docs-only ticket
- Keep the execution boundary to a one-file rewrite of `docs/Current/MPWO_WORK_ORDER_PACK.md`
- Do not reopen runtime architecture
- Do not touch code or tests
- Do not declare M12 obsolete in this ticket
- Recast M12 first, then let a separate later M12 audit/preflight decide whether any real live delta still remains
