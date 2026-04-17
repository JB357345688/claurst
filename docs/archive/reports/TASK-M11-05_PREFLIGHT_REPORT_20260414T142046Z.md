# TASK-M11-05 Preflight Report

## Ticket ID

`TASK-M11-05 — resolve_provider_with_fallback()`

## Verdict

`GO`

## Current Branch

`feature/provider-resolution-seam`

## Current HEAD

`eb2677329b0b70be7035e1c14cf40c0b3990e126`

## Authority Files Reviewed

- `AGENTS.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md`
- `docs/archive/reports/M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md`
- `docs/archive/reports/TASK-M11-04_CLOSEOUT_REPORT_20260414T141351Z.md`

## Verified File Paths

- `src-rust/crates/query/src/provider_resolution.rs`
- `src-rust/crates/query/src/health_cache.rs`
- `src-rust/crates/api/src/model_registry.rs`
- `src-rust/crates/api/src/provider_types.rs`
- `src-rust/crates/api/src/registry.rs`
- `src-rust/crates/api/src/provider.rs`
- `src-rust/crates/query/src/lib.rs`
- `src-rust/crates/query/src/agent_tool.rs`
- `src-rust/crates/tools/src/lib.rs`
- `src-rust/crates/core/src/provider_id.rs`
- `src-rust/crates/api/src/providers/openai_compat_providers.rs`

## Preflight Verdict Summary

Live repo reality matches the accepted M11-04 baseline exactly on branch and `HEAD`. The M11-05 implementation can remain scope-tight. A minimal execution pass only needs to touch `src-rust/crates/query/src/provider_resolution.rs`; the `ModelRegistry` helper anticipated by the M10 plan is not needed in current repo reality because `ModelRegistry` already exposes provider-local model enumeration and provider-default selection helpers.

## Verified Symbols / Repo Facts

- `TrustDomain` exists in `src-rust/crates/api/src/provider_types.rs` and is re-exported by `claurst_api`.
  - Exact live use path for M11-05: `claurst_api::TrustDomain`
  - Live mapping: `"ollama" | "lmstudio" | "lm-studio" | "llamacpp" | "llama-cpp" => Local`, all others `Cloud`
- `Capability`, `DEFAULT_REQUIRED_CAPABILITIES`, `model_supports_capability()`, and `provider_supports_capability()` already exist in `src-rust/crates/query/src/provider_resolution.rs`.
  - Exact live use path inside the file: same-module items
- `HealthCache` exists in `src-rust/crates/query/src/health_cache.rs` and is re-exported by `claurst-query`.
  - Exact live use paths for M11-05:
    - internal: `crate::health_cache::HealthCache` or `crate::HealthCache`
    - external: `claurst_query::HealthCache`
- `ProviderRegistry` exists in `src-rust/crates/api/src/registry.rs` and is re-exported by `claurst_api`.
  - `provider_ids()` returns `Vec<&ProviderId>`
  - `get(&ProviderId)` returns `Option<&Arc<dyn LlmProvider>>`
- `ProviderStatus` exists in `src-rust/crates/api/src/provider_types.rs` with the exact live variants:
  - `Healthy`
  - `Degraded { reason: String }`
  - `Unavailable { reason: String }`
  - This is directly orderable for M11-05 as `Healthy > Degraded > skip Unavailable`
- `LlmProvider::health_check()` exists in `src-rust/crates/api/src/provider.rs`
  - `HealthCache::probe_if_stale(provider_id: &str, provider: &dyn LlmProvider) -> ProviderStatus` is the exact live cache/probe seam
- `ToolContext` already exposes the upstream registries needed by later tickets:
  - `provider_registry: Option<Arc<claurst_api::ProviderRegistry>>`
  - `model_registry: Option<Arc<claurst_api::ModelRegistry>>`

## Current Resolution Surface Reality

- `resolve_provider_identity()` exact signature:
  - `pub fn resolve_provider_identity(explicit_provider: Option<&str>, model: &str, model_registry: Option<&ModelRegistry>) -> Result<ProviderIdentity, ProviderResolutionError>`
- `materialize_provider()` exact signature:
  - `pub fn materialize_provider(identity: &ProviderIdentity, registry: &ProviderRegistry, provider_configs: &HashMap<String, ProviderConfig>) -> Result<ExecutionTarget, ProviderResolutionError>`
- `ExecutionTarget` exact live shape:
  - `provider_id: String`
  - `model_id: String`
  - `provider: Arc<dyn LlmProvider>`
  - `resolution_source: ResolutionSource`
- `ProviderResolutionError` exact live variants:
  - `ProviderNotFound(String)`
  - `NoProviderForModel(String)`
  - `NoCredentials(String)`
  - `ProviderModelConflict { provider: String, model: String, model_provider: String }`
  - `ProviderUnavailable(String)`
- Current message convention is `thiserror`-driven display text per variant. There is no existing suggestion-bearing variant/message for the required `allow_fallback == false` case.
- There is no existing `resolve_provider_with_fallback()` function and no existing same-domain candidate-enumeration helper in `provider_resolution.rs`.

## Existing Fallback-Resolution Feasibility

### Overall

Feasible without widening beyond M11-05.

### Dependency readiness

- M11-01 output is present: `TrustDomain::for_provider()` exists and covers both canonical and alias local-provider IDs.
- M11-03 output is present: capability helpers exist in `provider_resolution.rs`.
- M11-04 output is present: `HealthCache` exists, is exported from `claurst-query`, and has deterministic tests.

### Provider enumeration / health feasibility

- Same-domain candidate enumeration is directly feasible with current APIs:
  - iterate `provider_registry.provider_ids()`
  - filter via `TrustDomain::for_provider(provider_id)`
  - resolve provider object via `provider_registry.get(provider_id)`
  - probe cached health via `health_cache.probe_if_stale(provider_id, provider.as_ref()).await`
- Registry reality supports M11-05’s intended domain behavior:
  - local providers are registered under canonical IDs `ollama`, `lm-studio`, `llama-cpp`
  - trust-domain mapping includes those canonical IDs and their alias spellings

### Capability filtering feasibility

Capability filtering is feasible entirely with current data and does not require widening scope.

- Model-level data source: `ModelRegistry::get(provider_id, model_id) -> Option<&ModelEntry>`
- Provider-level fallback data source: `provider.capabilities() -> ProviderCapabilities`
- Existing helper behavior already matches M11-05’s intended rule:
  - use `model_supports_capability()` when model entry exists
  - fall back to `provider_supports_capability()` when model entry lacks optional data

### Fallback model-selection feasibility

The M10-planned `ModelRegistry` gap is no longer present in live repo reality.

- Current `ModelRegistry` already exposes:
  - `list_by_provider(provider_id: &str) -> Vec<&ModelEntry>`
  - `best_model_for_provider(provider_id: &str) -> Option<String>`
- Result:
  - provider-local model enumeration is already available
  - provider-default selection is already available
  - no new `models_for_provider()` helper is required

Assessment of the helper question required by the prompt:

- `ModelRegistry` helper is `unnecessary`
- no authority-alignment blocker exists on this point
- `src-rust/crates/api/src/model_registry.rs` does not need to be in the minimal M11-05 execution patch

### Same-domain enforcement feasibility

Sufficient current trust-domain data exists to forbid cross-domain fallback in code with no extra pre-work.

- Local provider IDs produced by registry factories:
  - `ollama`
  - `lm-studio`
  - `llama-cpp`
- Cloud providers default to `TrustDomain::Cloud`
- No existing API exposes any cross-domain override, which matches the ticket’s non-goal

### Narrow implementation note

One live nuance should be handled deliberately during execution:

- `resolve_provider_identity()` can currently fail only with `ProviderModelConflict`
- fallback is straightforward only once a concrete `ProviderIdentity` exists
- therefore the clear narrow path is:
  - resolve identity first
  - if identity resolution fails, return that error directly
  - apply fallback only after identity exists and direct materialization fails

This is not a blocker, but it should be stated explicitly in the execution pass because the authority text says “wrap `resolve_provider_identity()` + `materialize_provider()`” more broadly than the live failure modes justify.

### Suggestion-message feasibility

The required `allow_fallback == false` suggestion is feasible with a narrow in-file error change.

- Current `ProviderResolutionError` does not carry suggestion text
- M11-05 can satisfy the requirement by adding a new narrow error variant or a narrow wrapper variant in `provider_resolution.rs`
- No external matches on `ProviderResolutionError` were found outside `provider_resolution.rs`, so this is low-risk and in-scope

## Existing Test Reality

- Natural test location: the existing `#[cfg(test)]` module inside `src-rust/crates/query/src/provider_resolution.rs`
- Current provider-resolution test module already contains:
  - `TestProvider`
  - `run_async(...)`
  - registry setup helpers
  - isolated auth-store helpers
  - existing provider/materialization and capability tests
- `HealthCache` has its own deterministic unit tests in `src-rust/crates/query/src/health_cache.rs`

Deterministic M11-05 tests are feasible now:

- same-domain fallback succeeds:
  - create a cloud or local registry with multiple providers
  - preload `HealthCache` with `Healthy` / `Degraded` / `Unavailable`
  - use `ModelRegistry::list_by_provider()`-backed candidate model selection
- cross-domain prohibited:
  - construct mixed local/cloud candidates
  - assert only same-domain candidates are considered
- `allow_fallback: false` errors with suggestion:
  - force direct materialization failure
  - assert returned error string contains suggestion text

One small test-harness extension will likely be needed:

- current `provider_resolution.rs::tests::TestProvider` always reports `Healthy` and fixed capabilities
- fallback tests will want a slightly richer mock provider or a second mock type with configurable capabilities / health behavior

This remains narrow and local to the same test module.

## Validation / Test Reality Verified

Commands run in live repo reality:

- `cd src-rust && cargo check -p claurst-query`
  - Result: `PASS`
  - Note: existing unrelated warning in `crates/query/src/compact.rs` (`unused import: Role`)
- `cd src-rust && cargo test -p claurst-query provider_resolution`
  - Result: `PASS` (`26 passed`)
- `cd src-rust && cargo test -p claurst-query -- fallback`
  - Result: `PASS`, but only `1` existing unrelated legacy fallback test matched:
    - `tests::provider_registry_some_resolution_failure_does_not_fallback_to_legacy_anthropic`

Best narrow validation commands for M11-05 execution:

- `cd src-rust && cargo check -p claurst-query`
- `cd src-rust && cargo test -p claurst-query provider_resolution`
- Authority-intent filtered path after M11-05 tests are added:
  - `cd src-rust && cargo test -p claurst-query -- fallback`

Current interpretation of the authority-intent filter:

- feasible as a naming convention
- not yet ticket-specific until M11-05 adds fallback-named tests in `provider_resolution.rs`

## Drift Found

- Branch drift: none
- `HEAD` drift from accepted M11-04 closeout: none
- Target-file structural drift: none
- Wider worktree drift exists:
  - `.gitignore` modified
  - many untracked docs/report artifacts
  - `src-rust/target/` present as untracked
- M11-05 source-path hygiene is currently clean:
  - no live diff on `provider_resolution.rs`
  - no live diff on `health_cache.rs`
  - no live diff on `model_registry.rs`
  - no live diff on `provider_types.rs`
  - no live diff on `registry.rs`
- Planning drift from M10 report:
  - the predicted `ModelRegistry` helper gap no longer exists because `list_by_provider()` and `best_model_for_provider()` are already present
- Validation drift from authority shorthand:
  - `cargo test -p claurst-query -- fallback` currently matches an unrelated existing fallback test, so execution should add explicit fallback-named provider-resolution tests to make the filter meaningful

## Blockers

None.

## Hosted Ollama Compatibility Baseline Preserved

`yes`

Basis:

- current `provider_resolution.rs` still contains the accepted hosted/local Ollama normalization and materialization logic
- existing hosted/local coverage remains present and passing:
  - `normalize_ollama_api_base_*`
  - `materialize_provider_accepts_ollama_api_base_override`
  - LM Studio / llama.cpp override tests
- the minimal M11-05 execution scope does not require touching `AuthStore`, `registry.rs`, or the hosted-Ollama normalization helper

## Exact Recommendation For Next Step

Proceed to the execution prompt for `TASK-M11-05` with scope limited to:

- `src-rust/crates/query/src/provider_resolution.rs`

Execution should:

1. Add `resolve_provider_with_fallback(...)` in `provider_resolution.rs`
2. Use existing `TrustDomain`, capability helpers, `HealthCache`, `ModelRegistry::list_by_provider()`, and `ModelRegistry::best_model_for_provider()`
3. Keep cross-domain fallback forbidden in code
4. Add a narrow suggestion-bearing `ProviderResolutionError` path for `allow_fallback == false`
5. Add fallback-named unit tests in the existing `provider_resolution` test module for:
   - same-domain fallback success
   - cross-domain prohibition
   - `allow_fallback: false` suggestion
6. Validate with:
   - `cd src-rust && cargo check -p claurst-query`
   - `cd src-rust && cargo test -p claurst-query provider_resolution`
   - `cd src-rust && cargo test -p claurst-query -- fallback`
