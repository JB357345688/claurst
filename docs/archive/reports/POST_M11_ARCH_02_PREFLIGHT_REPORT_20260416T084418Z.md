# 1. Ticket ID

`POST-M11-ARCH-02`

# 2. Verdict

`REMAIN-DEFERRED`

# 3. Timestamp UTC

`20260416T084418Z`

# 4. Branch / HEAD / worktree summary

- Current branch: `feature/provider-resolution-seam`
- Current `HEAD`: `038f3c20e01a96eec6397d506b477a461166f762`
- `HEAD` exactly matches the user-provided accepted `POST-M11-ARCH-01A` closeout commit.
- Worktree is not clean:
  - tracked modified: `6`
  - tracked deleted: `1`
  - tracked added: `0`
  - untracked: `296`
- Representative worktree noise:
  - modified: `.gitignore`, `src-rust/crates/api/src/providers/google.rs`, `src-rust/crates/core/src/effort.rs`, `src-rust/crates/core/src/lib.rs`, `src-rust/crates/core/src/remote_settings.rs`, `src-rust/crates/core/src/system_prompt.rs`
  - deleted: `docs/Current/TASK-M11-10_SPLIT_PLAN_AND_TEMPORARY_AUTHORITY_AMENDMENT.md`
  - untracked: many archived reports, `docs/Orchestrator_planning/`, `.codex`, `src-rust/target/`
- Scope attribution note:
  - the worktree is noisy enough to confuse later review/closure if not isolated
  - the files inspected for this ticket are clean in the current worktree

# 5. Authority reviewed

- Repo authority file reviewed: `AGENTS.md`
  - confirms one-ticket-only execution, preflight-first flow, verification-before-editing, and report placement under `docs/archive/reports/` ([AGENTS.md](/home/jordi/claurst/AGENTS.md:3), [AGENTS.md](/home/jordi/claurst/AGENTS.md:5), [AGENTS.md](/home/jordi/claurst/AGENTS.md:63))
- Live current-authority file reviewed: `docs/Current/MPWO_WORK_ORDER_PACK.md`
  - states it is the single active authority artifact in `docs/Current/` ([docs/Current/MPWO_WORK_ORDER_PACK.md](/home/jordi/claurst/docs/Current/MPWO_WORK_ORDER_PACK.md:5), [docs/Current/MPWO_WORK_ORDER_PACK.md](/home/jordi/claurst/docs/Current/MPWO_WORK_ORDER_PACK.md:22), [docs/Current/MPWO_WORK_ORDER_PACK.md](/home/jordi/claurst/docs/Current/MPWO_WORK_ORDER_PACK.md:30))
  - explicitly says the deleted split-era amendment doc is not live authority ([docs/Current/MPWO_WORK_ORDER_PACK.md](/home/jordi/claurst/docs/Current/MPWO_WORK_ORDER_PACK.md:25))
- `docs/Current/` contents checked directly. No newer conflicting current-authority artifact exists there. Other files present are either explicitly historical/non-controlling in the pack or general planning/reference docs.
- Nothing in the current pack contradicts evaluating deferred architecture item `POST-M11-ARCH-02`, provided the pass remains assessment-only and does not casually reopen accepted M11 runtime semantics ([docs/Current/MPWO_WORK_ORDER_PACK.md](/home/jordi/claurst/docs/Current/MPWO_WORK_ORDER_PACK.md:23)).

Historical evidence consulted only as non-controlling traceability:

- `TASK-M11-08_AUTHORITY_RECONCILIATION_REPORT_20260415T002331Z.md` records that `ToolContext.session_budget` and `ToolContext.health_cache` were structurally invalid with the then-current crate ownership ([docs/archive/reports/TASK-M11-08_AUTHORITY_RECONCILIATION_REPORT_20260415T002331Z.md](/home/jordi/claurst/docs/archive/reports/TASK-M11-08_AUTHORITY_RECONCILIATION_REPORT_20260415T002331Z.md:31))
- `POST_M11_REPO_ASSESSMENT_20260415T071321Z.md` records the same items as deliberate deferred architecture, not proven runtime holes ([docs/archive/reports/POST_M11_REPO_ASSESSMENT_20260415T071321Z.md](/home/jordi/claurst/docs/archive/reports/POST_M11_REPO_ASSESSMENT_20260415T071321Z.md:203))

Verified files, symbols, and commands for this preflight:

- Files:
  - `AGENTS.md`
  - `docs/Current/MPWO_WORK_ORDER_PACK.md`
  - `src-rust/crates/tools/src/lib.rs`
  - `src-rust/crates/query/src/session_budget.rs`
  - `src-rust/crates/query/src/health_cache.rs`
  - `src-rust/crates/query/src/agent_tool.rs`
  - `src-rust/crates/query/src/lib.rs`
  - `src-rust/crates/tools/src/team_tool.rs`
  - `src-rust/crates/cli/src/main.rs`
  - `src-rust/crates/query/src/provider_resolution.rs`
  - `src-rust/crates/query/src/agent_tool_tests.rs`
  - `src-rust/crates/query/src/provider_resolution_tests.rs`
  - `src-rust/crates/tools/Cargo.toml`
  - `src-rust/crates/query/Cargo.toml`
- Symbols searched:
  - `ToolContext`
  - `session_budget_for_session`
  - `with_registered_session_budget`
  - `HealthCache`
  - `with_registered_session_health_cache`
  - `session_health_cache_for_session`
  - `session_health_cache_or_new`
  - `SessionBudget`
- Commands/checks run:
  - branch / `HEAD` / worktree status
  - `docs/Current/` listing
  - repo-wide symbol search with `rg`
  - direct source/test inspection with `sed` and `nl`
  - targeted worktree cleanliness check for the ticket-relevant files

# 6. Current live seam findings

## 6.1 `ToolContext` current carriage

`ToolContext` currently carries:

- generic execution context: working dir, permission state, cost tracker, session id, file history, turn counter, non-interactive flag, MCP manager, config
- provider/model registries only:
  - `provider_registry`
  - `model_registry`

It does not carry `SessionBudget` or `HealthCache` ([src-rust/crates/tools/src/lib.rs](/home/jordi/claurst/src-rust/crates/tools/src/lib.rs:216)).

## 6.2 Session budget ownership and recovery

- Root ownership remains query-owned:
  - `QueryConfig` owns `session_budget: Option<Arc<SessionBudget>>` ([src-rust/crates/query/src/lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:116))
  - CLI root setup populates `QueryConfig.session_budget` from `--budget-usd`
  - CLI does not add any budget field to `ToolContext` ([src-rust/crates/cli/src/main.rs](/home/jordi/claurst/src-rust/crates/cli/src/main.rs:734), [src-rust/crates/cli/src/main.rs](/home/jordi/claurst/src-rust/crates/cli/src/main.rs:744))
- Runtime registration is query-owned:
  - `run_query_loop()` wraps execution in `with_registered_session_budget(&tool_ctx.session_id, config.session_budget.clone(), ...)` ([src-rust/crates/query/src/lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:878))
- Registry model:
  - global registry keyed by `session_id`
  - separate task-local stack for nearest active child scope
  - root/shared budget preserved in registry, child-local overlays resolved via task-local stack ([src-rust/crates/query/src/session_budget.rs](/home/jordi/claurst/src-rust/crates/query/src/session_budget.rs:11))
- Child/team recovery:
  - `AgentTool` and injected team runner both recover inherited budget via `session_budget_for_session(&ctx.session_id)` and then build child-local scope with `child_session_budget(...)` when `budget_usd` is set ([src-rust/crates/query/src/agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:140), [src-rust/crates/query/src/agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:156), [src-rust/crates/query/src/agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:499), [src-rust/crates/query/src/agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:798))
- Cost aggregation and cancellation remain on the query side:
  - after each turn, `run_query_loop_inner()` records turn cost into the effective session budget, checks cancellation, and emits `SessionBudgetExceeded` against the shared root budget ([src-rust/crates/query/src/lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:1359))

## 6.3 Session health-cache ownership and recovery

- Ownership remains query-owned in `claurst-query`:
  - `HealthCache` and its registry live in `src-rust/crates/query/src/health_cache.rs` ([src-rust/crates/query/src/health_cache.rs](/home/jordi/claurst/src-rust/crates/query/src/health_cache.rs:15))
- Runtime registration is query-owned:
  - `run_query_loop()` wraps execution in `with_registered_session_health_cache(&tool_ctx.session_id, ...)` before entering the loop ([src-rust/crates/query/src/lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:878))
- Registry model:
  - global registry keyed by `session_id`
  - reference-counted registration cleanup
  - nested registration preserves the initial owner for the session key instead of swapping caches ([src-rust/crates/query/src/health_cache.rs](/home/jordi/claurst/src-rust/crates/query/src/health_cache.rs:100), [src-rust/crates/query/src/health_cache.rs](/home/jordi/claurst/src-rust/crates/query/src/health_cache.rs:119))
- Child/team recovery:
  - `AgentTool` and injected team runner recover `Arc<HealthCache>` via `session_health_cache_or_new(&ctx.session_id)` and pass it directly into provider fallback resolution ([src-rust/crates/query/src/agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:144), [src-rust/crates/query/src/agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:410), [src-rust/crates/query/src/agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:765))

## 6.4 Parallel, mismatched, or good enough?

- Parallel at the ownership boundary:
  - both seams are query-owned
  - both are recovered from the shared `session_id`
  - both are registered by `run_query_loop()`
  - both are consumed inside `claurst-query`, not `claurst-tools`
- Intentionally asymmetric in semantics:
  - `SessionBudget` needs nested child-local overlays plus shared-root aggregation
  - `HealthCache` needs one session-scoped cache shared across sibling child/team fallback paths
- Current live repo reality says this is not a defect mismatch. It is a domain-appropriate asymmetry.

## 6.5 Current awkwardness / fragility / validation status

- The seams are implicit-by-`session_id`, not field-explicit.
- In current repo reality that indirection is bounded and already idiomatic:
  - `claurst-tools` itself uses `session_id`-keyed registries for shell state and snapshots specifically to avoid widening `ToolContext` ([src-rust/crates/tools/src/lib.rs](/home/jordi/claurst/src-rust/crates/tools/src/lib.rs:173))
- Existing tests cover the important current behaviors:
  - health cache nested owner preservation and cleanup ([src-rust/crates/query/src/health_cache.rs](/home/jordi/claurst/src-rust/crates/query/src/health_cache.rs:318))
  - budget nearest-active-scope restore and cleanup ([src-rust/crates/query/src/session_budget.rs](/home/jordi/claurst/src-rust/crates/query/src/session_budget.rs:323))
  - child + TeamCreate same-session health-cache reuse after `ARCH-01A` ([src-rust/crates/query/src/agent_tool_tests.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool_tests.rs:501))
  - provider-resolution same-session reuse and cross-session isolation ([src-rust/crates/query/src/provider_resolution_tests.rs](/home/jordi/claurst/src-rust/crates/query/src/provider_resolution_tests.rs:749))
- I did not find a current live call path that is correctness-fragile or blocked specifically because budget/cache are not explicit `ToolContext` fields.

# 7. Concrete problem assessment

Present-problem decision: **no concrete current deficiency is proven.**

Assessment against the allowed problem categories:

- Current query-owned registries creating an actual correctness / cleanup / isolation risk:
  - not proven
  - both seams use scoped registration and cleanup
  - current tests explicitly cover session visibility and cleanup behavior
- Current seams preventing a concrete feature or testability need:
  - not proven
  - the accepted path already delivers child/team budget behavior without `ToolContext.session_budget`
  - `ARCH-01A` now delivers session-scoped health-cache reuse without `ToolContext.health_cache`
- Current seams creating unavoidable crate-boundary ambiguity in active runtime paths:
  - not proven
  - the active runtime boundary is currently clearer, not less clear: query-owned runtime state remains in `claurst-query`, generic tool context remains in `claurst-tools`
- Current seams materially duplicating logic in a way that blocks future work:
  - not proven
  - there is some local duplication between `AgentTool` and the injected team runner in `agent_tool.rs`, but it is intra-file, query-local, and does not justify a cross-crate carrier change

Invalid framings rejected by live repo reality:

- “explicit carriers are cleaner”
- “ToolContext should own everything”
- “neutral crates are architecturally nicer”
- “it might help later”

None of those are sufficient reasons to reopen the accepted seams now.

# 8. Design-space comparison

| Option | Likely owning layer | Likely touched files | Reopens accepted seams? | Coupling / crate-boundary risk | Regression risk | Testability / isolation effect | Forces `ARCH-03` or broader redesign? | Assessment |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| A. Remain deferred / no change | None | None | No | Lowest | Lowest | Keeps current tested query-owned isolation | No | Preferred. No present deficiency requires change. |
| B. Explicit `ToolContext` carriage for budget and/or health cache | `claurst-tools` plus `claurst-query`, likely plus shared abstraction work | `src-rust/crates/tools/src/lib.rs`, `src-rust/crates/cli/src/main.rs`, `src-rust/crates/query/src/agent_tool.rs`, `src-rust/crates/query/src/lib.rs`, tests, possibly Cargo manifests or a neutral crate | Yes | High. `claurst-query` depends on `claurst-tools`, so direct query-type fields in `ToolContext` create reverse-dependency pressure immediately. | High | Superficially more explicit, but only by widening a generic cross-crate context with query-runtime state | Not strictly forced, but strongly pressures broader crate/API redesign | Not justified. Solves no proven current problem. |
| C. Query-owned explicit carrier/helper without changing `ToolContext` | `claurst-query` only | likely `src-rust/crates/query/src/agent_tool.rs`, possibly a new query-local helper module, `src-rust/crates/query/src/lib.rs`, tests | No, if kept query-local | Low to moderate | Low to moderate | Could reduce local duplication while preserving ownership boundary | No | This is the narrowest non-`ToolContext` escape hatch if a future defect is proven. No current need justifies opening it. |
| D. Neutral-crate handle / abstraction approach | likely `claurst-core` or a new shared crate, plus `claurst-tools` and `claurst-query` | multiple crates, Cargo manifests, constructors, tests | Yes | Highest. Broadens a neutral crate with runtime-specific state or opaque indirection. | High | Harder to reason about because ownership becomes abstracted rather than local | Very likely to cascade into broader architecture work | Not justified. Over-scoped for current repo reality. |

No additional narrower option is strongly suggested by the live repo beyond Option C, and Option C is still not currently justified.

# 9. Risk / interaction analysis

## 9.1 `SessionBudget` ownership and registration seams

Reopening carriage would risk:

- breaking the current root/shared-vs-child-local split encoded in `shared_budget()` and the task-local stack
- weakening the current nearest-active-budget lookup behavior
- destabilizing budget-exceeded event emission against the shared root budget

## 9.2 `ARCH-01A` session-scoped `HealthCache`

Reopening carriage would risk:

- undoing the accepted query-owned session-scoped cache reuse just established by `ARCH-01A`
- regressing the “preserve initial owner for a session key” behavior
- accidentally creating per-child caches again if the new carrier were not threaded perfectly through every runtime entrypoint

## 9.3 Child/team provider-resolution paths

Current provider fallback reuse depends on:

- same-session cache lookup inside `AgentTool`
- same-session cache lookup inside the injected team runner

That path is now covered by tests and is working without `ToolContext.health_cache`. Rewiring it through `ToolContext` would be churn against a currently passing seam.

## 9.4 `ToolContext` API surface in `claurst-tools`

Adding budget/cache carriage would make `ToolContext` a mixed abstraction boundary:

- generic tool invocation context
- plus query-runtime session state

That is worse than the current split. `ToolContext` is broadly constructed and used across CLI, query tests, and tools tests. Widening it would increase cross-crate coupling for little current value.

## 9.5 Hosted-Ollama / provider-materialization locality

Provider materialization stays local to `claurst-query` (`provider_resolution.rs`). `ToolContext` carriage would not improve that locality. It would instead add pressure to move runtime concerns outward even though the current provider-resolution logic and fallback health checks are already coherently local.

## 9.6 Team runner injection seams

`TeamCreateTool` in `claurst-tools` only knows about `Arc<ToolContext>` and an injected `AgentRunFn`. That injection seam currently avoids a crate cycle. Making `ToolContext` carry query-owned runtime types would either:

- create a direct reverse dependency, or
- force a neutral-crate abstraction move

Neither is justified by a current defect.

## 9.7 Existing tests covering budget/cache/session behavior

Current tests already cover:

- session budget registration, cleanup, and nearest-scope restoration
- health-cache registration, cleanup, same-session reuse, and cross-session isolation
- child/team fallback reuse after `ARCH-01A`

Changing carriage would increase regression surface without closing a proven coverage gap.

# 10. Validation expectations if later implemented

If a later ticket ever proves a real deficiency and proposes a narrow implementation, the validation boundary should include all of the following:

- `SessionBudget` behavior:
  - root/shared aggregation
  - child-local budget overlay
  - nearest-active task-local lookup
  - cleanup after scope exit
- `HealthCache` behavior:
  - same-session reuse across child/team fallback
  - cross-session isolation
  - nested registration preserving initial owner
  - cleanup after scope exit
- Child/team runtime behavior:
  - `AgentTool` fallback resolution still reuses session-scoped cache
  - Team runner still reuses the same session-scoped cache
  - child `budget_usd` still wraps rather than replaces inherited shared budget
- Query event behavior:
  - `SessionBudgetExceeded`
  - worker budget exceeded event emission
- Constructor / API surface checks:
  - every `ToolContext { ... }` construction site still compiles and preserves the accepted semantics

If `ToolContext` were changed despite the current recommendation, validation would need to expand to both `claurst-query` and `claurst-tools`, not just query-local tests.

# 11. Recommended next-step decision

`REMAIN-DEFERRED`

Reason:

- After `ARCH-01A`, `ToolContext` carriage is **less justified**, not more justified.
- The accepted/query-owned seams now cover both:
  - child/team budget behavior
  - session-scoped health-cache reuse
- No present correctness, cleanup, isolation, or feature-blocking defect was found that the current query-owned seams cannot already solve.
- Every `ToolContext` or neutral-crate option reopens rejected seams and introduces immediate reverse-dependency or abstraction-boundary pressure.

# 12. Exact follow-on boundary

Not applicable. No narrow implementation ticket is justified from current live repo reality.

# 13. Explicit out-of-scope list

This preflight found **no need** to reopen:

- `POST-M11-ARCH-01A`
- `POST-M11-ARCH-03`
- accepted M11 runtime tickets
- M12
- unrelated docs cleanup

Also still out of scope unless a future ticket proves a concrete need:

- provider-resolution redesign
- root/child model-selection redesign
- neutral-crate refactor for runtime handles
- TeamCreate outer-cancellation redesign

# 14. Risks / notes

- Drift found:
  - noisy worktree outside the ticket scope
  - deleted `docs/Current/TASK-M11-10_SPLIT_PLAN_AND_TEMPORARY_AUTHORITY_AMENDMENT.md` is not a blocker; the current pack explicitly expects it to be deleted and non-authoritative
- Blockers:
  - none for this preflight decision
- Notes:
  - inspected ticket-relevant files are clean in the worktree
  - no implementation was performed
  - no commit was created
  - no cargo validation was run in this preflight; conclusions are based on direct source/test inspection plus repo-state checks

# 15. Final recommendation

Keep `POST-M11-ARCH-02` as deferred architecture backlog.

Do not open an implementation ticket now.

Only reconsider it if a future ticket can first prove a concrete live defect that the current query-owned `session_id`-based recovery cannot solve. If that ever happens, the first valid next step should be a **query-local** preflight for a narrow helper/carrier inside `claurst-query`, not a default move into `ToolContext` and not a neutral-crate extraction by assumption.
