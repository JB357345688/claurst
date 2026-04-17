# TASK-M11-09 Preflight Report

## ticket id

`TASK-M11-09`

## verdict

`PASS-WITH-NOTES`

## timestamp UTC

`2026-04-15T01:27:10Z`

## branch / HEAD observed

- Branch: `feature/provider-resolution-seam`
- HEAD: `b4ad28ee9eb1e434f935fd2de70c4b402a5c74da`
- Working tree: not clean
- Observed repo noise outside ticket scope: modified `.gitignore`, many untracked docs/report files, untracked `.codex`, untracked `src-rust/target/`

## authority reviewed

- `AGENTS.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md`
- `docs/archive/reports/M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md`
- `docs/archive/reports/MPWO_REVISION_REPORT_20260414T003218Z.md`
- `docs/archive/reports/TASK-M11-08R_CLOSEOUT_REPORT_20260415T004401Z.md`
- `docs/archive/reports/TASK-M11-08B_CLOSEOUT_REPORT_20260415T011952Z.md`

## accepted-baseline comparison

- Expected accepted branch/HEAD matches live repo: branch `feature/provider-resolution-seam`, HEAD `b4ad28ee9eb1e434f935fd2de70c4b402a5c74da`
- Recent history confirms the required accepted baseline chain is present in ancestry:
  - `25518cac29d34353cb58c8811da1040a3da69247` = corrected `TASK-M11-08R`
  - `b4ad28ee9eb1e434f935fd2de70c4b402a5c74da` = `TASK-M11-08B`
- Current repo reality reflects those closures:
  - root `SessionBudget` registration exists in [`src-rust/crates/query/src/lib.rs`](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:696)
  - child `SessionBudget` inheritance and child cancel-token use exist in [`src-rust/crates/query/src/agent_tool.rs`](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:134) and [`src-rust/crates/query/src/agent_tool.rs`](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:140)
- Note: baseline verification is commit-based, not working-tree-clean, because unrelated repo noise is present

## verified target files / symbols / commands

- Files:
  - [`src-rust/crates/query/src/agent_tool.rs`](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:130)
  - [`src-rust/crates/tools/src/team_tool.rs`](/home/jordi/claurst/src-rust/crates/tools/src/team_tool.rs:37)
  - [`src-rust/crates/query/src/lib.rs`](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:82)
  - [`src-rust/crates/query/src/session_budget.rs`](/home/jordi/claurst/src-rust/crates/query/src/session_budget.rs:66)
  - [`src-rust/crates/tools/src/lib.rs`](/home/jordi/claurst/src-rust/crates/tools/src/lib.rs:88)
  - [`src-rust/crates/cli/src/main.rs`](/home/jordi/claurst/src-rust/crates/cli/src/main.rs:731)
- Symbols / seams:
  - `QueryConfig.max_tokens`
  - `CHILD_AGENT_FALLBACK_MAX_TOKENS`
  - `AgentInput`
  - `AgentSpec`
  - `AgentRunParams`
  - `register_agent_runner()`
  - `session_budget_for_session()`
- Commands:
  - `git branch --show-current`
  - `git rev-parse HEAD`
  - `git status --short --branch`
  - `cd src-rust && cargo check --workspace`

## exact scope confirmation in current repo reality

- This preflight is for revised `TASK-M11-09 = child max_tokens override wiring`, not the older stale M11-09 `allow_fallback` / `budget_usd` schema wording.
- Live authority evidence aligns with the revised definition:
  - `docs/Current/MPWO_WORK_ORDER_PACK.md` and `MPWO_REVISION_REPORT_20260414T003218Z.md` both describe M11-09 as child execution override wiring
  - `M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md` defines the exact fields:
    - `AgentInput.max_tokens: Option<u32>`
    - `AgentSpec.max_tokens: Option<u32>`
    - `AgentRunParams.max_tokens_override: Option<u32>`
- `QueryConfig.max_tokens` already exists as `u32` in [`src-rust/crates/query/src/lib.rs`](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:84); no new query-config field is needed
- `AgentInput`, `AgentSpec`, and `AgentRunParams` do not currently contain any `max_tokens` child override field
- JSON schema emission is already localized to owned file emitters:
  - `AgentTool.input_schema()` in [`agent_tool.rs`](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:197)
  - `TeamCreateTool.input_schema()` in [`team_tool.rs`](/home/jordi/claurst/src-rust/crates/tools/src/team_tool.rs:213)
- No live repo evidence forces a split; the revised ticket still fits as one narrow field-and-wiring pass

## child override seam findings

- Current child `max_tokens` source is still the D1 interim constant in every child path.
- Path 1: foreground `AgentTool` child run
  - `AgentInput` currently has `description`, `prompt`, `tools`, `system_prompt`, `max_turns`, `model`, `provider`, `isolation`, `run_in_background`, but no `max_tokens` field in [`agent_tool.rs`](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:148)
  - the child `QueryConfig` is constructed once in [`agent_tool.rs`](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:378) with `max_tokens: CHILD_AGENT_FALLBACK_MAX_TOKENS`
  - the synchronous foreground run uses that same `query_config` at [`agent_tool.rs`](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:473)
- Path 2: background `AgentTool` child run
  - background mode branches at [`agent_tool.rs`](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:407)
  - it clones the already-built `query_config` at [`agent_tool.rs`](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:418)
  - therefore background child `max_tokens` is also currently derived from the same hardcoded `CHILD_AGENT_FALLBACK_MAX_TOKENS` path, not an input override
- Path 3: cc-query-backed team-runner child loop via `register_agent_runner()`
  - `AgentRunParams` currently carries `description`, `prompt`, `tools`, `system_prompt`, `max_turns`, `ctx`, `provider_override`, `model_override`, but no max-token override in [`team_tool.rs`](/home/jordi/claurst/src-rust/crates/tools/src/team_tool.rs:37)
  - `AgentSpec` currently carries `name`, `role`, `tools`, `task`, `provider`, `model`, but no `max_tokens` field in [`team_tool.rs`](/home/jordi/claurst/src-rust/crates/tools/src/team_tool.rs:156)
  - `TeamCreateTool.execute()` constructs `AgentRunParams` at [`team_tool.rs`](/home/jordi/claurst/src-rust/crates/tools/src/team_tool.rs:413) and currently passes no max-token override
  - `init_team_swarm_runner()` destructures `AgentRunParams` in [`agent_tool.rs`](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:565) and builds the team child `QueryConfig` in [`agent_tool.rs`](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:639) with `max_tokens: CHILD_AGENT_FALLBACK_MAX_TOKENS`
- Backward-compatible default path already exists and should remain after this ticket:
  - `CHILD_AGENT_FALLBACK_MAX_TOKENS` is `4_096` in [`agent_tool.rs`](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:132)
  - revised M11-09 should demote this from sole source to `unwrap_or(...)` default source

## anticipated implementation shape

- `agent_tool.rs`
  - add `max_tokens: Option<u32>` to `AgentInput`
  - expose `max_tokens` in `AgentTool.input_schema()`
  - set `QueryConfig.max_tokens` to `params.max_tokens.unwrap_or(CHILD_AGENT_FALLBACK_MAX_TOKENS)` in the shared `query_config` constructor so foreground and background paths both inherit it
  - extend team-runner destructuring to receive `max_tokens_override`
  - set team child `QueryConfig.max_tokens` to `max_tokens_override.unwrap_or(CHILD_AGENT_FALLBACK_MAX_TOKENS)`
- `team_tool.rs`
  - add `max_tokens: Option<u32>` to `AgentSpec`
  - add `max_tokens_override: Option<u32>` to `AgentRunParams`
  - expose `max_tokens` in `TeamCreateTool.input_schema()` under each agent spec
  - pass `spec.max_tokens` into `AgentRunParams` in the `run_agent(...)` call
- No `QueryConfig` shape change is required
- No `ToolContext` change is required
- No `HealthCache`, `allow_fallback`, `budget_usd`, outer cancellation, event expansion, or provider-fallback redesign work is indicated by live code

## anticipated compile-fallout scope

- Likely narrow and local, not structural:
  - `team_tool.rs` struct definition and the single `AgentRunParams { ... }` constructor
  - `agent_tool.rs` `AgentRunParams` destructuring in `init_team_swarm_runner()`
  - the two owned JSON schema emitters
- Likely test fallout is additive rather than corrective:
  - existing tests that omit `max_tokens` should remain valid because the new fields are optional
  - M11-12 should add explicit propagation coverage for foreground, background, and team-runner paths
- No compile fallout is expected in `QueryConfig`, `session_budget.rs`, provider-resolution code, or CLI root query construction beyond line-local references if signatures are changed incorrectly

## validation command run and result

- Command: `cd src-rust && cargo check --workspace`
- Result: `PASS`
- Output summary:
  - `Compiling claurst v0.0.8 (/home/jordi/claurst/src-rust/crates/cli)`
  - `Finished dev profile [unoptimized + debuginfo] target(s) in 0.29s`

## drift found

- Working tree drift exists outside ticket scope:
  - modified `.gitignore`
  - many untracked docs/report artifacts
  - untracked `.codex`
  - untracked `src-rust/target/`
- Documentation-shape drift exists but is already called out by the user:
  - `docs/Current/MPWO_WORK_ORDER_PACK.md` is revision/summary-like rather than a clean rewritten work-order pack
  - despite that, it does reflect the revised M11-09 child `max_tokens` definition, so there is no blocking semantic conflict for this preflight

## blockers, if any

- No structural blocker found for revised `TASK-M11-09`
- Note only: patch hygiene for the future implementation pass must account for unrelated dirty worktree state

## hosted Ollama invariant assessment

- `preserved`
- Basis:
  - the required M11-09 seam is confined to child `max_tokens` override wiring in `agent_tool.rs` and `team_tool.rs`
  - no provider resolution algorithm change is required
  - no provider materialization change is required
  - no root CLI request-shaping change is required
  - no hosted Ollama-specific code path was identified as needing modification
- Closed-ticket risk assessment:
  - corrected `TASK-M11-08R` root session-budget wiring risk: low, because root `QueryConfig.session_budget` creation and root cancellation-token paths remain untouched
  - `TASK-M11-08B` child/team session-budget propagation risk: low, because `session_budget` inheritance and `child_cancel_token()` selection remain in place and only `QueryConfig.max_tokens` sourcing changes
  - `TASK-M11-05` same-domain fallback behavior risk: low, because `allow_fallback` is explicitly out of scope and the provider resolution/materialization calls stay structurally unchanged

## exact recommendation for next step

- Proceed with revised `TASK-M11-09` as a single implementation ticket.
- Keep the edit surface limited to:
  - [`src-rust/crates/query/src/agent_tool.rs`](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:148)
  - [`src-rust/crates/tools/src/team_tool.rs`](/home/jordi/claurst/src-rust/crates/tools/src/team_tool.rs:37)
- Implement only:
  - `AgentInput.max_tokens: Option<u32>`
  - `AgentSpec.max_tokens: Option<u32>`
  - `AgentRunParams.max_tokens_override: Option<u32>`
  - wiring through the three spawn paths
  - schema exposure in the two existing input-schema emitters
- Preserve backward compatibility by keeping `CHILD_AGENT_FALLBACK_MAX_TOKENS = 4096` as the default when the override is absent.
