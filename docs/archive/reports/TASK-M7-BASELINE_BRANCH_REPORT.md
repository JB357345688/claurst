# TASK-M7 Baseline Branch Report

## Objective

- Create a clean feature branch for the accepted provider-resolution seam baseline.
- Make accepted M7-01, M7-02, and M7-03 provider-resolution seam content real git baseline.
- Exclude unrelated staged and untracked work from the baseline commit so `TASK-M7-04` can begin from a clean, explicit baseline.

## Branch Created

- `feature/provider-resolution-seam`

## Baseline Commit Hash

- `a09b3daefe887f2794c9fc2154afd8ebc8b3ec27`
- Commit subject: `Establish provider resolution seam baseline`

## Files Included In Baseline Commit

- `src-rust/crates/query/src/lib.rs`
- `src-rust/crates/query/src/provider_resolution.rs`

## Files Explicitly Excluded

- `src-rust/crates/api/src/registry.rs`
- `RFC_PROVIDER_AWARE_WORKER_FABRIC.md`
- `RFC_PROVIDER_AWARE_WORKER_FABRIC_v2.md`
- `AGENTS.md`
- `CLAUDE.md`
- `GEMINI.md`
- `docs/`
- `mpwo-ticket-executor/`
- `src-rust/target/`
- Any other unrelated staged or untracked repo noise

## Whether provider_resolution.rs Is Now Real Git Baseline

- Yes.
- `src-rust/crates/query/src/provider_resolution.rs` is now committed on `feature/provider-resolution-seam` in commit `a09b3daefe887f2794c9fc2154afd8ebc8b3ec27`.
- This includes accepted content through M7-03, including `resolve_provider_identity()` and `materialize_provider()`.

## Whether lib.rs Prior Accepted Content Was Included And Why

- Yes.
- The committed `lib.rs` delta is limited to accepted prior-ticket seam wiring:
- `pub mod provider_resolution;`
- `pub use provider_resolution::*;`
- Reasoning: this is accepted M7-01 baseline scaffolding required for the new module to exist in real git history.
- No M7-04 logic was included. The inline resolution block, `use_provider_dispatch`, inline materialization block, and no-registry Anthropic path were not changed by this task.

## Whether Any Source Logic Changed

- No new source logic was authored in this task.
- The task only recorded already accepted provider-resolution seam content into git history on a dedicated branch.
- No source-file text was edited during this task; only git branch and commit state changed.

## Validation Commands Run

```bash
git status --short
git diff --name-only
git diff --cached --name-only
git branch --show-current
git log --oneline -1
git diff-tree --no-commit-id --name-only -r HEAD
```

## Validation Results

### git status --short

```text
D  RFC_PROVIDER_AWARE_WORKER_FABRIC.md
D  RFC_PROVIDER_AWARE_WORKER_FABRIC_v2.md
M  src-rust/crates/api/src/registry.rs
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
```

### git branch --show-current

```text
feature/provider-resolution-seam
```

### git log --oneline -1

```text
a09b3da Establish provider resolution seam baseline
```

### git diff-tree --no-commit-id --name-only -r HEAD

```text
src-rust/crates/query/src/lib.rs
src-rust/crates/query/src/provider_resolution.rs
```

## Ready For M7-04 Execution

- Yes.
- The accepted provider-resolution seam is now real git baseline on `feature/provider-resolution-seam`.
- Remaining unrelated staged files are still present in the working tree/index, but they were excluded from the baseline commit and must remain outside the M7-04 review basis.
