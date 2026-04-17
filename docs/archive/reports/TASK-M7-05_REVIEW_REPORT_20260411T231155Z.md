# TASK-M7-05 Review Report

- Ticket ID: `TASK-M7-05`
- Verdict: `ACCEPTABLE-WITH-NOTES`
- Pass/Fail: `PASS`
- Ready to close: `yes`
- Branch: `feature/provider-resolution-seam`

## Working Tree Summary

- Review basis: active unstaged diff in `src-rust/crates/query/src/provider_resolution.rs`
- Tracked modifications in review basis: `1` file
- Scoped diff summary: `179` insertions, `1` deletion in `provider_resolution.rs`
- Untracked repo noise present outside review basis: `AGENTS.md`, `CLAUDE.md`, `GEMINI.md`, `docs/`, `mpwo-ticket-executor/`, `src-rust/target/`
- Git state was inspected only; no git state was changed

## Authority Reviewed

- `/home/jordi/claurst/AGENTS.md`
- `/home/jordi/claurst/docs/Current/MPWO_WORK_ORDER_PACK.md`
- Exact ticket section: `TASK-M7-05 — Unit tests for resolve_provider_identity() (P1–P12)`

## Files Reviewed

- `src-rust/crates/query/src/provider_resolution.rs`
- `src-rust/crates/api/src/model_registry.rs`

## Ticket Requirements Reviewed

- Objective: add unit tests covering precedence rows `P1-P12` for `resolve_provider_identity()`
- Strict constraints:
  - no integration or network tests
  - no mock providers
  - no production code changes
  - do not test `materialize_provider()`
- Required validation command:
  - `cd /home/jordi/claurst/src-rust && cargo test -p claurst-query -- provider_resolution`
- Definition of done:
  - `12` unit tests exist in `provider_resolution.rs`
  - filtered `provider_resolution` test run passes
- Stop/escalate conditions reviewed:
  - if `ModelRegistry` setup is limited, document the limitation rather than broadening scope
  - if expectations disagree with behavior, investigate implementation rather than changing expected outputs

## Scope Compliance Assessment

- The active diff is confined to the existing `#[cfg(test)] mod tests` in `provider_resolution.rs`
- No production functions, enums, structs, or non-test logic were modified
- Existing `normalize_ollama_api_base` tests were preserved
- No unrelated test areas were changed beyond:
  - test-module imports
  - two local helper functions for repeated assertions
  - the `P1-P12` test additions

## P1-P12 Coverage Assessment

- `P1` present and coherent: explicit `openai` plus `openai/gpt-4o` asserts provider/model stripping and `ExplicitProvider`
- `P2` present and coherent: explicit `openai` plus bare `gpt-4o` asserts explicit-provider precedence
- `P3` present and coherent: explicit `openai` plus `anthropic/...` asserts exact `ProviderModelConflict`
- `P4` present and coherent: no explicit provider plus `google/...` asserts `ModelStringPrefix`
- `P5` present and coherent: reverse conflict case asserts exact `ProviderModelConflict`
- `P6` present and coherent: explicit anthropic pin with bare model asserts `ExplicitProvider`
- `P7` present and coherent: unknown namespace without registry asserts default anthropic path
- `P8` present and coherent with a note: uses `ModelRegistry::new()` and current public `find_provider_for_model()` behavior for `gemini-3-flash-preview`, asserting `ModelRegistry`
- `P9` present and coherent: registry present with unknown bare model asserts default fallback
- `P10` present and coherent: no registry plus bare anthropic model asserts default fallback
- `P11` present and coherent: explicit `openrouter` with nested-slash model asserts correct prefix stripping
- `P12` present and coherent: explicit `ollama` with bare model asserts explicit-provider precedence

## Production-Code Drift Assessment

- No production-code drift found
- The diff does not alter:
  - `resolve_provider_identity()`
  - `materialize_provider()`
  - `normalize_ollama_api_base()`
  - provider registry/runtime behavior
- No test expectation appears to paper over a production bug merely to make the ticket pass

## Hosted Ollama Compatibility Regression Assessment

- `TASK-M7-05` appears neutral to the accepted hosted Ollama compatibility baseline from `5f8dfe1`
- Existing `normalize_ollama_api_base` tests were not weakened or rewritten
- New helpers and new precedence tests do not encode assumptions about hosted vs local Ollama URL normalization
- No concern found that the M7-05 changes would indirectly weaken the hosted Ollama compatibility fix

## Validation Command Run

```bash
cd /home/jordi/claurst/src-rust && cargo test -p claurst-query -- provider_resolution
```

## Validation Result

- Pass
- Filtered test set result: `15 passed, 0 failed`
- Ticket-relevant result: all `P1-P12` tests passed
- Warning observed:
  - unrelated existing warning in `crates/query/src/compact.rs` for unused import `Role`
  - warning is not caused by the reviewed diff and does not affect ticket acceptance

## Acceptance Recommendation

- Recommendation: `ACCEPTABLE-WITH-NOTES`
- Exact violations: none
- Minimal corrective actions: none required for ticket acceptance

## Notes / Concerns

- `P8` does not construct an explicitly registered provider-model mapping because current public `ModelRegistry` API does not expose a public registration mutator
- Instead, `P8` exercises the registry resolution branch through the public `ModelRegistry::new()` plus current `find_provider_for_model()` behavior for `gemini-3-flash-preview`
- This is acceptable under the ticket's stop/escalate guidance, but it is a coupling note worth carrying forward into later registry-focused tickets

## Review Edit Status

- No source files were edited during this review
- No files were staged or committed during this review
- Only this markdown review report was created
