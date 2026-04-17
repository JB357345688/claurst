# TASK-M7-06 Review Report

- Ticket ID: `TASK-M7-06`
- Verdict: `ACCEPTABLE-WITH-NOTES`
- Pass/Fail: `PASS`
- Ready to close: `yes`
- Branch: `feature/provider-resolution-seam`

## Working Tree Summary

- Review basis: active unstaged diff for `src-rust/crates/query/src/provider_resolution.rs`
- Staged changes: none
- Modified tracked files:
  - `src-rust/crates/query/src/provider_resolution.rs`
- Untracked repo noise present:
  - `docs/`
  - `src-rust/target/`

Assessment:

- The active review patch is scope-clean for TASK-M7-06 as currently represented by the unstaged diff on `provider_resolution.rs`.
- Untracked `docs/` and `src-rust/target/` noise exists, but it does not contaminate the ticket diff under review.

## Authority Reviewed

- `/home/jordi/claurst/AGENTS.md`
- `/home/jordi/claurst/docs/Current/MPWO_WORK_ORDER_PACK.md`

MPWO items re-verified:

- TASK-M7-06 objective: unit tests for `materialize_provider()` covering auth-store refresh/materialization behavior, `api_base` overrides, and missing credentials
- Strict constraints:
  - modify only the existing `#[cfg(test)]` module target
  - do not modify production code
  - do not add network-dependent tests
  - do not retest `resolve_provider_identity()`
- Definition of done:
  - at least 2 materialize tests exist
  - tests pass or are `#[ignore]`-gated with clear explanation
- Validation command:
  - `cd src-rust && cargo test -p claurst-query -- provider_resolution`
- Stop/escalate condition:
  - if `ProviderRegistry` could not be constructed without real API keys, tests should be `#[ignore]` rather than inventing new abstractions
- Hosted-Ollama invariant report requirement:
  - review output must explicitly state whether the hosted Ollama baseline was preserved or intentionally changed by explicit ticket scope

## Files Reviewed

- `src-rust/crates/query/src/provider_resolution.rs`
- `src-rust/crates/core/src/auth_store.rs`

## Scope Compliance Assessment

Result: compliant.

Evidence:

- The diff is confined to the existing `#[cfg(test)] mod tests` in `src-rust/crates/query/src/provider_resolution.rs`.
- Added imports are test-only.
- Added helper:
  - `provider_identity(...)`
- Added tests:
  - `materialize_provider_returns_openai_target_from_happy_path`
  - `materialize_provider_returns_no_credentials_for_unknown_provider`
  - `materialize_provider_accepts_ollama_api_base_override`
- No production lines in `materialize_provider(...)`, `build_ollama_provider(...)`, or `normalize_ollama_api_base(...)` were modified.
- No unrelated pre-existing tests were changed.

## Behavior Coverage Assessment

Result: adequate and aligned to MPWO.

Coverage mapping:

- Registry provider found / happy path:
  - `materialize_provider_returns_openai_target_from_happy_path`
  - Review assessment: materially correct. It verifies success and `target.provider_id == "openai"` without over-asserting provider provenance. This is appropriate because ambient credentials could satisfy runtime-provider resolution before injected-registry fallback.
- Missing credentials error path:
  - `materialize_provider_returns_no_credentials_for_unknown_provider`
  - Review assessment: materially correct for current repo reality. `some-fake-provider` has no runtime provider factory path and an empty registry, so `ProviderResolutionError::NoCredentials(...)` is the correct assertion surface. The test uses `matches!(...)`, which is the correct pattern because the error enum does not implement `PartialEq`.
- Ollama `api_base` override happy path:
  - `materialize_provider_accepts_ollama_api_base_override`
  - Review assessment: appropriately narrow. It verifies successful construction and target identity without claiming direct inspection of the applied base URL through `Arc<dyn LlmProvider>`, which current public APIs do not expose cleanly.

No test expectation appears to paper over a production bug merely to satisfy the ticket.

## Production-Code Drift Assessment

Result: no production-code drift found.

Evidence:

- `materialize_provider(...)` remains unchanged at `src-rust/crates/query/src/provider_resolution.rs:157-215`
- `build_ollama_provider(...)` remains unchanged at `src-rust/crates/query/src/provider_resolution.rs:218-235`
- `normalize_ollama_api_base(...)` remains unchanged at `src-rust/crates/query/src/provider_resolution.rs:237-252`
- The review diff contains only test-module additions

## Hosted Ollama Compatibility Regression Assessment

Hosted Ollama compatibility baseline preserved

Assessment:

- TASK-M7-06 appears neutral to the accepted hosted Ollama baseline from `5f8dfe1`
- No code under review touches or weakens:
  - `normalize_ollama_api_base(...)`
  - Ollama auth lookup via `AuthStore::load().api_key_for(ProviderId::OLLAMA)`
  - environment-first precedence for `AuthStore::api_key_for("ollama")`
  - existing hosted Ollama normalization tests

Evidence:

- `normalize_ollama_api_base(...)` is still present in `src-rust/crates/query/src/provider_resolution.rs`
- Ollama auth lookup remains at `src-rust/crates/query/src/provider_resolution.rs:230`
- Environment-first Ollama precedence remains at `src-rust/crates/core/src/auth_store.rs:139-146`
- Existing hosted Ollama normalization tests remain present at `src-rust/crates/query/src/provider_resolution.rs:315-347`

Concern level:

- none

## Validation Command Run

```bash
cd /home/jordi/claurst/src-rust && cargo test -p claurst-query -- provider_resolution
```

## Validation Result

Result: passed.

- Filtered provider-resolution test set passed
- Observed result: `18 passed; 0 failed; 0 ignored`
- Observed warning:
  - unrelated unused import warning in `crates/query/src/compact.rs:1193`
- Ticket relevance of warning:
  - unrelated to TASK-M7-06

## Acceptance Recommendation

Recommendation: accept TASK-M7-06 and treat it as ready to close, subject to the normal human acceptance gate.

Basis:

- MPWO scope satisfied
- Validation passed
- No production-code drift found
- No hosted Ollama compatibility regression concern found

## Notes / Concerns

- The review basis is the active unstaged diff in `provider_resolution.rs`; there are no staged changes, but unrelated untracked `docs/` and `src-rust/target/` content remains in the worktree.
- The OpenAI happy-path test does not prove whether success came from the injected registry or ambient runtime credentials. That is acceptable because MPWO required only successful materialization with `target.provider_id == "openai"`, and a stricter provenance assertion would be brittle.
- No edits were made to source files during review. Git state was not altered; only this required markdown review report was written.
