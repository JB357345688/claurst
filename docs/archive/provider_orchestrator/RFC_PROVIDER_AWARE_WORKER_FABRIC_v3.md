# Architecture RFC: Multi-Provider Worker Orchestration

**Unified Provider Resolution for Root Sessions, Agents, and Teams in Claurst**

| Field | Value |
|-------|-------|
| **RFC ID** | RFC-0001-v3 |
| **Status** | Draft |
| **Author** | Claurst Core Team |
| **Date** | 2026-04-08 |
| **Supersedes** | RFC-0001-v2, RFC-0001-v1 |
| **Codebase Version** | `acae926` (2026-04-07) |
| **Affects** | `claurst-query`, `claurst-api`, `claurst-tools`, `claurst-core` |
| **Review History** | v1 reviewed by Codex/GPT-5.4; v2 reviewed by Codex/GPT-5.4; v3 incorporates all v2 review blockers |

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
18. [Appendix A: v2 Review Blockers and Dispositions](#appendix-a-v2-review-blockers-and-dispositions)

---

## 1. Executive Summary

Claurst's root query loop (`run_query_loop()` in `crates/query/src/lib.rs:660`) supports multi-provider routing across 12+ LLM providers via `ProviderRegistry`. Sub-agents spawned by `AgentTool` and parallel workers launched by `TeamCreateTool` do not — they hardcode `AnthropicClient::new(...)` and fail when `ANTHROPIC_API_KEY` is absent.

This RFC proposes a **two-deliverable** fix:

**Deliverable 1 (D1 — Minimal Fix, ~900 LoC):** Extract the existing provider-resolution logic from `run_query_loop()` into a two-phase resolution pipeline: `resolve_provider_identity()` (pure logic: inputs → provider ID + model ID) and `materialize_provider()` (side-effectful: auth-store refresh, api_base overrides → `Arc<dyn LlmProvider>`). Propagate `ProviderRegistry` and `ModelRegistry` into child `QueryConfig` for both `AgentTool` and `TeamCreateTool`. Includes a normative precedence matrix for all provider/model combinations and strict error semantics (no silent Anthropic fallback when a registry is present).

**Deliverable 2 (D2 — Expansion, ~1,400 LoC):** Add soft-budget enforcement via `SessionBudget`, spawn-time health checks via `HealthCache`, spawn-time fallback within trust domains, capability matching with explicit unknown-data policy, and cancellation propagation with defined token ownership.

v2 of this RFC was reviewed and produced 7 blocking findings. This v3 addresses all 7: normative precedence matrix (B1), strict fallback semantics (B2), two-phase resolution contract (B3), explicit `allow_cross_domain_fallback` placement (B4), deferred custom trust-domain support (B5), capability unknown-data policy (B6), and cancellation propagation specification (B7).

---

## 2. Problem Statement

### 2.1 The Break in Provider Symmetry

The root session resolves its provider through a priority chain (lines 854-926 of `crates/query/src/lib.rs`). The actual code-level priority is:

1. `config.provider` setting — if set to a non-`"anthropic"` value, it wins over all other sources
2. `provider/model` format in the model string — if the prefix is a known provider ID
3. `ModelRegistry` lookup — model name → provider mapping
4. Default to `"anthropic"`

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
| `ModelRegistry` | `crates/api/src/model_registry.rs` | Dynamic model→provider resolution via models.dev; already has `tool_calling`, `reasoning`, `vision` per-model flags |
| `ProviderCapabilities` | `crates/api/src/provider_types.rs` | 9 capability flags (provider-level) |
| `ProviderStatus` | `crates/api/src/provider_types.rs` | Health enum: Healthy / Degraded / Unavailable |
| `LlmProvider` trait | `crates/api/src/provider.rs` | `create_message_stream()`, `health_check()`, `capabilities()` |
| `CostTracker` | `crates/core/src/lib.rs:2741` | Atomic token/cost accumulation, model-aware pricing |
| Root provider dispatch | `crates/query/src/lib.rs:854-926` | Full priority chain (see Section 2.1) |
| Root `fallback_model` | `crates/query/src/lib.rs:1299-1319` | Per-turn model fallback on overloaded/rate-limit errors |
| `AgentTool` worktree isolation | `crates/query/src/agent_tool.rs` | Git worktree creation, background polling |
| `TeamCreateTool` cancellation | `crates/tools/src/team_tool.rs` | Per-agent CancellationToken via DashMap |
| `run_query_loop()` signature | `crates/query/src/lib.rs:660` | Takes `client: &AnthropicClient` as first param |
| `ModelEntry` per-model metadata | `crates/api/src/model_registry.rs:22-44` | `tool_calling`, `reasoning`, `vision`, pricing, family, status |

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
| G1 | **Shared provider resolution.** Extract the root's resolution logic into reusable functions. Both root and workers use the same code path. | D1 |
| G2 | **Registry propagation.** Workers inherit `ProviderRegistry` and `ModelRegistry` from the parent. | D1 |
| G3 | **Explicit provider/model selection.** Agent and TeamCreate callers can specify `provider` and `model` per worker. | D1 |
| G4 | **Fix both AgentTool and TeamCreateTool.** Both broken paths fixed in the same deliverable. | D1 |
| G5 | **Backward compatibility with explicit risk acknowledgment.** Workers on a non-Anthropic parent session now inherit that parent's provider — this is the desired fix, but it is a behavior change. | D1 |
| G6 | **Normative precedence matrix.** A single authoritative table defines provider/model resolution for every input combination. No ambiguity. | D1 |
| G7 | **Strict error semantics.** When `ProviderRegistry` is present and resolution fails, the call fails loudly. No silent Anthropic fallback. | D1 |
| G8 | **Soft-budget enforcement.** Per-worker and session-level USD caps, best-effort (not hard guarantees). | D2 |
| G9 | **Spawn-time health check.** Before dispatching a worker to a provider, verify the provider is reachable. | D2 |
| G10 | **Spawn-time fallback.** If the requested provider is unreachable at spawn time, fall back to an alternative within the same trust domain. | D2 |
| G11 | **Cancellation propagation.** Defined token ownership and stop-point semantics for budget-driven cancellation. | D2 |

### 4.2 Non-Goals

| # | Non-Goal | Rationale |
|---|----------|-----------|
| NG1 | **Per-API-call fallback / mid-conversation provider migration.** Requires message re-serialization across format boundaries. Deferred. | v1 finding #2 |
| NG2 | **Access profiles (full/read-only/search-only).** Separate concern. Deferred. | v1 finding #9 |
| NG3 | **Campaign DAG / artifact bus.** Separate problem domain. | Scope |
| NG4 | **Runtime orchestration (Docker/SSH).** | Scope |
| NG5 | **Model auto-selection via ML.** Capability matching is rule-based. | Complexity |
| NG6 | **Batch resolution policies.** Workers resolve individually. | v1 finding #10 |
| NG7 | **TUI enhancements.** Can ship independently. | Scope |
| NG8 | **Custom provider trust-domain override.** Requires extending `ProviderConfig` and provider registration. Explicitly deferred (see Appendix A, B5). | v2 blocker #5 |

---

## 5. Terminology and Naming Clarification

### 5.1 Feature Name

**Formal name:** **Multi-Provider Worker Orchestration** (abbreviated **MPWO**).

Code modules and feature-flag references use `mpwo` or `worker_orchestration`.

### 5.2 Glossary

| Term | Definition |
|------|-----------|
| **Root session** | The top-level `run_query_loop()` instance driven by the TUI or `--print` mode. Owns the `ProviderRegistry`. |
| **Worker** | Any `run_query_loop()` instance spawned by `AgentTool` or `TeamCreateTool`. |
| **Provider** | An `LlmProvider` implementation in the `ProviderRegistry`. Identified by `ProviderId`. |
| **Model** | A specific model ID within a provider. Resolved via `ModelRegistry`. |
| **Execution target** | A resolved `(ProviderId, model_id, Arc<dyn LlmProvider>)` tuple ready for dispatch. |
| **Trust domain** | A classification of providers by data-handling boundary: `Local` (Ollama, LM Studio, llama.cpp — data never leaves the machine), `Cloud` (all others). Fallback never crosses trust-domain boundaries unless explicitly opted in. |
| **Pinned provider** | An explicit `provider` in the worker request. The system must use this provider or fail — no fallback. Default behavior. |
| **Preferred provider** | A `provider` with `allow_fallback: true`. The system tries this provider first but may fall back to another in the same trust domain. |

### 5.3 Trust Domains

Providers are classified by a hardcoded list of known local provider IDs:

```rust
pub enum TrustDomain {
    /// Data stays on the local machine. Providers: ollama, lmstudio, llamacpp.
    Local,
    /// Data is sent to a cloud endpoint.
    Cloud,
}
```

**D2 only supports built-in provider IDs for trust-domain classification.** Custom provider trust-domain override (e.g., a user running vLLM locally with an `openai_compat` adapter) is explicitly deferred. Such providers default to `TrustDomain::Cloud`. A follow-on RFC will extend `ProviderConfig` with a `trust_domain` field and define the custom provider registration contract. *(Addresses v2 blocker #5.)*

**Rule:** Fallback never crosses trust domain boundaries. `Local → Local` and `Cloud → Cloud` are permitted. `Local → Cloud` is forbidden.

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
 │   // Two-phase resolution:                            │
 │   let identity = resolve_provider_identity(           │
 │     &effective_model, config_provider, registry,      │
 │     model_registry                                    │
 │   );                                                  │
 │   let target = materialize_provider(                  │
 │     &identity, registry, &provider_configs            │
 │   );                                                  │
 │ }                                                     │
 └───────┬─────────────────────┬─────────────────────────┘
         │                     │
         │ AgentTool           │ TeamCreateTool
         ▼                     ▼
 ┌──────────────────────────────────────────────────────┐
 │     Same two-phase resolution pipeline               │
 │  resolve_provider_identity() → materialize_provider()│
 │  Input:  model string, provider override,            │
 │          ProviderRegistry, ModelRegistry,             │
 │          provider_configs                             │
 │  Output: (ProviderId, model_id, Arc<dyn LlmProvider>)│
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
 resolve_provider_identity()  │ health check before dispatch
 + materialize_provider()     │
        │                     │
        ▼                     ▼
 ┌─────────────────────────────────────────────────┐
 │ Spawn-time fallback (D2 only):                  │
 │   if target.health == Unavailable:              │
 │     find next eligible in same trust domain     │
 │     or fail with descriptive error              │
 │                                                 │
 │ Budget enforcement (soft):                      │
 │   per-worker CostTracker with optional USD cap  │
 │   SessionBudget aggregates all workers          │
 │                                                 │
 │ Cancellation propagation:                       │
 │   SessionBudget owns root cancel token          │
 │   Workers hold child tokens linked to root      │
 └─────────────────────────────────────────────────┘
```

### 6.3 Key Design Principles

1. **Extract, don't duplicate.** The root's provider-resolution logic (lines 854-926 of `lib.rs`) is extracted into shared functions. Both root and workers call them. One resolution path, one set of precedence rules, one set of error messages.

2. **Two-phase resolution.** `resolve_provider_identity()` is pure logic (no side effects, no network). `materialize_provider()` handles auth-store refresh, api_base overrides, and provider construction. This separation preserves current root behavior including runtime credential refresh after `/connect` and local-provider `api_base` override handling. *(Addresses v2 blocker #3.)*

3. **Normative precedence matrix.** A single authoritative table (Section 7.1.1) defines resolution outcomes for every input combination. No ambiguity between the problem statement, the function contract, and the acceptance criteria. *(Addresses v2 blocker #1.)*

4. **Fail-closed when registry is present.** If `provider_registry` is `Some(...)` and resolution fails, the call fails with a descriptive error. No silent Anthropic fallback. The `client: &AnthropicClient` parameter is only used when `provider_registry` is `None`. *(Addresses v2 blocker #2.)*

5. **Pinned by default.** An explicit `provider` means "use this provider or fail." Fallback is opt-in via `allow_fallback: true`.

6. **No mid-conversation migration.** Fallback is evaluated at spawn time only. A worker that starts on provider X stays on provider X.

7. **Trust domains are inviolable.** Fallback from a local provider to a cloud provider never happens. (No `allow_cross_domain_fallback` in D2 — see Section 7.5.)

8. **Budget enforcement is soft.** `CostTracker` is post-hoc accounting. Under concurrency, a worker can overshoot its cap by one API call.

9. **Capability matching uses explicit unknown-data policy.** When model-level metadata is absent, field-by-field fallback rules apply. *(Addresses v2 blocker #6.)*

---

## 7. Component Design

### 7.1 Two-Phase Provider Resolution (Deliverable 1)

**Location:** `crates/query/src/provider_resolution.rs` (new file, extracted from `lib.rs:854-926`)

This is the core of Deliverable 1. The existing inline resolution logic is split into two functions.

#### Phase 1: `resolve_provider_identity()` — Pure Logic

```rust
/// The resolved identity of a provider+model pair (no side effects).
pub struct ProviderIdentity {
    pub provider_id: ProviderId,
    pub model_id: String,
    /// How the provider was resolved — for logging/diagnostics.
    pub resolution_source: ResolutionSource,
}

#[derive(Debug, Clone, Copy)]
pub enum ResolutionSource {
    /// Explicit `provider` parameter or config.provider
    ExplicitProvider,
    /// "provider/model" prefix in the model string
    ModelStringPrefix,
    /// ModelRegistry lookup (model name → provider)
    ModelRegistry,
    /// Default (no provider could be determined, using registry default)
    Default,
}

/// Resolve a provider identity from inputs. Pure logic — no network, no auth store.
///
/// See Section 7.1.1 for the normative precedence matrix.
pub fn resolve_provider_identity(
    model: &str,
    provider_override: Option<&str>,
    registry: &ProviderRegistry,
    model_registry: Option<&ModelRegistry>,
) -> Result<ProviderIdentity, ProviderResolutionError>;
```

#### Phase 2: `materialize_provider()` — Side-Effectful Construction

```rust
/// A fully materialized execution target, ready for API calls.
pub struct ExecutionTarget {
    pub provider_id: ProviderId,
    pub model_id: String,
    pub provider: Arc<dyn LlmProvider>,
    pub resolution_source: ResolutionSource,
}

/// Materialize a provider instance from a resolved identity.
///
/// This function:
/// 1. Checks the runtime auth-store for fresh credentials (supports `/connect` updates)
/// 2. Falls back to the registry's pre-built provider if no runtime provider available
/// 3. Applies `api_base` overrides from `provider_configs` for local providers
///    (ollama, lmstudio, llamacpp)
///
/// This preserves the current root behavior at lib.rs:937-977.
pub fn materialize_provider(
    identity: &ProviderIdentity,
    registry: &ProviderRegistry,
    provider_configs: &HashMap<String, ProviderConfig>,
) -> Result<ExecutionTarget, ProviderResolutionError>;
```

#### Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum ProviderResolutionError {
    #[error("Provider '{0}' not found in registry. Available: {1}")]
    ProviderNotFound(String, String),

    #[error("No provider could be resolved for model '{0}'")]
    NoProviderForModel(String),

    #[error("Provider '{0}' has no API key configured. Run /connect or set the appropriate environment variable.")]
    NoCredentials(String),

    #[error("Provider/model conflict: model '{model}' belongs to provider '{model_owner}', but provider '{requested}' was explicitly requested")]
    ProviderModelConflict {
        model: String,
        model_owner: String,
        requested: String,
    },

    #[error("Provider '{0}' is unavailable: {1}")]
    ProviderUnavailable(String, String),
}
```

#### 7.1.1 Normative Precedence Matrix (Addresses v2 Blocker #1)

This table is the **single authoritative specification** for `resolve_provider_identity()`. Any conflict between this table and other sections of the RFC is resolved in favor of this table.

The term `config_provider` refers to the `provider_override` parameter, which is sourced from `config.provider` (root), `params.provider` (Agent/TeamCreate), or the `--provider` CLI flag.

The term `model_string` refers to the `model` parameter passed to the function.

"Known prefix" means the `provider/` prefix in a model string matches one of the hardcoded known provider IDs (same list as current code at `lib.rs:879-886`).

| # | `config_provider` | `model_string` | Outcome | Source |
|---|-------------------|----------------|---------|--------|
| P1 | `Some("openai")` | `"gpt-4.1"` (bare) | `(openai, "gpt-4.1")` | `ExplicitProvider` |
| P2 | `Some("openai")` | `"openai/gpt-4.1"` (matching prefix) | `(openai, "gpt-4.1")` — prefix stripped | `ExplicitProvider` |
| P3 | `Some("openai")` | `"anthropic/claude-opus-4-6"` (conflicting prefix) | **Error: `ProviderModelConflict`** | — |
| P4 | `Some("openai")` | `"meta-llama/Llama-3.3"` (non-provider namespace) | `(openai, "meta-llama/Llama-3.3")` | `ExplicitProvider` |
| P5 | `Some("anthropic")` or `None` | `"openai/gpt-4.1"` (known prefix) | `(openai, "gpt-4.1")` | `ModelStringPrefix` |
| P6 | `Some("anthropic")` or `None` | `"meta-llama/Llama-3.3"` (unknown prefix) | Fall through to ModelRegistry; if no hit, `(anthropic, "meta-llama/Llama-3.3")` | `ModelRegistry` or `Default` |
| P7 | `Some("anthropic")` or `None` | `"gemini-3-flash-preview"` (bare, in registry) | `(google, "gemini-3-flash-preview")` | `ModelRegistry` |
| P8 | `Some("anthropic")` or `None` | `"claude-opus-4-6"` (bare, in registry as anthropic) | `(anthropic, "claude-opus-4-6")` | `ModelRegistry` or `Default` |
| P9 | `Some("anthropic")` or `None` | `"some-unknown-model"` (bare, not in registry) | `(anthropic, "some-unknown-model")` | `Default` |
| P10 | `None` | `""` (empty) | **Error: `NoProviderForModel`** | — |

**Conflict detection rule (P3):** When `config_provider` is set to a non-`"anthropic"` value and the model string has a known provider prefix that differs from `config_provider`, this is an unambiguous conflict. The function returns `ProviderModelConflict`. This is a **new behavior** not present in the current inline code, which would silently use `config_provider` and strip a mismatched prefix.

**`config_provider = "anthropic"` treated as `None`:** When `config_provider` is explicitly `"anthropic"`, it is treated the same as `None` for precedence purposes. This matches the current code behavior at `lib.rs:863` where the `filter(|p| *p != "anthropic")` guard causes the explicit-provider branch to be skipped for Anthropic.

### 7.2 ProviderRegistry (Existing — No Changes)

**Location:** `crates/api/src/registry.rs`

No modifications needed. The registry is a passive lookup table. The resolution functions compose with it.

### 7.3 ModelRegistry (Existing — Extended in D2)

**Location:** `crates/api/src/model_registry.rs`

**Current state (important for scope estimates):** `ModelEntry` already contains per-model `tool_calling: bool`, `reasoning: bool`, `vision: bool`, pricing fields, and `family`/`status` metadata. The models.dev parse fills some of these fields and defaults `vision` to `false`. There is **no** current data for `pdf_input`, `audio_input`, `video_input`, or `structured_output`.

**Deliverable 1:** No changes. Model-to-provider resolution already works.

**Deliverable 2:** Extend `ModelEntry` with additional capability fields:

```rust
pub struct ModelEntry {
    // ... existing fields (info, cost_*, tool_calling, reasoning, vision, family, status) ...

    // D2 additions — all Optional to support partial metadata:
    pub pdf_input: Option<bool>,       // NEW — None means unknown
    pub audio_input: Option<bool>,     // NEW
    pub structured_output: Option<bool>, // NEW
    pub max_output_tokens: Option<u32>,  // NEW
}
```

**Note:** `tool_calling`, `reasoning`, and `vision` remain non-Optional `bool` fields because they already exist and are populated today. The new fields use `Option<bool>` because data is not yet available from models.dev for all models.

**Data source:** The models.dev API already returns per-model metadata. `ModelRegistry::load_cache()` parses this. The extension adds structured capability extraction for the new fields from the existing data.

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

**D2 only supports built-in provider IDs.** Custom providers (e.g., vLLM behind an `openai_compat` adapter) default to `TrustDomain::Cloud`. The current `ProviderConfig` struct (in `crates/core/src/lib.rs:669-686`) has fields `api_key`, `api_base`, `enabled`, whitelists, and `options` — but no `type` or `trust_domain` field. Adding custom trust-domain support requires extending `ProviderConfig`, updating provider registration in `provider_from_key()`, and defining the full contract. That work is deferred to a follow-on RFC. *(Addresses v2 blocker #5.)*

### 7.5 Spawn-Time Fallback (Deliverable 2)

**Location:** `crates/query/src/provider_resolution.rs` (extend the D1 file)

Fallback is evaluated **once, at spawn time**, not per API call.

```rust
/// Resolve with fallback. Tries the primary target first; on health-check
/// failure, tries alternatives within the same trust domain.
///
/// Returns the first healthy target, or an error listing all attempted providers.
pub fn resolve_provider_with_fallback(
    model: &str,
    provider_override: Option<&str>,
    allow_fallback: bool,
    registry: &ProviderRegistry,
    model_registry: Option<&ModelRegistry>,
    provider_configs: &HashMap<String, ProviderConfig>,
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

**Cross-domain fallback is removed from D2.** v2 proposed an `allow_cross_domain_fallback` flag but never placed it in any schema or config surface (v2 blocker #4). Rather than under-specify it, v3 forbids cross-domain fallback entirely. `Local → Cloud` never happens. If a future use case requires it, a follow-on RFC will define the control surface properly. *(Addresses v2 blocker #4.)*

**Fallback model selection within a target provider:** When falling back, `resolve_provider_with_fallback()` selects a model as follows:
1. If the original model has a known equivalent on the fallback provider (via `ModelRegistry` family matching), use it.
2. Otherwise, use the fallback provider's default model (if the provider has one in the `ModelRegistry`).
3. If neither is available, skip this fallback candidate and try the next provider.

**Degraded provider ordering:** When evaluating fallback candidates, providers with `ProviderStatus::Healthy` are preferred over `ProviderStatus::Degraded`. Providers with `ProviderStatus::Unavailable` are skipped. If only `Degraded` providers are available, they are used (with a warning log).

**Interaction with existing `fallback_model`:** The existing `config.fallback_model` path in `run_query_loop()` (lib.rs:1299-1319) handles per-turn model fallback on overloaded/rate-limit errors *within the same provider*. This is orthogonal to spawn-time provider fallback and remains unchanged. "Spawn-time only" refers to provider-level fallback; model-level fallback within a session is existing behavior.

### 7.6 Capability Matching — Unknown-Data Policy (Deliverable 2)

*(Addresses v2 blocker #6.)*

When `resolve_provider_with_fallback()` checks whether a model satisfies `required_capabilities`, it uses the following per-field policy:

| Capability | `ModelEntry` field | Unknown-data rule |
|------------|-------------------|-------------------|
| `ToolCalling` | `tool_calling: bool` | Always known (existing field, defaults to `false`). `false` = ineligible. |
| `Reasoning` | `reasoning: bool` | Always known (existing field, defaults to `false`). `false` = ineligible. |
| `Vision` | `vision: bool` | Always known (existing field, defaults to `false`). `false` = ineligible. |
| `PdfInput` | `pdf_input: Option<bool>` | `None` → fall back to `ProviderCapabilities::pdf_input`. If provider-level is also unknown, treat as **ineligible**. |
| `AudioInput` | `audio_input: Option<bool>` | `None` → fall back to `ProviderCapabilities::audio_input`. If provider-level is also unknown, treat as **ineligible**. |
| `StructuredOutput` | `structured_output: Option<bool>` | `None` → fall back to `ProviderCapabilities::structured_output`. If provider-level is also unknown, treat as **ineligible**. |

**Rationale for "unknown = ineligible":** It is safer to reject a potentially-capable model than to route work to an incapable one. A user can always explicitly specify a provider+model to bypass capability matching.

**Default required capabilities:** When no `required_capabilities` are specified, `[ToolCalling]` is implicit. All Claurst workers need tool calling to participate in the agentic loop.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Capability {
    ToolCalling,
    Reasoning,
    Vision,
    PdfInput,
    AudioInput,
    StructuredOutput,
}
```

### 7.7 HealthCache (Deliverable 2)

**Location:** `crates/query/src/health_cache.rs` (new file)

```rust
pub struct HealthCache {
    statuses: DashMap<ProviderId, (ProviderStatus, Instant)>,
    ttl: Duration,
}

impl HealthCache {
    pub fn new(ttl: Duration) -> Self;
    pub fn get(&self, id: &ProviderId) -> Option<ProviderStatus>;
    pub fn update(&self, id: ProviderId, status: ProviderStatus);
    pub async fn probe_if_stale(
        &self,
        id: &ProviderId,
        provider: &dyn LlmProvider,
    ) -> ProviderStatus;
}
```

**No background probe task.** `resolve_provider_with_fallback()` probes on demand, using cached results when fresh.

### 7.8 Per-Worker CostTracker and SessionBudget (Deliverable 2)

**Location:** Extend `CostTracker` in `crates/core/src/lib.rs`; new `crates/query/src/session_budget.rs`

#### 7.8.1 CostTracker Extension

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

#### 7.8.2 SessionBudget

```rust
pub struct SessionBudget {
    max_budget_usd: Option<f64>,
    workers: DashMap<String, Arc<CostTracker>>,
    root_tracker: Arc<CostTracker>,
    /// Root cancellation token. When the session budget is exceeded,
    /// this token is cancelled, which cascades to all child tokens.
    cancel_token: CancellationToken,
}

impl SessionBudget {
    pub fn register_worker(&self, agent_id: String, tracker: Arc<CostTracker>);
    pub fn total_cost_usd(&self) -> f64;
    pub fn worker_costs(&self) -> Vec<(String, Option<String>, f64)>;
    pub fn is_exceeded(&self) -> bool;
    /// Check budget and fire cancellation if exceeded.
    /// Called by run_query_loop() after each completed API turn.
    pub fn check_and_cancel(&self);
    /// Create a child cancellation token linked to this budget's root token.
    pub fn child_cancel_token(&self) -> CancellationToken;
}
```

**Budget enforcement is explicitly soft.** Under concurrency, multiple workers can exceed their caps simultaneously. A single oversized API response can overshoot a cap. The `SessionBudget` checks are evaluated after each completed API turn, not before each API call. A worker can exceed its budget by at most the cost of one API round-trip.

### 7.9 Cancellation Propagation (Deliverable 2)

*(Addresses v2 blocker #7.)*

#### 7.9.1 Token Ownership Model

```
 SessionBudget
 └── root_cancel_token  (owned by SessionBudget)
      │
      ├── AgentTool worker cancel token  (child of root, via child_cancel_token())
      │    └── run_query_loop() cancel param
      │
      ├── AgentTool background worker cancel token  (child of root)
      │    └── run_query_loop() cancel param
      │
      └── TeamCreate per-agent cancel tokens  (children of root)
           ├── agent[0] cancel token  (also in ACTIVE_TEAMS for TeamDelete)
           ├── agent[1] cancel token
           └── ...
```

#### 7.9.2 Token Lifecycle

1. **Creation:** When `SessionBudget` is constructed (root session startup), it creates a root `CancellationToken`.
2. **Child token creation:** Each worker spawn calls `session_budget.child_cancel_token()`, which returns a new token that is cancelled when the root token is cancelled. This uses `CancellationToken::child_token()` from `tokio-util`.
3. **TeamCreate integration:** The existing `TeamCreateTool` creates per-agent cancel tokens for `TeamDelete` support. In D2, these per-agent tokens are children of the session budget's root token. TeamDelete cancels the per-agent token directly; budget exhaustion cancels the root token, which cascades to all children.
4. **Budget check:** `run_query_loop()` calls `session_budget.check_and_cancel()` after each completed API turn (after cost tracking at lib.rs:1390). If the session budget is exceeded, the root token is cancelled.

#### 7.9.3 Stop Points

When a cancellation token fires, the worker stops at the **next check point**. The check points are:

| Check point | Location | Behavior |
|-------------|----------|----------|
| Before next API call | `run_query_loop()` top of turn loop | If cancelled, return `QueryOutcome::Cancelled` |
| During stream | `run_query_loop()` stream accumulation | `tokio::select!` on cancel token vs stream. If cancelled, return partial result. |
| Before tool execution | `run_query_loop()` tool dispatch | If cancelled, skip remaining tool calls and return. |
| TeamCreate `tokio::select!` | `team_tool.rs:415-424` | Existing: `cancel.cancelled()` branch returns `"[Agent cancelled]"` |

**Cancellation does NOT interrupt a tool execution in progress.** If a Bash command is running, it completes. The cancellation is checked before the *next* tool call or API call.

#### 7.9.4 Nested Workers

If a worker spawns its own sub-agents (nested agent calls), the sub-agent's cancel token is a child of the spawning worker's token. This creates a cascading tree: root budget cancellation → all workers → all sub-workers. No explicit depth tracking is needed; the `child_token()` mechanism handles it automatically.

---

## 8. Execution Model

### 8.1 Root Session (Deliverable 1 Change)

The inline resolution logic is replaced with calls to the two-phase pipeline:

**Before (`lib.rs:854-926`, inline):**
```rust
if let Some(ref registry) = config.provider_registry {
    let (provider_id_str, model_id_str) = if let Some(p) = ... {
        // ... 70 lines of resolution logic
    };
    // ... 50 lines of materialization (auth refresh, api_base overrides)
    // dispatch
}
```

**After:**
```rust
if let Some(ref registry) = config.provider_registry {
    match resolve_provider_identity(
        &effective_model,
        tool_ctx.config.provider.as_deref(),
        registry,
        config.model_registry.as_deref(),
    ) {
        Ok(identity) => {
            match materialize_provider(
                &identity,
                registry,
                &tool_ctx.config.provider_configs,
            ) {
                Ok(target) => {
                    // dispatch using target.provider
                }
                Err(e) => {
                    // Resolution error — fail the API call.
                    // NO silent Anthropic fallback.
                    error!(%e, "Provider materialization failed");
                    return QueryOutcome::Error(e.into());
                }
            }
        }
        Err(e) => {
            // Resolution error — fail the API call.
            // NO silent Anthropic fallback.
            error!(%e, "Provider resolution failed");
            return QueryOutcome::Error(e.into());
        }
    }
} else {
    // No registry — use the legacy AnthropicClient parameter.
    // This is the ONLY path where the AnthropicClient param is used.
}
```

**Key change from v2:** When `provider_registry` is `Some(...)` and resolution/materialization fails, the call **fails with an error**. There is no `Err(e) => { /* fall back to client param */ }` branch. The `client: &AnthropicClient` parameter is **only** used in the `else` branch (no registry). *(Addresses v2 blocker #2.)*

### 8.2 AgentTool (Deliverable 1 Change)

**Before (`agent_tool.rs:230-248`):**
```rust
let api_key = std::env::var("ANTHROPIC_API_KEY")...;
let client = AnthropicClient::new(ClientConfig { api_key, .. });
let query_config = QueryConfig {
    provider_registry: None,
    model_registry: None,
    // ...
};
```

**After:**
```rust
// Get the parent's registries — REQUIRED.
let registry = ctx.provider_registry
    .as_ref()
    .ok_or_else(|| ToolError::new(
        "No ProviderRegistry available. This is an internal error — \
         the parent session must propagate its registry to workers."
    ))?;

// Resolve provider: explicit params > parent's config > default.
let identity = resolve_provider_identity(
    &resolved_model,
    params.provider.as_deref(),
    registry,
    ctx.model_registry.as_deref(),
)?;

let target = materialize_provider(
    &identity,
    registry,
    &ctx.config.provider_configs,
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

// Use target.provider for the API client.
```

**Worker paths NEVER silently fall back to Anthropic when registry is missing.** Missing registry in a worker is an invariant violation — it means the parent session failed to propagate its registry, which is a bug. *(Addresses v2 blocker #2.)*

The same change applies to the background-agent path (`agent_tool.rs:541`).

**Backward compatibility:** When `params.provider` is `None` and `params.model` is `None`, `resolve_provider_identity()` follows the same priority chain as the root, which defaults to the parent's configured provider. If the parent is on Anthropic, the worker is on Anthropic. If the parent is on OpenAI, the worker is on OpenAI. **This is a behavior change** for the case where a user is on a non-Anthropic provider: previously, agents would fail (ANTHROPIC_API_KEY not set); now they succeed on the parent's provider.

### 8.3 TeamCreateTool (Deliverable 1 Change)

The `AgentRunFn` callback is updated (see Section 9.3). The team runner in `init_team_swarm_runner()` (`lib.rs:520-604`) replaces its `AnthropicClient::new()` with the same two-phase resolution pipeline.

**Both AgentTool and TeamCreateTool are fixed in Deliverable 1.**

### 8.4 Parallel Workers

No change from current behavior. Workers are isolated at the conversation level, share the filesystem (unless worktree-isolated), and share the `ProviderRegistry` (read-only).

### 8.5 Spawn-Time Fallback (Deliverable 2)

When a worker's primary provider is unreachable at spawn time:

```
Worker A requested: anthropic/claude-opus-4-6, allow_fallback: true
  │
  ├── health_cache.probe_if_stale("anthropic") → Unavailable
  │
  ├── Find alternatives in TrustDomain::Cloud:
  │   ├── openai (Healthy, supports tool_calling) → selected
  │   │   └── Model selection: family match "claude" → no equivalent;
  │   │       use openai default model from ModelRegistry
  │
  ├── Log: "Worker A: anthropic unavailable, falling back to openai/gpt-4.1"
  │
  └── Worker A starts on openai/gpt-4.1
      └── Worker stays on openai for entire session
```

**If `allow_fallback: false` (default):**
```
Worker A requested: anthropic/claude-opus-4-6
  │
  ├── health_cache.probe_if_stale("anthropic") → Unavailable
  │
  └── Error: "Provider 'anthropic' is unavailable: connection refused.
             Set allow_fallback: true to enable automatic fallback."
```

**If the provider is local:**
```
Worker B requested: ollama/llama-3.3, allow_fallback: true
  │
  ├── health_cache.probe_if_stale("ollama") → Unavailable
  │
  ├── Find alternatives in TrustDomain::Local:
  │   ├── lmstudio (Healthy, supports tool_calling) → selected
  │
  └── Worker B starts on lmstudio
      (Never falls back to a Cloud provider — cross-domain forbidden)
```

---

## 9. Proposed Schema and Interface Changes

### 9.1 Agent Tool Input Schema (Extended)

New optional fields (D1):

```json
{
  "provider": {
    "type": "string",
    "description": "Provider to use (e.g., anthropic, openai, ollama). Pinned by default."
  }
}
```

Additional D2 fields:

```json
{
  "allow_fallback": {
    "type": "boolean",
    "default": false,
    "description": "If true, allow fallback to another provider in the same trust domain on failure."
  },
  "budget_usd": {
    "type": "number",
    "description": "Soft USD spending cap for this worker."
  }
}
```

All new fields are optional. Omitting them produces identical behavior to today (after the fix).

**Note:** `model` already exists on `AgentInput` today (`agent_tool.rs:144-146`). It is not a new field.

### 9.2 TeamCreate Agent Entry Schema (Extended)

New optional fields per agent:

```json
{
  "provider": { "type": "string" },
  "model": { "type": "string" },
  "allow_fallback": { "type": "boolean", "default": false },
  "budget_usd": { "type": "number" }
}
```

**Note:** `model` is new for TeamCreate agent entries (it did not exist before).

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

The runner receives the raw parameters and calls `resolve_provider_identity()` + `materialize_provider()` itself.

**Migration:** Two call sites change: `init_team_swarm_runner()` (producer) and `run_agent()` in `team_tool.rs` (consumer). Both are internal.

### 9.4 QueryConfig Extensions

```rust
pub struct QueryConfig {
    // ... existing fields unchanged ...

    // D2 only:
    pub session_budget: Option<Arc<SessionBudget>>,
}
```

### 9.5 ToolContext Extensions

```rust
pub struct ToolContext {
    // ... existing fields unchanged ...

    // D1: parent's registries, for workers that need to resolve providers
    pub provider_registry: Option<Arc<ProviderRegistry>>,
    pub model_registry: Option<Arc<ModelRegistry>>,

    // D2:
    pub session_budget: Option<Arc<SessionBudget>>,
    pub health_cache: Option<Arc<HealthCache>>,
}
```

**Note:** v2 used `ctx.worker_scheduler_registry` in the AgentTool snippet but `provider_registry` in the ToolContext definition. v3 uses `provider_registry` consistently in both places.

### 9.6 User-Visible Changes Acknowledgment

| Change | Visibility | Deliverable |
|--------|-----------|-------------|
| New `provider` field on Agent/TeamCreate | Visible in tool schemas; model may use it | D1 |
| `model` field added to TeamCreate agent entries | Visible in tool schema | D1 |
| Workers inherit parent provider instead of failing | Behavior change (desired) | D1 |
| Error messages change from "ANTHROPIC_API_KEY not set" to descriptive resolution errors | Behavior change (improved) | D1 |
| `provider/model` conflict now returns explicit error (P3) | New error path | D1 |
| New `allow_fallback`, `budget_usd` fields on Agent/TeamCreate | Visible in tool schemas | D2 |
| `/cost` may show per-worker breakdown | UI enhancement | D2 |

---

## 10. Permission, Isolation, and Security Considerations

### 10.1 API Key Isolation

Workers inherit provider references (`Arc<dyn LlmProvider>`) from the parent's `ProviderRegistry`. Raw API keys are encapsulated inside provider implementations and not extractable via the `LlmProvider` trait.

### 10.2 Trust Domain Enforcement

The `TrustDomain` classification (Section 7.4) prevents accidental data leakage:

- A worker pinned to `ollama` will never fall back to a cloud provider.
- Cross-domain fallback is forbidden in D2. No escape hatch.
- Trust domain classification is determined by hardcoded provider ID list. Custom providers default to `Cloud`.

### 10.3 Worktree Isolation

Unchanged from current behavior. Orthogonal to provider selection.

### 10.4 Budget as a Safety Boundary

Per-worker soft budgets prevent runaway cost. A worker exceeding its cap is stopped after the current turn. The session budget provides a secondary cap. Neither is a hard guarantee (see Section 7.8.2).

---

## 11. Failure Modes and Operational Risks

### 11.1 Failure Mode Catalog

| # | Failure Mode | Deliverable | Mitigation |
|---|-------------|-------------|------------|
| F1 | Requested provider not in registry | D1 | `ProviderResolutionError::ProviderNotFound` with list of available providers |
| F2 | No API key for resolved provider | D1 | `ProviderResolutionError::NoCredentials` with instructions to run `/connect` |
| F3 | Provider/model conflict (e.g., `provider=openai` + `model=claude-opus-4-6`) | D1 | `ProviderResolutionError::ProviderModelConflict` with clear message |
| F4 | `ProviderRegistry` is `None` in worker ToolContext | D1 | **Hard error** — invariant violation, not a fallback path. Message: "No ProviderRegistry available. This is an internal error." |
| F5 | `ProviderRegistry` is `None` in root session | D1 | Legacy path: use `client: &AnthropicClient`. This is the **only** path where the AnthropicClient param is used. |
| F6 | `ProviderRegistry` is present, resolution fails | D1 | **Hard error.** No silent Anthropic fallback. |
| F7 | Provider unreachable, fallback disabled (default) | D2 | Error with suggestion to set `allow_fallback: true` |
| F8 | Provider unreachable, all fallbacks in same trust domain exhausted | D2 | Error listing all attempted providers and their statuses |
| F9 | Worker exceeds individual budget | D2 | `QueryOutcome::BudgetExceeded`, partial results returned |
| F10 | Session budget exceeded | D2 | Root cancellation token fired, cascades to all workers |
| F11 | Context-window mismatch after model change | D1 | Worker uses the resolved model's context window. Auto-compact handles it. |
| F12 | Stale health cache | D2 | `probe_if_stale()` re-probes when TTL expires |

### 11.2 Backward-Compatibility Risk

**The behavior change:** A user on `--provider openai` who spawns an Agent will now see that agent succeed on OpenAI, where it previously failed with "ANTHROPIC_API_KEY not set." This is the desired fix.

**New error path (P3):** A user who passes `provider: "openai"` and `model: "anthropic/claude-opus-4-6"` will now get an explicit conflict error. Previously, the inline code would silently use the `config.provider` and strip the prefix. This is technically a behavior change, but the old behavior was almost certainly not what the user intended.

**Edge case:** A user who set `--provider openai` but relied on agents failing (to avoid agent costs) will now see agents succeed and incur costs. Unlikely but documented.

### 11.3 AgentRunFn Signature Break

The `AgentRunFn` type change (Section 9.3) is a breaking internal API change. It affects exactly two call sites:

1. `init_team_swarm_runner()` in `crates/query/src/lib.rs` (producer)
2. `run_agent()` in `crates/tools/src/team_tool.rs` (consumer)

Both are in the same workspace.

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
    resolution_source = ?target.resolution_source,
    "Worker provider resolved"
);

// Fallback (D2)
warn!(
    agent_id = %agent_id,
    primary_provider = %primary,
    fallback_provider = %fallback,
    fallback_model = %fallback_model,
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

// Session budget cancellation (D2)
error!(
    total_spent_usd = spent,
    session_budget_usd = budget,
    active_workers = count,
    "Session budget exceeded — cancelling all workers"
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

    /// Session budget exceeded — all workers being cancelled (D2).
    SessionBudgetExceeded {
        total_spent_usd: f64,
        budget_usd: f64,
    },
}
```

---

## 13. Rollout Plan

### Deliverable 1: Two-Phase Resolution and Registry Propagation

**Scope:** ~900 LoC (600 production + 300 test)

| # | Task | File | Change |
|---|------|------|--------|
| 1.1 | Implement `resolve_provider_identity()` (extracted from `run_query_loop()`) | New: `crates/query/src/provider_resolution.rs` | ~100 LoC |
| 1.2 | Implement `materialize_provider()` (extracted from `run_query_loop()`) | `crates/query/src/provider_resolution.rs` | ~80 LoC |
| 1.3 | Add `ProviderResolutionError` with `ProviderModelConflict` variant | `crates/query/src/provider_resolution.rs` | ~30 LoC |
| 1.4 | Replace inline resolution in `run_query_loop()` with two-phase calls | `crates/query/src/lib.rs` | ~120 LoC of inline code replaced with ~20 LoC |
| 1.5 | Replace `AnthropicClient::new()` in `AgentTool::execute()` (foreground path) | `crates/query/src/agent_tool.rs:230-248` | ~30 LoC replaced |
| 1.6 | Replace `AnthropicClient::new()` in `AgentTool::execute()` (background path) | `crates/query/src/agent_tool.rs:541` | ~30 LoC replaced |
| 1.7 | Propagate `provider_registry` and `model_registry` in child `QueryConfig` | `crates/query/src/agent_tool.rs:339-360` | ~5 LoC |
| 1.8 | Migrate `AgentRunFn` to struct-based `AgentRunParams` | `crates/tools/src/team_tool.rs` | ~40 LoC new struct, ~20 LoC changed |
| 1.9 | Update `init_team_swarm_runner()` to use two-phase resolution and `AgentRunParams` | `crates/query/src/lib.rs:520-604` | ~40 LoC replaced |
| 1.10 | Add `provider` to Agent input schema (note: `model` already exists) | `agent_tool.rs` | ~10 LoC |
| 1.11 | Add `provider` and `model` to TeamCreate agent entry schema | `team_tool.rs` | ~15 LoC |
| 1.12 | Add `provider_registry` and `model_registry` to `ToolContext` | `crates/tools/src/lib.rs` | ~10 LoC |
| 1.13 | Unit tests for `resolve_provider_identity()` — all precedence matrix rows (P1-P10) | `crates/query/src/provider_resolution.rs` (tests module) | ~150 LoC |
| 1.14 | Unit tests for `materialize_provider()` — auth refresh, api_base override | `crates/query/src/provider_resolution.rs` (tests module) | ~80 LoC |
| 1.15 | Integration test: Agent with explicit provider | `crates/query/tests/` | ~40 LoC |
| 1.16 | Integration test: Agent inherits parent provider | `crates/query/tests/` | ~40 LoC |

**Verification:** `cargo test --workspace` passes. Every row of the precedence matrix (P1-P10) has a corresponding unit test. Agent can be spawned with `provider: "openai"`. Agent without `provider` inherits parent's provider.

### Deliverable 2: Budget, Health, Fallback, and Cancellation

**Scope:** ~1,400 LoC (900 production + 500 test)

| # | Task | File | Change |
|---|------|------|--------|
| 2.1 | Add `TrustDomain` enum | `crates/api/src/provider_types.rs` | ~20 LoC |
| 2.2 | Extend `ModelEntry` with `pdf_input`, `audio_input`, `structured_output`, `max_output_tokens` | `crates/api/src/model_registry.rs` | ~30 LoC |
| 2.3 | Add `Capability` enum and matching logic with unknown-data policy | `crates/query/src/provider_resolution.rs` | ~80 LoC |
| 2.4 | Implement `HealthCache` | New: `crates/query/src/health_cache.rs` | ~100 LoC |
| 2.5 | Implement `resolve_provider_with_fallback()` with model selection and degraded ordering | `crates/query/src/provider_resolution.rs` | ~150 LoC |
| 2.6 | Extend `CostTracker` with `agent_id` and `provider_id` | `crates/core/src/lib.rs` | ~15 LoC |
| 2.7 | Implement `SessionBudget` with cancellation token management | New: `crates/query/src/session_budget.rs` | ~120 LoC |
| 2.8 | Wire `SessionBudget` into root session and worker spawn, including child token creation | `crates/cli/src/main.rs`, `agent_tool.rs` | ~50 LoC |
| 2.9 | Update TeamCreate to create per-agent tokens as children of session budget root | `team_tool.rs`, `lib.rs` | ~30 LoC |
| 2.10 | Add budget check + cancellation in `run_query_loop()` post-turn | `crates/query/src/lib.rs` | ~20 LoC |
| 2.11 | Add `allow_fallback` and `budget_usd` to Agent/TeamCreate schemas | `agent_tool.rs`, `team_tool.rs` | ~20 LoC |
| 2.12 | Add `QueryEvent` variants | `crates/query/src/lib.rs` | ~20 LoC |
| 2.13 | Add `session_budget` and `health_cache` to `QueryConfig` and `ToolContext` | `crates/query/src/lib.rs`, `crates/tools/src/lib.rs` | ~15 LoC |
| 2.14 | Unit tests: precedence matrix with fallback, capability matching, unknown-data policy | `crates/query/src/provider_resolution.rs` | ~120 LoC |
| 2.15 | Unit tests: HealthCache TTL, concurrent access, probe-if-stale | `crates/query/src/health_cache.rs` | ~60 LoC |
| 2.16 | Unit tests: SessionBudget aggregation, check_and_cancel, child token cascading | `crates/query/src/session_budget.rs` | ~100 LoC |
| 2.17 | Unit tests: TrustDomain classification, fallback within domain, cross-domain forbidden | Various | ~60 LoC |
| 2.18 | Integration test: spawn-time fallback with model selection | `crates/query/tests/` | ~80 LoC |
| 2.19 | Integration test: budget enforcement and cancellation propagation | `crates/query/tests/` | ~80 LoC |

**Verification:** Worker with `budget_usd: 0.50` stops at cap. Session budget cancellation cascades to all workers. Worker with `allow_fallback: true` and unavailable primary falls back within trust domain. Local-pinned worker never falls back to cloud. Capability matching rejects models with `Unknown` capabilities when required.

### Delivery Order

```
D1 (Two-Phase Resolution)  ██████████████████████  ~2 weeks
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
| AC1 | `resolve_provider_identity()` matches precedence matrix rows P1-P10 exactly | Unit test per row |
| AC2 | `materialize_provider()` prefers runtime auth-store provider over registry (preserves `/connect` behavior) | Unit test |
| AC3 | `materialize_provider()` applies `api_base` override for ollama/lmstudio/llamacpp | Unit test |
| AC4 | Agent with `provider: "openai"` routes to OpenAI | Integration test |
| AC5 | Agent with `provider: "ollama"` routes to local Ollama | Integration test (requires Ollama) |
| AC6 | Agent without `provider` or `model` inherits parent session's provider | Unit test |
| AC7 | Agent with `model: "gemini-3-flash-preview"` resolves to Google via ModelRegistry | Unit test |
| AC8 | Agent with conflicting `provider: "openai"` and `model: "anthropic/claude-opus-4-6"` returns `ProviderModelConflict` error | Unit test |
| AC9 | TeamCreate with mixed `provider` per agent dispatches correctly | Integration test |
| AC10 | `ANTHROPIC_API_KEY` absent + `--provider openai` → agent spawns successfully | Integration test |
| AC11 | `ANTHROPIC_API_KEY` absent + no provider override + no registry → graceful error | Unit test |
| AC12 | Worker with `provider_registry: None` in ToolContext → hard error (not silent Anthropic fallback) | Unit test |
| AC13 | Root session with `provider_registry: None` → uses legacy AnthropicClient | Unit test |
| AC14 | Root session with `provider_registry: Some(...)` + resolution error → hard error (no Anthropic fallback) | Unit test |
| AC15 | All existing tests pass without modification (except AgentRunFn callers) | `cargo test --workspace` |

### 14.2 Deliverable 2 Acceptance

| # | Criterion | Type |
|---|----------|------|
| AC16 | Worker with `budget_usd: 0.01` stops within two turns | Unit test with mock |
| AC17 | Session budget exceeded → root cancel token fires → all worker cancel tokens fire | Unit test |
| AC18 | TeamDelete cancels per-agent token without affecting other workers | Unit test |
| AC19 | Nested agent cancellation cascades from parent to child | Unit test |
| AC20 | Spawn-time fallback within Cloud trust domain | Unit test with mock health |
| AC21 | Spawn-time fallback from Local provider stays Local | Unit test |
| AC22 | Cross-domain fallback is forbidden (no escape hatch) | Unit test |
| AC23 | `allow_fallback: false` (default) → no fallback, descriptive error | Unit test |
| AC24 | HealthCache respects TTL and re-probes stale entries | Unit test |
| AC25 | Capability matching: `Unknown` capability → ineligible (for `Option<bool>` fields) | Unit test |
| AC26 | Capability matching: provider-level fallback when model-level `None` | Unit test |
| AC27 | Fallback model selection: family match preferred, then default model | Unit test |
| AC28 | Degraded providers used only when no Healthy providers available | Unit test |

### 14.3 Performance

| # | Criterion | Target |
|---|----------|--------|
| PA1 | `resolve_provider_identity()` latency | < 100μs |
| PA2 | `materialize_provider()` latency (no network) | < 1ms |
| PA3 | `resolve_provider_with_fallback()` with cache hit | < 1ms |
| PA4 | `resolve_provider_with_fallback()` with probe | < 6 seconds (5s probe timeout + overhead) |
| PA5 | Memory overhead per worker | < 1KB (excluding conversation) |

---

## 15. Test Strategy

### 15.1 Unit Tests

| Component | Test Focus | Approach |
|-----------|-----------|----------|
| `resolve_provider_identity()` | All 10 precedence matrix rows, empty model, model with multiple slashes | Mock `ProviderRegistry` with 3-4 providers |
| `materialize_provider()` | Auth-store refresh, api_base override for each local provider, missing credentials | Mock auth store + mock registry |
| `HealthCache` | TTL expiry, concurrent access, probe-if-stale | Real `DashMap`, mock provider `health_check()` |
| `SessionBudget` | Aggregation, per-worker soft caps, session cap, check_and_cancel, child token cascading | Real `CostTracker` instances, real `CancellationToken` |
| `TrustDomain` | Classification correctness for all known providers | Exhaustive match |
| `resolve_provider_with_fallback()` | Fallback within trust domain, cross-domain blocked, model selection, degraded ordering, all-fail error | Mock health cache + mock providers + mock ModelRegistry |
| Capability matching | Each capability with `KnownTrue`, `KnownFalse`, `Unknown`; provider-level fallback | Mock ModelEntry with various `Option<bool>` states |

### 15.2 Integration Tests

| Scenario | Setup | Assertion |
|----------|-------|-----------|
| Multi-provider Agent | Registry with Anthropic + mock OpenAI | Agent with `provider: "openai"` hits mock |
| Mixed-provider Team | Registry with 2 providers | Each worker hits its specified provider |
| No-key fallback | Only OpenAI key configured | Agent spawns on OpenAI without ANTHROPIC_API_KEY |
| Budget enforcement | Worker with budget $0.01 | Worker stops after first turn |
| Trust domain isolation | Worker on `ollama`, allow_fallback, ollama down | Fallback only to other local providers |
| Cancellation cascade | Session budget hit with 3 active workers | All 3 workers observe cancellation |
| Provider/model conflict | `provider: "openai"` + `model: "anthropic/claude-opus-4-6"` | `ProviderModelConflict` error |

### 15.3 Regression

Run `cargo test --workspace` at each milestone. The `AgentRunFn` signature change is the only expected test breakage, contained to `init_team_swarm_runner` and any mocks.

---

## 16. Open Questions and Tradeoffs

### OQ1: Should `run_query_loop()` drop the `client: &AnthropicClient` parameter?

**Option A:** Keep it (backward compat for no-registry callers).
**Option B:** Remove it; all providers go through the registry.

**Recommendation:** Option A for D1. The Anthropic client parameter serves as a fallback when `provider_registry` is `None` (tests, legacy code paths that haven't migrated yet). Option B is a clean-up for a future RFC once all callers are migrated. When `provider_registry` is `Some(...)`, the client parameter is **never** used — resolution failures are hard errors.

### OQ2: Should capability matching (D2) use `ProviderCapabilities` or model-level data?

**Resolved:** Model-level data preferred. Existing `ModelEntry` fields (`tool_calling`, `reasoning`, `vision`) are used directly. New fields (`pdf_input`, `audio_input`, `structured_output`) are `Option<bool>` with fallback to `ProviderCapabilities` when `None` and "ineligible" when both are unknown. See Section 7.6.

### OQ3: Should there be a config toggle for the behavioral change?

**Recommendation:** No. The old behavior was a bug, not a feature.

### OQ4: How should `TeamCreateTool` handle partial resolution failures?

**Recommendation:** Spawn the workers that resolved successfully. Return errors for those that didn't. The coordinator sees partial results and can decide how to proceed.

### OQ5: Default `required_capabilities` (D2)?

**Resolved:** `[ToolCalling]` is implicit when no capabilities are specified.

### OQ6: Local trust domain extensibility?

**Deferred.** D2 only supports built-in provider IDs for trust-domain classification. Custom provider trust-domain override requires extending `ProviderConfig` with a `trust_domain` field and updating provider registration. The current `ProviderConfig` does not have a `type` or `trust_domain` field, and `provider_from_key()` only knows hardcoded IDs. A follow-on RFC will define this properly. *(Addresses v2 blocker #5.)*

### OQ7: Interaction with existing `fallback_model`?

**Clarified:** The existing `config.fallback_model` mechanism (lib.rs:1299-1319) handles per-turn model fallback on overloaded/rate-limit errors *within the same provider/client*. This is orthogonal to spawn-time *provider-level* fallback in D2. Both mechanisms can coexist:
- Spawn-time fallback (D2): chooses which provider a worker starts on.
- `fallback_model` (existing): switches models mid-session within that provider when the primary model is rate-limited.

---

## 17. Final Recommendation

### Ship Deliverable 1 First

D1 is a **bug fix**. Workers should inherit their parent's provider configuration. The fix is small (~900 LoC), low-risk, and immediately useful. The two-phase resolution preserves all existing root behavior (auth-store refresh, api_base overrides) while making the logic reusable. The normative precedence matrix eliminates ambiguity.

### Ship Deliverable 2 as a Follow-On

D2 adds safety rails (budgets, fallback, health checks, cancellation propagation) that matter for production use of mixed-provider swarms. Ship it 2-3 weeks after D1, informed by real user feedback.

### Deferred to Future RFCs

| Topic | Why Deferred |
|-------|-------------|
| Access profiles (full/read-only/search-only) | Separate concern from provider dispatch |
| Per-API-call fallback / mid-conversation migration | Requires message format re-serialization |
| Custom provider trust-domain override | Requires extending `ProviderConfig` and provider registration |
| Cross-domain fallback (`allow_cross_domain_fallback`) | Requires properly defined control surface (config vs schema vs both) |
| Campaign DAG / artifact bus | Different problem domain |
| Background health probing | Unnecessary until spawn-time probe latency becomes measurable |
| TUI enhancements (provider column, enhanced /cost) | Can ship independently |

### Risk Assessment

| Category | D1 | D2 |
|----------|----|----|
| Technical risk | **Minimal** — extracting existing code into two functions | **Low** — new components, well-scoped |
| Backward-compat risk | **Low** — behavior change is from "fail" to "succeed"; new P3 conflict error is also an improvement | **Minimal** — new fields are optional |
| Scope risk | **Minimal** — ~900 LoC, well-understood code; slightly higher than v2 estimate due to two-phase split and conflict detection | **Low-Medium** — ~1,400 LoC; cancellation propagation adds complexity but is well-defined |

### Naming Decision

Use **"Multi-Provider Worker Orchestration"** for all internal references.

---

## Appendix A: v2 Review Blockers and Dispositions

| # | v2 Blocker | Severity | Disposition in v3 |
|---|-----------|----------|-------------------|
| B1 | Provider/model precedence is contradictory | Blocking | **Resolved.** Added normative precedence matrix (Section 7.1.1) with 10 explicit rows covering all input combinations. Added `ProviderModelConflict` error variant. The matrix is declared authoritative over any other section. |
| B2 | Anthropic fallback escape hatch is too lenient and internally inconsistent | Blocking | **Resolved.** Strict rule: when `provider_registry` is `Some(...)`, resolution errors are hard errors — no silent Anthropic fallback. `client: &AnthropicClient` only used when `provider_registry` is `None`. Worker paths fail loudly on missing registry (invariant violation). F4/F5/F6 in failure catalog explicitly define these behaviors. (Section 6.3 principle #4, Section 8.1, Section 8.2) |
| B3 | `resolve_provider()` cannot preserve current root behavior (missing auth-store refresh, api_base overrides) | Blocking | **Resolved.** Split into two-phase pipeline: `resolve_provider_identity()` (pure logic) + `materialize_provider()` (auth-store refresh, api_base overrides). `materialize_provider()` takes `provider_configs` and handles all the side-effectful construction that the current inline code does at lib.rs:937-977. (Section 7.1) |
| B4 | `allow_cross_domain_fallback` referenced but never placed in any schema | Blocking | **Resolved.** Removed entirely from D2. Cross-domain fallback is forbidden — no escape hatch. If needed, a follow-on RFC will define the control surface properly. (Section 7.5, NG8) |
| B5 | Custom local-provider trust-domain support is hand-wavy | Blocking | **Resolved.** Explicitly deferred. D2 only supports built-in provider IDs. The fake config example from v2 OQ6 is removed. A follow-on RFC will extend `ProviderConfig` with `trust_domain` and define the custom provider contract. (Section 5.3, Section 7.4, OQ6) |
| B6 | Capability matching depends on metadata semantics the RFC does not pin down | Blocking | **Resolved.** Added explicit unknown-data policy table (Section 7.6). Existing `bool` fields (`tool_calling`, `reasoning`, `vision`) are always known. New `Option<bool>` fields fall back to `ProviderCapabilities`, then "ineligible" if both are unknown. Acknowledged that current `ModelEntry` already has partial metadata. (Section 3.1, Section 7.3) |
| B7 | Cancellation propagation not specified | Blocking | **Resolved.** Added Section 7.9 with explicit token ownership model, token lifecycle, stop points, TeamCreate integration, and nested worker cascading. `SessionBudget` owns root token; workers get child tokens via `child_cancel_token()`. Stop point is "next API call or tool dispatch, not mid-tool-execution." |

### Non-Blocking Issues from v2 Review (also addressed)

| Issue | Disposition in v3 |
|-------|-------------------|
| `ctx.worker_scheduler_registry` naming inconsistency | Fixed: `provider_registry` used consistently (Section 9.5) |
| `model` field already exists on Agent, rollout table blurs that | Fixed: rollout table and Section 9.6 distinguish existing vs new fields |
| `ProviderStatus::Degraded` ordering in fallback | Defined: Healthy > Degraded > Unavailable (Section 7.5) |
| `fallback_model` interaction not addressed | Clarified: orthogonal mechanisms, both coexist (OQ7) |
| D1 LoC estimate low once auth refresh + api_base preserved | Adjusted: D1 estimate raised from ~800 to ~900 LoC |
| `ModelRegistry` current state under-described | Fixed: Section 3.1 lists existing fields; Section 7.3 distinguishes existing from new |

---

*End of RFC v3. This document supersedes RFC-0001-v2 and RFC-0001-v1. It incorporates all 7 blocking findings from the v2 adversarial review (2026-04-08) and all non-blocking issues.*
