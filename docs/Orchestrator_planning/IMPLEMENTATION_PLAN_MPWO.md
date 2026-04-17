# Multi-Provider Worker Orchestration (MPWO)
# Implementation Plan — Milestones 6 through 13

---

## 1. Executive Summary

This document is the execution plan for landing Multi-Provider Worker Orchestration in the Claurst codebase. It covers every milestone from the first implementation-planning task (Milestone 6) through final project closeout (Milestone 13).

The plan is grounded in two sources of truth:

- **RFC v3.1** — the authoritative design intent for the target state.
- **Reconciliation Report** — the authoritative record of what the current codebase actually implements today.

Where these two documents disagree, the reconciliation wins for describing current reality and the RFC wins for describing the intended end-state. This plan does not invent modules, helpers, or behavior that do not currently exist — it calls out every gap explicitly.

The work is organized into seven tracks:

1. **D1 Seam Extraction** — Extract the inline provider-resolution logic into reusable functions.
2. **D1 Propagation / Completion** — Wire workers (AgentTool, TeamCreate) through the shared seam.
3. **D1 Validation** — Prove D1 is landed with tests covering the precedence matrix, inheritance, pinning, and fail-loud behavior.
4. **D2 Planning** — Design the narrowest honest D2 landing at the real seam established by D1.
5. **D2 Implementation** — Land trust domains, health checks, fallback, budgets, and cancellation.
6. **Surrogate Test Retirement** — Replace the spec-proxy test once real D2 production behavior is testable.
7. **Final Integration / Closeout** — Close the loop between RFC intent and landed implementation.

---

## 2. Phase Transition Statement

The Claurst MPWO initiative is transitioning from architecture/RFC development into staged implementation.

**What is complete:**
- Architecture design (RFC v1 → v2 → v3 → v3.1) — stable enough to implement against.
- Adversarial review (7 blocking findings identified and resolved in v3/v3.1).
- Codebase reconciliation — repo reality mapped against RFC intent.
- First real production seam identified — the inline provider-resolution block at `crates/query/src/lib.rs:854-926`.
- Environment unblocked — toolchain, dependencies, and tests operational.

**What begins now:**
- Staged implementation, starting with D1 seam extraction.
- No more speculative RFC writing. Every task is grounded in specific code targets.
- D1 seam extraction is the hard prerequisite before any honest D2 work.

---

## 3. Current State vs Target State

### 3.1 Current State (from Reconciliation Report)

| Aspect | Status | Evidence |
|--------|--------|----------|
| Root provider resolution | Inline block at `lib.rs:854-926`; works but not reusable | Reconciliation §2 |
| Explicit provider pinning | PARTIAL — `"anthropic"` is filtered out at `lib.rs:863` via `filter(\|p\| *p != "anthropic")` | Reconciliation §3 |
| Parent/worker provider inheritance | MISSING — `AgentTool` sets `provider_registry: None` at `agent_tool.rs:356`; `AgentRunFn` has no provider params at `team_tool.rs:47` | Reconciliation §3 |
| Anthropic special-casing | Present in root (`lib.rs:863`), foreground agent (`agent_tool.rs:230`), and team runner (`agent_tool.rs:528`) | Reconciliation §3 |
| Provider/model conflict handling | MISSING — no `ProviderModelConflict` in production code | Reconciliation §3 |
| Trust-domain classification | MISSING — no `TrustDomain` in production code | Reconciliation §4 |
| Fallback candidate enumeration | MISSING — no production implementation | Reconciliation §4 |
| D2 surrogate test | Exists as a spec-proxy/RFC-anchor only | Reconciliation §6 |

### 3.2 Target State (from RFC v3.1)

| Aspect | Target |
|--------|--------|
| Provider resolution | Two-phase pipeline: `resolve_provider_identity()` (pure) + `materialize_provider()` (side-effectful), in new `crates/query/src/provider_resolution.rs` |
| Provider pinning | Uniform — all provider IDs including `"anthropic"` are true pins when explicit |
| Worker inheritance | Workers inherit `ProviderRegistry` + `ModelRegistry` from parent via `ToolContext` and `QueryConfig` |
| Conflict detection | `ProviderModelConflict` error for mismatched explicit provider + model prefix (P3, P5) |
| Trust domains | `TrustDomain::Local` / `TrustDomain::Cloud` classification for built-in provider IDs |
| Fallback | Spawn-time only, within same trust domain, via `resolve_provider_with_fallback()` |
| Budgets | Per-worker `CostTracker` + `SessionBudget` with soft USD caps |
| Cancellation | `SessionBudget` owns root cancel token; workers get child tokens |

---

## 4. Milestone Roadmap (Milestones 6–13)

```
M6  D1 Implementation Plan             ← THIS DOCUMENT (planning only)
M7  D1 Seam Extraction                 ← First real code change
M8  D1 Worker Propagation              ← AgentTool + TeamCreate wiring
M9  D1 Validation & Closeout           ← Tests proving D1 is landed
M10 D2 Implementation Planning         ← Design D2 at the real seam
M11 D2 Landing                         ← Trust domains, fallback, budgets, cancellation
M12 Surrogate Test Retirement          ← Replace spec-proxy with real D2 tests
M13 Final Integration & Closeout       ← Ship
```

**Dependency chain:** M7 → M8 → M9 → M10 → M11 → M12 → M13

M6 is this document. M7 is the first coding milestone.

---

## 5. Detailed Milestone Plans

---

### Milestone 6 — D1 Implementation Plan

**Purpose:** Produce a repo-grounded implementation plan for D1, not just an RFC.

**Goals & Scope:** Define exact code targets, preserved invariants, migration paths, test plan, and risks for the D1 seam extraction and propagation work.

**Status:** This document is the deliverable. Milestone 6 is complete upon acceptance of this plan.

**Exit Criteria:** This plan is reviewed and accepted as the execution guide for M7–M9.

---

### Milestone 7 — D1 Seam Extraction

**Purpose:** Extract the inline provider-resolution and provider-materialization logic from `run_query_loop()` into shared, reusable functions without changing any policy or behavior.

**Goals & Scope:**
- Create `crates/query/src/provider_resolution.rs` (new file).
- Implement `resolve_provider_identity()` — pure logic, no side effects.
- Implement `materialize_provider()` — side-effectful construction (auth-store refresh, api_base overrides).
- Define `ProviderIdentity`, `ExecutionTarget`, `ResolutionSource`, and `ProviderResolutionError` types.
- Replace the inline block at `lib.rs:854-926` and materialization at `lib.rs:931-977` with calls to the new functions.
- Root behavior must remain identical — this is a refactor, not a feature change.

**Exact Code Targets:**

| Target | File | Lines | Action |
|--------|------|-------|--------|
| Inline provider-resolution block | `crates/query/src/lib.rs` | 854–926 | Extract into `resolve_provider_identity()` |
| Anthropic filter guard | `crates/query/src/lib.rs` | 863 | Remove `filter(\|p\| *p != "anthropic")` — all explicit providers are true pins |
| Known-provider list | `crates/query/src/lib.rs` | 879–886 | Move into `provider_resolution.rs` as a shared constant |
| Runtime provider construction | `crates/query/src/lib.rs` | 937–977 | Extract into `materialize_provider()` |
| `use_provider_dispatch` decision | `crates/query/src/lib.rs` | 931–932 | Absorbed into the two-phase pipeline — when registry is `Some`, always use it |
| `ProviderResolutionError` (new) | `crates/query/src/provider_resolution.rs` | new | `ProviderNotFound`, `NoProviderForModel`, `NoCredentials`, `ProviderModelConflict`, `ProviderUnavailable` |
| `ProviderIdentity` (new) | `crates/query/src/provider_resolution.rs` | new | `provider_id`, `model_id`, `resolution_source` |
| `ExecutionTarget` (new) | `crates/query/src/provider_resolution.rs` | new | `provider_id`, `model_id`, `provider: Arc<dyn LlmProvider>`, `resolution_source` |
| `ResolutionSource` (new) | `crates/query/src/provider_resolution.rs` | new | `ExplicitProvider`, `ModelStringPrefix`, `ModelRegistry`, `Default` |
| Module declaration | `crates/query/src/lib.rs` | near top | `mod provider_resolution; pub use provider_resolution::*;` |

**What must be extracted:**

The entire decision tree at `lib.rs:854-926`:
1. `config.provider` check (currently filtered to skip `"anthropic"` — this filter is removed in D1).
2. `provider/model` prefix parsing against the known-provider list.
3. `ModelRegistry::find_provider_for_model()` lookup.
4. Default to `"anthropic"`.

Plus the materialization at `lib.rs:937-977`:
1. `runtime_provider_for()` auth-store check.
2. Registry lookup (`registry.get(&pid)`).
3. `api_base` override for local providers (ollama, lmstudio, llamacpp).
4. Final provider selection (`runtime_provider.or(registry_provider)`).

**What must remain behaviorally unchanged:**
- Root sessions with a `ProviderRegistry` must resolve providers identically (modulo the intentional removal of the `"anthropic"` filter).
- Root sessions without a `ProviderRegistry` must continue using the `client: &AnthropicClient` parameter unchanged.
- The `use_provider_dispatch` escape hatch (`provider_id_str != "anthropic" || client.api_key_is_empty()`) is removed: when `provider_registry` is `Some`, the two-phase pipeline is always used. When the pipeline succeeds, the resolved provider is used. When it fails, the call fails with an error — no silent Anthropic fallback.
- Auth-store refresh (supporting `/connect` runtime key additions) must be preserved in `materialize_provider()`.
- `api_base` overrides for local providers must be preserved in `materialize_provider()`.

**What must NOT be introduced yet:**
- No D2 types (`TrustDomain`, `HealthCache`, `SessionBudget`, `Capability`).
- No fallback behavior (`resolve_provider_with_fallback()`).
- No `allow_fallback` or `budget_usd` parameters.
- No changes to `AgentTool`, `TeamCreate`, or `AgentRunFn` — those are Milestone 8.
- No changes to `ToolContext` — that is Milestone 8.

**Preserved Invariants:**
1. `cargo test --workspace` passes after this milestone (root behavior unchanged except anthropic-filter removal).
2. Provider dispatch for non-Anthropic providers continues to work exactly as before.
3. The `client: &AnthropicClient` parameter is still accepted by `run_query_loop()` and used when `provider_registry` is `None`.
4. Capability shaping at `lib.rs:994-1003` is not moved — it stays inline in `run_query_loop()` and consumes `ExecutionTarget` output.

**Dependencies / Preconditions:**
- None. This is the first coding milestone.

**Risks & Assumptions:**
- **Risk:** The inline resolution logic may have subtle interactions with surrounding code (e.g., `effective_model` mutation earlier in the loop, `fallback_model` handling). **Mitigation:** Read the full loop carefully; the extraction must consume the same inputs and produce the same outputs.
- **Risk:** Removing the `"anthropic"` filter changes behavior for the edge case `--provider anthropic` + model with non-Anthropic prefix. **Mitigation:** This is an intentional, documented behavioral change (P5 in the precedence matrix). The old behavior was silently wrong.
- **Assumption:** The known-provider list at `lib.rs:879-886` is the complete set. Verify against `registry.rs:26-70` `provider_from_key()` match arms.

**Acceptance Gates / Exit Criteria:**
1. `resolve_provider_identity()` exists in `crates/query/src/provider_resolution.rs` and handles all 12 precedence matrix rows (P1–P12).
2. `materialize_provider()` exists and handles auth-store refresh + api_base overrides.
3. `ProviderModelConflict` is raised for P3 and P5 cases.
4. `run_query_loop()` at `lib.rs:854` now calls `resolve_provider_identity()` + `materialize_provider()` instead of inline logic.
5. When `provider_registry` is `Some` and resolution fails, the call fails with an error — no Anthropic fallback.
6. Unit tests for all 12 precedence matrix rows pass.
7. Unit tests for `materialize_provider()` (auth refresh, api_base override, missing credentials) pass.
8. `cargo test --workspace` passes.
9. `cargo clippy --workspace` passes.

**Deliverables / Outputs:**
- New file: `crates/query/src/provider_resolution.rs`
- Modified file: `crates/query/src/lib.rs` (inline extraction, module declaration)
- Unit tests in `provider_resolution.rs` `#[cfg(test)]` module

---

### Milestone 8 — D1 Worker Propagation

**Purpose:** Make AgentTool and TeamCreate worker paths use the shared provider-resolution seam established in M7, inheriting provider selection from the parent session.

**Goals & Scope:**
- Add `provider_registry` and `model_registry` fields to `ToolContext`.
- Remove hardcoded `AnthropicClient::new()` from both AgentTool paths (foreground + background).
- Remove hardcoded `AnthropicClient::new()` from `init_team_swarm_runner()`.
- Migrate `AgentRunFn` to struct-based `AgentRunParams`.
- Add `provider` field to `AgentInput` schema.
- Add `provider` and `model` fields to `AgentSpec` (TeamCreate).
- Workers resolve their provider via `resolve_provider_identity()` + `materialize_provider()`.
- Workers propagate `provider_registry` and `model_registry` to child `QueryConfig`.

**Exact Code Targets:**

| Target | File | Lines | Action |
|--------|------|-------|--------|
| `ToolContext` struct | `crates/tools/src/lib.rs` | 209–223 | Add `provider_registry: Option<Arc<ProviderRegistry>>` and `model_registry: Option<Arc<ModelRegistry>>` |
| `AgentInput` struct | `crates/query/src/agent_tool.rs` | ~125–150 (requires confirmation) | Add `provider: Option<String>` field |
| `AgentInput` schema | `crates/query/src/agent_tool.rs` | `input_schema()` method | Add `"provider"` to JSON schema |
| Foreground agent client creation | `crates/query/src/agent_tool.rs` | 229–248 | Replace `ANTHROPIC_API_KEY` + `AnthropicClient::new()` with `resolve_provider_identity()` + `materialize_provider()` using `ctx.provider_registry` |
| Foreground agent `QueryConfig` | `crates/query/src/agent_tool.rs` | 339–360 | Set `provider_registry: Some(registry.clone())` and `model_registry: ctx.model_registry.clone()` instead of `None` |
| Foreground agent model resolution | `crates/query/src/agent_tool.rs` | 264–268 | Use `resolve_provider_identity()` with `params.provider` and `params.model` as inputs, replacing the bare `DEFAULT_MODEL` fallback |
| Background agent client creation | `crates/query/src/agent_tool.rs` | ~370–420 (requires confirmation) | Same pattern as foreground: resolve via shared seam, propagate registries |
| `AgentRunFn` type | `crates/tools/src/team_tool.rs` | 47–58 | Replace 6-positional-param fn with `AgentRunParams` struct-based signature |
| `AgentRunParams` struct (new) | `crates/tools/src/team_tool.rs` | new | `description`, `prompt`, `tools`, `system_prompt`, `max_turns`, `ctx`, `provider_override`, `model_override` |
| `run_agent()` | `crates/tools/src/team_tool.rs` | 76–89 | Update to pass `AgentRunParams` |
| `AgentSpec` struct | `crates/tools/src/team_tool.rs` | 172–182 | Add `provider: Option<String>` and `model: Option<String>` fields |
| `TeamCreateTool::input_schema()` | `crates/tools/src/team_tool.rs` | `input_schema()` | Add `"provider"` and `"model"` to agent entry schema |
| `TeamCreateTool::execute()` | `crates/tools/src/team_tool.rs` | ~380–430 (requires confirmation) | Pass `spec.provider` and `spec.model` through to `run_agent()` via `AgentRunParams` |
| `init_team_swarm_runner()` | `crates/query/src/agent_tool.rs` | 517–607 | Replace `ANTHROPIC_API_KEY` + `AnthropicClient::new()` with `resolve_provider_identity()` + `materialize_provider()` using `ctx.provider_registry`; accept `AgentRunParams` |
| `ToolContext` construction in `main.rs` | `crates/cli/src/main.rs` | ~1108–1128 (requires confirmation) | Populate `provider_registry` and `model_registry` fields when constructing `ToolContext` |

**Provider Pinning Semantics (how this milestone handles them):**
- When `params.provider` is `Some("X")`, `resolve_provider_identity()` treats `X` as a pin — use `X` or fail. No fallback (D2 concept, not introduced here).
- When `params.provider` is `None` and `params.model` is `None`, the worker inherits the parent's default provider. If the parent is on OpenAI, the worker is on OpenAI.
- When `params.provider` is `None` but `params.model` is `Some("gemini-3-flash-preview")`, the `ModelRegistry` resolves it to Google.
- `ProviderModelConflict` is raised for conflicting explicit provider + model prefix (already implemented in M7).

**Parent/Worker Inheritance:**
- `ToolContext` gains `provider_registry` and `model_registry` fields.
- `main.rs` populates these fields with the root session's registries (the same `Arc`s already built at startup).
- Workers read from `ctx.provider_registry` — they share the parent's registry (read-only, `Arc`-wrapped).
- Workers set `provider_registry: Some(registry.clone())` in their child `QueryConfig`, enabling sub-agents of sub-agents to also inherit.

**Fail-Loud Behavior:**
- Worker paths NEVER silently fall back to Anthropic. Missing `provider_registry` in a worker `ToolContext` is an invariant violation — hard error.
- Resolution failure with registry present → hard error, descriptive message.
- No `ANTHROPIC_API_KEY` requirement for workers when using non-Anthropic providers.

**Removal of Anthropic-Only Assumptions:**
- `agent_tool.rs:230-248`: `ANTHROPIC_API_KEY` env var check + `AnthropicClient::new()` → removed, replaced with `resolve_provider_identity()` + `materialize_provider()`.
- `agent_tool.rs:528-541`: Same pattern in `init_team_swarm_runner()` → removed.
- `agent_tool.rs:356`: `provider_registry: None` → `provider_registry: Some(registry.clone())`.
- `agent_tool.rs:359`: `model_registry: None` → `model_registry: ctx.model_registry.clone()`.

**Preserved Invariants:**
1. Workers that do not specify `provider` or `model` behave identically to today when the parent is on Anthropic (the default provider resolution path returns Anthropic).
2. `run_query_loop()` signature is unchanged — it still accepts `client: &AnthropicClient` as first param.
3. The `client` parameter is still used for workers that somehow end up with `provider_registry: None` in `QueryConfig` (should not happen post-M8, but the legacy path remains for safety).
4. Existing tool tests that don't involve AgentTool or TeamCreate are unaffected.

**Dependencies / Preconditions:**
- Milestone 7 complete — `resolve_provider_identity()` and `materialize_provider()` exist and are tested.

**Risks & Assumptions:**
- **Risk:** `ToolContext` is constructed in multiple places (CLI main, tests, other tools). Adding required fields may break compilation in test helpers. **Mitigation:** New fields are `Option<...>`, so existing construction sites can set `None` and still compile. Only CLI main.rs populates them with real registries.
- **Risk:** `AgentRunFn` signature change is a breaking internal API change affecting `init_team_swarm_runner()` (producer) and `run_agent()` (consumer). **Mitigation:** Both call sites are in the same workspace. The change is mechanical.
- **Risk:** The `run_query_loop()` call inside `init_team_swarm_runner()` still takes `client: &AnthropicClient` as first param. Workers that resolve to Anthropic need a client. **Mitigation:** When the resolved provider is Anthropic, construct an `AnthropicClient` from the resolved credentials. When non-Anthropic, the `client` param is still passed but unused (the `provider_registry: Some(...)` branch in `run_query_loop()` takes over). Long-term, OQ1 from the RFC addresses removing the `client` param entirely — deferred.
- **Assumption:** `ToolContext` is in `crates/tools/src/lib.rs:209`. Adding `ProviderRegistry` and `ModelRegistry` fields requires `claurst-tools` to depend on `claurst-api`. **Verification needed:** Check if `claurst-tools` already depends on `claurst-api` in Cargo.toml. If not, this dependency must be added, or the fields should use a trait-object type to avoid the dependency. Alternatively, the fields could be `Option<Arc<dyn Any>>` with downcasting, but this is ugly. Prefer adding the dependency if possible.

**Acceptance Gates / Exit Criteria:**
1. `AgentTool::execute()` no longer reads `ANTHROPIC_API_KEY` or constructs `AnthropicClient` directly.
2. `init_team_swarm_runner()` no longer reads `ANTHROPIC_API_KEY` or constructs `AnthropicClient` directly.
3. Workers propagate `provider_registry` and `model_registry` in child `QueryConfig`.
4. `AgentRunFn` accepts `AgentRunParams` struct.
5. `AgentSpec` has `provider` and `model` fields.
6. `ToolContext` has `provider_registry` and `model_registry` fields.
7. `main.rs` populates `ToolContext` with root session registries.
8. Worker with `provider: "openai"` resolves to OpenAI (tested).
9. Worker without `provider` or `model` inherits parent's provider (tested).
10. Worker with missing `provider_registry` in `ToolContext` → hard error (tested).
11. `cargo test --workspace` passes.
12. `cargo clippy --workspace` passes.

**Deliverables / Outputs:**
- Modified: `crates/tools/src/lib.rs` (ToolContext fields)
- Modified: `crates/query/src/agent_tool.rs` (foreground + background agent, init_team_swarm_runner)
- Modified: `crates/tools/src/team_tool.rs` (AgentRunFn, AgentRunParams, AgentSpec, TeamCreateTool schema)
- Modified: `crates/cli/src/main.rs` (ToolContext construction)
- Tests: integration tests for agent with explicit provider, agent inheriting parent provider

---

### Milestone 9 — D1 Validation and Closeout

**Purpose:** Prove that D1 is actually landed with comprehensive test coverage, and declare D1 complete and shippable independently.

**Goals & Scope:**
- Write remaining unit and integration tests covering all D1 acceptance criteria (AC1–AC15 from RFC v3.1 §14.1).
- Verify no regression in existing test suite.
- Confirm that root and worker paths use the same core provider-resolution flow.

**Exact Code Targets:**

| Target | File | Action |
|--------|------|--------|
| Precedence matrix coverage | `crates/query/src/provider_resolution.rs` `#[cfg(test)]` | Verify all P1–P12 rows have passing unit tests (from M7) |
| Materialize auth-store refresh | `crates/query/src/provider_resolution.rs` `#[cfg(test)]` | Test that `materialize_provider()` prefers runtime auth-store over registry (preserves `/connect`) |
| Materialize api_base override | `crates/query/src/provider_resolution.rs` `#[cfg(test)]` | Test api_base override for ollama, lmstudio, llamacpp |
| Agent explicit provider routing | `crates/query/tests/` or inline | Integration test: Agent with `provider: "openai"` dispatches to OpenAI |
| Agent parent inheritance | `crates/query/tests/` or inline | Integration test: Agent without `provider`/`model` inherits parent |
| Agent conflict detection | `crates/query/src/provider_resolution.rs` `#[cfg(test)]` | Both directions: `openai` + `anthropic/model` (P3), `anthropic` + `openai/model` (P5) |
| Worker missing registry → hard error | `crates/query/src/agent_tool.rs` or `crates/query/tests/` | Unit test: ToolContext with `provider_registry: None` → invariant error |
| Root missing registry → legacy path | `crates/query/src/lib.rs` or `crates/query/tests/` | Unit test: QueryConfig with `provider_registry: None` → uses `client` param |
| Root registry + resolution failure → hard error | `crates/query/tests/` | Unit test: registry present, provider not found → error, no Anthropic fallback |
| TeamCreate mixed providers | `crates/tools/tests/` or `crates/query/tests/` | Integration test: TeamCreate with different providers per agent |
| No-key agent spawn | `crates/query/tests/` | Integration test: `ANTHROPIC_API_KEY` absent, `--provider openai` → agent succeeds |
| Regression suite | workspace-wide | `cargo test --workspace` passes |

**Validation Themes:**
1. **Migrated provider-resolution coverage:** Every row of the precedence matrix (P1–P12) has a unit test that calls `resolve_provider_identity()` with matching inputs and asserts the expected `(provider_id, model_id, resolution_source)` output.
2. **Provider pinning/conflict behavior:** P3 and P5 (conflicting provider + model prefix) return `ProviderModelConflict`. P6 (explicit `"anthropic"` pin with bare model known to another provider) honors the pin.
3. **Worker inheritance validation:** Worker with `None` provider inherits parent. Worker with explicit provider overrides parent.
4. **Regression protection:** No test reintroduces inline-only resolution or Anthropic-only worker construction. The three original hardcoded `AnthropicClient::new()` sites (agent_tool.rs:230, agent_tool.rs:541, agent_tool.rs:528 in init_team_swarm_runner) are verified to no longer exist.

**Preserved Invariants:**
1. All pre-existing tests pass without modification (except AgentRunFn signature callers, which were updated in M8).
2. `cargo clippy --workspace` clean.

**Dependencies / Preconditions:**
- Milestones 7 and 8 complete.

**Risks & Assumptions:**
- **Risk:** Integration tests may require mock providers. **Mitigation:** Use the existing `ProviderRegistry` with mock `LlmProvider` implementations. The `LlmProvider` trait is already defined and can be mocked.
- **Risk:** Some tests may require network access (e.g., Ollama). **Mitigation:** Mark these as `#[ignore]` with clear instructions. Focus unit tests on the resolution logic, which is pure.

**Acceptance Gates / Exit Criteria:**
1. All 15 acceptance criteria (AC1–AC15) from RFC v3.1 §14.1 are covered by tests.
2. `cargo test --workspace` passes.
3. `cargo clippy --workspace` passes.
4. Manual smoke test: run Claurst with `--provider openai`, spawn an agent, confirm it uses OpenAI.
5. D1 is declared complete and shippable independently.

**Deliverables / Outputs:**
- Complete test suite for D1 provider resolution
- D1 completion declaration — provider-aware worker orchestration (D1) is a landed codebase capability

---

### Milestone 10 — D2 Implementation Planning

**Purpose:** Re-evaluate the now-real D1 seam and design the narrowest honest D2 landing sequence.

**Goals & Scope:**
- This is a planning milestone, not a coding milestone.
- Evaluate the `provider_resolution.rs` seam established by D1 and determine the exact extension points for D2.
- Design the trust-domain classification, fallback candidate enumeration, same-domain fallback behavior, capability/health-based filtering, and cancellation propagation.
- Produce a D2 implementation plan grounded in the real seam, not the hypothetical one from the RFC.

**Planning Work:**

**A. Trust-Domain Classification:**
- Define `TrustDomain` enum (`Local`, `Cloud`) in `crates/api/src/provider_types.rs`.
- Hardcoded classification: `"ollama"`, `"lmstudio"`, `"llamacpp"` → `Local`; all others → `Cloud`.
- D2 only supports built-in provider IDs. Custom providers default to `Cloud`.
- Evaluate: does the `provider_resolution.rs` seam need to know about trust domains, or is trust-domain logic only in the fallback path?

**B. Fallback Candidate Enumeration:**
- `resolve_provider_with_fallback()` wraps `resolve_provider_identity()` + `materialize_provider()`.
- Candidates: all providers in `ProviderRegistry` sharing the primary's trust domain that pass health/capability checks.
- Ordering: `Healthy` > `Degraded` > skip `Unavailable`.
- Model selection within fallback: family match → provider default → skip.
- Evaluate: does the current `ProviderRegistry` API expose enough for candidate enumeration, or does it need a new method (e.g., `providers_in_domain()`)?

**C. Same-Domain Fallback Behavior:**
- Fallback is evaluated once at spawn time, not per API call.
- `allow_fallback: false` (default) → no fallback; error with suggestion.
- `allow_fallback: true` → try alternatives in same trust domain.
- Cross-domain fallback is forbidden — no escape hatch.

**D. Capability / Health-Based Filtering:**
- `HealthCache` with DashMap and TTL.
- `Capability` enum: `ToolCalling`, `Reasoning`, `Vision`, `PdfInput`, `AudioInput`, `StructuredOutput`.
- Unknown-data policy: existing `bool` fields always known; new `Option<bool>` fields fall back to `ProviderCapabilities`, then ineligible.
- Default required capabilities: `[ToolCalling]` implicit.

**E. Seam Re-Evaluation:**
- After D1, `provider_resolution.rs` contains `resolve_provider_identity()` and `materialize_provider()`. D2 adds `resolve_provider_with_fallback()` to the same file.
- Check: does the D1 seam's function signatures support the D2 extension cleanly, or do they need adjustment?
- Check: does `ProviderRegistry` need new methods for D2 (iterate providers, filter by capability)?

**Dependencies / Preconditions:**
- Milestone 9 complete — D1 is landed and validated.

**Acceptance Gates / Exit Criteria:**
1. D2 implementation plan produced, grounded in the real `provider_resolution.rs` seam.
2. All extension points identified and verified against the current codebase.
3. No premature D2 code written.

**Deliverables / Outputs:**
- D2 implementation plan document
- Updated risk assessment for D2 based on D1 experience

---

### Milestone 11 — D2 Landing

**Purpose:** Implement D2 at the real seam created by D1.

**Goals & Scope:**
- Add `TrustDomain` enum to `crates/api/src/provider_types.rs`.
- Extend `ModelEntry` with `pdf_input`, `audio_input`, `structured_output`, `max_output_tokens` (all `Option<bool>` / `Option<u32>`).
- Add `Capability` enum and matching logic with unknown-data policy to `provider_resolution.rs`.
- Implement `HealthCache` in new `crates/query/src/health_cache.rs`.
- Implement `resolve_provider_with_fallback()` in `provider_resolution.rs`.
- Extend `CostTracker` with `agent_id` and `provider_id` fields.
- Implement `SessionBudget` in new `crates/query/src/session_budget.rs`.
- Wire `SessionBudget` into root session and worker spawn.
- Add cancellation propagation: `SessionBudget` owns root cancel token, workers get child tokens.
- Add `allow_fallback` and `budget_usd` to Agent/TeamCreate schemas.
- Add `QueryEvent` variants for worker resolution, budget exceeded, session budget exceeded.

**Exact Code Targets:**

| Target | File | Action |
|--------|------|--------|
| `TrustDomain` enum | `crates/api/src/provider_types.rs` | New enum: `Local`, `Cloud`; `for_provider()` method |
| `ModelEntry` extension | `crates/api/src/model_registry.rs` | Add `pdf_input: Option<bool>`, `audio_input: Option<bool>`, `structured_output: Option<bool>`, `max_output_tokens: Option<u32>` |
| `Capability` enum | `crates/query/src/provider_resolution.rs` | New enum + matching logic |
| `HealthCache` | `crates/query/src/health_cache.rs` (new file) | DashMap-based cache with TTL, `probe_if_stale()` |
| `resolve_provider_with_fallback()` | `crates/query/src/provider_resolution.rs` | Wraps identity + materialize with health check + fallback |
| `CostTracker` extension | `crates/core/src/lib.rs` | Add `agent_id: Option<String>`, `provider_id: Option<String>` |
| `SessionBudget` | `crates/query/src/session_budget.rs` (new file) | Budget tracking, `check_and_cancel()`, `child_cancel_token()` |
| Budget wiring | `crates/cli/src/main.rs`, `crates/query/src/agent_tool.rs` | Create `SessionBudget` at root, pass to workers |
| TeamCreate cancel tokens | `crates/tools/src/team_tool.rs`, `crates/query/src/lib.rs` | Per-agent tokens as children of session budget root |
| Budget check in loop | `crates/query/src/lib.rs` | Add `session_budget.check_and_cancel()` after cost tracking |
| Schema updates | `agent_tool.rs`, `team_tool.rs` | Add `allow_fallback`, `budget_usd` to input schemas |
| `AgentRunParams` extension | `crates/tools/src/team_tool.rs` | Add `budget_usd: Option<f64>`, `allow_fallback: bool` |
| `QueryConfig` extension | `crates/query/src/lib.rs` | Add `session_budget: Option<Arc<SessionBudget>>` |
| `ToolContext` extension | `crates/tools/src/lib.rs` | Add `session_budget: Option<Arc<SessionBudget>>`, `health_cache: Option<Arc<HealthCache>>` |
| `QueryEvent` variants | `crates/query/src/lib.rs` | `WorkerProviderResolved`, `WorkerBudgetExceeded`, `SessionBudgetExceeded` |

**Important Constraint:**
- Do NOT invent a broad policy/config system unless genuinely required. The D2 implementation should be the narrowest honest extension of the D1 seam.
- `TrustDomain` classification is a hardcoded provider-ID match, not a configurable system.
- `allow_fallback` defaults to `false` — pinned behavior is the default.

**Preserved Invariants:**
1. All D1 behavior unchanged — new D2 fields are optional and default to existing behavior when absent.
2. `cargo test --workspace` passes (D1 + D2 tests).
3. Cross-domain fallback is forbidden — enforced at the code level, no config escape.

**Dependencies / Preconditions:**
- Milestone 10 complete — D2 plan produced and reviewed.

**Risks & Assumptions:**
- **Risk:** `SessionBudget` + cancellation token integration with existing `TeamCreate` per-agent cancel tokens may be tricky. TeamCreate already creates per-agent tokens. D2 makes these children of the session budget's root token. **Mitigation:** `CancellationToken::child_token()` from `tokio-util` handles this.
- **Risk:** `HealthCache` TTL and async probing adds latency to worker spawning. **Mitigation:** Cache hits are <1ms. Probes are bounded by a 5s timeout.
- **Risk:** `ModelEntry` field additions may require parsing changes in `ModelRegistry::load_cache()`. **Mitigation:** New fields are `Option<...>` — missing data defaults to `None`.

**Acceptance Gates / Exit Criteria:**
1. All 13 D2 acceptance criteria (AC16–AC28) from RFC v3.1 §14.2 are covered by tests.
2. `resolve_provider_with_fallback()` exists and handles trust-domain isolation, capability matching, and degraded ordering.
3. `SessionBudget` tracks costs and fires cancellation tokens.
4. Worker with `allow_fallback: true` falls back within same trust domain.
5. Local-pinned worker never falls back to Cloud.
6. `cargo test --workspace` passes.
7. `cargo clippy --workspace` passes.

**Deliverables / Outputs:**
- New files: `health_cache.rs`, `session_budget.rs`
- Extended: `provider_resolution.rs`, `provider_types.rs`, `model_registry.rs`, `lib.rs` (core + query), `agent_tool.rs`, `team_tool.rs`, `main.rs`
- D2 test suite

---

### Milestone 12 — Surrogate Test Retirement / Replacement

**Purpose:** Remove or replace the temporary RFC-anchor/spec-proxy test once real production D2 behavior is testable.

**Goals & Scope:**
- The current surrogate test (referenced in `d2_test_micro_patch_report.md`) records intended D2 semantics against real registry primitives without mutating production behavior. It was created as a placeholder.
- Once D2 production behavior exists (trust domains, fallback enumeration, capability matching), this surrogate is no longer needed.
- Either remove it entirely or rewrite it as a proper integration test that exercises real production D2 code paths.

**Decision Criteria:**
- **Keep temporarily (during M11):** While D2 is being implemented, the surrogate provides a cross-reference against RFC intent. It does no harm.
- **Remove/replace (M12 trigger):** Remove the surrogate once ALL of the following exist:
  1. `resolve_provider_with_fallback()` has unit tests covering trust-domain isolation, same-domain fallback, and cross-domain prohibition.
  2. `HealthCache` has unit tests covering TTL and probe-if-stale.
  3. `SessionBudget` has unit tests covering aggregation and cancellation cascading.
  4. At least one integration test exercises the full spawn-time fallback path.
- **What replaces it:** The D2 test suite from Milestone 11. The surrogate test's intent is subsumed by real production tests.

**Exact Code Targets:**
- `d2_test_micro_patch_report.md` — the report describing the surrogate. Remove or archive.
- The surrogate test itself — location requires confirmation (referenced in the report but not located during reconciliation). If it lives in `crates/api/tests/` or `crates/query/tests/`, identify and either delete or rewrite.

**Dependencies / Preconditions:**
- Milestone 11 complete — real D2 production tests exist.

**Acceptance Gates / Exit Criteria:**
1. No surrogate D2 logic remains as a stand-in for missing production behavior.
2. All RFC-intended D2 semantics are covered by real production tests.
3. `cargo test --workspace` passes.

**Deliverables / Outputs:**
- Removed: surrogate test file(s) and report
- Verified: D2 test coverage is comprehensive

---

### Milestone 13 — Final Integration and Project Closeout

**Purpose:** Close the loop between RFC intent and landed implementation. Provider-Aware Worker Orchestration is no longer an RFC initiative — it is a landed codebase capability.

**Goals & Scope:**
- Verify full alignment between RFC v3.1 and implemented behavior.
- Confirm all acceptance criteria (AC1–AC28) pass.
- Confirm performance targets (PA1–PA5) are met.
- Remove any temporary scaffolding, TODO comments, or conditional compilation gates.
- Update CLAUDE.md if needed to reflect new architecture.
- Produce a final summary of what was delivered vs what was deferred.

**Exact Code Targets:**
- All files modified across M7–M12.
- `CLAUDE.md` — update workspace architecture description if the new modules warrant it.
- RFC v3.1 — mark status as "Implemented" (or archive).

**Dependencies / Preconditions:**
- All milestones M7–M12 complete.

**Acceptance Gates / Exit Criteria:**
1. All 28 acceptance criteria (AC1–AC28) pass.
2. Performance targets (PA1–PA5) met.
3. `cargo test --workspace` passes.
4. `cargo clippy --workspace` passes.
5. No surrogate/temporary test logic remains.
6. Implementation matches RFC v3.1 intent (with documented deviations if any).

**Deliverables / Outputs:**
- Final implementation commit(s)
- Updated documentation
- Project closeout summary: delivered, deferred, lessons learned

---

## 6. Dependency and Sequencing Summary

```
M6 (this plan)
 │
 ▼
M7 (D1 Seam Extraction)
 │  Creates: provider_resolution.rs with resolve_provider_identity() + materialize_provider()
 │  Modifies: lib.rs (replaces inline resolution)
 │
 ▼
M8 (D1 Worker Propagation)
 │  Requires: M7 functions exist
 │  Creates: AgentRunParams, ToolContext fields
 │  Modifies: agent_tool.rs, team_tool.rs, main.rs
 │
 ▼
M9 (D1 Validation)
 │  Requires: M7 + M8 code complete
 │  Creates: Full D1 test suite
 │  Outcome: D1 declared shippable
 │
 ▼
M10 (D2 Planning)
 │  Requires: D1 landed (M9)
 │  Creates: D2 implementation plan
 │  Evaluates: real seam extension points
 │
 ▼
M11 (D2 Landing)
 │  Requires: M10 plan reviewed
 │  Creates: health_cache.rs, session_budget.rs, TrustDomain, Capability, fallback
 │  Modifies: provider_resolution.rs, provider_types.rs, model_registry.rs, etc.
 │
 ▼
M12 (Surrogate Retirement)
 │  Requires: M11 D2 tests exist
 │  Removes: surrogate test, d2_test_micro_patch_report.md
 │
 ▼
M13 (Closeout)
    Requires: All above complete
    Outcome: MPWO is a landed codebase capability
```

**No milestone may be skipped.** D1 seam extraction (M7) is the hard prerequisite for all subsequent work. D2 implementation (M11) must not begin before D1 is validated (M9).

---

## 7. Risks and Scope Discipline Rules

### 7.1 Scope Rules

1. **Do not jump directly to D2.** D1 seam extraction is the necessary bridge between the RFC and the current codebase.
2. **Do not invent broad policy/config systems.** `TrustDomain` is a hardcoded enum match. `allow_fallback` is a boolean field. No YAML policy files, no dynamic rule engines.
3. **Do not treat the surrogate D2 test as production coverage.** It is a spec-proxy only.
4. **Do not introduce D2 types during D1 work.** No `TrustDomain`, `HealthCache`, `SessionBudget`, or `Capability` until M11.
5. **Do not change `run_query_loop()` signature in D1.** The `client: &AnthropicClient` param stays. Its removal is a future clean-up (OQ1).
6. **Do not add `allow_cross_domain_fallback`.** Cross-domain fallback is forbidden in D2. No escape hatch.

### 7.2 Risk Register

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| Inline extraction introduces subtle behavioral change | Medium | High | Comprehensive P1–P12 tests; compare resolution outputs before/after |
| `ToolContext` field additions break compilation in tests | Low | Low | Fields are `Option<...>`; defaults to `None` |
| `AgentRunFn` signature change breaks internal callers | Certain | Low | Only 2 call sites; mechanical change |
| `claurst-tools` may not depend on `claurst-api` | Medium | Medium | Verify Cargo.toml; add dependency or use trait objects |
| Integration tests need mock providers | High | Low | Mock `LlmProvider` trait implementations |
| Performance regression from two-phase resolution | Low | Low | `resolve_provider_identity()` is pure logic, <100us |
| Custom providers appear as unintended Cloud fallback candidates (D2) | Medium | Medium | Default `allow_fallback: false`; explicit pinning recommended |

---

## 8. Immediate Next Actions

The first coding move after this plan is accepted:

1. **Create `crates/query/src/provider_resolution.rs`** with:
   - `KNOWN_PROVIDERS` constant (moved from `lib.rs:879-886`).
   - `ProviderIdentity`, `ExecutionTarget`, `ResolutionSource`, `ProviderResolutionError` types.
   - `resolve_provider_identity()` function body extracted from `lib.rs:854-926`.
   - `materialize_provider()` function body extracted from `lib.rs:937-977`.
   - `#[cfg(test)]` module with unit tests for P1–P12.

2. **Modify `crates/query/src/lib.rs`** to:
   - Add `mod provider_resolution; pub use provider_resolution::*;`.
   - Replace the inline block at lines 854–926 with a call to `resolve_provider_identity()`.
   - Replace the materialization block at lines 931–977 with a call to `materialize_provider()`.
   - Remove the `filter(|p| *p != "anthropic")` guard at line 863.
   - Remove the `use_provider_dispatch` variable — when registry is `Some`, always use the two-phase pipeline.

3. **Run `cargo test --workspace`** to verify no regression.

4. **Run `cargo clippy --workspace`** to verify clean build.

This is Milestone 7.

---

## 9. Final Phase Summary

| Phase | Status | Outcome |
|-------|--------|---------|
| Environment unblock (M0) | Complete | Toolchain, deps, tests working |
| Architecture design (M1–M2) | Complete | RFC v3.1 stable |
| Adversarial review (M3) | Complete | All 7 blockers resolved |
| Codebase reconciliation (M4) | Complete | Repo reality mapped |
| First seam identified (M5) | Complete | Inline block at lib.rs:854 |
| **D1 Implementation Plan (M6)** | **This document** | **Execution guide for M7–M13** |
| D1 Seam Extraction (M7) | Next | Extract resolve + materialize |
| D1 Worker Propagation (M8) | After M7 | AgentTool + TeamCreate wiring |
| D1 Validation (M9) | After M8 | Tests proving D1 landed |
| D2 Planning (M10) | After M9 | Design D2 at real seam |
| D2 Landing (M11) | After M10 | Trust domains, fallback, budgets |
| Surrogate Retirement (M12) | After M11 | Replace spec-proxy tests |
| Final Closeout (M13) | After M12 | MPWO is landed capability |

The project has crossed from RFC development into staged implementation. The first real coding move is M7: extract the inline provider-resolution block at `lib.rs:854-926` into `provider_resolution.rs`. Everything after that follows the milestone chain defined above.

---

*End of Implementation Plan. This document serves as the deliverable for Milestone 6.*
