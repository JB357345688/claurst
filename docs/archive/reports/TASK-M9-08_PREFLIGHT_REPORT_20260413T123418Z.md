# TASK-M9-08 Preflight Report

## Ticket
- `TASK-M9-08 — Root registry + resolution failure -> hard error test`

## Timestamp UTC
- `2026-04-13T12:34:18Z`

## Branch
- Expected: `feature/provider-resolution-seam`
- Actual: `feature/provider-resolution-seam`

## Verdict
- `READY-WITH-NOTES`

## Repo State Summary
- `git branch --show-current` returned `feature/provider-resolution-seam`.
- `git diff --name-only` returned no tracked unstaged changes.
- `git diff --cached --name-only` returned no staged changes.
- `git status --short --branch` shows substantial unrelated untracked repo noise, including `.codex`, `docs/Current/`, many prior report files under `docs/archive/reports/`, and `src-rust/target/`.
- `git log --oneline --decorate -n 20` shows `TASK-M9-07` at `HEAD`, plus prior `TASK-M9-04`, `TASK-M9-03`, `TASK-M9-02`, and `TASK-M8-11` commits in ancestry.

## Authority Reviewed
- `AGENTS.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md`

## Dependency Baseline Confirmed
- `M8-11` is present in branch history via commit `b5b6dd4 TASK-M8-11 reconcile M8 workspace validation and formatting`.
- `M9-02`, `M9-03`, `M9-04`, and `M9-07` are present in branch history.
- Per active prompt authority, `M9-01`, `M9-05`, and `M9-06` are already complete and do not need reopening.
- Current branch reality is compatible with starting `M9-08` after accepted `M8-11`.
- Hosted Ollama compatibility baseline preserved.
- Hosted Ollama remains a background invariant only; nothing in this preflight indicates a need to reopen or widen that scope.

## Exact M9-08 Contract
- Root-path counterpart to `M9-07`.
- Active branch to verify: `provider_registry = Some(...)`.
- Failure mode to verify: provider resolution/materialization failure in the registry-backed seam.
- Required behavior: root call returns hard error.
- Forbidden behavior: silent fallthrough to the legacy Anthropic client path.
- Ticket authority file target says `src-rust/crates/query/tests/`.

## Verified Files / Symbols / Commands
- Files reviewed:
- `AGENTS.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md`
- `src-rust/crates/query/src/lib.rs`
- `src-rust/crates/query/src/provider_resolution.rs`
- `src-rust/crates/query/src/agent_tool.rs`
- `src-rust/crates/api/src/registry.rs`
- `src-rust/crates/tools/src/lib.rs`
- Symbols verified:
- `run_query_loop(...)`
- `QueryConfig::provider_registry`
- `resolve_provider_identity(...)`
- `materialize_provider(...)`
- `ProviderResolutionError::NoCredentials`
- `provider_registry_none_uses_legacy_anthropic_client_path()`
- Commands run:
- `git branch --show-current`
- `git status --short --branch`
- `git diff --name-only`
- `git diff --cached --name-only`
- `git log --oneline --decorate -n 20`
- `sed`
- `rg`
- `find`
- `date -u +%Y%m%dT%H%M%SZ`

## Current Code Reality
- `run_query_loop(...)` in `src-rust/crates/query/src/lib.rs:675` has an explicit registry-backed branch at [src-rust/crates/query/src/lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:871).
- That branch calls `resolve_provider_identity(...)` at [src-rust/crates/query/src/lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:875) and immediately returns `QueryOutcome::Error(ClaudeError::Api(...))` on failure at lines 881-884.
- The same branch calls `materialize_provider(...)` at [src-rust/crates/query/src/lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:887) and immediately returns `QueryOutcome::Error(ClaudeError::Api(...))` on failure at lines 893-900.
- A code comment at lines 871-873 explicitly states that failures in the registry-backed path "do not fall through to the raw Anthropic client path below."
- The legacy Anthropic call site remains below that branch at [src-rust/crates/query/src/lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:1291).
- `resolve_provider_identity(...)` in [src-rust/crates/query/src/provider_resolution.rs](/home/jordi/claurst/src-rust/crates/query/src/provider_resolution.rs:101) only defaults to Anthropic when there is no explicit provider, no model-string provider prefix, and no model-registry match.
- `materialize_provider(...)` in [src-rust/crates/query/src/provider_resolution.rs](/home/jordi/claurst/src-rust/crates/query/src/provider_resolution.rs:157) returns `ProviderResolutionError::NoCredentials(identity.provider_id.clone())` when neither runtime auth-backed provider nor registry provider is available at lines 205-207.
- Root test helpers already exist in `src-rust/crates/query/src/lib.rs`:
- `make_tool_context(parent_provider: Option<&str>)` at [src-rust/crates/query/src/lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:2182) injects the root provider hint through `tool_ctx.config.provider`.
- `run_root_query(...)` at [src-rust/crates/query/src/lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:2204) exercises `run_query_loop(...)` through the real root test harness.

## Existing No-Fallback Test Audit
- `provider_registry_none_uses_legacy_anthropic_client_path` in [src-rust/crates/query/src/lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:2359)
- Covers the opposite branch from `M9-08`: `provider_registry = None` on the root path.
- Strongly proves legacy Anthropic path is still used when registry is absent.
- Does not cover `provider_registry = Some(...)`.
- Not `M9-08` equivalent.
- `materialize_provider_returns_no_credentials_for_unknown_provider` in [src-rust/crates/query/src/provider_resolution.rs](/home/jordi/claurst/src-rust/crates/query/src/provider_resolution.rs:648)
- Covers seam-only materialization failure for an explicit fake provider against an empty registry.
- Proves seam error shape only.
- Does not prove root branch entry, root error propagation, or no legacy fallback.
- Overlapping but weaker than `M9-08`.
- `materialize_provider_returns_no_credentials_for_known_provider_without_auth` in [src-rust/crates/query/src/provider_resolution.rs](/home/jordi/claurst/src-rust/crates/query/src/provider_resolution.rs:774)
- Covers seam-only materialization failure for `openai` without auth and without registry provider.
- Stronger input realism than the fake-provider test.
- Still does not prove root branch entry or absence of legacy Anthropic fallthrough.
- Overlapping but weaker than `M9-08`.
- `agent_tool_errors_when_provider_registry_missing` in [src-rust/crates/query/src/agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:933)
- Worker-path contrast only.
- Proves agent path hard-errors when registry is missing.
- Not root-path coverage and not `provider_registry = Some(...)`.
- Exact `M9-08` equivalent coverage already exists: `no`.
- Weak or indirect current coverage:
- The seam-level `materialize_provider_*no_credentials*` tests prove generic seam error generation.
- The existing root-path `provider_registry_none_*` test proves the opposite branch.
- No current test proves the specific root-path `provider_registry = Some(...)` failure branch returns early without using the legacy Anthropic client.
- Current tests that only prove generic error behavior rather than specific root no-fallback behavior:
- `materialize_provider_returns_no_credentials_for_unknown_provider`
- `materialize_provider_returns_no_credentials_for_known_provider_without_auth`

## M9-08 Coverage Matrix
| Required case | Existing test name(s) | Status | Basis for classification | Likely assertion needed if follow-up execution is required |
|---|---|---|---|---|
| `provider_registry = Some(...)` and requested provider not available -> hard error result | `materialize_provider_returns_no_credentials_for_unknown_provider`; `materialize_provider_returns_no_credentials_for_known_provider_without_auth` | `PARTIAL` | Seam-only tests prove `materialize_provider(...)` emits `NoCredentials`, but no root-path test currently asserts `run_query_loop(...)` returns `QueryOutcome::Error` in this branch. | Add a root test that sets `config.provider_registry = Some(Arc::new(ProviderRegistry::new()))`, requests `openai`, and asserts `QueryOutcome::Error` contains `No credentials available for provider 'openai'`. |
| No legacy Anthropic fallback when `provider_registry` is `Some(...)` and resolution/materialization fails | `provider_registry_none_uses_legacy_anthropic_client_path` | `MISSING` | Existing root test proves only the `None` branch uses legacy Anthropic. There is no root-path test for the `Some(...)` failure branch proving non-fallthrough. | In the same root test, assert the error text does not contain the legacy Anthropic markers from `M9-07`, especially `Authentication error: No API key for the selected model.` |

## Likely Smallest Edit Surface For Execution
- Smallest correct edit surface appears to be `src-rust/crates/query/src/lib.rs` under the existing `#[cfg(test)]` module.
- Reason:
- The root harness already exists there through `make_tool_context(...)`, `with_isolated_provider_auth(...)`, and `run_root_query(...)`.
- The authority-hinted `src-rust/crates/query/tests/` surface is absent in current repo reality.
- Creating a new integration-test surface would be larger than necessary for this ticket.
- Production code changes appear unnecessary.

## Validation Readiness
- Current production behavior already appears correct for `M9-08`.
- `M9-08` does not appear audit-only / validation-only because exact root-path coverage is still missing.
- Expected follow-up is a small local test addition only.
- Recommended future test shape:
- Use `provider_registry: Some(Arc::new(ProviderRegistry::new()))`.
- Use isolated auth with no `ANTHROPIC_API_KEY` and no `OPENAI_API_KEY`.
- Request `openai` via the root provider hint (`tool_ctx.config.provider`) and model `gpt-4o`.
- Assert returned error mentions `No credentials available for provider 'openai'`.
- Assert returned error does not contain the legacy Anthropic auth markers proven by `M9-07`.
- Stable future validation filter recommendation:
- Prefer a narrow name substring such as `provider_registry_some_resolution_failure`.

## Drift Found
- Minor path drift:
- `src-rust/crates/query/tests/` does not exist.
- Current root-path tests live in `src-rust/crates/query/src/lib.rs` under `#[cfg(test)]`.
- Minor contract-shape drift:
- Ticket wording says "request a provider not in it."
- Current seam implementation does not emit `ProviderNotFound` for this case.
- Instead, absence of both runtime auth-backed provider and registry provider yields `ProviderResolutionError::NoCredentials(...)`.
- This does not block the ticket objective because the required observable is hard error plus no legacy fallback, not a specific enum variant name in the ticket text.

## Blockers
- `none`

## Notes
- The `provider_registry = Some(...)` branch in `run_query_loop(...)` is already implemented with early return on seam failure.
- The missing piece is proof, not behavior.
- The strongest deterministic `M9-08` follow-up appears to be a root-path unit test that uses a real empty `ProviderRegistry`, requests `openai`, and asserts both:
- positive seam-failure signal: `No credentials available for provider 'openai'`
- negative no-fallback signal: absence of the legacy Anthropic auth error text used by `M9-07`
- This keeps scope within the active ticket and avoids any production edits.
