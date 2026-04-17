# TASK-M9-12 Closeout Report

## Ticket
`TASK-M9-12 — Full regression + D1 completion declaration (corrective patch closeout)`

## Timestamp UTC
`20260413T153943Z`

## Exact Defect Being Corrected
- Defect 1:
  - workspace build regression in `src-rust/crates/cli/src/main.rs`
  - `claurst_query::run_query_loop(...)` now expects `Option<&AnthropicClient>`, but five CLI call sites still passed `&AnthropicClient`
- Defect 2:
  - workspace test nondeterminism in `claurst-query`
  - three separate test-only env/auth locks allowed parallel tests to race on `HOME` and `.claurst/auth.json`, leaking an OpenAI credential between tests

## Files Changed
- `src-rust/crates/cli/src/main.rs`
- `src-rust/crates/query/src/agent_tool.rs`
- `src-rust/crates/query/src/lib.rs`
- `src-rust/crates/query/src/provider_resolution.rs`
- `docs/archive/reports/TASK-M9-12_EXECUTION_RERUN_REPORT_20260413T153943Z.md`
- `docs/archive/reports/TASK-M9-12_CLOSEOUT_REPORT_20260413T153943Z.md`

## Smallest Fix Applied
- In `src-rust/crates/cli/src/main.rs`:
  - wrapped the five failing `client_clone.as_ref()` arguments in `Some(...)`
  - no logic change beyond matching the current `run_query_loop(...)` signature
- In `src-rust/crates/query/src/lib.rs`:
  - added one crate-wide test-only provider-auth mutex helper
- In `src-rust/crates/query/src/agent_tool.rs`
  and `src-rust/crates/query/src/provider_resolution.rs`:
  - switched `with_isolated_provider_auth(...)` to use the shared crate-wide lock
  - adjusted test-only imports accordingly
- No runtime feature scope was widened
- No D2 logic was introduced

## Validations Rerun
- `cd src-rust && cargo fmt --all -- --check`
- `cd src-rust && cargo build --workspace`
- `cd src-rust && cargo test --workspace`
- `cd src-rust && cargo clippy --workspace --all-targets`

## Result
- `cargo fmt --all -- --check` -> `PASS`
- `cargo build --workspace` -> `PASS`
- `cargo test --workspace` -> `PASS`
- `cargo clippy --workspace --all-targets` -> `PASS`
- Manual smoke test -> not run because the environment had no usable OpenAI credential, no `$HOME/.claurst/auth.json`, and no unrestricted outbound network access
- Final ticket outcome -> `PASS / D1 COMPLETE WITH SMOKE-TEST ENV LIMITATION`

## Remaining Blocker
- `none` for automated validation
- Environment limitation remains for live OpenAI smoke execution only

## Notes
- Unrelated untracked workspace/report/build noise was left untouched
- Hosted Ollama compatibility baseline remains preserved
- Review basis remains scope-clean relative to this rerun: the four tracked source files above plus the required report artifacts
