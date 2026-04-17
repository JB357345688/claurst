# POST-M8-05 Compaction Hardening Execution Report

## Title

POST-M8-05 shared compaction/context-collapse provider-parity hardening execution report

## Pass Name

`POST-M8-05 shared compaction/context-collapse provider-parity hardening`

## Timestamp UTC

`20260412T124625Z`

## Branch

`feature/provider-resolution-seam`

## Working Tree Summary Before Execution

- `git branch --show-current` reconfirmed branch `feature/provider-resolution-seam`.
- `git status --short --untracked-files=no` showed no staged or unstaged tracked source drift before execution.
- Untracked noise was present and tolerated under `.codex/`, `docs/Current/`, `docs/archive/reports/`, `docs/Orchestrator_planning/`, `docs/archive/provider_orchestrator/`, and `src-rust/target/`.
- Review basis for this hardening pass is the active unstaged diff in exactly two tracked files.

## Authority Reconfirmed

- Reread `/home/jordi/claurst/AGENTS.md`.
- Reread the relevant invariants in `/home/jordi/claurst/docs/Current/MPWO_WORK_ORDER_PACK.md` covering preserved `run_query_loop(client: &AnthropicClient, ...)`, preserved legacy `provider_registry=None` behaviour, and the M8-04/M8-05 provider-resolution seam constraints.
- Reread `/home/jordi/claurst/docs/archive/reports/POST_M8_05_COMPACTION_HARDENING_PREFLIGHT_REPORT_20260412T122507Z.md`.
- Reconfirmed current repo reality still matched the preflight:
  - the registry-backed branch in `run_query_loop()` exited before the legacy shared compaction/context-collapse block
  - `compact.rs` still bound the shared compaction helpers directly to `&AnthropicClient`
  - provider resolution/materialization context already existed in `run_query_loop()`
- This remains a preventive hardening, not a live-defect hotfix.

## Exact Files Changed

- Code patch stayed in exactly these two files: yes
- Source files changed:
  - `src-rust/crates/query/src/lib.rs`
  - `src-rust/crates/query/src/compact.rs`
- Report artifact written:
  - `docs/archive/reports/POST_M8_05_COMPACTION_HARDENING_EXECUTION_REPORT_20260412T124625Z.md`
- `run_query_loop()` signature unchanged: yes

## Exact Changes Made

- In `src-rust/crates/query/src/lib.rs`, added a provider-branch-local compaction decision point immediately after assistant message capture/push and before the existing registry-branch tool-use handling.
- Kept provider resolution/materialization centralized in `run_query_loop()`: yes.
- Kept the legacy `provider_registry=None` Anthropic path unchanged: yes.
- Kept `run_query_loop(client: &AnthropicClient, ...)` unchanged: yes.
- Passed already-resolved provider execution context from `run_query_loop()` into provider-aware compaction helpers by reusing:
  - resolved provider instance
  - resolved model id
  - provider-specific options built from the already-resolved provider/model context
- In `src-rust/crates/query/src/compact.rs`, added provider-aware compaction/helper entrypoints:
  - `compact_conversation_with_provider(...)`
  - `auto_compact_if_needed_with_provider(...)`
  - `reactive_compact_with_provider(...)`
  - `context_collapse_with_provider(...)`
  - internal helper `summarise_head_with_provider(...)`
- The new provider-aware helpers consume already-resolved provider execution context and do not resolve providers or registries independently inside `compact.rs`.
- No fallback behaviour was added.

## Provider-Aware Compaction Path Recheck

- Registry-backed compaction/context-collapse now has a provider-aware path: yes.
- New registry-backed callsites were added in `src-rust/crates/query/src/lib.rs`:
  - `context_collapse_with_provider(...)`
  - `reactive_compact_with_provider(...)`
  - `auto_compact_if_needed_with_provider(...)`
- New provider-aware helper entrypoints were added in `src-rust/crates/query/src/compact.rs`:
  - `compact_conversation_with_provider(...)`
  - `auto_compact_if_needed_with_provider(...)`
  - `reactive_compact_with_provider(...)`
  - `context_collapse_with_provider(...)`
- The legacy later shared Anthropic tail remains intact for `provider_registry=None` execution.
- Post-M8-05 registry-backed runs now have a provider-aware compaction/context-collapse path without depending on raw `AnthropicClient` semantics in that branch.

## Validation Commands Run

- `cd /home/jordi/claurst/src-rust && cargo check -p claurst-query`

## Validation Results

- PASS
- `cargo check -p claurst-query` succeeded.

## Deviations From Intended Scope

- None.

## Blockers

- None.

## Hosted Ollama Invariant Outcome

- Hosted Ollama compatibility baseline preserved
- Registry-backed compaction/context-collapse now routes through the already-materialized provider path for this hardening seam and does not introduce new registries or fallback behaviour.

## Scope Compliance Assessment

- Patch stayed in exactly the two intended files: yes.
- `run_query_loop()` signature was unchanged: yes.
- Legacy `provider_registry=None` Anthropic behaviour was unchanged: yes.
- Provider resolution/materialization remained centralized in `run_query_loop()`: yes.
- No forbidden files were modified.
- No API-crate behaviour was changed.
- No full provider-branch tail unification was introduced.
- No stop-hook, session-memory, autodream, tool-ordering, or unrelated post-turn redesign was introduced.

## Next-step note

- Hardening pass is implemented and validated on the active unstaged two-file diff and is ready for human review.
- No commit was made.
