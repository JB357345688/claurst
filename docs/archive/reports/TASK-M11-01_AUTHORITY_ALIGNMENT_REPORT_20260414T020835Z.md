# TASK-M11-01 Authority Alignment Report

## Ticket ID

`TASK-M11-01 — TrustDomain enum`

## Files Reviewed

- `AGENTS.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md`
- `docs/archive/reports/TASK-M11-01_PREFLIGHT_REPORT_20260414T015011Z.md`
- `docs/archive/reports/M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md`

## Exact Authority Mismatch Found

The latest preflight report established that `TASK-M11-01` was blocked because the approved hardcoded local-provider mapping was narrower than live repo reality.

Authority wording before correction:

- `"ollama" | "lmstudio" | "llamacpp" => Local`
- all others => `Cloud`

Verified live repo reality from the authoritative preflight:

- canonical runtime / registry-backed local provider IDs:
  - `"ollama"`
  - `"lm-studio"`
  - `"llama-cpp"`
- existing alias / user-facing forms still present in repo:
  - `"lmstudio"`
  - `"llamacpp"`

Because later M11 fallback logic is expected to call `TrustDomain::for_provider()` on provider IDs enumerated from `provider_registry.provider_ids()`, the narrower wording would misclassify canonical local registry IDs `"lm-studio"` and `"llama-cpp"` as `Cloud`.

## Exact Wording Change Made

Updated `docs/Current/MPWO_WORK_ORDER_PACK.md` in the `TASK-M11-01` authority summary so it now explicitly approves the corrected hardcoded local mapping:

- `"ollama" | "lmstudio" | "lm-studio" | "llamacpp" | "llama-cpp" => Local`
- all others => `Cloud`

The canonical authority file now states this in two places:

1. `Section 6: Milestone 11 Actionable Breakdown` summary bullet for `M11-01`
2. `Ready-for-Codex Statement` basis for `M11-01`

No unrelated ticket wording was revised.

## Why The Change Is Required For Registry-Backed Correctness

`TrustDomain::for_provider()` is planned to classify provider IDs that come from the provider registry, not only user-entered aliases. The registry-backed canonical provider IDs for LM Studio and llama.cpp are hyphenated. If the authority continued to approve only the unhyphenated aliases, a correct implementation against the literal old wording would still be wrong at runtime for registry-enumerated providers.

This correction aligns the ticket with verified repo reality while preserving the intended narrow design:

- still hardcoded only
- still no config system
- still no YAML
- still no env-var driven trust-domain configuration
- still no custom-provider trust-domain support beyond default `Cloud`

## Whether Scope Changed Materially

`no`

This is an authority-alignment correction, not a feature expansion. It does not add new behavior classes, new files, new configuration surfaces, or later-ticket logic. It only broadens the hardcoded local match to include the already-existing canonical provider IDs that the live registry uses.

## Whether M11-01 Is Now Ready For Execution

`yes`

Basis:

- `docs/Current/MPWO_WORK_ORDER_PACK.md` remains the live canonical execution authority.
- The blocker recorded in `docs/archive/reports/TASK-M11-01_PREFLIGHT_REPORT_20260414T015011Z.md` has been resolved in the canonical authority text.
- The correction is narrow and keeps `TASK-M11-01` within its original single-file, hardcoded-match scope.

## Recommended Next Step

Execute `TASK-M11-01` against the corrected canonical authority, using the hardcoded local mapping exactly as now approved in `docs/Current/MPWO_WORK_ORDER_PACK.md`:

- `"ollama" | "lmstudio" | "lm-studio" | "llamacpp" | "llama-cpp" => Local`
- all others => `Cloud`

Treat the older wording in `docs/archive/reports/M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md` as superseded by the corrected canonical MPWO on this point.
