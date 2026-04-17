# M11 Convergence Review

## timestamp UTC

`2026-04-15T04:28:25Z`

## branch / HEAD observed

- Branch: `feature/provider-resolution-seam`
- HEAD: `bfabfd5934b0afe801f2e7de9f15a7f6258c563b`
- Expected accepted latest HEAD: `bfabfd5934b0afe801f2e7de9f15a7f6258c563b` -- matched exactly
- Working tree: dirty / noisy from unrelated `.gitignore`, untracked docs artifacts, `.codex`, and `src-rust/target/`; this did not block the docs-only review, but the review basis must stay explicit
- Read-only compile probe: `cd src-rust && cargo check --workspace`
- Compile probe result: `PASS`

## authority inputs reviewed

- `AGENTS.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md`
- `docs/Current/TASK-M11-10_SPLIT_PLAN_AND_TEMPORARY_AUTHORITY_AMENDMENT.md`
- `docs/archive/reports/M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md`
- `docs/archive/reports/MPWO_REVISION_REPORT_20260414T003218Z.md`
- `docs/archive/reports/TASK-M11-08_AUTHORITY_RECONCILIATION_REPORT_20260415T002331Z.md`
- `docs/archive/reports/TASK-M11-08B_AUTHORITY_REPORT_20260415T005148Z.md`
- `docs/archive/reports/TASK-M11-10B_AUTHORITY_REPORT_20260415T030449Z.md`
- `docs/archive/reports/TASK-M11-10B_SPLIT_AUTHORITY_REPORT_20260415T031240Z.md`
- Closeout evidence:
  - `docs/archive/reports/TASK-M11-08R_CLOSEOUT_REPORT_20260415T004401Z.md`
  - `docs/archive/reports/TASK-M11-08B_CLOSEOUT_REPORT_20260415T011952Z.md`
  - `docs/archive/reports/TASK-M11-09_CLOSEOUT_REPORT_20260415T014220Z.md`
  - `docs/archive/reports/TASK-M11-10A_CLOSEOUT_REPORT_20260415T024141Z.md`
  - `docs/archive/reports/TASK-M11-10B1_CLOSEOUT_REPORT_20260415T034453Z.md`
  - `docs/archive/reports/TASK-M11-10B2_CLOSEOUT_REPORT_20260415T041344Z.md`
- Live repo surfaces inspected:
  - `src-rust/crates/query/src/lib.rs`
  - `src-rust/crates/query/src/session_budget.rs`
  - `src-rust/crates/query/src/agent_tool.rs`
  - `src-rust/crates/tools/src/team_tool.rs`
  - `src-rust/crates/tools/src/lib.rs`
  - `src-rust/crates/query/src/provider_resolution.rs`
  - `src-rust/crates/query/src/health_cache.rs`
  - `src-rust/crates/cli/src/main.rs`

Note:
- `docs/Current/MPWO_WORK_ORDER_PACK.md` currently contains the MPWO revision summary rather than a detailed executable work-order pack. For this review, the later accepted split authority reports and accepted closeouts were therefore the controlling interpretation for what M11 now means.

## accepted baseline reviewed

- `TASK-M11-01` closed at `dc772aa`
- `TASK-M11-02` closed at `fe21969`
- `TASK-M11-03` closed at `828b08e`
- `TASK-M11-04` closed at `eb26773`
- `TASK-M11-05` closed at `1472024`
- `TASK-M11-06` closed at `cf8201f`
- `TASK-M11-07` closed at `0942e4a`
- corrected `TASK-M11-08R` closed at `25518cac29d34353cb58c8811da1040a3da69247`
- `TASK-M11-08B` closed at `b4ad28ee9eb1e434f935fd2de70c4b402a5c74da`
- revised `TASK-M11-09` closed at `4ef9547dab51959f7b39c473f929b81f05ee1134`
- `TASK-M11-10A` closed at `ea046c52da82dfd9778f4065bd36b36e28d73c8a`
- `TASK-M11-10B1` closed at `3812df04ec87ce0e96f851da2d18ab38b24f0b99`
- `TASK-M11-10B2` closed at `bfabfd5934b0afe801f2e7de9f15a7f6258c563b`
- Live branch / HEAD matches the supplied latest accepted baseline exactly

## implemented M11 capability map

Coverage verdict:
- `yes`
- The accepted split chain (`08R`, `08B`, `09`, `10A`, `10B1`, `10B2`), on top of accepted `M11-01` through `M11-07`, does collectively reconstruct the intended runtime capability the revised M11 ladder was aiming for.
- The broader revised M11 ladder wanted D2 child execution to have first-class spawn-time control for provider/model behavior, shared session-budget inheritance, and child-local budget semantics distinct from `max_budget_usd`. Live repo reality now has those capabilities in place. The remaining downstream work is event expansion and integrated validation only.

Definitely implemented now:
- Earlier M11 capability substrate is present in live code:
  - `Capability`, `DEFAULT_REQUIRED_CAPABILITIES`, `model_supports_capability()`, and `provider_supports_capability()` are live in [provider_resolution.rs](/home/jordi/claurst/src-rust/crates/query/src/provider_resolution.rs:54)
  - same-domain fallback routing via `resolve_provider_with_fallback(...)` is live in [provider_resolution.rs](/home/jordi/claurst/src-rust/crates/query/src/provider_resolution.rs:302)
  - `HealthCache` is implemented and usable at [health_cache.rs](/home/jordi/claurst/src-rust/crates/query/src/health_cache.rs:12)
- Root session-budget wiring is live:
  - root `--budget-usd` creates a `SessionBudget` in [main.rs](/home/jordi/claurst/src-rust/crates/cli/src/main.rs:734)
  - `QueryConfig.session_budget` remains distinct from `max_budget_usd` in [lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:117)
- Shared session-budget propagation is live across all child/team query-loop paths:
  - `run_query_loop()` wraps execution with `with_registered_session_budget(...)` in [lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:694)
  - inherited shared budget lookup is live in [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:134)
  - child/team `QueryConfig` construction carries `session_budget` in [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:438) and [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:692)
- Session-budget cancellation is wired through the actual query-loop cancel path:
  - `SessionBudget::child_cancel_token()` is used for foreground, background, and team-runner child loops in [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:140), [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:470), and [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:701)
  - `run_query_loop()` records spend and calls `check_and_cancel()` in [lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:1169) and [lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:1439)
- Child `max_tokens` override is live across all required spawn paths:
  - schema/runtime fields exist in [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:178) and [team_tool.rs](/home/jordi/claurst/src-rust/crates/tools/src/team_tool.rs:43)
  - runtime use exists in [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:423) and [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:685)
- Child `allow_fallback` is live across all required spawn paths:
  - schema/runtime fields exist in [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:187) and [team_tool.rs](/home/jordi/claurst/src-rust/crates/tools/src/team_tool.rs:44)
  - child provider resolution uses the fallback-aware seam in [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:332) and [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:652)
- Child `budget_usd` now has the intended layered semantics:
  - query-owned layered seam exists via `SessionBudget::child_scope(...)` in [session_budget.rs](/home/jordi/claurst/src-rust/crates/query/src/session_budget.rs:36)
  - parent-preserving spend/cancel recursion exists in [session_budget.rs](/home/jordi/claurst/src-rust/crates/query/src/session_budget.rs:45) and [session_budget.rs](/home/jordi/claurst/src-rust/crates/query/src/session_budget.rs:52)
  - task-local stack plus shared-root registration avoid overwriting the parent shared budget in [session_budget.rs](/home/jordi/claurst/src-rust/crates/query/src/session_budget.rs:11), [session_budget.rs](/home/jordi/claurst/src-rust/crates/query/src/session_budget.rs:98), and [session_budget.rs](/home/jordi/claurst/src-rust/crates/query/src/session_budget.rs:131)
  - child/team carriage is live in [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:148), [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:416), [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:678), and [team_tool.rs](/home/jordi/claurst/src-rust/crates/tools/src/team_tool.rs:45)
- The final accepted split path therefore restores the broader revised M11 schema/runtime intent in split form:
  - `max_tokens` via `M11-09`
  - `allow_fallback` via `M11-10A`
  - `budget_usd` via `M11-10B1` plus `M11-10B2`

Still pending for downstream tickets only:
- `TASK-M11-11` event expansion
  - `QueryEvent` still only contains `Stream`, `ToolStart`, `ToolEnd`, `TurnComplete`, `Status`, `Error`, and `TokenWarning` in [lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:399)
  - no `WorkerProviderResolved`, `WorkerBudgetExceeded`, or `SessionBudgetExceeded` variants are present yet
- `TASK-M11-12` integrated D2 tests / validation intent
  - no dedicated M11-12 test landing is present in the reviewed live surfaces
  - the only required live validation performed in this review was `cargo check --workspace`, which passed

## intentionally deferred / excluded items

- `ToolContext.session_budget`
  - Classification: `intentionally out of M11 scope`
  - Basis: accepted authority reconciliation and later split authorities explicitly reject adding concrete query-owned budget fields to `ToolContext`; live [tools/src/lib.rs](/home/jordi/claurst/src-rust/crates/tools/src/lib.rs:216) still has only crate-neutral context plus provider/model registries
- `ToolContext.health_cache`
  - Classification: `intentionally out of M11 scope`
  - Basis: same crate-boundary correction as above; later accepted authorities continue to forbid it, and live [tools/src/lib.rs](/home/jordi/claurst/src-rust/crates/tools/src/lib.rs:216) contains no such field
- global/shared `HealthCache` plumbing
  - Classification: `intentionally deferred to later work`
  - Basis: fallback routing works today with runtime-local `HealthCache::new()` in [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:333) and [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:654); no accepted M11 closeout or authority assigns a shared/cache-carried variant as part of current convergence
- TeamCreate outer-cancellation redesign
  - Classification: `intentionally deferred to later work`
  - Basis: `team_tool.rs` still has its own TeamDelete outer-cancellation path, and accepted `08B` / `10B` authorities explicitly kept redesign of that tools-layer behavior out of scope for the corrected M11 path

None of the above read as accidentally missed.
None currently require new authority in order to proceed to `TASK-M11-11`.

## semantic drift assessment

Semantic drift verdict:
- `no` material runtime drift
- `yes` limited documentation-shape drift

Why the chain still converges cleanly:
- The earlier broad M10 / MPWO expectation that `ToolContext` would directly carry `SessionBudget` and `HealthCache` was formally corrected by accepted `TASK-M11-08` authority reconciliation, then preserved by accepted `08B`, `10B`, and `10B split` authority reports
- The earlier broad `M11-10` wording that treated `allow_fallback` and `budget_usd` as one ticket was formally split into:
  - `10A` child `allow_fallback`
  - `10B1` query-owned child-budget seam
  - `10B2` child/team budget carriage
- Live code matches the corrected split path rather than a contradictory hybrid:
  - `ToolContext` still has no concrete budget/cache fields
  - child fallback is carried in agent/team runtime surfaces
  - layered child budget semantics are query-owned first, then carried into agent/team runtime surfaces
- Live HEAD matches the latest accepted closeout commit exactly, so there is no evidence that later unreviewed code altered the accepted path

Where limited documentation drift remains:
- `docs/Current/MPWO_WORK_ORDER_PACK.md` in repo is a revision summary, not a detailed executable pack
- older broader wording still exists in the planning trail, but later accepted split authority supersedes it

Assessment:
- This is an authority-shape / wording drift issue, not a dropped runtime requirement
- The split tickets did not cause semantic loss in the implemented runtime behavior

## downstream readiness assessment

- `TASK-M11-11` readiness:
  - `yes`
  - The accepted runtime shape now supports proceeding to `TASK-M11-11` without hidden contradiction
  - Reason: the child/team runtime seams that `M11-11` will need to report on now exist and are stable:
    - fallback-aware child provider resolution
    - shared parent session-budget inheritance
    - child-local layered `budget_usd` semantics
    - distinct `max_tokens` override
  - Because `QueryEvent` has not already been widened, `M11-11` still has a clean, unclaimed delta

- `TASK-M11-12` readiness:
  - `yes`, but with wording clarification recommended
  - The validation intent still matches what actually landed if interpreted against the split path rather than the older unsplit `M11-10` expectation
  - `M11-12` should validate:
    - inherited parent shared session-budget accounting from `08B`
    - child `max_tokens` override from `09`
    - child `allow_fallback` from `10A`
    - layered child `budget_usd` behavior from `10B1` and `10B2`
    - continued distinction from `max_budget_usd`

- Wording / acceptance clarification needed:
  - `yes`, but not as a blocking authority rewrite
  - `M11-11` preflight should explicitly describe events against the landed layered `SessionBudget` model and avoid conflating child `budget_usd` with `max_budget_usd`
  - `M11-12` preflight should explicitly treat `10A`, `10B1`, and `10B2` as the accepted replacement for the old broader `M11-10` schema/runtime assumption

## recommendation for next step

- Proceed directly to `TASK-M11-11` preflight
- Another docs-only authority correction is not required first
- The `M11-11` preflight should explicitly cite the accepted split baseline:
  - `08R`
  - `08B`
  - `09`
  - `10A`
  - `10B1`
  - `10B2`
- The `M11-11` preflight should explicitly avoid relying on:
  - stale unsplit `M11-10` wording
  - any `ToolContext.session_budget`
  - any `ToolContext.health_cache`

## whether M11 still converges on intended end state

`yes`

## notes / risks

- The working tree is noisy, so future review bases must keep excluding unrelated docs noise, `.codex`, `.gitignore`, and `src-rust/target/`
- The main remaining precision risk is documentation phrasing, not missing runtime behavior
- `M11-11` and `M11-12` need to describe and validate the layered `SessionBudget` model that actually landed, not the older single-ticket `budget_usd` assumption
- No evidence from this review suggests reopening accepted tickets `08R`, `08B`, `09`, `10A`, `10B1`, or `10B2`
