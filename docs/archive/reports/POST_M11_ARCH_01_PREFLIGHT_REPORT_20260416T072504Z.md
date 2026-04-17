# Ticket ID

`POST-M11-ARCH-01`

# Verdict

`GO-NARROW-IMPLEMENTATION-PREFLIGHT`

# Timestamp UTC

`2026-04-16T07:25:04Z`

# Branch / HEAD / worktree summary

- Current branch: `feature/provider-resolution-seam`
- Current HEAD: `63595c387ac8fd2f5adbf9cf75d45a724153c3db`
- Accepted post-M11 runtime baseline referenced by prompt: `b157924e130fdf71c09a3787b47dd5eb1f31d542`
- Worktree is dirty and noisy. `git status --short --branch` shows:
- Modified: `.gitignore`
- Deleted: `docs/Current/TASK-M11-10_SPLIT_PLAN_AND_TEMPORARY_AUTHORITY_AMENDMENT.md`
- Large untracked set under `docs/archive/reports/`, `docs/archive/provider_orchestrator/`, `docs/Orchestrator_planning/`, `.codex`, `src-rust/.codex`, and `src-rust/target/`
- Scope-attribution risk: later implementation/review work will need explicit patch hygiene because the worktree already contains substantial unrelated noise.
- Drift verdict for this preflight: no structural drift found in the inspected `HealthCache` / provider-resolution / team / session seams. The deleted split-era current doc matches the live authority pack and is not a blocker.

# Authority reviewed

- Repo authority file reviewed: `AGENTS.md`
- Live current authority artifact reviewed: `docs/Current/MPWO_WORK_ORDER_PACK.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md` explicitly states it is the single active authority artifact in `docs/Current/`.
- The pack explicitly says the deleted split-era amendment doc must not be treated as live authority.
- `ls -lt docs/Current` shows no newer alternate current-authority pack. Other files in `docs/Current/` are older and the pack itself classifies them as historical/non-controlling.
- Nothing in the current pack contradicts evaluating deferred post-M11 architecture backlog. The pack says M11 is complete, preserves the accepted post-M11 baseline, and says not to casually reopen accepted runtime semantics.
- Preflight authority verdict: evaluating `POST-M11-ARCH-01` is allowed, provided the work stays narrow and does not reopen accepted M11/M12 runtime semantics.

# Current `HealthCache` ownership/use findings

- `HealthCache` is defined in `src-rust/crates/query/src/health_cache.rs:12-68`.
- It stores `DashMap<String, (ProviderStatus, Instant)>` with a 30s TTL and a 5s probe timeout.
- `probe_if_stale()` does `get -> provider.health_check() -> insert` and has no in-flight probe coalescing.
- `HealthCache` is re-exported from `src-rust/crates/query/src/lib.rs:19-36`.
- Live runtime instantiation is only in `src-rust/crates/query/src/agent_tool.rs`:
- Foreground/background `AgentTool` path creates a fresh cache at `agent_tool.rs:402-411`.
- Team runner injection path creates a fresh cache at `agent_tool.rs:758-766`.
- `resolve_provider_with_fallback()` accepts `&HealthCache` at `src-rust/crates/query/src/provider_resolution.rs:306-389`.
- Fallback only runs when direct `materialize_provider()` fails and `allow_fallback` is true.
- `resolve_provider_with_fallback()` iterates same-trust-domain candidates from `provider_registry.provider_ids()` and probes each candidate through `health_cache.probe_if_stale(...)`.
- `ToolContext` does not carry `HealthCache`. Live shape in `src-rust/crates/tools/src/lib.rs:214-235` includes `provider_registry` and `model_registry`, but no cache field.
- `TeamCreateTool` shares an `Arc<ToolContext>` across agent futures at `src-rust/crates/tools/src/team_tool.rs:400-452`, but it does not carry or inject a health cache.
- `run_query_loop()` registers session budgets at `src-rust/crates/query/src/lib.rs:867-894` via `with_registered_session_budget(...)`, but there is no parallel `HealthCache` ownership/lifetime seam.
- Root query paths do not use `HealthCache` today. In `src-rust/crates/query/src/lib.rs:1098-1126`, root registry-backed dispatch uses `resolve_provider_identity()` plus `materialize_provider()` directly and never calls `resolve_provider_with_fallback()`.
- Practical lifetime classification:
- Not process-global
- Not per-session
- Not per-query-loop
- Not per-team-run
- More narrow than per-child
- In practice it is per fallback-resolution call: one cache is allocated, passed into one `resolve_provider_with_fallback(...)`, then dropped before the child query loop begins.
- Current tests reflect only local cache semantics:
- `src-rust/crates/query/src/health_cache.rs:187-262` tests miss/hit/TTL/probe error/timeout
- `src-rust/crates/query/src/provider_resolution_tests.rs:663-752` creates fresh `HealthCache::new()` per test call
- `src-rust/crates/query/src/agent_tool_tests.rs:412-438` covers single-agent same-domain fallback only
- No test currently exercises session-shared ownership, team-shared reuse, or cross-agent duplicate suppression.

# Concrete problem assessment

- There is no current correctness defect in the accepted runtime path. The accepted path works with runtime-local `HealthCache::new()`.
- There is, however, a concrete present ownership/lifetime problem: the live cache lifetime is so short that TTL reuse is effectively unavailable across related child/team fallback work.
- Within one `resolve_provider_with_fallback()` call, each candidate provider is probed at most once. Because the cache is dropped immediately afterward, the current `HealthCache` gives almost no reuse value beyond that single call.
- Repeated same-provider probing is concretely plausible in current call paths:
- Repeated `AgentTool` spawns in the same parent session with `allow_fallback: true` each allocate a fresh cache and re-probe the same same-domain candidates.
- `TeamCreateTool` defaults to parallel execution and each agent run gets its own fresh cache through the injected runner. Multiple same-session agents with `allow_fallback: true` can therefore probe the same candidate providers independently.
- This is especially plausible when multiple agents request a missing/unavailable direct provider and rely on same-domain fallback, because that is exactly when `resolve_provider_with_fallback()` runs.
- Secondary issue: related child/team fallback decisions in one session do not have any shared health view. Once one child has learned that a candidate is degraded or unavailable, sibling children do not reuse that result.
- Invalid framings rejected:
- This is not justified merely because session ownership is “cleaner”.
- This is not justified merely because a global singleton is architecturally elegant.
- This is not justification to move cache carriage into `ToolContext`.
- Problem verdict: there is a real present runtime inefficiency and consistency gap around child/team fallback reuse. It is not a correctness bug, but it is concrete enough to justify a narrow follow-on if that follow-on stays inside `claurst-query`.

# Design-space comparison

| Option | Likely owner | Likely touched files | ToolContext / neutral-crate move required | Coupling / regression / isolation / namespace risk | Hosted-Ollama / materialization locality | ARCH-02 / ARCH-03 impact | Assessment |
| --- | --- | --- | --- | --- | --- | --- | --- |
| A. Remain deferred / no change | None | None | No | Lowest regression risk, but leaves `HealthCache` effectively per-call and leaves duplicate child/team probing unchanged | Preserved | No reopening | Defensible, but leaves a concrete reuse gap in accepted child/team fallback seams |
| B. Query-owned, session-shared cache | `claurst-query`, keyed by `session_id` | `query/src/health_cache.rs`, `query/src/lib.rs`, `query/src/agent_tool.rs`, query tests | No | Moderate implementation risk, low coupling if lifecycle stays in query crate, test isolation manageable if registration cleanup mirrors `SessionBudget` | Preserved if materialization stays in `provider_resolution.rs` and cache stores status only | Can avoid reopening both deferred items | Narrowest viable implementation shape |
| C. Query-runtime-scoped shared cache not keyed by session | Static inside `claurst-query` | Similar to B, but simpler registry | No | Higher namespace/lifetime/test-isolation risk because unrelated sessions in one process would share provider status keyed only by provider ID | More fragile under per-session `provider_configs` and auth differences | Avoids other tickets, but creates new cross-session semantics | Not recommended |
| D. Process-global shared cache | Process-wide singleton | Similar to C, potentially wider docs/tests | No | Highest risk: cross-session contamination, unclear invalidation, unsafe assumptions about provider ID meaning across sessions/tests | Weakest locality story because provider status becomes detached from session config/auth context | Likely expands architecture surface | Reject |
| E. Team-run-local cache only | Team runner / TeamCreate path only | `query/src/agent_tool.rs` only, maybe tests | No | Narrow but fragmented; direct `AgentTool` path still re-probes, nested children still split semantics | Preserved | Avoids other tickets, but creates inconsistent child-vs-team behavior | Reject as too seam-specific |

Additional option notes:

- Option B aligns with the existing session seam already present for `SessionBudget` in `src-rust/crates/query/src/session_budget.rs:11-171`.
- Option B can stay entirely inside `claurst-query` if the session lifetime is owned by `run_query_loop()` and read by `AgentTool` / team-runner resolution sites.
- Options C and D are materially riskier than they first appear because `HealthCache` keys only by provider ID string, while actual provider materialization depends on per-session config and auth inputs.

# Risk / interaction analysis

- `resolve_provider_with_fallback()`:
- Resolution order, trust-domain filtering, and healthy-before-degraded ordering in `provider_resolution.rs:306-389` must remain unchanged.
- A session-shared cache should change only ownership/lifetime of cached status, not fallback semantics.
- Child `allow_fallback`:
- Default false must remain unchanged in `agent_tool.rs:402` and `team_tool.rs:416`.
- Sharing cache must not widen fallback candidacy or make fallback implicit.
- Child/team provider-resolution paths:
- These are the only live runtime consumers of `HealthCache` today.
- A narrow implementation can target these without changing root registry-backed dispatch.
- `SessionBudget` ownership seams:
- Existing session-budget registration already proves the repo has a query-owned session registry pattern.
- A parallel health-cache registry can mirror that pattern without moving budget/cache carriage into `ToolContext`.
- Hosted-Ollama neutrality / provider-materialization locality:
- `materialize_provider()` special-cases Ollama and local base URL overrides in `provider_resolution.rs:197-255` and `392-420`.
- Any shared cache must remain a status cache only. It must not absorb provider construction, auth loading, or base URL selection.
- Existing `HealthCache` tests:
- Current tests do not cover shared ownership or cleanup. A later implementation must add that coverage.
- Team-runner injection seams:
- `TeamCreateTool` itself should not need schema or context changes.
- The injected runner in `agent_tool.rs:699-838` is the correct narrow seam for team-agent reuse.
- Query-owned, session-shared acceptance criteria:
- Session identity is already present on `ToolContext.session_id`.
- Session-budget registry makes session ownership a naturally aligned model.
- It can stay inside `claurst-query`.
- Root query path should own lifetime via `run_query_loop()` registration, but root query dispatch does not need to start calling fallback.
- Child/team paths should share the same session cache because they already execute under the same session ID.
- In-flight probe coalescing is not required to justify this first narrow ticket, but it must be explicitly left out of scope if the ticket only claims session-shared reuse.
- Process-global rejection criteria:
- No live code or docs define a safe cross-session sharing contract.
- Provider status keyed only by provider ID is unsafe across sessions that may differ in auth/config.
- Tests currently rely on fresh caches and isolated auth environments; global cache would raise isolation risk immediately.
- Adopting process-global first would force a broader invalidation/namespace contract than this ticket allows.
- No-change assessment:
- No-change remains acceptable if the bar is “no correctness defect”.
- No-change is weaker if the bar is “does `HealthCache` have meaningful runtime reuse today?” because live code says mostly no.

# Validation expectations if later implemented

- Focused unit tests for session-shared ownership:
- registering a session cache
- reusing the same cache across multiple `AgentTool` fallback resolutions in one session
- cleanup after session scope ends
- TTL behavior tests under shared ownership:
- cached status reused within TTL across separate child/team resolution calls
- stale entries re-probed after TTL expiry
- Duplicate-probe behavior tests:
- explicit test that sequential same-session resolutions do not re-probe within TTL
- if coalescing remains out of scope, tests should document that concurrent cold misses may still probe more than once
- Child/team fallback coverage:
- `AgentTool` same-session repeated fallback reuse
- `TeamCreate` sibling agents reusing a session cache when probes are staggered or sequential
- Test-isolation protection:
- session cleanup between tests
- no leaked cached status across independent session IDs
- Observability / non-regression checks:
- provider selection outcome unchanged
- `allow_fallback` gating unchanged
- hosted-Ollama provider materialization unchanged
- `SessionBudget` inheritance/cancellation unchanged

# Recommended next-step decision

`GO-NARROW-IMPLEMENTATION-PREFLIGHT`

Reason:

- The live repo shows a concrete present reuse gap: `HealthCache` is effectively per resolution call and does not currently provide meaningful reuse across related child/team fallback work.
- A narrow implementation boundary exists that stays entirely inside `claurst-query`, does not require `ToolContext` carriage, and does not reopen cancellation redesign.
- Process-global sharing is not justified by live repo reality and is materially riskier than session-scoped sharing.

# Exact follow-on boundary

- Exact ticket name: `POST-M11-ARCH-01A — Session-scoped HealthCache ownership in claurst-query`
- Exact scope:
- Introduce a query-owned session-scoped `HealthCache` lifetime keyed by `session_id`
- Make that cache available to live child/team fallback-resolution sites only
- Keep provider materialization, trust-domain logic, and `allow_fallback` semantics unchanged
- Exact likely files touched:
- `src-rust/crates/query/src/health_cache.rs`
- `src-rust/crates/query/src/lib.rs`
- `src-rust/crates/query/src/agent_tool.rs`
- `src-rust/crates/query/src/provider_resolution_tests.rs`
- `src-rust/crates/query/src/agent_tool_tests.rs`
- Exact files that must remain out of scope:
- `src-rust/crates/tools/src/lib.rs`
- `src-rust/crates/tools/src/team_tool.rs`
- `src-rust/crates/query/src/provider_resolution.rs` except for signature-preserving plumbing if strictly necessary
- any neutral/shared crate move
- M12 docs or unrelated current/archival docs
- Exact non-regression boundaries:
- no reintroduction of `ToolContext.health_cache`
- no change to `resolve_provider_with_fallback()` selection semantics
- no change to child/team `allow_fallback` defaults or trust-domain rules
- no process-global cache semantics
- no provider materialization/auth/base-URL logic changes
- no `SessionBudget` ownership redesign
- Change class: `runtime + tests`
- In-flight probe coalescing: explicitly out of scope for this first ticket
- Rationale for leaving coalescing out of scope:
- session-shared lifetime is the narrowest safe first correction
- parallel cold-miss suppression is a separable later concern once ownership is settled

# Explicit out-of-scope list

- `POST-M11-ARCH-02`
- `POST-M11-ARCH-03`
- accepted M11 runtime tickets
- M12
- unrelated docs cleanup
- `ToolContext` budget/cache carriage reconsideration
- TeamCreate outer-cancellation redesign
- neutral-crate relocation of `HealthCache`
- process-global singleton adoption

# Risks / notes

- Verified commands/checks run in this preflight:
- `git branch --show-current`
- `git rev-parse HEAD`
- `git status --short --branch`
- `ls -lt docs/Current`
- repo-wide `rg` searches for `HealthCache`, `probe_if_stale`, `resolve_provider_with_fallback`, `ToolContext`, `session_budget_for_session`, `with_registered_session_budget`, `allow_fallback`, `run_query_loop`, and provider `health_check()`
- direct inspection of:
- `src-rust/crates/query/src/health_cache.rs`
- `src-rust/crates/query/src/provider_resolution.rs`
- `src-rust/crates/query/src/agent_tool.rs`
- `src-rust/crates/query/src/lib.rs`
- `src-rust/crates/query/src/session_budget.rs`
- `src-rust/crates/query/src/provider_resolution_tests.rs`
- `src-rust/crates/query/src/agent_tool_tests.rs`
- `src-rust/crates/tools/src/team_tool.rs`
- `src-rust/crates/tools/src/lib.rs`
- `src-rust/crates/cli/src/main.rs`
- No code or tests were run. This pass was inspection/analysis/reporting only.
- Residual caution:
- A session-shared cache without coalescing will not eliminate all duplicate probes during parallel cold-start fan-out.
- That is acceptable only if the follow-on ticket is explicitly framed as an ownership/lifetime correction, not as full duplicate suppression.

# Final recommendation

Proceed with a narrow implementation preflight for a query-owned, session-scoped `HealthCache` inside `claurst-query`, and reject process-global sharing for now.

Do not reopen `ToolContext` carriage, cancellation redesign, M11 runtime tickets, or M12. The live repo supports one narrow follow-on centered on cache lifetime/ownership only.
