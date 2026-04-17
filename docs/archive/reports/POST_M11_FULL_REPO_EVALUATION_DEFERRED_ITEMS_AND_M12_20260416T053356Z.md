# Post-M11 Full Repo Evaluation: Deferred Items and M12

## 1. Executive verdict

The repo is no longer sitting exactly on the accepted post-M11 runtime closeout hash `b157924e130fdf71c09a3787b47dd5eb1f31d542`; live `HEAD` is `2def737b4a723184db22b791f6527609db7abc8e` on `feature/provider-resolution-seam`, with accepted post-M11 cleanup commits layered on top. On the seams that matter for this evaluation, live code still matches the accepted post-M11 runtime position: `SessionBudget` inheritance is query-owned and task-local, fallback still uses runtime-local `HealthCache::new()`, `ToolContext` still carries only provider/model registries, and TeamCreate outer cancellation is still a separate tools-layer token path.

Proven by current code:
- `ToolContext` has `provider_registry` and `model_registry` only.
- `agent_tool.rs` uses `session_budget_for_session(...)`, `SessionBudget::child_scope(...)`, and runtime-local `HealthCache::new()`.
- `team_tool.rs` still creates outer `CancellationToken::new()` values for TeamDelete.
- `QueryEvent` already includes `WorkerProviderResolved`, `WorkerBudgetExceeded`, and `SessionBudgetExceeded`.

Inferred from accepted reports:
- The accepted post-M11 runtime baseline is `08R -> 08B -> 09 -> 10A -> 10B1 -> 10B2 -> 11 -> 12`.
- The post-M11 cleanup ladder has advanced current `HEAD` through accepted authority consolidation, formatter cleanup, lint cleanup tranches, API cleanup, and query test reorganization.
- The three deferred items were intentionally left out of accepted M11 and later classified as backlog architecture, not as open defects.

Architectural judgment:
- The three deferred items still look like valid deferrals, not hidden unfinished M11 work.
- M12 should not wait for those three items.
- M12 as currently described in legacy planning is not authority-ready, because the supposed live surrogate test exists only as archived documentation, not as tracked source.
- The lowest-rework path is: reconcile M12 authority first, then do any real M12 audit/cleanup on the current accepted D2 baseline, and leave the three deferred architecture items parked until a later milestone proves an actual need.

## 2. docs/Current inventory

Live inventory was enumerated first and all seven live files were read.

| File | Classification | Controlling or contextual | Unique value for this evaluation |
|---|---|---|---|
| `docs/Current/MPWO_WORK_ORDER_PACK.md` | controlling authority | controlling | Current active authority summary; states M11 complete at `b157924...` and gives the accepted split runtime chain. |
| `docs/Current/D1_REVIEW_REPORT_20260413T233604Z.md` | historical/non-controlling | contextual only | Confirms the accepted D1 seam baseline that M11 and the deferred items still build on. |
| `docs/Current/M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md` | planning/reference | contextual only | Repo-grounded D2 plan; explains the original M11 extension points and the original M12 meaning. |
| `docs/Current/IMPLEMENTATION_PLAN_MPWO.md` | planning/reference | contextual only | Original milestone ladder and the earliest explicit description of the surrogate-test M12 concept. |
| `docs/Current/HOSTED_OLLAMA_IMPLEMENTATION_PLAN.md` | planning/reference | contextual only | Reinforces the hosted-Ollama invariant and why provider-materialization details are intentionally localized. |
| `docs/Current/MPWO_WORK_ORDER_PACK_pre_M10_revision.md` | historical/non-controlling | contextual only | Detailed old ticket bodies for M7-M12; useful for historical M12 wording, but superseded and stale as authority. |
| `docs/Current/RFC_PROVIDER_AWARE_WORKER_FABRIC_v3.1.md` | planning/reference | contextual only | Highest-detail design intent for D1/D2, including trust-domain, fallback, budget, and cancellation goals. |

Inventory interpretation:
- Only `MPWO_WORK_ORDER_PACK.md` is controlling.
- Every other live file is contextual only.
- Several contextual files are useful, but they describe older planning assumptions that current accepted split authority later narrowed or rejected.

Live authority drift that matters:
- `docs/Current/TASK-M11-10_SPLIT_PLAN_AND_TEMPORARY_AUTHORITY_AMENDMENT.md` is referenced by the active pack as a historical/non-controlling file, but it is currently a tracked deletion in the worktree, so the active pack points to a file that is not live.
- The accepted `POST-M11-01` docs consolidation closeout described a five-file `docs/Current/` set. Live `docs/Current/` now has seven files because three untracked contextual files were added and the demoted split-plan file is deleted.
- M12 wording in the live contextual docs still assumes a live surrogate test; live repo reality does not.

## 3. Authority and context reviewed

Repo-level authority:
- `AGENTS.md`

All live `docs/Current/` files:
- `D1_REVIEW_REPORT_20260413T233604Z.md`
- `HOSTED_OLLAMA_IMPLEMENTATION_PLAN.md`
- `IMPLEMENTATION_PLAN_MPWO.md`
- `M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md`
- `MPWO_WORK_ORDER_PACK.md`
- `MPWO_WORK_ORDER_PACK_pre_M10_revision.md`
- `RFC_PROVIDER_AWARE_WORKER_FABRIC_v3.1.md`

Accepted archive reports read directly for this evaluation:
- `TASK-M11-08_AUTHORITY_RECONCILIATION_REPORT_20260415T002331Z.md`
- `TASK-M11-10A_EXECUTION_REPORT_20260415T023511Z.md`
- `TASK-M11-10B_SPLIT_AUTHORITY_REPORT_20260415T031240Z.md`
- `TASK-M11-11_CLOSEOUT_REPORT_20260415T053856Z.md`
- `TASK-M11-12_GATE_CLARIFICATION_REPORT_20260415T060015Z.md`
- `TASK-M11-12_CLOSEOUT_REPORT_20260415T065244Z.md`
- `TASK-M11_D2_RECON_REPORT_20260414T012649Z.md`
- `M11_CONVERGENCE_REVIEW_20260415T042825Z.md`
- `POST_M11_REPO_ASSESSMENT_20260415T071321Z.md`
- `POST_M11_01_CLOSEOUT_REPORT_20260415T074353Z.md`
- `POST_M11_02_CLOSEOUT_REPORT_20260415T082535Z.md`
- `POST_M11_03_CLOSEOUT_REPORT_20260415T090527Z.md`
- `POST_M11_08A_CLOSEOUT_REPORT_20260416T001346Z.md`

Live code surfaces inspected directly:
- `src-rust/crates/query/src/health_cache.rs`
- `src-rust/crates/query/src/agent_tool.rs`
- `src-rust/crates/query/src/provider_resolution.rs`
- `src-rust/crates/query/src/session_budget.rs`
- `src-rust/crates/query/src/lib.rs`
- `src-rust/crates/tools/src/lib.rs`
- `src-rust/crates/tools/src/team_tool.rs`
- `src-rust/crates/cli/src/main.rs`
- `src-rust/crates/api/src/model_registry.rs`
- `src-rust/crates/api/src/provider_types.rs`

Additional live reality checks:
- branch / `HEAD`
- worktree status
- `docs/Current/` live inventory
- diff from accepted runtime closeout hash `b157924...` to live `HEAD`
- repo-wide search for deferred-item seams and M12 surrogate artifacts

## 4. Live repo state checked

Branch / `HEAD` / worktree:
- Branch: `feature/provider-resolution-seam`
- `HEAD`: `2def737b4a723184db22b791f6527609db7abc8e`
- Worktree is noisy:
  - modified `.gitignore`
  - deleted `docs/Current/TASK-M11-10_SPLIT_PLAN_AND_TEMPORARY_AUTHORITY_AMENDMENT.md`
  - untracked `docs/Current/HOSTED_OLLAMA_IMPLEMENTATION_PLAN.md`
  - untracked `docs/Current/MPWO_WORK_ORDER_PACK_pre_M10_revision.md`
  - untracked `docs/Current/RFC_PROVIDER_AWARE_WORKER_FABRIC_v3.1.md`
  - many untracked archive reports and `src-rust/target/`

Current accepted repo state:
- The accepted D2 runtime closeout hash remains `b157924e130fdf71c09a3787b47dd5eb1f31d542`.
- Live `HEAD` has moved 12 commits beyond that hash.
- Those later commits are accepted post-M11 cleanup/documentation/test-organization work rather than a reopened D2 feature ladder.
- I directly verified accepted closeouts at least through `POST-M11-08A`, and current `HEAD` matches the accepted `POST-M11-08A` commit.

Does live repo reality still match the accepted post-M11 baseline?
- Runtime semantics on the deferred-item seams: yes.
- `docs/Current` authority shape: no.

What is proven live now:
- `QueryConfig` carries `session_budget`.
- Root CLI creates a root `SessionBudget` from `--budget-usd`.
- `run_query_loop()` wraps execution in `with_registered_session_budget(...)`.
- Child/team budget inheritance is implemented through `session_budget_for_session(...)` and `SessionBudget::child_scope(...)`, not through `ToolContext`.
- `ToolContext` still has no `session_budget` or `health_cache`.
- Fallback wiring already exists and depends on `HealthCache`, but `agent_tool.rs` instantiates runtime-local caches.
- TeamCreate still creates outer cancellation tokens in `claurst-tools` and races them against `run_agent(...)`.
- `QueryEvent` expansion is already live.

Authority / documentation / structure drift that matters:
- The active pack still points to a deleted split-plan file.
- The active pack is intentionally terse; live `docs/Current/` now mixes active authority with untracked planning/history docs, which raises prompt-discipline risk.
- M12 still appears in active/context docs as a surrogate-test-retirement milestone, but live repo reality shows the surrogate is archive-only.
- `M11_CONVERGENCE_REVIEW_20260415T042825Z.md` is no longer fully accurate for live `HEAD` because later accepted cleanup and test-organization work advanced the repo.

Are the three deferred items still deliberate deferred architecture?
- Yes.
- Current code and accepted reports show accepted alternative seams, not missing implementations:
  - query-owned session-budget registry instead of `ToolContext.session_budget`
  - runtime-local `HealthCache` instead of a shared/global cache
  - unchanged TeamCreate outer cancellation instead of a redesign

## 5. Deferred item 1 analysis

### `POST-M11-ARCH-01` — shared/global `HealthCache` decision

What exact problem or capability would it solve?
- Reuse provider-health probe results across more than one child/team resolution path.
- Avoid repeated `HealthCache::new()` creation and repeated provider health probes when many workers spawn in the same session or process.
- Provide one consistent health-view owner if later work wants root and child fallback to share probe state.

What is proven by current code?
- `resolve_provider_with_fallback(...)` requires `&HealthCache`.
- `HealthCache` exists, is TTL-based, and has unit tests.
- `agent_tool.rs` creates a fresh `HealthCache::new()` in the foreground child path and another fresh `HealthCache::new()` in the injected team-runner path.
- No global/shared `HealthCache` owner exists in `query`, `cli`, or `tools`.
- `ToolContext` does not carry `health_cache`.

What is inferred from accepted reports?
- `TASK-M11-10A` intentionally used runtime-local caches and explicitly avoided `ToolContext.health_cache`.
- `POST_M11_REPO_ASSESSMENT_20260415T071321Z.md` classified global/shared `HealthCache` plumbing as safe backlog, not an open gap.

What remains architectural judgment?
- A shared/global cache would be cleaner if fallback becomes hot, team fan-out grows, or provider health probe latency becomes user-visible.
- Today that is a performance/ownership choice, not a correctness fix.

Files and crate boundaries likely involved:
- `src-rust/crates/query/src/agent_tool.rs`
- `src-rust/crates/query/src/health_cache.rs`
- `src-rust/crates/query/src/provider_resolution.rs`
- Possibly `src-rust/crates/cli/src/main.rs` or `src-rust/crates/query/src/lib.rs` if a root/session owner is introduced
- Possibly `src-rust/crates/tools/src/lib.rs` only if this is incorrectly folded into the rejected `ToolContext` carriage idea

Prerequisites that would need to be settled first:
- Who owns the shared cache: root query loop, per-session query runtime, or process-global query state.
- Whether the cache must be shared only inside `claurst-query`, or also across the tool/query crate seam.
- Whether root-session provider resolution is supposed to use the same cache, or whether sharing is child-only.

Accepted seams at risk of reopening:
- `TASK-M11-10A` child fallback wiring
- the accepted rejection of `ToolContext.health_cache`
- the hosted-Ollama neutrality rule if provider-resolution plumbing is widened carelessly

Scope assessment:
- Medium, but architecture-heavy enough that it should not be treated as a cleanup ticket.

Recommendation:
- Stay deferred.
- Classify as safe backlog unless live evidence later shows repeated health probes are a real latency/regression problem.

## 6. Deferred item 2 analysis

### `POST-M11-ARCH-02` — `ToolContext` budget/cache carriage reconsideration

What exact problem or capability would it solve?
- Replace the current query-owned task-local/session registry seam with explicit carrier fields or another explicit handoff mechanism.
- Potentially make child/team budget and cache inheritance more direct and easier to reason about from call signatures alone.
- Potentially create one unified carriage story for both `SessionBudget` and `HealthCache`.

What is proven by current code?
- `ToolContext` lives in `claurst-tools` and contains only `provider_registry` and `model_registry` beyond the generic tool context fields.
- `SessionBudget` and `HealthCache` live in `claurst-query`.
- Budget inheritance currently works without `ToolContext` carriage:
  - `run_query_loop()` registers the active budget via `with_registered_session_budget(...)`
  - child paths recover it with `session_budget_for_session(...)`
  - child-local `budget_usd` becomes `SessionBudget::child_scope(...)`
- `HealthCache` is not carried at all; runtime-local instantiation is used instead.

What is inferred from accepted reports?
- `TASK-M11-08_AUTHORITY_RECONCILIATION_REPORT_20260415T002331Z.md` declared the original `ToolContext.session_budget` / `ToolContext.health_cache` assumption invalid as written because it implied a reverse dependency from `claurst-tools` into `claurst-query`.
- `TASK-M11-10B` split authority kept those fields forbidden while still delivering layered child budget semantics by a query-owned seam.
- `POST_M11_REPO_ASSESSMENT_20260415T071321Z.md` treated `ToolContext` carriage reconsideration as deferred architecture, not missing runtime.

What remains architectural judgment?
- An explicit carrier could be cleaner than the current task-local/session registry seam.
- But reopening this now would mostly trade a working accepted implementation for a broader crate-boundary refactor.
- It is not needed to execute M12 or to preserve current D2 behavior.

Files and crate boundaries likely involved:
- `src-rust/crates/tools/src/lib.rs`
- `src-rust/crates/query/src/session_budget.rs`
- `src-rust/crates/query/src/agent_tool.rs`
- `src-rust/crates/tools/src/team_tool.rs`
- `src-rust/crates/cli/src/main.rs`
- Possibly moving types into `claurst-core` or another neutral crate if explicit carriage is ever chosen

Prerequisites that would need to be settled first:
- Which crate should own any shared neutral handle or carrier type.
- Whether `SessionBudget` and `HealthCache` must move, be wrapped, or be abstracted behind a trait/object-safe handle.
- Whether the current query-owned task-local registry has a proven deficiency beyond architectural aesthetics.

Accepted seams at risk of reopening:
- `08B` inherited parent accounting
- `10B1` layered child-budget seam
- `10B2` child/team `budget_usd` carriage
- the accepted no-`ToolContext.session_budget` / no-`ToolContext.health_cache` split path

Scope assessment:
- Architecture-heavy.

Recommendation:
- Keep blocked until a later milestone has a concrete feature requirement that the current query-owned seam cannot satisfy.
- For current planning purposes, treat it as safe backlog.

## 7. Deferred item 3 analysis

### `POST-M11-ARCH-03` — TeamCreate outer-cancellation redesign

What exact problem or capability would it solve?
- Unify TeamDelete cancellation and query-loop/session-budget cancellation into one coherent cancellation tree.
- Make it explicit how outer tools-layer cancellation, inner query-loop cancellation, and child/team budget cancellation interact.
- Potentially improve cancellation observability and reduce ambiguity around which token actually stopped a worker.

What is proven by current code?
- `team_tool.rs` still creates one outer `CancellationToken::new()` per agent and stores those in `ACTIVE_TEAMS`.
- TeamDelete cancels only those outer tokens.
- TeamCreate waits on `tokio::select! { out = run_agent(...), _ = cancel.cancelled() => ... }`.
- The query-side team runner separately constructs a query-loop cancel token from the active `SessionBudget` with `child_cancel_token()`.
- Outer and inner cancellation layers are separate.

What is inferred from accepted reports?
- `TASK-M11-08B`, `TASK-M11-09`, `TASK-M11-10A`, `TASK-M11-10B1`, and `TASK-M11-10B2` all explicitly kept TeamCreate outer-cancellation redesign out of scope.
- `TASK-M11-10B_SPLIT_AUTHORITY_REPORT_20260415T031240Z.md` says not to redesign TeamCreate outer cancellation unless later evidence proves it unavoidable.
- `POST_M11_REPO_ASSESSMENT_20260415T071321Z.md` classified it as safe backlog.

What remains architectural judgment?
- The current two-layer approach is inelegant, but live evidence does not prove it is broken.
- A redesign is only justified if later work needs one authoritative cancellation model, stronger guarantees around long-running tool interruption, or clearer event semantics than the current split layer provides.

Files and crate boundaries likely involved:
- `src-rust/crates/tools/src/team_tool.rs`
- `src-rust/crates/query/src/agent_tool.rs`
- `src-rust/crates/query/src/session_budget.rs`
- Possibly `src-rust/crates/query/src/lib.rs` if query-loop cancellation/event emission changes

Prerequisites that would need to be settled first:
- Which layer owns authoritative cancellation: tools-layer TeamDelete, query-owned `SessionBudget`, or a new neutral handle.
- Whether the redesign must preserve the current `claurst-tools` independence from `claurst-query`.
- Whether background/tool subprocess cancellation semantics also need to be in scope.

Accepted seams at risk of reopening:
- TeamDelete behavior
- the injected `AgentRunParams` / `register_agent_runner()` seam
- accepted session-budget child token behavior

Scope assessment:
- Architecture-heavy.

Recommendation:
- Remain deferred.
- It is a watchpoint, but not a current blocker.

## 8. M12 interaction assessment

What M12 currently appears to involve, based on the reviewed material:
- In the active/current planning trail, M12 still reads as “surrogate test retirement / replacement”.
- The older detailed work packs say M12 should:
  - locate and audit a surrogate test,
  - remove or replace it,
  - verify D2 coverage completeness.

What is proven by current repo reality:
- The supposed surrogate D2 test is not in tracked source.
- The surrogate exists only as archived documentation at `docs/archive/provider_orchestrator/d2_test_micro_patch_report.md`.
- `TASK-M11_D2_RECON_REPORT_20260414T012649Z.md` already documented that the surrogate was archive-only, not live code.
- Live D2 production tests now exist across:
  - `src-rust/crates/api/src/provider_types.rs`
  - `src-rust/crates/query/src/health_cache.rs`
  - `src-rust/crates/query/src/session_budget.rs`
  - `src-rust/crates/query/src/provider_resolution_tests.rs`
  - `src-rust/crates/query/src/agent_tool_tests.rs`
  - `src-rust/crates/query/src/lib.rs`

What is inferred from accepted reports:
- `TASK-M11-12` already closed with crate-level D2 validation on the accepted split baseline.
- `POST_M11_REPO_ASSESSMENT_20260415T071321Z.md` then classified the three architecture items as backlog and did not identify M12 as blocked on them.

Technical readiness of M12:
- As originally written: no.
- There is no live surrogate test to remove, so the original “find live surrogate, then delete/replace it” description is stale.
- As a recast “coverage audit plus archive-only surrogate confirmation” milestone: yes.

Authority readiness of M12:
- No.
- The current active pack still inherits stale M12 wording from older planning.
- The narrowest authority gap is not in code; it is in the active milestone description.

Would doing the three deferred items first simplify M12?
- No.
- It would change the very seams M12 should be auditing and would make current D2 evidence stale.

Would doing the three deferred items first make M12 evidence stale?
- Yes.
- Especially:
  - a shared/global `HealthCache` would change fallback runtime ownership and probe reuse assumptions,
  - `ToolContext` carriage changes would change how child/team budget inheritance is justified,
  - TeamCreate outer-cancellation redesign would change cancellation semantics and likely test surfaces.

Would doing M12 first reduce architectural uncertainty?
- Only after M12 is authority-reconciled.
- A recast M12 would tell you whether the current accepted baseline still has any real coverage gap before reopening architecture.

Are current M12 assumptions likely to collide with the three deferred items?
- Yes.
- Current M12 planning assumes a stable D2 baseline plus a live surrogate artifact.
- The deferred items would change the D2 baseline.
- Live repo reality shows the surrogate is archive-only, so current M12 wording already needs reconciliation before any execution prompt is safe.

Narrowest doc/reconciliation step needed before any M12 execution prompt:
- Update `docs/Current/MPWO_WORK_ORDER_PACK.md` so M12 says:
  - the historical surrogate exists only in `docs/archive/provider_orchestrator/d2_test_micro_patch_report.md`,
  - there is no live tracked surrogate test to remove,
  - M12 is now a live D2 coverage-audit / archive-cleanup milestone, or is explicitly declared obsolete if the audit finds no remaining live delta.
- In the same step, reconcile the active pack’s reference to the now-deleted split-plan file.

Verdict on M12 interaction:
- M12 should not wait for `POST-M11-ARCH-01/02/03`.
- But M12 should not be executed from current authority text without one narrow reconciliation step first.

## 9. Sequence comparison

### Sequence A

Do all three deferred architecture items before any M12 work.

Benefits:
- You settle long-term architecture before another validation/cleanup milestone.
- If a later milestone truly needs shared cache ownership or unified cancellation, you would not need to revisit current designs twice.

Likely rework risk:
- High.
- None of the three items is proven necessary today.
- Doing them first would rewrite accepted seams before current D2 evidence is formally closed out by a recast M12.

Regression risk:
- High relative to the other options.
- All three touch sensitive runtime seams:
  - fallback
  - budget inheritance
  - team cancellation

Process / authority risk:
- High.
- The accepted split path explicitly deferred these.
- Starting here would reopen rejected assumptions, especially around `ToolContext` carriage.

Evidence stability:
- Poor.
- M12 would then be auditing a moving target.

Architectural lock-in risk:
- High.
- You would be forcing architecture decisions before current repo reality proves they are needed.

Verdict:
- Not recommended.

### Sequence B

Do M12 work before the three deferred architecture items.

Benefits:
- Preserves the accepted split D2 baseline long enough to audit it.
- Avoids reopening architecture first.
- Keeps current fallback/budget/cancellation evidence stable while M12 is assessed.

Likely rework risk:
- Low to medium.
- Low if M12 is first authority-reconciled.
- Medium if someone executes old M12 wording and wastes effort chasing a nonexistent live surrogate.

Regression risk:
- Low if M12 stays audit/doc/test-only.

Process / authority risk:
- Medium.
- Directionally correct, but current M12 wording is stale.

Evidence stability:
- Good.

Architectural lock-in risk:
- Low.

Verdict:
- Better than Sequence A, but incomplete unless preceded by a small authority-reconciliation step.

### Sequence C

Mixed order / selective interleave.

Recommended mixed sequence:
1. Reconcile active authority for M12 and live `docs/Current` drift.
2. Run a recast M12 preflight / coverage audit on the current accepted D2 baseline.
3. Execute any truly necessary M12 delta only if the audit finds one.
4. Leave `POST-M11-ARCH-01/02/03` deferred unless that audit surfaces a concrete new need.

Benefits:
- Lowest rework.
- Fixes the actual present blocker, which is authority drift, not runtime absence.
- Preserves current evidence while still allowing M12 to move.
- Avoids speculative architecture.

Likely rework risk:
- Lowest of the three options.

Regression risk:
- Lowest.

Process / authority risk:
- Lowest.
- It addresses the stale M12 wording before execution.

Evidence stability:
- Best.

Architectural lock-in risk:
- Low.

Verdict:
- Best sequence.

## 10. Recommended ticket ladder

Recommended next-step ladder:

1. `POST-M11-M12-AUTH-RECON` — docs/reconciliation
   - Update `docs/Current/MPWO_WORK_ORDER_PACK.md` to reflect live `docs/Current` inventory, the deleted split-plan file, and the fact that the historical D2 surrogate is archive-only.
   - Recast M12 around live D2 coverage audit or explicitly declare it obsolete if no live delta remains.

2. `M12-RECUT-PREFLIGHT` — architecture preflight
   - Audit live D2 coverage against the current accepted split baseline and the archive-only surrogate intent.
   - Decide whether any code/test/doc delta remains for M12 at all.

3. `M12-EXECUTION` — implementation ticket
   - Only if step 2 finds a real remaining delta.
   - Expected scope should be test/doc/archive cleanup, not runtime architecture reopening.

4. `POST-M11-ARCH-01` — safe backlog
   - Shared/global `HealthCache` decision.

5. `POST-M11-ARCH-02` — safe backlog
   - `ToolContext` budget/cache carriage reconsideration.

6. `POST-M11-ARCH-03` — safe backlog
   - TeamCreate outer-cancellation redesign.

Deferred-item classification:
- `POST-M11-ARCH-01`: safe backlog
- `POST-M11-ARCH-02`: safe backlog
- `POST-M11-ARCH-03`: safe backlog

## 11. Risks / watchpoints

Sequence-sensitive risks:
- Doing architecture first will stale current D2 evidence and turn M12 into a moving target.
- Doing M12 from current stale wording risks chasing a nonexistent live surrogate artifact.

Likely reopen points:
- `agent_tool.rs` child provider-resolution block
- `session_budget.rs` task-local/shared-budget registry seam
- `team_tool.rs` outer cancellation and TeamDelete path
- any attempt to push query-owned runtime types back into `ToolContext`

Seams most at risk of rework:
- `src-rust/crates/query/src/agent_tool.rs`
- `src-rust/crates/query/src/session_budget.rs`
- `src-rust/crates/tools/src/team_tool.rs`
- `src-rust/crates/tools/src/lib.rs`
- `src-rust/crates/query/src/provider_resolution.rs`

Places where current docs disagree with code reality:
- Active pack references a deleted split-plan file.
- Active/context M12 wording still assumes a live surrogate-test target.
- `M11_CONVERGENCE_REVIEW_20260415T042825Z.md` is stale for live `HEAD` because later accepted cleanup and test-organization changes landed afterward.
- `docs/Current/` now contains additional untracked contextual files that were not part of the accepted `POST-M11-01` authority-consolidation shape.

Places where M12 is under-specified:
- It does not name the live current D2 test surfaces.
- It does not acknowledge that the surrogate lives only in archive docs.
- It does not state whether M12 is now a docs/audit milestone, a test-gap milestone, or already effectively satisfied.

## 12. Final yes/no answers

- Should M12 wait for the 3 deferred architecture items? `no`
- Should the 3 deferred architecture items wait until a later milestone instead? `yes`
- Is there enough authority/context now to start the best next ticket immediately? `yes`, but only for an authority-reconciliation / M12-recast ticket, not for architecture implementation
- What is the single best next move? `Write and accept one narrow docs/reconciliation ticket that updates MPWO current authority to reflect live docs/Current reality and recasts M12 around archive-only surrogate confirmation plus live D2 coverage audit.`
