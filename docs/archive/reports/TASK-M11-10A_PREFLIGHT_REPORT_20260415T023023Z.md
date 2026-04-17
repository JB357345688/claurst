# TASK-M11-10A Preflight Report

## ticket id

`TASK-M11-10A`

## verdict

`PASS-WITH-NOTES`

## timestamp UTC

`2026-04-15T02:30:23Z`

## branch / HEAD observed

- Branch: `feature/provider-resolution-seam`
- HEAD: `4ef9547dab51959f7b39c473f929b81f05ee1134`
- HEAD matches the accepted latest baseline named for this ticket: `yes`
- Working tree state: not clean
- Observed repo noise:
  - modified `.gitignore`
  - many untracked docs/report files under `docs/`
  - untracked `.codex`
  - untracked `src-rust/target/`

## authority reviewed

- `AGENTS.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md`
- `docs/Current/TASK-M11-10_SPLIT_PLAN_AND_TEMPORARY_AUTHORITY_AMENDMENT.md`
- `docs/archive/reports/M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md`
- `docs/archive/reports/MPWO_REVISION_REPORT_20260414T003218Z.md`
- `docs/archive/reports/TASK-M11-08R_CLOSEOUT_REPORT_20260415T004401Z.md`
- `docs/archive/reports/TASK-M11-08B_CLOSEOUT_REPORT_20260415T011952Z.md`
- `docs/archive/reports/TASK-M11-09_CLOSEOUT_REPORT_20260415T014220Z.md`
- `docs/archive/reports/TASK-M11-10_PREFLIGHT_REPORT_20260415T021006Z.md`

## accepted-baseline comparison

- Accepted baseline commits are present on the current branch in the expected order:
  - `25518cac29d34353cb58c8811da1040a3da69247` (`TASK-M11-08R`)
  - `b4ad28ee9eb1e434f935fd2de70c4b402a5c74da` (`TASK-M11-08B`)
  - `4ef9547dab51959f7b39c473f929b81f05ee1134` (`TASK-M11-09`)
- Live repo reality matches the corrected path those tickets established:
  - root `SessionBudget` wiring exists in `src-rust/crates/cli/src/main.rs` and `src-rust/crates/query/src/lib.rs`
  - child/team session-budget propagation exists through the query-owned session-id keyed seam in `src-rust/crates/query/src/session_budget.rs` and `src-rust/crates/query/src/agent_tool.rs`
  - child `max_tokens` override wiring exists across foreground, background, and team-runner child paths in `src-rust/crates/query/src/agent_tool.rs` and `src-rust/crates/tools/src/team_tool.rs`
- Rejected broad-path assumptions remain rejected in live code:
  - no `ToolContext.session_budget`
  - no `ToolContext.health_cache`
  - no concrete query-owned types added to `ToolContext`

## verified target files / symbols / commands

- Primary target files inspected:
  - `src-rust/crates/query/src/agent_tool.rs`
  - `src-rust/crates/tools/src/team_tool.rs`
- Supporting read-only files inspected:
  - `src-rust/crates/query/src/provider_resolution.rs`
  - `src-rust/crates/query/src/health_cache.rs`
  - `src-rust/crates/query/src/lib.rs`
  - `src-rust/crates/query/src/session_budget.rs`
  - `src-rust/crates/tools/src/lib.rs`
  - `src-rust/crates/cli/src/main.rs`
- Symbols verified in live code:
  - `AgentInput`
  - `AgentSpec`
  - `AgentRunParams`
  - `register_agent_runner()`
  - `resolve_provider_with_fallback()`
  - `resolve_provider_identity()`
  - `materialize_provider()`
  - `HealthCache`
  - `SessionBudget`
  - `register_session_budget()`
  - `session_budget_for_session()`
- Commands run:
  - `git branch --show-current`
  - `git rev-parse HEAD`
  - `git status --short --branch`
  - `git log --oneline --decorate -n 6`
  - `git diff --name-only`
  - `cd src-rust && cargo check --workspace`

## exact scope confirmation in current repo reality

- This preflight is for `TASK-M11-10A = allow_fallback only`, not full `TASK-M11-10`.
- The split-plan amendment is consistent with live repo reality and is the controlling authority for this step.
- Current type reality:
  - `AgentInput` currently has `max_tokens`, `model`, `provider`, and background/isolation fields, but no `allow_fallback`
  - `AgentSpec` currently has `provider`, `model`, and `max_tokens`, but no `allow_fallback`
  - `AgentRunParams` currently has `provider_override`, `model_override`, and `max_tokens_override`, but no `allow_fallback`
- Current schema reality:
  - `AgentTool` schema exposes `max_tokens`, `model`, `provider`, `isolation`, `run_in_background`, but not `allow_fallback`
  - `TeamCreateTool` agent schema exposes `provider`, `model`, and `max_tokens`, but not `allow_fallback`
- Current spawn-path reality:
  - foreground `AgentTool` child runs and background `AgentTool` child runs share one provider-resolution block before the branch on `run_in_background`
  - the cc-query-backed team-runner child path resolves provider separately inside the closure registered through `register_agent_runner()`
- Current default behavior already aligns with the proposed `10A` default:
  - child provider resolution does not use fallback today
  - adding `allow_fallback: Option<bool>` on input/spec types and resolving it into `AgentRunParams.allow_fallback: bool` can preserve omitted behavior as `false`
- No current-repo evidence requires widening into `budget_usd`, session-budget redesign, or TeamCreate cancellation redesign in order to complete `allow_fallback` wiring.

## child fallback seam findings

1. Foreground `AgentTool` child run
- Provider resolution currently happens in `src-rust/crates/query/src/agent_tool.rs:281-314` via `resolve_provider_identity()` followed by `materialize_provider()`.
- Session-budget inheritance currently comes from `inherited_session_budget(&ctx.session_id)` in `src-rust/crates/query/src/agent_tool.rs:383`.
- Child cancel-token behavior currently derives from `session_budget.child_cancel_token()` when an inherited session budget exists, otherwise `CancellationToken::new()`, via `inherited_child_cancel_token(...)`.
- Child `max_tokens` currently comes from `params.max_tokens.unwrap_or(CHILD_AGENT_FALLBACK_MAX_TOKENS)` in `src-rust/crates/query/src/agent_tool.rs:385-389`.
- Child `max_budget_usd` is currently `None`.

2. Background `AgentTool` child run
- Provider resolution is the same shared block as the foreground path because both reuse the already-built `target` from `src-rust/crates/query/src/agent_tool.rs:281-314`.
- Session-budget inheritance is the same shared `query_config.session_budget` built from `inherited_session_budget(&ctx.session_id)`.
- Child cancel-token behavior uses `inherited_child_cancel_token(config_bg.session_budget.as_ref())` in `src-rust/crates/query/src/agent_tool.rs:433-445`.
- Child `max_tokens` is the same `params.max_tokens.unwrap_or(4096)` path because background uses the shared `query_config`.

3. cc-query-backed team-runner child loop via `register_agent_runner()`
- `TeamCreateTool` currently constructs `AgentRunParams` with `provider_override`, `model_override`, and `max_tokens_override` in `src-rust/crates/tools/src/team_tool.rs:397-432`.
- Provider resolution currently happens inside the registered runner closure in `src-rust/crates/query/src/agent_tool.rs:613-639` via `resolve_provider_identity()` and `materialize_provider()`.
- Session-budget inheritance currently comes from `inherited_session_budget(&ctx.session_id)` in `src-rust/crates/query/src/agent_tool.rs:647-658`.
- Child cancel-token behavior currently uses `inherited_child_cancel_token(query_config.session_budget.as_ref())` in `src-rust/crates/query/src/agent_tool.rs:667-679`.
- Child `max_tokens` currently comes from `max_tokens_override.unwrap_or(CHILD_AGENT_FALLBACK_MAX_TOKENS)` in `src-rust/crates/query/src/agent_tool.rs:649-652`.
- Child `max_budget_usd` is still the default `None`.

4. Fallback-aware provider seam
- `resolve_provider_with_fallback()` already exists at `src-rust/crates/query/src/provider_resolution.rs:302-388`.
- Its live signature already matches the narrow `10A` need:
  - explicit provider hint
  - model
  - model registry
  - provider registry
  - provider configs
  - `&HealthCache`
  - `allow_fallback: bool`
- When `allow_fallback` is `false`, it returns the direct materialization error wrapped as fallback-disabled behavior.
- When `allow_fallback` is `true`, it narrows fallback candidates to the same `TrustDomain` before attempting healthy/degraded alternatives.

5. Session-budget and max-token seams to preserve
- Child session-budget inheritance currently depends on the accepted `08B` query-owned session-id keyed registry in `src-rust/crates/query/src/session_budget.rs:66-89`.
- `register_session_budget()` still overwrites the registered budget for an occupied `session_id` and increments the registration count.
- Child `max_tokens` override remains the accepted `09` seam and already works independently of provider fallback.
- Nothing in live repo reality requires `allow_fallback` wiring to alter either seam.

## anticipated implementation shape

- Narrow expected changes:
  - add `AgentInput.allow_fallback: Option<bool>`
  - add `AgentSpec.allow_fallback: Option<bool>`
  - add `AgentRunParams.allow_fallback: bool`
  - expose `allow_fallback` in both `AgentTool` and `TeamCreateTool` schemas
  - thread the resolved boolean through:
    - foreground `AgentTool` child runs
    - background `AgentTool` child runs
    - cc-query-backed team-runner child loops registered through `register_agent_runner()`
  - replace the two current child provider-resolution call sites in `agent_tool.rs` with narrow use of `resolve_provider_with_fallback(...)`
- Narrow acceptable runtime cache path:
  - runtime-local `HealthCache::new()` usage is viable for this ticket
  - no evidence supports adding `ToolContext.health_cache`
  - no evidence supports any `ToolContext.session_budget` addition
- Expected defaulting rule:
  - omitted `allow_fallback` should be resolved to `false`
  - that preserves current behavior exactly
- No live-repo reason to widen into:
  - `budget_usd`
  - session-budget registry redesign
  - TeamCreate outer-cancellation redesign
  - provider-policy redesign beyond existing same-domain fallback seam

## anticipated compile-fallout scope

- Likely narrow constructor/schema fallout only:
  - `src-rust/crates/query/src/agent_tool.rs`
  - `src-rust/crates/tools/src/team_tool.rs`
- Likely minor supporting fallout:
  - imports in `agent_tool.rs` for `HealthCache` and `resolve_provider_with_fallback`
  - destructuring of `AgentRunParams` in `init_team_swarm_runner()`
  - local tests in `agent_tool.rs`, `team_tool.rs`, or provider-resolution tests if new coverage is added
- Structural churn is not indicated for `10A`.
- `src-rust/crates/query/src/lib.rs`, `src-rust/crates/query/src/session_budget.rs`, `src-rust/crates/tools/src/lib.rs`, and `src-rust/crates/cli/src/main.rs` do not appear to need changes for `allow_fallback` only.

## validation command run and result

- Command run: `cd src-rust && cargo check --workspace`
- Result: `PASS`
- Output summary:
  - `Compiling claurst v0.0.8 (/home/jordi/claurst/src-rust/crates/cli)`
  - `Finished dev profile [unoptimized + debuginfo] target(s) in 0.33s`

## drift found

- No branch drift: branch and HEAD exactly match the accepted baseline for this ticket.
- No baseline-ticket drift:
  - `08R` root session-budget wiring is present
  - `08B` child/team session-budget propagation is present
  - revised `09` child `max_tokens` override wiring is present
- Authority drift note:
  - `docs/Current/MPWO_WORK_ORDER_PACK.md` still reflects the unsplit `M11-10` wording (`allow_fallback, budget_usd` together)
  - this is not a blocker for `10A` because the split-plan amendment explicitly supersedes stale wording for the current corrected path
- Repo-state note:
  - dirty worktree noise exists and should be kept out of any later execution/review basis

## blockers, if any

- No structural blocker was found for `TASK-M11-10A`.
- Notes to preserve during execution:
  - do not recombine `10A` with `10B`
  - do not introduce `ToolContext.session_budget`
  - do not introduce `ToolContext.health_cache`
  - do not redesign the accepted `08B` session-id keyed propagation seam
  - keep unrelated worktree noise out of the ticket diff

## hosted Ollama invariant assessment

- Hosted Ollama compatibility is not at risk from the narrow `10A` shape.
- Basis:
  - the existing fallback seam already constrains candidate providers by `TrustDomain`
  - `TASK-M11-05` same-domain fallback behavior remains the governing provider-policy seam
  - `10A` only needs to expose and wire the child-level boolean that decides whether those existing rules are used
  - no hosted-Ollama request-shaping, auth-store, CLI root wiring, or root query-loop redesign is implicated

## exact recommendation for next step

- Proceed to `TASK-M11-10A` implementation as a single narrow ticket.
- Keep the implementation explicitly limited to:
  - `allow_fallback` schema exposure in `AgentTool` and `TeamCreateTool`
  - `AgentInput` / `AgentSpec` / `AgentRunParams` field wiring
  - narrow child-path integration with `resolve_provider_with_fallback(...)`
  - default `allow_fallback = false`
- Do not include any `budget_usd` behavior in this ticket.
- Do not modify the accepted `08R`, `08B`, or `09` seams except for the minimum child-path fallback wiring needed to consume them unchanged.
