# TASK-M7-07 Review Report

- Ticket ID: `TASK-M7-07`
- Verdict: `ACCEPTABLE-WITH-NOTES`
- Pass/Fail: `PASS`
- Ready to close: `yes`
- Branch: `feature/provider-resolution-seam`

## Working Tree Summary

- Review basis from current repo state:
  - tracked unstaged diff: `src-rust/crates/query/src/lib.rs`
  - tolerated untracked noise: `.codex`, `docs/`, `src-rust/target/`
- No staged changes were present during review.
- No edits were made during review except creation of this review report artifact under `docs/archive/reports/`.

## Authority Reviewed

- `/home/jordi/claurst/AGENTS.md`
- `/home/jordi/claurst/docs/Current/MPWO_WORK_ORDER_PACK.md`
- MPWO section `TASK-M7-07 — Workspace validation`
- MPWO section `2A. Standing Non-Regression Invariant — Hosted Ollama Compatibility`

Extracted authority:

- Objective:
  - verify the workspace compiles, tests pass, and clippy is clean after M7 changes
- Strict constraints:
  - do not fix pre-existing clippy warnings in unrelated files
  - do not modify files outside those already modified in `M7-01` through `M7-06`
  - do not add features or surrounding cleanup
- Required validation commands:
  - `cd /home/jordi/claurst/src-rust && cargo fmt --all -- --check`
  - `cd /home/jordi/claurst/src-rust && cargo build --workspace`
  - `cd /home/jordi/claurst/src-rust && cargo test --workspace`
  - `cd /home/jordi/claurst/src-rust && cargo clippy --workspace --all-targets`
- Definition of done:
  - `build` succeeds
  - `test` passes
  - `clippy` has no new warnings from M7 code
  - `fmt --check` passes
- Stop / escalate conditions:
  - stop if workspace test failures are unrelated and outside M7-modified behavior
  - escalate if more than 3 files would need fixes
- Hosted-Ollama reporting requirement:
  - review report must explicitly state either `Hosted Ollama compatibility baseline preserved` or `Hosted Ollama compatibility baseline intentionally changed by explicit ticket scope`

## Files Reviewed

- `src-rust/crates/query/src/lib.rs`
- `src-rust/crates/query/src/provider_resolution.rs`
- `src-rust/crates/core/src/auth_store.rs`

## Scope Compliance Assessment

- The active tracked diff is confined to the already-modified M7 seam section inside `run_query_loop()`.
- The diff consists of exactly three seam-local clippy cleanups:
  - direct function reference replacing a redundant closure in provider thinking config
  - outer-pattern rewrite removing a collapsible nested `if let` in provider stream tool-call capture
  - borrow cleanup replacing explicit auto-deref / needless borrow in `execute_tool(...)`
- No edits broaden beyond the provider-dispatch seam.
- No unrelated cleanup was performed in `lib.rs`.
- No other source files were changed for TASK-M7-07 review basis.

## Validation Assessment

Validation rerun during review:

```bash
cd /home/jordi/claurst/src-rust && cargo fmt --all -- --check
cd /home/jordi/claurst/src-rust && cargo build --workspace
cd /home/jordi/claurst/src-rust && cargo test --workspace
cd /home/jordi/claurst/src-rust && cargo clippy --workspace --all-targets
```

Results:

- `cargo fmt --all -- --check`: `PASS`
- `cargo build --workspace`: `PASS`
- `cargo test --workspace`: `PASS`
- `cargo clippy --workspace --all-targets`: `PASS with warnings`

Assessment:

- All four required commands completed successfully.
- Remaining warnings are unrelated/pre-existing and outside authorized M7 cleanup scope.
- `claurst-query` still has unrelated warnings outside the reviewed seam section, including:
  - `src-rust/crates/query/src/lib.rs:2011` `items_after_test_module`
  - `src-rust/crates/query/src/lib.rs:1863` `unnecessary_map_or`
  - `src-rust/crates/query/src/compact.rs` warning family
- Those warnings are not part of the seam-local corrective patch and do not violate TASK-M7-07 because the definition of done is “no new warnings from M7 code,” not global zero-warning cleanup.

## Production-Code Drift Assessment

- The seam-local patch is minimal and behavior-neutral:
  - `effective_thinking_budget.map(claurst_api::ThinkingConfig::enabled)` is equivalent to the prior closure call
  - matching `ContentBlock::ToolUse` in the outer stream event arm preserves the same insertion behavior into `tool_call_blocks`
  - `execute_tool(tool_name.as_str(), &tool_input, tools, tool_ctx)` is semantically identical to the prior borrowed call
- No control-flow, provider selection, hosted-Ollama handling, auth lookup, or fallback behavior changed.
- No new production logic or cleanup outside the reported clippy issues was introduced.

## Hosted Ollama Compatibility Regression Assessment

Hosted Ollama compatibility baseline preserved

Verification basis:

- `normalize_ollama_api_base(...)` remains present in `src-rust/crates/query/src/provider_resolution.rs`.
- Ollama auth lookup via `AuthStore::load().api_key_for(ProviderId::OLLAMA)` remains present in `src-rust/crates/query/src/provider_resolution.rs`.
- Environment-first precedence for `AuthStore::api_key_for("ollama")` remains intact in `src-rust/crates/core/src/auth_store.rs`.
- `git diff -- src-rust/crates/query/src/provider_resolution.rs src-rust/crates/core/src/auth_store.rs` was empty during review.
- The active TASK-M7-07 diff does not touch hosted-Ollama-sensitive code paths.

## Acceptance Recommendation

- Classification: `ACCEPTABLE-WITH-NOTES`
- Recommendation:
  - TASK-M7-07 satisfies MPWO scope and validation requirements
  - the seam-local `lib.rs` fix stayed within the authorized corrective area
  - the active patch is scope-clean for review basis
  - ticket is ready for human closeout / acceptance

## Notes / Concerns

- Review basis depends on the current unstaged diff in `src-rust/crates/query/src/lib.rs`; that basis is explicit and scope-clean.
- Workspace clippy still emits many unrelated warnings outside ticket scope; they should not be folded into TASK-M7-07.
- No edits were made during review beyond this report artifact.
