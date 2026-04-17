# TASK-M11-07 Preflight Report

## Ticket ID

`TASK-M11-07 — SessionBudget implementation`

## Verdict

`GO`

## Current Branch

`feature/provider-resolution-seam`

## Current HEAD

`cf8201fefaa95585e5910eda87f83fdcc7d67663`

## Authority Files Reviewed

- `AGENTS.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md`
- `docs/archive/reports/M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md`
- `docs/archive/reports/TASK-M11-06_CLOSEOUT_REPORT_20260414T150212Z.md`

## Verified File Paths

- `src-rust/crates/query/src/lib.rs` exists and is the crate root for `claurst-query`.
- `src-rust/crates/query/src/session_budget.rs` does not exist yet.
- `src-rust/crates/query/Cargo.toml` exists and directly includes `parking_lot` and `tokio-util`.
- `src-rust/crates/tools/src/lib.rs` exists and `ToolContext` does not yet include any session-budget field.
- `src-rust/crates/tools/src/team_tool.rs` exists and still owns team-local cancellation tokens.
- `src-rust/crates/query/src/agent_tool.rs` exists and still creates fresh child cancellation tokens.
- `src-rust/crates/cli/src/main.rs` exists and is the current root owner of `max_budget_usd` CLI wiring.

## Verified Symbols / Repo Facts

- `claurst-query` currently declares `pub mod health_cache;` and `pub use health_cache::*;` in `src-rust/crates/query/src/lib.rs`.
- `claurst-query` does not currently declare `pub mod session_budget;` or `pub use session_budget::*;`.
- `QueryConfig` currently contains `max_budget_usd: Option<f64>` and does not contain `session_budget`.
- `run_query_loop(...)` already accepts a `tokio_util::sync::CancellationToken`.
- `QueryOutcome` already contains `BudgetExceeded { cost_usd, limit_usd }`.
- `QueryEvent` does not yet contain any session-budget-specific variants.
- `parking_lot = { workspace = true }` is already a direct dependency of `claurst-query`.
- `tokio-util = { workspace = true }` is already a direct dependency of `claurst-query`.
- Current `CostTracker` state is post-M11-06 reality: attribution fields are present in `src-rust/crates/core/src/lib.rs`.

## Scope / Ownership Reality

- The authority file targets for M11-07 match live repo structure:
  - new file: `src-rust/crates/query/src/session_budget.rs`
  - module wiring: `src-rust/crates/query/src/lib.rs`
- No live file-path drift was found for the intended crate ownership.
- A minimal implementation pass can remain limited to exactly those two Rust files.
- No M11-08 wiring is required to land the standalone utility type itself.

## Existing Budget / Cancellation Reality

### Existing budget behavior

- Root-session budget currently lives in `QueryConfig.max_budget_usd`.
- CLI wiring sets it in `src-rust/crates/cli/src/main.rs` when `--max-budget-usd` is provided.
- `run_query_loop()` checks `config.max_budget_usd` after usage is added to `CostTracker` and returns `QueryOutcome::BudgetExceeded` when total tracked spend meets or exceeds the per-loop limit.
- This is a per-loop mechanism, not a cross-session/shared-root mechanism.

### Existing cancellation behavior

- `run_query_loop()` already respects the passed `CancellationToken`.
- Query crate child/runner paths still create fresh, unconnected tokens:
  - background agent path in `src-rust/crates/query/src/agent_tool.rs`
  - foreground sub-agent path in `src-rust/crates/query/src/agent_tool.rs`
  - team runner injection path in `src-rust/crates/query/src/agent_tool.rs`
- `src-rust/crates/tools/src/team_tool.rs` separately creates one fresh `CancellationToken` per team agent for `TeamDeleteTool`.
- Query-side helper consumers such as `compact.rs`, `away_summary.rs`, and `cron_scheduler.rs` use cancellation tokens passed to them; they do not define a competing root-budget abstraction.

### Overlap / conflict assessment

- There is no existing `SessionBudget` type or equivalent budget helper in `claurst-query`.
- The only overlapping budget concept is `max_budget_usd`, and its semantics are distinct:
  - `max_budget_usd`: per-loop spend cap
  - `SessionBudget`: planned cross-session/shared-root spend cap with child-token propagation
- This distinction is coherent with live code and does not force wider changes in M11-07.
- The authority choice between `AtomicF64` and mutex is not a blocker. Given current repo reality, `parking_lot::Mutex<f64>` is the narrowest fit:
  - `parking_lot` is already a direct dependency in this crate
  - the crate already uses `parking_lot` broadly
  - no extra atomics or float bit-casting machinery is needed

## Existing Test Reality

- `claurst-query` places unit tests inline in the owning module file via `#[cfg(test)] mod tests`.
- Relevant precedent: `src-rust/crates/query/src/health_cache.rs` is a newly added utility module with inline unit tests in the same file.
- Both plain `#[test]` and async `#[tokio::test]` are already used in this crate.
- For M11-07, the natural placement is inline tests inside the new `session_budget.rs`.
- Child-token propagation can be tested deterministically with current dependencies:
  - synchronous path: create root, create child, call `check_and_cancel()` / `cancel()`, assert `is_cancelled()`
  - async path is also available if desired because `#[tokio::test]` is already established in-crate

## Module-Wiring Reality

- `src-rust/crates/query/src/lib.rs` will need only the standard module exports:
  - `pub mod session_budget;`
  - `pub use session_budget::*;`
- No additional `QueryConfig`, `ToolContext`, CLI, or agent wiring belongs in M11-07.
- Those wider integrations remain M11-08+ scope.

## Validation Commands Verified

- `cd src-rust && cargo check -p claurst-query`
  - Result: `PASS`
- `cd src-rust && cargo test -p claurst-query -- session_budget`
  - Result: `PASS` as a valid narrow filter command
  - Current behavior: `0` tests run because `session_budget` tests do not exist yet
  - Note: this run emitted one unrelated existing warning from `crates/query/src/compact.rs` about an unused `Role` import

## Independence Check Against Closed Tickets

- Current branch and HEAD exactly match the accepted M11-06 closeout baseline:
  - branch: `feature/provider-resolution-seam`
  - HEAD: `cf8201fefaa95585e5910eda87f83fdcc7d67663`
- No relevant live drift was found in the closed M11-01 through M11-06 code areas that would force M11-07 to widen.
- M11-07 remains independent from the already-closed behavior. The standalone utility can land before any M11-08 wiring.

## Drift Found

- `docs/Current/MPWO_WORK_ORDER_PACK.md` is structurally unexpected: it currently contains an MPWO revision report summary, not the full detailed ticket pack body. This is authority-document drift, but not execution-blocking for this ticket because:
  - the active prompt restates the M11-07 ticket scope
  - the accepted M10 D2 plan contains the detailed M11-07 file targets, method shape, and validation command
- The worktree is dirty with unrelated changes and many untracked report/doc artifacts, including `.gitignore`, `docs/Current/`, `docs/archive/reports/`, and `src-rust/target/`.
- `src-rust/crates/query/src/lib.rs` already contains `health_cache` from prior tickets, so M11-07 must add `session_budget` without disturbing the accepted module baseline.
- Narrow validation already exposes one unrelated warning in `src-rust/crates/query/src/compact.rs`; this is non-blocking for M11-07 preflight.

## Blockers

- None for `TASK-M11-07` itself.

## Hosted Ollama Compatibility Baseline Preserved

`yes`

Basis:

- Live branch and HEAD still match the accepted M11-06 baseline.
- M11-07 can remain a query-crate-local utility addition with no provider-resolution or hosted-Ollama path changes.
- This preflight pass made no runtime-code changes.

## Exact Recommendation For Next Step

Proceed with a narrow execution pass for `TASK-M11-07` only:

1. Add `src-rust/crates/query/src/session_budget.rs` implementing `SessionBudget` with:
   - `budget_usd: f64`
   - `spent: parking_lot::Mutex<f64>`
   - `root_token: tokio_util::sync::CancellationToken`
   - methods `new`, `record_cost`, `check_and_cancel`, `child_cancel_token`, `is_cancelled`
2. Add only `pub mod session_budget;` and `pub use session_budget::*;` to `src-rust/crates/query/src/lib.rs`.
3. Keep tests inline in `session_budget.rs`; cover:
   - cost accumulation
   - threshold-triggered cancellation
   - child token propagation
4. Do not touch `QueryConfig`, `ToolContext`, `agent_tool.rs`, `team_tool.rs`, or CLI wiring in this ticket.
5. Preserve patch hygiene explicitly because the repo is already dirty outside ticket scope.
