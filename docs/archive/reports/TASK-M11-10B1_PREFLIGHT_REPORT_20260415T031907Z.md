# TASK-M11-10B1 Preflight Report

## ticket id

`TASK-M11-10B1`

## verdict

`PASS-WITH-NOTES`

## timestamp UTC

`2026-04-15T03:19:07Z`

## branch / HEAD observed

- Branch: `feature/provider-resolution-seam`
- HEAD: `ea046c52da82dfd9778f4065bd36b36e28d73c8a`
- Expected accepted latest HEAD from authority: `ea046c52da82dfd9778f4065bd36b36e28d73c8a`
- Match result: `exact match`

## authority reviewed

- `AGENTS.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md`
- `docs/Current/TASK-M11-10_SPLIT_PLAN_AND_TEMPORARY_AUTHORITY_AMENDMENT.md`
- `docs/archive/reports/M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md`
- `docs/archive/reports/MPWO_REVISION_REPORT_20260414T003218Z.md`
- `docs/archive/reports/TASK-M11-08R_CLOSEOUT_REPORT_20260415T004401Z.md`
- `docs/archive/reports/TASK-M11-08B_CLOSEOUT_REPORT_20260415T011952Z.md`
- `docs/archive/reports/TASK-M11-09_CLOSEOUT_REPORT_20260415T014220Z.md`
- `docs/archive/reports/TASK-M11-10A_CLOSEOUT_REPORT_20260415T024141Z.md`
- `docs/archive/reports/TASK-M11-10B_AUTHORITY_REPORT_20260415T030449Z.md`
- `docs/archive/reports/TASK-M11-10B_SPLIT_AUTHORITY_REPORT_20260415T031240Z.md`
- Live code surfaces inspected:
  - `src-rust/crates/query/src/session_budget.rs`
  - `src-rust/crates/query/src/lib.rs`
  - `src-rust/crates/query/src/agent_tool.rs`
  - `src-rust/crates/tools/src/team_tool.rs`
  - `src-rust/crates/tools/src/lib.rs`
  - `src-rust/crates/query/src/provider_resolution.rs`
  - `src-rust/crates/query/src/health_cache.rs`
  - `src-rust/crates/cli/src/main.rs`

## accepted-baseline comparison

- `TASK-M11-08R` preserved in live code:
  - root CLI `budget_usd` still maps to `QueryConfig.session_budget = Some(Arc::new(SessionBudget::new(usd)))` in `src-rust/crates/cli/src/main.rs:731-735`
  - `QueryConfig.session_budget` still exists in `src-rust/crates/query/src/lib.rs:115-120`
  - `run_query_loop()` still records spend into `session_budget` and calls `check_and_cancel()` in `src-rust/crates/query/src/lib.rs:1414-1417`
- `TASK-M11-08B` preserved in live code:
  - `run_query_loop()` still registers the active budget under `tool_ctx.session_id` via `register_session_budget(...)` in `src-rust/crates/query/src/lib.rs:696-699`
  - child/team paths still inherit by `session_budget_for_session(&ctx.session_id)` in `src-rust/crates/query/src/agent_tool.rs:134-137`
  - child/team paths still derive cancellation from the inherited budget via `child_cancel_token()` in `src-rust/crates/query/src/agent_tool.rs:140-145`
- `TASK-M11-09` preserved in live code:
  - `AgentInput.max_tokens: Option<u32>` still exists in `src-rust/crates/query/src/agent_tool.rs:163-165`
  - `AgentSpec.max_tokens: Option<u32>` still exists in `src-rust/crates/tools/src/team_tool.rs:174-176`
  - `AgentRunParams.max_tokens_override: Option<u32>` still exists in `src-rust/crates/tools/src/team_tool.rs:37-44`
  - child loops still default through `CHILD_AGENT_FALLBACK_MAX_TOKENS = 4096` in `src-rust/crates/query/src/agent_tool.rs:132` and `:398-402`, `:658-659`
- `TASK-M11-10A` preserved in live code:
  - `AgentInput.allow_fallback: Option<bool>` still exists in `src-rust/crates/query/src/agent_tool.rs:172-174`
  - `AgentSpec.allow_fallback: Option<bool>` still exists in `src-rust/crates/tools/src/team_tool.rs:177-179`
  - `AgentRunParams.allow_fallback: bool` still exists in `src-rust/crates/tools/src/team_tool.rs:43-45`
  - child resolution still uses `resolve_provider_with_fallback(...)` with `allow_fallback` in `src-rust/crates/query/src/agent_tool.rs:312-323` and `:627-636`
- Rejected assumptions remain rejected in live code:
  - no `ToolContext.session_budget` in `src-rust/crates/tools/src/lib.rs:216-235`
  - no `ToolContext.health_cache` in `src-rust/crates/tools/src/lib.rs:216-235`
  - no concrete query-owned budget/cache type added to `ToolContext`

## verified target files / symbols / commands

- Verified files:
  - `src-rust/crates/query/src/session_budget.rs`
  - `src-rust/crates/query/src/lib.rs`
  - `src-rust/crates/query/src/agent_tool.rs`
  - `src-rust/crates/tools/src/team_tool.rs`
  - `src-rust/crates/tools/src/lib.rs`
  - `src-rust/crates/query/src/provider_resolution.rs`
  - `src-rust/crates/query/src/health_cache.rs`
  - `src-rust/crates/cli/src/main.rs`
- Verified symbols:
  - `SessionBudget`
  - `register_session_budget()`
  - `session_budget_for_session()`
  - `QueryConfig.session_budget`
  - `run_query_loop()`
  - `AgentInput.max_tokens`
  - `AgentInput.allow_fallback`
  - `AgentSpec.max_tokens`
  - `AgentSpec.allow_fallback`
  - `AgentRunParams.max_tokens_override`
  - `AgentRunParams.allow_fallback`
  - `resolve_provider_with_fallback()`
  - `HealthCache`
- Commands run:
  - `git branch --show-current`
  - `git rev-parse HEAD`
  - `git status --short --branch`
  - `git diff -- .gitignore`
  - `git diff --name-only ea046c52da82dfd9778f4065bd36b36e28d73c8a..HEAD`
  - `git show --stat --oneline --no-patch 25518cac29d34353cb58c8811da1040a3da69247 b4ad28ee9eb1e434f935fd2de70c4b402a5c74da 4ef9547dab51959f7b39c473f929b81f05ee1134 ea046c52da82dfd9778f4065bd36b36e28d73c8a`
  - targeted `rg -n` and `nl -ba | sed -n` inspections across the files above
  - `cd src-rust && cargo check --workspace`

## exact scope confirmation in current repo reality

- This preflight is for `TASK-M11-10B1 = query-owned child budget seam resolution`, not unsplit `10B`, not `10B2`, and not schema/runtime carriage.
- `10B1` can stay query-owned only on live repo reality.
- The blocker is still the query-owned runtime seam:
  - one `QueryConfig.session_budget`
  - one registry slot per shared `session_id`
  - one spend-record/check path in `run_query_loop()`
- No current live evidence makes `team_tool.rs` necessary for `10B1`.
- `10B2` remains blocked until `10B1` is accepted.
- Explicitly excluded scope remains valid and necessary:
  - no `AgentInput.budget_usd`
  - no `AgentSpec.budget_usd`
  - no `AgentRunParams.budget_usd`
  - no `ToolContext.session_budget`
  - no `ToolContext.health_cache`
  - no `HealthCache` budget semantics
  - no reopening of `09` or `10A`
  - no reinterpretation of child `budget_usd` as `max_budget_usd`

## child-budget seam findings

1. Current `SessionBudget` is a single mutable budget object with:
   - `budget_usd`
   - `spent`
   - one `root_token`
   - methods `record_cost()`, `check_and_cancel()`, `child_cancel_token()`, `is_cancelled()`
   in `src-rust/crates/query/src/session_budget.rs:12-43`
2. Current registry behavior is single-entry per `session_id`.
   - `SESSION_BUDGET_REGISTRY` is `DashMap<String, (Arc<SessionBudget>, usize)>` in `src-rust/crates/query/src/session_budget.rs:8-9`
   - `register_session_budget()` overwrites the stored budget pointer on an occupied entry in `src-rust/crates/query/src/session_budget.rs:70-75`
   - `session_budget_for_session()` returns only one `Arc<SessionBudget>` in `src-rust/crates/query/src/session_budget.rs:86-90`
3. `run_query_loop()` currently supports only one live budget per loop.
   - registration at entry: `src-rust/crates/query/src/lib.rs:696-699`
   - spend record + cancel check after usage accounting: `src-rust/crates/query/src/lib.rs:1414-1417`
   - separate per-loop `max_budget_usd` guard remains intact: `src-rust/crates/query/src/lib.rs:1421-1433`
4. All child/team inheritance sites currently carry only the inherited parent budget under the shared parent `session_id`.
   - foreground/background AgentTool child runs build `QueryConfig { session_budget: session_budget.clone(), max_budget_usd: None, ... }` in `src-rust/crates/query/src/agent_tool.rs:396-416`
   - background cancel token is derived from that inherited budget in `src-rust/crates/query/src/agent_tool.rs:446-459`
   - foreground cancel token is derived from that inherited budget in `src-rust/crates/query/src/agent_tool.rs:492-505`
   - query-backed team runner again inherits `session_budget` by `ctx.session_id` and sets it in `QueryConfig` in `src-rust/crates/query/src/agent_tool.rs:654-668`
   - `TeamCreateTool` only carries `max_tokens_override` and `allow_fallback` today; there is no `budget_usd` field in `src-rust/crates/tools/src/team_tool.rs:37-47` and `:156-180`
5. What breaks if a child-specific `SessionBudget` is introduced under the current seam:
   - child registration would replace the parent registry entry for the same shared `session_id`
   - descendant children would inherit the replaced child budget instead of the parent shared budget
   - child spend would no longer necessarily hit the accepted parent shared-session accounting path from `08B`
   - that would violate the corrected-path requirement that child/descendant spend still count against the inherited parent shared `SessionBudget`
6. Child `budget_usd` therefore cannot mean:
   - `max_budget_usd`
   - replacement of the parent shared `SessionBudget`
   - a synonym for `max_tokens`
7. The faithful meaning on current repo reality remains:
   - an additional child-local cumulative USD cap for the spawned child subtree
   - while preserving accounting against the inherited parent shared `SessionBudget`
8. Descendant inheritance semantics that must be preserved once child-local budget exists:
   - descendants of a child with child-local budget must still spend against the root shared parent session budget from `08B`
   - descendants must also remain subject to the nearest active ancestor child-local cap for that subtree
   - if a deeper descendant later gets its own child-local budget, the deeper subtree must remain constrained by all active ancestor child-local caps plus the root shared budget
9. Current repo reality therefore points to a layered/nested/composite budget seam, or an equivalently narrow query-owned formulation, rather than a second independent single-slot `SessionBudget`
10. `HealthCache` remains unrelated.
    - it is only passed into `resolve_provider_with_fallback(...)` in `src-rust/crates/query/src/provider_resolution.rs:302-310`
    - its implementation is provider health TTL caching only in `src-rust/crates/query/src/health_cache.rs:12-68`
    - no inspected budget path depends on it

## anticipated implementation shape

- The narrowest valid `10B1` shape is still query-owned seam work in:
  - `src-rust/crates/query/src/session_budget.rs`
  - `src-rust/crates/query/src/lib.rs`
- The seam should preserve the accepted external meaning of root `budget_usd` from `08R` and inherited parent shared-session accounting from `08B`.
- Live repo reality suggests the accepted seam needs one of these narrow formulations:
  - layered budget accounting
  - nested budget scopes
  - composite budget object
  - another query-owned formulation with equivalent behavior
- The seam must support three runtime facts simultaneously:
  - child-local cumulative USD cap exists
  - parent shared-session accounting still records all descendant spend
  - nested descendants inherit the correct combined semantics
- `team_tool.rs` should stay out of `10B1`.
- `agent_tool.rs` should stay out of `10B1` unless the chosen query-owned API shape makes a tiny compatibility adjustment unavoidable; live repo reality does not make that unavoidable today.

## anticipated compile-fallout scope

- Expected owned compile-fallout scope for `10B1`:
  - `src-rust/crates/query/src/session_budget.rs`
  - `src-rust/crates/query/src/lib.rs`
- Likely no unavoidable additional fallout is implied by current repo reality if the seam preserves the outward query-owned registration/inheritance surface.
- If any extra query-side fallout appears, `src-rust/crates/query/src/agent_tool.rs` is the only plausible next file, and only for narrow compatibility adjustment.
- `src-rust/crates/tools/src/team_tool.rs` remains out of scope for `10B1`.

## validation command run and result

- Command: `cd src-rust && cargo check --workspace`
- Result: `PASS`
- Output summary:
  - `Compiling claurst v0.0.8 (/home/jordi/claurst/src-rust/crates/cli)`
  - `Finished dev profile [unoptimized + debuginfo] target(s) in 0.32s`

## drift found

- Working tree is dirty and noisy:
  - tracked `.gitignore` modified to add `.envrc` and `.env`
  - many untracked docs/report files
  - untracked `.codex`
  - untracked `src-rust/target/`
- The controlling authority docs named in this preflight are present locally but are not git-tracked at this baseline.
  - This is not a structural blocker for this preflight because the prompt explicitly designates them as evidence authority.
  - It does mean the review basis must keep authority-doc provenance explicit.
- No code drift was observed against the accepted HEAD named for this ticket:
  - branch matched
  - HEAD matched
  - `git diff --name-only ea046c52da82dfd9778f4065bd36b36e28d73c8a..HEAD` returned no committed code drift

## blockers, if any

- No structural blocker was found for `TASK-M11-10B1` preflight.
- Notes that must stay explicit:
  - worktree noise exists outside ticket scope
  - authority docs are untracked local artifacts
  - `10B2` remains blocked pending accepted `10B1`

## hosted Ollama invariant assessment

`preserved`

Basis:

- Live hosted-Ollama-sensitive surfaces are outside the narrow `10B1` seam boundary.
- `10B1` can stay query-owned and budget-semantics-only.
- No inspected evidence ties hosted Ollama compatibility, same-domain fallback behavior from `TASK-M11-05`, or provider materialization to the child-budget seam problem.
- Preserving file scope to `session_budget.rs` and `query::lib.rs` avoids reopening:
  - hosted Ollama compatibility
  - same-domain fallback behavior from `TASK-M11-05`
  - `09` child `max_tokens`
  - `10A` child `allow_fallback`

## exact recommendation for next step

- Proceed only to `TASK-M11-10B1` implementation planning/execution, not to `10B2`.
- Keep the ticket explicitly framed as:
  - `TASK-M11-10B1 = query-owned child budget seam resolution`
  - not schema/runtime carriage
  - not `max_budget_usd` reinterpretation
  - not `max_tokens` work
- Constrain the implementation prompt to the minimal query-owned seam needed to support:
  - child-local cumulative USD cap for a child subtree
  - preserved parent shared-session accounting from `08B`
  - correct nested descendant inheritance semantics
- Keep default owned file scope to:
  - `src-rust/crates/query/src/session_budget.rs`
  - `src-rust/crates/query/src/lib.rs`
- Treat `src-rust/crates/query/src/agent_tool.rs` as fallback-only compatibility scope if and only if preflight-era assumptions prove insufficient during implementation.
- Keep `src-rust/crates/tools/src/team_tool.rs` out of `10B1`.
- Keep `HealthCache` out of scope.
- Keep `TASK-M11-10B2` blocked until `10B1` is implemented, validated, and accepted.
