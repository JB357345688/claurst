# TASK-M11-04 Preflight Report

## Ticket ID

`TASK-M11-04 — HealthCache implementation`

## Verdict

`GO`

## Current Branch

`feature/provider-resolution-seam`

## Current HEAD

`828b08ebdf5a7789997497c4b579447056f64d5d`

## Authority Files Reviewed

- `AGENTS.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md`
- `docs/archive/reports/M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md`
- `docs/archive/reports/TASK-M11-03_CLOSEOUT_REPORT_20260414T134737Z.md`

## Preflight Verdict Summary

Live repo reality is consistent enough to execute `TASK-M11-04` without widening scope. The target file does not yet exist, the query crate already has `dashmap`, `tokio`, and `tokio-util`, and there is a direct narrow health-probe surface via `LlmProvider::health_check()`.

No structural drift invalidates the ticket. The only meaningful drift is type-shape guidance: live core ownership of provider IDs is `ProviderId`, but the current query seam (`ProviderIdentity`, `ExecutionTarget`, resolution helpers) still passes provider IDs as `String`, so a string-keyed `HealthCache` remains the narrowest ticket-compatible fit.

## Verified File Paths

- `src-rust/crates/query/src/lib.rs` exists and is the owning module root for `claurst-query`
- `src-rust/crates/query/src/health_cache.rs` does not exist
- `src-rust/crates/query/Cargo.toml` already includes `dashmap`
- `src-rust/crates/api/src/provider.rs` owns the provider trait probe surface
- `src-rust/crates/api/src/provider_types.rs` owns `ProviderStatus`
- `src-rust/crates/core/src/provider_id.rs` owns `ProviderId`
- `src-rust/crates/api/src/registry.rs` owns provider enumeration/lookup helpers
- `src-rust/crates/query/src/provider_resolution.rs` is the immediate downstream consumer surface for M11-05

## Verified Symbols / Repo Facts

### ProviderId

- Ownership point: `src-rust/crates/core/src/provider_id.rs`
- Exact type: `pub struct ProviderId(String);`
- Live usage: registry keys are `HashMap<ProviderId, Arc<dyn LlmProvider>>`
- Query-crate reality: `provider_resolution.rs` currently stores `provider_id` as `String` in both `ProviderIdentity` and `ExecutionTarget`

### ProviderStatus

- Ownership point: `src-rust/crates/api/src/provider_types.rs`
- Exact type: `pub enum ProviderStatus { Healthy, Degraded { reason: String }, Unavailable { reason: String } }`
- Live re-export reality: query tests already import it as `claurst_api::ProviderStatus`

### DashMap / Async / Timeout Availability

- `dashmap` is a workspace dependency in `src-rust/Cargo.toml`
- `claurst-query` already depends on `dashmap` directly in `src-rust/crates/query/Cargo.toml`
- `tokio` is available with workspace `full` features
- `tokio-util` is already a query dependency
- `tokio::time::timeout` is available for a 5s probe timeout

### Time-Type Reality

- Query crate already uses `std::time::Duration`
- Query crate already uses `tokio::time::Instant` for stall-reset logic in `lib.rs`
- Query crate does not currently use `std::time::Instant` in its source files
- Narrow implementation guidance: cached timestamps can use `std::time::Instant` plus `std::time::Duration` without conflicting with existing runtime code, while async probe timeout should use `tokio::time::timeout`

### Cache-Key Shape Reality

- `DashMap<ProviderId, ...>` is technically usable because `ProviderId` derives `Clone + Eq + Hash`
- It is not the narrowest fit for current query-crate seams because M11-05-facing query types still expose provider IDs as `String`
- Repo reality therefore favors `DashMap<String, (ProviderStatus, Instant)>` or equivalent string-boundary adaptation inside `HealthCache`

## Existing Provider-Health Probe Reality

- Direct probe surface exists today: `claurst_api::LlmProvider::health_check(&self) -> Result<ProviderStatus, ProviderError>`
- `ProviderRegistry` already exposes:
  - `get(&self, id: &ProviderId) -> Option<&Arc<dyn LlmProvider>>`
  - `provider_ids(&self) -> Vec<&ProviderId>`
  - `check_all_health(&self) -> Vec<(ProviderId, ProviderStatus)>`
- Narrowest viable M11-04 implementation path:
  - accept `provider_id: &str`
  - accept `provider: &dyn LlmProvider`
  - on stale/miss, call `tokio::time::timeout(Duration::from_secs(5), provider.health_check()).await`
  - map timeout or trait error to `ProviderStatus::Unavailable`
  - update the cache and return the status

No extra shim is required. No blocker exists on the health-probe surface.

## Module-Wiring Reality

- `src-rust/crates/query/src/lib.rs` is the correct owning module file
- Existing pattern already re-exports module APIs (`pub use provider_resolution::*;`)
- Exact ticket-local wiring is straightforward:
  - add `mod health_cache;` or `pub mod health_cache;`
  - add `pub use health_cache::*;`
- There is no crate-visibility blocker

## Existing Test Reality

- Query crate already uses inline module tests (`#[cfg(test)] mod tests`) in source files such as:
  - `src-rust/crates/query/src/provider_resolution.rs`
  - `src-rust/crates/query/src/agent_tool.rs`
- Query crate already uses both:
  - `#[test]` for synchronous logic
  - `#[tokio::test]` for async logic
- Best narrow test location for M11-04:
  - inline tests inside `src-rust/crates/query/src/health_cache.rs`
- Best narrow test style:
  - cache hit/miss/expiry: module-local tests can construct `HealthCache` with a short TTL or inject backdated timestamps directly into the private map for deterministic expiry testing
  - probe behavior: async mock provider using `#[tokio::test]`
- Determinism assessment:
  - hit/miss/expiry are testable deterministically without wider plumbing
  - timeout is also testable with a slow mock provider, though that is slower unless a paused-time approach is added

## Validation Commands Verified

- `cd src-rust && cargo check -p claurst-query`
  - Result: `PASS`
- `cd src-rust && cargo test -p claurst-query -- health_cache`
  - Result: `PASS` as a command path
  - Current test count: `0 passed; 0 failed; 114 filtered out`
  - Interpretation: the authority-intent filtered validation path is valid now, but no `health_cache` tests exist yet because the module has not been implemented

Recommended execution-time validation for the ticket remains:

- `cd src-rust && cargo check -p claurst-query`
- `cd src-rust && cargo test -p claurst-query -- health_cache`

## Bounded Recon for Immediate M11-04 Relevance

### Independence From M11-01 / M11-02 / M11-03

- M11-04 does not require new `TrustDomain` logic from M11-01
- M11-04 does not require `ModelEntry` capability fields from M11-02
- M11-04 does not require capability-matching helpers from M11-03
- Live repo reality supports the planning claim that M11-04 is independently executable

### M11-05 Consumption Readiness

- M11-05’s planned `resolve_provider_with_fallback(..., health_cache: &HealthCache, ...)` shape matches live repo reality
- `provider_resolution.rs` already has the M11-03 capability helpers M11-05 will need
- `ProviderRegistry` already exposes candidate enumeration and lookup surfaces M11-05 will need
- No extra pre-M11-05 plumbing is required beyond:
  - landing `HealthCache`
  - exporting it from `claurst-query`

Later runtime injection of a shared cache into higher layers is still later-ticket work, not a blocker to M11-04.

## Drift Found

- Branch drift: none
- HEAD drift: none
- Target path drift: none (`src-rust/crates/query/src/health_cache.rs` is absent as expected)
- Type-shape drift:
  - prompt guidance mentions a `ProviderId`-keyed cache shape
  - accepted planning support specifies `DashMap<String, (ProviderStatus, Instant)>`
  - live query seam strongly favors the planning-report string-keyed shape
- Worktree hygiene drift:
  - current repo has unrelated modified/untracked files, including `.gitignore`, report artifacts, `docs/Current/`, and `src-rust/target/`
  - this is not a blocker for M11-04 execution, but it should be kept explicit in later patch-hygiene/review reporting
- Unrelated warning drift:
  - `cargo test -p claurst-query -- health_cache` surfaced an existing unused-import warning in `crates/query/src/compact.rs`
  - unrelated to this ticket

## Blockers

None.

## Hosted Ollama Compatibility Baseline Preserved

`yes`

Basis:

- This was a preflight-only pass with no implementation changes
- Live branch and HEAD still match the accepted M11-03 closeout baseline
- Existing Ollama/provider materialization logic in `provider_resolution.rs` was only inspected, not modified
- The narrow M11-04 seam uses generic `LlmProvider::health_check()` and does not require changing hosted Ollama handling

## Exact Recommendation For Next Step

Proceed to the execution prompt for `TASK-M11-04` with a narrow implementation in `src-rust/crates/query/src/health_cache.rs`, exported from `src-rust/crates/query/src/lib.rs`, using:

- a string-keyed `DashMap` cache aligned to current query-crate seams
- `ProviderStatus` from `claurst_api`
- `LlmProvider::health_check()` wrapped in a hardcoded 5s `tokio::time::timeout`
- inline unit tests in `health_cache.rs` for cache hit, miss, and deterministic expiry

Do not widen into M11-05 fallback resolution logic during the M11-04 execution pass.
