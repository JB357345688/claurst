# Architecture RFC: Provider-Aware Worker Fabric

**High-Level Design for Multi-Provider Sub-Agent Orchestration in Claurst**

| Field | Value |
|-------|-------|
| **RFC ID** | RFC-0001 |
| **Status** | Draft |
| **Author** | Claurst Core Team |
| **Date** | 2026-04-08 |
| **Codebase Version** | `acae926` (2026-04-07) |
| **Affects** | `claurst-query`, `claurst-api`, `claurst-tools`, `claurst-core`, `claurst-tui` |

---

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [Problem Statement](#2-problem-statement)
3. [Current State and Architectural Gap](#3-current-state-and-architectural-gap)
4. [Goals and Non-Goals](#4-goals-and-non-goals)
5. [Terminology and Naming Clarification](#5-terminology-and-naming-clarification)
6. [Target Architecture](#6-target-architecture)
7. [Component Design](#7-component-design)
8. [Execution Model](#8-execution-model)
9. [Proposed Schema and Interface Changes](#9-proposed-schema-and-interface-changes)
10. [Permission, Isolation, and Security Considerations](#10-permission-isolation-and-security-considerations)
11. [Failure Modes and Operational Risks](#11-failure-modes-and-operational-risks)
12. [Observability and Telemetry Requirements](#12-observability-and-telemetry-requirements)
13. [Rollout Plan in Phases](#13-rollout-plan-in-phases)
14. [Acceptance Criteria](#14-acceptance-criteria)
15. [Test Strategy](#15-test-strategy)
16. [Open Questions and Tradeoffs](#16-open-questions-and-tradeoffs)
17. [Final Recommendation](#17-final-recommendation)

---

## 1. Executive Summary

Claurst's root query loop (`run_query_loop()` in `crates/query/src/lib.rs`) already supports multi-provider routing across 12+ LLM providers via `ProviderRegistry`. However, sub-agents spawned by `AgentTool` and parallel workers launched by `TeamCreateTool` hard-code `AnthropicClient::new(...)` and fail outright when `ANTHROPIC_API_KEY` is absent. This RFC proposes a unified provider-dispatch layer — the **Worker Fabric** — that propagates provider resolution, capability matching, health-aware fallback, and per-worker budget enforcement from the root session into every sub-agent and team worker.

The design introduces five new internal components (no new user-facing tools): a **WorkerScheduler**, a **CapabilityMatcher**, a **FallbackEngine**, a periodic **HealthProbe**, and a per-agent **BudgetLedger**. These compose with the existing `ProviderRegistry` and `ModelRegistry` to form a reusable dispatch path that replaces the two direct `AnthropicClient` construction sites in `agent_tool.rs` and the `AgentRunFn` callback consumed by `team_tool.rs`.

The result: users can spawn mixed-provider swarms (e.g., local Ollama workers for bulk search, Anthropic workers for synthesis, Google workers for large-context analysis) from a single coordinator session, with automatic fallback if a provider endpoint degrades or goes down.

---

## 2. Problem Statement

### 2.1 The Break in Provider Symmetry

The root session resolves its provider through a well-defined priority chain (lines 854-889 of `crates/query/src/lib.rs`):

1. Explicit `provider/model` format in the model string
2. `config.provider` setting (from `--provider` flag or `settings.json`)
3. `ModelRegistry` lookup (e.g., `gemini-3-flash-preview` → `google`)
4. Default to `anthropic`

Sub-agents bypass this entire path. At `agent_tool.rs:230-248`, the `AgentTool` directly reads `ANTHROPIC_API_KEY` from the environment and constructs a bare `AnthropicClient`. At `agent_tool.rs:356`, the resulting `QueryConfig` sets `provider_registry: None` and `model_registry: None`, meaning the child loop will always use the hardcoded Anthropic client regardless of the parent's provider configuration.

The `TeamCreateTool` (`team_tool.rs`) delegates via the injected `AgentRunFn` callback, which chains back into the same Anthropic-only code path in `init_team_swarm_runner()`.

### 2.2 Consequences

| Scenario | Current Behavior | Expected Behavior |
|----------|-----------------|-------------------|
| User sets `--provider openai` and spawns an Agent | Agent fails with "ANTHROPIC_API_KEY not set" | Agent uses OpenAI via parent's ProviderRegistry |
| Coordinator on Ollama creates a TeamCreate | All workers fail | Workers route through Ollama (or configured providers) |
| Agent specifies `model: "gemini-3-flash-preview"` | Ignored; uses DEFAULT_MODEL on Anthropic | ModelRegistry resolves to GoogleProvider |
| Anthropic has a temporary outage | All agents fail | Workers degrade to fallback provider |
| Mixed swarm: cheap local workers + expensive API workers | Not possible | Workers dispatch to specified providers independently |

### 2.3 Scope of the Gap

The gap is narrow but load-bearing. Only two call sites construct `AnthropicClient` directly (`agent_tool.rs:243` and `agent_tool.rs:541`). The fix is not "add more providers" — the provider infrastructure already exists. The fix is **propagating the existing infrastructure into child execution contexts**.

---

## 3. Current State and Architectural Gap

### 3.1 What Already Works

| Component | Location | Status |
|-----------|----------|--------|
| `ProviderRegistry` | `crates/api/src/registry.rs` | 35+ providers registered, health-check API, default provider |
| `ModelRegistry` | `crates/api/src/model_registry.rs` | Dynamic model→provider resolution via models.dev |
| `ProviderCapabilities` | `crates/api/src/provider_types.rs` | 9 capability flags (streaming, tool_calling, thinking, etc.) |
| `ProviderStatus` | `crates/api/src/provider_types.rs` | Health enum: Healthy / Degraded / Unavailable |
| `LlmProvider` trait | `crates/api/src/provider.rs` | `create_message_stream()`, `health_check()`, `capabilities()` |
| `CostTracker` | `crates/core/src/lib.rs` | Atomic token/cost accumulation, model-aware pricing |
| `QueryConfig.provider_registry` | `crates/query/src/lib.rs:116` | Optional Arc, plumbed through root loop |
| `QueryConfig.model_registry` | `crates/query/src/lib.rs:123` | Optional Arc, plumbed through root loop |
| Root provider dispatch | `crates/query/src/lib.rs:854-889` | Multi-step resolution with known-provider list |
| `AgentTool` worktree isolation | `crates/query/src/agent_tool.rs` | Git worktree creation, background polling |
| `TeamCreateTool` cancel tokens | `crates/tools/src/team_tool.rs` | Per-agent CancellationToken via DashMap |

### 3.2 What Does Not Work

| Component | Location | Gap |
|-----------|----------|-----|
| Agent provider dispatch | `agent_tool.rs:230-248` | Hardcoded `AnthropicClient::new(...)`, `ANTHROPIC_API_KEY` required |
| Agent QueryConfig | `agent_tool.rs:339-360` | `provider_registry: None`, `model_registry: None` |
| AgentRunFn signature | `team_tool.rs:46-57` | No `provider` or `model` parameter; no capability requirements |
| Agent CostTracker | `agent_tool.rs` | Shares parent's `CostTracker` but no per-agent partitioning |
| Fallback on failure | Nowhere | No fallback logic for sub-agents; single-provider-or-fail |
| Health-aware routing | `registry.rs:134` | `check_all_health()` exists but is never called by agent dispatch |
| Budget enforcement | `agent_tool.rs:354` | `max_budget_usd: None` always; parent budget not propagated |

### 3.3 Architectural Diagram — Current State

```
 Root Session (TUI / --print)
 ┌──────────────────────────────────────────┐
 │ QueryConfig {                            │
 │   provider_registry: Some(Arc<PR>),      │
 │   model_registry: Some(Arc<MR>),         │
 │   max_budget_usd: Some(5.0),             │
 │ }                                        │
 │                                          │
 │ run_query_loop() ──► provider dispatch   │
 │   ├── anthropic                          │
 │   ├── openai                             │
 │   ├── google                             │
 │   ├── ollama (local)                     │
 │   └── ... 30+ more                      │
 └───────┬──────────────────────────────────┘
         │
         │  AgentTool.execute()
         ▼
 Sub-Agent
 ┌──────────────────────────────────────────┐
 │ AnthropicClient::new(ANTHROPIC_API_KEY)  │  ◄── hardcoded
 │ QueryConfig {                            │
 │   provider_registry: None,               │  ◄── gap
 │   model_registry: None,                  │  ◄── gap
 │   max_budget_usd: None,                  │  ◄── gap
 │ }                                        │
 └──────────────────────────────────────────┘
```

---

## 4. Goals and Non-Goals

### 4.1 Goals

| # | Goal | Measurable Outcome |
|---|------|--------------------|
| G1 | **Provider propagation** | Sub-agents and team workers inherit the parent's `ProviderRegistry` and `ModelRegistry`. Agents can override the provider per-worker. |
| G2 | **Capability matching** | When a worker specifies `required_capabilities: ["tool_calling", "image_input"]`, the scheduler selects a provider/model pair that satisfies all constraints. |
| G3 | **Health-aware fallback** | If the requested provider returns `Unavailable` or `Degraded`, the scheduler tries the next eligible provider that matches capabilities. |
| G4 | **Per-worker budget enforcement** | Each worker gets an individual `CostTracker` with an optional USD cap. The coordinator can inspect per-worker spend. Aggregate spend is tracked at the session level. |
| G5 | **Backward compatibility** | Existing `Agent` and `TeamCreate` invocations without `provider` / `required_capabilities` fields continue to work identically. |
| G6 | **Zero new tools** | This RFC modifies internal plumbing only. No new user-facing tools or slash commands are added. |

### 4.2 Non-Goals

| # | Non-Goal | Rationale |
|---|----------|-----------|
| NG1 | Campaign DAG / artifact bus | Separate RFC scope; requires stage-dependency graph and typed artifact system. |
| NG2 | Runtime orchestration (Docker/SSH/libvirt) | Separate from provider dispatch; belongs in ULTRAPLAN-OSS. |
| NG3 | KAIROS tick engine integration | KAIROS is not yet implemented; this RFC should not depend on it. |
| NG4 | Model auto-selection via ML | Capability matching is rule-based, not learned. ML-based selection is a future extension. |
| NG5 | Cross-provider conversation migration | Moving a mid-conversation agent from one provider to another is out of scope. |
| NG6 | New billing/payment integration | Budget enforcement uses local tracking only; no integration with provider billing APIs. |

---

## 5. Terminology and Naming Clarification

### 5.1 Feature Name

The working name **"Provider-Aware Worker Fabric"** is evocative but non-standard. The term "fabric" implies a mesh or network topology, which overstates the complexity — this is fundamentally a dispatch and scheduling layer, not a service mesh.

**Recommendation:** Use **"Multi-Provider Worker Orchestration"** (abbreviated **MPWO**) as the formal internal name. Reasons:

- "Multi-provider" is precise and self-explanatory
- "Worker" aligns with existing `AgentMode::Worker` terminology
- "Orchestration" correctly describes the scheduling, fallback, and budget coordination responsibilities

The name "Provider-Aware Worker Fabric" can remain as a colloquial/marketing name for the feature, but code modules, feature flags, and internal documentation should use `mpwo` or `worker_orchestration`.

### 5.2 Glossary

| Term | Definition |
|------|-----------|
| **Root session** | The top-level `run_query_loop()` instance driven by the TUI or `--print` mode. Owns the `ProviderRegistry`. |
| **Worker** | Any `run_query_loop()` instance spawned by `AgentTool` or `TeamCreateTool`. Workers may be foreground (blocking), background (polled), or parallel (joined). |
| **Provider** | An `LlmProvider` implementation registered in the `ProviderRegistry`. Identified by `ProviderId` (e.g., `"anthropic"`, `"ollama"`, `"openai"`). |
| **Model** | A specific model ID within a provider (e.g., `"claude-opus-4-6"`, `"gpt-4.1"`, `"gemma-3-27b"`). Resolved via `ModelRegistry`. |
| **Capability** | A boolean flag in `ProviderCapabilities` (e.g., `tool_calling`, `thinking`, `image_input`). Used by the CapabilityMatcher to filter eligible providers. |
| **Execution target** | A resolved `(ProviderId, model_id, Arc<dyn LlmProvider>)` tuple ready for dispatch. The output of the WorkerScheduler. |
| **Fallback chain** | An ordered list of execution targets that satisfy the same capability constraints, tried sequentially on failure. |
| **Budget ledger** | A per-worker `CostTracker` instance with an optional USD cap. Aggregated at the session level. |
| **Access profile** | A permission scope for a worker: `full`, `read-only`, or `search-only`. Controls which tools the worker can invoke. |

---

## 6. Target Architecture

### 6.1 Architectural Diagram — Target State

```
 Root Session
 ┌───────────────────────────────────────────────────────┐
 │ QueryConfig {                                         │
 │   provider_registry: Some(Arc<ProviderRegistry>),     │
 │   model_registry: Some(Arc<ModelRegistry>),           │
 │   worker_scheduler: Some(Arc<WorkerScheduler>),  NEW  │
 │   session_budget: Some(Arc<SessionBudget>),      NEW  │
 │ }                                                     │
 │                                                       │
 │ run_query_loop() ──► provider dispatch (unchanged)    │
 └───────┬─────────────────────┬─────────────────────────┘
         │                     │
         │ AgentTool           │ TeamCreateTool
         ▼                     ▼
 ┌───────────────┐   ┌──────────────────────────────────┐
 │ WorkerScheduler│   │ WorkerScheduler                  │
 │  .resolve()   │   │  .resolve_batch(agents[])        │
 └───────┬───────┘   └──────────┬───────────────────────┘
         │                      │
         ▼                      ▼
 ┌──────────────────────────────────────────────────────┐
 │              CapabilityMatcher                        │
 │  Input:  required_capabilities + preferred_provider   │
 │  Output: ranked list of (provider, model) pairs       │
 └───────────────────────┬──────────────────────────────┘
                         │
                         ▼
 ┌──────────────────────────────────────────────────────┐
 │              FallbackEngine                           │
 │  Try execution targets in order                       │
 │  On Unavailable/Degraded → next target                │
 │  On ProviderError(rate_limit) → backoff + retry       │
 │  On success → lock to that target for session         │
 └───────────────────────┬──────────────────────────────┘
                         │
         ┌───────────────┼──────────────────┐
         ▼               ▼                  ▼
 ┌──────────────┐ ┌──────────────┐  ┌──────────────┐
 │ Worker A     │ │ Worker B     │  │ Worker C     │
 │ anthropic/   │ │ ollama/      │  │ google/      │
 │ opus-4-6     │ │ llama-3.3    │  │ gemini-3     │
 │ budget: $2   │ │ budget: $0   │  │ budget: $1   │
 │ CostTracker  │ │ CostTracker  │  │ CostTracker  │
 └──────────────┘ └──────────────┘  └──────────────┘
         │               │                  │
         └───────────────┼──────────────────┘
                         ▼
                 SessionBudget (aggregate)
```

### 6.2 Key Design Principles

1. **Propagation over construction.** Workers inherit the parent's `ProviderRegistry` and `ModelRegistry` by reference (`Arc`). They never construct their own clients.

2. **Resolution is lazy.** The `WorkerScheduler` resolves the execution target at worker spawn time, not at `AgentTool` construction time. This allows health checks to reflect current state.

3. **Fallback is per-attempt.** Each API call within a worker's `run_query_loop()` goes through the `FallbackEngine`. A single worker may start on Anthropic, fail over to OpenAI mid-conversation, and resume on Anthropic if it recovers. (See [Section 16, Open Question OQ3](#oq3) for whether mid-conversation migration is desirable.)

4. **Budget is hierarchical.** The `SessionBudget` tracks aggregate spend. Each worker has an individual `CostTracker` that feeds into the session aggregate. A worker exceeding its individual cap is stopped; the session continues.

5. **Backward-compatible defaults.** If `provider`, `model`, and `required_capabilities` are all absent from an `Agent` call, behavior is identical to today: use the root session's default model on the root session's default provider.

---

## 7. Component Design

### 7.1 ProviderRegistry (Existing — No Changes Required)

**Location:** `crates/api/src/registry.rs`

The existing `ProviderRegistry` is sufficient. It already supports:
- Registration of `Arc<dyn LlmProvider>` by `ProviderId`
- Lookup by ID: `get(&ProviderId) -> Option<Arc<dyn LlmProvider>>`
- Health check: `check_all_health() -> Vec<(ProviderId, ProviderStatus)>`
- Default provider resolution
- Environment-based auto-registration with auth store (`from_environment_with_auth_store`)

**No modifications needed.** The registry is a passive data structure; the new components compose around it.

### 7.2 ModelRegistry (Existing — No Changes Required)

**Location:** `crates/api/src/model_registry.rs`

The existing `ModelRegistry` maps model IDs to provider IDs and fetches model metadata from models.dev. The `effective_model_for_config()` function resolves the final model ID given user configuration.

**No modifications needed.** Already supports the resolution patterns this RFC requires.

### 7.3 WorkerScheduler (New)

**Location:** `crates/query/src/worker_scheduler.rs` (new file)

The `WorkerScheduler` is the primary entry point for worker dispatch. It replaces the two `AnthropicClient::new(...)` call sites in `agent_tool.rs`.

```rust
/// Resolves an execution target for a worker given its requirements.
///
/// Returned `ExecutionTarget` contains a resolved provider, model, and
/// a pre-configured `CostTracker` with the worker's budget cap.
pub struct WorkerScheduler {
    provider_registry: Arc<ProviderRegistry>,
    model_registry: Arc<ModelRegistry>,
    capability_matcher: CapabilityMatcher,
    fallback_engine: FallbackEngine,
    health_cache: Arc<HealthCache>,
    session_budget: Arc<SessionBudget>,
}

/// The resolved dispatch target for a single worker.
pub struct ExecutionTarget {
    /// The provider that will serve this worker's API calls.
    pub provider: Arc<dyn LlmProvider>,
    /// The resolved provider ID.
    pub provider_id: ProviderId,
    /// The resolved model ID (provider-local, prefix stripped).
    pub model_id: String,
    /// Per-worker cost tracker, linked to the session budget.
    pub cost_tracker: Arc<CostTracker>,
    /// Ordered fallback targets (excluding the primary).
    pub fallback_chain: Vec<(ProviderId, String, Arc<dyn LlmProvider>)>,
}

impl WorkerScheduler {
    /// Resolve an execution target for a single worker.
    ///
    /// Resolution priority:
    ///   1. Explicit `provider` + `model` from worker params
    ///   2. Explicit `model` resolved via ModelRegistry
    ///   3. Explicit `provider` with provider's default model
    ///   4. CapabilityMatcher filtering by `required_capabilities`
    ///   5. Root session's default provider and model
    pub fn resolve(
        &self,
        params: &WorkerParams,
    ) -> Result<ExecutionTarget, WorkerSchedulerError>;

    /// Resolve execution targets for a batch of workers (TeamCreate).
    /// Returns one ExecutionTarget per worker, or an error for workers
    /// that cannot be satisfied.
    pub fn resolve_batch(
        &self,
        workers: &[WorkerParams],
    ) -> Vec<Result<ExecutionTarget, WorkerSchedulerError>>;
}

/// Worker requirements extracted from Agent or TeamCreate params.
pub struct WorkerParams {
    pub provider: Option<String>,
    pub model: Option<String>,
    pub required_capabilities: Vec<Capability>,
    pub budget_usd: Option<f64>,
    pub access_profile: AccessProfile,
}
```

**Design notes:**

- `resolve()` is synchronous. Health data comes from the `HealthCache` (which is updated asynchronously; see Section 7.6). This avoids blocking agent spawn on network calls.
- `resolve_batch()` is a convenience for `TeamCreateTool` that calls `resolve()` per worker. Future optimization: batch health checks and share results.
- The `WorkerScheduler` is constructed once at root session startup and shared via `Arc` in `QueryConfig`.

### 7.4 CapabilityMatcher (New)

**Location:** `crates/query/src/capability_matcher.rs` (new file)

Given a set of required capabilities and the `ProviderRegistry`, returns an ordered list of eligible `(provider_id, model_id)` pairs.

```rust
/// Capability constraint for worker matching.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Capability {
    ToolCalling,
    Thinking,
    ImageInput,
    PdfInput,
    AudioInput,
    VideoInput,
    Streaming,
    Caching,
    StructuredOutput,
}

pub struct CapabilityMatcher;

impl CapabilityMatcher {
    /// Returns providers that satisfy ALL required capabilities,
    /// ordered by preference:
    ///   1. preferred_provider (if specified and eligible)
    ///   2. root session's default provider
    ///   3. remaining eligible providers, sorted by health then cost
    pub fn match_providers(
        &self,
        registry: &ProviderRegistry,
        health_cache: &HealthCache,
        required: &[Capability],
        preferred_provider: Option<&str>,
        default_provider: &ProviderId,
    ) -> Vec<(ProviderId, Arc<dyn LlmProvider>)>;
}
```

**Matching logic:**

```
for each provider in registry:
    caps = provider.capabilities()
    if all required capabilities are true in caps:
        add to eligible list

sort eligible by:
    1. preferred_provider first (if present)
    2. default_provider second
    3. Healthy > Degraded > Unavailable
    4. Lower cost-per-token (from ModelRegistry metadata, if available)
```

The `Capability` enum maps 1:1 to the fields of the existing `ProviderCapabilities` struct (`provider_types.rs:199`). This is intentional — the enum provides a serializable, schema-friendly representation for tool input schemas, while `ProviderCapabilities` remains the provider's self-reported state.

### 7.5 FallbackEngine (New)

**Location:** `crates/query/src/fallback_engine.rs` (new file)

The `FallbackEngine` wraps API calls with automatic retry and provider failover. It operates **within** a worker's `run_query_loop()`, not at spawn time.

```rust
pub struct FallbackEngine {
    primary: ExecutionTarget,
    retry_config: RetryConfig,
}

impl FallbackEngine {
    /// Attempt an API call on the primary target. On transient failure,
    /// try each fallback target in order. On all-fail, return the last error.
    ///
    /// Transient failures:
    ///   - HTTP 429 (rate limited) — backoff, retry same provider
    ///   - HTTP 503 (service unavailable) — try next provider
    ///   - HTTP 529 (overloaded) — try next provider
    ///   - Connection refused — try next provider
    ///   - Timeout — try next provider
    ///
    /// Non-transient failures (no fallback):
    ///   - HTTP 400 (bad request)
    ///   - HTTP 401 (unauthorized)
    ///   - HTTP 413 (context overflow)
    ///   - Any error from a successful stream (partial response)
    pub async fn execute_with_fallback<F, Fut, T>(
        &self,
        make_request: F,
    ) -> Result<(T, ProviderId), ProviderError>
    where
        F: Fn(Arc<dyn LlmProvider>, &str) -> Fut,
        Fut: Future<Output = Result<T, ProviderError>>;
}
```

**Integration point:** The `FallbackEngine` replaces the direct `client.create_message_stream()` call in `run_query_loop()`. For the root session (which already has provider dispatch), the fallback engine wraps the existing dispatch. For workers, it is the primary dispatch mechanism.

**Important constraint:** Fallback across providers mid-conversation requires message format re-transformation (Anthropic format ≠ OpenAI format). The existing `MessageTransformer` trait handles this, but the conversation history must be re-serialized for the new provider. This adds latency but is correct.

### 7.6 HealthProbe (New)

**Location:** `crates/query/src/health_probe.rs` (new file)

A background task that periodically probes registered providers and updates a shared `HealthCache`.

```rust
pub struct HealthCache {
    statuses: DashMap<ProviderId, (ProviderStatus, Instant)>,
    ttl: Duration,
}

impl HealthCache {
    /// Get cached status. Returns None if no probe has run yet.
    pub fn get(&self, provider_id: &ProviderId) -> Option<ProviderStatus>;

    /// Update status for a provider.
    pub fn update(&self, provider_id: ProviderId, status: ProviderStatus);

    /// Check if a provider is considered healthy (Healthy or Degraded).
    pub fn is_available(&self, provider_id: &ProviderId) -> bool;
}

pub struct HealthProbe {
    registry: Arc<ProviderRegistry>,
    cache: Arc<HealthCache>,
    interval: Duration,
}

impl HealthProbe {
    /// Spawn a background tokio task that probes all registered
    /// providers at `interval`. Updates HealthCache on each probe.
    /// Returns a JoinHandle and CancellationToken for shutdown.
    pub fn spawn(self) -> (JoinHandle<()>, CancellationToken);
}
```

**Configuration:**

| Parameter | Default | Source |
|-----------|---------|--------|
| `health_check_interval` | 60 seconds | `settings.json` → `worker_orchestration.health_check_interval_sec` |
| `health_cache_ttl` | 120 seconds | 2x the check interval |
| `probe_timeout` | 5 seconds | Per-provider probe timeout |

**Design decision:** Health checks are opt-in and only activate when `provider_registry` contains more than one provider. A single-provider setup (the default) skips probing entirely to avoid unnecessary network traffic.

### 7.7 Budget and Cost Tracking

**Location:** Modifications to `crates/core/src/lib.rs` (`CostTracker`) and new `crates/query/src/session_budget.rs`

#### 7.7.1 Per-Worker CostTracker (Existing — Minor Extension)

The existing `CostTracker` is already per-instance and thread-safe. Each worker gets its own `CostTracker` instance, created by the `WorkerScheduler`:

```rust
// In WorkerScheduler::resolve():
let worker_cost_tracker = CostTracker::with_model(&resolved_model_id);
```

**Extension needed:** Add an `agent_id` label to `CostTracker` for attribution:

```rust
pub struct CostTracker {
    input_tokens: AtomicU64,
    output_tokens: AtomicU64,
    cache_creation_tokens: AtomicU64,
    cache_read_tokens: AtomicU64,
    pricing: parking_lot::RwLock<ModelPricing>,
    agent_id: Option<String>,         // NEW
    provider_id: Option<ProviderId>,   // NEW
}
```

#### 7.7.2 SessionBudget (New)

Aggregates all worker `CostTracker` instances into a session-level budget:

```rust
pub struct SessionBudget {
    /// Total session budget cap (from config).
    max_budget_usd: Option<f64>,
    /// Per-worker trackers, indexed by agent_id.
    workers: DashMap<String, Arc<CostTracker>>,
    /// The root session's own tracker.
    root_tracker: Arc<CostTracker>,
}

impl SessionBudget {
    /// Register a worker's tracker. Called by WorkerScheduler::resolve().
    pub fn register_worker(&self, agent_id: String, tracker: Arc<CostTracker>);

    /// Total spend across all workers + root session.
    pub fn total_cost_usd(&self) -> f64;

    /// Per-worker cost breakdown.
    pub fn worker_costs(&self) -> Vec<(String, f64)>;

    /// Check if session budget is exceeded.
    pub fn is_exceeded(&self) -> bool;
}
```

**Budget enforcement hierarchy:**

```
Session max_budget_usd (from config or --max-budget flag)
  ├── Root session CostTracker
  ├── Worker A CostTracker (budget_usd: $2.00)
  ├── Worker B CostTracker (budget_usd: None → inherits remaining session budget)
  └── Worker C CostTracker (budget_usd: $1.00)

Enforcement:
  - Per-worker: run_query_loop() checks worker's CostTracker against its cap
  - Session-wide: run_query_loop() checks SessionBudget.is_exceeded() after each turn
  - A worker exceeding its individual cap → QueryOutcome::BudgetExceeded (worker stops)
  - Session budget exceeded → all workers receive cancellation signal
```

### 7.8 Permission and Access Profile Handling

**Location:** Modifications to `crates/tools/src/lib.rs` (`ToolContext`) and `crates/query/src/agent_tool.rs`

Workers operate under **access profiles** that restrict which tools they may invoke:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessProfile {
    /// All tools available (minus Agent to prevent recursion).
    Full,
    /// Read-only tools only: Read, Glob, Grep, WebFetch, WebSearch, LSP.
    ReadOnly,
    /// Search tools only: Glob, Grep, WebSearch.
    SearchOnly,
}

impl AccessProfile {
    /// Filter a tool list to only tools allowed by this profile.
    pub fn filter_tools(&self, tools: Vec<Box<dyn Tool>>) -> Vec<Box<dyn Tool>>;
}
```

**Current state:** `AgentTool` already supports tool allowlisting via the `tools` parameter (`agent_tool.rs:254-262`). The `AccessProfile` is a higher-level abstraction that provides named presets. When both `tools` (explicit allowlist) and `access_profile` are specified, the explicit allowlist takes precedence.

**Tool filtering priority:**

1. Explicit `tools` allowlist (existing behavior, unchanged)
2. `access_profile` preset
3. Default: all tools minus `Agent` (existing behavior, unchanged)

---

## 8. Execution Model

### 8.1 Root Session

**No behavioral change.** The root session continues to use its existing provider dispatch path (`lib.rs:854-889`). The only addition is that the root session now constructs a `WorkerScheduler` and `SessionBudget` at startup and stores them in `QueryConfig`:

```rust
// In crates/cli/src/main.rs, after ProviderRegistry construction:
let health_cache = Arc::new(HealthCache::new(Duration::from_secs(120)));
let worker_scheduler = Arc::new(WorkerScheduler::new(
    provider_registry.clone(),
    model_registry.clone(),
    health_cache.clone(),
    session_budget.clone(),
));
query_config.worker_scheduler = Some(worker_scheduler);
query_config.session_budget = Some(session_budget);

// Optionally start health probing if multiple providers:
if provider_registry.provider_count() > 1 {
    let probe = HealthProbe::new(provider_registry.clone(), health_cache, Duration::from_secs(60));
    let (_handle, _cancel) = probe.spawn();
}
```

### 8.2 Agent (AgentTool)

**Before (current):**
```
AgentTool.execute()
  → read ANTHROPIC_API_KEY from env
  → AnthropicClient::new(key)
  → QueryConfig { provider_registry: None, model_registry: None }
  → run_query_loop(client, config, tools, messages)
```

**After (proposed):**
```
AgentTool.execute()
  → extract WorkerParams from tool input (provider, model, capabilities, budget, access_profile)
  → worker_scheduler.resolve(params) → ExecutionTarget
  → QueryConfig {
      provider_registry: Some(parent.provider_registry),
      model_registry: Some(parent.model_registry),
      worker_scheduler: Some(parent.worker_scheduler),  // for nested agents if allowed
      max_budget_usd: params.budget_usd,
      fallback_model: computed from ExecutionTarget.fallback_chain,
    }
  → run_query_loop(target.provider, config, filtered_tools, messages)
```

Key changes in `agent_tool.rs`:

1. **Remove** the `ANTHROPIC_API_KEY` read and `AnthropicClient::new()` call (lines 230-248).
2. **Add** `WorkerParams` extraction from the tool input JSON.
3. **Call** `worker_scheduler.resolve()` to get the `ExecutionTarget`.
4. **Propagate** parent registries into the child `QueryConfig`.
5. **Set** `max_budget_usd` from the worker params.
6. **Use** `AccessProfile::filter_tools()` to restrict the tool set.

### 8.3 TeamCreate

**Before (current):**
```
TeamCreateTool.execute()
  → for each agent: run_agent(description, prompt, tools, system, max_turns, ctx)
  → join_all(futures)
  → concatenate results
```

**After (proposed):**
```
TeamCreateTool.execute()
  → for each agent:
      extract WorkerParams (provider, model, capabilities, budget, access_profile)
      → worker_scheduler.resolve_batch(all_params)
  → for each (agent, target):
      run_agent(description, prompt, tools, system, max_turns, ctx, target)  // extended signature
  → join_all(futures)
  → concatenate results with per-worker cost annotations
```

This requires extending the `AgentRunFn` signature (see Section 9.3).

### 8.4 Parallel Workers

Parallel workers (multiple agents in a `TeamCreate`) execute concurrently via `tokio::spawn` and `join_all`. Each worker has:

- Its own `ExecutionTarget` (potentially different providers)
- Its own `CostTracker` (linked to `SessionBudget`)
- Its own `CancellationToken` (existing mechanism)
- Its own `run_query_loop()` instance

**Concurrency guarantees:**
- Workers are isolated at the conversation level (separate message histories)
- Workers share the filesystem (unless `isolation: "worktree"` is set)
- Workers share the `ProviderRegistry` (read-only)
- Workers share the `SessionBudget` (atomic operations, no contention)
- Workers share `HealthCache` (lock-free DashMap reads)

### 8.5 Degraded / Fallback Routing

When a worker's primary provider fails with a transient error:

```
Worker A: anthropic/claude-opus-4-6
  │
  ├── API call → HTTP 529 (overloaded)
  │
  ├── FallbackEngine: try fallback_chain[0]
  │   └── openai/gpt-4.1
  │       └── API call → success
  │           └── Continue conversation on openai/gpt-4.1
  │
  ├── (next turn) API call on openai/gpt-4.1 → success
  │
  └── (health probe updates: anthropic → Healthy)
      └── Worker stays on openai/gpt-4.1 for this session
          (no mid-conversation provider migration)
```

**Fallback policy:**
- A worker that fails over stays on the fallback provider for the remainder of its session. This avoids the complexity and risk of mid-conversation provider migration.
- The `FallbackEngine` logs the failover event with the original and fallback provider IDs.
- If all providers in the fallback chain fail, the worker returns `QueryOutcome::Error` with a descriptive message listing all attempted providers.

---

## 9. Proposed Schema and Interface Changes

### 9.1 Agent Tool Input Schema (Extended)

New optional fields added to the `Agent` tool's JSON input schema:

```json
{
  "type": "object",
  "properties": {
    "prompt": { "type": "string", "description": "The task for the agent to perform" },
    "description": { "type": "string", "description": "Short description of the task" },
    "model": { "type": "string", "description": "Model override (existing)" },
    "provider": {
      "type": "string",
      "description": "Preferred provider id (e.g., anthropic, openai, ollama, google)"
    },
    "required_capabilities": {
      "type": "array",
      "items": {
        "type": "string",
        "enum": ["tool_calling", "thinking", "image_input", "pdf_input",
                 "audio_input", "video_input", "streaming", "caching",
                 "structured_output"]
      },
      "description": "Capabilities the provider must support"
    },
    "budget_usd": {
      "type": "number",
      "description": "Maximum USD spend for this worker"
    },
    "access_profile": {
      "type": "string",
      "enum": ["full", "read-only", "search-only"],
      "description": "Tool access restriction for this worker"
    },
    "run_in_background": { "type": "boolean" },
    "isolation": { "type": "string", "enum": ["worktree"] },
    "tools": { "type": "array", "items": { "type": "string" } },
    "system_prompt": { "type": "string" },
    "max_turns": { "type": "integer" }
  },
  "required": ["prompt"]
}
```

All new fields are optional and backward-compatible.

### 9.2 TeamCreate Tool Input Schema (Extended)

New optional fields on each agent entry within `TeamCreate.agents[]`:

```json
{
  "type": "object",
  "properties": {
    "name": { "type": "string" },
    "prompt": { "type": "string" },
    "provider": { "type": "string" },
    "model": { "type": "string" },
    "required_capabilities": {
      "type": "array",
      "items": { "type": "string" }
    },
    "budget_usd": { "type": "number" },
    "access_profile": {
      "type": "string",
      "enum": ["full", "read-only", "search-only"]
    },
    "tools": { "type": "array", "items": { "type": "string" } },
    "system_prompt": { "type": "string" }
  },
  "required": ["name", "prompt"]
}
```

### 9.3 AgentRunFn Signature (Breaking Internal Change)

The `AgentRunFn` type in `team_tool.rs` must be extended to pass provider context:

**Before:**
```rust
pub type AgentRunFn = Arc<
    dyn Fn(
        String,              // description
        String,              // prompt
        Option<Vec<String>>, // tools allowlist
        Option<String>,      // system prompt
        Option<u32>,         // max_turns
        Arc<ToolContext>,    // context
    ) -> Pin<Box<dyn Future<Output = String> + Send>>
    + Send + Sync,
>;
```

**After:**
```rust
pub type AgentRunFn = Arc<
    dyn Fn(
        String,              // description
        String,              // prompt
        Option<Vec<String>>, // tools allowlist
        Option<String>,      // system prompt
        Option<u32>,         // max_turns
        Arc<ToolContext>,    // context
        WorkerDispatch,      // NEW: resolved provider context
    ) -> Pin<Box<dyn Future<Output = String> + Send>>
    + Send + Sync,
>;

/// Encapsulates the provider dispatch information for a worker.
/// Passed from TeamCreateTool through AgentRunFn to run_query_loop().
pub struct WorkerDispatch {
    pub provider: Arc<dyn LlmProvider>,
    pub provider_id: ProviderId,
    pub model_id: String,
    pub cost_tracker: Arc<CostTracker>,
    pub fallback_chain: Vec<(ProviderId, String, Arc<dyn LlmProvider>)>,
    pub access_profile: AccessProfile,
    pub budget_usd: Option<f64>,
}

impl Default for WorkerDispatch {
    /// Default dispatch: use the parent's Anthropic configuration.
    /// This preserves backward compatibility for callers that don't
    /// specify provider context.
    fn default() -> Self { /* ... */ }
}
```

This is an internal API change (not user-visible). The `init_team_swarm_runner()` function in `crates/query/src/lib.rs` (or wherever it currently lives) must update its closure to accept and use the new `WorkerDispatch` parameter.

### 9.4 QueryConfig Extensions

```rust
pub struct QueryConfig {
    // ... existing fields unchanged ...

    /// Worker scheduler for resolving sub-agent provider targets.
    /// Shared across the session; None for legacy/test configurations.
    pub worker_scheduler: Option<Arc<WorkerScheduler>>,

    /// Session-level budget aggregator.
    /// None for legacy/test configurations.
    pub session_budget: Option<Arc<SessionBudget>>,
}
```

### 9.5 ToolContext Extensions

```rust
pub struct ToolContext {
    // ... existing fields unchanged ...

    /// Worker scheduler for resolving sub-agent provider targets.
    /// Available to tools that spawn sub-agents (AgentTool, TeamCreateTool).
    pub worker_scheduler: Option<Arc<WorkerScheduler>>,

    /// Session-level budget aggregator.
    pub session_budget: Option<Arc<SessionBudget>>,
}
```

---

## 10. Permission, Isolation, and Security Considerations

### 10.1 Access Profile Enforcement

Access profiles restrict a worker's tool set at spawn time. This is enforced by filtering the tool list before passing it to `run_query_loop()`. The model cannot bypass access profiles because it only sees the tools provided in the API request's `tools` array.

| Profile | Allowed Tools | Use Case |
|---------|---------------|----------|
| `full` | All tools minus `Agent` | Default; code generation, editing, testing |
| `read-only` | Read, Glob, Grep, WebFetch, WebSearch, LSP, ToolSearch, TaskGet, TaskList | Research, code review, analysis |
| `search-only` | Glob, Grep, WebSearch, ToolSearch | Bulk code search, index building |

### 10.2 API Key Isolation

Workers inherit provider references (`Arc<dyn LlmProvider>`) from the parent's `ProviderRegistry`. They never see raw API keys. The keys are encapsulated inside the provider implementations and are not extractable via the `LlmProvider` trait. This is the existing behavior for the root session; extending it to workers does not change the security posture.

### 10.3 Worktree Isolation

The existing `isolation: "worktree"` mechanism (`agent_tool.rs:310-337`) is unchanged. Workers with worktree isolation get a separate git worktree, preventing filesystem conflicts. This is orthogonal to provider selection.

### 10.4 Budget as a Security Boundary

Per-worker budgets prevent a misbehaving or hijacked worker from consuming unbounded resources. The `SessionBudget` acts as a secondary check — even if a worker's individual budget is set high, the session budget provides an upper bound.

**Threat model consideration:** A worker cannot increase its own budget because the `CostTracker` is created by the `WorkerScheduler` (outside the worker's control). The worker can only observe its remaining budget, not modify its cap.

### 10.5 Provider Credential Scoping

When a user configures multiple providers (e.g., Anthropic for production, Ollama for development), workers access only providers registered in the shared `ProviderRegistry`. There is no mechanism for a worker to register new providers or use credentials not already configured by the user.

---

## 11. Failure Modes and Operational Risks

### 11.1 Failure Mode Catalog

| # | Failure Mode | Impact | Mitigation |
|---|-------------|--------|------------|
| F1 | Requested provider not in registry | Worker fails to spawn | `WorkerScheduler::resolve()` returns `Err(ProviderNotFound)`. Agent tool returns error to model with list of available providers. |
| F2 | No provider satisfies required capabilities | Worker fails to spawn | `CapabilityMatcher` returns empty list. Error message includes required capabilities and available providers with their capabilities. |
| F3 | Primary provider down, all fallbacks exhausted | Worker fails mid-execution | `FallbackEngine` returns `Err` with all attempted providers listed. Worker returns partial results (if any) plus error context. |
| F4 | Provider returns auth error (401) for worker | Worker fails, non-retriable | No fallback (auth errors are not transient). Worker returns auth error. User must fix credentials via `/connect` or env vars. |
| F5 | Worker exceeds individual budget | Worker stops, session continues | `run_query_loop()` returns `QueryOutcome::BudgetExceeded`. Partial results returned to coordinator. |
| F6 | Session budget exceeded | All workers receive cancellation | `SessionBudget::is_exceeded()` triggers cancellation tokens. Graceful shutdown: workers finish current API call, then stop. |
| F7 | Health probe cannot reach provider | `HealthCache` marks as Unavailable | `CapabilityMatcher` deprioritizes but does not exclude (health data may be stale). Direct API call may still succeed. |
| F8 | Message format incompatible after fallback | API error on fallback provider | `MessageTransformer` re-serializes conversation for new provider. If transformation fails, fallback skips this provider. |
| F9 | Local provider (Ollama) slow to respond | Worker blocks for extended time | Per-worker timeout (from `QueryConfig.max_turns` and model-specific timeouts). Worker can be cancelled via `CancellationToken`. |
| F10 | Race condition in SessionBudget | Over-budget by one turn | Acceptable: atomic operations in `CostTracker` ensure eventual consistency. Over-budget by at most one API call's cost. |

### 11.2 Operational Risks

| Risk | Likelihood | Severity | Mitigation |
|------|-----------|----------|------------|
| Breaking `AgentRunFn` signature affects callers | Certain | Low | Only one call site (`init_team_swarm_runner`); change is coordinated. Default `WorkerDispatch` preserves old behavior. |
| Health probes add network traffic | Low | Low | Only active when multiple providers registered. Configurable interval (default 60s). |
| Complex fallback chains confuse users | Medium | Medium | Log all fallback events. `/cost` command shows which provider served each worker. |
| Different models produce inconsistent results within a team | Medium | Medium | Document that mixed-provider teams may produce stylistically inconsistent outputs. This is inherent to multi-model orchestration. |

---

## 12. Observability and Telemetry Requirements

### 12.1 Structured Logging

All new components emit structured logs via `tracing`:

```rust
// WorkerScheduler
info!(
    agent_id = %agent_id,
    provider = %target.provider_id,
    model = %target.model_id,
    fallback_count = target.fallback_chain.len(),
    budget_usd = ?params.budget_usd,
    "Worker execution target resolved"
);

// FallbackEngine
warn!(
    agent_id = %agent_id,
    failed_provider = %primary.provider_id,
    error = %err,
    fallback_provider = %fallback.provider_id,
    "Provider failover triggered"
);

// HealthProbe
debug!(
    provider = %provider_id,
    status = ?status,
    latency_ms = latency.as_millis(),
    "Health probe completed"
);
```

### 12.2 Cost Command Extension

The `/cost` slash command should be extended to show per-worker breakdowns:

```
Session cost: $3.47

  Root session:     $1.20  (anthropic/claude-opus-4-6)
  Worker "search":  $0.02  (ollama/llama-3.3-70b)
  Worker "analyze": $0.85  (google/gemini-3-flash)
  Worker "patch":   $1.40  (anthropic/claude-sonnet-4-6)

  Budget remaining: $1.53 / $5.00
```

### 12.3 Agent View Extension

The existing `agents_view.rs` TUI component should display provider information per agent:

```
Active Agents
─────────────────────────────────────────────
  ID        Provider        Model            Cost    Status
  a1b2c3    anthropic       opus-4-6         $1.40   Running
  d4e5f6    ollama          llama-3.3        $0.02   Complete
  g7h8i9    google          gemini-3-flash   $0.85   Running
```

### 12.4 QueryEvent Extensions

New `QueryEvent` variants for observability:

```rust
pub enum QueryEvent {
    // ... existing variants ...

    /// A worker's provider was resolved.
    WorkerProviderResolved {
        agent_id: String,
        provider_id: ProviderId,
        model_id: String,
        fallback_count: usize,
    },

    /// A worker failed over to a different provider.
    WorkerProviderFailover {
        agent_id: String,
        from_provider: ProviderId,
        to_provider: ProviderId,
        reason: String,
    },

    /// A worker exceeded its individual budget.
    WorkerBudgetExceeded {
        agent_id: String,
        spent_usd: f64,
        budget_usd: f64,
    },
}
```

---

## 13. Rollout Plan in Phases

### Phase 0: Foundation (Week 1)

**Goal:** Introduce new types and modules without changing existing behavior.

| Task | File | Change |
|------|------|--------|
| Add `Capability` enum | `crates/query/src/capability_matcher.rs` | New file |
| Add `AccessProfile` enum | `crates/tools/src/lib.rs` | New type |
| Add `WorkerParams` struct | `crates/query/src/worker_scheduler.rs` | New file, struct only |
| Add `ExecutionTarget` struct | `crates/query/src/worker_scheduler.rs` | Struct only |
| Add `WorkerDispatch` struct | `crates/tools/src/team_tool.rs` | New struct with Default impl |
| Add `SessionBudget` struct | `crates/query/src/session_budget.rs` | New file |
| Add `HealthCache` struct | `crates/query/src/health_probe.rs` | New file |
| Extend `CostTracker` with `agent_id` and `provider_id` | `crates/core/src/lib.rs` | Additive, backward-compatible |
| Extend `QueryConfig` with optional fields | `crates/query/src/lib.rs` | All new fields are `Option`, default `None` |
| Extend `ToolContext` with optional fields | `crates/tools/src/lib.rs` | All new fields are `Option`, default `None` |

**Verification:** `cargo build --workspace` and `cargo test --workspace` pass. Zero behavioral changes.

### Phase 1: WorkerScheduler + CapabilityMatcher (Week 2)

**Goal:** Implement resolution logic and wire it into `AgentTool`.

| Task | File | Change |
|------|------|--------|
| Implement `CapabilityMatcher::match_providers()` | `capability_matcher.rs` | Core matching logic |
| Implement `WorkerScheduler::resolve()` | `worker_scheduler.rs` | Resolution priority chain |
| Implement `WorkerScheduler::resolve_batch()` | `worker_scheduler.rs` | Batch wrapper |
| Replace `AnthropicClient::new()` in `AgentTool` | `agent_tool.rs` | Use `WorkerScheduler` when available, fall back to existing path when `worker_scheduler` is `None` |
| Construct `WorkerScheduler` in `main.rs` | `crates/cli/src/main.rs` | Build and inject into `QueryConfig` |
| Propagate `provider_registry` + `model_registry` to child `QueryConfig` | `agent_tool.rs:339-360` | Set from parent instead of `None` |

**Verification:** Agent can be spawned with `provider: "openai"` and routes to OpenAI. Agent without `provider` uses parent's default. All existing tests pass.

### Phase 2: Budget Enforcement + Access Profiles (Week 3)

**Goal:** Per-worker budgets and tool restrictions.

| Task | File | Change |
|------|------|--------|
| Implement `SessionBudget` | `session_budget.rs` | Registration, aggregation, enforcement |
| Wire `SessionBudget` into root session | `main.rs`, `lib.rs` | Construct and inject |
| Wire per-worker `CostTracker` through `WorkerScheduler` | `worker_scheduler.rs` | Create per-worker tracker, register with SessionBudget |
| Implement `AccessProfile::filter_tools()` | `tools/lib.rs` | Tool filtering logic |
| Apply `access_profile` in `AgentTool` | `agent_tool.rs` | Filter tools before passing to `run_query_loop` |
| Extend `/cost` command | `commands/` | Per-worker breakdown display |

**Verification:** Worker with `budget_usd: 0.50` stops at $0.50. Worker with `access_profile: "read-only"` cannot use Edit or Bash tools.

### Phase 3: FallbackEngine + HealthProbe (Week 4)

**Goal:** Automatic failover and health monitoring.

| Task | File | Change |
|------|------|--------|
| Implement `FallbackEngine::execute_with_fallback()` | `fallback_engine.rs` | Retry + failover logic |
| Implement `HealthProbe::spawn()` | `health_probe.rs` | Background probing task |
| Wire `FallbackEngine` into worker `run_query_loop()` | `lib.rs` | Wrap API dispatch calls |
| Conditionally start `HealthProbe` in `main.rs` | `main.rs` | Only when multi-provider |
| Add `QueryEvent` variants for failover | `lib.rs` | New event types |

**Verification:** Simulated provider failure (e.g., via test mock) triggers automatic failover. Health probe correctly updates `HealthCache`.

### Phase 4: TeamCreate Integration + AgentRunFn Migration (Week 5)

**Goal:** Extend team workers with provider awareness.

| Task | File | Change |
|------|------|--------|
| Extend `AgentRunFn` signature | `team_tool.rs` | Add `WorkerDispatch` parameter |
| Update `init_team_swarm_runner()` | `lib.rs` (or wherever it lives) | Pass `WorkerDispatch` through to `run_query_loop` |
| Update `TeamCreateTool::execute()` | `team_tool.rs` | Extract per-agent `WorkerParams`, call `resolve_batch`, pass `WorkerDispatch` |
| Extend team result output | `team_tool.rs` | Include per-worker provider and cost in results |
| Update `agents_view.rs` | `tui/agents_view.rs` | Show provider column |

**Verification:** TeamCreate with mixed providers (e.g., `[{provider: "ollama"}, {provider: "anthropic"}]`) dispatches correctly. Cost breakdown shows per-worker provider attribution.

### Phase 5: Polish + Documentation (Week 6)

**Goal:** Edge cases, error messages, documentation.

| Task | File | Change |
|------|------|--------|
| Improve error messages for unsatisfied capability constraints | `worker_scheduler.rs` | Descriptive errors listing available providers |
| Add `/providers` command enhancement | `commands/` | Show per-provider capability matrix |
| Handle edge case: provider removed from registry during session | `worker_scheduler.rs` | Graceful error, not panic |
| Write integration tests | `tests/` | Multi-provider scenarios |
| Update CLAUDE.md | `CLAUDE.md` | Document new fields and behavior |

---

## 14. Acceptance Criteria

### 14.1 Functional Acceptance

| # | Criterion | Verification |
|---|----------|--------------|
| AC1 | `Agent` with `provider: "openai"` routes to OpenAI provider | Integration test |
| AC2 | `Agent` with `provider: "ollama"` routes to local Ollama | Integration test (requires running Ollama) |
| AC3 | `Agent` without `provider` uses parent session's default provider | Unit test |
| AC4 | `Agent` with `required_capabilities: ["tool_calling"]` selects eligible provider | Unit test |
| AC5 | `Agent` with unsatisfiable capabilities returns descriptive error | Unit test |
| AC6 | `Agent` with `budget_usd: 0.50` stops at budget cap | Unit test with mock provider |
| AC7 | `Agent` with `access_profile: "read-only"` cannot execute Edit or Bash | Unit test |
| AC8 | `TeamCreate` with mixed providers dispatches correctly per-worker | Integration test |
| AC9 | Provider failover triggers on HTTP 503/529 | Unit test with mock HTTP |
| AC10 | `/cost` shows per-worker provider and cost breakdown | Manual verification |
| AC11 | `ANTHROPIC_API_KEY` absent + `provider: "openai"` → agent spawns successfully | Integration test |
| AC12 | Session budget exceeded → all workers cancelled | Unit test |
| AC13 | Health probe updates `HealthCache` at configured interval | Unit test with mock provider |
| AC14 | Backward compatibility: existing Agent calls (no new fields) produce identical behavior | Regression test suite |

### 14.2 Performance Acceptance

| # | Criterion | Target |
|---|----------|--------|
| PA1 | `WorkerScheduler::resolve()` latency | < 1ms (no network calls) |
| PA2 | Memory overhead per-worker | < 1KB (excluding conversation history) |
| PA3 | Health probe network usage | < 1 request/provider/minute |
| PA4 | `SessionBudget::total_cost_usd()` contention | Lock-free (atomic reads only) |

---

## 15. Test Strategy

### 15.1 Unit Tests

| Component | Test Focus | Mock Dependencies |
|-----------|-----------|-------------------|
| `CapabilityMatcher` | Correct filtering, ordering, empty results | Mock `ProviderRegistry` with 3-4 providers, each with different capabilities |
| `WorkerScheduler` | Resolution priority chain (all 5 levels) | Mock `ProviderRegistry`, `ModelRegistry`, `HealthCache` |
| `FallbackEngine` | Retry on 429, failover on 503, no fallback on 400 | Mock HTTP responses |
| `SessionBudget` | Aggregation, per-worker caps, session cap | Real `CostTracker` instances |
| `AccessProfile` | Tool filtering correctness for each profile | Real tool list from `all_tools()` |
| `HealthCache` | TTL expiry, concurrent access | Real `DashMap` |

### 15.2 Integration Tests

| Scenario | Setup | Assertion |
|----------|-------|-----------|
| Multi-provider Agent | `ProviderRegistry` with Anthropic + mock OpenAI | Agent with `provider: "openai"` hits mock OpenAI endpoint |
| Mixed-provider Team | `ProviderRegistry` with 2 providers | Each team worker hits its specified provider |
| Fallback scenario | Primary provider returns 503, secondary returns 200 | Worker completes on secondary provider |
| Budget enforcement | Worker with budget $0.01 | Worker stops after first API call |
| No Anthropic key | Only OpenAI in registry | Agent spawns successfully without ANTHROPIC_API_KEY |

### 15.3 Regression Tests

Run the existing test suite (`cargo test --workspace`) at each phase boundary. All existing tests must pass without modification (except the `AgentRunFn` signature change in Phase 4, which requires updating `init_team_swarm_runner` and any test harnesses that mock it).

### 15.4 Manual Test Scenarios

| Scenario | Steps |
|----------|-------|
| Ollama + Anthropic swarm | 1. Start Ollama locally. 2. Configure both providers. 3. Spawn a TeamCreate with one Ollama worker (search) and one Anthropic worker (synthesis). 4. Verify both complete. 5. Check `/cost` shows per-worker breakdown. |
| Provider outage simulation | 1. Configure Anthropic as default. 2. Block Anthropic API (firewall rule or mock). 3. Spawn Agent with fallback to OpenAI. 4. Verify failover logged and agent completes on OpenAI. |
| Budget cascade | 1. Set session budget to $1.00. 2. Spawn 3 workers with $0.40 budget each. 3. Verify first two complete, third is cancelled when session budget exceeded. |

---

## 16. Open Questions and Tradeoffs

### OQ1: Should `provider` and `required_capabilities` be mutually exclusive?

**Option A: Mutually exclusive.** If `provider` is set, skip capability matching entirely (the user knows what they want). If only `required_capabilities` is set, use the `CapabilityMatcher`.

**Option B: Composable.** If both are set, verify that the specified provider satisfies the required capabilities. Error if it doesn't.

**Recommendation:** Option B. It provides a safety net: `provider: "ollama", required_capabilities: ["thinking"]` would error if the Ollama model doesn't support thinking, rather than silently producing degraded output.

### OQ2: Should the `FallbackEngine` operate at the worker level or the API-call level?

**Option A: Worker level.** On spawn failure, try a different provider. Once spawned, a worker is committed to its provider.

**Option B: API-call level.** Each API call within a worker's `run_query_loop()` can fail over to a different provider.

**Recommendation:** Option A for Phase 1-2, Option B for Phase 3+. Worker-level fallback is simpler and avoids the mid-conversation message format re-serialization problem. API-call-level fallback provides better resilience but requires more careful handling of conversation state.

### <a name="oq3"></a>OQ3: Should mid-conversation provider migration be supported?

If a worker fails over mid-conversation, should the conversation history be re-serialized for the new provider and continue? Or should the worker terminate with partial results?

**Recommendation:** No mid-conversation migration in v1. The worker should terminate with partial results and an error message. The coordinator can spawn a new worker on the fallback provider with a fresh conversation that includes the partial results as context. This is simpler and avoids subtle bugs from format conversion.

### OQ4: How should `TeamCreateTool` handle mixed success/failure in `resolve_batch()`?

If a team has 5 workers and 2 fail to resolve (provider not found), should the team:
- A) Fail entirely (no workers spawn)
- B) Spawn the 3 that resolved and return errors for the 2 that didn't
- C) Ask the user before proceeding with partial team

**Recommendation:** Option B. The coordinator is better positioned to handle partial results than to completely abort. The team result should clearly indicate which workers failed and why.

### OQ5: Should there be a default `required_capabilities` set?

When no `required_capabilities` are specified, should the system assume `["tool_calling"]` (since all Claurst tools require tool calling)?

**Recommendation:** Yes. `tool_calling` should be an implicit minimum requirement for all workers. This prevents accidentally dispatching to a provider/model that cannot call tools, which would cause the worker to immediately stall.

### OQ6: Feature flag strategy

Should this feature be gated behind a feature flag (e.g., `worker_orchestration`) or always-on?

**Recommendation:** No feature flag. The changes are backward-compatible by design (all new fields are `Option`, all default to `None`). The new code paths only activate when the user specifies `provider` or `required_capabilities`. The risk of silent behavioral changes is zero: existing code paths are preserved identically.

### OQ7: How should `AgentRunFn` signature change be managed?

The `AgentRunFn` is a cross-crate internal API. Changing its signature is a breaking change for the one call site in `init_team_swarm_runner()`.

**Options:**
- A) Change signature directly (one call site, manageable)
- B) Add a new `AgentRunFnV2` and deprecate the old one
- C) Use a builder/config struct instead of positional parameters

**Recommendation:** Option C. The current signature has 6 positional parameters (already unwieldy). Replace with a struct:

```rust
pub struct AgentRunParams {
    pub description: String,
    pub prompt: String,
    pub tools: Option<Vec<String>>,
    pub system_prompt: Option<String>,
    pub max_turns: Option<u32>,
    pub ctx: Arc<ToolContext>,
    pub dispatch: Option<WorkerDispatch>,  // new, optional
}

pub type AgentRunFn = Arc<
    dyn Fn(AgentRunParams) -> Pin<Box<dyn Future<Output = String> + Send>>
    + Send + Sync,
>;
```

This is cleaner, extensible, and the migration is contained to two files (`team_tool.rs` and `init_team_swarm_runner`).

---

## 17. Final Recommendation

### Ship This RFC

The gap between root-session and sub-agent provider capabilities is the single highest-leverage architectural deficiency in Claurst today. Every other planned feature — Campaign DAG, KAIROS, ULTRAPLAN-OSS — assumes that workers can dispatch to arbitrary providers. Fixing this foundation first unblocks the entire feature roadmap.

### Implementation Priority

1. **Phase 1 (WorkerScheduler + CapabilityMatcher)** delivers the core value: multi-provider agents. Ship this first and get user feedback.
2. **Phase 2 (Budget + Access Profiles)** adds safety rails. Essential before exposing mixed-provider teams to production use.
3. **Phase 3 (Fallback + Health)** adds resilience. Important but can follow 2-3 weeks after Phase 2.
4. **Phase 4 (TeamCreate integration)** completes the story. Depends on Phases 1-3.
5. **Phase 5 (Polish)** is ongoing throughout.

### Naming Decision

Use **"Multi-Provider Worker Orchestration"** (`mpwo`) for internal references, module names, and documentation. The name "Provider-Aware Worker Fabric" may continue in user-facing materials if desired, but the internal codebase should use the more precise term.

### Risk Assessment

| Category | Level | Justification |
|----------|-------|---------------|
| Technical risk | **Low** | The provider infrastructure already exists. This RFC is plumbing, not invention. |
| Backward compatibility risk | **Minimal** | All changes are additive. Existing behavior preserved when new fields are absent. |
| Operational risk | **Low** | Fallback and budget enforcement reduce operational risk compared to the status quo (hard-fail on Anthropic outage). |
| Scope risk | **Medium** | Phase 3 (FallbackEngine) has nuanced edge cases around message format conversion. Mitigated by the recommendation to defer mid-conversation migration. |

### Lines of Code Estimate

| Component | New LoC | Modified LoC |
|-----------|---------|-------------|
| `worker_scheduler.rs` | ~250 | — |
| `capability_matcher.rs` | ~120 | — |
| `fallback_engine.rs` | ~200 | — |
| `health_probe.rs` | ~150 | — |
| `session_budget.rs` | ~100 | — |
| `agent_tool.rs` | — | ~80 (replace AnthropicClient path) |
| `team_tool.rs` | ~40 (WorkerDispatch, AgentRunParams) | ~60 (resolve + pass dispatch) |
| `lib.rs` (query) | — | ~30 (QueryConfig fields, init_team_swarm_runner) |
| `lib.rs` (core) | — | ~10 (CostTracker extensions) |
| `lib.rs` (tools) | ~30 (AccessProfile) | ~10 (ToolContext fields) |
| `main.rs` (cli) | — | ~20 (construct WorkerScheduler) |
| Tests | ~400 | — |
| **Total** | **~1,290 new** | **~210 modified** |

This is a well-scoped change: ~1,500 lines across 6 new files and 6 modified files.

---

*End of RFC. This document was authored 2026-04-08 to guide implementation of multi-provider worker orchestration in the Claurst codebase.*
