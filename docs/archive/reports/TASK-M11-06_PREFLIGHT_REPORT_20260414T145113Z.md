# TASK-M11-06 Preflight Report

## Ticket ID

`TASK-M11-06 — CostTracker extension`

## Verdict

`GO`

## Current Branch

`feature/provider-resolution-seam`

## Current HEAD

`1472024c466011d76f4f003ac20587d2090be3df`

## Authority Files Reviewed

- `AGENTS.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md`
- `docs/archive/reports/M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md`
- `docs/archive/reports/TASK-M11-05_CLOSEOUT_REPORT_20260414T143657Z.md`

## Verified File Paths

- `src-rust/crates/core/src/lib.rs`
- `src-rust/crates/core/Cargo.toml`
- `src-rust/crates/cli/src/main.rs`
- `src-rust/crates/query/src/lib.rs`
- `src-rust/crates/query/src/agent_tool.rs`
- `src-rust/crates/query/src/cron_scheduler.rs`
- `src-rust/crates/tools/src/team_tool.rs`

## Verified Symbols / Repo Facts

- `CostTracker` is present where authority expects: `src-rust/crates/core/src/lib.rs:2850`.
- Current definition is `#[derive(Debug, Default)] pub struct CostTracker` with private fields:
  - `input_tokens: AtomicU64`
  - `output_tokens: AtomicU64`
  - `cache_creation_tokens: AtomicU64`
  - `cache_read_tokens: AtomicU64`
  - `pricing: parking_lot::RwLock<ModelPricing>`
- `agent_id` and `provider_id` do not exist today.
- `CostTracker::new()` exists at `src-rust/crates/core/src/lib.rs:2860` with signature `pub fn new() -> Arc<Self>`.
- `CostTracker::with_model(model: &str) -> Arc<Self>` also exists at `src-rust/crates/core/src/lib.rs:2867`.
- `set_model(&self, model: &str)` already exists; no other `CostTracker` setters exist today.
- No other constructors/builders were found beyond `new()` and `with_model()`.
- `CostTracker::with_model()` appears currently unused in the workspace.
- `parking_lot` is already available in the workspace and core crate:
  - `src-rust/Cargo.toml`
  - `src-rust/crates/core/Cargo.toml`

## Existing CostTracker And Construction-Site Reality

- The live `CostTracker` implementation already uses `parking_lot::RwLock` for `pricing`, so the M10 `RwLock<Option<String>>` refinement fits current concurrency style better than plain `Option<String>`.
- The MPWO shorthand (`Option<String>`) vs M10 detail (`RwLock<Option<String>>`) is a non-blocking authority-detail mismatch, not a structural blocker.
- Direct construction sites are spread across the Rust workspace, not confined to `core/src/lib.rs`.
- Runtime construction is concentrated at top-level CLI/session entry points:
  - `src-rust/crates/cli/src/main.rs:395` for pre-session named-command context
  - `src-rust/crates/cli/src/main.rs:637` for the main session tracker
- Remaining direct `CostTracker::new()` sites found in `core`, `query`, `tools`, `commands`, and `tui` are test helpers.
- Because all live callers use constructors returning `Arc<CostTracker>`, adding internal fields does not force immediate multi-file call-site edits.
- The M10 `new() + setters` approach does avoid widening scope.
- Minimal execution scope should still account for both constructors:
  - `new()`
  - `with_model()`
  Even if `new()` is the dominant runtime path, `with_model()` also builds `Self` and should remain internally consistent.

## Existing Worker / Wiring Reality

- Live sub-agent / worker paths do not construct independent `CostTracker` instances.
- The main session constructs one tracker in `cli/src/main.rs` and passes it into `ToolContext`.
- Query dispatch resolves provider identity later, inside `run_query_loop`, after the tracker already exists:
  - `src-rust/crates/query/src/lib.rs:877`
  - materialized target available by `src-rust/crates/query/src/lib.rs:889`
- Agent / worker paths reuse the parent tracker by cloning the same `Arc<CostTracker>`:
  - background agent path: `src-rust/crates/query/src/agent_tool.rs:402-416`
  - synchronous agent path: `src-rust/crates/query/src/agent_tool.rs:456-462`
  - team runner path: `src-rust/crates/query/src/agent_tool.rs:638-644`
  - cron scheduler child query path: `src-rust/crates/query/src/cron_scheduler.rs:78-92`
- `team_tool.rs` has agent IDs in team metadata, but those IDs are not currently wired into `CostTracker`.
- Therefore the M10 line "Workers call these setters when constructing their `CostTracker`" is not literally executable against live repo reality:
  - workers are not constructing their own trackers
  - they share the parent tracker
  - concurrent worker setter calls on one shared tracker would be last-writer-wins, not durable per-worker attribution
- Preflight conclusion: current worker call sites should remain unchanged in M11-06. The narrow, ticket-safe scope is to add the fields plus setters only, and defer actual worker/provider wiring to a later ticket with explicit authority.

## Dependency / Trait Reality

- `CostTracker` currently derives `Debug` and `Default`; it does not derive `Clone`, `Serialize`, or `Deserialize`.
- No `impl Clone for CostTracker` was found.
- No serialization/deserialization use of `CostTracker` was found.
- Based on current live usage of `RwLock<ModelPricing>` under `#[derive(Default)]`, adding `RwLock<Option<String>>` fields appears compatible with existing trait behavior because `Option<String>` defaults to `None`.
- No evidence was found that M11-06 needs to change cost calculation logic, token accounting, or event shapes.

## Validation Commands Verified

- Authority command:
  - `cd src-rust && cargo check --workspace`
  - Result: `PASS`
- Useful narrower iteration command for a core-only implementation pass:
  - `cd src-rust && cargo check -p claurst-core`
  - Result: `PASS`

## Drift Found

- Current live branch and HEAD still match the accepted M11-05 closeout baseline; no branch/HEAD drift found.
- Repo working tree is not clean:
  - modified: `.gitignore`
  - many untracked report/document paths, including `docs/Current/` and `docs/archive/reports/`
  - `src-rust/target/` is present and untracked
- Authority-support files used for this preflight are currently untracked in git, but present on disk and readable:
  - `docs/Current/MPWO_WORK_ORDER_PACK.md`
  - `docs/archive/reports/M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md`
  - `docs/archive/reports/TASK-M11-05_CLOSEOUT_REPORT_20260414T143657Z.md`
- Authority wording drift:
  - MPWO shorthand suggests plain `Option<String>`
  - M10 detail specifies `parking_lot::RwLock<Option<String>>` plus setters
  - live code strongly favors the M10 `RwLock` shape
- Authority/runtime drift:
  - M10 says workers set fields when constructing their `CostTracker`
  - live worker paths share the parent tracker and do not construct independent trackers

## Blockers

- No blocker for a narrow core-only M11-06 execution pass.
- Blocker would arise only if M11-06 were interpreted to require immediate worker-side setter wiring, because live worker architecture does not construct per-worker trackers and shared-tracker mutation would not provide reliable per-worker attribution.

## Hosted Ollama Compatibility Baseline Preserved

`yes`

Basis:

- No code was edited in preflight.
- Required workspace validation passed on current HEAD.
- Recon did not identify any need to touch hosted-Ollama-specific resolution or dispatch logic in M11-06.

## Exact Recommendation For Next Step

Proceed with a narrow implementation pass limited to `src-rust/crates/core/src/lib.rs`:

1. Add `agent_id` and `provider_id` to `CostTracker` using `parking_lot::RwLock<Option<String>>`.
2. Keep cost calculation and usage accounting unchanged.
3. Add `set_agent_id(&self, id: String)` and `set_provider_id(&self, id: String)`.
4. Preserve constructor compatibility by ensuring both `new()` and `with_model()` produce trackers with both fields unset.
5. Do not modify worker / agent call sites in M11-06.
6. Treat actual worker/provider attribution wiring as a later ticket requiring explicit authority, because current worker paths share one tracker rather than constructing distinct trackers.
