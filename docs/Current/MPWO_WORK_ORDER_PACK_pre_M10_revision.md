# MPWO Work Order Pack

Generated from `IMPLEMENTATION_PLAN_MPWO.md` (Milestone 6 deliverable).
Revised 2026-04-14 to align M11 section with accepted M10 D2 plan (`M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md`).

**Date:** 2026-04-09 (original), 2026-04-14 (M11 revision)
**Scope:** M7 (detailed, D1 complete), M8 (detailed, D1 complete), M9 (actionable, D1 complete), M11 (actionable, revised per accepted M10 plan), M12 (actionable)
**Not included:** M10 (planning-only, complete), M13 (future closeout)
**D1 status:** Review-accepted per `D1_REVIEW_REPORT_20260413T233604Z.md`. All M7/M8/M9 tickets closed.

---

## 1. Execution Order Summary

```
TASK-M7-01  Create provider_resolution.rs with core types
TASK-M7-02  Extract resolve_provider_identity() from lib.rs:854-926
TASK-M7-03  Extract materialize_provider() from lib.rs:937-977
TASK-M7-04  Wire lib.rs to call new functions (replace inline blocks)
TASK-M7-05  Unit tests for resolve_provider_identity() (P1–P12)
TASK-M7-06  Unit tests for materialize_provider()
TASK-M7-07  Workspace validation

TASK-M8-01  Add provider_registry and model_registry to ToolContext
TASK-M8-02  Populate ToolContext fields in main.rs
TASK-M8-03  Add provider field to AgentInput and schema
TASK-M8-04  Replace foreground agent AnthropicClient with shared seam
TASK-M8-05  Replace background agent AnthropicClient with shared seam
TASK-M8-06  Migrate AgentRunFn to AgentRunParams
TASK-M8-07  Add provider/model to AgentSpec and TeamCreate schema
TASK-M8-08  Replace init_team_swarm_runner() AnthropicClient with shared seam
TASK-M8-09  Wire TeamCreateTool::execute() to pass provider/model through AgentRunParams
TASK-M8-10  Unit tests for worker provider resolution
TASK-M8-11  Workspace validation

TASK-M9-01  Complete P1-P12 coverage audit
TASK-M9-02  Materialize coverage (auth-store, api_base, missing creds)
TASK-M9-03  Agent explicit provider routing integration test
TASK-M9-04  Agent parent inheritance integration test
TASK-M9-05  Agent conflict detection tests
TASK-M9-06  Worker missing registry → hard error test
TASK-M9-07  Root missing registry → legacy path test
TASK-M9-08  Root registry + resolution failure → hard error test
TASK-M9-09  TeamCreate mixed providers integration test
TASK-M9-10  No-key agent spawn integration test
TASK-M9-11  Hardcoded-removal verification test
TASK-M9-12  Full regression + D1 completion declaration

TASK-M11-01 TrustDomain enum
TASK-M11-02 ModelEntry extension
TASK-M11-03 Capability enum and matching
TASK-M11-04 HealthCache implementation
TASK-M11-05 resolve_provider_with_fallback()
TASK-M11-06 CostTracker extension
TASK-M11-07 SessionBudget implementation
TASK-M11-08 Budget + cancellation wiring
TASK-M11-09 Child execution override wiring (max_tokens + spawn-time settings)
TASK-M11-10 Schema updates (allow_fallback, budget_usd)
TASK-M11-11 QueryEvent variants
TASK-M11-12 D2 test suite + workspace validation

TASK-M12-01 Locate and audit surrogate test
TASK-M12-02 Remove/replace surrogate test
TASK-M12-03 D2 coverage completeness verification
```

---

## 2. Ticket Dependency Graph

```
M7 Seam Extraction:
  M7-01 → M7-02 → M7-04
  M7-01 → M7-03 → M7-04
  M7-04 → M7-05
  M7-04 → M7-06
  M7-05 → M7-07
  M7-06 → M7-07

M8 Worker Propagation (all require M7-07 complete):
  M8-01 → M8-02
  M8-01 → M8-04
  M8-01 → M8-05
  M8-03 → M8-04
  M8-03 → M8-05
  M8-06 → M8-08
  M8-07 → M8-09
  M8-06 → M8-09
  M8-08 → M8-10
  M8-04 → M8-10
  M8-05 → M8-10
  M8-09 → M8-10
  M8-10 → M8-11

M9 Validation (requires M8-11 complete):
  M9-01 through M9-11 can run in parallel
  All → M9-12

M11 D2 Landing (requires M10 planning complete):
  M11-01 → M11-05
  M11-02 → M11-03 → M11-05
  M11-04 → M11-05
  M11-06 → M11-07 → M11-08
  M11-08 → M11-09
  M11-05 → M11-10
  M11-08 → M11-10
  M11-09 → M11-10
  M11-08 → M11-11
  M11-09 → M11-11
  M11-10 → M11-12
  M11-11 → M11-12

M12 Surrogate Retirement (requires M11-12 complete):
  M12-01 → M12-02 → M12-03
```

## 2A. Standing Non-Regression Invariant — Hosted Ollama Compatibility (Remaining M7–M12 Seam Work)

For all remaining tickets in Milestones 7 through 12 that touch provider resolution, provider materialization, provider config handling, auth-store lookup, runtime provider selection, trust-domain classification, fallback behavior, or seam-validation tests, the hosted Ollama compatibility behavior accepted in commit `5f8dfe1` is treated as authoritative baseline and is not active patch scope unless a later ticket explicitly says otherwise.

Remaining in-scope tickets MUST NOT silently weaken, bypass, or regress any of the following baseline behaviors:

- hosted/root Ollama URL normalization via `normalize_ollama_api_base(...)`
- Ollama auth lookup via `AuthStore::load().api_key_for(ProviderId::OLLAMA)`
- environment-first precedence for `AuthStore::api_key_for("ollama")`

Execution rule:

- treat the hosted Ollama compatibility behavior from `5f8dfe1` as accepted baseline
- do not revert, refactor away, or behaviorally alter that baseline as incidental cleanup
- if a ticket touches provider resolution, provider materialization, provider config handling, auth-store lookup paths, runtime provider selection, local/cloud provider classification, or fallback resolution, explicitly verify that this baseline remains intact
- if a planned change appears to interact with these behaviors, stop and escalate unless the ticket explicitly includes updated hosted Ollama acceptance criteria

Review / acceptance rule:

Any preflight, execution, or review report for an in-scope ticket MUST explicitly state one of:

- `Hosted Ollama compatibility baseline preserved`
- `Hosted Ollama compatibility baseline intentionally changed by explicit ticket scope`

Silence is not sufficient.

This invariant does not require every in-scope ticket to add new dedicated Ollama regression tests. It does require later tickets to remain neutral to the accepted hosted Ollama compatibility baseline unless an explicit later ticket re-scopes that behavior.

---

## 3. Milestone 7 Detailed Tickets

---

### TASK-M7-01 — Create provider_resolution.rs and core types

**Milestone:** 7

**Objective:** Create the new module file and define all type scaffolding that `resolve_provider_identity()` and `materialize_provider()` will use. This is the foundation for seam extraction.

**Why This Ticket Exists:** The inline provider-resolution logic at `lib.rs:854-926` is not reusable. Workers (M8) need the same logic. The types must exist before the extraction functions can be written.

**Exact Code Targets:**

| Target | File | Status |
|--------|------|--------|
| New module file | `crates/query/src/provider_resolution.rs` | **Confirmed** — file does not exist |
| `mod` declaration | `crates/query/src/lib.rs` line ~11 (near other `pub mod` lines) | **Confirmed** — see `lib.rs:11-21` |
| `ResolutionSource` enum | `crates/query/src/provider_resolution.rs` | New |
| `ProviderIdentity` struct | `crates/query/src/provider_resolution.rs` | New |
| `ExecutionTarget` struct | `crates/query/src/provider_resolution.rs` | New |
| `ProviderResolutionError` enum | `crates/query/src/provider_resolution.rs` | New |
| `KNOWN_PROVIDERS` constant | `crates/query/src/provider_resolution.rs` | New — sourced from `lib.rs:879-886` **and** `registry.rs:26-70` |

**Preconditions:**
- None. This is the first coding ticket.

**Step-by-Step Instructions:**

1. Create file `crates/query/src/provider_resolution.rs`.
2. Add the following `use` statements at the top:
   ```rust
   use std::sync::Arc;
   use claurst_api::{LlmProvider, ProviderRegistry, ModelRegistry};
   use claurst_core::ProviderId;
   ```
3. Define `KNOWN_PROVIDERS` as a `&[&str]` constant. **CRITICAL:** The list must be the union of:
   - `lib.rs:879-886` current known-providers list, AND
   - all match arms in `registry.rs:26-70` `provider_from_key()`
   The merged list is:
   ```rust
   pub const KNOWN_PROVIDERS: &[&str] = &[
       "anthropic", "openai", "google", "groq", "mistral",
       "deepseek", "xai", "cohere", "perplexity", "cerebras",
       "openrouter", "togetherai", "together-ai", "deepinfra",
       "venice", "github-copilot", "ollama", "lmstudio",
       "llamacpp", "azure", "amazon-bedrock", "huggingface",
       "nvidia", "fireworks", "sambanova", "codex",
       "siliconflow", "moonshot", "zhipu", "qwen", "nebius",
       "novita", "ovhcloud", "scaleway", "vultr", "vultr-ai",
       "baseten", "friendli", "upstage", "stepfun",
   ];
   ```
   Verify this against `registry.rs:26-70` before committing. If any new providers have been added to `provider_from_key()` since this ticket was written, include them.
4. Define `ResolutionSource`:
   ```rust
   #[derive(Debug, Clone, PartialEq, Eq)]
   pub enum ResolutionSource {
       ExplicitProvider,
       ModelStringPrefix,
       ModelRegistry,
       Default,
   }
   ```
5. Define `ProviderIdentity`:
   ```rust
   #[derive(Debug, Clone)]
   pub struct ProviderIdentity {
       pub provider_id: String,
       pub model_id: String,
       pub resolution_source: ResolutionSource,
   }
   ```
6. Define `ExecutionTarget`:
   ```rust
   #[derive(Debug, Clone)]
   pub struct ExecutionTarget {
       pub provider_id: String,
       pub model_id: String,
       pub provider: Arc<dyn LlmProvider>,
       pub resolution_source: ResolutionSource,
   }
   ```
7. Define `ProviderResolutionError`:
   ```rust
   #[derive(Debug, thiserror::Error)]
   pub enum ProviderResolutionError {
       #[error("Provider '{0}' not found in registry")]
       ProviderNotFound(String),
       #[error("No provider found for model '{0}'")]
       NoProviderForModel(String),
       #[error("No credentials available for provider '{0}'")]
       NoCredentials(String),
       #[error("Provider/model conflict: explicit provider '{provider}' but model '{model}' belongs to '{model_provider}'")]
       ProviderModelConflict {
           provider: String,
           model: String,
           model_provider: String,
       },
       #[error("Provider '{0}' is unavailable")]
       ProviderUnavailable(String),
   }
   ```
8. Add stub function signatures (bodies will be filled in M7-02 and M7-03):
   ```rust
   pub fn resolve_provider_identity(
       explicit_provider: Option<&str>,
       model: &str,
       model_registry: Option<&ModelRegistry>,
   ) -> Result<ProviderIdentity, ProviderResolutionError> {
       todo!("M7-02")
   }

   pub fn materialize_provider(
       identity: &ProviderIdentity,
       registry: &ProviderRegistry,
       provider_configs: &std::collections::HashMap<String, claurst_core::config::ProviderConfig>,
   ) -> Result<ExecutionTarget, ProviderResolutionError> {
       todo!("M7-03")
   }
   ```
9. Open `crates/query/src/lib.rs`. Add `pub mod provider_resolution;` near line 11 (after the other `pub mod` lines). Add `pub use provider_resolution::*;` near line 21 (after the other `pub use` lines).
10. Verify it compiles: `cd src-rust && cargo check -p claurst-query`.

**Strict Constraints — Do NOT:**
- Do NOT implement the function bodies yet — use `todo!()` stubs.
- Do NOT add any D2 types (`TrustDomain`, `HealthCache`, `SessionBudget`, `Capability`, `allow_fallback`).
- Do NOT modify any other file besides `provider_resolution.rs` and `lib.rs` (module declaration only).
- Do NOT remove or modify any existing code in `lib.rs` beyond adding the module declaration and pub use.
- Do NOT add `thiserror` as a dependency without first checking if it is already in `crates/query/Cargo.toml`. If it is not, add it.

**Definition of Done:**
- `provider_resolution.rs` exists with all 4 types, the `KNOWN_PROVIDERS` constant, and two `todo!()` stub functions.
- `lib.rs` has `pub mod provider_resolution;` and `pub use provider_resolution::*;`.
- `cargo check -p claurst-query` compiles without errors.

**Validation Commands:**
```bash
cd src-rust && cargo check -p claurst-query
```

**Expected Output / Artifact Changes:**
- New: `crates/query/src/provider_resolution.rs`
- Modified: `crates/query/src/lib.rs` (2 lines added near top)
- Possibly modified: `crates/query/Cargo.toml` (if `thiserror` not already present)

**Stop / Escalate Conditions:**
- If `crates/query/src/lib.rs` does not have the `pub mod` block near lines 11-21 as expected, investigate before adding the declaration elsewhere.
- If `thiserror` is not available and cannot be added, use manual `impl std::fmt::Display` + `impl std::error::Error` instead.
- If `claurst_api::ProviderRegistry`, `claurst_api::ModelRegistry`, or `claurst_api::LlmProvider` are not importable from `claurst-query`, stop and investigate the dependency graph.

---

### TASK-M7-02 — Implement resolve_provider_identity()

**Milestone:** 7

**Objective:** Implement the pure provider-identity resolution function that replaces the inline decision tree at `lib.rs:854-926`.

**Why This Ticket Exists:** The inline resolution logic must be extracted into a reusable function so that workers (M8) can call it too. This function is pure — no side effects, no I/O.

**Exact Code Targets:**

| Target | File | Lines | Status |
|--------|------|-------|--------|
| Resolution decision tree | `crates/query/src/lib.rs` | 854–926 | **Confirmed** — to be extracted |
| `resolve_provider_identity()` stub | `crates/query/src/provider_resolution.rs` | from M7-01 | **Confirmed** |

**Preconditions:**
- TASK-M7-01 complete — types and stubs exist.

**Step-by-Step Instructions:**

1. Open `crates/query/src/lib.rs` at lines 854-926 and study the decision tree.
2. Open `crates/query/src/provider_resolution.rs`.
3. Replace the `todo!("M7-02")` body of `resolve_provider_identity()` with logic that implements the following precedence (matching the inline logic at lib.rs:854-926, with the `"anthropic"` filter **removed**):

   **P1:** `explicit_provider` is `Some(p)` AND `model` starts with `"{p}/"` → provider=`p`, model=strip prefix. Source=`ExplicitProvider`.

   **P2:** `explicit_provider` is `Some(p)` AND `model` has no provider prefix → provider=`p`, model=`model`. Source=`ExplicitProvider`.

   **P3:** `explicit_provider` is `Some(p)` AND `model` starts with `"{other_known_provider}/"` where other_known ≠ p → return `ProviderModelConflict`.

   **P4:** `explicit_provider` is `None` AND `model` is `"{known_provider}/{model_name}"` → provider=known_provider, model=model_name. Source=`ModelStringPrefix`.

   **P5:** `explicit_provider` is `Some(p)` AND `model` is `"{other_known_provider}/{model_name}"` where other_known ≠ p → return `ProviderModelConflict`. (Same as P3 but from a different entry path; may collapse.)

   **P6:** `explicit_provider` is `Some("anthropic")` AND `model` is bare (no prefix) → provider=`"anthropic"`, model=`model`. Source=`ExplicitProvider`. (Anthropic pin is now honored — the old filter is removed.)

   **P7:** `explicit_provider` is `None` AND `model` is `"{unknown_namespace}/{model_name}"` (not a known provider prefix) → fall through to registry lookup or default.

   **P8:** `explicit_provider` is `None` AND `model` is bare AND `model_registry.find_provider_for_model(model)` returns `Some(pid)` → provider=`pid`, model=`model`. Source=`ModelRegistry`.

   **P9:** `explicit_provider` is `None` AND `model` is bare AND registry lookup returns `None` → provider=`"anthropic"`, model=`model`. Source=`Default`.

   **P10-P12:** Edge cases from the precedence matrix (explicit anthropic + known model from another provider is P3/P5; explicit provider + bare model is P2).

4. Implementation guidance — translate the existing inline logic directly:
   - First check: if `explicit_provider` is `Some(p)`:
     - Strip `"{p}/"` prefix from model if present. This is P1.
     - If model contains `/` and the prefix is a different known provider, return `ProviderModelConflict`. This is P3/P5.
     - Otherwise use `p` as provider, model as-is. This is P2/P6.
   - Second check: if `explicit_provider` is `None` and model contains `/`:
     - Split on first `/`. If the left part is in `KNOWN_PROVIDERS`, use it. This is P4.
     - If not a known provider, fall through. This is P7.
   - Third check: `model_registry.find_provider_for_model(model)`. This is P8.
   - Default: `"anthropic"`. This is P9.

5. **CRITICAL difference from current inline code:** The current code at line 863 has `.filter(|p| *p != "anthropic")` which skips the explicit-provider path when provider is `"anthropic"`. In the new function, **do NOT filter out anthropic**. All explicit providers, including `"anthropic"`, are true pins. This is the one intentional behavioral change.

6. **For P7 (unknown namespace prefix with no explicit provider):** The current code at `lib.rs:892-893` falls back to `config.provider.unwrap_or("anthropic")`. In the extracted function, since `explicit_provider` is `None` in P7, fall back to `"anthropic"` as default (the `explicit_provider` param already captures the config provider). The caller will pass `config.provider.as_deref()` as `explicit_provider`.

7. Verify the function signature matches what M7-01 created. If `model_registry` parameter type needs adjustment to accept the temp-registry fallback from `lib.rs:900-913`, adjust accordingly — the caller will handle constructing the registry.

**Strict Constraints — Do NOT:**
- Do NOT add any fallback behavior beyond defaulting to `"anthropic"`.
- Do NOT add `allow_fallback`, `TrustDomain`, or any D2 concepts.
- Do NOT modify `lib.rs` yet — that is M7-04.
- Do NOT add network calls, auth-store lookups, or any side effects. This function is pure.
- Do NOT change the `KNOWN_PROVIDERS` list unless `registry.rs:26-70` has changed since M7-01.

**Definition of Done:**
- `resolve_provider_identity()` has a complete body that handles P1-P12.
- No `todo!()` remains in this function.
- `cargo check -p claurst-query` compiles.

**Validation Commands:**
```bash
cd src-rust && cargo check -p claurst-query
```

**Expected Output / Artifact Changes:**
- Modified: `crates/query/src/provider_resolution.rs` (function body filled in)

**Stop / Escalate Conditions:**
- If the inline logic at `lib.rs:854-926` has changed materially from what is described here (e.g., new branches, different fallback logic), stop and re-read the full block before proceeding.
- If `ModelRegistry::find_provider_for_model()` does not exist or has a different signature than expected, stop and investigate.

---

### TASK-M7-03 — Implement materialize_provider()

**Milestone:** 7

**Objective:** Implement the side-effectful provider materialization function that replaces the inline construction at `lib.rs:937-977`.

**Why This Ticket Exists:** The materialization logic (auth-store refresh, registry lookup, api_base overrides) must be extracted so workers can reuse it. Unlike `resolve_provider_identity()`, this function has side effects (reads auth store, constructs providers).

**Exact Code Targets:**

| Target | File | Lines | Status |
|--------|------|-------|--------|
| Runtime provider construction | `crates/query/src/lib.rs` | 937–977 | **Confirmed** |
| `runtime_provider_for()` | `crates/api/src/registry.rs` | 73–80 | **Confirmed** |
| `materialize_provider()` stub | `crates/query/src/provider_resolution.rs` | from M7-01 | **Confirmed** |

**Preconditions:**
- TASK-M7-01 complete — types and stubs exist.

**Step-by-Step Instructions:**

1. Open `crates/query/src/lib.rs` at lines 937-977.
2. Open `crates/query/src/provider_resolution.rs`.
3. Review the `materialize_provider()` function signature from M7-01. Verify it accepts:
   - `identity: &ProviderIdentity` (from `resolve_provider_identity()`)
   - `registry: &ProviderRegistry` (the provider registry)
   - `provider_configs: &HashMap<String, ProviderConfig>` (for api_base overrides)
4. Replace the `todo!("M7-03")` body with logic extracted from `lib.rs:937-977`:

   a. **Auth-store refresh (lines 941-942):** Call `claurst_api::registry::runtime_provider_for(&identity.provider_id)` to get a fresh provider from the auth store. This ensures `/connect` runtime key additions are picked up.

   b. **Registry lookup (lines 944-950):** If `runtime_provider` is `Some`, skip registry lookup (auth store takes priority). Otherwise, construct a `ProviderId` from `identity.provider_id` and call `registry.get(&pid).cloned()`.

   c. **api_base override (lines 956-977):** Check `provider_configs.get(&identity.provider_id)` for an `api_base` override. If present and the provider is `"ollama"`, `"lmstudio"`, `"lm-studio"`, `"llamacpp"`, or `"llama-cpp"`, rebuild the provider with the override URL using `openai_compat_providers`. Copy the exact match arms from `lib.rs:963-974`.

   d. **Final selection (line 980):** `let provider = runtime_provider.or(registry_provider);`

   e. **Return:** If `provider` is `Some`, wrap in `ExecutionTarget` and return `Ok(...)`. If `None`, return `Err(ProviderResolutionError::NoCredentials(identity.provider_id.clone()))`.

5. Add the necessary imports at the top of `provider_resolution.rs`:
   ```rust
   use claurst_core::config::ProviderConfig;
   use std::collections::HashMap;
   ```

**Strict Constraints — Do NOT:**
- Do NOT add health checks, capability checks, or any D2 logic.
- Do NOT modify `lib.rs` yet — that is M7-04.
- Do NOT change `runtime_provider_for()` in `registry.rs`.
- Do NOT add any fallback behavior — if the provider can't be materialized, return an error.
- Do NOT add new provider aliases beyond what exists at `lib.rs:963-974`.

**Definition of Done:**
- `materialize_provider()` has a complete body handling auth-store refresh, registry lookup, api_base override, and error on missing provider.
- No `todo!()` remains in this function.
- `cargo check -p claurst-query` compiles.

**Validation Commands:**
```bash
cd src-rust && cargo check -p claurst-query
```

**Expected Output / Artifact Changes:**
- Modified: `crates/query/src/provider_resolution.rs` (function body filled in)

**Stop / Escalate Conditions:**
- If `claurst_api::registry::runtime_provider_for()` does not exist or has a different signature than at `registry.rs:73`, stop and investigate.
- If `ProviderRegistry::get()` does not return `Option<&Arc<dyn LlmProvider>>`, adjust the code accordingly.
- If `claurst_api::providers::openai_compat_providers` module does not exist or the builder pattern `.with_base_url()` is unavailable, stop and investigate.

---

### TASK-M7-04 — Wire lib.rs to call new functions

**Milestone:** 7

**Objective:** Replace the inline provider-resolution and materialization blocks in `run_query_loop()` with calls to the new functions, completing the seam extraction.

**Why This Ticket Exists:** This is the actual extraction — the inline logic is replaced with calls to the shared functions. After this ticket, the resolution path flows through `provider_resolution.rs`.

**Exact Code Targets:**

| Target | File | Lines | Action | Status |
|--------|------|-------|--------|--------|
| Inline resolution block | `crates/query/src/lib.rs` | 854–926 | Replace with `resolve_provider_identity()` call | **Confirmed** |
| `use_provider_dispatch` variable | `crates/query/src/lib.rs` | 931–932 | Remove — when registry is `Some`, always use pipeline | **Confirmed** |
| Materialization block | `crates/query/src/lib.rs` | 934–980 | Replace with `materialize_provider()` call | **Confirmed** |
| Anthropic filter | `crates/query/src/lib.rs` | 863 | Removed — `resolve_provider_identity()` handles all providers | **Confirmed** |
| Capability shaping | `crates/query/src/lib.rs` | 994–1003 | **DO NOT MOVE** — stays inline, consumes `ExecutionTarget` output | **Confirmed** |

**Preconditions:**
- TASK-M7-02 complete — `resolve_provider_identity()` implemented.
- TASK-M7-03 complete — `materialize_provider()` implemented.

**Step-by-Step Instructions:**

1. Open `crates/query/src/lib.rs`.
2. Locate the block at line 862: `if let Some(ref registry) = config.provider_registry {`.
3. Inside this `if let` block, replace lines 863–926 (the entire resolution decision tree) with:
   ```rust
   let identity = provider_resolution::resolve_provider_identity(
       tool_ctx.config.provider.as_deref(),
       &effective_model,
       config.model_registry.as_deref(),
   )?;
   ```
   **Note on `?`:** The current function may not return `Result`. If `run_query_loop()` does not return a `Result` type, you must handle the error explicitly — e.g., `match resolve_provider_identity(...) { Ok(id) => id, Err(e) => { /* emit error event, skip to next turn or return error outcome */ } }`. Study the existing error handling pattern in the loop (look for how other errors are handled nearby) and match that pattern.

4. Remove lines 931–932 (`let use_provider_dispatch = ...`).
5. Remove the `if use_provider_dispatch {` guard at line 934. The pipeline is always used when registry is `Some`.
6. Replace lines 937-980 (the materialization block) with:
   ```rust
   let target = provider_resolution::materialize_provider(
       &identity,
       registry,
       &tool_ctx.config.provider_configs,
   )?;
   ```
   Handle errors the same way as step 3.

7. After materialization, the code at line 980 currently does `let provider = runtime_provider.or(registry_provider);` followed by `if let Some(provider) = provider { ... }`. Replace this with using `target.provider` directly (it's guaranteed to be present in `ExecutionTarget`).

8. Update the debug log and status event to use `target.provider_id` and `target.model_id` instead of `provider_id_str` and `model_id_str`.

9. The capability-shaping block at lines ~994-1003 currently reads `provider_id_str` and `model_id_str`. Update these references to use `target.provider_id` and `target.model_id`.

10. **CRITICAL:** The block inside `if let Some(ref registry) = config.provider_registry` currently ends with a `continue` or falls through to the Anthropic path. After extraction, when the pipeline succeeds, it should proceed with the resolved provider. When it fails, it should return an error — NOT fall through to the Anthropic client. Verify the control flow.

11. **CRITICAL:** The `else` branch (when `config.provider_registry` is `None`) must remain unchanged — it still uses the `client: &AnthropicClient` parameter. Do not modify this path.

12. Verify that the `known_providers` array at lines 879-886 is no longer referenced from `lib.rs` (it is now in `provider_resolution.rs` as `KNOWN_PROVIDERS`). Remove the old inline array.

13. Verify it compiles: `cd src-rust && cargo check -p claurst-query`.

**Strict Constraints — Do NOT:**
- Do NOT modify the `else` branch (when registry is `None`) — the `client: &AnthropicClient` path must remain.
- Do NOT move the capability-shaping logic at lines ~994-1003 — it stays inline.
- Do NOT change the signature of `run_query_loop()`.
- Do NOT change any code below the provider dispatch block (request building, streaming, tool dispatch, etc.).
- Do NOT touch `agent_tool.rs` or `team_tool.rs` — those are M8.

**Definition of Done:**
- The inline resolution block at lib.rs:854-926 is replaced with a call to `resolve_provider_identity()`.
- The inline materialization block at lib.rs:937-977 is replaced with a call to `materialize_provider()`.
- The `use_provider_dispatch` variable is removed.
- The `"anthropic"` filter at line 863 is removed (handled by the new function).
- The `else` (no registry) branch is unchanged.
- Capability shaping remains inline.
- `cargo check -p claurst-query` compiles.

**Validation Commands:**
```bash
cd src-rust && cargo check -p claurst-query
cd src-rust && cargo check --workspace
```

**Expected Output / Artifact Changes:**
- Modified: `crates/query/src/lib.rs` (inline blocks replaced with function calls, ~70 lines removed, ~15 lines added)

**Stop / Escalate Conditions:**
- If `run_query_loop()` uses `provider_id_str` or `model_id_str` in places NOT covered by this ticket (e.g., further down in the loop body for cost tracking or logging), identify all such sites and update them. If there are more than 5 such sites, escalate — the extraction may need a different approach.
- If `tool_ctx.config.provider_configs` does not exist or has a different type than `HashMap<String, ProviderConfig>`, check the `Config` struct definition and adjust.
- If the error-handling pattern in the loop body is unclear, read 50 lines above and below the extraction site to find how other errors are handled, and match that pattern.

---

### TASK-M7-05 — Unit tests for resolve_provider_identity() (P1–P12)

**Milestone:** 7

**Objective:** Write unit tests covering all 12 rows of the precedence matrix for `resolve_provider_identity()`.

**Why This Ticket Exists:** The precedence matrix is the core correctness guarantee for D1. Each row must have a passing test that calls the function with matching inputs and asserts the expected output.

**Exact Code Targets:**

| Target | File | Status |
|--------|------|--------|
| `#[cfg(test)]` module | `crates/query/src/provider_resolution.rs` | New — bottom of file |

**Preconditions:**
- TASK-M7-02 complete — `resolve_provider_identity()` implemented.

**Step-by-Step Instructions:**

1. Open `crates/query/src/provider_resolution.rs`.
2. Add a `#[cfg(test)] mod tests { ... }` block at the bottom.
3. Write one test per precedence row. For tests that need a `ModelRegistry`, create a minimal mock. The `ModelRegistry` struct should have a `new()` method and a way to register provider-model mappings — check its API.
4. Required tests:

   **P1 — Explicit provider matches model prefix:**
   ```
   explicit_provider: Some("openai"), model: "openai/gpt-4o"
   Expected: provider="openai", model="gpt-4o", source=ExplicitProvider
   ```

   **P2 — Explicit provider, bare model:**
   ```
   explicit_provider: Some("openai"), model: "gpt-4o"
   Expected: provider="openai", model="gpt-4o", source=ExplicitProvider
   ```

   **P3 — Explicit provider conflicts with model prefix:**
   ```
   explicit_provider: Some("openai"), model: "anthropic/claude-sonnet-4-20250514"
   Expected: Err(ProviderModelConflict { provider: "openai", model: "anthropic/claude-sonnet-4-20250514", model_provider: "anthropic" })
   ```

   **P4 — No provider, model has known prefix:**
   ```
   explicit_provider: None, model: "google/gemini-2.5-flash"
   Expected: provider="google", model="gemini-2.5-flash", source=ModelStringPrefix
   ```

   **P5 — Explicit provider conflicts with model prefix (reverse of P3):**
   ```
   explicit_provider: Some("anthropic"), model: "openai/gpt-4o"
   Expected: Err(ProviderModelConflict { ... })
   ```

   **P6 — Explicit "anthropic" pin, bare model:**
   ```
   explicit_provider: Some("anthropic"), model: "claude-sonnet-4-20250514"
   Expected: provider="anthropic", model="claude-sonnet-4-20250514", source=ExplicitProvider
   ```

   **P7 — No provider, model has unknown namespace:**
   ```
   explicit_provider: None, model: "meta-llama/Llama-3.3-70B"
   Expected: provider="anthropic" (default), model="meta-llama/Llama-3.3-70B", source=Default
   ```
   (Or ModelRegistry if one is provided and resolves it.)

   **P8 — No provider, bare model, registry resolves:**
   ```
   explicit_provider: None, model: "gemini-3-flash-preview"
   model_registry: (configured to map "gemini-3-flash-preview" → "google")
   Expected: provider="google", model="gemini-3-flash-preview", source=ModelRegistry
   ```

   **P9 — No provider, bare model, no registry match:**
   ```
   explicit_provider: None, model: "some-unknown-model"
   model_registry: (configured but no match)
   Expected: provider="anthropic", model="some-unknown-model", source=Default
   ```

   **P10 — No provider, no model registry:**
   ```
   explicit_provider: None, model: "claude-sonnet-4-20250514"
   model_registry: None
   Expected: provider="anthropic", model="claude-sonnet-4-20250514", source=Default
   ```

   **P11 — Explicit provider, model has nested slash (OpenRouter style):**
   ```
   explicit_provider: Some("openrouter"), model: "openrouter/meta-llama/Llama-3.3-70B"
   Expected: provider="openrouter", model="meta-llama/Llama-3.3-70B", source=ExplicitProvider
   ```

   **P12 — Local provider (ollama), bare model:**
   ```
   explicit_provider: Some("ollama"), model: "llama3"
   Expected: provider="ollama", model="llama3", source=ExplicitProvider
   ```

5. Each test should call `resolve_provider_identity(explicit_provider, model, model_registry.as_ref())` and assert on the result's `provider_id`, `model_id`, and `resolution_source`.

**Strict Constraints — Do NOT:**
- Do NOT add integration tests that require network access — these are pure unit tests.
- Do NOT create mock providers — `resolve_provider_identity()` does not use providers, only `ModelRegistry`.
- Do NOT modify any production code in this ticket.
- Do NOT test `materialize_provider()` — that is M7-06.

**Definition of Done:**
- 12 unit tests exist in `provider_resolution.rs` `#[cfg(test)]` module.
- All 12 tests pass: `cd src-rust && cargo test -p claurst-query -- provider_resolution`.

**Validation Commands:**
```bash
cd src-rust && cargo test -p claurst-query -- provider_resolution
```

**Expected Output / Artifact Changes:**
- Modified: `crates/query/src/provider_resolution.rs` (test module added)

**Stop / Escalate Conditions:**
- If `ModelRegistry` does not have a `new()` constructor or a way to register test entries, investigate its API. If creating a test-suitable `ModelRegistry` requires complex setup, use `None` for tests that don't need it and document the limitation.
- If any test fails because the function behavior doesn't match the expected output, do NOT change the expected output — investigate whether the function implementation (M7-02) is correct.

---

### TASK-M7-06 — Unit tests for materialize_provider()

**Milestone:** 7

**Objective:** Write unit tests covering materialization: auth-store refresh, api_base overrides, and missing-credentials error.

**Why This Ticket Exists:** `materialize_provider()` has side effects (auth store reads, provider construction) that must be tested. These tests verify the materialization pipeline works independently of the resolution step.

**Exact Code Targets:**

| Target | File | Status |
|--------|------|--------|
| `#[cfg(test)]` module | `crates/query/src/provider_resolution.rs` | Extend from M7-05 |

**Preconditions:**
- TASK-M7-03 complete — `materialize_provider()` implemented.
- TASK-M7-05 complete — test module exists.

**Step-by-Step Instructions:**

1. Open `crates/query/src/provider_resolution.rs`, locate the `#[cfg(test)]` module.
2. Add tests for `materialize_provider()`. These tests need a `ProviderRegistry` with mock providers. Check whether `ProviderRegistry` has a builder or `insert` method. If not, you may need to construct one via its public API.
3. Required tests:

   **Test: Registry provider found:**
   - Create a `ProviderIdentity` with `provider_id: "openai"`.
   - Create a `ProviderRegistry` containing an OpenAI provider.
   - Call `materialize_provider()` with empty `provider_configs`.
   - Assert `Ok(target)` with `target.provider_id == "openai"`.

   **Test: No credentials (empty registry, no auth store):**
   - Create a `ProviderIdentity` with `provider_id: "some-fake-provider"`.
   - Create an empty `ProviderRegistry`.
   - Call `materialize_provider()`.
   - Assert `Err(ProviderResolutionError::NoCredentials("some-fake-provider"))`.

   **Test: api_base override for ollama:**
   - Create a `ProviderIdentity` with `provider_id: "ollama"`.
   - Create `provider_configs` with an entry for `"ollama"` containing `api_base: Some("http://custom:11434")`.
   - Call `materialize_provider()`.
   - Assert the returned provider is constructed (does not error).
   - Verifying the actual base URL is hard without inspecting internals — at minimum verify the call succeeds.

4. If constructing a `ProviderRegistry` with mock entries is too complex (requires real API keys or network), mark those tests as `#[ignore]` with a comment explaining why, and write what you can test.

**Strict Constraints — Do NOT:**
- Do NOT modify production code.
- Do NOT add network-dependent tests without `#[ignore]`.
- Do NOT test `resolve_provider_identity()` here — that was M7-05.

**Definition of Done:**
- At least 2 materialize tests exist (happy path + error path).
- Tests pass or are `#[ignore]`-gated with clear explanation.

**Validation Commands:**
```bash
cd src-rust && cargo test -p claurst-query -- provider_resolution
```

**Expected Output / Artifact Changes:**
- Modified: `crates/query/src/provider_resolution.rs` (tests added to existing test module)

**Stop / Escalate Conditions:**
- If `ProviderRegistry` cannot be constructed in tests without real API keys, mark all materialize tests as `#[ignore]` and document the limitation. Do not invent a mock ProviderRegistry trait or wrapper.

---

### TASK-M7-07 — Workspace validation

**Milestone:** 7

**Objective:** Verify the entire workspace compiles, all tests pass, and clippy is clean after M7 changes.

**Why This Ticket Exists:** Final validation gate for Milestone 7. Ensures the extraction introduced no regressions.

**Exact Code Targets:** None — this is a validation-only ticket.

**Preconditions:**
- All M7 tickets (M7-01 through M7-06) complete.

**Step-by-Step Instructions:**

1. Run `cd src-rust && cargo build --workspace`.
2. Run `cd src-rust && cargo test --workspace`.
3. Run `cd src-rust && cargo clippy --workspace --all-targets`.
4. If any failures occur:
   - Read the error messages.
   - If the error is in `provider_resolution.rs` or the modified section of `lib.rs`, fix it.
   - If the error is in an unrelated file, do NOT fix it — report it.
   - If a pre-existing test fails, investigate whether the M7 changes caused the failure. If yes, fix. If no, report.
5. Run `cd src-rust && cargo fmt --all -- --check` and fix any formatting issues.

**Strict Constraints — Do NOT:**
- Do NOT fix pre-existing clippy warnings in unrelated files.
- Do NOT modify any file that was not already modified in M7-01 through M7-06.
- Do NOT add new features or "clean up" surrounding code.

**Definition of Done:**
- `cargo build --workspace` succeeds.
- `cargo test --workspace` passes (all tests, including pre-existing ones).
- `cargo clippy --workspace --all-targets` has no new warnings from M7 code.
- `cargo fmt --all -- --check` passes.

**Validation Commands:**
```bash
cd src-rust && cargo fmt --all -- --check
cd src-rust && cargo build --workspace
cd src-rust && cargo test --workspace
cd src-rust && cargo clippy --workspace --all-targets
```

**Expected Output / Artifact Changes:**
- Possibly modified: any M7 files if fixups were needed.

**Stop / Escalate Conditions:**
- If `cargo test --workspace` fails on a test that is NOT in `provider_resolution.rs` and was NOT modified by M7, stop and investigate whether the extraction changed observable behavior.
- If more than 3 files need fixes, escalate — the extraction may have introduced unintended side effects.

---

## 4. Milestone 8 Detailed Tickets

---

### TASK-M8-01 — Add provider_registry and model_registry to ToolContext

**Milestone:** 8

**Objective:** Extend `ToolContext` with optional provider and model registry fields so tools (especially AgentTool and TeamCreate) can access the parent session's registries.

**Why This Ticket Exists:** Workers need access to the parent's provider/model registries to resolve their own providers via the shared seam. `ToolContext` is the shared context passed to every tool invocation.

**Exact Code Targets:**

| Target | File | Lines | Status |
|--------|------|-------|--------|
| `ToolContext` struct | `crates/tools/src/lib.rs` | 209–223 | **Confirmed** |

**Preconditions:**
- Milestone 7 complete.
- **Verification:** `claurst-tools` already depends on `claurst-api` (confirmed in Cargo.toml). No new dependency needed.

**Step-by-Step Instructions:**

1. Open `crates/tools/src/lib.rs`.
2. Locate the `ToolContext` struct at line 209.
3. Add two new fields after the `config` field (line 222):
   ```rust
   /// Optional provider registry for multi-provider dispatch.
   /// Populated by the CLI from the root session's registry.
   pub provider_registry: Option<Arc<claurst_api::ProviderRegistry>>,
   /// Optional model registry for dynamic model→provider resolution.
   pub model_registry: Option<Arc<claurst_api::ModelRegistry>>,
   ```
4. Add `use std::sync::Arc;` at the top of the file if not already present.
5. Search the entire codebase for all sites that construct a `ToolContext`. These include:
   - `crates/cli/src/main.rs` line ~647 (M8-02 will handle this)
   - Any test files that create `ToolContext`
6. For ALL construction sites found (except `main.rs`), add the new fields as `None`:
   ```rust
   provider_registry: None,
   model_registry: None,
   ```
7. Verify it compiles: `cd src-rust && cargo check --workspace`.

**Strict Constraints — Do NOT:**
- Do NOT add `session_budget`, `health_cache`, or any D2 fields — those are M11.
- Do NOT change any existing fields on `ToolContext`.
- Do NOT add any methods to `ToolContext` for provider resolution — that logic lives in `provider_resolution.rs`.
- Do NOT make the fields non-optional (they must be `Option<...>`).

**Definition of Done:**
- `ToolContext` has `provider_registry: Option<Arc<ProviderRegistry>>` and `model_registry: Option<Arc<ModelRegistry>>`.
- All existing `ToolContext` construction sites compile with the new fields set to `None`.
- `cargo check --workspace` succeeds.

**Validation Commands:**
```bash
cd src-rust && cargo check --workspace
```

**Expected Output / Artifact Changes:**
- Modified: `crates/tools/src/lib.rs`
- Modified: any test file or helper that constructs `ToolContext` (add `None` fields)

**Stop / Escalate Conditions:**
- If there are more than 10 `ToolContext` construction sites, escalate — consider whether a builder pattern or `Default` impl should be used instead.
- If `Arc` is not imported and adding it creates conflicts, investigate.

---

### TASK-M8-02 — Populate ToolContext fields in main.rs

**Milestone:** 8

**Objective:** Wire the root session's `ProviderRegistry` and `ModelRegistry` into the `ToolContext` constructed at startup.

**Why This Ticket Exists:** The CLI `main()` function builds the `ToolContext`. It must pass the root session's registries so that tools can access them.

**Exact Code Targets:**

| Target | File | Lines | Status |
|--------|------|-------|--------|
| `ToolContext` construction | `crates/cli/src/main.rs` | ~647–658 | **Confirmed** |
| `provider_registry` and `model_registry` source | `crates/cli/src/main.rs` | search for where these are built at startup | **Requires confirmation** |

**Preconditions:**
- TASK-M8-01 complete — `ToolContext` has the new fields.

**Step-by-Step Instructions:**

1. Open `crates/cli/src/main.rs`.
2. **First, locate where `provider_registry` is built at startup.** Search for `ProviderRegistry::` or `provider_registry` in main.rs. It is likely built earlier in `main()` and stored in a variable or passed to `QueryConfig`. Identify the exact variable name and type.
3. **Locate where `model_registry` is built at startup.** Same search for `ModelRegistry::` or `model_registry`.
4. Go to the `ToolContext` construction at line ~647.
5. Add the two new fields:
   ```rust
   provider_registry: provider_registry_arc.clone(),  // or whatever the variable name is
   model_registry: model_registry_arc.clone(),
   ```
   If the registries are `Arc<...>` already, clone the Arc. If they are bare values, wrap in `Arc::new(...)` first. If they are `Option<Arc<...>>`, pass them directly.
6. Verify it compiles.

**Strict Constraints — Do NOT:**
- Do NOT create new ProviderRegistry or ModelRegistry instances — use the ones already built at startup.
- Do NOT modify how the registries are built.
- Do NOT modify the QueryConfig construction.
- Do NOT change any other field in the ToolContext construction.

**Definition of Done:**
- `ToolContext` construction in `main.rs` populates `provider_registry` and `model_registry` with the root session's registries.
- `cargo check --workspace` succeeds.

**Validation Commands:**
```bash
cd src-rust && cargo check --workspace
```

**Expected Output / Artifact Changes:**
- Modified: `crates/cli/src/main.rs` (2 lines added to ToolContext construction)

**Stop / Escalate Conditions:**
- If `provider_registry` or `model_registry` are not built before the `ToolContext` construction, stop and trace the startup flow. They may be built later or conditionally. Report the finding.
- If wrapping in `Arc` creates ownership issues, investigate.

---

### TASK-M8-03 — Add provider field to AgentInput and schema

**Milestone:** 8

**Objective:** Allow callers to specify an explicit provider when spawning an agent via the AgentTool.

**Why This Ticket Exists:** Agents must be able to pin to a specific provider (e.g., `"openai"`) rather than always inheriting. The `provider` field enables this.

**Exact Code Targets:**

| Target | File | Lines | Status |
|--------|------|-------|--------|
| `AgentInput` struct | `crates/query/src/agent_tool.rs` | 130–155 | **Confirmed** |
| `AgentTool::input_schema()` | `crates/query/src/agent_tool.rs` | in `impl Tool for AgentTool` | **Confirmed** |

**Preconditions:**
- None within M8 (can proceed in parallel with M8-01/M8-02).

**Step-by-Step Instructions:**

1. Open `crates/query/src/agent_tool.rs`.
2. Locate the `AgentInput` struct at line 130.
3. Add a new field after `model`:
   ```rust
   /// Optional: explicit provider override (e.g., "openai", "google").
   #[serde(default)]
   provider: Option<String>,
   ```
4. Locate the `input_schema()` method in the `impl Tool for AgentTool` block.
5. Add `"provider"` to the JSON schema properties:
   ```rust
   "provider": {
       "type": "string",
       "description": "Explicit provider to use for this agent (e.g., 'openai', 'google'). When omitted, inherits from parent session."
   },
   ```
6. Verify it compiles: `cd src-rust && cargo check -p claurst-query`.

**Strict Constraints — Do NOT:**
- Do NOT add `allow_fallback`, `budget_usd`, or any D2 fields.
- Do NOT modify `AgentTool::execute()` yet — that is M8-04.
- Do NOT change any existing fields on `AgentInput`.

**Definition of Done:**
- `AgentInput` has `provider: Option<String>`.
- `input_schema()` includes `"provider"` in the JSON schema.
- `cargo check -p claurst-query` compiles.

**Validation Commands:**
```bash
cd src-rust && cargo check -p claurst-query
```

**Expected Output / Artifact Changes:**
- Modified: `crates/query/src/agent_tool.rs` (struct field + schema property)

**Stop / Escalate Conditions:**
- None expected. This is a straightforward field addition.

---

### TASK-M8-04 — Replace foreground agent AnthropicClient with shared seam

**Milestone:** 8

**Objective:** Replace the hardcoded `ANTHROPIC_API_KEY` + `AnthropicClient::new()` in the foreground agent path with `resolve_provider_identity()` + `materialize_provider()`.

**Why This Ticket Exists:** The foreground agent path (lines 229-248) currently hardcodes Anthropic. After this ticket, it uses the shared resolution seam and inherits the parent's provider.

**Exact Code Targets:**

| Target | File | Lines | Status |
|--------|------|-------|--------|
| API key resolution | `crates/query/src/agent_tool.rs` | 229–240 | **Confirmed** |
| Client creation | `crates/query/src/agent_tool.rs` | 242–248 | **Confirmed** |
| Model resolution | `crates/query/src/agent_tool.rs` | 264–268 | **Confirmed** |
| QueryConfig construction | `crates/query/src/agent_tool.rs` | 339–360 | **Confirmed** |

**Preconditions:**
- TASK-M8-01 complete — `ToolContext` has registry fields.
- TASK-M8-03 complete — `AgentInput` has `provider` field.
- Milestone 7 complete — `resolve_provider_identity()` and `materialize_provider()` exist.

**Step-by-Step Instructions:**

1. Open `crates/query/src/agent_tool.rs`.
2. At the top, add imports:
   ```rust
   use crate::provider_resolution::{resolve_provider_identity, materialize_provider};
   ```
3. Locate the `execute()` method in `impl Tool for AgentTool`.
4. **Replace lines 229-248** (API key + client creation) with provider resolution:
   ```rust
   // Require provider_registry in ToolContext — invariant for worker paths.
   let registry = match ctx.provider_registry.as_ref() {
       Some(r) => r,
       None => return ToolResult::error(
           "Cannot spawn sub-agent: provider_registry not available in ToolContext".to_string()
       ),
   };

   // Resolve provider: explicit param > model prefix > parent inheritance > default.
   let identity = match resolve_provider_identity(
       params.provider.as_deref(),
       &model,   // model resolved at line 264-268
       ctx.model_registry.as_deref(),
   ) {
       Ok(id) => id,
       Err(e) => return ToolResult::error(format!("Provider resolution failed: {}", e)),
   };

   let target = match materialize_provider(
       &identity,
       registry,
       &ctx.config.provider_configs,
   ) {
       Ok(t) => t,
       Err(e) => return ToolResult::error(format!("Provider materialization failed: {}", e)),
   };
   ```
   **Note:** The model resolution at lines 264-268 must happen BEFORE provider resolution (since `resolve_provider_identity` takes the model as input). Reorder if necessary: resolve model first, then resolve provider.

5. **For the `run_query_loop()` call:** The function still requires `client: &AnthropicClient` as its first parameter. When the resolved provider is Anthropic, construct an `AnthropicClient` from the resolved credentials. When non-Anthropic, construct a dummy/empty `AnthropicClient` (it will be unused because `provider_registry: Some(...)` in `QueryConfig` causes the registry path to be taken). Study how the current code constructs the client and adapt.

6. **Update QueryConfig at lines 339-360:**
   - Change `provider_registry: None` at line 356 to `provider_registry: Some(registry.clone())`.
   - Change `model_registry: None` at line 359 to `model_registry: ctx.model_registry.clone()`.
   - Set `model` to `target.model_id` (the resolved model).

7. Verify it compiles.

**Strict Constraints — Do NOT:**
- Do NOT remove the `client` parameter from `run_query_loop()` — its removal is deferred (OQ1).
- Do NOT add fallback behavior — if resolution fails, return an error.
- Do NOT modify the background agent path — that is M8-05.
- Do NOT modify `init_team_swarm_runner()` — that is M8-08.
- Do NOT change the tool list construction or system prompt logic.

**Definition of Done:**
- Foreground agent no longer reads `ANTHROPIC_API_KEY` or calls `AnthropicClient::new()` directly.
- Foreground agent uses `resolve_provider_identity()` + `materialize_provider()`.
- Child `QueryConfig` has `provider_registry: Some(...)` and `model_registry: ctx.model_registry.clone()`.
- `cargo check -p claurst-query` compiles.

**Validation Commands:**
```bash
cd src-rust && cargo check -p claurst-query
```

**Expected Output / Artifact Changes:**
- Modified: `crates/query/src/agent_tool.rs` (foreground agent path rewritten)

**Stop / Escalate Conditions:**
- If `run_query_loop()` cannot accept a dummy `AnthropicClient` when the provider is non-Anthropic (e.g., it validates the API key), investigate the function's behavior when `provider_registry` is `Some`. The registry path should bypass the client entirely.
- If model resolution at lines 264-268 has complex interactions with the provider resolution, trace the full flow before making changes.

---

### TASK-M8-05 — Replace background agent AnthropicClient with shared seam

**Milestone:** 8

**Objective:** Apply the same provider-resolution changes to the background agent path as done for foreground in M8-04.

**Why This Ticket Exists:** The background agent path (inside the `if params.run_in_background` block) has its own client creation that also hardcodes Anthropic. It must use the shared seam too.

**Exact Code Targets:**

| Target | File | Lines | Status |
|--------|------|-------|--------|
| Background agent block | `crates/query/src/agent_tool.rs` | 362–420 | **Confirmed** — the `if params.run_in_background { ... }` block |
| Background client clone | `crates/query/src/agent_tool.rs` | 374 | **Confirmed** — `let client_bg = client.clone();` |
| Background QueryConfig | `crates/query/src/agent_tool.rs` | 376 | **Confirmed** — `let config_bg = query_config.clone();` |

**Preconditions:**
- TASK-M8-04 complete — foreground path already updated; the resolved `target` and updated `query_config` from M8-04 are available.

**Step-by-Step Instructions:**

1. Open `crates/query/src/agent_tool.rs`.
2. Locate the `if params.run_in_background {` block at ~line 364.
3. The background path currently clones the client at line 374 (`let client_bg = client.clone()`). Since M8-04 already changed how `client` is created (or replaced it with a resolved provider), verify that the background path uses the same resolved client/config.
4. The background path clones `query_config` at line 376. Since M8-04 updated `query_config` to have `provider_registry: Some(...)`, the clone should automatically carry the registry. Verify this.
5. If the background path has any ADDITIONAL `AnthropicClient::new()` calls or `ANTHROPIC_API_KEY` reads that were not already removed by M8-04's changes to the shared code path, remove them and use the resolved provider.
6. Verify that the background path's tool list construction still works (it re-creates tools at line 369 — this should not need changes).

**Strict Constraints — Do NOT:**
- Do NOT change the `tokio::spawn` structure.
- Do NOT modify the worktree cleanup logic.
- Do NOT add new fields to the background closure capture beyond what's needed.

**Definition of Done:**
- Background agent path uses the provider-aware client/config from M8-04.
- No `ANTHROPIC_API_KEY` reads or `AnthropicClient::new()` calls remain in the background path.
- `cargo check -p claurst-query` compiles.

**Validation Commands:**
```bash
cd src-rust && cargo check -p claurst-query
```

**Expected Output / Artifact Changes:**
- Modified: `crates/query/src/agent_tool.rs` (background agent block updated)

**Stop / Escalate Conditions:**
- If the background block has a fundamentally different client-creation path from the foreground block, investigate before making changes.

---

### TASK-M8-06 — Migrate AgentRunFn to AgentRunParams

**Milestone:** 8

**Objective:** Replace the 6-positional-parameter `AgentRunFn` type alias with a struct-based `AgentRunParams` parameter.

**Why This Ticket Exists:** The `AgentRunFn` at `team_tool.rs:47-58` takes 6 positional parameters. Adding `provider_override` and `model_override` would make it 8, which is unmaintainable. A struct is cleaner and extensible.

**Exact Code Targets:**

| Target | File | Lines | Status |
|--------|------|-------|--------|
| `AgentRunFn` type alias | `crates/tools/src/team_tool.rs` | 47–58 | **Confirmed** |
| `run_agent()` function | `crates/tools/src/team_tool.rs` | 76–89 | **Confirmed** |
| `register_agent_runner()` | `crates/tools/src/team_tool.rs` | 66–70 | **Confirmed** |
| `AGENT_RUNNER` static | `crates/tools/src/team_tool.rs` | 60 | **Confirmed** |

**Preconditions:**
- None within M8 (can proceed in parallel with M8-01 through M8-05, but M8-08 and M8-09 depend on this).

**Step-by-Step Instructions:**

1. Open `crates/tools/src/team_tool.rs`.
2. Before the `AgentRunFn` type alias, add a new struct:
   ```rust
   /// Parameters for running a sub-agent via the registered runner.
   pub struct AgentRunParams {
       pub description: String,
       pub prompt: String,
       pub tools: Option<Vec<String>>,
       pub system_prompt: Option<String>,
       pub max_turns: Option<u32>,
       pub ctx: Arc<ToolContext>,
       pub provider_override: Option<String>,
       pub model_override: Option<String>,
   }
   ```
3. Replace the `AgentRunFn` type alias (lines 47-58) with:
   ```rust
   pub type AgentRunFn = Arc<
       dyn Fn(AgentRunParams) -> Pin<Box<dyn Future<Output = String> + Send>>
           + Send
           + Sync,
   >;
   ```
4. Update `run_agent()` (lines 76-89) to accept `AgentRunParams`:
   ```rust
   async fn run_agent(params: AgentRunParams) -> String {
       if let Some(runner) = AGENT_RUNNER.get() {
           runner(params).await
       } else {
           "[No agent runner registered — cc-query not initialised]".to_string()
       }
   }
   ```
5. Find all call sites of `run_agent()` within `team_tool.rs`. There is one at ~line 416-423 inside `TeamCreateTool::execute()`. Update it to pass an `AgentRunParams` struct:
   ```rust
   run_agent(AgentRunParams {
       description,
       prompt: agent_task,
       tools,
       system_prompt: Some(system_prompt),
       max_turns: Some(10),
       ctx: ctx_inner,
       provider_override: None,  // will be wired in M8-09
       model_override: None,     // will be wired in M8-09
   })
   ```
6. The `init_team_swarm_runner()` in `agent_tool.rs` is the producer of `AgentRunFn`. It must be updated in M8-08 to match the new signature. For now, just ensure `team_tool.rs` compiles with the new types.
7. `cargo check -p claurst-tools` will fail until M8-08 updates the producer. That's expected. Verify that `team_tool.rs` itself has no internal errors.

**Strict Constraints — Do NOT:**
- Do NOT add D2 fields (`allow_fallback`, `budget_usd`) to `AgentRunParams` — those are M11.
- Do NOT change `register_agent_runner()` or `AGENT_RUNNER` beyond the type change.
- Do NOT modify `init_team_swarm_runner()` — that is M8-08.

**Definition of Done:**
- `AgentRunParams` struct exists with all 8 fields.
- `AgentRunFn` type alias uses `AgentRunParams`.
- `run_agent()` accepts `AgentRunParams`.
- Call site in `TeamCreateTool::execute()` passes `AgentRunParams` (with `provider_override: None`, `model_override: None`).

**Validation Commands:**
```bash
cd src-rust && cargo check -p claurst-tools  # may fail until M8-08
```

**Expected Output / Artifact Changes:**
- Modified: `crates/tools/src/team_tool.rs` (type alias, struct, function, call site)

**Stop / Escalate Conditions:**
- If `run_agent()` is called from more places than expected within `team_tool.rs`, update all call sites.
- If the `Pin<Box<dyn Future<...>>>` return type needs adjustment for the new signature, adapt accordingly.

---

### TASK-M8-07 — Add provider/model to AgentSpec and TeamCreate schema

**Milestone:** 8

**Objective:** Allow TeamCreate callers to specify per-agent provider and model overrides.

**Why This Ticket Exists:** TeamCreate agents must be able to use different providers. The `AgentSpec` and input schema must expose these fields.

**Exact Code Targets:**

| Target | File | Lines | Status |
|--------|------|-------|--------|
| `AgentSpec` struct | `crates/tools/src/team_tool.rs` | 172–182 | **Confirmed** |
| `TeamCreateTool::input_schema()` | `crates/tools/src/team_tool.rs` | 223–260 | **Confirmed** |

**Preconditions:**
- None within M8 (can proceed in parallel).

**Step-by-Step Instructions:**

1. Open `crates/tools/src/team_tool.rs`.
2. Locate `AgentSpec` at line 172.
3. Add two new fields:
   ```rust
   /// Optional provider override for this agent.
   #[serde(default)]
   provider: Option<String>,
   /// Optional model override for this agent.
   #[serde(default)]
   model: Option<String>,
   ```
4. Locate `input_schema()` in `impl Tool for TeamCreateTool`.
5. In the `"agents"` → `"items"` → `"properties"` section, add:
   ```rust
   "provider": {
       "type": "string",
       "description": "Explicit provider for this agent (e.g., 'openai'). Inherits from parent if omitted."
   },
   "model": {
       "type": "string",
       "description": "Model override for this agent. Inherits from parent if omitted."
   },
   ```
6. Verify it compiles: `cd src-rust && cargo check -p claurst-tools`.

**Strict Constraints — Do NOT:**
- Do NOT add `allow_fallback` or `budget_usd` — those are M11.
- Do NOT modify `TeamCreateTool::execute()` — that is M8-09.
- Do NOT change existing fields on `AgentSpec`.

**Definition of Done:**
- `AgentSpec` has `provider: Option<String>` and `model: Option<String>`.
- Schema includes both fields.
- `cargo check -p claurst-tools` compiles.

**Validation Commands:**
```bash
cd src-rust && cargo check -p claurst-tools
```

**Expected Output / Artifact Changes:**
- Modified: `crates/tools/src/team_tool.rs` (struct fields + schema properties)

**Stop / Escalate Conditions:**
- None expected.

---

### TASK-M8-08 — Replace init_team_swarm_runner() AnthropicClient with shared seam

**Milestone:** 8

**Objective:** Update `init_team_swarm_runner()` to accept `AgentRunParams` and use `resolve_provider_identity()` + `materialize_provider()` instead of hardcoded `ANTHROPIC_API_KEY` + `AnthropicClient::new()`.

**Why This Ticket Exists:** `init_team_swarm_runner()` at `agent_tool.rs:517-607` is the producer of the `AgentRunFn` closure. It currently hardcodes Anthropic. It must use the shared seam and respect `AgentRunParams.provider_override` and `model_override`.

**Exact Code Targets:**

| Target | File | Lines | Status |
|--------|------|-------|--------|
| `init_team_swarm_runner()` | `crates/query/src/agent_tool.rs` | 517–607 | **Confirmed** |
| ANTHROPIC_API_KEY check | `crates/query/src/agent_tool.rs` | 528–539 | **Confirmed** |
| Client creation | `crates/query/src/agent_tool.rs` | 541–552 | **Confirmed** |
| QueryConfig construction | `crates/query/src/agent_tool.rs` | 575–584 | **Confirmed** |

**Preconditions:**
- TASK-M8-06 complete — `AgentRunFn` expects `AgentRunParams`.
- Milestone 7 complete — resolution functions exist.

**Step-by-Step Instructions:**

1. Open `crates/query/src/agent_tool.rs`.
2. Locate `init_team_swarm_runner()` at line 517.
3. The closure currently takes 6 positional parameters. Update it to take a single `AgentRunParams`:
   ```rust
   let runner: claurst_tools::AgentRunFn = Arc::new(
       |params: claurst_tools::AgentRunParams| {
           Box::pin(async move {
               // ... implementation
           }) as Pin<Box<dyn std::future::Future<Output = String> + Send>>
       },
   );
   ```
4. Inside the closure:
   a. **Replace lines 528-552** (API key + client creation) with:
   ```rust
   let registry = match params.ctx.provider_registry.as_ref() {
       Some(r) => r,
       None => return format!(
           "[Agent '{}' failed: provider_registry not available in ToolContext]",
           params.description
       ),
   };

   let model = params.model_override
       .unwrap_or_else(|| claurst_core::constants::DEFAULT_MODEL.to_string());

   let identity = match crate::provider_resolution::resolve_provider_identity(
       params.provider_override.as_deref(),
       &model,
       params.ctx.model_registry.as_deref(),
   ) {
       Ok(id) => id,
       Err(e) => return format!(
           "[Agent '{}' provider resolution failed: {}]",
           params.description, e
       ),
   };

   let target = match crate::provider_resolution::materialize_provider(
       &identity,
       registry,
       &params.ctx.config.provider_configs,
   ) {
       Ok(t) => t,
       Err(e) => return format!(
           "[Agent '{}' provider materialization failed: {}]",
           params.description, e
       ),
   };
   ```
   b. For the `AnthropicClient` parameter of `run_query_loop()`: same approach as M8-04 — construct a client from resolved credentials if Anthropic, or a dummy client if non-Anthropic (the registry path will be used).

   c. **Update QueryConfig** (lines 575-584):
   ```rust
   let query_config = crate::QueryConfig {
       model: target.model_id,
       // ... other fields unchanged ...
       provider_registry: Some(registry.clone()),
       model_registry: params.ctx.model_registry.clone(),
       ..Default::default()
   };
   ```

5. Update variable references throughout the closure body: use `params.description`, `params.prompt`, `params.tools`, `params.system_prompt`, `params.max_turns`, `params.ctx` instead of the old positional names.

6. Verify it compiles: `cd src-rust && cargo check -p claurst-query`.

**Strict Constraints — Do NOT:**
- Do NOT remove the `client` parameter from `run_query_loop()`.
- Do NOT add fallback behavior.
- Do NOT modify `register_agent_runner()` beyond what the type change requires.
- Do NOT change the `run_query_loop()` call structure beyond passing the new config/client.

**Definition of Done:**
- `init_team_swarm_runner()` closure accepts `AgentRunParams`.
- No `ANTHROPIC_API_KEY` reads or `AnthropicClient::new()` calls remain.
- Child `QueryConfig` propagates `provider_registry` and `model_registry`.
- `cargo check -p claurst-query` compiles (combined with M8-06 changes).

**Validation Commands:**
```bash
cd src-rust && cargo check --workspace
```

**Expected Output / Artifact Changes:**
- Modified: `crates/query/src/agent_tool.rs` (init_team_swarm_runner rewritten)

**Stop / Escalate Conditions:**
- If the closure capture becomes too complex for the `Pin<Box<...>>` return type (lifetime or Send issues), investigate whether the closure needs to be restructured.
- If `run_query_loop()` validates the `AnthropicClient` API key on entry even when `provider_registry` is `Some`, trace the function's entry path.

---

### TASK-M8-09 — Wire TeamCreateTool::execute() to pass provider/model through AgentRunParams

**Milestone:** 8

**Objective:** Connect the `AgentSpec.provider` and `AgentSpec.model` fields to the `AgentRunParams` when spawning agents in `TeamCreateTool::execute()`.

**Why This Ticket Exists:** M8-07 added the fields to `AgentSpec`. M8-06 added the fields to `AgentRunParams`. This ticket connects them.

**Exact Code Targets:**

| Target | File | Lines | Status |
|--------|------|-------|--------|
| Agent future construction | `crates/tools/src/team_tool.rs` | 382–430 | **Confirmed** |
| `run_agent()` call | `crates/tools/src/team_tool.rs` | 416–423 | **Confirmed** |

**Preconditions:**
- TASK-M8-06 complete — `AgentRunParams` has `provider_override` and `model_override`.
- TASK-M8-07 complete — `AgentSpec` has `provider` and `model`.

**Step-by-Step Instructions:**

1. Open `crates/tools/src/team_tool.rs`.
2. Locate the agent future construction loop at ~line 382.
3. In the closure that captures per-agent data, also capture `spec.provider.clone()` and `spec.model.clone()`.
4. In the `run_agent(AgentRunParams { ... })` call (updated in M8-06), change:
   ```rust
   provider_override: None,
   model_override: None,
   ```
   to:
   ```rust
   provider_override: provider_override,
   model_override: model_override,
   ```
   where `provider_override` and `model_override` are the captured values from the spec.
5. Verify it compiles.

**Strict Constraints — Do NOT:**
- Do NOT add D2 fields.
- Do NOT change the parallel/sequential execution logic.
- Do NOT modify the cancellation token logic.

**Definition of Done:**
- `TeamCreateTool::execute()` passes `spec.provider` and `spec.model` through to `AgentRunParams`.
- `cargo check -p claurst-tools` compiles.

**Validation Commands:**
```bash
cd src-rust && cargo check -p claurst-tools
```

**Expected Output / Artifact Changes:**
- Modified: `crates/tools/src/team_tool.rs` (closure captures + AgentRunParams fields)

**Stop / Escalate Conditions:**
- None expected.

---

### TASK-M8-10 — Unit tests for worker provider resolution

**Milestone:** 8

**Objective:** Write tests verifying that the worker paths (AgentTool, init_team_swarm_runner) correctly resolve providers.

**Why This Ticket Exists:** The worker provider resolution is the core M8 deliverable. Tests must verify: explicit provider works, parent inheritance works, missing registry fails loudly.

**Exact Code Targets:**

| Target | File | Status |
|--------|------|--------|
| Tests | `crates/query/src/agent_tool.rs` `#[cfg(test)]` or `crates/query/tests/` | New |

**Preconditions:**
- TASK-M8-04, M8-05, M8-08 complete.

**Step-by-Step Instructions:**

1. Determine the best location for tests. If `agent_tool.rs` already has a `#[cfg(test)]` module, add tests there. Otherwise, create one.
2. These tests may be difficult to write as true integration tests (they need a full `ToolContext` with registries). Write what is feasible:

   **Test: Missing provider_registry in ToolContext → error:**
   - Construct a `ToolContext` with `provider_registry: None`.
   - Call `AgentTool::execute()` with valid input.
   - Assert the result is an error containing "provider_registry not available".

   **Test: Explicit provider in AgentInput:**
   - If a mock registry can be constructed, test that `provider: "openai"` in the input causes resolution to `"openai"`.
   - If mocking is too complex, write the test structure with `#[ignore]` and document why.

   **Test: Parent inheritance (no provider/model specified):**
   - If feasible, verify that when `AgentInput.provider` is `None` and `AgentInput.model` is `None`, the resolution defaults to the parent's provider.

3. Mark network-dependent tests as `#[ignore]`.

**Strict Constraints — Do NOT:**
- Do NOT invent mock frameworks or test utilities beyond what's needed.
- Do NOT test TeamCreate execution here — focus on AgentTool.

**Definition of Done:**
- At least the "missing registry → error" test exists and passes.
- Other tests exist even if `#[ignore]`-gated.

**Validation Commands:**
```bash
cd src-rust && cargo test -p claurst-query -- agent_tool
```

**Expected Output / Artifact Changes:**
- Modified: `crates/query/src/agent_tool.rs` (test module added/extended)

**Stop / Escalate Conditions:**
- If constructing a `ToolContext` in tests requires too many dependencies, focus on the "missing registry" test and document the limitation.

---

### TASK-M8-11 — Workspace validation

**Milestone:** 8

**Objective:** Verify the entire workspace compiles, all tests pass, and clippy is clean after M8 changes.

**Why This Ticket Exists:** Final validation gate for Milestone 8.

**Exact Code Targets:** None — validation only.

**Preconditions:**
- All M8 tickets (M8-01 through M8-10) complete.

**Step-by-Step Instructions:**

1. Run `cd src-rust && cargo fmt --all -- --check` and fix formatting.
2. Run `cd src-rust && cargo build --workspace`.
3. Run `cd src-rust && cargo test --workspace`.
4. Run `cd src-rust && cargo clippy --workspace --all-targets`.
5. Same rules as M7-07: fix M8-related failures, report unrelated ones.

**Strict Constraints — Do NOT:**
- Same as M7-07.

**Definition of Done:**
- All four validation commands pass.

**Validation Commands:**
```bash
cd src-rust && cargo fmt --all -- --check
cd src-rust && cargo build --workspace
cd src-rust && cargo test --workspace
cd src-rust && cargo clippy --workspace --all-targets
```

**Expected Output / Artifact Changes:**
- Possibly modified: any M8 files if fixups needed.

**Stop / Escalate Conditions:**
- Same as M7-07.

---

## 5. Milestone 9 Actionable Breakdown

---

### TASK-M9-01 — Complete P1-P12 coverage audit

**Milestone:** 9
**Objective:** Verify all 12 precedence matrix rows have passing unit tests. Fill gaps from M7-05.
**Files:** `crates/query/src/provider_resolution.rs` `#[cfg(test)]`
**Steps:**
1. Enumerate existing tests against P1-P12.
2. Add any missing rows.
3. Run `cargo test -p claurst-query -- provider_resolution`.
**Do NOT:** Add integration tests here — this is unit-level only.
**Validation:** All 12 tests pass.
**Depends on:** M8-11

---

### TASK-M9-02 — Materialize coverage (auth-store, api_base, missing creds)

**Milestone:** 9
**Objective:** Verify `materialize_provider()` tests cover auth-store refresh, api_base override, and missing credentials.
**Files:** `crates/query/src/provider_resolution.rs` `#[cfg(test)]`
**Steps:**
1. Audit existing materialize tests from M7-06.
2. Add missing coverage for: auth-store key takes priority over registry, api_base for lmstudio/llamacpp, error on no credentials.
**Do NOT:** Add network-dependent tests without `#[ignore]`.
**Validation:** Tests pass.
**Depends on:** M8-11

---

### TASK-M9-03 — Agent explicit provider routing integration test

**Milestone:** 9
**Objective:** Integration test: Agent with `provider: "openai"` dispatches to OpenAI.
**Files:** `crates/query/tests/` or `crates/query/src/agent_tool.rs`
**Steps:**
1. Create an integration test that constructs a `ToolContext` with a mock `ProviderRegistry` containing an OpenAI mock.
2. Call `AgentTool::execute()` with `provider: "openai"`.
3. Assert the resolved provider is OpenAI (check logs, mock calls, or result format).
**Do NOT:** Require actual OpenAI API keys — use mocks. Mark as `#[ignore]` if mocking is infeasible.
**Validation:** `cargo test -p claurst-query -- agent_explicit_provider`
**Depends on:** M8-11

---

### TASK-M9-04 — Agent parent inheritance integration test

**Milestone:** 9
**Objective:** Integration test: Agent without `provider`/`model` inherits parent's provider.
**Files:** `crates/query/tests/`
**Steps:**
1. Parent on OpenAI → spawn agent with no provider/model → agent resolves to OpenAI.
2. Assert via mock or logging.
**Do NOT:** Change production code.
**Validation:** Test passes.
**Depends on:** M8-11

---

### TASK-M9-05 — Agent conflict detection tests

**Milestone:** 9
**Objective:** Tests for P3 and P5 (provider/model conflicts).
**Files:** `crates/query/src/provider_resolution.rs` `#[cfg(test)]`
**Steps:**
1. Test `resolve_provider_identity(Some("openai"), "anthropic/claude-sonnet-4-20250514", ...)` → `ProviderModelConflict`.
2. Test `resolve_provider_identity(Some("anthropic"), "openai/gpt-4o", ...)` → `ProviderModelConflict`.
**Do NOT:** Test conflicts with unknown providers (those are not errors).
**Validation:** Tests pass.
**Depends on:** M8-11

---

### TASK-M9-06 — Worker missing registry → hard error test

**Milestone:** 9
**Objective:** Verify that `ToolContext` with `provider_registry: None` in a worker path produces a hard error.
**Files:** `crates/query/src/agent_tool.rs` `#[cfg(test)]`
**Steps:**
1. Construct `ToolContext` with `provider_registry: None`.
2. Call `AgentTool::execute()`.
3. Assert error result.
**Do NOT:** Allow fallback to Anthropic.
**Validation:** Test passes.
**Depends on:** M8-11

---

### TASK-M9-07 — Root missing registry → legacy path test

**Milestone:** 9
**Objective:** Verify that `QueryConfig` with `provider_registry: None` uses the `client: &AnthropicClient` parameter.
**Files:** `crates/query/src/lib.rs` `#[cfg(test)]` or `crates/query/tests/`
**Steps:**
1. Call the root path with `provider_registry: None`.
2. Verify the Anthropic client path is taken.
**Do NOT:** Remove the legacy path.
**Validation:** Test passes.
**Depends on:** M8-11

---

### TASK-M9-08 — Root registry + resolution failure → hard error test

**Milestone:** 9
**Objective:** Verify that when `provider_registry` is `Some` but provider resolution fails, the call fails with error — no Anthropic fallback.
**Files:** `crates/query/tests/`
**Steps:**
1. Provide a registry but request a provider not in it.
2. Assert error (not a silent fallback to Anthropic).
**Do NOT:** Add fallback behavior to fix the test.
**Validation:** Test passes.
**Depends on:** M8-11

---

### TASK-M9-09 — TeamCreate mixed providers integration test

**Milestone:** 9
**Objective:** Integration test: TeamCreate spawns agents with different providers per agent.
**Files:** `crates/tools/tests/` or `crates/query/tests/`
**Steps:**
1. Create a team with agent A on `"openai"` and agent B on `"google"`.
2. Verify each agent resolves to its specified provider.
**Do NOT:** Require real API keys — use mocks. Mark as `#[ignore]` if necessary.
**Validation:** Test passes or is `#[ignore]`-gated with explanation.
**Depends on:** M8-11

---

### TASK-M9-10 — No-key agent spawn integration test

**Milestone:** 9
**Objective:** Integration test: `ANTHROPIC_API_KEY` absent, `--provider openai` → agent succeeds.
**Files:** `crates/query/tests/`
**Steps:**
1. Unset `ANTHROPIC_API_KEY` in test environment.
2. Spawn agent with `provider: "openai"` and a mock OpenAI provider in the registry.
3. Assert success (no "API key not set" error).
**Do NOT:** Actually call OpenAI — mock it.
**Validation:** Test passes.
**Depends on:** M8-11

---

### TASK-M9-11 — Hardcoded-removal verification test

**Milestone:** 9
**Objective:** Verify the three original hardcoded `AnthropicClient::new()` sites no longer exist.
**Files:** `crates/query/src/agent_tool.rs`
**Steps:**
1. `grep` for `AnthropicClient::new()` in `agent_tool.rs`.
2. Assert zero matches (the foreground agent, background agent, and init_team_swarm_runner sites are all removed).
3. Also verify no `ANTHROPIC_API_KEY` env var reads remain in `agent_tool.rs`.
**Do NOT:** Write code — this is a verification task only.
**Validation:** `grep -c 'AnthropicClient::new' crates/query/src/agent_tool.rs` returns 0.
**Depends on:** M8-11

---

### TASK-M9-12 — Full regression + D1 completion declaration

**Milestone:** 9
**Objective:** Run full validation suite and declare D1 complete.
**Files:** Entire workspace.
**Steps:**
1. `cargo fmt --all -- --check`
2. `cargo build --workspace`
3. `cargo test --workspace`
4. `cargo clippy --workspace --all-targets`
5. Manual smoke test: run Claurst with `--provider openai`, spawn an agent, confirm it uses OpenAI (if possible in the test environment).
6. Document: "D1 is complete. Provider-resolution seam is landed. Workers inherit parent providers."
**Do NOT:** Start D2 work.
**Validation:** All commands pass. D1 declared shippable.
**Depends on:** M9-01 through M9-11

---

## 6. Milestone 11 Actionable Breakdown

All M11 tickets depend on **M10 complete** (D2 planning milestone). M10 is complete per `M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md`. Do not start M11 until M10's D2 implementation plan is reviewed and accepted.

**D1-established child independence (already working — no D2 work needed):**
- Parent and child can use **different models** via `AgentInput.model` / `AgentSpec.model`.
- Parent and child can use **different providers** via `AgentInput.provider` / `AgentSpec.provider`.

**D2 extends child independence with first-class settings:**
- Child `max_tokens` (M11-09) — replaces `CHILD_AGENT_FALLBACK_MAX_TOKENS` interim.
- Child `allow_fallback` (M11-10) — spawn-time fallback permission.
- Child `budget_usd` (M11-10) — per-agent USD budget.

**Three independent budget/limit mechanisms (never conflate):**
- `max_tokens` — token count for API response length (per-call).
- `max_budget_usd` — per-loop USD budget (existing D1, in `QueryConfig`).
- `SessionBudget` — cross-all-loops USD budget with `CancellationToken` propagation (new D2).

**Recommended serial execution order (from M10 plan):**
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
M11-10  Schema updates — allow_fallback + budget_usd (depends on M11-05, M11-08, M11-09)
M11-11  QueryEvent variants (depends on M11-08, M11-09)
M11-12  D2 test suite + workspace validation (depends on all above)
```

---

### TASK-M11-01 — TrustDomain enum

**Milestone:** 11
**Objective:** Define `TrustDomain::Local` / `TrustDomain::Cloud` in `crates/api/src/provider_types.rs`.
**Files:** `crates/api/src/provider_types.rs`
**Steps:**
1. Add `TrustDomain` enum with `Local` and `Cloud` variants, deriving `Debug, Clone, Copy, PartialEq, Eq`.
2. Add `TrustDomain::for_provider(provider_id: &str) -> TrustDomain` — hardcoded match: `"ollama" | "lmstudio" | "llamacpp"` → `Local`, all others → `Cloud`.
3. No config system. Hardcoded only.
**Do NOT:** Make trust domains configurable. No YAML, no env vars. No custom provider support beyond `Cloud` default. Do not modify existing types in `provider_types.rs`.
**Validation:** `cargo check -p claurst-api`
**Depends on:** M10

---

### TASK-M11-02 — ModelEntry extension

**Milestone:** 11
**Objective:** Extend `ModelEntry` with D2 capability fields.
**Files:** `crates/api/src/model_registry.rs`
**Steps:**
1. Add to `ModelEntry`: `pdf_input: Option<bool>`, `audio_input: Option<bool>`, `structured_output: Option<bool>`, `max_output_tokens: Option<u32>`.
2. All new fields default to `None`. Use `#[serde(default)]` on each.
3. Update any `ModelEntry` construction in the bundled snapshot to include the new fields as `None` (or rely on serde `default`).
**Do NOT:** Change existing fields. Do not add required fields. Do not modify `ModelRegistry` methods.
**Validation:** `cargo check -p claurst-api` and existing model registry tests pass.
**Depends on:** M10

---

### TASK-M11-03 — Capability enum and matching

**Milestone:** 11
**Objective:** Define `Capability` enum and matching logic in `provider_resolution.rs`.
**Files:** `crates/query/src/provider_resolution.rs`
**Steps:**
1. Add `Capability` enum: `ToolCalling`, `Reasoning`, `Vision`, `PdfInput`, `AudioInput`, `StructuredOutput`.
2. Add function `model_supports_capability(entry: &ModelEntry, cap: &Capability) -> Option<bool>` — returns `Some(true/false)` for known fields, `None` for unknown.
3. Add function `provider_supports_capability(caps: &ProviderCapabilities, cap: &Capability) -> bool` — maps `Capability` variant to `ProviderCapabilities` field. This is the fallback data source when `ModelEntry` returns `None`.
4. Unknown-data policy: existing `bool` fields → always known; new `Option<bool>` fields → `None` means ineligible for fallback.
5. Default required capabilities: `[ToolCalling]` — module-level constant, not configurable.
**Do NOT:** Add a configuration system for capabilities. Do not modify `resolve_provider_identity()` or `materialize_provider()`. Do not modify `ProviderCapabilities` (it already has all needed fields).
**Validation:** `cargo check -p claurst-query` and unit tests for matching logic.
**Depends on:** M11-02

---

### TASK-M11-04 — HealthCache implementation

**Milestone:** 11
**Objective:** Implement `HealthCache` with DashMap and TTL in a new file.
**Files:** `crates/query/src/health_cache.rs` (new), `crates/query/src/lib.rs` (add `mod health_cache; pub use health_cache::*;`)
**Steps:**
1. Create `health_cache.rs`.
2. Implement `HealthCache` with `DashMap<String, (ProviderStatus, Instant)>` (key is provider ID string).
3. `get(provider_id: &str)` → returns cached status if `Instant::elapsed() < ttl`.
4. `insert(provider_id: &str, status: ProviderStatus)` → stores status with current `Instant`.
5. `probe_if_stale(provider_id: &str, provider: &dyn LlmProvider)` → async: returns cached if within TTL, otherwise calls `provider.health_check()` with 5s timeout, updates cache.
6. TTL: 30s default (hardcoded constant).
7. `new() -> Self` → creates cache with empty DashMap and 30s TTL.
8. Add `mod health_cache; pub use health_cache::*;` to `lib.rs`.
**Do NOT:** Add complex retry logic. No backoff. No periodic background refresh. Probe timeout hardcoded to 5s.
**Validation:** Unit tests for cache hit/miss/expiry/timeout.
**Depends on:** M10 (independent of M11-01/02/03)

---

### TASK-M11-05 — resolve_provider_with_fallback()

**Milestone:** 11
**Objective:** Implement the fallback resolution wrapper in `provider_resolution.rs`.
**Files:** `crates/query/src/provider_resolution.rs`, `crates/api/src/model_registry.rs` (small helper addition)
**Steps:**
1. Add `resolve_provider_with_fallback()` — wraps `resolve_provider_identity()` + `materialize_provider()`.
2. When `allow_fallback: false` → no fallback, error with suggestion message ("try allow_fallback: true").
3. When `allow_fallback: true` → enumerate candidates in same trust domain from registry (`provider_registry.provider_ids()` filtered by `TrustDomain::for_provider()`), filter by health (`Healthy` > `Degraded` > skip `Unavailable`) via `health_cache.probe_if_stale()`, filter by capability (at least `[ToolCalling]`), try each.
4. Capability check: get `ModelEntry` for candidate's model, call `model_supports_capability()`, fall back to `provider_supports_capability()` if model data is `None`.
5. Model selection within fallback: family match first (same `family` as original model), then provider default, then skip.
6. Cross-domain fallback is forbidden — no code path permits it.
7. **ModelRegistry helper:** If `ModelRegistry` does not expose `models_for_provider()`, add a minimal ~10-line `models_for_provider(provider_id: &str) -> Vec<&ModelEntry>` helper to `model_registry.rs`. This is a targeted addition, not a registry redesign.
**Do NOT:** Add `allow_cross_domain_fallback`. Cross-domain is forbidden, no escape hatch. Do not invent a policy engine. Do not modify `resolve_provider_identity()` or `materialize_provider()` signatures.
**Validation:** Unit tests: same-domain fallback succeeds, cross-domain prohibited, `allow_fallback: false` errors with suggestion.
**Depends on:** M11-01, M11-03, M11-04

---

### TASK-M11-06 — CostTracker extension

**Milestone:** 11
**Objective:** Add `agent_id` and `provider_id` fields to `CostTracker`.
**Files:** `crates/core/src/lib.rs` (in the `CostTracker` section around line 2850)
**Steps:**
1. Add `agent_id: parking_lot::RwLock<Option<String>>` and `provider_id: parking_lot::RwLock<Option<String>>` to `CostTracker`.
2. Update `CostTracker::new()` to initialize both as `None`.
3. Add setters: `set_agent_id(&self, id: String)`, `set_provider_id(&self, id: String)`.
4. Workers call these setters when constructing their `CostTracker`.
**Do NOT:** Change cost calculation logic. Do not add new cost events. Do not modify existing `CostTracker` fields.
**Validation:** `cargo check --workspace`
**Depends on:** M10 (independent of M11-01 through M11-05)

---

### TASK-M11-07 — SessionBudget implementation

**Milestone:** 11
**Objective:** Implement `SessionBudget` with cost tracking and cancel token.
**Files:** `crates/query/src/session_budget.rs` (new), `crates/query/src/lib.rs` (add `mod session_budget; pub use session_budget::*;`)
**Steps:**
1. Create `session_budget.rs`.
2. `SessionBudget` struct holds: `budget_usd: f64`, `spent: parking_lot::Mutex<f64>` (or `AtomicU64` with f64 bit-casting), root `CancellationToken`.
3. `new(budget_usd: f64) -> Self` → creates budget with zero spent and a new root token.
4. `record_cost(&self, cost_usd: f64)` → adds cost to spent.
5. `check_and_cancel(&self)` → if `spent >= budget_usd`, cancel the root token.
6. `child_cancel_token(&self) -> CancellationToken` → returns `self.root_token.child_token()`.
7. `is_cancelled(&self) -> bool` → returns `self.root_token.is_cancelled()`.
8. Add module to `lib.rs`.
**Do NOT:** Add complex accounting. No per-provider budgets. No persistent storage.
**Validation:** Unit tests for budget check + cancel + child token propagation.
**Depends on:** M10 (independent of M11-01 through M11-05)

---

### TASK-M11-08 — Budget + cancellation wiring

**Milestone:** 11
**Objective:** Wire `SessionBudget` into root session and worker spawn paths.
**Files:** `crates/cli/src/main.rs`, `crates/query/src/agent_tool.rs`, `crates/query/src/lib.rs`, `crates/tools/src/lib.rs` (ToolContext), `crates/tools/src/team_tool.rs`
**Steps:**
1. In `main.rs`: create `SessionBudget` when `--budget-usd` CLI flag is set. Pass `Arc<SessionBudget>` into `QueryConfig` and `ToolContext`.
2. Add `session_budget: Option<Arc<SessionBudget>>` to `QueryConfig` (lib.rs).
3. Add `session_budget: Option<Arc<SessionBudget>>` and `health_cache: Option<Arc<HealthCache>>` to `ToolContext` (tools/src/lib.rs).
4. In `run_query_loop()` (lib.rs): after existing budget guard at ~line 1397, add `if let Some(ref sb) = config.session_budget { sb.record_cost(turn_cost); sb.check_and_cancel(); }`.
5. In `agent_tool.rs`: when creating child `CancellationToken`, use `session_budget.child_cancel_token()` if session budget exists, otherwise `CancellationToken::new()` as today. Apply at all three spawn points (foreground, background, team runner).
6. TeamCreate per-agent tokens become children of session budget root token (via `AgentRunParams.ctx.session_budget`).
**Do NOT:** Add per-agent budgets. Do not change `legacy_client: Option<&AnthropicClient>` on `run_query_loop()`. Do not remove the existing `max_budget_usd` per-loop budget mechanism. Do not modify `CHILD_AGENT_FALLBACK_MAX_TOKENS` — child `max_tokens` override wiring is M11-09's scope.
**Validation:** `cargo check --workspace`
**Depends on:** M11-07

---

### TASK-M11-09 — Child execution override wiring (max_tokens + spawn-time settings)

**Milestone:** 11
**Objective:** Make child `max_tokens` a first-class D2 setting, replacing the `CHILD_AGENT_FALLBACK_MAX_TOKENS` interim. Ensure children can have independent execution settings passed through all three spawn paths.

**Why this ticket exists:** D1 introduced `CHILD_AGENT_FALLBACK_MAX_TOKENS = 4_096` as an interim constant at `agent_tool.rs:132`. This ticket supersedes it with a first-class child `max_tokens` field, settable per-agent at spawn time. The constant is retained only as the backward-compatible default when no explicit child `max_tokens` is specified.

**Files:** `crates/query/src/agent_tool.rs`, `crates/tools/src/team_tool.rs`

**Steps:**
1. Add to `AgentInput` (agent_tool.rs):
   - `max_tokens: Option<u32>` (absent = backward-compatible D1 default of `4096`)
2. Add to `AgentSpec` (team_tool.rs):
   - `max_tokens: Option<u32>` (absent = backward-compatible D1 default of `4096`)
3. Add to `AgentRunParams` (team_tool.rs):
   - `max_tokens_override: Option<u32>` (resolved from `AgentSpec.max_tokens` or `AgentInput.max_tokens`)
4. Wire child `QueryConfig.max_tokens` from override in all three spawn paths:
   - **Foreground agent** (agent_tool.rs): `config.max_tokens = input.max_tokens.unwrap_or(CHILD_AGENT_FALLBACK_MAX_TOKENS)`
   - **Background agent** (agent_tool.rs): same pattern
   - **Team runner** (agent_tool.rs / team_tool.rs): `config.max_tokens = params.max_tokens_override.unwrap_or(CHILD_AGENT_FALLBACK_MAX_TOKENS)`
5. Update JSON schemas in `input_schema()` for `AgentTool` and `TeamCreateTool` to expose `max_tokens` as an optional integer field.
6. `CHILD_AGENT_FALLBACK_MAX_TOKENS` constant: retain as the named default value (used in `unwrap_or` above). The constant is no longer the sole code path — it is the backward-compatible fallback.

**Backward compatibility guarantee:** When `max_tokens` is absent/`None` in `AgentInput`, `AgentSpec`, or `AgentRunParams`, behavior is identical to D1: child gets `4096`. Existing callers that do not set `max_tokens` see zero behavioral change.

**Child execution independence summary (what is first-class after D2):**

| Setting | Parent field | Child field | D1 status | D2 status |
|---|---|---|---|---|
| Model | `QueryConfig.model` | `AgentInput.model` / `AgentSpec.model` | Already independent | No change |
| Provider | `QueryConfig.provider` | `AgentInput.provider` / `AgentSpec.provider` | Already independent | No change |
| `max_tokens` | `QueryConfig.max_tokens` | `AgentInput.max_tokens` / `AgentSpec.max_tokens` | Interim `4096` fallback | **First-class (this ticket)** |
| `allow_fallback` | N/A | `AgentInput.allow_fallback` / `AgentSpec.allow_fallback` | Not supported | First-class (M11-10) |
| `budget_usd` | `QueryConfig.max_budget_usd` | `AgentInput.budget_usd` / `AgentSpec.budget_usd` | Not supported | First-class (M11-10) |

**Do NOT:** Finalize parent/child token policy beyond making child `max_tokens` settable. Do not change parent `max_tokens` resolution. Do not add dynamic per-model `max_tokens` negotiation. Do not remove `CHILD_AGENT_FALLBACK_MAX_TOKENS` — it remains as the named default constant.
**Validation:** `cargo check --workspace && cargo test -p claurst-query -- agent_tool`
**Depends on:** M11-08

---

### TASK-M11-10 — Schema updates (allow_fallback, budget_usd)

**Milestone:** 11
**Objective:** Expose remaining D2 controls in Agent/TeamCreate input schemas. Note: child `max_tokens` schema was already added in M11-09; this ticket adds fallback and budget controls.
**Files:** `crates/query/src/agent_tool.rs`, `crates/tools/src/team_tool.rs`
**Steps:**
1. Add to `AgentInput` (agent_tool.rs):
   - `allow_fallback: Option<bool>` (default `None` → treated as `false`)
   - `budget_usd: Option<f64>`
2. Add to `AgentSpec` (team_tool.rs):
   - `allow_fallback: Option<bool>` (default `None` → treated as `false`)
   - `budget_usd: Option<f64>`
3. Add to `AgentRunParams` (team_tool.rs):
   - `allow_fallback: bool` (resolved from `Option`)
   - `budget_usd: Option<f64>`
4. Update JSON schemas in `input_schema()` for `AgentTool` and `TeamCreateTool` to include `allow_fallback` and `budget_usd`.
5. Wire `allow_fallback` through to `resolve_provider_with_fallback()` in the spawn paths.
6. Wire `budget_usd` through to `SessionBudget` construction for sub-agents (or pass existing session budget if no per-agent budget).
**Do NOT:** Default `allow_fallback` to true. Pinned behavior is the default. Do not add `allow_cross_domain_fallback`. Do not modify `max_tokens` wiring — that is M11-09's scope.
**Validation:** `cargo check --workspace`
**Depends on:** M11-05, M11-08, M11-09

---

### TASK-M11-11 — QueryEvent variants

**Milestone:** 11
**Objective:** Add D2-specific event variants for observability.
**Files:** `crates/query/src/lib.rs`
**Steps:**
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
**Do NOT:** Add UI rendering for these events — that's a separate concern.
**Validation:** `cargo check --workspace`
**Depends on:** M11-08, M11-09

---

### TASK-M11-12 — D2 test suite + workspace validation

**Milestone:** 11
**Objective:** Write D2 acceptance tests (AC16-AC28) and validate workspace.
**Files:** `crates/query/tests/`, `crates/query/src/provider_resolution.rs`, `crates/query/src/health_cache.rs`, `crates/query/src/session_budget.rs`, `crates/query/src/agent_tool.rs`
**Steps:**
1. Unit tests for trust-domain classification: local providers → `Local`, cloud → `Cloud`.
2. Unit tests for capability matching: known fields → `Some`, unknown → `None`, fallback to `ProviderCapabilities`.
3. Unit tests for health cache TTL: hit, miss, expiry, timeout handling.
4. Unit tests for session budget: threshold check, cancel propagation, child token behavior.
5. Unit tests for `resolve_provider_with_fallback()` — same-domain fallback succeeds, cross-domain prohibited, `allow_fallback: false` errors with suggestion, health ordering (Healthy > Degraded > skip Unavailable).
6. Child execution override tests:
   - Explicit `max_tokens` propagated to child `QueryConfig`.
   - Absent `max_tokens` defaults to `4096` (backward compatibility).
   - `budget_usd` propagated to child `SessionBudget`.
   - `allow_fallback` propagated to child provider resolution.
   - Cover foreground, background, and team runner spawn paths explicitly.
7. Full workspace validation:
   - `cargo fmt --all -- --check`
   - `cargo build --workspace`
   - `cargo test --workspace`
   - `cargo clippy --workspace --all-targets`
**Do NOT:** Skip any of AC16-AC28. Mark infeasible tests as `#[ignore]` with explanation. Do not start M12 work.
**Validation:** All four workspace validation commands pass.
**Depends on:** M11-01 through M11-11

---

## 7. Milestone 12 Actionable Breakdown

---

### TASK-M12-01 — Locate and audit surrogate test

**Milestone:** 12
**Objective:** Find the surrogate D2 test referenced in `d2_test_micro_patch_report.md` and audit what it covers.
**Files:** Search `crates/api/tests/`, `crates/query/tests/`, and the report itself.
**Steps:**
1. Read `d2_test_micro_patch_report.md` to identify the surrogate test location.
2. `grep` for test function names or markers mentioned in the report.
3. List what the surrogate test asserts.
4. Cross-reference against the D2 test suite from M11-12.
**Do NOT:** Delete anything yet.
**Validation:** Surrogate test location identified and coverage documented.
**Depends on:** M11-12

---

### TASK-M12-02 — Remove/replace surrogate test

**Milestone:** 12
**Objective:** Remove the surrogate test now that real D2 production tests exist.
**Files:** Surrogate test file(s), `d2_test_micro_patch_report.md`
**Steps:**
1. If the surrogate test's assertions are fully covered by M11-12 tests, delete it.
2. If any assertion is NOT covered, either add the missing test to the D2 suite first, or rewrite the surrogate as a proper integration test.
3. Remove or archive `d2_test_micro_patch_report.md`.
**Do NOT:** Delete tests that cover behavior not yet tested elsewhere.
**Validation:** `cargo test --workspace` passes. No surrogate D2 logic remains.
**Depends on:** M12-01

---

### TASK-M12-03 — D2 coverage completeness verification

**Milestone:** 12
**Objective:** Verify all RFC-intended D2 semantics are covered by real production tests.
**Files:** Entire test suite.
**Steps:**
1. Walk through AC16-AC28 from RFC v3.1 §14.2.
2. For each AC, identify the test(s) that cover it.
3. Document any gaps.
4. Fill gaps.
5. `cargo test --workspace` passes.
**Do NOT:** Start M13 work.
**Validation:** Every AC has at least one corresponding test. All pass.
**Depends on:** M12-02

---

## 8. Open Verification Items

These items were marked "requires confirmation" in the implementation plan. Execution agents must verify them before writing code.

| ID | Item | Where to verify | What to do if different |
|----|------|----------------|------------------------|
| V1 | `AgentInput` struct location at `agent_tool.rs:~125-150` | `grep -n 'struct AgentInput' crates/query/src/agent_tool.rs` | Use the actual line number |
| V2 | Background agent block at `agent_tool.rs:~370-420` | `grep -n 'run_in_background' crates/query/src/agent_tool.rs` | Use the actual line number |
| V3 | `TeamCreateTool::execute()` agent spawn at `team_tool.rs:~380-430` | `grep -n 'run_agent' crates/tools/src/team_tool.rs` | Use the actual line number |
| V4 | `ToolContext` construction in `main.rs` at `~1108-1128` | `grep -n 'ToolContext' crates/cli/src/main.rs` | **Verified:** actual location is `main.rs:647-658` |
| V5 | `claurst-tools` depends on `claurst-api` | `grep 'claurst-api' crates/tools/Cargo.toml` | **Verified:** dependency exists |
| V6 | `known_providers` list completeness vs `registry.rs:26-70` | Compare arrays | **Verified:** inline list at `lib.rs:879-886` is INCOMPLETE — missing: `codex`, `siliconflow`, `moonshot`, `zhipu`, `qwen`, `nebius`, `novita`, `ovhcloud`, `scaleway`, `vultr`, `vultr-ai`, `baseten`, `friendli`, `upstage`, `stepfun`. The `KNOWN_PROVIDERS` constant in M7-01 must include all of them. |
| V7 | `ModelRegistry::find_provider_for_model()` exists and signature | `grep -n 'find_provider_for_model' crates/api/src/model_registry.rs` | Adapt function call if signature differs |
| V8 | `ProviderRegistry::get()` return type | `grep -n 'fn get' crates/api/src/registry.rs` | Adapt materialization code |
| V9 | Surrogate D2 test location | Search `crates/api/tests/`, `crates/query/tests/` | Needed for M12-01 |
| V10 | Where `provider_registry` and `model_registry` are built in `main.rs` | `grep -n 'ProviderRegistry\|ModelRegistry' crates/cli/src/main.rs` | Needed for M8-02 |
| V11 | `thiserror` dependency in `crates/query/Cargo.toml` | `grep 'thiserror' crates/query/Cargo.toml` | Add if missing (M7-01) |

---

## 9. Global Rules for Execution Agents

Every execution agent working on these tickets MUST follow these rules:

### Scope Discipline
1. **Do not expand scope beyond the ticket.** If the ticket says "modify file X," do not touch file Y unless the ticket explicitly lists it.
2. **Do not rewrite adjacent systems.** If you see code near your change that could be "improved," leave it alone.
3. **Do not replace explicit behavior with a "cleaner" abstraction** unless the ticket requires it.
4. **Do not silently fix unrelated issues.** Unrelated bugs, warnings, or style issues are not your problem.
5. **Do not add comments, docstrings, or type annotations** to code you did not change.
6. **Do not add error handling, fallbacks, or validation** beyond what the ticket specifies.

### Quality Gates
7. **Do not continue past a failed validation command.** If `cargo test` fails, stop and investigate.
8. **Do not skip validation commands.** Every ticket has a "Validation Commands" section. Run all of them.
9. **Do not commit with `--no-verify`** or skip any pre-commit hooks.

### Uncertainty Handling
10. **Do not guess when repo reality conflicts with the ticket.** If a struct is not where the ticket says it is, STOP. Read the actual file. If the difference is minor (line number drift), adapt. If the difference is structural (struct doesn't exist, different fields), escalate.
11. **Do not invent types, modules, helpers, or abstractions** not specified in the ticket.
12. **Do not silently resolve uncertainty.** If the ticket says "requires confirmation," verify first.

### D2 Boundary
13. **Do not introduce D2 types during M7, M8, or M9 work.** The following are forbidden until M11: `TrustDomain`, `HealthCache`, `SessionBudget`, `Capability`, `allow_fallback`, `budget_usd`, `max_tokens_override` (on `AgentRunParams`), `resolve_provider_with_fallback()`. Note: `CHILD_AGENT_FALLBACK_MAX_TOKENS` is a D1 interim constant — it is superseded by M11-09 but must not be removed before M11-09 lands.
14. **Do not remove the `client: &AnthropicClient` parameter** from `run_query_loop()`. Its removal is deferred.
15. **Do not remove legacy compatibility paths** (the `else` branch when `provider_registry` is `None`).
16. **Do not conflate budget/limit mechanisms.** `max_tokens` is a token count (per-call). `max_budget_usd` is per-loop USD (D1). `SessionBudget` is cross-session USD (D2). These are independent and must never be mixed.

### Communication
17. **When you stop due to a blocker, report clearly:** what you expected, what you found, what you tried, and why you stopped.
18. **When a ticket is complete, report:** files changed, tests passing, any deviations from the ticket instructions.

---

*End of MPWO Work Order Pack.*
