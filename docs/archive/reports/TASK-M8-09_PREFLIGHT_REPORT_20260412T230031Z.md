# TASK-M8-09 Preflight Report

- Ticket: `TASK-M8-09`
- Timestamp UTC: `20260412T230031Z`
- Branch: `feature/provider-resolution-seam`
- Verdict: `READY-WITH-NOTES`

## Preflight Verdict

- Verdict: `READY-WITH-NOTES`
- Hosted Ollama compatibility baseline preserved
- Rationale:
  - no tracked unstaged drift
  - no tracked staged drift
  - required dependency baseline is present in current branch history
  - `TASK-M8-09` scope is narrow and currently isolated to `src-rust/crates/tools/src/team_tool.rs`
  - notes remain about untracked workspace artifacts under `docs/Current/`, `docs/Orchestrator_planning/`, `docs/archive/reports/`, `.codex/`, `docs/archive/provider_orchestrator/`, and `src-rust/target/`

## Repo State Summary

- Verified commands:
  - `git branch --show-current`
  - `git status --short`
  - `git diff --name-only`
  - `git diff --cached --name-only`
  - `git log --oneline --decorate -n 20`
- Verified results:
  - branch is `feature/provider-resolution-seam`
  - `git diff --name-only`: empty
  - `git diff --cached --name-only`: empty
  - `git status --short` shows untracked noise only; no tracked drift
- Drift found:
  - tracked drift: none
  - untracked workspace noise: present, reported above
- Blockers:
  - none

## Authority Reviewed

- `AGENTS.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md`
- `docs/Orchestrator_planning/IMPLEMENTATION_PLAN_MPWO.md`
- `docs/archive/reports/TASK-M8-07_M8-08_RECONCILIATION_REPORT_20260412T160309Z.md`
- `docs/archive/reports/TASK-M8-07_COMMIT_VERIFICATION_REPORT_20260412T225608Z.md`
- `docs/archive/reports/TASK-M8-08_COMMIT_VERIFICATION_REPORT_20260412T152849Z.md`

## Dependency Baseline Confirmed

- Required closed baseline commits are present in current branch history:
  - `5d472cf` `TASK-M8-07 add provider/model fields to team spec schema`
  - `1d531da` `TASK-M8-08 wire team runner producer through shared provider seam`
  - `ea9da37` `TASK-M8-06 replace AgentRunFn positional seam with AgentRunParams`
- MPWO dependency contract for `TASK-M8-09`:
  - explicit dependencies: `TASK-M8-06`, `TASK-M8-07`
  - downstream edge: `TASK-M8-09 -> TASK-M8-10`
- Current source reality note:
  - `TASK-M8-08` is not an explicit MPWO prerequisite for `TASK-M8-09`, but its already-landed producer seam is the consumer that will receive the wired overrides

## Exact MPWO Contract For TASK-M8-09

- Owned file:
  - `src-rust/crates/tools/src/team_tool.rs`
- Owned symbols / behaviors:
  - per-agent capture block in `TeamCreateTool::execute()`
  - `run_agent(AgentRunParams { ... })` call in `TeamCreateTool::execute()`
  - behavior: pass `spec.provider` to `provider_override` and `spec.model` to `model_override`
- Exact dependencies:
  - `AgentRunParams` already includes `provider_override` and `model_override`
  - `AgentSpec` already includes `provider` and `model`
- Explicit out-of-scope items:
  - schema changes
  - query crate changes
  - D2 fields such as `allow_fallback` and `budget_usd`
  - parallel/sequential execution logic
  - cancellation token logic
- Validation command required later:
  - `cd src-rust && cargo check -p claurst-tools`
- Stop / escalate conditions:
  - MPWO states `None expected`

## Current Code Reality

- Verified files / symbols:
  - `src-rust/crates/tools/src/team_tool.rs`
    - `AgentRunParams` already contains `provider_override: Option<String>` and `model_override: Option<String>` at lines 37-46
    - `AgentSpec` already contains `provider: Option<String>` and `model: Option<String>` at lines 159-174
    - `TeamCreateTool::input_schema()` already exposes `agents[].provider` and `agents[].model`
    - `TeamCreateTool::execute()` still passes:
      - `provider_override: None`
      - `model_override: None`
  - `src-rust/crates/query/src/agent_tool.rs`
    - `init_team_swarm_runner()` destructures `AgentRunParams`
    - `model_override` is consumed to select the model before resolution
    - `provider_override` is consumed by `resolve_provider_identity(...)`
    - resolved provider/model flow through `materialize_provider(...)` into the child `QueryConfig`
- Post-M8-07 / M8-08 state checks:
  - `AgentSpec` now has provider/model: yes
  - `TeamCreateTool::input_schema()` exposes provider/model: yes
  - `TeamCreateTool::execute()` still passes `None` / `None`: yes
  - `AgentRunParams` supports override fields: yes
  - M8-08 producer seam consumes override fields in `agent_tool.rs`: yes

## Scope Confirmation

- Is `TASK-M8-09` limited to `TeamCreateTool::execute()` wiring only?
  - yes
- Does `TASK-M8-09` require any schema change?
  - no; `TASK-M8-07` already landed the `AgentSpec` and schema fields
- Does `TASK-M8-09` require any query crate change?
  - no; the query-side producer seam already consumes `provider_override` / `model_override`
- Are there any latent interface mismatches between current `TeamCreateTool::execute()` and `AgentRunParams` consumption?
  - no structural mismatch found; the types already align as owned `Option<String>` values
- Is there any risk that wiring `spec.provider` / `spec.model` into `provider_override` / `model_override` widens into unrelated behavior?
  - low risk only
  - effect is confined to agents created by `TeamCreateTool` that explicitly set provider/model
  - omitted values continue to follow existing inheritance/default behavior
  - no change is required to provider resolution, provider materialization, auth-store lookup, or hosted-Ollama handling

## Likely Smallest Edit Surface

- Single file: `src-rust/crates/tools/src/team_tool.rs`
- Single function: `TeamCreateTool::execute()`
- Smallest expected code delta:
  - capture `spec.provider.clone()` into a local `provider_override`
  - capture `spec.model.clone()` into a local `model_override`
  - replace `provider_override: None` with `provider_override`
  - replace `model_override: None` with `model_override`

## Validation Plan For Implementation Pass

- Required MPWO validation command:
  - `cd src-rust && cargo check -p claurst-tools`
- Preflight execution choice:
  - not run in this pass
  - reason: this was requested as read-only preflight, and inspection did not reveal a boundary issue that required compile confirmation

## Notes

- `TeamCreateTool::description()` text still summarizes the older input shape and does not mention `provider` / `model`; MPWO does not assign that text to `TASK-M8-09`, so it is not part of this ticket’s owned edit surface.
