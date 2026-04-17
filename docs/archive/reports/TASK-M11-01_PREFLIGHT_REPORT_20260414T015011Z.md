# TASK-M11-01 Preflight Report

## Ticket ID

`TASK-M11-01 — TrustDomain enum`

## Verdict

`BLOCKED`

## Current Branch

`feature/provider-resolution-seam`

## Current HEAD

Short: `6b362a0`
Full: `6b362a09c4ef4d614840ed199869bb9d38600e16`

## Authority Files Reviewed

- `AGENTS.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md`
- `docs/Current/M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md`
- `docs/Current/D1_REVIEW_REPORT_20260413T233604Z.md`

## Verified File Path(s)

- Target file exists at `src-rust/crates/api/src/provider_types.rs`
- `src-rust/Cargo.toml` is the live workspace root
- `src-rust/crates/api/Cargo.toml` declares package `claurst-api`

## Verified Symbols / Repo Facts

- `TrustDomain` does **not** already exist in `src-rust` (`rg -n "\bTrustDomain\b" src-rust` returned no matches).
- `provider_types.rs` is the provider-agnostic shared types module for the API crate and is publicly re-exported by `src-rust/crates/api/src/lib.rs:31` and `src-rust/crates/api/src/lib.rs:63`.
- `provider_types.rs` has an obvious insertion point for a new top-level enum between `ProviderStatus` and `AuthMethod` without modifying existing types:
  - `ProviderStatus` ends at `src-rust/crates/api/src/provider_types.rs:237`
  - `AuthMethod` begins at `src-rust/crates/api/src/provider_types.rs:243`
- Adding a new enum plus `impl TrustDomain` in that gap can be done without modifying any existing type definitions.
- `ollama` is a valid current provider identifier:
  - canonical constant at `src-rust/crates/core/src/provider_id.rs:48`
  - local provider registered at `src-rust/crates/api/src/registry.rs:300`
  - current query known-provider set includes it at `src-rust/crates/query/src/provider_resolution.rs:23`
- `lmstudio` and `llamacpp` are valid current identifiers in repo reality as accepted aliases / user-facing IDs:
  - present in query known-provider set at `src-rust/crates/query/src/provider_resolution.rs:24-25`
  - present in TUI provider selection at `src-rust/crates/tui/src/app.rs:298` and `src-rust/crates/tui/src/app.rs:305`
- However, the **canonical runtime provider IDs** for those two providers are hyphenated:
  - `ProviderId::LM_STUDIO = "lm-studio"` at `src-rust/crates/core/src/provider_id.rs:49`
  - `ProviderId::LLAMA_CPP = "llama-cpp"` at `src-rust/crates/core/src/provider_id.rs:50`
  - local provider factories use those canonical IDs at `src-rust/crates/api/src/providers/openai_compat_providers.rs:38` and `src-rust/crates/api/src/providers/openai_compat_providers.rs:52`
  - `ProviderRegistry::with_available_providers()` registers those providers via the factories at `src-rust/crates/api/src/registry.rs:301-302`
- Current repo code already treats both spellings as aliases in some places:
  - `src-rust/crates/query/src/provider_resolution.rs:190-195` accepts both `lmstudio | lm-studio` and `llamacpp | llama-cpp`
- Immediate M11 ownership fit is confirmed:
  - the accepted M10 plan designates `provider_types.rs` as the ownership point for `TrustDomain`
  - later D2 planning expects `TrustDomain::for_provider()` to be called from fallback resolution logic over provider IDs enumerated from the registry

## Validation Command(s) Verified

- Ticket command from authority: `cd src-rust && cargo check -p claurst-api`
- Verified against live workspace layout: `src-rust` is the correct working directory
- Live result: `PASS`

## Drift Found

- Non-blocking path-prefix drift:
  - some materials reference `crates/api/src/provider_types.rs`
  - live repo path is `src-rust/crates/api/src/provider_types.rs`
  - this is path-prefix drift only, not structural drift
- No branch / HEAD drift:
  - live branch is still `feature/provider-resolution-seam`
  - live HEAD is still `6b362a0`, matching the accepted D1 baseline reports
- Blocking identifier drift / authority conflict:
  - the ticket hardcodes only `"ollama" | "lmstudio" | "llamacpp" => Local`
  - live canonical runtime IDs for LM Studio and llama.cpp are `"lm-studio"` and `"llama-cpp"`
  - because later M11 fallback design expects `TrustDomain::for_provider()` to classify provider IDs coming from `ProviderRegistry::provider_ids()`, implementing the ticket exactly as written would misclassify registered local providers `lm-studio` and `llama-cpp` as `Cloud`
- Repo is not clean:
  - unrelated modified `.gitignore`
  - many untracked docs and build artifacts
  - noted only for review-basis hygiene; not the blocker for this preflight

## Blockers

- **Material ticket/repo mismatch on provider identifiers.**
  - The ticket wording is too narrow for live repo reality.
  - `lmstudio` / `llamacpp` are real aliases, but the registry-backed canonical IDs are `lm-studio` / `llama-cpp`.
  - Implementing the exact hardcoded mapping from the ticket would be mechanically easy but semantically wrong for the registry-backed D2 path that the accepted M10 plan describes.

## Hosted Ollama Compatibility Baseline Preserved

`yes`

Basis:
- No source files were modified in this preflight pass.
- The target ticket is scoped to `provider_types.rs`, not the hosted-Ollama seam in `provider_resolution.rs` / `auth_store.rs`.
- Live repo still contains the D1 hosted-Ollama seam behavior previously accepted.

## Exact Recommendation For Next Step

Do **not** execute `TASK-M11-01` yet.

First update the ticket authority so that one of the following is explicitly approved:

1. `TrustDomain::for_provider()` must classify both canonical and alias forms as local:
   - `"ollama" | "lmstudio" | "lm-studio" | "llamacpp" | "llama-cpp" => Local`

2. Or, upstream canonicalization must be defined before `TrustDomain::for_provider()` is called, and the ticket must say that only canonicalized IDs reach this function.

After that clarification, rerun preflight or proceed directly to execution against the clarified ticket text.
