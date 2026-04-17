# TASK-M7-05 Preflight Report

- Ticket ID: `TASK-M7-05`
- Verdict: `READY-WITH-NOTES`
- Current branch: `feature/provider-resolution-seam`

## Git Working Tree Summary

- Staged changes: none
- Unstaged tracked changes: none
- Untracked files/directories present, including `AGENTS.md`, `CLAUDE.md`, `GEMINI.md`, `docs/`, `mpwo-ticket-executor/`, and `src-rust/target/`
- Git state was inspected only; no git state was altered

## Baseline Commit Verification Results

- `a09b3da` (`establish provider-resolution seam baseline`): present and contained in current branch history
- `5881983` (`TASK-M7-04 wire run_query_loop through provider resolution seam`): present and contained in current branch history
- `255e3c7` (`cleanup remove obsolete provider worker fabric RFCs`): present and contained in current branch history
- `5f8dfe1` (`fix hosted Ollama compatibility on provider seam`): present and contained in current branch history

## Verified Authority Sources

- `/home/jordi/claurst/AGENTS.md`
- `/home/jordi/claurst/docs/Current/MPWO_WORK_ORDER_PACK.md`
- `TASK-M7-05` located at `docs/Current/MPWO_WORK_ORDER_PACK.md:516`

## Ticket Scope Verification

- Objective: add unit tests covering precedence rows `P1-P12` for `resolve_provider_identity()`
- Preconditions from MPWO: `TASK-M7-02` complete
- Exact code target from MPWO: `src-rust/crates/query/src/provider_resolution.rs` only, test module at bottom of file
- Strict constraints verified from MPWO:
  - do not add integration or network tests
  - do not create mock providers
  - do not modify production code in this ticket
  - do not test `materialize_provider()` in this ticket
- Definition of done from MPWO:
  - 12 unit tests exist in `provider_resolution.rs`
  - `cd src-rust && cargo test -p claurst-query -- provider_resolution` passes
- Stop/escalate conditions from MPWO:
  - if `ModelRegistry` lacks simple test setup, investigate API and keep scope narrow
  - if behavior disagrees with expected outputs, investigate implementation instead of changing expectations

## Verified Files

- `src-rust/crates/query/src/provider_resolution.rs`
- `src-rust/crates/api/src/model_registry.rs`
- `src-rust/crates/api/src/lib.rs`
- `src-rust/Cargo.toml`
- `src-rust/crates/query/Cargo.toml`

## Verified Symbols

- `ResolutionSource` exists at `provider_resolution.rs:50`
- `ProviderIdentity` exists at `provider_resolution.rs:58`
- `ProviderResolutionError` exists at `provider_resolution.rs:84`
- `ProviderResolutionError::ProviderModelConflict` exists at `provider_resolution.rs:92`
- `resolve_provider_identity(...)` exists at `provider_resolution.rs:101` with signature:
  - `fn resolve_provider_identity(explicit_provider: Option<&str>, model: &str, model_registry: Option<&ModelRegistry>) -> Result<ProviderIdentity, ProviderResolutionError>`
- `resolve_provider_identity()` is implemented, not `todo!()`
- Existing `#[cfg(test)] mod tests` already exists at `provider_resolution.rs:255`; current tests only cover `normalize_ollama_api_base`, not provider resolution
- `ModelRegistry` lives at `src-rust/crates/api/src/model_registry.rs:50`
- `ModelRegistry::new()` exists at `model_registry.rs:63`
- `ModelRegistry::find_provider_for_model()` exists at `model_registry.rs:208`
- `ModelRegistry` is re-exported from `src-rust/crates/api/src/lib.rs:80`

## Verified Commands

- Validation command from MPWO: `cd src-rust && cargo test -p claurst-query -- provider_resolution`
- Command remains plausible in current repo reality because:
  - `src-rust/` exists and is the Rust workspace root
  - workspace member `crates/query` exists in `src-rust/Cargo.toml`
  - package name `claurst-query` is declared in `src-rust/crates/query/Cargo.toml:2`
- Validation command was inspected only in preflight; tests were not run

## Preconditions Status

- `TASK-M7-02` precondition satisfied:
  - `resolve_provider_identity()` exists and is implemented in `provider_resolution.rs:101-155`
- Accepted `TASK-M7-04` baseline is corroborated:
  - commit `5881983` is in current branch history
  - current production call site exists at `src-rust/crates/query/src/lib.rs:860`

## Drift Found

- Classification: `line drift only`
- Details:
  - MPWO expects a new bottom-of-file test module, but repo reality already has a bottom-of-file `#[cfg(test)]` module with unrelated `normalize_ollama_api_base` tests
  - execution should extend the existing test module rather than add a second one

## Blockers

- None

## Notes For Execution Phase

- Execution scope remains tight: only `src-rust/crates/query/src/provider_resolution.rs` should be edited for this ticket
- `ModelRegistry` has a public `new()` constructor and `find_provider_for_model()` lookup, but no public registration mutator was found
- That does not block `TASK-M7-05`: the current `find_provider_for_model()` implementation already contains family heuristics for `gemini*` and `gemma*`, so `P8` is still testable with public API and `P9` can use an unknown bare model for the no-match path
- Existing untracked repo noise means the review basis should be stated explicitly during execution/closure

## Edit Status

- No source edits were made in this preflight
- Only this markdown report file was created
