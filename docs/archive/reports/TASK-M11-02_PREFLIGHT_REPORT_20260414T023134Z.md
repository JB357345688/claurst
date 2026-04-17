# TASK-M11-02 Preflight Report

## Ticket ID

`TASK-M11-02 — ModelEntry extension`

## Verdict

`GO`

## Current Branch

`feature/provider-resolution-seam`

## Current HEAD

`dc772aac2631d91b0d4c10daa8086616d9e203d8`

## Authority Files Reviewed

- `AGENTS.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md`
- `docs/archive/reports/M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md`
- `docs/archive/reports/TASK-M11-01_CLOSEOUT_REPORT_20260414T022628Z.md`

## Verified File Paths

- `src-rust/crates/api/src/model_registry.rs`
- `src-rust/crates/api/src/lib.rs`
- `src-rust/crates/query/src/provider_resolution.rs`
- `src-rust/crates/api/src/provider_types.rs`

## Verified Symbols / Repo Facts

- `ModelEntry` exists at `src-rust/crates/api/src/model_registry.rs:24`.
- Current `ModelEntry` fields are:
  - `info: ModelInfo`
  - `cost_input: Option<f64>`
  - `cost_output: Option<f64>`
  - `cost_cache_read: Option<f64>`
  - `cost_cache_write: Option<f64>`
  - `tool_calling: bool`
  - `reasoning: bool`
  - `vision: bool`
  - `family: Option<String>`
  - `status: String`
- `ModelEntry` currently derives `Debug, Clone, serde::Serialize, serde::Deserialize`.
- The four M11-02 fields do not yet exist:
  - `pdf_input: Option<bool>`
  - `audio_input: Option<bool>`
  - `structured_output: Option<bool>`
  - `max_output_tokens: Option<u32>`
- The obvious insertion point is immediately after `vision: bool` at `model_registry.rs:39`, matching the accepted M10 wording.
- `claurst_api::ModelEntry` is already re-exported from `src-rust/crates/api/src/lib.rs:80`, so M11-03 can consume these new fields without extra API-surface plumbing.
- `provider_resolution.rs` already depends on `ModelRegistry` today and has the live D1 lookup tests that exercise registry-backed resolution.

## Existing Parser / Loading Reality

- `ModelRegistry::load_cache()` exists by that exact name at `model_registry.rs:573`.
- Cache loading is mixed-mode:
  - Raw `models.dev` JSON is detected heuristically and parsed manually via `parse_models_dev_response()`.
  - Serialized registry cache is loaded through `serde_json::from_str::<HashMap<String, ModelEntry>>()` at `model_registry.rs:592`.
- This means parser behavior for saved cache entries is already serde-driven, not hand-wired per field.
- Conclusion on the authority conflict:
  - The MPWO/user wording that says "`load_cache()` must parse the new optional JSON fields" is descriptive drift, not a blocker.
  - Adding the new fields to `ModelEntry` is sufficient for the serde-backed cache path; no `load_cache()` algorithm change is required.
  - `#[serde(default)]` on the new fields is authority-aligned and harmless.
- Important nuance:
  - Compile-correct implementation will still require touching existing `ModelEntry` struct literals inside `ModelRegistry` helper methods, even though the M10 ticket says "Do not modify `ModelRegistry` methods."
  - In live repo reality this is a narrow wording tension, not a material blocker, because those edits are constructor-field additions only and do not change method behavior.

## Bundled Snapshot / Manual Construction Reality

- Manual `ModelEntry` construction sites exist only in the target file:
  - bundled Anthropic snapshot at `model_registry.rs:118`
  - bundled OpenAI snapshot at `model_registry.rs:158`
  - bundled Google snapshot at `model_registry.rs:207`
  - `models.dev` ingest path at `model_registry.rs:530`
- No external `claurst_api::ModelEntry { ... }` construction sites were found elsewhere in the Rust workspace.
- Result:
  - M11-02 is correctly owned by `src-rust/crates/api/src/model_registry.rs`.
  - A narrow implementation will need to extend those four local struct literals for compile correctness.
  - No separate fixture file updates were discovered in current repo reality.

## Existing Test Reality

- There are no dedicated unit tests in `src-rust/crates/api/src/model_registry.rs`.
- `cargo test -p claurst-api -- --list` shows 30 current `claurst-api` tests, none named for `model_registry`.
- The nearest existing registry-behavior coverage is in `src-rust/crates/query/src/provider_resolution.rs`:
  - `provider_resolution::tests::p8_no_provider_bare_model_registry_resolves`
  - `provider_resolution::tests::p9_no_provider_bare_model_registry_has_no_match`
  - `provider_resolution::tests::p10_no_provider_without_model_registry_defaults`
- These tests confirm that current D1 registry-backed provider resolution works and provide a useful non-regression smoke surface adjacent to M11-02.
- A narrow M11-02 implementation does not appear to require test fixture edits, because no serialized model-registry fixtures were found.

## Validation Commands Verified

- Required authority command:
  - `cd src-rust && cargo check -p claurst-api`
  - Live result: `PASS`
- Live-reality crate test command:
  - `cd src-rust && cargo test -p claurst-api`
  - Live result: `PASS` (`30 passed; 0 failed`)
- Useful adjacent smoke for current registry behavior:
  - `cd src-rust && cargo test -p claurst-query provider_resolution`
  - Live result: `PASS` (`22 passed; 0 failed`)
  - Note: emits an unrelated pre-existing warning from `crates/query/src/compact.rs` for an unused `Role` import.

## Drift Found

- Branch and HEAD do **not** drift from the accepted M11-01 baseline. Live repo still matches:
  - branch `feature/provider-resolution-seam`
  - HEAD `dc772aac2631d91b0d4c10daa8086616d9e203d8`
- The working tree is dirty and noisy:
  - `.gitignore` is modified
  - many docs/report files are untracked
  - `src-rust/target/` is present and untracked
- `docs/Current/MPWO_WORK_ORDER_PACK.md` on disk reads as an MPWO revision report / summary, not a full actionable ticket body. Practical M11-02 implementation detail therefore comes from the accepted M10 report plus the active prompt.
- There are no dedicated `model_registry` tests despite ticket wording that says "existing model registry tests pass."
- The parser-method authority tension is non-blocking:
  - `load_cache()` itself does not need logic changes
  - some existing `ModelRegistry` method bodies will still need local struct-literal field additions for compile correctness

## Blockers

- None.

## Hosted Ollama Compatibility Baseline Preserved

`yes`

Basis:

- Current branch/HEAD still match the accepted M11-01 closeout baseline.
- The target scope is confined to `claurst-api` model metadata.
- Live adjacent smoke `cargo test -p claurst-query provider_resolution` passes, including hosted/local provider resolution behavior.

## Exact Recommendation For Next Step

Proceed to the implementation pass for `TASK-M11-02` with scope limited to `src-rust/crates/api/src/model_registry.rs`.

Implementation should:

- add the four new `Option` fields to `ModelEntry` after `vision`
- mark them with `#[serde(default)]`
- set them to `None` in the three bundled snapshot constructors and the `parse_models_dev_response()` constructor
- leave `load_cache()` logic unchanged

Validation for the execution pass should be:

- `cd src-rust && cargo check -p claurst-api`
- `cd src-rust && cargo test -p claurst-api`

Optional but recommended adjacent smoke after implementation:

- `cd src-rust && cargo test -p claurst-query provider_resolution`
