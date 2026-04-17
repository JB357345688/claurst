# TASK-M8-02 Preflight Report

## Ticket
`TASK-M8-02`

## Timestamp UTC
`20260412T052210Z`

## Branch
`feature/provider-resolution-seam`

## Working Tree Summary
- Current branch verified with `git branch --show-current`: `feature/provider-resolution-seam`.
- Tracked working tree is clean enough to begin M8-02 preflight and a future execution pass on tracked source files:
  - `git status --short --untracked-files=no` returned clean.
  - `git diff --name-only` returned clean.
  - `git diff --cached --name-only` returned clean.
- Staged changes: none.
- Unstaged tracked changes: none.
- Untracked noise is significant and would affect review-basis clarity if not called out:
  - `docs/Current/MPWO_WORK_ORDER_PACK.md`
  - many prior `.md` reports under `docs/archive/reports/`
  - `.codex/`
  - `docs/Orchestrator_planning/`
  - `docs/archive/provider_orchestrator/`
  - `src-rust/target/`
- Repo-root `AGENTS.md` exists on disk but is gitignored by `.gitignore:1`.
- `docs/Current/MPWO_WORK_ORDER_PACK.md` exists on disk but is untracked.
- `TASK-M8-01` appears already committed, not pending final commit: `3f9b783 (HEAD -> feature/provider-resolution-seam) TASK-M8-01 add provider and model registries to ToolContext`.

## Authority Reviewed
- `/home/jordi/claurst/AGENTS.md`
  - reviewed from disk
  - not tracked in git baseline
  - ignored by `.gitignore`
- `/home/jordi/claurst/docs/Current/MPWO_WORK_ORDER_PACK.md`
  - reviewed from disk
  - not tracked in git baseline
  - untracked in working tree

| Kind | Target | Verified Result | Evidence |
|---|---|---|---|
| Authority | `AGENTS.md` | Present and reviewed; gitignored | on-disk file, `.gitignore:1` |
| Authority | `docs/Current/MPWO_WORK_ORDER_PACK.md` | Present and reviewed; untracked | `git status --short`, `MPWO_WORK_ORDER_PACK.md:861-914` |
| Commit/baseline | `TASK-M8-01` | Already committed at `HEAD` | `git log --oneline --decorate -n 12` |
| Symbol | `ToolContext.provider_registry` | `Option<Arc<claurst_api::ProviderRegistry>>` present | `src-rust/crates/tools/src/lib.rs:230-232` |
| Symbol | `ToolContext.model_registry` | `Option<Arc<claurst_api::ModelRegistry>>` present | `src-rust/crates/tools/src/lib.rs:233-234` |
| Production startup site | `ToolContext { ... }` | still sets both new fields to `None` | `src-rust/crates/cli/src/main.rs:651-664` |
| Registry source | `provider_registry` | built before `ToolContext` as bare `ProviderRegistry` | `src-rust/crates/cli/src/main.rs:612-613`, `src-rust/crates/api/src/registry.rs:257-284` |
| Registry source | `model_registry` | built after `ToolContext` as `Arc<ModelRegistry>` | `src-rust/crates/cli/src/main.rs:711`, `src-rust/crates/cli/src/main.rs:854-858` |
| Query seam | `QueryConfig` registry fields | populated later for root session | `src-rust/crates/cli/src/main.rs:714-742`, `src-rust/crates/query/src/lib.rs:117-128` |
| Validation | `cd src-rust && cargo check --workspace` | confirmed from MPWO; not run in this preflight | `docs/Current/MPWO_WORK_ORDER_PACK.md:903-906` |

## Exact M8-02 Scope Confirmation
- **Objective:** Wire the root session's `ProviderRegistry` and `ModelRegistry` into the `ToolContext` constructed at startup.
- **Exact code targets from MPWO:**
  - `ToolContext` construction in `crates/cli/src/main.rs` at approximately `647-658`, confirmed now at `src-rust/crates/cli/src/main.rs:651-664`
  - startup source locations for `provider_registry` and `model_registry`, confirmed by inspection
- **Precondition:** `TASK-M8-01` complete so `ToolContext` has the new fields. Current repo reality supports this.
- **Required behavior:** production startup `ToolContext` must carry the root session registries instead of `None`.
- **Strict constraints from MPWO:**
  - do not create new registries
  - do not modify how registries are built
  - do not modify `QueryConfig` construction
  - do not change any other field in the `ToolContext` construction
- **Definition of done:**
  - startup `ToolContext` in `main.rs` populated with root registries
  - `cd src-rust && cargo check --workspace` succeeds
- **Stop / escalate conditions from MPWO:**
  - if registries are not built before `ToolContext` construction, stop and trace startup flow
  - if `Arc` wrapping creates ownership issues, investigate
- **MPWO framing:** the ticket is written as simple field population only.
- **Current repo reality:** a pure literal-field substitution at the existing `ToolContext` site is unsupported because the needed values are not both available there yet.
- **Startup-order wrinkle in `main.rs`:** `confirmed`.
- **Routing/materialization scope:** MPWO does not frame M8-02 as a broader routing/materialization ticket. Current repo reality does not require widening into that work.

## Current Post-M8-01 State
- `ToolContext` now contains both required fields:
  - `provider_registry: Option<Arc<claurst_api::ProviderRegistry>>`
  - `model_registry: Option<Arc<claurst_api::ModelRegistry>>`
  - confirmed at `src-rust/crates/tools/src/lib.rs:216-235`
- Test construction sites in `claurst-tools` were updated to `None`, consistent with M8-01:
  - `src-rust/crates/tools/src/lib.rs:544-559`
  - `src-rust/crates/tools/src/lib.rs:574-589`
- The current production `ToolContext` literal still uses:
  - `provider_registry: None`
  - `model_registry: None`
  - confirmed at `src-rust/crates/cli/src/main.rs:651-664`
- Relevant production `ToolContext` construction sites:
  - exactly one production startup site found in `src-rust/crates/cli/src/main.rs:651-664`
  - other `ToolContext` literals found during search are test-only in `claurst-tools`
- Hosted Ollama compatibility baseline preserved

## Registry Construction / Availability Timeline
| Step | Event | Evidence | Note |
|---|---|---|---|
| 1 | `ClientConfig` built for startup client/provider construction | `src-rust/crates/cli/src/main.rs:596-600` | shared input for client and provider registry |
| 2 | `AnthropicClient` constructed | `src-rust/crates/cli/src/main.rs:602-605` | existing root client path |
| 3 | `provider_registry` first created as bare `ProviderRegistry` | `src-rust/crates/cli/src/main.rs:612-613` | constructor returns `Self`, confirmed by `src-rust/crates/api/src/registry.rs:257-284` |
| 4 | `ToolContext` constructed | `src-rust/crates/cli/src/main.rs:651-664` | both new registry fields still `None` here |
| 5 | `model_registry` created | `src-rust/crates/cli/src/main.rs:711`, `src-rust/crates/cli/src/main.rs:854-858` | already an `Arc<ModelRegistry>` at creation |
| 6 | `QueryConfig` constructed from `config` plus `&model_registry` | `src-rust/crates/cli/src/main.rs:714-716`, `src-rust/crates/query/src/lib.rs:174-180` | root query path gets model registry before tools do |
| 7 | `query_config.model_registry = Some(model_registry.clone())` | `src-rust/crates/cli/src/main.rs:716` | `QueryConfig` now holds the shared model registry |
| 8 | bare `provider_registry` wrapped into `Arc` | `src-rust/crates/cli/src/main.rs:741` | no type mismatch; Arc wrapping already exists in current code |
| 9 | `query_config.provider_registry = Some(provider_registry.clone())` | `src-rust/crates/cli/src/main.rs:742` | root query path now has provider registry |
| 10 | cron scheduler receives cloned `tool_ctx` and `query_config` | `src-rust/crates/cli/src/main.rs:772-778` | `query_config` has registries; `tool_ctx` does not |
| 11 | provider resolution/materialization happens later in `run_query_loop()` only when `config.provider_registry` is `Some` | `src-rust/crates/query/src/lib.rs:874-901` | this is later seam behavior, not startup construction |

## Dependency / Interface Shape Notes
- **M8-02 depends on M8-01 in MPWO:** yes, explicitly at `docs/Current/MPWO_WORK_ORDER_PACK.md:876-877`.
- **M8-02 depends on M8-01 in current repo reality:** yes, because the target `ToolContext` fields now exist and compile in source.
- **Primary likely issue:** startup-order / construction-order change in `src-rust/crates/cli/src/main.rs`.
- **Secondary shape detail:** `provider_registry` is not yet an `Arc` at its first creation site, but current repo already wraps it with `Arc::new(...)` later in the same function. This is not evidence of an `Arc` incompatibility problem.
- **Import/dependency exposure:** `main.rs` already imports `Arc` at `src-rust/crates/cli/src/main.rs:46` and already uses it multiple times.
- **Ownership/lifetime shape:** no lifetime dead-end is evident. Both registries are owned values/Arcs in startup scope and are already cloned into other startup structures.
- **Claim: not a dependency problem:** supported.
- **Claim: not an Arc incompatibility problem:** supported.
- **Claim: not a lifetime dead-end:** supported.
- **Broader interface drift:** unsupported by current evidence.
- **Consumer-state note:** current consumer tickets are not yet wired to use these `ToolContext` fields. For example, `AgentTool` still builds worker `QueryConfig` with `provider_registry: None` and `model_registry: None` at `src-rust/crates/query/src/agent_tool.rs:337-357`. That matches later M8 tickets and does not block M8-02 preflight.

## Likely Edit Surface
- **Definitely in scope**
  - `src-rust/crates/cli/src/main.rs`
  - specifically the startup block spanning current `provider_registry` construction, `ToolContext` construction, `model_registry` construction, and `QueryConfig` wiring at `612-742`
- **Maybe in scope depending on startup ordering**
  - no additional file is currently indicated
  - only intra-file statement relocation/reordering in `main.rs` is suggested by current evidence
- **Should remain untouched if the ticket stays tight**
  - `src-rust/crates/tools/src/lib.rs`
  - `src-rust/crates/query/src/lib.rs`
  - `src-rust/crates/api/src/registry.rs`
  - `src-rust/crates/api/src/model_registry.rs`
  - tests
- **Current evidence suggests:** a small reshuffle/relocation of `ToolContext` construction in `main.rs`, not a narrower literal-field substitution only at the current line, and not a broader routing/materialization change.

## Validation Command
- Confirmed from MPWO:

```bash
cd src-rust && cargo check --workspace
```

- Not run during this preflight.
- Reason for skipping: this pass was required to remain read-only except for the report file, and `cargo check` would write into `src-rust/target/`, violating that constraint.
- Basis used instead: current repo inspection, symbol verification, and `git log` confirmation that `TASK-M8-01` is already committed at `HEAD`.

## Drift Found
- **Structural drift exists:** no.
- **Authority-baseline drift exists:** yes.
  - `AGENTS.md` is present on disk but gitignored.
  - `docs/Current/MPWO_WORK_ORDER_PACK.md` is present on disk but untracked.
- **Implementation-shape drift exists:** yes, but local and understandable.
  - MPWO describes M8-02 as a simple population step.
  - Current repo reality confirms the registries are not both available at the current `ToolContext` construction point.
  - This makes the expected execution change a local startup-order adjustment inside `main.rs`, not a pure two-line substitution.

## Blockers
- No hard blocker identified for a future execution pass.
- Notes that must remain explicit when execution begins:
  - review basis is noisy because of many untracked files
  - the MPWO stop condition about registries not yet being available before `ToolContext` construction is triggered and now traced
  - the execution pass must keep scope tight to `main.rs`

## Verdict
- **Preflight verdict:** `READY-WITH-NOTES`
- **Why:** target symbols and types are present, `TASK-M8-01` is already committed, and the ticket still appears implementable inside `main.rs` alone.
- **Startup-order wrinkle:** `confirmed`
- **Execution classification:** primarily a startup/construction-order change in `main.rs`
- **Larger routing/materialization change required:** no current evidence
- **Patch hygiene note for next pass:** isolate the tracked `main.rs` delta from existing untracked docs/report noise
