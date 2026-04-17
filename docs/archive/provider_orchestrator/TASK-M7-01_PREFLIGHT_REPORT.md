# TASK-M7-01 Preflight Report

**Preflight verdict**

Preflight passed. Repo reality matches `TASK-M7-01` closely enough to execute safely with no structural drift found.

**Verified paths and symbols**

- [provider_resolution.rs](/home/jordi/claurst/src-rust/crates/query/src/provider_resolution.rs) does not exist yet. This matches the ticket.
- [lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:11) has the expected top-level `pub mod` block at lines 11-20 and `pub use` block starting at line 21, so module wiring can be added cleanly near the top.
- [Cargo.toml](/home/jordi/claurst/src-rust/crates/query/Cargo.toml:8) already depends on `claurst-api`, and [thiserror](/home/jordi/claurst/src-rust/crates/query/Cargo.toml:17) is already present. No dependency adjustment is needed.
- [claurst_api::LlmProvider](/home/jordi/claurst/src-rust/crates/api/src/lib.rs:66), [claurst_api::ProviderRegistry](/home/jordi/claurst/src-rust/crates/api/src/lib.rs:72), and [claurst_api::ModelRegistry](/home/jordi/claurst/src-rust/crates/api/src/lib.rs:80) are re-exported from `claurst_api` and are importable from `claurst-query`.
- [ModelRegistry::find_provider_for_model](/home/jordi/claurst/src-rust/crates/api/src/model_registry.rs:208) exists, which is consistent with the seam this ticket sets up.
- [claurst_core::ProviderId](/home/jordi/claurst/src-rust/crates/core/src/lib.rs:8) is re-exported from `claurst_core`.
- [ProviderConfig](/home/jordi/claurst/src-rust/crates/core/src/lib.rs:669) exists under `claurst_core::config`, matching the planned stub signature.
- The current inline known-provider list is at [lib.rs:879-886](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:879), and the registry source is [registry.rs:26-69](/home/jordi/claurst/src-rust/crates/api/src/registry.rs:26).

**Provider list confirmation**

The verified `KNOWN_PROVIDERS` union to use for `TASK-M7-01` is:

```rust
[
    "anthropic", "openai", "google", "groq", "mistral",
    "deepseek", "xai", "cohere", "perplexity", "cerebras",
    "openrouter", "togetherai", "together-ai", "deepinfra",
    "venice", "github-copilot", "ollama", "lmstudio",
    "llamacpp", "azure", "amazon-bedrock", "huggingface",
    "nvidia", "fireworks", "sambanova", "codex",
    "siliconflow", "moonshot", "zhipu", "qwen", "nebius",
    "novita", "ovhcloud", "scaleway", "vultr", "vultr-ai",
    "baseten", "friendli", "upstage", "stepfun",
]
```

This matches the work-order note: the inline `lib.rs` list is incomplete and must be augmented with the additional providers present in `provider_from_key()`.

**Required adjustments before execution**

- No prerequisite repo fixes are needed before execution.
- `crates/query/Cargo.toml` does not need a `thiserror` change.
- During execution, `KNOWN_PROVIDERS` must use the verified union above, not the shorter inline `lib.rs` array alone.
- `lib.rs` changes must stay limited to `pub mod provider_resolution;` and `pub use provider_resolution::*;`.

**Blockers**

- None.
