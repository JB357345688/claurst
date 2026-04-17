# POST-M8-05 Compaction Hardening Commit Verification Report

## Title

POST-M8-05 shared compaction/context-collapse provider-parity hardening commit verification report

## Pass Name

`POST-M8-05 shared compaction/context-collapse provider-parity hardening`

## Timestamp UTC

`20260412T132915Z`

## Branch

`feature/provider-resolution-seam`

## Repo State Observed

- `git branch --show-current`: `feature/provider-resolution-seam`
- `git status --short` showed no tracked modifications and no staged tracked changes
- `git diff --name-only`: empty
- `git diff --cached --name-only`: empty
- Only tolerated untracked noise was present under `.codex/`, `docs/Current/`, `docs/archive/reports/`, `docs/Orchestrator_planning/`, `docs/archive/provider_orchestrator/`, and `src-rust/target/`

## Commit Presence Check

- hardening closeout commit already existed
- Matching commit hash:
  - `ced600545fb3517c9995f022d7772ec5fe5f514d`
- One-line subject:
  - `Harden provider-aware compaction for post-M8-05 registry-backed runs`
- `git show --stat` summary:
  - `ced6005 Harden provider-aware compaction for post-M8-05 registry-backed runs`
  - `docs/archive/reports/POST_M8_05_COMPACTION_HARDENING_CLOSEOUT_REPORT_20260412T130308Z.md | 86 +++++`
  - `src-rust/crates/query/src/compact.rs | 380 ++++++++++++++++++++-`
  - `src-rust/crates/query/src/lib.rs | 87 +++++`
  - `3 files changed, 551 insertions(+), 2 deletions(-)`

## Reviewed Basis Recheck

- No active tracked source diff remains in the repo: confirmed
- `run_query_loop(client: &AnthropicClient, ...)` signature remains present in `src-rust/crates/query/src/lib.rs`: confirmed
- Provider resolution/materialization remains centralized in `run_query_loop()`: confirmed
- Provider-aware compaction helper entrypoints remain present in `src-rust/crates/query/src/compact.rs`: confirmed
  - `summarise_head_with_provider(...)`
  - `compact_conversation_with_provider(...)`
  - `auto_compact_if_needed_with_provider(...)`
  - `reactive_compact_with_provider(...)`
  - `context_collapse_with_provider(...)`
- No contrary tracked drift was found against the known reviewed basis for:
  - legacy `provider_registry=None` Anthropic preservation
  - no fallback behavior
  - no api crate changes
  - no full provider-branch tail unification

## Validation Commands Run

- None in this verification pass

## Validation Results

- Not rerun in this pass because the hardening closeout commit already existed and the tracked repo state was clean

## Commit Outcome

- No new commit was created in this pass
- The required closeout commit was already present at `HEAD`
- This hardening pass is now fully closed: yes

## Hosted Ollama Invariant Outcome

Hosted Ollama compatibility baseline preserved

## Next-step note

- No later M8 or other work was started
- No duplicate closeout commit was created

## Verdict

- Verification verdict: PASS
