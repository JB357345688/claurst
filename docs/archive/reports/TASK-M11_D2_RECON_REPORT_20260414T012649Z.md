# TASK-M11 D2 Recon Report

## 1. Title

`TASK-M11 / M12 D2 recon preflight (read-only)`

## 2. Timestamp UTC

`2026-04-14T01:26:49Z`

## 3. Branch / HEAD observed

- Branch: `feature/provider-resolution-seam`
- HEAD: `6b362a09c4ef4d614840ed199869bb9d38600e16`
- HEAD subject: `6b362a0 TASK-M9-12 complete D1 provider-resolution seam validation and smoke gate`

## 4. Authority reviewed

- `AGENTS.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md`
- `docs/Orchestrator_planning/IMPLEMENTATION_PLAN_MPWO.md`
- `docs/Current/D1_REVIEW_REPORT_20260413T233604Z.md`
- `docs/Current/M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md`
- `docs/archive/reports/MPWO_REVISION_REPORT_20260414T003218Z.md`

## 5. Scope of this recon pass

- Read-only recon only.
- Narrow D2 scope only:
  - `TASK-M11-05`
  - `TASK-M11-08`
  - `TASK-M11-09`
  - `TASK-M12-01` through `TASK-M12-03`
- No implementation, staging, or commits.
- Hosted-Ollama non-regression treated as standing background authority per `docs/Current/M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md:272` and `:719-809`.

## 6. Repo state and workspace reality

- Ticket id: `TASK-M11_D2_RECON`
- Preflight verdict: `PASS-WITH-NOTES`
- Current tracked drift is limited to `.gitignore`:
  - `git status --short --untracked-files=no` -> `M .gitignore`
- No tracked Rust-source drift is present:
  - `git diff --name-only -- src-rust` returned no output.
- There is substantial untracked workspace/report noise under:
  - `docs/Current/`
  - `docs/Orchestrator_planning/`
  - `docs/archive/provider_orchestrator/`
  - `docs/archive/reports/`
  - `src-rust/target/`
- Rust workspace root is `src-rust/`, not repo root:
  - only workspace manifest found was `src-rust/Cargo.toml:1-16`
- Current D1 accepted baseline still claims no D2 work has started:
  - `docs/Current/D1_REVIEW_REPORT_20260413T233604Z.md:170-172`
- Live tracked source matches that claim for D2-only symbols:
  - no tracked hits for `TrustDomain`, `HealthCache`, `SessionBudget`, `resolve_provider_with_fallback`, `allow_fallback`, `budget_usd`, `max_tokens_override`, `WorkerProviderResolved`, `WorkerBudgetExceeded`, or `SessionBudgetExceeded`
  - existing hits for `max_budget_usd` and `fallback_model` are pre-D2 baseline surfaces at `src-rust/crates/query/src/lib.rs:113-116` and `:1397-1409`
- Note on authority-doc status:
  - the cited `docs/Current/` and `docs/Orchestrator_planning/` files were readable in this checkout but are currently untracked in git status

## 7. Verified M11 surface map

| Surface | Live location | Notes |
|---|---|---|
| `QueryConfig` | `src-rust/crates/query/src/lib.rs:78-129` | Existing D1 fields include `max_budget_usd`, `fallback_model`, `provider_registry`, `model_registry` |
| Root per-loop budget guard | `src-rust/crates/query/src/lib.rs:1397-1409` | Existing `max_budget_usd` abort point |
| `QueryEvent` | `src-rust/crates/query/src/lib.rs:391-424` | No D2 worker/session budget variants yet |
| Root registry-backed seam callsite | `src-rust/crates/query/src/lib.rs:874-891` | Calls `resolve_provider_identity()` then `materialize_provider()` |
| `resolve_provider_identity()` | `src-rust/crates/query/src/provider_resolution.rs:101-155` | Pure D1 seam |
| `materialize_provider()` | `src-rust/crates/query/src/provider_resolution.rs:157-215` | Side-effectful provider materialization |
| `AgentInput` | `src-rust/crates/query/src/agent_tool.rs:135-163` | Has `model`, `provider`, background flag; no D2 fields yet |
| `AgentTool::execute()` | `src-rust/crates/query/src/agent_tool.rs:233-470` | Foreground + background child path |
| Background spawn branch | `src-rust/crates/query/src/agent_tool.rs:390-435` | Reuses prebuilt child config/context |
| Foreground child run | `src-rust/crates/query/src/agent_tool.rs:453-466` | Fresh token per run |
| D1 child max-token interim | `src-rust/crates/query/src/agent_tool.rs:130-132` | `CHILD_AGENT_FALLBACK_MAX_TOKENS = 4_096` |
| Team runner registration | `src-rust/crates/query/src/agent_tool.rs:543-649` | `init_team_swarm_runner()` implements the actual team child query-loop path |
| `AgentRunParams` | `src-rust/crates/tools/src/team_tool.rs:37-46` | Only carries provider/model overrides today |
| `run_agent()` dispatch | `src-rust/crates/tools/src/team_tool.rs:68-73` | Converges team path into registered runner |
| `AgentSpec` | `src-rust/crates/tools/src/team_tool.rs:156-172` | Has `provider` and `model`; no D2 fields yet |
| `TeamCreateTool::execute()` | `src-rust/crates/tools/src/team_tool.rs:267-440` | Multi-agent spawn surface |
| Team per-agent cancel tokens | `src-rust/crates/tools/src/team_tool.rs:367-372` | Distinct from query-loop cancel token inside runner |
| `ToolContext` definition | `src-rust/crates/tools/src/lib.rs:216-235` | Has `provider_registry`, `model_registry`; no `session_budget` / `health_cache` |
| CLI `ToolContext` construction | `src-rust/crates/cli/src/main.rs:729-742` | Primary root propagation point |
| Root provider/model registry setup | `src-rust/crates/cli/src/main.rs:612-613`, `:696-701`, `:726-741` | Root session already propagates both registries |
| Root run cancel token | `src-rust/crates/cli/src/main.rs:1115-1134` | Root query loop still starts from fresh `CancellationToken::new()` |
| `CostTracker` | `src-rust/crates/core/src/lib.rs:2850-2899` | No D2 attribution fields yet |
| `ModelRegistry` | `src-rust/crates/api/src/model_registry.rs:50-59` | Core registry for model/provider lookup |
| `find_provider_for_model()` | `src-rust/crates/api/src/model_registry.rs:261-319` | Existing bare-model provider lookup |
| `list_by_provider()` | `src-rust/crates/api/src/model_registry.rs:321-327` | Important live reality for M11-05 |
| `best_model_for_provider()` | `src-rust/crates/api/src/model_registry.rs:337-388` | Existing provider-default selector |
| `ProviderRegistry` | `src-rust/crates/api/src/registry.rs:21-24` | Existing provider registry |
| `provider_ids()` | `src-rust/crates/api/src/registry.rs:128-130` | Existing candidate enumeration entrypoint |
| `check_all_health()` | `src-rust/crates/api/src/registry.rs:135-146` | Existing whole-registry health scan |
| `LlmProvider::health_check()` / `capabilities()` | `src-rust/crates/api/src/provider.rs:80-86` | Live D2 health/capability seams |
| `ProviderCapabilities` | `src-rust/crates/api/src/provider_types.rs:183-209` | Already includes `tool_calling`, `thinking`, `image_input`, `pdf_input`, `audio_input`, `structured_output` |
| `ProviderStatus` | `src-rust/crates/api/src/provider_types.rs:230-237` | `Healthy`, `Degraded`, `Unavailable` |

## 8. M11-05 findings

- Real fallback seam still sits exactly where M10 said it would:
  - identity resolution at `src-rust/crates/query/src/provider_resolution.rs:101-155`
  - provider materialization at `src-rust/crates/query/src/provider_resolution.rs:157-215`
  - root seam consumption at `src-rust/crates/query/src/lib.rs:874-891`
- Existing provider-registry/model-registry lookup surfaces are sufficient for D2 candidate enumeration:
  - `ProviderRegistry::provider_ids()` at `src-rust/crates/api/src/registry.rs:128-130`
  - `ProviderRegistry::get()` at `src-rust/crates/api/src/registry.rs:113-116`
  - `ModelRegistry::find_provider_for_model()` at `src-rust/crates/api/src/model_registry.rs:261-319`
  - `ModelRegistry::list_by_provider()` at `src-rust/crates/api/src/model_registry.rs:321-327`
  - `ModelRegistry::best_model_for_provider()` at `src-rust/crates/api/src/model_registry.rs:337-388`
- `ModelRegistry::models_for_provider()` does **not** exist.
- Exact helper-gap assessment:
  - functional gap: `none confirmed`
  - naming/API drift: `confirmed`
  - reason: the live code already has `list_by_provider(provider_id) -> Vec<&ModelEntry>` at `src-rust/crates/api/src/model_registry.rs:321-327`, which satisfies the enumeration need M10 described
  - smallest possible compatibility helper, if M11-05 still wants the name `models_for_provider()`: add a one-line wrapper in the same `impl ModelRegistry` block delegating to `list_by_provider()`
- This means M10 risk `R1` is stale relative to current repo reality:
  - `docs/Current/M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md:435-439`
  - M11-05 does **not** need a new storage/index design to enumerate provider models
- Existing health/status surfaces that M11-05 will rely on later:
  - `ProviderStatus` enum at `src-rust/crates/api/src/provider_types.rs:230-237`
  - provider trait health seam at `src-rust/crates/api/src/provider.rs:80`
  - provider trait capability seam at `src-rust/crates/api/src/provider.rs:86`
  - `ProviderCapabilities` fields already cover the planned D2 capability fallback fields at `src-rust/crates/api/src/provider_types.rs:183-205`
- Exact files/symbols M11-05 will need to touch later:
  - primary: `src-rust/crates/query/src/provider_resolution.rs`
    - add `resolve_provider_with_fallback(...)`
    - consume `ProviderRegistry::provider_ids()`
    - consume `ProviderStatus`
    - consume `LlmProvider::capabilities()`
  - prerequisite-owned but directly coupled:
    - `src-rust/crates/api/src/provider_types.rs` for `TrustDomain::for_provider(...)`
    - `src-rust/crates/query/src/health_cache.rs` for `HealthCache`
    - `src-rust/crates/query/src/provider_resolution.rs` for capability helpers from M11-03
  - optional only if naming consistency is desired:
    - `src-rust/crates/api/src/model_registry.rs` for `models_for_provider()` alias to `list_by_provider()`
- Hidden dependency/drift not already called out by M10:
  - the root loop already has an unrelated per-loop `fallback_model` path at `src-rust/crates/query/src/lib.rs:116` and `:1303-1323`
  - M11-05 must keep spawn-time D2 provider fallback conceptually separate from this existing root-loop model fallback

## 9. M11-08 / M11-09 spawn-path findings

### Foreground `AgentTool` path

- Entry surface: `src-rust/crates/query/src/agent_tool.rs:233-470`
- Provider/model override flow today:
  - input fields at `:149-154`
  - parent-provider hint derivation at `:260-272`
  - resolution at `:285-288`
  - materialization at `:290-293`
- Child `max_tokens` source today:
  - `QueryConfig.max_tokens = CHILD_AGENT_FALLBACK_MAX_TOKENS` at `:362-365`
- Child budget source today:
  - `max_budget_usd: None` at `:377`
- Child fallback source today:
  - `fallback_model: None` at `:378`
- Child registry propagation today:
  - `provider_registry: Some(registry.clone())` at `:379`
  - `model_registry: ctx.model_registry.clone()` at `:382`
- Cancellation token creation today:
  - `CancellationToken::new()` at `:454`
- Later D2 flow points:
  - `allow_fallback` must affect the resolution call site currently at `:285`
  - budget/session propagation must affect the `QueryConfig` literal at `:362-383`
  - session-budget child token must replace the fresh token at `:454`

### Background `AgentTool` path

- Branch location: `src-rust/crates/query/src/agent_tool.rs:390-447`
- This path does **not** build a second child config; it reuses:
  - `foreground_ctx` from `:385-386`
  - `query_config` from `:362-383`
- Cancellation token creation today:
  - `CancellationToken::new()` at `:408`
- Child `max_tokens` source today:
  - indirectly still `:364` because background path reuses the already-built `query_config`
- Child budget/fallback source today:
  - indirectly still `:377-378`
- Later D2 flow points:
  - M11-09 must update the shared child-config construction at `:362-383`; changing only the background branch will miss the real source
  - M11-08 must replace the fresh token at `:408`

### Team / multi-agent path

- Team entry surface: `src-rust/crates/tools/src/team_tool.rs:267-440`
- Team per-agent cancellation surface today:
  - `CancellationToken::new()` per agent at `:367-372`
- Team override capture today:
  - `provider_override = spec.provider.clone()` at `:389`
  - `model_override = spec.model.clone()` at `:390`
- Team dispatch convergence today:
  - `run_agent(AgentRunParams { ... })` at `:412-423`
  - `run_agent()` forwards to the registered runner at `src-rust/crates/tools/src/team_tool.rs:68-73`
  - actual runner lives in `src-rust/crates/query/src/agent_tool.rs:543-649`
- Runner-side provider/model flow today:
  - params destructure at `src-rust/crates/query/src/agent_tool.rs:548-557`
  - resolution at `:586-590`
  - materialization at `:600-604`
- Important live difference from direct `AgentTool` path:
  - team runner uses only `provider_override` at `:587`
  - it does **not** derive a `parent_provider` hint equivalent to `AgentTool::execute()` `:260-272`
  - no current team-path test covers omitted-provider inheritance; existing team test covers only explicit mixed providers at `src-rust/crates/query/src/agent_tool.rs:988-1044`
- Team child `max_tokens` source today:
  - `QueryConfig.max_tokens = CHILD_AGENT_FALLBACK_MAX_TOKENS` at `src-rust/crates/query/src/agent_tool.rs:620-623`
- Team child budget source today:
  - inherited `..Default::default()` at `:630`, therefore `max_budget_usd: None`
- Team child registry propagation today:
  - `provider_registry: Some(registry.clone())` at `:628`
  - `model_registry: ctx.model_registry.clone()` at `:629`
- Team child query-loop cancellation token today:
  - fresh token at `:636`
- Later D2 flow points:
  - `AgentSpec` at `src-rust/crates/tools/src/team_tool.rs:156-172` must gain `max_tokens`, then later `allow_fallback` and `budget_usd`
  - `AgentRunParams` at `src-rust/crates/tools/src/team_tool.rs:37-46` must gain `max_tokens_override`, then later `allow_fallback` and `budget_usd`
  - team per-agent cancel tokens at `src-rust/crates/tools/src/team_tool.rs:367-372` and runner token at `src-rust/crates/query/src/agent_tool.rs:636` both matter for M11-08

### Duplication / convergence assessment

- Convergence is still as expected:
  - direct single-agent path lives in `src-rust/crates/query/src/agent_tool.rs`
  - multi-agent orchestration lives in `src-rust/crates/tools/src/team_tool.rs`
  - actual team child query-loop execution converges back into `src-rust/crates/query/src/agent_tool.rs:543-649`
- Code-sharing reality:
  - background path is mostly shared with foreground because it reuses the same prebuilt child config/context
  - team runner duplicates much of the foreground path's provider-resolution and child-config assembly instead of calling a shared helper

### Paths that can silently keep using `CHILD_AGENT_FALLBACK_MAX_TOKENS`

- Direct `AgentTool` foreground path via `src-rust/crates/query/src/agent_tool.rs:364`
- Direct `AgentTool` background path because it reuses that same config at `src-rust/crates/query/src/agent_tool.rs:401`
- Team runner path via `src-rust/crates/query/src/agent_tool.rs:622`

### Exact `SessionBudget` / `ToolContext` propagation surfaces later

- Root creation and first propagation:
  - CLI budget flag already exists at `src-rust/crates/cli/src/main.rs:244-245`
  - root per-loop budget assignment today at `src-rust/crates/cli/src/main.rs:719-720`
  - root `ToolContext` literal at `src-rust/crates/cli/src/main.rs:729-742`
  - root run token at `src-rust/crates/cli/src/main.rs:1116-1134`
- `QueryConfig` additions will fan out to:
  - struct definition `src-rust/crates/query/src/lib.rs:78-129`
  - default impl `src-rust/crates/query/src/lib.rs:131-155`
  - test helper config literal `src-rust/crates/query/src/lib.rs:2127-2147`
  - direct child config literal `src-rust/crates/query/src/agent_tool.rs:362-383`
  - team runner child config literal `src-rust/crates/query/src/agent_tool.rs:620-630`
- `ToolContext` additions will fan out to:
  - struct definition `src-rust/crates/tools/src/lib.rs:216-235`
  - test literals `src-rust/crates/tools/src/lib.rs:544-559` and `:574-589`
  - query test literal `src-rust/crates/query/src/lib.rs:2198-2212`
  - agent-tool test literal `src-rust/crates/query/src/agent_tool.rs:686-700`
  - CLI root literal `src-rust/crates/cli/src/main.rs:729-742`

## 10. M12 surrogate-test findings

- Historical surrogate artifact exists exactly where requested:
  - `docs/archive/provider_orchestrator/d2_test_micro_patch_report.md`
- What the artifact says:
  - it documents a proposed test-only patch against `src-rust/crates/api/src/registry.rs`
  - it names test helper concepts `TestTrustDomain` and `cloud_fallback_candidate_ids()` at `docs/archive/provider_orchestrator/d2_test_micro_patch_report.md:57-78`
  - it names the test `unknown_custom_providers_default_to_cloud_and_enter_cloud_candidate_scope` at `docs/archive/provider_orchestrator/d2_test_micro_patch_report.md:190-242`
  - it describes the test as an executable-spec regression around registry/health/capability seams at `:3-17`
- Live-source reality:
  - that surrogate test is **not present** in tracked source
  - search found no live hits for:
    - `unknown_custom_providers_default_to_cloud_and_enter_cloud_candidate_scope`
    - `cloud_fallback_candidate_ids`
    - `TestTrustDomain`
  - current live `src-rust/crates/api/src/registry.rs` ends with `impl Default for ProviderRegistry` at `:471-474` and has no test module
- Search evidence:
  - `find src-rust/crates/api/tests src-rust/crates/query/tests -maxdepth 2 -type f | sort` failed because both directories do not exist
  - broad `rg` over `src-rust/crates/query/src`, `docs/archive/reports`, and `docs/archive/provider_orchestrator` found the surrogate only in archive docs and planning docs, not in tracked source
- Assessment of the historical surrogate if it were ever reintroduced:
  - it would cover one narrow unique intent:
    - unknown/custom providers default to Cloud
    - cloud candidate selection filters by health and tool-calling
  - it would still be removable later because real D2 production tests should subsume it; it is not a production seam test today
- Current likely future homes for real M12 coverage:
  - trust-domain tests:
    - likely inline in `src-rust/crates/api/src/provider_types.rs` when `TrustDomain` lands
    - or inline in `src-rust/crates/query/src/provider_resolution.rs` if tested through fallback behavior
  - capability tests:
    - likely inline in `src-rust/crates/query/src/provider_resolution.rs`
    - existing inline test module already exists there at `src-rust/crates/query/src/provider_resolution.rs:254`
  - health-cache tests:
    - future `src-rust/crates/query/src/health_cache.rs`
  - session-budget tests:
    - future `src-rust/crates/query/src/session_budget.rs`
  - fallback-resolution tests:
    - likely inline in `src-rust/crates/query/src/provider_resolution.rs`
  - spawn-time integration tests:
    - likely inline in `src-rust/crates/query/src/agent_tool.rs`
    - existing inline integration-style test module already exists there at `src-rust/crates/query/src/agent_tool.rs:658`
    - `src-rust/crates/tools/src/team_tool.rs` currently shows no test module hits, so team-path integration tests are more naturally anchored in `agent_tool.rs` where the current team runner tests already live

## 11. Drift / hazards

- `[naming drift]` M10 plan says M11-05 may need `ModelRegistry::models_for_provider()`, but live code already has `ModelRegistry::list_by_provider()` at `src-rust/crates/api/src/model_registry.rs:321-327`.
- `[confirmed gap]` Team runner does not currently derive a parent-provider hint like direct `AgentTool::execute()` does; compare `src-rust/crates/query/src/agent_tool.rs:260-272` vs `:586-590`. This is not a blocker for recon, but it is a real spawn-path behavior difference that could matter when modifying the team path again.
- `[confirmed gap]` Team path currently has two cancellation layers that are not unified:
  - team orchestration token at `src-rust/crates/tools/src/team_tool.rs:367-372`
  - child query-loop token at `src-rust/crates/query/src/agent_tool.rs:636`
- `[confirmed gap]` All three child execution paths still default to `CHILD_AGENT_FALLBACK_MAX_TOKENS`:
  - direct child config at `src-rust/crates/query/src/agent_tool.rs:364`
  - background path via reused config at `src-rust/crates/query/src/agent_tool.rs:401`
  - team runner config at `src-rust/crates/query/src/agent_tool.rs:622`
- `[likely low-risk]` M11-08 `ToolContext` / `QueryConfig` additions have wider constructor fan-out than the primary files alone suggest, especially in inline tests and helper constructors.
- `[likely low-risk]` Existing root-loop `fallback_model` behavior at `src-rust/crates/query/src/lib.rs:1303-1323` is adjacent terminology that could confuse D2 spawn-time fallback work if code/comments are not explicit.
- `[file-path drift]` Authority docs under `docs/Current/` and `docs/Orchestrator_planning/` are readable but currently untracked in this checkout; execution should keep citing exact paths rather than assuming committed status.
- `[file-path drift]` There are no live `src-rust/crates/api/tests/` or `src-rust/crates/query/tests/` directories; M12 test work will likely remain inline-source-unit/integration style unless those trees are created intentionally.

## 12. Structural blockers

- None confirmed from this recon pass.
- No evidence was found that would justify stopping M11 on structural grounds.
- The strongest issues found are scope-contained hazards, not blockers:
  - team-path behavior drift
  - cancellation-surface duplication
  - stale M10 helper-gap assumption

## 13. Recommended next executable ticket

`TASK-M11-01`

Reason:
- This recon was read-only and does not change MPWO ordering.
- `docs/archive/reports/MPWO_REVISION_REPORT_20260414T003218Z.md:142-149` still makes `TASK-M11-01` the next executable ticket.
- The recon mainly reduces later risk for `TASK-M11-05`, `TASK-M11-08`, `TASK-M11-09`, and M12 coverage planning.

## 14. Exact evidence commands run

```text
git branch --show-current
git rev-parse HEAD
git log -1 --oneline
git status --short
git status --short --untracked-files=no
git diff --name-only -- src-rust
pwd
find . -maxdepth 2 -type f -name Cargo.toml
find src-rust/crates -maxdepth 2 -type d \( -name tests -o -path '*/src' \) | sort
find src-rust/crates/query/src -maxdepth 1 -type f | sort
find src-rust/crates/api/tests src-rust/crates/query/tests -maxdepth 2 -type f | sort
find docs/archive/provider_orchestrator -maxdepth 2 -type f | sort
find docs/archive/reports -maxdepth 1 -type f | sort | rg "D2|M11|M12|surrogate|provider_orchestrator|micro_patch|fallback|budget|health|trust"
date -u +%Y%m%dT%H%M%SZ
date -u +%Y-%m-%dT%H:%M:%SZ
rg -n "TASK-M11-05|TASK-M11-08|TASK-M11-09|TASK-M12-01|TASK-M12-02|TASK-M12-03|D2|hosted-Ollama|hosted Ollama" docs/Current/MPWO_WORK_ORDER_PACK.md docs/Orchestrator_planning/IMPLEMENTATION_PLAN_MPWO.md docs/Current/D1_REVIEW_REPORT_20260413T233604Z.md docs/Current/M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md docs/archive/reports/MPWO_REVISION_REPORT_20260414T003218Z.md
rg -n "TrustDomain|HealthCache|SessionBudget|resolve_provider_with_fallback|allow_fallback|budget_usd|max_tokens_override|WorkerProviderResolved|WorkerBudgetExceeded|SessionBudgetExceeded" src-rust
rg -n "struct AgentInput|fn spawn_background_agent|spawn_background|init_team_swarm_runner|struct ToolContext|ToolContext \{|struct QueryConfig|struct CostTracker|struct ModelRegistry|enum ProviderStatus|check_all_health|health_check|TeamCreateTool|fn execute\(|struct AgentRunParams|struct AgentSpec|CHILD_AGENT_FALLBACK_MAX_TOKENS|CancellationToken::new\(|provider_override|model_override|max_budget_usd|provider_registry|model_registry|resolve_provider_identity|materialize_provider|models_for_provider" src-rust/crates
rg -n "models_for_provider|list_by_provider|best_model_for_provider" src-rust/crates/api/src/model_registry.rs src-rust/crates/tui/src/model_picker.rs
rg -n "surrogate|spec-proxy|proxy|TrustDomain|HealthCache|SessionBudget|resolve_provider_with_fallback|allow_fallback|budget_usd|WorkerProviderResolved|WorkerBudgetExceeded|SessionBudgetExceeded|provider-aware|fallback" src-rust/crates/api/tests src-rust/crates/query/tests src-rust/crates/query/src docs/archive/reports docs/archive/provider_orchestrator
rg -n "provider_resolution|health_check|ProviderStatus|CostTracker|BudgetExceeded|fallback_model|TeamCreate|AgentTool|CancellationToken::new\(|list_by_provider|best_model_for_provider" src-rust/crates/query/src src-rust/crates/api/src src-rust/crates/core/tests src-rust/crates/tui/tests
rg -n "ToolContext \{" src-rust/crates
rg -n "QueryConfig \{" src-rust/crates/query/src src-rust/crates/cli/src
rg -n "check_all_health\(|health_check\(|capabilities\(" src-rust/crates/api/src src-rust/crates/query/src
rg -n "budget_usd|allow_fallback|max_tokens_override|session_budget|health_cache" docs/Current/M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md docs/Current/MPWO_WORK_ORDER_PACK.md
rg -n "unknown_custom_providers_default_to_cloud_and_enter_cloud_candidate_scope|cloud_fallback_candidate_ids|TestTrustDomain|trust_domain_for_provider\(|spec-proxy|surrogate" src-rust/crates/api/src/registry.rs src-rust/crates/api/src src-rust/crates/query/src
rg -n "QueryEvent|enum QueryEvent|WorkerProviderResolved|WorkerBudgetExceeded|SessionBudgetExceeded" src-rust/crates/query/src/lib.rs
rg -n "allow_fallback|budget_usd|max_tokens_override|max_tokens" src-rust/crates/query/src/agent_tool.rs src-rust/crates/tools/src/team_tool.rs
rg -n "max_budget_usd|budget-usd|fallback_model|CancellationToken::new\(|run_query_loop\(" src-rust/crates/cli/src/main.rs src-rust/crates/query/src/agent_tool.rs src-rust/crates/tools/src/team_tool.rs
rg -n "parent_provider|provider_override|TeamCreate|inherit|inherits|parent session|ctx\.config\.provider|openai|google|provider_registry" src-rust/crates/query/src/agent_tool.rs
rg -n "provider" docs/archive/reports/TASK-M8-08_EXECUTION_REPORT_20260412T151649Z.md docs/archive/reports/TASK-M8-09_COMMIT_VERIFICATION_REPORT_20260412T232152Z.md docs/archive/reports/TASK-M9-06_EXECUTION_REPORT_20260413T085936Z.md docs/archive/reports/TASK-M9-06_PREFLIGHT_REPORT_20260413T084627Z.md
rg -n "teamcreate|TeamCreate|parent_inherits|mixed_providers|provider_registry_some_resolution_failure|fallback_model" src-rust/crates/query/src/agent_tool.rs src-rust/crates/query/src/lib.rs
rg -n "#\[cfg\(test\)\]|tokio::test|#[ ]*test" src-rust/crates/api/src/model_registry.rs src-rust/crates/api/src/provider_types.rs src-rust/crates/query/src/agent_tool.rs src-rust/crates/query/src/provider_resolution.rs src-rust/crates/tools/src/team_tool.rs
sed -n '1,260p' docs/Current/MPWO_WORK_ORDER_PACK.md
sed -n '1,260p' docs/Orchestrator_planning/IMPLEMENTATION_PLAN_MPWO.md
sed -n '1,260p' docs/Current/M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md
sed -n '1,260p' docs/archive/reports/MPWO_REVISION_REPORT_20260414T003218Z.md
nl -ba docs/Current/D1_REVIEW_REPORT_20260413T233604Z.md | sed -n '145,180p'
nl -ba docs/Current/M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md | sed -n '390,590p'
nl -ba docs/Current/MPWO_WORK_ORDER_PACK.md | sed -n '1,160p'
nl -ba docs/archive/provider_orchestrator/d2_test_micro_patch_report.md | sed -n '1,240p'
nl -ba docs/archive/provider_orchestrator/d2_test_micro_patch_report.md | sed -n '240,340p'
nl -ba src-rust/Cargo.toml | sed -n '1,120p'
nl -ba src-rust/crates/api/src/model_registry.rs | sed -n '1,340p'
nl -ba src-rust/crates/api/src/model_registry.rs | sed -n '337,430p'
nl -ba src-rust/crates/api/src/provider.rs | sed -n '60,95p'
nl -ba src-rust/crates/api/src/provider_types.rs | sed -n '180,280p'
nl -ba src-rust/crates/api/src/registry.rs | sed -n '1,180p'
nl -ba src-rust/crates/api/src/registry.rs | sed -n '360,520p'
nl -ba src-rust/crates/cli/src/main.rs | sed -n '600,710p'
nl -ba src-rust/crates/cli/src/main.rs | sed -n '720,750p'
nl -ba src-rust/crates/cli/src/main.rs | sed -n '1108,1145p'
nl -ba src-rust/crates/core/src/lib.rs | sed -n '2850,2915p'
nl -ba src-rust/crates/query/src/agent_tool.rs | sed -n '120,470p'
nl -ba src-rust/crates/query/src/agent_tool.rs | sed -n '540,650p'
nl -ba src-rust/crates/query/src/agent_tool.rs | sed -n '660,760p'
nl -ba src-rust/crates/query/src/agent_tool.rs | sed -n '900,1040p'
nl -ba src-rust/crates/query/src/lib.rs | sed -n '30,80p'
nl -ba src-rust/crates/query/src/lib.rs | sed -n '70,160p'
nl -ba src-rust/crates/query/src/lib.rs | sed -n '391,445p'
nl -ba src-rust/crates/query/src/lib.rs | sed -n '860,930p'
nl -ba src-rust/crates/query/src/lib.rs | sed -n '1388,1410p'
nl -ba src-rust/crates/query/src/provider_resolution.rs | sed -n '90,220p'
nl -ba src-rust/crates/query/src/provider_resolution.rs | sed -n '250,360p'
nl -ba src-rust/crates/tools/src/lib.rs | sed -n '216,245p'
nl -ba src-rust/crates/tools/src/lib.rs | sed -n '530,595p'
nl -ba src-rust/crates/tools/src/team_tool.rs | sed -n '30,60p'
nl -ba src-rust/crates/tools/src/team_tool.rs | sed -n '60,110p'
nl -ba src-rust/crates/tools/src/team_tool.rs | sed -n '150,440p'
```

## 15. Files changed

- Report artifact only:
  - `docs/archive/reports/TASK-M11_D2_RECON_REPORT_20260414T012649Z.md`
- No source files were edited.
- No files were staged.
- No commits were made.

## 16. Verdict

`PASS-WITH-NOTES`

This read-only D2 recon is complete and execution-useful. The repo is still on the accepted D1 baseline with no tracked D2 implementation started. M11-05, M11-08, and M11-09 have clear live seams. The most important updates versus the M10 plan are:

- M11-05 does **not** appear blocked on a missing model-enumeration helper because `ModelRegistry::list_by_provider()` already exists.
- M11-08 must wire both query-loop cancel tokens and the separate TeamCreate per-agent cancel tokens.
- M11-09 still has three live child paths effectively pinned to `CHILD_AGENT_FALLBACK_MAX_TOKENS`.
- The historical M12 surrogate exists only as archived documentation, not as live tracked test code.
