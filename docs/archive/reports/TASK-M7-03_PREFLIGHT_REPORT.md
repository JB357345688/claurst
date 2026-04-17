# TASK-M7-03 Preflight Report

**Ticket id**

`TASK-M7-03`

**Preflight verdict**

Preflight passed. Repo reality matches `TASK-M7-03` closely enough to execute safely with no structural drift found.

There is line-number drift only: the work order cites `lib.rs:937-977`, while the live materialization span starts at [lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:944), the final provider selection is at [lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:982), and the live missing-credentials branch is at [lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:1263).

**Verified paths and symbols**

- [provider_resolution.rs](/home/jordi/claurst/src-rust/crates/query/src/provider_resolution.rs:157) exists and `materialize_provider(...)` is still the M7-01 stub with `todo!("M7-03")`.
- The extracted type seam already exists in [provider_resolution.rs](/home/jordi/claurst/src-rust/crates/query/src/provider_resolution.rs:58): `ProviderIdentity`, `ExecutionTarget`, and `ProviderResolutionError` are present and usable for M7-03.
- The live inline materialization logic is still present in [lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:944) and remains in the caller-owned non-Anthropic dispatch block gated by [lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:933).
- [runtime_provider_for](/home/jordi/claurst/src-rust/crates/api/src/registry.rs:73) still exists with the expected callable signature:
  `pub fn runtime_provider_for(provider_id: &str) -> Option<Arc<dyn LlmProvider>>`
- [ProviderRegistry::get](/home/jordi/claurst/src-rust/crates/api/src/registry.rs:113) still returns `Option<&Arc<dyn LlmProvider>>`, so the ticket’s expected `.cloned()` lookup pattern remains correct.
- [ProviderConfig](/home/jordi/claurst/src-rust/crates/core/src/lib.rs:669) still exposes [api_base: Option<String>](/home/jordi/claurst/src-rust/crates/core/src/lib.rs:673), which is the exact field path the inline override logic uses.
- The local provider rebuild helpers still exist in [openai_compat_providers.rs](/home/jordi/claurst/src-rust/crates/api/src/providers/openai_compat_providers.rs:19), [openai_compat_providers.rs](/home/jordi/claurst/src-rust/crates/api/src/providers/openai_compat_providers.rs:36), and [openai_compat_providers.rs](/home/jordi/claurst/src-rust/crates/api/src/providers/openai_compat_providers.rs:52), and the builder method [with_base_url(...)](/home/jordi/claurst/src-rust/crates/api/src/providers/openai_compat.rs:129) is still available.
- The module path `claurst_api::providers::openai_compat_providers` is live in the current inline source at [lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:962).

**Live step mapping**

- Auth-store refresh:
  [lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:944) calls `claurst_api::registry::runtime_provider_for(&provider_id_str)` to prefer fresh auth-store credentials over startup-time registry state.
- Registry lookup fallback:
  [lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:946) sets `registry_provider` to `None` when `runtime_provider` exists; otherwise it uses `registry.get(&pid).cloned()`.
- `api_base` override detection:
  [lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:958) reads `tool_ctx.config.provider_configs.get(&provider_id_str).and_then(|pc| pc.api_base.as_deref())`.
- Exact local-provider override rebuild path:
  [lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:967), [lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:970), and [lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:973) rebuild the supported local providers through `openai_compat_providers::*().with_base_url(base_url)`.
- Final provider selection:
  [lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:982) performs `let provider = runtime_provider.or(registry_provider);`.
- Live missing-provider behavior:
  [lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:1263) returns an explicit error only when the resolved provider is non-Anthropic and no provider instance was materialized.
  [lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:1290) still preserves Anthropic fallthrough to the raw client path when no auth-store-backed provider was materialized.

**Exact override alias set**

- `ollama`
- `lmstudio`
- `lm-studio`
- `llamacpp`
- `llama-cpp`

No additional alias arms are present in the inline override block and none should be added during M7-03.

**Required adjustments before execution**

- Use the live anchors above rather than the stale `937-977` line numbers from the work order.
- `provider_resolution.rs` will need the execution-local imports the ticket calls for:
  `use std::collections::HashMap;`
  `use claurst_core::config::ProviderConfig;`
- Keep the change scoped to [provider_resolution.rs](/home/jordi/claurst/src-rust/crates/query/src/provider_resolution.rs:157) only. No extra file edits are required for the M7-03 implementation itself.
- Do not move `use_provider_dispatch`, event emission, capability shaping, stream handling, or Anthropic raw-client fallthrough out of `lib.rs`; those remain caller behavior and are outside M7-03.
- The extracted function should encode missing materialization as `ProviderResolutionError::NoCredentials(identity.provider_id.clone())`, which is ticket-aligned even though the current inline caller path still turns the non-Anthropic case into a `QueryOutcome::Error`.

**Blockers**

- None.
