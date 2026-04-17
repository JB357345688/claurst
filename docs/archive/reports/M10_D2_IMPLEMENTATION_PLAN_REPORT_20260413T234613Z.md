# M10 D2 Implementation Plan Report

## Timestamp UTC

`2026-04-13T23:46:13Z`

## Branch Observed

`feature/provider-resolution-seam`

HEAD: `6b362a0 TASK-M9-12 complete D1 provider-resolution seam validation and smoke gate`

## Authority Reviewed

- `AGENTS.md` -- full
- `docs/Current/MPWO_WORK_ORDER_PACK.md` -- M11 ticket definitions (sections 6 and 8), dependency graph, section 2A (Hosted Ollama invariant), global rules (section 9)
- `docs/Orchestrator_planning/IMPLEMENTATION_PLAN_MPWO.md` -- Milestone 10 planning guidance, Milestone 11 code targets, risk register
- `docs/archive/reports/D1_REVIEW_REPORT_20260413T233604Z.md` -- D1 accepted baseline

## D1 Baseline Accepted for Planning

D1 is review-accepted per `D1_REVIEW_REPORT_20260413T233604Z.md`. The provider-resolution seam is landed. Workers inherit parent providers. All four automated validation commands pass. The hosted-Ollama non-regression invariant is preserved.

**D1-established child independence (already working):**
- **Different parent/child models:** Already supported. `AgentInput.model` and `AgentSpec.model` allow specifying a child model distinct from the parent's. `AgentRunParams.model_override` propagates this into child `QueryConfig`. No D2 work is needed to enable this -- it is a D1 capability.
- **Different parent/child providers:** Already supported. `AgentInput.provider` and `AgentSpec.provider` allow specifying a child provider distinct from the parent's. `AgentRunParams.provider_override` propagates this into child provider resolution. No D2 work is needed to enable this -- it is a D1 capability.

**D1 interim measure that D2 must supersede:**
- `CHILD_AGENT_FALLBACK_MAX_TOKENS = 4_096` at `agent_tool.rs:132` -- a D1-safe interim fallback for spawned child agents. D2 replaces this with a first-class child `max_tokens` setting (see TASK-M11-09). The constant will be retained only as the backward-compatible default when no explicit child `max_tokens` is specified.

---

## 6. Repo Reality Verification

### 6.1 Core D1 Seam Functions

**`resolve_provider_identity()`** -- `provider_resolution.rs:101`
```rust
pub fn resolve_provider_identity(
    explicit_provider: Option<&str>,
    model: &str,
    model_registry: Option<&ModelRegistry>,
) -> Result<ProviderIdentity, ProviderResolutionError>
```
- Synchronous, pure function.
- Returns `ProviderIdentity { provider_id, model_id, resolution_source }`.
- Handles: explicit provider, model-string prefix, model-registry lookup, default-to-anthropic.
- **D2 assessment:** This function does not need modification for D2. `resolve_provider_with_fallback()` wraps it; it does not replace it.

**`materialize_provider()`** -- `provider_resolution.rs:157`
```rust
pub fn materialize_provider(
    identity: &ProviderIdentity,
    registry: &ProviderRegistry,
    provider_configs: &HashMap<String, ProviderConfig>,
) -> Result<ExecutionTarget, ProviderResolutionError>
```
- Synchronous (auth-store lookup is synchronous).
- Returns `ExecutionTarget { provider_id, model_id, provider: Arc<dyn LlmProvider>, resolution_source }`.
- Ollama special-casing at line 162: URL normalization, auth lookup.
- **D2 assessment:** This function does not need modification for D2. `resolve_provider_with_fallback()` calls it for each fallback candidate.

### 6.2 Query Loop and Config

**`run_query_loop()`** -- `lib.rs:675`
```rust
pub async fn run_query_loop(
    legacy_client: Option<&claurst_api::AnthropicClient>,
    messages: &mut Vec<Message>,
    tools: &[Box<dyn Tool>],
    tool_ctx: &ToolContext,
    config: &QueryConfig,
    cost_tracker: Arc<CostTracker>,
    event_tx: Option<mpsc::UnboundedSender<QueryEvent>>,
    cancel_token: tokio_util::sync::CancellationToken,
    mut pending_messages: Option<&mut Vec<String>>,
) -> QueryOutcome
```
- `legacy_client` is `Option<&AnthropicClient>` (M9-11 change).
- Already has budget guard at `lib.rs:1397`: checks `config.max_budget_usd` after each turn.
- Already has `QueryOutcome::BudgetExceeded` variant.
- **D2 assessment:** `SessionBudget` can integrate alongside the existing `max_budget_usd` mechanism. The existing mechanism is per-loop; `SessionBudget` is cross-session. D2 wiring adds `session_budget.check_and_cancel()` near the existing budget guard, and workers get child cancel tokens from `SessionBudget` instead of creating fresh `CancellationToken::new()`.

**`QueryConfig`** -- `lib.rs:78`

Current fields relevant to D2:
- `max_budget_usd: Option<f64>` (line 113) -- existing per-loop budget
- `fallback_model: Option<String>` (line 116) -- existing single-model fallback for overload/rate-limit
- `provider_registry: Option<Arc<ProviderRegistry>>` (line 121)

Fields D2 must add:
- `session_budget: Option<Arc<SessionBudget>>`

**No drift from MPWO planning expectations.**

### 6.3 ToolContext

`ToolContext` -- `tools/src/lib.rs:216`

Current fields relevant to D2:
- `provider_registry: Option<Arc<ProviderRegistry>>` (line 232)
- `model_registry: Option<Arc<ModelRegistry>>` (line 234)
- `cost_tracker: Arc<CostTracker>` (line 220)

Fields D2 must add:
- `session_budget: Option<Arc<SessionBudget>>`
- `health_cache: Option<Arc<HealthCache>>`

**No drift from MPWO planning expectations.**

### 6.4 AgentRunParams

`AgentRunParams` -- `team_tool.rs:37`

Current fields:
- `description`, `prompt`, `tools`, `system_prompt`, `max_turns`, `ctx: Arc<ToolContext>`, `provider_override: Option<String>`, `model_override: Option<String>`

Fields D2 must add:
- `allow_fallback: bool` (default false)
- `budget_usd: Option<f64>`
- `max_tokens_override: Option<u32>` (child execution override -- replaces `CHILD_AGENT_FALLBACK_MAX_TOKENS` interim)

**No drift from MPWO planning expectations.**

### 6.5 ProviderRegistry

`ProviderRegistry` -- `api/src/registry.rs:21`

- `providers: HashMap<ProviderId, Arc<dyn LlmProvider>>`
- Methods: `new()`, `register()`, `set_default()`, `get()`, `default_provider()`, `default_provider_id()`, `provider_ids()`, `check_all_health()`

**D2 sufficiency assessment:**
- **Candidate enumeration:** `provider_ids()` returns `Vec<&ProviderId>`, and `get()` returns `Option<&Arc<dyn LlmProvider>>`. This is sufficient to iterate all registered providers and filter by trust domain. **No new `providers_in_domain()` method is needed at the registry level** -- trust-domain filtering can be done in `resolve_provider_with_fallback()` by iterating `provider_ids()` and calling `TrustDomain::for_provider()`.
- **Health checking:** `check_all_health()` exists but does sequential network calls for all providers. For D2 fallback, we need per-provider cached health status. `HealthCache` wraps `check_all_health()` results with a TTL, and `resolve_provider_with_fallback()` consults the cache. The existing `health_check()` method on `LlmProvider` trait (`provider.rs:80`) is the per-provider probe point.
- **Capability access:** `LlmProvider::capabilities()` returns `ProviderCapabilities` (static, no network call). This is the provider-level capability source. Model-level capabilities come from `ModelEntry`.

### 6.6 ModelRegistry and ModelEntry

`ModelEntry` -- `model_registry.rs:24`

Current capability fields:
- `tool_calling: bool`
- `reasoning: bool`
- `vision: bool`
- `family: Option<String>`

Fields D2 must add:
- `pdf_input: Option<bool>`
- `audio_input: Option<bool>`
- `structured_output: Option<bool>`
- `max_output_tokens: Option<u32>`

`ModelRegistry` -- `model_registry.rs:50`
- `find_provider_for_model()` at line 261: resolves bare model name to provider ID.
- No `entries()` or `iter()` method visible for enumerating all models by provider.

**D2 assessment:** For fallback model selection (family match within alternative provider), `resolve_provider_with_fallback()` needs to enumerate models for a given provider. Currently there is no `models_for_provider()` method. **Planning note:** M11-05 may need a small addition to `ModelRegistry` (e.g., `models_for_provider(provider_id) -> Vec<&ModelEntry>`) or the fallback logic can iterate the internal `entries` HashMap directly if it is exposed. This is a minor risk -- see Risk Register.

### 6.7 CostTracker

`CostTracker` -- `core/src/lib.rs:2850`

Current fields:
- `input_tokens: AtomicU64`
- `output_tokens: AtomicU64`
- `cache_creation_tokens: AtomicU64`
- `cache_read_tokens: AtomicU64`
- `pricing: parking_lot::RwLock<ModelPricing>`

D2 adds:
- `agent_id: Option<String>`
- `provider_id: Option<String>`

**D2 assessment:** Adding two optional fields to `CostTracker` is straightforward. The `#[derive(Debug, Default)]` will need manual handling for the new fields (they default to `None`). Construction sites pass `CostTracker::new()`, so the new fields must be initialized there.

### 6.8 QueryEvent

`QueryEvent` -- `lib.rs:391`

Current variants: `Stream`, `ToolStart`, `ToolEnd`, `TurnComplete`, `Status`, `Error`, `TokenWarning`.

D2 adds: `WorkerProviderResolved`, `WorkerBudgetExceeded`, `SessionBudgetExceeded`.

**No structural conflict.**

### 6.9 provider_types.rs

`provider_types.rs` -- `api/src/provider_types.rs`

Current contents: `StopReason`, `ProviderRequest`, `ProviderResponse`, `StreamEvent`, `ProviderCapabilities`, `ProviderStatus`, `AuthMethod`, `ApiKeyHeader`, `SystemPromptStyle`.

**Critical finding for D2:** `ProviderCapabilities` already has these fields:
- `tool_calling: bool`
- `thinking: bool` (maps to `Reasoning` capability)
- `image_input: bool` (maps to `Vision`)
- `pdf_input: bool`
- `audio_input: bool`
- `structured_output: bool`

This means the provider-level capability data for D2's `Capability` enum matching is **already present** in the trait-level API. The `model_supports_capability()` function can use `ModelEntry` fields first, then fall back to `ProviderCapabilities` for unknown-data policy.

`TrustDomain` will be added to this file as specified by M11-01.

### 6.10 Cancellation Token Usage

Current state in `agent_tool.rs`:
- Foreground agent: `CancellationToken::new()` at line 454
- Background agent: `CancellationToken::new()` at line 408
- Team runner: `CancellationToken::new()` at line 636

All three create fresh, unconnected tokens. D2 will make these child tokens of `SessionBudget`'s root token when a session budget exists.

`tokio-util` is already a dependency of `claurst-query` (`Cargo.toml:24`). `CancellationToken::child_token()` is part of `tokio-util::sync`, so no new dependency is needed.

### 6.11 DashMap Availability

`dashmap` is already a workspace dependency and is already in `claurst-query`'s `Cargo.toml:26`. No new dependency needed for `HealthCache`.

### 6.12 Module Structure

`claurst-query` current module declarations (`lib.rs:10-19`):
```
agent_tool, auto_dream, away_summary, command_queue, compact,
context_analyzer, coordinator, cron_scheduler, provider_resolution,
session_memory, skill_prefetch
```

D2 adds two new modules: `health_cache`, `session_budget`.

---

## 7. M10 Planning Verdict

**PASS-WITH-NOTES**

The D1 seam is well-structured for D2 extension. The existing `ProviderRegistry`, `LlmProvider` trait, `ProviderCapabilities`, `ProviderStatus`, and `health_check()` APIs provide nearly all the infrastructure D2 needs. The MPWO M11 ticket breakdown is sound and largely aligned with repo reality.

Notes:
1. `ModelRegistry` may need a small helper for enumerating models by provider (for fallback model selection in M11-05). See Risk R1.
2. `ProviderCapabilities` already covers the capability fields that M11-03 needs to match against. The `Capability` enum can map directly to `ProviderCapabilities` fields as a fallback when `ModelEntry` returns `None`.
3. Three independent budget/limit mechanisms coexist and must not be conflated: `max_tokens` (token count for API response length), `max_budget_usd` (per-loop USD, existing D1), `SessionBudget` (cross-all-loops USD, new D2).
4. Parent/child model and provider independence is already a D1 capability. D2 extends child independence with `max_tokens`, `budget_usd`, and `allow_fallback` as first-class spawn-time settings (TASK-M11-09, TASK-M11-10).

---

## 8. Real D2 Seam Assessment

### Where D2 extends the D1 seam

| Extension point | D2 addition | File |
|---|---|---|
| `provider_resolution.rs` | `resolve_provider_with_fallback()` wrapping `resolve_provider_identity()` + `materialize_provider()` | `crates/query/src/provider_resolution.rs` |
| `provider_resolution.rs` | `Capability` enum + `model_supports_capability()` | `crates/query/src/provider_resolution.rs` |
| `provider_types.rs` | `TrustDomain` enum + `for_provider()` | `crates/api/src/provider_types.rs` |
| `model_registry.rs` | 4 new `Option` fields on `ModelEntry` | `crates/api/src/model_registry.rs` |
| New file | `HealthCache` with DashMap + TTL | `crates/query/src/health_cache.rs` |
| New file | `SessionBudget` with budget tracking + cancel token | `crates/query/src/session_budget.rs` |
| `lib.rs` (query) | 3 new `QueryEvent` variants + `session_budget.check_and_cancel()` insertion | `crates/query/src/lib.rs` |
| `lib.rs` (tools) | `session_budget` + `health_cache` fields on `ToolContext` | `crates/tools/src/lib.rs` |
| `agent_tool.rs` | Wire `allow_fallback` + `budget_usd` through spawn paths; child cancel tokens from `SessionBudget` | `crates/query/src/agent_tool.rs` |
| `agent_tool.rs` | `AgentInput.max_tokens: Option<u32>` + wire child `QueryConfig.max_tokens` from override; supersede `CHILD_AGENT_FALLBACK_MAX_TOKENS` | `crates/query/src/agent_tool.rs` |
| `team_tool.rs` | `allow_fallback` + `budget_usd` + `max_tokens` on `AgentSpec` + `AgentRunParams` | `crates/tools/src/team_tool.rs` |
| `main.rs` | Create `SessionBudget` at root; pass `health_cache` to `ToolContext` | `crates/cli/src/main.rs` |
| `core/src/lib.rs` | `agent_id` + `provider_id` on `CostTracker` | `crates/core/src/lib.rs` |

### What must stay unchanged

1. `resolve_provider_identity()` -- signature and behavior unchanged.
2. `materialize_provider()` -- signature and behavior unchanged. Ollama special-casing at line 162 must remain.
3. `run_query_loop()` -- `legacy_client: Option<&AnthropicClient>` parameter preserved. Legacy compatibility path (the `else` branch when `provider_registry` is `None`) preserved.
4. `CHILD_AGENT_FALLBACK_MAX_TOKENS = 4_096` -- D2 supersedes this interim constant with first-class child `max_tokens` fields (TASK-M11-09). The constant value (`4096`) is retained as the backward-compatible default when no explicit child `max_tokens` is specified. The constant itself may be removed or kept as a named default -- M11-09 decides.
5. Hosted Ollama compatibility: `normalize_ollama_api_base()`, `AuthStore::load().api_key_for(ProviderId::OLLAMA)`, environment-first precedence.
6. All existing D1 tests must continue to pass.

### What is risky or ambiguous

1. **Model enumeration for fallback:** `ModelRegistry` does not expose a `models_for_provider()` method. The fallback model-selection step ("family match -> provider default -> skip") needs this.
2. **`HealthCache` probe latency at spawn time:** If all cached health entries are stale, `resolve_provider_with_fallback()` must probe sequentially. This adds latency proportional to the number of same-domain providers.
3. **`SessionBudget` vs existing `max_budget_usd` vs `max_tokens`:** Three independent mechanisms. `max_tokens` is a token count (API response length). `max_budget_usd` is per-loop USD (already in D1). `SessionBudget` is cross-all-loops USD with a shared `CancellationToken`. The interaction must be clearly defined: `max_budget_usd` aborts the individual loop; `SessionBudget` cancels the root token, which propagates to all child tokens. `max_tokens` is per-API-call and does not interact with either budget.
4. **Child `max_tokens` transition (M11-09):** Three spawn paths in `agent_tool.rs` must all be updated consistently. If one path is missed, it silently falls back to `CHILD_AGENT_FALLBACK_MAX_TOKENS`, which is correct but hides the regression. See Risk R6.

---

## 9. Exact D2 Landing Plan for M11

### TASK-M11-01 -- TrustDomain enum

**Purpose:** Define trust-domain classification for provider IDs.

**Exact file targets:**
- `src-rust/crates/api/src/provider_types.rs`

**What to implement:**
1. Add `TrustDomain` enum with `Local` and `Cloud` variants, deriving `Debug, Clone, Copy, PartialEq, Eq`.
2. Add `TrustDomain::for_provider(provider_id: &str) -> TrustDomain`:
   - `"ollama" | "lmstudio" | "llamacpp"` -> `Local`
   - All others -> `Cloud`
3. No config system. Hardcoded match only.

**Prerequisites:** M10 (this plan).

**Validation:** `cd src-rust && cargo check -p claurst-api`

**Non-goals / forbidden scope:**
- No YAML/env-var configuration for trust domains.
- No custom-provider trust-domain support beyond `Cloud` default.
- Do not modify any other file.
- Do not modify existing types in `provider_types.rs`.

---

### TASK-M11-02 -- ModelEntry extension

**Purpose:** Add D2 capability fields to `ModelEntry`.

**Exact file targets:**
- `src-rust/crates/api/src/model_registry.rs`

**What to implement:**
1. Add to `ModelEntry` struct (after existing `vision: bool` at ~line 39):
   - `pdf_input: Option<bool>`
   - `audio_input: Option<bool>`
   - `structured_output: Option<bool>`
   - `max_output_tokens: Option<u32>`
2. All fields default to `None`. Use `#[serde(default)]` on each.
3. Update any `ModelEntry` construction in the bundled snapshot to include the new fields as `None` (or rely on serde `default`).

**Prerequisites:** M10 (this plan).

**Validation:** `cd src-rust && cargo check -p claurst-api && cargo test -p claurst-api`

**Non-goals / forbidden scope:**
- Do not change existing fields.
- Do not add required fields.
- Do not modify `ModelRegistry` methods.

---

### TASK-M11-03 -- Capability enum and matching

**Purpose:** Define capability matching logic for D2 fallback filtering.

**Exact file targets:**
- `src-rust/crates/query/src/provider_resolution.rs`

**What to implement:**
1. Add `Capability` enum: `ToolCalling`, `Reasoning`, `Vision`, `PdfInput`, `AudioInput`, `StructuredOutput`.
2. Add `model_supports_capability(entry: &ModelEntry, cap: &Capability) -> Option<bool>`:
   - For `ToolCalling`, `Reasoning`, `Vision`: read from `ModelEntry`'s existing `bool` fields -> always `Some(true/false)`.
   - For `PdfInput`, `AudioInput`, `StructuredOutput`: read from `ModelEntry`'s new `Option<bool>` fields -> return the `Option` directly (`None` = unknown -> ineligible for fallback).
3. Add `provider_supports_capability(caps: &ProviderCapabilities, cap: &Capability) -> bool`:
   - Maps `Capability` variant to the corresponding `ProviderCapabilities` field. This is the fallback data source when `ModelEntry` returns `None`.
4. Default required capabilities: `[ToolCalling]` -- a module-level constant, not configurable.

**Prerequisites:** M11-02 (for `ModelEntry` new fields).

**Validation:** `cd src-rust && cargo check -p claurst-query && cargo test -p claurst-query -- capability`

**Non-goals / forbidden scope:**
- No configuration system for required capabilities.
- Do not modify `resolve_provider_identity()` or `materialize_provider()`.
- Do not modify `ProviderCapabilities` (it already has all needed fields).

---

### TASK-M11-04 -- HealthCache implementation

**Purpose:** Cache provider health status with TTL to avoid per-spawn probe latency.

**Exact file targets:**
- `src-rust/crates/query/src/health_cache.rs` (new file)
- `src-rust/crates/query/src/lib.rs` (add `mod health_cache; pub use health_cache::*;`)

**What to implement:**
1. Create `health_cache.rs`.
2. `HealthCache` struct with:
   - `cache: DashMap<String, (ProviderStatus, Instant)>` (key is provider ID string)
   - `ttl: Duration` (hardcoded 30s constant)
3. `get(provider_id: &str) -> Option<ProviderStatus>`: returns cached status if `Instant::elapsed() < ttl`.
4. `insert(provider_id: &str, status: ProviderStatus)`: stores status with current `Instant`.
5. `probe_if_stale(provider_id: &str, provider: &dyn LlmProvider) -> ProviderStatus`:
   - If cache hit within TTL, return cached.
   - Otherwise call `provider.health_check()` with a 5s timeout (`tokio::time::timeout`).
   - On timeout/error, return `ProviderStatus::Unavailable`.
   - Update cache.
6. `new() -> Self`: creates cache with empty DashMap and 30s TTL.

**Prerequisites:** M10 (this plan). Independent of M11-01/02/03.

**Validation:** `cd src-rust && cargo test -p claurst-query -- health_cache` -- unit tests for cache hit, miss, expiry, timeout.

**Non-goals / forbidden scope:**
- No complex retry logic.
- No backoff.
- No periodic background refresh.
- Probe timeout hardcoded to 5s.

---

### TASK-M11-05 -- resolve_provider_with_fallback()

**Purpose:** Core D2 fallback resolution function.

**Exact file targets:**
- `src-rust/crates/query/src/provider_resolution.rs`

**What to implement:**
1. Add `resolve_provider_with_fallback()`:
   ```rust
   pub async fn resolve_provider_with_fallback(
       explicit_provider: Option<&str>,
       model: &str,
       model_registry: Option<&ModelRegistry>,
       provider_registry: &ProviderRegistry,
       provider_configs: &HashMap<String, ProviderConfig>,
       health_cache: &HealthCache,
       allow_fallback: bool,
   ) -> Result<ExecutionTarget, ProviderResolutionError>
   ```
2. Step 1: Call `resolve_provider_identity()` + `materialize_provider()`.
3. Step 2: If resolution succeeds, return.
4. Step 3: If resolution fails and `allow_fallback == false`, return error with suggestion message ("try allow_fallback: true").
5. Step 4: If `allow_fallback == true`:
   a. Determine primary provider's trust domain via `TrustDomain::for_provider()`.
   b. Enumerate all registered providers in same trust domain: iterate `provider_registry.provider_ids()`, filter by `TrustDomain::for_provider(id) == primary_domain`.
   c. For each candidate (ordered Healthy > Degraded, skip Unavailable): call `health_cache.probe_if_stale()`.
   d. For each healthy/degraded candidate: check capability match (at least `[ToolCalling]`).
   e. For capability check: get `ModelEntry` for candidate's model, call `model_supports_capability()`, fall back to `provider_supports_capability()` if model data is `None`.
   f. Model selection within fallback: family match first (same `family` as original model), then provider default, then skip.
   g. Call `materialize_provider()` for the selected candidate.
6. Cross-domain fallback is forbidden. No code path crosses trust domain boundary.

**Prerequisites:** M11-01 (TrustDomain), M11-03 (Capability matching), M11-04 (HealthCache).

**Planning note -- ModelRegistry gap:** Fallback model selection (step 4f) requires enumerating models for a given provider. If `ModelRegistry` does not expose `models_for_provider()`, this ticket must either:
- Add a minimal `models_for_provider(provider_id: &str) -> Vec<&ModelEntry>` to `ModelRegistry` (preferred, ~10 lines), or
- Use `provider_registry.get(id)` -> `provider.list_models()` but this is async/network and defeats the cache purpose.

The preferred approach is a small `ModelRegistry` helper. This is a targeted 10-line addition, not a registry redesign. It should be scoped into M11-05's file targets as: `src-rust/crates/api/src/model_registry.rs` (addition only).

**Validation:** `cd src-rust && cargo test -p claurst-query -- fallback` -- unit tests: same-domain fallback succeeds, cross-domain prohibited, `allow_fallback: false` returns error with suggestion.

**Non-goals / forbidden scope:**
- No `allow_cross_domain_fallback` parameter.
- No policy engine.
- No per-call fallback (spawn-time only).
- Do not modify `resolve_provider_identity()` or `materialize_provider()` signatures.

---

### TASK-M11-06 -- CostTracker extension

**Purpose:** Add agent/provider attribution to cost tracking.

**Exact file targets:**
- `src-rust/crates/core/src/lib.rs` (in the `CostTracker` section around line 2850)

**What to implement:**
1. Add to `CostTracker`:
   - `agent_id: parking_lot::RwLock<Option<String>>`
   - `provider_id: parking_lot::RwLock<Option<String>>`
2. Update `CostTracker::new()` to initialize both as `None`.
3. Add setters: `set_agent_id(&self, id: String)`, `set_provider_id(&self, id: String)`.
4. Workers call these setters when constructing their `CostTracker`.

**Prerequisites:** M10 (this plan). Independent of M11-01 through M11-05.

**Validation:** `cd src-rust && cargo check --workspace`

**Non-goals / forbidden scope:**
- Do not change cost calculation logic.
- Do not add new cost events.
- Do not modify existing `CostTracker` fields.

---

### TASK-M11-07 -- SessionBudget implementation

**Purpose:** Cross-session budget tracking with cancellation.

**Exact file targets:**
- `src-rust/crates/query/src/session_budget.rs` (new file)
- `src-rust/crates/query/src/lib.rs` (add `mod session_budget; pub use session_budget::*;`)

**What to implement:**
1. Create `session_budget.rs`.
2. `SessionBudget` struct:
   - `budget_usd: f64`
   - `spent: parking_lot::Mutex<f64>` (or `AtomicU64` with f64 bit-casting)
   - `root_token: CancellationToken`
3. `new(budget_usd: f64) -> Self`: creates budget with zero spent and a new root token.
4. `record_cost(&self, cost_usd: f64)`: adds cost to spent.
5. `check_and_cancel(&self)`: if `spent >= budget_usd`, cancel root token.
6. `child_cancel_token(&self) -> CancellationToken`: returns `self.root_token.child_token()`.
7. `is_cancelled(&self) -> bool`: returns `self.root_token.is_cancelled()`.

**Prerequisites:** M10 (this plan). Independent of M11-01 through M11-05.

**Validation:** `cd src-rust && cargo test -p claurst-query -- session_budget` -- unit tests for budget check + cancel + child token propagation.

**Non-goals / forbidden scope:**
- No per-provider budgets.
- No persistent storage.
- No complex accounting.

---

### TASK-M11-08 -- Budget + cancellation wiring

**Purpose:** Wire `SessionBudget` into root session and worker spawn paths.

**Exact file targets:**
- `src-rust/crates/cli/src/main.rs`
- `src-rust/crates/query/src/agent_tool.rs`
- `src-rust/crates/query/src/lib.rs`
- `src-rust/crates/tools/src/lib.rs` (ToolContext)
- `src-rust/crates/tools/src/team_tool.rs`

**What to implement:**
1. In `main.rs`: create `SessionBudget` when `--budget-usd` CLI flag is set. Pass `Arc<SessionBudget>` into `QueryConfig` and `ToolContext`.
2. Add `session_budget: Option<Arc<SessionBudget>>` to `QueryConfig` (lib.rs).
3. Add `session_budget: Option<Arc<SessionBudget>>` and `health_cache: Option<Arc<HealthCache>>` to `ToolContext` (tools/src/lib.rs).
4. In `run_query_loop()` (lib.rs): after existing budget guard at ~line 1397, add `if let Some(ref sb) = config.session_budget { sb.record_cost(turn_cost); sb.check_and_cancel(); }`.
5. In `agent_tool.rs`: when creating child `CancellationToken`, use `session_budget.child_cancel_token()` if session budget exists, otherwise `CancellationToken::new()` as today. Apply at all three spawn points (foreground line 454, background line 408, team runner line 636).
6. In `team_tool.rs`: per-agent cancel tokens become children of session budget root token (via `AgentRunParams.ctx.session_budget`).

**Prerequisites:** M11-07 (SessionBudget exists).

**Validation:** `cd src-rust && cargo check --workspace`

**Non-goals / forbidden scope:**
- No per-agent budgets.
- Do not change `legacy_client: Option<&AnthropicClient>` on `run_query_loop()`.
- Do not remove the existing `max_budget_usd` per-loop budget mechanism.
- Do not modify `CHILD_AGENT_FALLBACK_MAX_TOKENS` -- child `max_tokens` override wiring is M11-09's scope.

---

### TASK-M11-09 -- Child execution override wiring (max_tokens + spawn-time settings)

**Purpose:** Make child `max_tokens` a first-class D2 setting, replacing the `CHILD_AGENT_FALLBACK_MAX_TOKENS` interim. Ensure children can have independent execution settings (model, max_tokens, budget, fallback permission) passed through all three spawn paths.

**Exact file targets:**
- `src-rust/crates/query/src/agent_tool.rs`
- `src-rust/crates/tools/src/team_tool.rs`

**What to implement:**

1. Add to `AgentInput` (agent_tool.rs):
   - `max_tokens: Option<u32>` (absent = backward-compatible D1 default of `4096`)

2. Add to `AgentSpec` (team_tool.rs):
   - `max_tokens: Option<u32>` (absent = backward-compatible D1 default of `4096`)

3. Add to `AgentRunParams` (team_tool.rs):
   - `max_tokens_override: Option<u32>` (resolved from `AgentSpec.max_tokens` or `AgentInput.max_tokens`)

4. Wire child `QueryConfig.max_tokens` from override in all three spawn paths:
   - **Foreground agent** (agent_tool.rs ~line 454): `config.max_tokens = input.max_tokens.unwrap_or(CHILD_AGENT_FALLBACK_MAX_TOKENS)`
   - **Background agent** (agent_tool.rs ~line 408): same pattern
   - **Team runner** (agent_tool.rs ~line 636 / team_tool.rs): `config.max_tokens = params.max_tokens_override.unwrap_or(CHILD_AGENT_FALLBACK_MAX_TOKENS)`

5. Update JSON schemas in `input_schema()` for `AgentTool` and `TeamCreateTool` to expose `max_tokens` as an optional integer field.

6. `CHILD_AGENT_FALLBACK_MAX_TOKENS` constant: retain as the named default value (used in `unwrap_or` above). The constant is no longer the sole code path -- it is the backward-compatible fallback when `max_tokens` is not explicitly set.

**Backward compatibility guarantee:**
- When `max_tokens` is absent/`None` in `AgentInput`, `AgentSpec`, or `AgentRunParams`, behavior is identical to D1: child gets `4096`.
- Existing callers that do not set `max_tokens` see zero behavioral change.

**Child execution independence summary (what is first-class after this ticket):**

| Setting | Parent field | Child field | D1 status | D2 status |
|---|---|---|---|---|
| Model | `QueryConfig.model` | `AgentInput.model` / `AgentSpec.model` | Already independent (D1) | No change needed |
| Provider | `QueryConfig.provider` | `AgentInput.provider` / `AgentSpec.provider` | Already independent (D1) | No change needed |
| `max_tokens` | `QueryConfig.max_tokens` | `AgentInput.max_tokens` / `AgentSpec.max_tokens` | Interim `4096` fallback | **First-class (this ticket)** |
| `allow_fallback` | N/A (root has no fallback) | `AgentInput.allow_fallback` / `AgentSpec.allow_fallback` | Not supported | First-class (M11-10) |
| `budget_usd` | `QueryConfig.max_budget_usd` | `AgentInput.budget_usd` / `AgentSpec.budget_usd` | Not supported | First-class (M11-10) |

**Prerequisites:** M11-08 (budget + cancellation wiring exists, spawn paths are modified).

**Validation:** `cd src-rust && cargo check --workspace && cargo test -p claurst-query -- agent_tool`

**Non-goals / forbidden scope:**
- Do not finalize parent/child token policy beyond making child `max_tokens` settable.
- Do not change parent `max_tokens` resolution.
- Do not add dynamic per-model `max_tokens` negotiation.
- Do not remove `CHILD_AGENT_FALLBACK_MAX_TOKENS` -- it remains as the named default constant.

---

### TASK-M11-10 -- Schema updates (allow_fallback, budget_usd)

**Purpose:** Expose remaining D2 controls in Agent/TeamCreate input schemas. Note: child `max_tokens` schema was already added in M11-09; this ticket adds fallback and budget controls.

**Exact file targets:**
- `src-rust/crates/query/src/agent_tool.rs`
- `src-rust/crates/tools/src/team_tool.rs`

**What to implement:**
1. Add to `AgentInput` (agent_tool.rs):
   - `allow_fallback: Option<bool>` (default `None` -> treated as `false`)
   - `budget_usd: Option<f64>`
2. Add to `AgentSpec` (team_tool.rs):
   - `allow_fallback: Option<bool>` (default `None` -> treated as `false`)
   - `budget_usd: Option<f64>`
3. Add to `AgentRunParams` (team_tool.rs):
   - `allow_fallback: bool` (resolved from `Option`)
   - `budget_usd: Option<f64>`
4. Update JSON schemas in `input_schema()` for `AgentTool` and `TeamCreateTool` to include `allow_fallback` and `budget_usd`.
5. Wire `allow_fallback` through to `resolve_provider_with_fallback()` in the spawn paths.
6. Wire `budget_usd` through to `SessionBudget` construction for sub-agents (or pass existing session budget if no per-agent budget).

**Prerequisites:** M11-05 (resolve_provider_with_fallback exists), M11-08 (budget wiring exists), M11-09 (child execution override wiring exists -- spawn-path modification pattern established).

**Validation:** `cd src-rust && cargo check --workspace`

**Non-goals / forbidden scope:**
- `allow_fallback` defaults to `false`. Pinned behavior is default.
- Do not add `allow_cross_domain_fallback`.
- Do not modify `max_tokens` wiring -- that is M11-09's scope.

---

### TASK-M11-11 -- QueryEvent variants

**Purpose:** Add D2 observability events.

**Exact file targets:**
- `src-rust/crates/query/src/lib.rs`

**What to implement:**
1. Add to `QueryEvent` enum:
   ```rust
   WorkerProviderResolved {
       agent_id: String,
       provider_id: String,
       model_id: String,
       was_fallback: bool,
   },
   WorkerBudgetExceeded {
       agent_id: String,
       cost_usd: f64,
       limit_usd: f64,
   },
   SessionBudgetExceeded {
       cost_usd: f64,
       limit_usd: f64,
   },
   ```
2. Emit `WorkerProviderResolved` after successful provider resolution in agent spawn paths.
3. Emit `WorkerBudgetExceeded` when a worker's per-loop budget is exceeded.
4. Emit `SessionBudgetExceeded` when `SessionBudget::check_and_cancel()` triggers.

**Prerequisites:** M11-08 (budget wiring exists, spawn paths are updated), M11-09 (child execution override wiring exists).

**Validation:** `cd src-rust && cargo check --workspace`

**Non-goals / forbidden scope:**
- No UI rendering for these events (separate concern, not D2 scope).

---

### TASK-M11-12 -- D2 test suite + workspace validation

**Purpose:** Write D2 acceptance tests and validate entire workspace.

**Exact file targets:**
- `src-rust/crates/query/src/provider_resolution.rs` (trust-domain + capability + fallback tests)
- `src-rust/crates/query/src/health_cache.rs` (cache tests)
- `src-rust/crates/query/src/session_budget.rs` (budget tests)
- `src-rust/crates/query/src/agent_tool.rs` (child execution override tests + integration tests)

**What to implement:**
1. Trust-domain classification tests: local providers classified as Local, cloud as Cloud.
2. Capability matching tests: known fields -> `Some`, unknown -> `None`, fallback to `ProviderCapabilities`.
3. Health cache tests: TTL hit, miss, expiry, timeout handling.
4. Session budget tests: threshold check, cancel propagation, child token behavior.
5. Fallback resolution tests:
   - Same-domain fallback succeeds.
   - Cross-domain fallback prohibited (Local -> Cloud forbidden, Cloud -> Local forbidden).
   - `allow_fallback: false` returns error with suggestion.
   - Health ordering: Healthy > Degraded > skip Unavailable.
6. Child execution override tests:
   - Explicit `max_tokens` propagated to child `QueryConfig`.
   - Absent `max_tokens` defaults to `4096` (backward compatibility).
   - `budget_usd` propagated to child `SessionBudget`.
   - `allow_fallback` propagated to child provider resolution.
7. Full workspace validation:
   - `cargo fmt --all -- --check`
   - `cargo build --workspace`
   - `cargo test --workspace`
   - `cargo clippy --workspace --all-targets`

**Prerequisites:** M11-01 through M11-11 all complete.

**Validation:** All four workspace validation commands pass.

**Non-goals / forbidden scope:**
- Mark genuinely infeasible tests (requiring live provider network) as `#[ignore]` with explanation.
- Do not start M12 work.

---

## 10. Required D2 Invariants

1. **Same-domain-only fallback:** `resolve_provider_with_fallback()` must never cross trust domain boundaries. No code path, no config escape, no `allow_cross_domain_fallback`.

2. **No cross-domain fallback:** Local-pinned worker never falls back to Cloud. Cloud-pinned worker never falls back to Local. Enforced at code level in `resolve_provider_with_fallback()`.

3. **D1 behavior preserved when D2 fields are absent:** All new D2 fields are `Option` or default to "D1 behavior":
   - `allow_fallback` absent/None -> `false` -> no fallback -> D1 behavior.
   - `budget_usd` absent/None -> no session budget -> D1 behavior.
   - `session_budget` absent/None -> workers create fresh `CancellationToken::new()` -> D1 behavior.
   - `health_cache` absent/None -> `resolve_provider_with_fallback()` probes inline -> acceptable degraded behavior.
   - New `ModelEntry` fields `None` -> ineligible for fallback model selection but existing model resolution unchanged.

4. **Hosted Ollama non-regression preserved:**
   - `normalize_ollama_api_base()` untouched.
   - `AuthStore::load().api_key_for(ProviderId::OLLAMA)` untouched.
   - Environment-first precedence for Ollama auth untouched.
   - `TrustDomain::for_provider("ollama")` returns `Local` -- consistent with Ollama being a local provider.
   - Ollama special-casing in `materialize_provider()` at `provider_resolution.rs:162` untouched.

5. **Legacy compatibility path preserved:** `run_query_loop(legacy_client: Option<&AnthropicClient>, ...)` parameter unchanged. The `else` branch when `provider_registry` is `None` unchanged.

6. **Child `max_tokens` backward compatibility:** When `AgentInput.max_tokens`, `AgentSpec.max_tokens`, or `AgentRunParams.max_tokens_override` is absent/`None`, child behavior is identical to D1: `max_tokens` defaults to `4096` (the value of `CHILD_AGENT_FALLBACK_MAX_TOKENS`). Callers that do not set `max_tokens` see zero behavioral change.

7. **Child execution independence preserved:** Parent and child can already use different models and providers (D1 capability). D2 extends this with independent `max_tokens`, `budget_usd`, and `allow_fallback` -- all optional, all defaulting to D1 behavior when absent.

8. **USD budget vs token budget distinction:** `SessionBudget` tracks USD spend across all query loops (cross-session). `max_budget_usd` in `QueryConfig` tracks USD per individual loop (existing D1). `max_tokens` on `QueryConfig` is a token count limit for API response length. These three are independent mechanisms that never conflate with each other.

---

## 11. Risk Register

| ID | Risk | Impact | Likelihood | Mitigation |
|---|---|---|---|---|
| R1 | `ModelRegistry` lacks `models_for_provider()` -- fallback model selection in M11-05 cannot enumerate candidate models by provider | M11-05 blocked or requires inline workaround | High (confirmed: no such method exists today) | M11-05 adds a minimal ~10-line `models_for_provider()` helper to `model_registry.rs`. This is a targeted addition, not a registry redesign. Alternative: iterate all entries with string-prefix matching on `"provider_id/"` keys, but this is fragile. |
| R2 | `HealthCache` probe latency at spawn time when cache is cold | User-perceived delay when spawning agents with fallback in environments with many same-domain providers | Low (most environments have 1-3 same-domain providers) | TTL of 30s means subsequent spawns within 30s hit cache. First-spawn probe is bounded by 5s timeout per provider. |
| R3 | `SessionBudget` vs `max_budget_usd` interaction confusion | Developer confusion about which budget mechanism applies when | Medium | M11-08 must clearly document: `max_budget_usd` is per-loop (already exists, D1), `SessionBudget` is cross-all-loops (D2). Both fire independently. `SessionBudget` additionally cancels child tokens. |
| R4 | `CostTracker` adding `RwLock` fields may conflict with `#[derive(Default)]` | Compilation failure | Low | Manually implement required defaults or use `parking_lot::RwLock::new(None)` in `new()`. |
| R5 | Test nondeterminism from shared test-only auth lock (existing D1 issue) may resurface with D2's async health probing tests | Flaky tests | Medium | D2 health cache tests should use mock providers with deterministic responses, not real auth store. Reuse the `with_isolated_provider_auth()` pattern from D1 for any tests that touch auth. |
| R6 | Child `max_tokens` transition: M11-09 modifies all three spawn paths in `agent_tool.rs` to read `max_tokens` from `AgentInput` / `AgentRunParams`. If any spawn path is missed, that path silently continues using the `CHILD_AGENT_FALLBACK_MAX_TOKENS` constant, which is correct but hides the regression | Silent behavioral inconsistency across spawn paths | Medium | M11-12 must include a test that sets `max_tokens` via `AgentInput` and asserts the child `QueryConfig.max_tokens` matches (not the `4096` default). Cover foreground, background, and team runner paths explicitly. |
| R7 | Budget concept confusion: three independent mechanisms (`max_tokens` = token count, `max_budget_usd` = per-loop USD, `SessionBudget` = cross-session USD) may be confused by implementors or callers | Wrong field set by callers, leading to unexpected behavior | Low | M11-09 and M11-10 must use unambiguous field names. `max_tokens` is a token count. `budget_usd` is USD. `max_budget_usd` remains its existing name. Documentation in code comments must state the unit for each field. |

---

## 12. Open Questions / Confirmations Needed Before M11

1. **CLI `--budget-usd` flag:** M11-08 adds a `--budget-usd` CLI flag to `main.rs`. Confirm this is the intended user-facing interface for session budget, rather than a config-file-only setting.

2. **`models_for_provider()` scope in M11-05:** Confirm that adding a small helper to `ModelRegistry` in the `claurst-api` crate is acceptable within M11-05's scope, given that the MPWO file target for M11-05 is `crates/query/src/provider_resolution.rs`. The alternative is to add it as a separate mini-ticket before M11-05.

   **Recommendation:** Include it in M11-05's file targets (add `crates/api/src/model_registry.rs` as a secondary target). The helper is ~10 lines and directly serves M11-05's fallback model-selection step. Filing a separate ticket for 10 lines of code would be process overhead without proportional value.

3. **Child `max_tokens` default value:** M11-09 retains `CHILD_AGENT_FALLBACK_MAX_TOKENS = 4_096` as the backward-compatible default. Confirm that `4096` is the correct long-term default for spawned child agents, or whether it should be derived from the child model's `max_output_tokens` (from `ModelEntry`). **Recommendation:** Keep `4096` as the static default for M11. Model-aware token negotiation is future scope beyond D2.

---

## 13. Codex Handoff Section

### Recommended first executable ticket: TASK-M11-01 (TrustDomain enum)

**Why M11-01 should be first:**
- It is the simplest possible ticket: a single enum + a single `match` function in a single file.
- It has no code dependencies beyond M10 (this plan).
- It establishes the trust-domain vocabulary that M11-03 and M11-05 depend on.
- It is zero-risk to existing D1 behavior (pure addition, no modification of existing types).
- Validation is `cargo check -p claurst-api` only -- fast and low blast radius.

**Parallelizable tickets after M11-01:**
- M11-02 (ModelEntry extension) -- independent of M11-01.
- M11-04 (HealthCache) -- independent of M11-01/02/03.
- M11-06 (CostTracker extension) -- independent of everything except M10.
- M11-07 (SessionBudget) -- independent of everything except M10.

The critical path is: M11-01 + M11-02 -> M11-03 -> M11-05 -> M11-09 -> M11-10 -> M11-11 -> M11-12.

Recommended execution order for serial (one-ticket-at-a-time) Codex execution:
```
M11-01  TrustDomain enum
M11-02  ModelEntry extension
M11-04  HealthCache implementation
M11-06  CostTracker extension
M11-07  SessionBudget implementation
M11-03  Capability enum and matching (depends on M11-02)
M11-05  resolve_provider_with_fallback() (depends on M11-01, M11-03, M11-04)
M11-08  Budget + cancellation wiring (depends on M11-07)
M11-09  Child execution override wiring (depends on M11-08)
M11-10  Schema updates -- allow_fallback + budget_usd (depends on M11-05, M11-08, M11-09)
M11-11  QueryEvent variants (depends on M11-08, M11-09)
M11-12  D2 test suite + workspace validation (depends on all above)
```

This order front-loads the independent leaf tickets and defers the integration tickets until their dependencies are met. M11-09 (child execution overrides) is placed immediately after M11-08 (budget/cancellation wiring) because both modify the same spawn paths in `agent_tool.rs` -- doing them back-to-back minimizes conflict risk.

---

## 14. Final Recommendation

**M10 is complete and ready for human review.**

The D1 seam is well-suited for D2 extension. The MPWO M11 ticket breakdown (now 12 tickets: M11-01 through M11-12) aligns with repo reality with one minor gap (R1: `ModelRegistry` model enumeration helper needed in M11-05). All other extension points, dependencies, and data structures are confirmed present and sufficient.

Child execution independence is first-class in this plan: parent/child model and provider independence is already D1-supported; child `max_tokens`, `budget_usd`, and `allow_fallback` become first-class D2 settings via M11-09 and M11-10. The `CHILD_AGENT_FALLBACK_MAX_TOKENS` interim constant is superseded but retained as the backward-compatible default.

The plan is narrow, honest, and directly executable by Codex one ticket at a time. No speculative architecture or RFC redesign is proposed. All D2 additions are optional/defaulted to preserve D1 behavior. The hosted-Ollama non-regression invariant is explicitly preserved in the plan.
