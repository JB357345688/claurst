# TASK-M11-12 Gate Clarification Report

## 1. timestamp UTC

`2026-04-15T06:00:15Z`

## 2. authority inputs reviewed

- `AGENTS.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md`
- `docs/Current/TASK-M11-10_SPLIT_PLAN_AND_TEMPORARY_AUTHORITY_AMENDMENT.md`
- `docs/archive/reports/M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md`
- `docs/archive/reports/MPWO_REVISION_REPORT_20260414T003218Z.md`
- `docs/archive/reports/M11_CONVERGENCE_REVIEW_20260415T042825Z.md`
- `docs/archive/reports/TASK-M11-11_CLOSEOUT_REPORT_20260415T053856Z.md`
- `docs/archive/reports/TASK-M11-12_PREFLIGHT_REPORT_20260415T054738Z.md`

Read-only probes rerun for this clarification:

- `git rev-parse HEAD` -> `0c9dac407e82fccdfe16337bc2c05a6aeb816ca5`
- `cd src-rust && cargo fmt --all -- --check` -> `FAIL`
- `cd src-rust && cargo test -p claurst-query agent_tool::tests::teamcreate_mixed_providers_per_agent_dispatch -- --exact` -> `FAIL`
- `cd src-rust && cargo clippy -p claurst-core --all-targets -- -D warnings` -> `FAIL`
- `cd src-rust && cargo test -p claurst-api` -> `PASS`
- `cd src-rust && cargo clippy -p claurst-api --all-targets --no-deps -- -D warnings` -> `FAIL`
- `cd src-rust && cargo clippy -p claurst-query --all-targets --no-deps -- -D warnings` -> `FAIL`

## 3. preflight halt summary

The accepted split runtime baseline remains:

- `08R`
- `08B`
- `09`
- `10A`
- `10B1`
- `10B2`
- `11`

The `TASK-M11-12` preflight halted not because the D2 validation content is structurally wrong, but because the as-written closure gates currently require baseline cleanup outside a narrow validation ticket:

- `cargo fmt --all -- --check` fails before new M11-12 work starts.
- `cargo test --workspace` is currently blocked by one stale `claurst-query` TeamCreate observability assertion that still expects pre-`M11-11` output.
- `cargo clippy --workspace --all-targets -- -D warnings` fails in `claurst-core` on unrelated lint debt before any M11-12-specific test work.

Preflight and rerun evidence both support the same conclusion: M11-12 remains a single narrow validation ticket on content, but its current closure wording overreaches the accepted split path.

## 4. option analysis

### Option A — baseline cleanup first

Assessment:

- This is honest but too broad for the accepted split path.
- It would force separate pre-work for repo-wide formatting debt, the stale query-side observability assertion, and unrelated `claurst-core` lint cleanup before the D2 validation ticket can even start.
- The stale `claurst-query` failure is not generic baseline debt. It is a ticket-relevant regression against accepted `TASK-M11-11` observability behavior, so pushing it out of M11-12 would separate the validation ticket from the exact seam it is supposed to validate.
- The unrelated full-workspace clippy debt is real, but the evidence does not show that M11-12 needs that debt resolved in order to validate the accepted split runtime path.

Conclusion:

- Option A is defensible only if M11-12 cannot be meaningfully validated otherwise.
- Current evidence does not support that threshold.

### Option B — narrowed M11-12 validation gate

Assessment:

- This preserves M11-12 as one narrow D2 validation ticket on the accepted split baseline.
- It keeps the stale query-side TeamCreate observability test in scope because that failure is directly caused by accepted `TASK-M11-11` behavior and lives on a `query`-owned validation surface.
- It removes unrelated repo-wide blockers from the required closure gate without reopening accepted runtime tickets.
- It still preserves meaningful validation by requiring the D2-owned tests and the accepted split runtime path to pass on the owning crates.

Question-by-question answer set:

1. Is the stale `claurst-query` TeamCreate observability test ticket-relevant and therefore valid to fix inside M11-12?
   - `yes`
   - The failing assertion in `agent_tool::tests::teamcreate_mixed_providers_per_agent_dispatch` now mismatches accepted `TASK-M11-11` behavior by expecting raw pre-observability output instead of the `[[CLAURST_QUERY_OBS:...]]` payload. That is M11-12 validation work, not unrelated cleanup.
2. Is workspace-wide `cargo clippy --workspace --all-targets -- -D warnings` realistic for M11-12 as scoped, given unrelated `claurst-core` lint debt?
   - `no`
   - It fails immediately in `claurst-core`, outside the narrow M11-12 validation delta.
   - Even crate-scoped `--no-deps` clippy probes on `claurst-api` and `claurst-query` also fail on pre-existing lint debt, so a blocking clippy gate is not currently realistic for M11-12 without separately authorizing lint-cleanup scope.
3. Is workspace-wide `cargo fmt --all -- --check` realistic for M11-12 as scoped, given baseline formatting drift outside the ticket-owned validation work?
   - `no` as a blocking closure gate
   - It fails before M11-12 begins. M11-12 may normalize formatting in files it must touch, but requiring a full-workspace fmt-clean baseline would convert the ticket into a formatting cleanup pass.
4. What is the narrowest credible gate set for M11-12 that still validates the accepted split path?
   - Require split-path validation on the D2-owned crates and query-owned observability seam.
   - Keep workspace-wide fmt/clippy as informational probes only until separately-authorized baseline cleanup exists.
5. What exact acceptance language should GPT/WebUI use for M11-12 going forward?
   - See Sections 6 and 9 below.

Conclusion:

- Option B is the narrowest honest choice.
- It validates the accepted split path without silently broadening M11-12 into a repository cleanup ticket.

## 5. recommended gate decision

`Recommend Option B`

Decision basis:

- The accepted split chain already converged on a valid runtime baseline in the convergence review and `TASK-M11-11` closeout.
- The only currently failing test that is behaviorally relevant to that baseline is query-owned and directly tied to accepted `TASK-M11-11` observability.
- Repo-wide fmt/clippy gates are presently baseline-debt gates, not meaningful acceptance gates for M11-12 as scoped.

Therefore:

- Keep `TASK-M11-12` as one narrow D2 validation ticket.
- Permit M11-12 to fix the stale `claurst-query` TeamCreate observability assertion.
- Do not require separate pre-work for unrelated `claurst-core` clippy cleanup as a prerequisite to M11-12.
- Do not require full-workspace fmt/clippy as blocking closure gates for this ticket.

## 6. exact revised acceptance / validation gate for M11-12

`TASK-M11-12` closes against the accepted split runtime baseline, not against broad repository cleanup.

Blocking acceptance requirements:

1. Validate the accepted split baseline only:
   - `08R` root session-budget wiring
   - `08B` inherited parent shared-session accounting
   - `09` child `max_tokens`
   - `10A` child `allow_fallback`
   - `10B1` layered child-budget seam
   - `10B2` child/team `budget_usd` carriage
   - `11` `QueryEvent` expansion / observability
2. Add or update D2 acceptance tests on the owning surfaces needed to validate that baseline.
3. The query-side TeamCreate observability assertion must be updated so accepted `TASK-M11-11` behavior is validated rather than rejected.
4. Required blocking commands for closure:
   - `cd src-rust && cargo build --workspace`
   - `cd src-rust && cargo test -p claurst-api`
   - `cd src-rust && cargo test -p claurst-query`
5. Any Rust files touched by M11-12 must be returned rustfmt-clean within the ticket-owned diff.

Informational, non-blocking baseline probes for this ticket:

- `cd src-rust && cargo fmt --all -- --check`
- `cd src-rust && cargo clippy --workspace --all-targets -- -D warnings`

Required interpretation of those informational probes:

- Record their results in M11-12 reporting.
- Do not fail M11-12 solely because those two workspace-wide probes still surface pre-existing baseline debt outside the ticket’s narrow validation delta.

## 7. what M11-12 may fix in-scope

- The stale `claurst-query` TeamCreate observability assertion that still expects pre-`TASK-M11-11` raw output.
- Inline D2 acceptance tests on the originally-owned validation surfaces:
  - `src-rust/crates/api/src/provider_types.rs`
  - `src-rust/crates/query/src/provider_resolution.rs`
  - `src-rust/crates/query/src/health_cache.rs`
  - `src-rust/crates/query/src/session_budget.rs`
  - `src-rust/crates/query/src/agent_tool.rs`
- Any narrowly necessary query-owned test adjustment needed to validate `TASK-M11-11` observability/sanitization on the accepted split path.
- Incidental formatting cleanup in files already touched for the above work.
- Ticket-local test fallout caused directly by the allowed M11-12 delta.

## 8. what remains explicitly out of scope

- Repo-wide or crate-wide lint cleanup as a new goal for M11-12.
- Unrelated `claurst-core` clippy remediation.
- Full-workspace formatting cleanup on untouched files.
- Reopening or redesigning accepted tickets:
  - `08R`
  - `08B`
  - `09`
  - `10A`
  - `10B1`
  - `10B2`
  - `11`
- Any reinterpretation of child `budget_usd` as `max_budget_usd`.
- Any `ToolContext.session_budget` or `ToolContext.health_cache` redesign.
- Hosted Ollama behavior changes.
- M12 work.

## 9. exact proposed wording snippet for GPT/WebUI to adopt

> `TASK-M11-12` remains one narrow D2 validation ticket on the accepted split baseline (`08R`, `08B`, `09`, `10A`, `10B1`, `10B2`, `11`). It may add or adjust inline tests on the D2-owned validation surfaces and may fix the stale `claurst-query` TeamCreate observability assertion introduced by accepted `TASK-M11-11` behavior. Required blocking validation for closeout is: `cd src-rust && cargo build --workspace`, `cd src-rust && cargo test -p claurst-api`, and `cd src-rust && cargo test -p claurst-query`. Workspace probes `cd src-rust && cargo fmt --all -- --check` and `cd src-rust && cargo clippy --workspace --all-targets -- -D warnings` must still be recorded, but they are informational only for M11-12 until separately-authorized baseline cleanup resolves the existing debt. Do not reopen accepted tickets `08R`, `08B`, `09`, `10A`, `10B1`, `10B2`, or `11`.`

## 10. whether M11-12 may proceed after this clarification

`yes`

Basis:

- The blocker was gate wording, not loss of a credible validation path.
- Option B defines a narrow, still-meaningful closure gate tied to the accepted split baseline.

## 11. notes / risks

- The repo still carries real baseline debt in workspace fmt/clippy. This report does not erase that debt; it only concludes that M11-12 is not the correct ticket to absorb it wholesale.
- Because `cargo clippy -p claurst-api --all-targets --no-deps -- -D warnings` and `cargo clippy -p claurst-query --all-targets --no-deps -- -D warnings` also fail today, any future ticket that wants a blocking clippy gate will need explicit cleanup authority.
- M11-12 reporting should state clearly that workspace fmt/clippy results were recorded but non-blocking under this clarification.
- Patch hygiene still matters. M11-12 should stay limited to the D2-owned validation surfaces and the one ticket-relevant stale query-side assertion.
