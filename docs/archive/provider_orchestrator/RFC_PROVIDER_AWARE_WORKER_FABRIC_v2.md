# Architecture RFC: Multi-Provider Worker Orchestration

**Unified Provider Resolution for Root Sessions, Agents, and Teams in Claurst**

| Field | Value |
|-------|-------|
| **RFC ID** | RFC-0001-v2 |
| **Status** | Draft |
| **Author** | Claurst Core Team |
| **Date** | 2026-04-08 |
| **Supersedes** | RFC-0001-v1 (Provider-Aware Worker Fabric) |
| **Codebase Version** | `acae926` (2026-04-07) |
| **Affects** | `claurst-query`, `claurst-api`, `claurst-tools`, `claurst-core` |
| **Review History** | v1 reviewed by Codex/GPT-5.4 (2026-04-08); 11 findings incorporated |

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
13. [Rollout Plan](#13-rollout-plan)
14. [Acceptance Criteria](#14-acceptance-criteria)
15. [Test Strategy](#15-test-strategy)
16. [Open Questions and Tradeoffs](#16-open-questions-and-tradeoffs)
17. [Final Recommendation](#17-final-recommendation)
18. [Appendix A: v1 Review Findings and Dispositions](#appendix-a-v1-review-findings-and-dispositions)

---

## 1. Executive Summary

Claurst's root query loop (`run_query_loop()` in `crates/query/src/lib.rs:660`) supports multi-provider routing across 12+ LLM providers via `ProviderRegistry`. Sub-agents spawned by `AgentTool` and parallel workers launched by `TeamCreateTool` do not — they hardcode `AnthropicClient::new(...)` and fail when `ANTHROPIC_API_KEY` is absent.

This RFC proposes a **two-deliverable** fix:

**Deliverable 1 (Minimal Fix):** Extract the existing provider-resolution logic from `run_query_loop()` into a shared `resolve_provider()` function, then propagate `ProviderRegistry` and `ModelRegistry` into child `QueryConfig` for both `AgentTool` and `TeamCreateTool`. This eliminates the hardcoded Anthropic path and enables explicit per-worker `provider` and `model` selection. Estimated scope: **~800 LoC** (including tests).

**Deliverable 2 (Expansion):** Add soft-budget enforcement via `SessionBudget`, spawn-time health checks via `HealthCache`, and spawn-time fallback (no mid-conversation provider migration). Estimated scope: **~1,200 LoC** additional.

v1 of this RFC bundled both deliverables and introduced inconsistencies between the fallback model, capability matching granularity, and backward-compatibility claims. This v2 narrows the core proposal, resolves those inconsistencies, and defers access profiles and per-API-call fallback to follow-on work.

---

## 2. Problem Statement

### 2.1 The Break in Provider Symmetry

The root session resolves its provider through a well-defined priority chain (lines 854-926 of `crates/query/src/lib.rs`):

1. Explicit `provider/model` format in the model string (e.g., `openai/gpt-4.1`)
2. `config.provider` setting (from `--provider` flag or `settings.json`)
3. `ModelRegistry` lookup (e.g., `gemini-3-flash-preview` → `google`)
4. Default to `anthropic`

This logic lives **inline** within `run_query_loop()`. It is not a reusable function.

Sub-agents bypass this entire path. There are **three** hardcoded Anthropic construction sites:

| Site | Location | Context |
|------|----------|---------|
| Agent foreground | `agent_tool.rs:230-248` | `AgentTool::execute()` reads `ANTHROPIC_API_KEY`, constructs `AnthropicClient` |
| Agent background | `agent_tool.rs:541` | Background agent closure, same pattern |
| Team runner | `lib.rs:528-541` (`init_team_swarm_runner`) | `AgentRunFn` closure, same pattern |

All three set `provider_registry: None` and `model_registry: None` in the child `QueryConfig`, which means the child `run_query_loop()` never enters its provider-dispatch branch (line 862: `if let Some(ref registry) = config.provider_registry`).

### 2.2 Consequences

| Scenario | Current Behavior | Expected Behavior |
|----------|-----------------|-------------------|
| User sets `--provider openai` and spawns an Agent | Fails: "ANTHROPIC_API_KEY not set" | Agent uses OpenAI via parent's ProviderRegistry |
| Coordinator on Ollama creates a TeamCreate | All workers fail | Workers route through Ollama |
| Agent specifies `model: "gemini-3-flash-preview"` | Ignored; uses DEFAULT_MODEL on Anthropic | ModelRegistry resolves to GoogleProvider |
| Mixed swarm: local + cloud workers | Not possible | Workers dispatch to specified providers |

### 2.3 Scope of the Gap

The bug is narrow: three call sites construct `AnthropicClient` directly. The fix is **extracting and reusing the existing resolution logic**, not inventing a new dispatch system.

---

## 3. Current State and Architectural Gap

### 3.1 What Already Works

| Component | Location | Status |
|-----------|----------|--------|
| `ProviderRegistry` | `crates/api/src/registry.rs` | 35+ providers, health-check API, default provider |
| `ModelRegistry` | `crates/api/src/model_registry.rs` | Dynamic model→provider resolution via models.dev |
| `ProviderCapabilities` | `crates/api/src/provider_types.rs` | 9 capability flags (provider-level) |
| `ProviderStatus` | `crates/api/src/provider_types.rs` | Health enum: Healthy / Degraded / Unavailable |
| `LlmProvider` trait | `crates/api/src/provider.rs` | `create_message_stream()`, `health_check()`, `capabilities()` |
| `CostTracker` | `crates/core/src/lib.rs:2741` | Atomic token/cost accumulation, model-aware pricing |
| Root provider dispatch | `crates/query/src/lib.rs:854-926` | Full 4-step resolution chain |
| `AgentTool` worktree isolation | `crates/query/src/agent_tool.rs` | Git worktree creation, background polling |
| `TeamCreateTool` cancellation | `crates/tools/src/team_tool.rs` | Per-agent CancellationToken via DashMap |
| `run_query_loop()` signature | `crates/query/src/lib.rs:660` | Takes `client: &AnthropicClient` as first param |

### 3.2 What Does Not Work

| Component | Location | Gap |
|-----------|----------|-----|
| Agent provider dispatch | `agent_tool.rs:230-248`, `:541` | Hardcoded `AnthropicClient::new(...)` |
| Agent QueryConfig | `agent_tool.rs:339-360`, `:575-583` | `provider_registry: None`, `model_registry: None` |
| `AgentRunFn` signature | `team_tool.rs:46-57` | No provider/model parameters |
| `run_query_loop()` first param | `lib.rs:661` | `client: &AnthropicClient` — assumes Anthropic as fallback |
| Provider resolution reuse | `lib.rs:854-926` | Inline in `run_query_loop()`, not extractable |

### 3.3 Architectural Diagram — Current State

```
 Root Session
 ┌───────────────────────────────────────────────┐
 │ run_query_loop(client: &AnthropicClient, ...) │
 │   QueryConfig {                               │
 │     provider_registry: Some(Arc<PR>),          │
 │     model_registry: Some(Arc<MR>),             │
 │   }                                           │
 │   Provider dispatch @ line 854-926            │
 │     ├── anthropic (via client param)          │
 │     ├── openai (via registry)                 │
 │     ├── google, ollama, ... (via registry)    │
 └───────┬───────────────────────────────────────┘
         │
         │  AgentTool.execute() / init_team_swarm_runner()
         ▼
 Sub-Agent / Team Worker
 ┌───────────────────────────────────────────────┐
 │ AnthropicClient::new(ANTHROPIC_API_KEY)       │  ◄── hardcoded
 │ run_query_loop(client, ...) {                 │
 │   QueryConfig {                               │
 │     provider_registry: None,                  │  ◄── gap
 │     model_registry: None,                     │  ◄── gap
 │   }                                           │
 │   // Provider dispatch branch never entered   │
 │ }                                             │
 └───────────────────────────────────────────────┘
```

---

## 4. Goals and Non-Goals

### 4.1 Goals

| # | Goal | Deliverable |
|---|------|-------------|
| G1 | **Shared provider resolution.** Extract the root's resolution logic into a reusable function. Both root and workers use the same code path. | D1 |
| G2 | **Registry propagation.** Workers inherit `ProviderRegistry` and `ModelRegistry` from the parent. | D1 |
| G3 | **Explicit provider/model selection.** Agent and TeamCreate callers can specify `provider` and `model` per worker. | D1 |
| G4 | **Fix both AgentTool and TeamCreateTool.** Both broken paths fixed in the same deliverable. | D1 |
| G5 | **Backward compatibility with explicit risk acknowledgment.** Workers on a non-Anthropic parent session now inherit that parent's provider — this is the desired fix, but it is a behavior change. | D1 |
| G6 | **Soft-budget enforcement.** Per-worker and session-level USD caps, best-effort (not hard guarantees). | D2 |
| G7 | **Spawn-time health check.** Before dispatching a worker to a provider, verify the provider is reachable. | D2 |
| G8 | **Spawn-time fallback.** If the requested provider is unreachable at spawn time, fall back to an alternative that meets the same constraints — but only within the same trust domain (see Section 7.5). | D2 |

### 4.2 Non-Goals

| # | Non-Goal | Rationale |
|---|----------|-----------|
| NG1 | **Per-API-call fallback / mid-conversation provider migration.** Too complex for v1. Requires message re-serialization across format boundaries. Deferred to future RFC. | v1 review finding #2 |
| NG2 | **Access profiles (full/read-only/search-only).** Worker authorization is a separate concern from provider dispatch. Deferred to follow-on RFC. | v1 review finding #9 |
| NG3 | **Campaign DAG / artifact bus.** Separate problem domain. | Scope |
| NG4 | **Runtime orchestration (Docker/SSH).** Belongs in ULTRAPLAN-OSS. | Scope |
| NG5 | **Model auto-selection via ML.** Capability matching is rule-based. | Complexity |
| NG6 | **Batch resolution policies (fairness, anti-concentration).** `resolve_batch()` removed; workers resolve individually. | v1 review finding #10 |
| NG7 | **TUI enhancements.** Provider column in agents_view, enhanced `/cost` — desirable but not part of the core fix. Can ship independently. | Scope |

---

## 5. Terminology and Naming Clarification

### 5.1 Feature Name

**Formal name:** **Multi-Provider Worker Orchestration** (abbreviated **MPWO**).

The v1 name "Provider-Aware Worker Fabric" implied a mesh topology that does not exist. "Multi-Provider Worker Orchestration" describes the actual mechanism: resolving providers for workers and coordinating their execution.

Code modules and feature-flag references use `mpwo` or `worker_orchestration`.

### 5.2 Glossary

| Term | Definition |
|------|-----------|
| **Root session** | The top-level `run_query_loop()` instance driven by the TUI or `--print` mode. Owns the `ProviderRegistry`. |
| **Worker** | Any `run_query_loop()` instance spawned by `AgentTool` or `TeamCreateTool`. |
| **Provider** | An `LlmProvider` implementation in the `ProviderRegistry`. Identified by `ProviderId`. |
| **Model** | A specific model ID within a provider. Resolved via `ModelRegistry`. |
| **Execution target** | A resolved `(ProviderId, model_id, Arc<dyn LlmProvider>)` tuple ready for dispatch. |
| **Trust domain** | A classification of providers by data-handling boundary: `local` (Ollama, LM Studio, llama.cpp — data never leaves the machine), `cloud` (all others). Fallback never crosses trust-domain boundaries unless explicitly opted in. |
| **Pinned provider** | An explicit `provider` in the worker request. The system must use this provider or fail — no fallback to a different provider. Default behavior. |
| **Preferred provider** | A `provider` with `allow_fallback: true`. The system tries this provider first but may fall back to another in the same trust domain. |

### 5.3 Trust Domains

This concept was absent from v1 and raised as finding #1 in review. Providers are classified:

```rust
pub enum TrustDomain {
    /// Data stays on the local machine. Providers: ollama, lmstudio, llamacpp.
    Local,
    /// Data is sent to a cloud endpoint.
    Cloud,
}
```

A user who specifies `provider: "ollama"` almost certainly intends "keep this data local." Fallback to a cloud provider would violate that intent. Conversely, fallback from one cloud provider to another (e.g., Anthropic → OpenAI) is generally acceptable if the user opts in.

**Rule:** Fallback never crosses trust domain boundaries. `Local → Local` and `Cloud → Cloud` are permitted. `Local → Cloud` is forbidden unless the user explicitly sets `allow_cross_domain_fallback: true`.

---

## 6. Target Architecture

### 6.1 Architectural Diagram — Target State (Deliverable 1)

```
 Root Session
 ┌───────────────────────────────────────────────────────┐
 │ run_query_loop(client, ...) {                         │
 │   QueryConfig {                                       │
 │     provider_registry: Some(Arc<ProviderRegistry>),   │
 │     model_registry: Some(Arc<ModelRegistry>),          │
 │   }                                                   │
 │                                                       │
 │   // Provider dispatch uses resolve_provider()  NEW   │
 │   let target = resolve_provider(                      │
 │     &effective_model, &config, &tool_ctx.config        │
 │   );                                                  │
 │ }                                                     │
 └───────┬─────────────────────┬─────────────────────────┘
         │                     │
         │ AgentTool           │ TeamCreateTool
         ▼                     ▼
 ┌──────────────────────────────────────────────────────┐
 │     resolve_provider()  (same function)               │
 │  Input:  model string, provider override,             │
 │          ProviderRegistry, ModelRegistry               │
 │  Output: (ProviderId, model_id, Arc<dyn LlmProvider>) │
 └───────────────────────┬──────────────────────────────┘
                         │
         ┌───────────────┼──────────────────┐
         ▼               ▼                  ▼
 ┌──────────────┐ ┌──────────────┐  ┌──────────────┐
 │ Worker A     │ │ Worker B     │  │ Worker C     │
 │ anthropic/   │ │ ollama/      │  │ google/      │
 │ opus-4-6     │ │ llama-3.3    │  │ gemini-3     │
 │ CostTracker  │ │ CostTracker  │  │ CostTracker  │
 └──────────────┘ └──────────────┘  └──────────────┘
```

### 6.2 Architectural Diagram — Target State (Deliverable 2 additions)

```
                    ┌─────────────────────┐
                    │   HealthCache       │
                    │   (DashMap, async    │
                    │    probed on demand) │
                    └─────────┬───────────┘
                              │
 resolve_provider()           │ health check before dispatch
        │                     │
        ▼                     ▼
 ┌─────────────────────────────────────────────────┐
 │ Spawn-time fallback (Deliverable 2 only):       │
 │   if target.health == Unavailable:              │
 │     find next eligible in same trust domain     │
 │     or fail with descriptive error              │
 │                                                 │
 │ Budget enforcement (soft):                      │
 │   per-worker CostTracker with optional USD cap  │
 │   SessionBudget aggregates all workers          │
 └─────────────────────────────────────────────────┘
```

### 6.3 Key Design Principles

1. **Extract, don't duplicate.** The root's provider-resolution logic (lines 854-926 of `lib.rs`) is extracted into a shared `resolve_provider()` function. Both root and workers call it. One resolution path, one set of precedence rules, one set of error messages. *(Addresses v1 finding #3.)*

2. **Pinned by default.** An explicit `provider` means "use this provider or fail." Fallback is opt-in via `allow_fallback: true`. *(Addresses v1 finding #1.)*

3. **No mid-conversation migration.** Fallback is evaluated at spawn time only. A worker that starts on provider X stays on provider X for its entire session. If X goes down mid-conversation, the worker fails with partial results. *(Addresses v1 finding #2.)*

4. **Trust domains are inviolable.** Fallback from a local provider to a cloud provider never happens without explicit user consent. *(Addresses v1 finding #1.)*

5. **Budget enforcement is soft.** `CostTracker` is post-hoc accounting. Under concurrency, a worker can overshoot its cap by one API call. The RFC does not pretend otherwise. *(Addresses v1 finding #5.)*

6. **Capability matching requires model-level metadata.** Provider-level `ProviderCapabilities` is insufficient — models within one provider differ in capability. Deliverable 2 extends `ModelRegistry` with per-model capability data. *(Addresses v1 finding #4.)*

---

## 7. Component Design

### 7.1 `resolve_provider()` — Shared Resolution Function (Deliverable 1)

**Location:** `crates/query/src/provider_resolution.rs` (new file, extracted from `lib.rs:854-926`)

This is the core of Deliverable 1. The existing inline resolution logic is extracted verbatim into a standalone function, then called from both the root loop and worker spawn paths.

```rust
/// The resolved execution target for an API call.
pub struct ExecutionTarget {
    pub provider_id: ProviderId,
    pub model_id: String,
    pub provider: Arc<dyn LlmProvider>,
}

/// Resolve a provider and model for a given configuration.
///
/// Resolution priority (unchanged from current root behavior):
///   1. Explicit `provider_override` (from worker params or config.provider)
///   2. "provider/model" format in the model string
///   3. ModelRegistry lookup (model name → provider)
///   4. Default to the registry's default provider (typically "anthropic")
///
/// This function is called by:
///   - run_query_loop() for root session dispatch
///   - AgentTool::execute() for sub-agent dispatch
///   - init_team_swarm_runner() for team worker dispatch
pub fn resolve_provider(
    model: &str,
    provider_override: Option<&str>,
    registry: &ProviderRegistry,
    model_registry: Option<&ModelRegistry>,
) -> Result<ExecutionTarget, ProviderResolutionError>;

#[derive(Debug, thiserror::Error)]
pub enum ProviderResolutionError {
    #[error("Provider '{0}' not found in registry. Available: {1}")]
    ProviderNotFound(String, String),

    #[error("No provider could be resolved for model '{0}'")]
    NoProviderForModel(String),

    #[error("Provider '{0}' has no API key configured. Run /connect or set the appropriate environment variable.")]
    NoCredentials(String),
}
```

**Migration plan for `run_query_loop()`:** The inline code at lines 854-926 is replaced with a call to `resolve_provider()`. The `client: &AnthropicClient` parameter remains for now as the Anthropic-path fallback, but `resolve_provider()` handles all non-Anthropic dispatch. A future cleanup can remove the `AnthropicClient` parameter entirely once all callers go through the registry.

### 7.2 ProviderRegistry (Existing — No Changes)

**Location:** `crates/api/src/registry.rs`

No modifications needed. The registry is a passive lookup table. `resolve_provider()` composes with it.

### 7.3 ModelRegistry (Existing — Extended in D2)

**Location:** `crates/api/src/model_registry.rs`

**Deliverable 1:** No changes. Model-to-provider resolution already works.

**Deliverable 2:** Extend `ModelEntry` with per-model capability metadata:

```rust
/// Extended in D2 to support model-level capability matching.
pub struct ModelEntry {
    pub model_id: String,
    pub provider_id: ProviderId,
    pub display_name: Option<String>,
    // D2 additions:
    pub capabilities: Option<ModelCapabilities>,  // NEW
}

/// Model-level capabilities (finer than ProviderCapabilities).
/// Optional — when absent, falls back to provider-level capabilities.
pub struct ModelCapabilities {
    pub tool_calling: bool,
    pub thinking: bool,
    pub image_input: bool,
    pub pdf_input: bool,
    pub max_context_tokens: Option<u32>,
    pub max_output_tokens: Option<u32>,
}
```

This addresses v1 finding #4: capability matching must be model-specific, not provider-specific. Within one provider, `gpt-4.1` supports tool calling but `gpt-4.1-nano` may not. Without model-level data, the capability matcher makes incorrect selections.

**Data source:** The models.dev API already returns per-model metadata. `ModelRegistry::load_cache()` parses this. The extension adds structured capability extraction from the existing data.

### 7.4 `TrustDomain` Classification (Deliverable 2)

**Location:** `crates/api/src/provider_types.rs` (add to existing file)

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrustDomain {
    Local,
    Cloud,
}

impl TrustDomain {
    pub fn for_provider(provider_id: &str) -> Self {
        match provider_id {
            "ollama" | "lmstudio" | "llamacpp" => TrustDomain::Local,
            _ => TrustDomain::Cloud,
        }
    }
}
```

This is a simple classification. The list of local providers is hardcoded because it is small and stable. Users who run custom local endpoints through OpenAI-compatible adapters can override via configuration (see Section 9.2).

### 7.5 Spawn-Time Fallback (Deliverable 2)

**Location:** `crates/query/src/provider_resolution.rs` (extend the D1 file)

Fallback is evaluated **once, at spawn time**, not per API call. This is the single v1 model recommended by the review.

```rust
/// Resolve with fallback. Tries the primary target first; on health-check
/// failure, tries alternatives within the same trust domain.
///
/// Returns the first healthy target, or an error listing all attempted providers.
pub fn resolve_provider_with_fallback(
    model: &str,
    provider_override: Option<&str>,
    allow_fallback: bool,
    allow_cross_domain: bool,
    registry: &ProviderRegistry,
    model_registry: Option<&ModelRegistry>,
    health_cache: &HealthCache,
    required_capabilities: &[Capability],
) -> Result<ExecutionTarget, ProviderResolutionError>;
```

**Fallback rules:**

| Scenario | Behavior |
|----------|----------|
| Explicit `provider`, `allow_fallback: false` (default) | Use specified provider or fail. **No fallback.** |
| Explicit `provider`, `allow_fallback: true` | Try specified provider; on failure, try others in same trust domain that satisfy capability constraints. |
| No explicit `provider` | Use resolution priority chain. On failure, try alternatives in same trust domain. |
| `allow_cross_domain_fallback: true` | Permit `Local → Cloud` fallback (rarely desired, never default). |

**What "spawn-time" means:** `resolve_provider_with_fallback()` runs a lightweight health check (HTTP HEAD or equivalent) against the resolved provider before returning. If the provider is unreachable, it tries alternatives. Once a worker starts, it is committed to its resolved provider for the duration.

### 7.6 HealthCache (Deliverable 2)

**Location:** `crates/query/src/health_cache.rs` (new file)

```rust
pub struct HealthCache {
    statuses: DashMap<ProviderId, (ProviderStatus, Instant)>,
    ttl: Duration,
}

impl HealthCache {
    pub fn new(ttl: Duration) -> Self;

    /// Get cached status, or None if expired/missing.
    pub fn get(&self, id: &ProviderId) -> Option<ProviderStatus>;

    /// Update status for a provider.
    pub fn update(&self, id: ProviderId, status: ProviderStatus);

    /// Probe a provider and update the cache. Called by resolve_provider_with_fallback().
    pub async fn probe_if_stale(
        &self,
        id: &ProviderId,
        provider: &dyn LlmProvider,
    ) -> ProviderStatus;
}
```

**Design decision: no background probe task.** v1 proposed a periodic `HealthProbe` background task. This is unnecessary for Deliverable 2. Instead, `resolve_provider_with_fallback()` probes on demand, using cached results when fresh. This eliminates the probe lifecycle/shutdown edge case (v1 finding #11) and avoids unnecessary network traffic when no workers are being spawned.

A background probe can be added later if spawn-time latency becomes a problem.

### 7.7 Per-Worker CostTracker and SessionBudget (Deliverable 2)

**Location:** Extend `CostTracker` in `crates/core/src/lib.rs`; new `crates/query/src/session_budget.rs`

#### 7.7.1 CostTracker Extension

```rust
pub struct CostTracker {
    input_tokens: AtomicU64,
    output_tokens: AtomicU64,
    cache_creation_tokens: AtomicU64,
    cache_read_tokens: AtomicU64,
    pricing: parking_lot::RwLock<ModelPricing>,
    agent_id: Option<String>,         // NEW — attribution label
    provider_id: Option<String>,      // NEW — which provider served this worker
}
```

Two new optional fields for attribution. No behavioral change to existing callers.

#### 7.7.2 SessionBudget

```rust
pub struct SessionBudget {
    max_budget_usd: Option<f64>,
    workers: DashMap<String, Arc<CostTracker>>,
    root_tracker: Arc<CostTracker>,
}

impl SessionBudget {
    pub fn register_worker(&self, agent_id: String, tracker: Arc<CostTracker>);
    pub fn total_cost_usd(&self) -> f64;
    pub fn worker_costs(&self) -> Vec<(String, Option<String>, f64)>; // (agent_id, provider_id, cost)
    pub fn is_exceeded(&self) -> bool;
}
```

**Budget enforcement is explicitly soft.** From the RFC:

> Per-worker budget enforcement is best-effort. Under concurrency, multiple workers can exceed their individual caps simultaneously. A single oversized API response can overshoot a cap by its full cost. The `SessionBudget` checks are evaluated after each completed API turn, not before each API call. This means a worker can exceed its budget by at most the cost of one API round-trip.
>
> **This is acceptable.** Preflight cost estimation is unreliable (model output length is unpredictable), and reservation-based admission control would add complexity disproportionate to the risk. The soft guarantee is: the system notices overspend within one turn and stops the worker.

---

## 8. Execution Model

### 8.1 Root Session (Deliverable 1 Change)

The only change to the root session is replacing the inline resolution logic with a call to `resolve_provider()`:

**Before (`lib.rs:854-926`, inline):**
```rust
if let Some(ref registry) = config.provider_registry {
    let (provider_id_str, model_id_str) = if let Some(p) = ... {
        // ... 70 lines of resolution logic
    };
    // dispatch
}
```

**After:**
```rust
if let Some(ref registry) = config.provider_registry {
    match resolve_provider(&effective_model, config_provider, registry, config.model_registry.as_deref()) {
        Ok(target) => {
            // dispatch using target.provider
        }
        Err(e) => {
            // fall back to client param (AnthropicClient) — preserves existing behavior
        }
    }
}
```

The `client: &AnthropicClient` parameter is retained as the Anthropic-path fallback. This ensures zero behavioral change for the common case (user on Anthropic, no `provider_registry` set).

### 8.2 AgentTool (Deliverable 1 Change)

**Before (`agent_tool.rs:230-248`):**
```rust
let api_key = std::env::var("ANTHROPIC_API_KEY")...;
let client = AnthropicClient::new(ClientConfig { api_key, .. });
// ...
let query_config = QueryConfig {
    provider_registry: None,
    model_registry: None,
    // ...
};
```

**After:**
```rust
// Get the parent's registries.
let registry = ctx.worker_scheduler_registry
    .as_ref()
    .ok_or_else(|| "No ProviderRegistry available")?;

// Resolve provider: explicit params > parent's config > default.
let target = resolve_provider(
    &resolved_model,
    params.provider.as_deref(),
    registry,
    ctx.model_registry.as_deref(),
)?;

// Build a per-worker CostTracker.
let worker_cost_tracker = CostTracker::with_model(&target.model_id);

let query_config = QueryConfig {
    model: target.model_id.clone(),
    provider_registry: Some(registry.clone()),      // PROPAGATED
    model_registry: ctx.model_registry.clone(),       // PROPAGATED
    max_budget_usd: params.budget_usd,
    // ...
};

// Use target.provider instead of AnthropicClient.
```

The same change applies to the background-agent path (`agent_tool.rs:541`).

**Backward compatibility:** When `params.provider` is `None` and `params.model` is `None`, `resolve_provider()` follows the same priority chain as the root, which defaults to the parent's configured provider. If the parent is on Anthropic, the worker is on Anthropic. If the parent is on OpenAI, the worker is on OpenAI. **This is a behavior change** for the case where a user is on a non-Anthropic provider: previously, agents would fail (ANTHROPIC_API_KEY not set); now they succeed on the parent's provider. This is the intended fix, but it is explicitly acknowledged as a behavior change.

### 8.3 TeamCreateTool (Deliverable 1 Change)

The `AgentRunFn` callback is updated (see Section 9.3). The team runner in `init_team_swarm_runner()` (`lib.rs:520-604`) replaces its `AnthropicClient::new()` with the same `resolve_provider()` path.

**Both AgentTool and TeamCreateTool are fixed in Deliverable 1.** This addresses v1 finding #6.

### 8.4 Parallel Workers

No change from v1. Workers are isolated at the conversation level, share the filesystem (unless worktree-isolated), and share the `ProviderRegistry` (read-only).

### 8.5 Spawn-Time Fallback (Deliverable 2)

When a worker's primary provider is unreachable at spawn time:

```
Worker A requested: anthropic/claude-opus-4-6, allow_fallback: true
  │
  ├── health_cache.probe_if_stale("anthropic") → Unavailable
  │
  ├── Find alternatives in TrustDomain::Cloud:
  │   ├── openai (Healthy, supports tool_calling) → selected
  │
  ├── Log: "Worker A: anthropic unavailable, falling back to openai/gpt-4.1"
  │
  └── Worker A starts on openai/gpt-4.1
      └── Worker stays on openai for entire session (no mid-run migration)
```

**If `allow_fallback: false` (default):**
```
Worker A requested: anthropic/claude-opus-4-6
  │
  ├── health_cache.probe_if_stale("anthropic") → Unavailable
  │
  └── Error: "Provider 'anthropic' is unavailable. Set allow_fallback: true to enable automatic fallback."
```

**If the provider is local:**
```
Worker B requested: ollama/llama-3.3, allow_fallback: true
  │
  ├── health_cache.probe_if_stale("ollama") → Unavailable
  │
  ├── Find alternatives in TrustDomain::Local:
  │   ├── lmstudio (Healthy) → selected
  │
  └── Worker B starts on lmstudio
      (Never falls back to a Cloud provider)
```

---

## 9. Proposed Schema and Interface Changes

### 9.1 Agent Tool Input Schema (Extended)

New optional fields:

```json
{
  "provider": {
    "type": "string",
    "description": "Provider to use (e.g., anthropic, openai, ollama). Pinned by default."
  },
  "allow_fallback": {
    "type": "boolean",
    "default": false,
    "description": "If true, allow fallback to another provider in the same trust domain on failure."
  },
  "budget_usd": {
    "type": "number",
    "description": "Soft USD spending cap for this worker (D2)."
  }
}
```

All new fields are optional. Omitting them produces identical behavior to today (after the fix).

**Removed from v1:** `required_capabilities` and `access_profile` are deferred. Capability matching is a D2 concern that requires model-level metadata (Section 7.3). Access profiles are deferred to a follow-on RFC (v1 finding #9).

### 9.2 TeamCreate Agent Entry Schema (Extended)

Same new optional fields per agent:

```json
{
  "provider": { "type": "string" },
  "model": { "type": "string" },
  "allow_fallback": { "type": "boolean", "default": false },
  "budget_usd": { "type": "number" }
}
```

### 9.3 AgentRunFn Signature (Breaking Internal Change)

The `AgentRunFn` in `team_tool.rs` is replaced with a struct-based signature:

**Before (6 positional parameters):**
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

**After (struct-based):**
```rust
pub struct AgentRunParams {
    pub description: String,
    pub prompt: String,
    pub tools: Option<Vec<String>>,
    pub system_prompt: Option<String>,
    pub max_turns: Option<u32>,
    pub ctx: Arc<ToolContext>,
    // D1 additions:
    pub provider_override: Option<String>,
    pub model_override: Option<String>,
    // D2 additions:
    pub budget_usd: Option<f64>,
    pub allow_fallback: bool,
}

pub type AgentRunFn = Arc<
    dyn Fn(AgentRunParams) -> Pin<Box<dyn Future<Output = String> + Send>>
    + Send + Sync,
>;
```

This is cleaner than v1's `WorkerDispatch` struct, which tried to pre-resolve the provider before the runner saw it. Instead, the runner receives the raw parameters and calls `resolve_provider()` itself — the same path as `AgentTool::execute()`.

**Migration:** Two call sites change: `init_team_swarm_runner()` (producer) and `run_agent()` in `team_tool.rs` (consumer). Both are internal.

### 9.4 QueryConfig Extensions

```rust
pub struct QueryConfig {
    // ... existing fields unchanged ...

    // D2 only:
    pub session_budget: Option<Arc<SessionBudget>>,
}
```

Note: `worker_scheduler` from v1 is removed. The `resolve_provider()` function replaces it — no scheduler object needed.

### 9.5 ToolContext Extensions

```rust
pub struct ToolContext {
    // ... existing fields unchanged ...

    // D1: parent's registries, for workers that need to spawn sub-workers
    // (not currently used, but prevents future re-introduction of the gap)
    pub provider_registry: Option<Arc<ProviderRegistry>>,
    pub model_registry: Option<Arc<ModelRegistry>>,

    // D2:
    pub session_budget: Option<Arc<SessionBudget>>,
}
```

### 9.6 User-Visible Changes Acknowledgment

v1 claimed "zero new tools / internal plumbing only." This was overstated (v1 finding #8). The actual user-visible changes are:

| Change | Visibility |
|--------|-----------|
| New `provider`, `allow_fallback`, `budget_usd` fields on Agent/TeamCreate | Visible in tool schemas; model may use them |
| Workers inherit parent provider instead of failing | Behavior change (desired) |
| Error messages change from "ANTHROPIC_API_KEY not set" to descriptive resolution errors | Behavior change (improved) |
| `/cost` may show per-worker breakdown (D2) | UI enhancement |

---

## 10. Permission, Isolation, and Security Considerations

### 10.1 API Key Isolation

Workers inherit provider references (`Arc<dyn LlmProvider>`) from the parent's `ProviderRegistry`. Raw API keys are encapsulated inside provider implementations and not extractable via the `LlmProvider` trait. Workers cannot register new providers or access credentials not configured by the user.

### 10.2 Trust Domain Enforcement

The `TrustDomain` classification (Section 7.4) prevents accidental data leakage:

- A worker pinned to `ollama` will never fall back to a cloud provider.
- The `allow_cross_domain_fallback` flag exists as an escape hatch but defaults to `false`.
- Trust domain classification is determined by provider ID, not by network topology. A user running a custom OpenAI-compatible endpoint locally must register it as a local provider in config to get local trust domain treatment.

### 10.3 Worktree Isolation

Unchanged from current behavior. Orthogonal to provider selection.

### 10.4 Budget as a Safety Boundary

Per-worker soft budgets prevent runaway cost. A worker exceeding its cap is stopped after the current turn. The session budget provides a secondary cap. Neither is a hard guarantee (see Section 7.7.2).

---

## 11. Failure Modes and Operational Risks

### 11.1 Failure Mode Catalog

| # | Failure Mode | Deliverable | Mitigation |
|---|-------------|-------------|------------|
| F1 | Requested provider not in registry | D1 | `ProviderResolutionError::ProviderNotFound` with list of available providers |
| F2 | No API key for resolved provider | D1 | `ProviderResolutionError::NoCredentials` with instructions to run `/connect` |
| F3 | Provider unreachable, fallback disabled (default) | D2 | Error with suggestion to set `allow_fallback: true` |
| F4 | Provider unreachable, all fallbacks in same trust domain exhausted | D2 | Error listing all attempted providers and their statuses |
| F5 | Worker exceeds individual budget | D2 | `QueryOutcome::BudgetExceeded`, partial results returned |
| F6 | Session budget exceeded | D2 | Cancellation tokens fired for all workers |
| F7 | Conflicting `provider` and `model` (model not available on specified provider) | D1 | `resolve_provider()` detects mismatch and returns error: "Model 'X' is not available on provider 'Y'" |
| F8 | Context-window mismatch after model change | D1 | Worker uses the resolved model's context window. If the model has a smaller window than expected, auto-compact handles it (existing mechanism) |
| F9 | Stale health cache | D2 | `probe_if_stale()` re-probes when TTL expires. Stale = probe, not stale = use cache |
| F10 | `ProviderRegistry` is `None` in ToolContext (legacy code path) | D1 | Fall back to `ANTHROPIC_API_KEY` env var (existing behavior preserved) |

### 11.2 Backward-Compatibility Risk

v1 claimed "zero risk of silent behavioral changes." This is incorrect (v1 finding #7).

**The behavior change:** A user on `--provider openai` who spawns an Agent will now see that agent succeed on OpenAI, where it previously failed with "ANTHROPIC_API_KEY not set." This is the desired fix, but it changes observable behavior.

**Mitigation:** No config toggle is needed. The old behavior (hard failure) was never desirable. Users who were affected by this bug will see improvement, not regression. Users on the default Anthropic provider see no change.

**Edge case:** A user who set `--provider openai` but relied on agents failing (e.g., to avoid agent costs) will now see agents succeed and incur costs. This is unlikely but documented.

### 11.3 AgentRunFn Signature Break

The `AgentRunFn` type change (Section 9.3) is a breaking internal API change. It affects exactly two call sites:

1. `init_team_swarm_runner()` in `crates/query/src/lib.rs` (producer)
2. `run_agent()` in `crates/tools/src/team_tool.rs` (consumer)

Both are in the same workspace. The change is coordinated and compiled together.

---

## 12. Observability and Telemetry Requirements

### 12.1 Structured Logging

All resolution events are logged via `tracing`:

```rust
// Successful resolution
info!(
    agent_id = %agent_id,
    provider = %target.provider_id,
    model = %target.model_id,
    resolution_source = %source, // "explicit", "model_registry", "default"
    "Worker provider resolved"
);

// Fallback (D2)
warn!(
    agent_id = %agent_id,
    primary_provider = %primary,
    fallback_provider = %fallback,
    reason = %reason,
    trust_domain = ?domain,
    "Worker spawn-time fallback"
);

// Budget exceeded (D2)
warn!(
    agent_id = %agent_id,
    spent_usd = spent,
    budget_usd = budget,
    "Worker budget exceeded, stopping"
);
```

### 12.2 QueryEvent Extensions (Deliverable 2)

```rust
pub enum QueryEvent {
    // ... existing variants ...

    /// A worker's provider was resolved (D1 — informational).
    WorkerProviderResolved {
        agent_id: String,
        provider_id: String,
        model_id: String,
    },

    /// A worker exceeded its budget (D2).
    WorkerBudgetExceeded {
        agent_id: String,
        spent_usd: f64,
        budget_usd: f64,
    },
}
```

Fallback events are logged but not emitted as `QueryEvent` — they occur before the worker's event channel is set up.

---

## 13. Rollout Plan

### Deliverable 1: Shared Resolution and Registry Propagation

**Scope:** ~800 LoC (500 production + 300 test)

| # | Task | File | Change |
|---|------|------|--------|
| 1.1 | Extract `resolve_provider()` from `run_query_loop()` | New: `crates/query/src/provider_resolution.rs` | ~150 LoC new (extracted + cleaned up) |
| 1.2 | Replace inline resolution in `run_query_loop()` with call to `resolve_provider()` | `crates/query/src/lib.rs` | ~70 LoC of inline code replaced with ~10 LoC call |
| 1.3 | Replace `AnthropicClient::new()` in `AgentTool::execute()` (foreground path) | `crates/query/src/agent_tool.rs:230-248` | ~30 LoC replaced |
| 1.4 | Replace `AnthropicClient::new()` in `AgentTool::execute()` (background path) | `crates/query/src/agent_tool.rs:541` | ~30 LoC replaced |
| 1.5 | Propagate `provider_registry` and `model_registry` in child `QueryConfig` | `crates/query/src/agent_tool.rs:339-360` | ~5 LoC changed |
| 1.6 | Migrate `AgentRunFn` to struct-based `AgentRunParams` | `crates/tools/src/team_tool.rs` | ~40 LoC new (struct), ~20 LoC changed |
| 1.7 | Update `init_team_swarm_runner()` to use `resolve_provider()` and `AgentRunParams` | `crates/query/src/lib.rs:520-604` | ~40 LoC replaced |
| 1.8 | Add `provider` and `model` to Agent/TeamCreate input schemas | `agent_tool.rs`, `team_tool.rs` | ~20 LoC |
| 1.9 | Add `provider_registry` and `model_registry` to `ToolContext` | `crates/tools/src/lib.rs` | ~10 LoC |
| 1.10 | Unit tests for `resolve_provider()` | New: `crates/query/src/provider_resolution.rs` (tests module) | ~200 LoC |
| 1.11 | Integration test: Agent with explicit provider | `crates/query/tests/` | ~50 LoC |
| 1.12 | Integration test: Agent inherits parent provider | `crates/query/tests/` | ~50 LoC |

**Verification:** `cargo test --workspace` passes. Agent can be spawned with `provider: "openai"`. Agent without `provider` inherits parent's provider. Agent on Anthropic parent behaves identically to before.

### Deliverable 2: Budget, Health, and Spawn-Time Fallback

**Scope:** ~1,200 LoC (800 production + 400 test)

| # | Task | File | Change |
|---|------|------|--------|
| 2.1 | Add `TrustDomain` enum | `crates/api/src/provider_types.rs` | ~20 LoC |
| 2.2 | Add `ModelCapabilities` to `ModelEntry` | `crates/api/src/model_registry.rs` | ~50 LoC |
| 2.3 | Implement `HealthCache` | New: `crates/query/src/health_cache.rs` | ~100 LoC |
| 2.4 | Implement `resolve_provider_with_fallback()` | `crates/query/src/provider_resolution.rs` | ~120 LoC |
| 2.5 | Extend `CostTracker` with `agent_id` and `provider_id` | `crates/core/src/lib.rs` | ~15 LoC |
| 2.6 | Implement `SessionBudget` | New: `crates/query/src/session_budget.rs` | ~80 LoC |
| 2.7 | Wire `SessionBudget` into root session and worker spawn | `crates/cli/src/main.rs`, `agent_tool.rs` | ~30 LoC |
| 2.8 | Add `budget_usd` and `allow_fallback` to Agent/TeamCreate schemas | `agent_tool.rs`, `team_tool.rs` | ~20 LoC |
| 2.9 | Add `Capability` enum for matching | `crates/query/src/provider_resolution.rs` | ~40 LoC |
| 2.10 | Implement capability matching in `resolve_provider_with_fallback()` | `crates/query/src/provider_resolution.rs` | ~80 LoC |
| 2.11 | Add `QueryEvent` variants | `crates/query/src/lib.rs` | ~20 LoC |
| 2.12 | Add `session_budget` to `QueryConfig` and `ToolContext` | `crates/query/src/lib.rs`, `crates/tools/src/lib.rs` | ~10 LoC |
| 2.13 | Budget check in `run_query_loop()` post-turn | `crates/query/src/lib.rs` | ~15 LoC |
| 2.14 | Unit tests: HealthCache, SessionBudget, TrustDomain, fallback | Various | ~250 LoC |
| 2.15 | Integration test: spawn-time fallback | `crates/query/tests/` | ~80 LoC |
| 2.16 | Integration test: budget enforcement | `crates/query/tests/` | ~70 LoC |

**Verification:** Worker with `budget_usd: 0.50` stops at cap. Worker with `allow_fallback: true` and unavailable primary provider falls back within trust domain. Local-pinned worker never falls back to cloud.

### Delivery Order

```
D1 (Shared Resolution)     ██████████████████████  ~2 weeks
  |
  ├── Ship, gather feedback
  |
D2 (Budget + Fallback)        ████████████████████████████  ~3 weeks
```

D1 is shippable independently. It resolves the core bug. D2 adds safety rails and resilience.

---

## 14. Acceptance Criteria

### 14.1 Deliverable 1 Acceptance

| # | Criterion | Type |
|---|----------|------|
| AC1 | `resolve_provider()` produces identical results to the current inline logic for all provider/model combinations | Unit test (compare against captured outputs) |
| AC2 | Agent with `provider: "openai"` routes to OpenAI | Integration test |
| AC3 | Agent with `provider: "ollama"` routes to local Ollama | Integration test (requires Ollama) |
| AC4 | Agent without `provider` or `model` inherits parent session's provider | Unit test |
| AC5 | Agent with `model: "gemini-3-flash-preview"` resolves to Google via ModelRegistry | Unit test |
| AC6 | Agent with conflicting `provider: "openai"` and `model: "claude-opus-4-6"` returns descriptive error | Unit test |
| AC7 | TeamCreate with mixed `provider` per agent dispatches correctly | Integration test |
| AC8 | `ANTHROPIC_API_KEY` absent + `--provider openai` → agent spawns successfully | Integration test |
| AC9 | `ANTHROPIC_API_KEY` absent + no provider override + no registry → graceful error (not panic) | Unit test |
| AC10 | All existing tests pass without modification (except AgentRunFn callers) | `cargo test --workspace` |

### 14.2 Deliverable 2 Acceptance

| # | Criterion | Type |
|---|----------|------|
| AC11 | Worker with `budget_usd: 0.01` stops within two turns | Unit test with mock |
| AC12 | Session budget exceeded → workers receive cancellation | Unit test |
| AC13 | Spawn-time fallback within Cloud trust domain | Unit test with mock health |
| AC14 | Spawn-time fallback from Local provider stays Local | Unit test |
| AC15 | `allow_cross_domain_fallback: false` (default) prevents Local → Cloud | Unit test |
| AC16 | `allow_fallback: false` (default) → no fallback, descriptive error | Unit test |
| AC17 | HealthCache respects TTL and re-probes stale entries | Unit test |
| AC18 | Model-level capability matching selects correct model | Unit test |

### 14.3 Performance

| # | Criterion | Target |
|---|----------|--------|
| PA1 | `resolve_provider()` latency (no health check) | < 1ms |
| PA2 | `resolve_provider_with_fallback()` with cache hit | < 1ms |
| PA3 | `resolve_provider_with_fallback()` with probe | < 6 seconds (5s probe timeout + overhead) |
| PA4 | Memory overhead per worker | < 1KB (excluding conversation) |

---

## 15. Test Strategy

### 15.1 Unit Tests

| Component | Test Focus | Approach |
|-----------|-----------|----------|
| `resolve_provider()` | All 4 priority levels, edge cases (unknown provider, conflicting provider+model, empty model string, model with slashes) | Mock `ProviderRegistry` with 3-4 providers |
| `HealthCache` | TTL expiry, concurrent access, probe-if-stale | Real `DashMap`, mock provider `health_check()` |
| `SessionBudget` | Aggregation, per-worker soft caps, session cap | Real `CostTracker` instances |
| `TrustDomain` | Classification correctness | Exhaustive match on known providers |
| `resolve_provider_with_fallback()` | Fallback within trust domain, cross-domain blocked, all-fail error | Mock health cache + mock providers |

### 15.2 Integration Tests

| Scenario | Setup | Assertion |
|----------|-------|-----------|
| Multi-provider Agent | Registry with Anthropic + mock OpenAI | Agent with `provider: "openai"` hits mock |
| Mixed-provider Team | Registry with 2 providers | Each worker hits its specified provider |
| No-key fallback | Only OpenAI key configured | Agent spawns on OpenAI without ANTHROPIC_API_KEY |
| Budget enforcement | Worker with budget $0.01 | Worker stops after first turn |
| Trust domain isolation | Worker on `ollama`, allow_fallback, ollama down | Fallback only to other local providers, not cloud |

### 15.3 Regression

Run `cargo test --workspace` at each milestone. The `AgentRunFn` signature change is the only expected test breakage, contained to `init_team_swarm_runner` and any mocks.

---

## 16. Open Questions and Tradeoffs

### OQ1: Should `run_query_loop()` drop the `client: &AnthropicClient` parameter?

**Option A:** Keep it (backward compat, Anthropic fast path).
**Option B:** Remove it; all providers go through the registry.

**Recommendation:** Option A for D1. The Anthropic client parameter serves as a fallback when `provider_registry` is `None` (tests, legacy code paths). Option B is a clean-up for a future RFC once all callers are migrated.

### OQ2: Should capability matching (D2) use `ProviderCapabilities` or model-level data?

**Resolved:** Model-level data (Section 7.3). Provider-level capabilities are too coarse — models within one provider differ. `ModelRegistry` is extended with `ModelCapabilities`. When model-level data is absent, fall back to provider-level capabilities.

### OQ3: Should there be a config toggle for the behavioral change?

**Recommendation:** No. The old behavior (agents hardcoded to Anthropic) was a bug, not a feature. Adding a toggle preserves the bug. The change is self-explanatory: agents inherit their parent's provider configuration.

### OQ4: How should `TeamCreateTool` handle partial resolution failures?

If a team has 5 workers and 2 fail to resolve:

**Recommendation:** Spawn the 3 that resolved. Return errors for the 2 that didn't. The coordinator sees partial results and can decide how to proceed. This is consistent with how `join_all` already handles individual worker failures.

### OQ5: Default `required_capabilities` (D2)?

When no `required_capabilities` are specified, should `tool_calling` be implicit?

**Recommendation:** Yes. All Claurst workers need tool calling. A model without tool calling cannot participate in the agentic loop. This is an implicit minimum in D2's capability matcher.

### OQ6: Local trust domain extensibility?

A user running a custom OpenAI-compatible endpoint locally (e.g., vLLM) registers it as an `openai_compat` provider. It defaults to `TrustDomain::Cloud`.

**Recommendation:** Add a `trust_domain` field to the provider configuration in `settings.json`:

```json
{
  "providers": {
    "my-local-vllm": {
      "type": "openai_compat",
      "base_url": "http://localhost:8000/v1",
      "trust_domain": "local"
    }
  }
}
```

This is a D2 concern. For D1, trust domains are not needed because there is no fallback.

---

## 17. Final Recommendation

### Ship Deliverable 1 First

D1 is a **bug fix**, not a feature. Workers should inherit their parent's provider configuration. The fix is small (~800 LoC), low-risk, and immediately useful. It unblocks every user who has tried to use Claurst with a non-Anthropic provider and discovered that agents don't work.

### Ship Deliverable 2 as a Follow-On

D2 adds safety rails (budgets, fallback, health checks) that matter for production use of mixed-provider swarms. It is important but not urgent. Ship it 2-3 weeks after D1, informed by real user feedback from D1.

### Deferred to Future RFCs

| Topic | Why Deferred |
|-------|-------------|
| Access profiles (full/read-only/search-only) | Separate concern from provider dispatch; needs its own security analysis |
| Per-API-call fallback / mid-conversation migration | Requires message format re-serialization; complex and risky |
| Campaign DAG / artifact bus | Different problem domain |
| Background health probing | Unnecessary until spawn-time probe latency becomes a measurable problem |
| TUI enhancements (provider column, enhanced /cost) | Can ship independently, no architectural dependency |

### Risk Assessment

| Category | D1 | D2 |
|----------|----|----|
| Technical risk | **Minimal** — extracting existing code | **Low** — new components, but well-scoped |
| Backward-compat risk | **Low** — behavior change is from "fail" to "succeed" | **Minimal** — new fields are optional |
| Scope risk | **Minimal** — ~800 LoC, well-understood code | **Low** — ~1,200 LoC, some edge cases in fallback |

### Naming Decision

Use **"Multi-Provider Worker Orchestration"** for all internal references. The v1 name "Provider-Aware Worker Fabric" is retired.

---

## Appendix A: v1 Review Findings and Dispositions

| # | Finding | Severity | Disposition in v2 |
|---|---------|----------|-------------------|
| 1 | Fallback can violate user intent and trust boundaries | High | **Resolved.** Added `TrustDomain` classification (Section 7.4). Pinned-by-default semantics (Section 5.2). Fallback never crosses trust domains without opt-in. |
| 2 | Internally inconsistent about fallback scope | High | **Resolved.** v2 specifies spawn-time fallback only. No mid-conversation migration. No per-API-call fallback. No message re-serialization. One model, consistently applied. (Section 6.3, principle #3) |
| 3 | WorkerScheduler duplicates root resolution logic | High | **Resolved.** `WorkerScheduler` removed. Replaced with shared `resolve_provider()` function extracted from `run_query_loop()`. Both root and workers call the same function. (Section 7.1) |
| 4 | Capability matching at wrong abstraction level | High | **Resolved.** D2 extends `ModelRegistry` with per-model `ModelCapabilities`. Provider-level capabilities used as fallback when model-level data absent. (Section 7.3) |
| 5 | Budget enforcement described as hard but mechanism is soft | Medium-high | **Resolved.** v2 explicitly calls it "soft-budget enforcement" throughout. Documented that overshoot of up to one API call is expected. (Section 7.7.2) |
| 6 | TeamCreate deferred to Phase 4 | Medium-high | **Resolved.** Both `AgentTool` and `TeamCreateTool` fixed in Deliverable 1. (Section 8.3, Section 13) |
| 7 | Understates backward-compatibility risk | Medium | **Resolved.** v2 explicitly acknowledges the behavior change: agents on non-Anthropic parents now succeed where they previously failed. Documented as desired fix, not zero-risk. (Section 8.2, Section 11.2) |
| 8 | "Zero new tools / internal plumbing only" overstated | Medium | **Resolved.** v2 includes a "User-Visible Changes Acknowledgment" table (Section 9.6) listing all externally observable changes. |
| 9 | AccessProfile out of scope | Medium | **Resolved.** Deferred to follow-on RFC. Removed from v2 entirely. (Section 4.2, NG2) |
| 10 | resolve_batch() premature | Medium | **Resolved.** `resolve_batch()` removed. Workers resolve individually via `resolve_provider()`. (Section 4.2, NG6) |
| 11 | Missing edge cases | Medium | **Resolved.** Added explicit handling for: conflicting provider+model (F7), context-window mismatch (F8), stale health cache (F9), legacy code path with no registry (F10), probe lifecycle (no background probe — on-demand only), trust domain for local providers (OQ6). (Sections 7.6, 11.1, 16) |

---

*End of RFC v2. This document supersedes RFC-0001-v1 and incorporates all review findings from Codex/GPT-5.4 review (2026-04-08).*
