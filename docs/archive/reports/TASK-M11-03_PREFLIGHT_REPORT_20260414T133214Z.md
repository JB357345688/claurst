# TASK-M11-03 Preflight Report

## Ticket ID

`TASK-M11-03 — Capability enum and matching`

## Verdict

`GO`

## Current Branch

`feature/provider-resolution-seam`

## Current HEAD

`fe2196942df14c459c73d273d55abcee932bf602`

## Authority Files Reviewed

- `AGENTS.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md`
- `docs/archive/reports/M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md`
- `docs/archive/reports/TASK-M11-02_CLOSEOUT_REPORT_20260414T025746Z.md`

## Verified File Paths

- `src-rust/crates/query/src/provider_resolution.rs`
- `src-rust/crates/query/src/lib.rs`
- `src-rust/crates/query/src/agent_tool.rs`
- `src-rust/crates/api/src/model_registry.rs`
- `src-rust/crates/api/src/provider_types.rs`
- `src-rust/crates/api/src/lib.rs`

## Preflight Verdict Basis

- Live branch and HEAD exactly match the accepted M11-02 closeout baseline: `feature/provider-resolution-seam` at `fe2196942df14c459c73d273d55abcee932bf602`.
- `src-rust/crates/query/src/provider_resolution.rs` exists and structurally matches the authority as the correct ownership file for M11-03.
- `TrustDomain` from M11-01 exists in `src-rust/crates/api/src/provider_types.rs` and is re-exported by `claurst_api`, so it is available to `provider_resolution.rs` for later M11-05 use without extra plumbing.
- `ModelEntry` in `src-rust/crates/api/src/model_registry.rs` already contains the four M11-02 fields in live repo reality: `pdf_input: Option<bool>`, `audio_input: Option<bool>`, `structured_output: Option<bool>`, `max_output_tokens: Option<u32>`.
- The correct narrow import path for M11-03 into `provider_resolution.rs` is `claurst_api::ModelEntry`; `ProviderCapabilities` and `TrustDomain` are likewise available via `claurst_api` re-exports.
- `claurst-query` already exports `provider_resolution` publicly via `pub mod provider_resolution;` and `pub use provider_resolution::*;`, so M11-05 will be able to consume the new enum/helper without extra module plumbing.

## Verified Symbols / Repo Facts

- `provider_resolution.rs` currently defines `KNOWN_PROVIDERS`, `ResolutionSource`, `ProviderIdentity`, `ExecutionTarget`, `ProviderResolutionError`, `resolve_provider_identity()`, `materialize_provider()`, and `normalize_ollama_api_base()`.
- `Capability` does not exist in `provider_resolution.rs`.
- No capability-matching helper currently exists in `provider_resolution.rs` or elsewhere in the Rust workspace.
- The obvious insertion seam is near the top of `provider_resolution.rs`, immediately after `KNOWN_PROVIDERS` and before the existing resolution types/functions, or immediately before `resolve_provider_identity()`. Either location stays within the ticket’s single-file scope.
- `ModelRegistry` already exposes `get(&self, provider_id: &str, model_id: &str) -> Option<&ModelEntry>`, which is sufficient for later M11-05 consumption of `model_supports_capability()`.
- `ModelRegistry` still does not expose `models_for_provider()`. That remains an M11-05 risk only and is not a blocker for M11-03.

## Existing Capability-Surface Reality

- `ModelEntry` capability fields currently available:
- `tool_calling: bool`
- `reasoning: bool`
- `vision: bool`
- `pdf_input: Option<bool>`
- `audio_input: Option<bool>`
- `structured_output: Option<bool>`
- `ProviderCapabilities` capability fields currently available:
- `tool_calling: bool`
- `thinking: bool`
- `image_input: bool`
- `pdf_input: bool`
- `audio_input: bool`
- `structured_output: bool`
- Unknown-data policy is implementable from live types:
- Existing `bool` model fields are always known.
- New `Option<bool>` model fields can return `None`, which matches the “unknown -> ineligible for fallback” rule.
- Authority-consistency check result:
- The live M10 plan and the live MPWO both treat provider-level capability fallback as part of the M11 seam. The plan explicitly states `model_supports_capability()` should use `ModelEntry` first and fall back to `ProviderCapabilities` when model data is `None`.
- This is not a blocker for M11-03. The helper mapping itself is in-scope for M11-03; the actual fallback-resolution wiring remains M11-05.
- M11-03 can remain scope-tight if it limits itself to defining `Capability`, `model_supports_capability()`, `provider_supports_capability()`, the default capability constant, and unit tests in `provider_resolution.rs`.

## Existing Test Reality

- `provider_resolution.rs` already contains a substantial in-file `#[cfg(test)]` module with 22 unit tests.
- The existing test module is the correct place for M11-03 matching-logic tests.
- Existing local test seams are sufficient:
- `TestProvider` already returns a `ProviderCapabilities` value, so provider-level fallback mapping can be tested without new shared fixtures.
- There is no existing `ModelEntry` fixture/helper in this file, but a small local constructor can be added narrowly inside the existing test module without touching shared test infrastructure.
- No existing test names include `capability`, so the authority’s filtered test command currently matches zero tests until new capability-named tests are added.

## Validation Commands Verified

- Ran `cd src-rust && cargo check -p claurst-query`
- Result: `PASS`
- Ran `cd src-rust && cargo test -p claurst-query capability -- --list`
- Result: `PASS`, but `0 tests` matched the current filter.
- Ran `cd src-rust && cargo test -p claurst-query provider_resolution -- --list`
- Result: `PASS`; current provider-resolution module exposes 22 unit tests.
- Best narrow crate-test command for this ticket in live repo reality:
- Before new tests exist: `cd src-rust && cargo test -p claurst-query provider_resolution`
- After adding capability-focused test names: `cd src-rust && cargo test -p claurst-query capability`

## Drift Found

- The repository is not clean before M11-03 execution. `git status --short` shows unrelated modified and untracked paths, including `.gitignore`, many report artifacts, `.codex` directories, and `src-rust/target/`.
- This dirt does not structurally block M11-03, but it does matter for later patch-isolation review hygiene.
- The user prompt described canonical M11-03 wording as model-entry-centric. Live authority is slightly broader: the MPWO/M10 plan also call for `provider_supports_capability()` and a default required-capabilities constant in the same file.
- The authority validation filter `cargo test -p claurst-query capability` is not yet meaningful against the live baseline because no test names currently contain `capability`.

## Blockers

None.

## Hosted Ollama Compatibility Baseline Preserved

`yes`

Basis:

- Live branch/HEAD still match the accepted M11-02 baseline.
- This preflight pass made no code changes.
- `provider_resolution.rs` already contains hosted-Ollama normalization coverage; M11-03 can proceed in the same file, but later execution must preserve those existing tests and behaviors.

## Exact Recommendation For Next Step

Proceed to the execution prompt for `TASK-M11-03` with scope limited to `src-rust/crates/query/src/provider_resolution.rs` only:

- add `Capability`
- add the default required-capabilities constant
- add `model_supports_capability(entry: &ModelEntry, cap: &Capability) -> Option<bool>`
- add `provider_supports_capability(caps: &ProviderCapabilities, cap: &Capability) -> bool`
- add narrowly scoped unit tests in the existing `provider_resolution` test module, naming the new tests with `capability` so the authority filter becomes real
- validate with `cd src-rust && cargo check -p claurst-query` and `cd src-rust && cargo test -p claurst-query capability`
