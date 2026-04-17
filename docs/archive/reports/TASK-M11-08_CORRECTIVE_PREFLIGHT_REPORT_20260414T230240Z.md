# TASK-M11-08 Corrective Preflight Report

## ticket id

`TASK-M11-08`

## verdict

`HALT-REQUIRES-AUTHORITY-REVISION`

## timestamp UTC

`2026-04-14T23:02:40Z`

## branch / HEAD observed

- Branch: `feature/provider-resolution-seam`
- HEAD: `0942e4aefe99184b8caf4259d9cf9006616d6c6c`

## authority reviewed

- `AGENTS.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md`
- `docs/archive/reports/M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md`
- `docs/archive/reports/TASK-M11-08_PREFLIGHT_REPORT_20260414T153327Z.md`
- `docs/archive/reports/TASK-M11-08_EXECUTION_REPORT_20260414T223857Z.md`
- accepted baseline supplied in prompt through `TASK-M11-07`

Authority note:
- `docs/Current/MPWO_WORK_ORDER_PACK.md` currently reads as a 149-line MPWO revision summary, not a full detailed ticket pack. It confirms that M11-08 includes `ToolContext` in scope, but it does not contain a detailed `TASK-M11-08` body to override the concrete plan/report interpretation.

## blocker claim tested

Read-only commands used:
- `sed -n` / `nl -ba` on the required live files
- `rg -n` across the required runtime surfaces
- `cd src-rust && cargo metadata --no-deps --format-version 1`
- `cargo tree --manifest-path src-rust/crates/query/Cargo.toml -p claurst-query --depth 1`
- `cargo tree --manifest-path src-rust/crates/tools/Cargo.toml -p claurst-tools --depth 1`

Claim tested:
- whether adding concrete `Option<Arc<SessionBudget>>` and `Option<Arc<HealthCache>>` fields to `claurst_tools::ToolContext` is structurally valid
- whether existing reachable state already lets child/team runtime paths access a shared session budget without those concrete `ToolContext` fields

Result:
- the dependency-cycle concern is real
- no existing reachable path carries `SessionBudget` from the parent query loop into `AgentTool` / team-runner child `QueryConfig`
- `HealthCache` is not used on M11-08 runtime paths

## cargo dependency findings

- `claurst-query` depends on `claurst-tools` directly at [src-rust/crates/query/Cargo.toml](/home/jordi/claurst/src-rust/crates/query/Cargo.toml:6), specifically `claurst-tools = { workspace = true }` at line 10.
- `claurst-tools` does not depend on `claurst-query`; its dependency list at [src-rust/crates/tools/Cargo.toml](/home/jordi/claurst/src-rust/crates/tools/Cargo.toml:6) contains no `claurst-query` entry.
- `cargo tree` confirms the same one-way edge:
  - `claurst-query -> claurst-tools`
  - no reverse `claurst-tools -> claurst-query`
- `ToolContext` is defined in [src-rust/crates/tools/src/lib.rs](/home/jordi/claurst/src-rust/crates/tools/src/lib.rs:216).
- `SessionBudget` is defined in [src-rust/crates/query/src/session_budget.rs](/home/jordi/claurst/src-rust/crates/query/src/session_budget.rs:5).
- `HealthCache` is defined in `claurst-query` and consumed by provider fallback code at [src-rust/crates/query/src/provider_resolution.rs](/home/jordi/claurst/src-rust/crates/query/src/provider_resolution.rs:302).

Conclusion:
- the reported concrete-type cycle is real
- adding concrete `claurst_query::{SessionBudget, HealthCache}` fields to `claurst_tools::ToolContext` would require a reverse dependency from `claurst-tools` into `claurst-query`, which is invalid because `claurst-query` already depends on `claurst-tools`

## `ToolContext` / `QueryConfig` reachability findings

- `ToolContext` currently carries:
  - `config: claurst_core::config::Config`
  - `provider_registry`
  - `model_registry`
  - `cost_tracker`
  - no `QueryConfig`
  - no `SessionBudget`
  - no `HealthCache`
  - see [src-rust/crates/tools/src/lib.rs](/home/jordi/claurst/src-rust/crates/tools/src/lib.rs:216)
- `Config` itself has `provider` and `provider_configs`, but no `session_budget`, no budget handle, and no `health_cache`; see [src-rust/crates/core/src/lib.rs](/home/jordi/claurst/src-rust/crates/core/src/lib.rs:710).
- `QueryConfig` also currently has no `session_budget`; its current budget field is only `max_budget_usd`; see [src-rust/crates/query/src/lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:82).
- The root CLI builds `query_config` and `tool_ctx` separately at [src-rust/crates/cli/src/main.rs](/home/jordi/claurst/src-rust/crates/cli/src/main.rs:700). `ToolContext` receives `config`, registries, and `cost_tracker`, but not the constructed `QueryConfig`.
- `AgentTool` child runs construct fresh child `QueryConfig` values from `ToolContext`-reachable state only at [src-rust/crates/query/src/agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:362) and [src-rust/crates/query/src/agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:620).
- Those child builders can already reach:
  - provider selection via `ctx.config.provider`
  - provider configs via `ctx.config.provider_configs`
  - output style via `ctx.config.effective_output_style()`
  - model registry via `ctx.model_registry`
- They cannot reach a parent `QueryConfig.session_budget` because no `QueryConfig` object, no session-budget handle, and no equivalent field are present on `ToolContext`.

Answer:
- `ToolContext` already carries enough state for provider/model/output-style inheritance
- it does not carry enough state for `team_tool.rs` or `agent_tool.rs` to reach a shared `QueryConfig.session_budget`

## team cancellation-path reachability findings

- `TeamCreateTool` creates one outer cancellation token per team agent at [src-rust/crates/tools/src/team_tool.rs](/home/jordi/claurst/src-rust/crates/tools/src/team_tool.rs:367).
- Those outer tokens are used in `tokio::select!` solely around `run_agent(...)` vs `cancel.cancelled()` at [src-rust/crates/tools/src/team_tool.rs](/home/jordi/claurst/src-rust/crates/tools/src/team_tool.rs:412).
- `AgentRunParams` currently carries only:
  - prompt/task metadata
  - `ctx: Arc<ToolContext>`
  - provider/model overrides
  - no parent `QueryConfig`
  - no session-budget handle
  - no cancel-token parent handle
  - see [src-rust/crates/tools/src/team_tool.rs](/home/jordi/claurst/src-rust/crates/tools/src/team_tool.rs:37)
- The injected query-side team runner builds a fresh child `QueryConfig` from `ctx` at [src-rust/crates/query/src/agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:620) and then creates a fresh query-loop token at [src-rust/crates/query/src/agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:636).
- Foreground/background `AgentTool` child runs have the same issue:
  - fresh child `QueryConfig` from `ctx` at [src-rust/crates/query/src/agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:362)
  - fresh query-loop tokens at [src-rust/crates/query/src/agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:408) and [src-rust/crates/query/src/agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:454)

Answer:
- if `QueryConfig` alone gained `session_budget`, `team_tool.rs` still would not be able to reach it from existing reachable context
- the same is true for normal `AgentTool` child paths
- root-only `QueryConfig.session_budget` would therefore not satisfy M11-08 child/team propagation

Clarification:
- the outer `TeamCreateTool` token layer is primarily for `TeamDelete`
- but even if that outer layer were left unchanged, the inner team-runner query loop still cannot inherit a shared `SessionBudget` from current reachable state
- so the child/team blockage is still real

## `health_cache` necessity findings for M11-08 specifically

- `HealthCache` is consumed by `resolve_provider_with_fallback(...)` in [src-rust/crates/query/src/provider_resolution.rs](/home/jordi/claurst/src-rust/crates/query/src/provider_resolution.rs:302).
- Current runtime dispatch in `run_query_loop()` still uses `resolve_provider_identity(...)` and `materialize_provider(...)` directly at [src-rust/crates/query/src/lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:878).
- `rg` across the inspected runtime surfaces found no `resolve_provider_with_fallback(...)` call in:
  - `src-rust/crates/cli/src/main.rs`
  - `src-rust/crates/query/src/lib.rs`
  - `src-rust/crates/query/src/agent_tool.rs`
  - `src-rust/crates/tools/src/team_tool.rs`
- No `allow_fallback`, `budget_usd`, or related M11-10/M11-09 runtime wiring exists yet on these surfaces.

Answer:
- `health_cache` is not actually consumed anywhere on M11-08 runtime paths
- the planned `ToolContext.health_cache` addition is forward-plumbing for later fallback/runtime tickets, not required for M11-08 execution behavior itself

## minimal viable next-step interpretation, if any

Classification:
- fully in-scope salvage: none
- salvage only with narrow authority adaptation: none proven by current repo seam
- hard halt: yes

Why no narrow salvage exists:
- a root-only interpretation is technically possible:
  - add `QueryConfig.session_budget`
  - create/store it in CLI
  - charge/check it in `run_query_loop()`
- but that does not propagate into child/team runs, because both `AgentTool` and the injected team runner only receive `ToolContext`, not the parent `QueryConfig`
- dropping child/team propagation would materially narrow M11-08 below its current authority/planning intent and would not be a faithful execution of the ticket
- omitting `health_cache` alone is not enough to save the ticket, because the shared `SessionBudget` seam remains unresolved

## forbidden widenings that would be required otherwise

Any implementation path from the current repo state would require at least one authority-level widening such as:
- moving `SessionBudget` and/or `HealthCache` into a crate that `claurst-tools` can depend on
- introducing a new neutral crate for cross-crate runtime state
- inventing a new cross-crate registry or other session-budget handoff architecture
- adding trait-object or erased-type indirection for budget/cache handles
- widening into later-ticket schema/runtime work (`allow_fallback`, `budget_usd`, `max_tokens_override`) to make `HealthCache` relevant

Those are outside the allowed narrow path for this corrective pass.

## hosted Ollama invariant assessment

- preserved
- the blocker analysis required no changes to provider resolution or provider materialization
- current registry-backed runtime still uses `resolve_provider_identity(...)` plus `materialize_provider(...)`; see [src-rust/crates/query/src/lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:878)
- `HealthCache` is not active in the current M11-08 runtime path, so no hosted-Ollama behavior is implicated by this halt decision

## exact recommendation for the next prompt

Do not run an implementation pass for `TASK-M11-08` yet.

Use the next prompt to request an authority-correction pass only, for example:

`Revise TASK-M11-08 authority to reconcile the cross-crate seam: ToolContext lives in claurst-tools, while SessionBudget and HealthCache live in claurst-query. Specify the approved narrow design for child/team session-budget propagation, or explicitly defer that propagation. Do not implement code.`

In repo terms, the current ticket should remain halted until the work-order / planning authority explicitly resolves that mismatch.
