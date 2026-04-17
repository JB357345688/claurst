# TASK-M9-05 Preflight Report

## Ticket
- `TASK-M9-05`

## Timestamp UTC
- `2026-04-13T08:09:28Z`

## Branch
- Expected: `feature/provider-resolution-seam`
- Actual: `feature/provider-resolution-seam`

## Verdict
- `READY-WITH-NOTES`

## Repo State Summary
- `git branch --show-current` matched the expected branch: `feature/provider-resolution-seam`
- `git status --short --branch` showed a noisy worktree with untracked files and directories, but no tracked edits in the active ticket file
- `git diff --name-only` returned no tracked unstaged diffs
- `git diff --cached --name-only` returned no staged diffs
- `git diff --name-only -- src-rust/crates/query/src/provider_resolution.rs` returned no diff for the active file
- `git diff --cached --name-only -- src-rust/crates/query/src/provider_resolution.rs` returned no staged diff for the active file
- `git status --porcelain | rg -c '^\?\?'` reported `116` untracked entries
- `git log --oneline --decorate -n 20` shows the accepted baseline and later M9 work on this branch:
  - `2f1f169` `TASK-M9-04 prove agent inherits parent provider on openai dispatch`
  - `c28ef22` `TASK-M9-03 prove agent explicit provider routes to openai`
  - `662b29a` `TASK-M9-02 add materialize provider coverage tests`
  - `b5b6dd4` `TASK-M8-11 reconcile M8 workspace validation and formatting`
- `git show --stat --oneline --summary` confirmed:
  - `662b29a` touched `src-rust/crates/query/src/provider_resolution.rs`
  - `c28ef22` and `2f1f169` touched `src-rust/crates/query/src/agent_tool.rs`, not the active M9-05 file

## Authority Reviewed
- Repo-local [AGENTS.md](/home/jordi/claurst/AGENTS.md:1) was reviewed as controlling repo authority
- [docs/Current/MPWO_WORK_ORDER_PACK.md](/home/jordi/claurst/docs/Current/MPWO_WORK_ORDER_PACK.md:1627) was reviewed as controlling ticket authority
- Relevant repo authority confirmed:
  - execute exactly one ticket at a time
  - verify files, symbols, commands, and drift before editing
  - keep scope limited to the active ticket
  - preflight must report verdict, verified files/symbols/commands, drift, and blockers
- Relevant ticket authority confirmed:
  - objective is tests for `P3` and `P5`
  - target file is the local `#[cfg(test)]` surface in provider resolution
  - do not test unknown-provider conflicts
  - validation target later is only that tests pass

## Dependency Baseline Confirmed
- Branch history includes `TASK-M8-11` commit `b5b6dd4`, so the required dependency is present in current branch reality
- Branch history also includes committed and verified follow-on M9 work:
  - `TASK-M9-02` commit `662b29a`
  - `TASK-M9-03` commit `c28ef22`
  - `TASK-M9-04` commit `2f1f169`
- Earlier `M9-01` evidence exists in reports and already treated `P3` and `P5` as covered and passing:
  - [TASK-M9-01_PREFLIGHT_REPORT_20260413T045856Z.md](/home/jordi/claurst/docs/archive/reports/TASK-M9-01_PREFLIGHT_REPORT_20260413T045856Z.md:104)
  - [TASK-M9-01_EXECUTION_REPORT_20260413T050804Z.md](/home/jordi/claurst/docs/archive/reports/TASK-M9-01_EXECUTION_REPORT_20260413T050804Z.md:45)
- `M9-01` through `M9-04` do not need reopening for this preflight
- The hosted-Ollama compatibility baseline remains a preserved background invariant only; nothing in the active ticket requires reopening or revalidating that surface during preflight

## Exact M9-05 Contract
- Objective: tests for provider/model conflict cases `P3` and `P5`
- Required case 1:
  - `resolve_provider_identity(Some("openai"), "anthropic/claude-sonnet-4-20250514", ...)`
  - expected result: `ProviderModelConflict`
- Required case 2:
  - `resolve_provider_identity(Some("anthropic"), "openai/gpt-4o", ...)`
  - expected result: `ProviderModelConflict`
- Required edit surface later, if any: local `#[cfg(test)]` only in `src-rust/crates/query/src/provider_resolution.rs`
- Explicit exclusions:
  - do not test conflicts with unknown providers
  - do not widen into integration tests
  - do not change production code

## Verified Files / Symbols / Commands
- Files reviewed:
  - [AGENTS.md](/home/jordi/claurst/AGENTS.md:1)
  - [docs/Current/MPWO_WORK_ORDER_PACK.md](/home/jordi/claurst/docs/Current/MPWO_WORK_ORDER_PACK.md:1627)
  - [src-rust/crates/query/src/provider_resolution.rs](/home/jordi/claurst/src-rust/crates/query/src/provider_resolution.rs:1)
  - [TASK-M9-01_PREFLIGHT_REPORT_20260413T045856Z.md](/home/jordi/claurst/docs/archive/reports/TASK-M9-01_PREFLIGHT_REPORT_20260413T045856Z.md:75)
  - [TASK-M9-01_EXECUTION_REPORT_20260413T050804Z.md](/home/jordi/claurst/docs/archive/reports/TASK-M9-01_EXECUTION_REPORT_20260413T050804Z.md:40)
- Symbols verified in current repo reality:
  - `KNOWN_PROVIDERS`
  - `ProviderResolutionError::ProviderModelConflict`
  - `resolve_provider_identity(...)`
  - local `#[cfg(test)] mod tests`
  - `assert_provider_model_conflict(...)`
  - `assert_identity(...)`
  - `provider_identity(...)`
- Commands executed:
  - `git branch --show-current`
  - `git status --short --branch`
  - `git diff --name-only`
  - `git diff --cached --name-only`
  - `git log --oneline --decorate -n 20`
  - `git diff --name-only -- src-rust/crates/query/src/provider_resolution.rs`
  - `git diff --cached --name-only -- src-rust/crates/query/src/provider_resolution.rs`
  - `git show --stat --oneline --summary 662b29a`
  - `git show --stat --oneline --summary c28ef22`
  - `git show --stat --oneline --summary 2f1f169`
  - `rg -n "TASK-M9-05|M9-05|P3|P5|provider/model conflicts|Agent conflict detection tests" docs/Current/MPWO_WORK_ORDER_PACK.md`
  - `rg -n "resolve_provider_identity|assert_provider_model_conflict|assert_identity|provider_identity|ProviderModelConflict|#\\[cfg\\(test\\)\\]" src-rust/crates/query/src/provider_resolution.rs`
  - `rg -n "P3|P5|ProviderModelConflict|conflict" docs/archive/reports/TASK-M9-0{1,2,3,4}_*.md`

## Current Code Reality
- [src-rust/crates/query/src/provider_resolution.rs](/home/jordi/claurst/src-rust/crates/query/src/provider_resolution.rs:6) defines `KNOWN_PROVIDERS`, which includes both `anthropic` and `openai`
- [src-rust/crates/query/src/provider_resolution.rs](/home/jordi/claurst/src-rust/crates/query/src/provider_resolution.rs:101) defines:
  - `resolve_provider_identity(explicit_provider: Option<&str>, model: &str, model_registry: Option<&ModelRegistry>)`
- The conflict branch in [provider_resolution.rs](/home/jordi/claurst/src-rust/crates/query/src/provider_resolution.rs:106) behaves exactly as the ticket expects:
  - when `explicit_provider` is present
  - and `model` contains a slash
  - and the prefix is in `KNOWN_PROVIDERS`
  - and the prefix differs from the explicit provider
  - the function returns `ProviderResolutionError::ProviderModelConflict { provider, model, model_provider }`
- That same branch explicitly excludes unknown-provider prefixes from the error path, because it only errors when `KNOWN_PROVIDERS.contains(&model_provider)` is true
- The local [#[cfg(test)] module](/home/jordi/claurst/src-rust/crates/query/src/provider_resolution.rs:254) exists at the expected location
- Existing helper functions in the test module already support exact conflict assertions:
  - [assert_identity(...)](/home/jordi/claurst/src-rust/crates/query/src/provider_resolution.rs:414)
  - [assert_provider_model_conflict(...)](/home/jordi/claurst/src-rust/crates/query/src/provider_resolution.rs:430)
  - [provider_identity(...)](/home/jordi/claurst/src-rust/crates/query/src/provider_resolution.rs:450)
- `assert_provider_model_conflict(...)` is strong enough for this ticket because it asserts:
  - correct error variant: `ProviderModelConflict`
  - correct explicit provider field
  - correct raw model string field
  - correct model-provider prefix field

## Existing Conflict Test Audit
- Relevant current conflict test names:
  - [p3_explicit_provider_conflicts_with_model_prefix](/home/jordi/claurst/src-rust/crates/query/src/provider_resolution.rs:522)
  - [p5_explicit_provider_conflicts_with_reverse_model_prefix](/home/jordi/claurst/src-rust/crates/query/src/provider_resolution.rs:539)
- What each test covers:
  - `p3_explicit_provider_conflicts_with_model_prefix`
    - calls `assert_provider_model_conflict("openai", "anthropic/claude-sonnet-4-20250514", "anthropic")`
    - this is the exact required M9-05 `P3` input and expected provider-prefix conflict
  - `p5_explicit_provider_conflicts_with_reverse_model_prefix`
    - calls `assert_provider_model_conflict("anthropic", "openai/gpt-4o", "openai")`
    - this is the exact required M9-05 `P5` input and expected reverse-direction provider-prefix conflict
- Whether exact M9-05-equivalent coverage already exists:
  - yes, both required cases already exist with the exact required inputs
- Whether any tests are weak, indirect, or overlapping:
  - neither `P3` nor `P5` is weak or indirect; both are direct unit tests of the precise conflict branch
  - they intentionally overlap as mirrored conflict directions; this is required by the ticket, not accidental redundancy
  - no weaker substitute test is being used instead of the required exact cases
- Whether current assertions match the contract exactly enough:
  - yes
  - the helper checks the exact `ProviderModelConflict` variant and all relevant fields, including the unmodified raw model string
- Whether unknown-provider conflict behavior is intentionally excluded:
  - yes
  - there is no explicit-provider unknown-prefix conflict test in this local test surface
  - current coverage does include [p7_no_provider_with_unknown_namespace_defaults](/home/jordi/claurst/src-rust/crates/query/src/provider_resolution.rs:557), but that is a separate no-provider default case and does not violate the M9-05 exclusion
- Whether earlier M9 reports already treated `P3` and `P5` as covered and passing:
  - yes
  - [TASK-M9-01_PREFLIGHT_REPORT_20260413T045856Z.md](/home/jordi/claurst/docs/archive/reports/TASK-M9-01_PREFLIGHT_REPORT_20260413T045856Z.md:104) marked both rows `COVERED`
  - [TASK-M9-01_EXECUTION_REPORT_20260413T050804Z.md](/home/jordi/claurst/docs/archive/reports/TASK-M9-01_EXECUTION_REPORT_20260413T050804Z.md:45) recorded both rows as present and passed

## M9-05 Coverage Matrix
| Required case | Existing test name(s), if any | Status | Basis for classification | Likely assertion needed if follow-up execution is required |
|---|---|---|---|---|
| `openai + anthropic/claude-sonnet-4-20250514 -> ProviderModelConflict` | `p3_explicit_provider_conflicts_with_model_prefix` | `COVERED` | Exact input is already present; helper asserts the `ProviderModelConflict` variant plus `provider == "openai"`, `model == "anthropic/claude-sonnet-4-20250514"`, and `model_provider == "anthropic"` | None; current assertion already matches the contract strongly enough |
| `anthropic + openai/gpt-4o -> ProviderModelConflict` | `p5_explicit_provider_conflicts_with_reverse_model_prefix` | `COVERED` | Exact input is already present; helper asserts the `ProviderModelConflict` variant plus `provider == "anthropic"`, `model == "openai/gpt-4o"`, and `model_provider == "openai"` | None; current assertion already matches the contract strongly enough |

## Likely Smallest Edit Surface For Execution
- Smallest correct edit surface remains the existing local `#[cfg(test)]` module in [provider_resolution.rs](/home/jordi/claurst/src-rust/crates/query/src/provider_resolution.rs:254)
- Based on current read-only evidence, `TASK-M9-05` likely requires no source edit at all
- The most likely later execution shape is audit-only / validation-only:
  - confirm the existing `P3` and `P5` tests are the intended contract coverage
  - run the ticket’s validation target
  - do not change production code

## Validation Readiness
- Later validation target from ticket authority remains: tests pass
- Validation was intentionally not run in this preflight session
- Current naming supports a conflict-specific filter if later execution wants a narrow run:
  - recommended narrow substring: `explicit_provider_conflicts`
  - exact test names are also stable enough for single-case reruns
- A broader `provider_resolution` filter would work, but it is less precise than the existing conflict-specific names

## Drift Found
- Minor path drift only:
  - MPWO wording uses `crates/query/src/provider_resolution.rs`
  - repo reality is `src-rust/crates/query/src/provider_resolution.rs`
  - this is not a structural blocker because the target file exists at the expected logical location inside the workspace layout
- Worktree noise:
  - many untracked files exist under `docs/`, reports, and `src-rust/target/`
  - this should be called out for later review hygiene, but it does not block the current ticket because the active file has no tracked diff
- No structural drift found in the active ticket surface:
  - no missing file
  - no missing or relocated `#[cfg(test)]` section
  - no signature drift in `resolve_provider_identity(...)`
  - no mismatch between the required cases and the current canonical conflict inputs

## Blockers
- None

## Notes
- Read-only evidence indicates that `TASK-M9-05` is already satisfied in current repo reality
- The exact required `P3` and `P5` cases already exist, use the exact required inputs, and assert the exact required error details
- Current evidence supports treating `TASK-M9-05` as an audit/confirmation ticket rather than a source-edit ticket
- If later execution is performed, it should stay within the existing local test module and should not reopen `M9-01` through `M9-04`
- Unknown-provider conflict cases remain intentionally excluded, consistent with the ticket contract
