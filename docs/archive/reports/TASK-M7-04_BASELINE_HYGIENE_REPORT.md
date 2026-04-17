# TASK-M7-04 Baseline Hygiene Report

## Ticket

- `TASK-M7-04`

## Objective

- Establish a clean, explicit review basis for `TASK-M7-04` before implementation.
- Treat accepted prior-ticket content as baseline rather than part of the M7-04 delta.
- Exclude unrelated staged changes and repo noise from the M7-04 review basis.

## Review Basis Chosen

- Chosen basis: active unstaged diff only, against the current staged index as a synthetic accepted baseline.
- Synthetic baseline contents:
- `src-rust/crates/query/src/lib.rs` staged content is treated as accepted prior-ticket baseline.
- `src-rust/crates/query/src/provider_resolution.rs` staged content now includes the accepted M7-03 implementation and is treated as accepted baseline.
- Exclusion rule:
- `TASK-M7-04` review is not based on `git diff --cached`.
- `TASK-M7-04` review is based on future unstaged diff in `src-rust/crates/query/src/lib.rs` only, unless the ticket later requires another file explicitly.

## Files Intentionally Included In Active M7-04 Basis

- `src-rust/crates/query/src/lib.rs`

## Files Intentionally Excluded

- `src-rust/crates/query/src/provider_resolution.rs`
- `src-rust/crates/api/src/registry.rs`
- `RFC_PROVIDER_AWARE_WORKER_FABRIC.md`
- `RFC_PROVIDER_AWARE_WORKER_FABRIC_v2.md`
- `AGENTS.md`
- `CLAUDE.md`
- `GEMINI.md`
- `docs/`
- `mpwo-ticket-executor/`
- `src-rust/target/`
- Any other staged or untracked repo noise outside the active unstaged `lib.rs` delta

## Whether provider_resolution.rs Accepted M7-03 Content Is Now Baseline

- Yes.
- Action taken: staged `src-rust/crates/query/src/provider_resolution.rs` after confirming its only unstaged delta was the accepted M7-03 `materialize_provider()` body.
- Result: `git diff -- src-rust/crates/query/src/provider_resolution.rs` is now empty, and the staged file contains the accepted M7-03 implementation.

## Whether lib.rs Prior Accepted Content Is Now Baseline

- Yes.
- No source edit was needed.
- `src-rust/crates/query/src/lib.rs` already had no unstaged diff, so its current staged content is the explicit pre-M7-04 baseline for later active review.

## Whether Any Source Logic Changed

- No.
- No source file contents were edited.
- Only git index state changed: `src-rust/crates/query/src/provider_resolution.rs` was staged so accepted M7-03 content becomes baseline.

## Validation Commands Run

```bash
git status --short
git diff --name-only
git diff --cached --name-only
git diff -- src-rust/crates/query/src/lib.rs
git diff -- src-rust/crates/query/src/provider_resolution.rs
git diff --cached -- src-rust/crates/query/src/provider_resolution.rs
```

## Validation Results

### git status --short

```text
D  RFC_PROVIDER_AWARE_WORKER_FABRIC.md
D  RFC_PROVIDER_AWARE_WORKER_FABRIC_v2.md
M  src-rust/crates/api/src/registry.rs
M  src-rust/crates/query/src/lib.rs
A  src-rust/crates/query/src/provider_resolution.rs
?? AGENTS.md
?? CLAUDE.md
?? GEMINI.md
?? docs/
?? mpwo-ticket-executor/
?? src-rust/target/
```

### git diff --name-only

```text
```

### git diff --cached --name-only

```text
RFC_PROVIDER_AWARE_WORKER_FABRIC.md
RFC_PROVIDER_AWARE_WORKER_FABRIC_v2.md
src-rust/crates/api/src/registry.rs
src-rust/crates/query/src/lib.rs
src-rust/crates/query/src/provider_resolution.rs
```

### git diff -- src-rust/crates/query/src/lib.rs

```text
```

### git diff -- src-rust/crates/query/src/provider_resolution.rs

```text
```

### git diff --cached -- src-rust/crates/query/src/provider_resolution.rs

- Result: staged diff shows the full new module including the accepted `materialize_provider()` implementation, confirming accepted M7-03 content is now baseline in the index.

## Remaining Blocker, If Any

- None, provided M7-04 implementation and review continue to use the explicit review basis above.
- Practical constraint: keep M7-04 source edits unstaged during implementation so the active review patch remains the unstaged `lib.rs` delta against the staged synthetic baseline.

## Ready For M7-04 Execution

- Yes
