# POST-M8-05 Compaction Hardening Review Report

## Title

POST-M8-05 shared compaction/context-collapse provider-parity hardening review report

## Pass Name

`POST-M8-05 shared compaction/context-collapse provider-parity hardening`

## Timestamp UTC

`20260412T125252Z`

## Branch

`feature/provider-resolution-seam`

## Working Tree Status

- `git branch --show-current`: `feature/provider-resolution-seam`
- `git status --short --untracked-files=no`: two tracked modified files only
  - `src-rust/crates/query/src/lib.rs`
  - `src-rust/crates/query/src/compact.rs`
- `git diff --name-only`: current active tracked diff is limited to exactly those same two files
- `git diff --cached --name-only`: no staged tracked changes
- Pre-existing untracked noise exists under tolerated locations including `.codex/`, `docs/Current/`, `docs/archive/reports/`, `docs/Orchestrator_planning/`, `docs/archive/provider_orchestrator/`, and `src-rust/target/`
- Review basis: active unstaged diff in exactly two tracked files, with unrelated untracked noise excluded from the patch basis

## Authority Reconfirmed

- Reread `/home/jordi/claurst/AGENTS.md`
- Reread the relevant MPWO invariant sections in `/home/jordi/claurst/docs/Current/MPWO_WORK_ORDER_PACK.md`
- Reconfirmed this is an orchestrator-authorized post-M8-05 hardening extension, not a normal M8 ladder ticket
- Review criteria applied:
  - patch should stay in exactly:
    - `src-rust/crates/query/src/lib.rs`
    - `src-rust/crates/query/src/compact.rs`
  - `run_query_loop(client: &AnthropicClient, ...)` signature must remain unchanged
  - legacy `provider_registry=None` Anthropic path must remain unchanged
  - provider resolution/materialization must remain centralized in `run_query_loop()`
  - no fallback behavior
  - no api crate changes
  - no full provider-branch tail unification
  - no unrelated stop-hook, session-memory, autodream, tool-ordering, or other post-turn redesign

## Actual Diff Review

- `run_query_loop()` signature remains unchanged at `src-rust/crates/query/src/lib.rs`
- Provider resolution and materialization still occur inside `run_query_loop()` in the registry-backed branch
- The new compaction decision point was added inside the registry-backed branch after assistant message capture/push and before the existing tool-use handling
- `compact.rs` gained provider-aware helper entrypoints rather than provider resolution logic
  - `summarise_head_with_provider(...)`
  - `compact_conversation_with_provider(...)`
  - `auto_compact_if_needed_with_provider(...)`
  - `reactive_compact_with_provider(...)`
  - `context_collapse_with_provider(...)`
- `compact.rs` does not call `resolve_provider_identity(...)` or `materialize_provider(...)`
- No tracked source modifications were found outside the intended two files
- No api crate changes were present in the active tracked diff
- No full provider-branch tail unification was introduced
- No fallback behavior was added by this hardening diff
- No unrelated stop-hook, session-memory, autodream, tool-ordering, or broader post-turn redesign was introduced by this hardening diff

## Drift Versus Execution Report

- Current repo reality matches the execution report at `/home/jordi/claurst/docs/archive/reports/POST_M8_05_COMPACTION_HARDENING_EXECUTION_REPORT_20260412T124625Z.md`
- No drift was found between the claimed two-file hardening scope and the current tracked diff
- No drift was found on the key assertions:
  - `run_query_loop()` signature unchanged
  - legacy `provider_registry=None` Anthropic path preserved
  - provider resolution/materialization remained centralized in `run_query_loop()`
  - provider-aware compaction entrypoints were added in `compact.rs`
  - registry-backed compaction/context-collapse path is now provider-aware

## Findings

- No review findings were identified in the active two-file hardening diff relative to the stated orchestrator constraints

## Exact Violations

- None found

## Minimal Corrective Actions

- None

## Residual Risks And Testing Gaps

- This review pass was read-only and did not rerun validation commands
- Closure confidence depends on the previously reported execution-pass validation remaining representative of the current unstaged two-file diff
- The diff does not add dedicated tests for the new provider-aware compaction helper entrypoints; residual risk is limited to runtime path coverage rather than scope compliance

## Hosted Ollama Invariant Outcome

Hosted Ollama compatibility baseline preserved

## Review Decision

- pass/fail: PASS
- ready to close: yes
- closure basis: the active tracked diff is scope-clean, matches the hardening intent, preserves the required legacy and signature invariants, and shows no repo-reality drift from the execution report
