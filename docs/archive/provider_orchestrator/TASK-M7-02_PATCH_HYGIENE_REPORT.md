# Ticket

TASK-M7-02

# Objective

Make the patch review-clean for TASK-M7-02 by:

- making `src-rust/crates/query/src/provider_resolution.rs` tracked and visible in the active diff
- splitting unrelated tracked changes out of the active M7-02 patch
- preserving existing source contents without changing resolver logic

# Files Intentionally Included In Final Patch

- `src-rust/crates/query/src/provider_resolution.rs`

# Files Intentionally Excluded From Final Patch

- `src-rust/crates/api/src/registry.rs`
- `RFC_PROVIDER_AWARE_WORKER_FABRIC.md`
- `RFC_PROVIDER_AWARE_WORKER_FABRIC_v2.md`
- `src-rust/crates/query/src/lib.rs`

# Whether provider_resolution.rs Is Now Tracked

Yes.

`src-rust/crates/query/src/provider_resolution.rs` was marked with `git add -N` so it is now tracked as intent-to-add and appears in the active review diff without changing its contents.

# Whether lib.rs Remains And Why

`src-rust/crates/query/src/lib.rs` does not remain in the active TASK-M7-02 review patch.

Reason:

- its visible change is the previously introduced M7-01 module wiring (`pub mod provider_resolution;` and `pub use provider_resolution::*;`)
- no M7-02 logic work was needed in `lib.rs`
- to enforce single-ticket review hygiene, it was split out of the active patch by git state only, with source content left unchanged

# Whether Any Source Logic Changed

No.

No resolver logic changed.
No source file contents were edited as part of this cleanup.
Only git tracking/staging state changed.

# Validation Commands Run

```bash
git status --short
git diff --name-only
git diff --cached --name-only
```

# Validation Results

`git status --short`:

```text
D  RFC_PROVIDER_AWARE_WORKER_FABRIC.md
D  RFC_PROVIDER_AWARE_WORKER_FABRIC_v2.md
M  src-rust/crates/api/src/registry.rs
M  src-rust/crates/query/src/lib.rs
 A src-rust/crates/query/src/provider_resolution.rs
?? AGENTS.md
?? CLAUDE.md
?? GEMINI.md
?? TASK-M7-02_REVIEW_REPORT.md
?? docs/
?? mpwo-ticket-executor/
?? src-rust/target/
```

`git diff --name-only`:

```text
src-rust/crates/query/src/provider_resolution.rs
```

`git diff --cached --name-only`:

```text
RFC_PROVIDER_AWARE_WORKER_FABRIC.md
RFC_PROVIDER_AWARE_WORKER_FABRIC_v2.md
src-rust/crates/api/src/registry.rs
src-rust/crates/query/src/lib.rs
```

Interpretation:

- the active unstaged review diff is now limited to `src-rust/crates/query/src/provider_resolution.rs`
- unrelated tracked changes were split out of the active M7-02 patch
- no cargo validation was run because no source code content changed during cleanup

# Remaining Blocker, If Any

None for patch hygiene.

There are still unrelated staged and untracked files in the worktree, but they are no longer part of the active M7-02 review diff.

# Ready To Re-review

Yes
