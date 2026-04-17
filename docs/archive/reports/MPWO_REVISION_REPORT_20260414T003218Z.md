# MPWO Revision Report

## Timestamp UTC

`2026-04-14T00:32:18Z`

## Branch Observed

`feature/provider-resolution-seam`

HEAD: `6b362a0 TASK-M9-12 complete D1 provider-resolution seam validation and smoke gate`

## Authority Reviewed

- `AGENTS.md` -- full
- `docs/Current/MPWO_WORK_ORDER_PACK.md` -- the canonical execution authority (target of this revision)
- `docs/Orchestrator_planning/IMPLEMENTATION_PLAN_MPWO.md` -- secondary planning guidance (read-only, not modified)
- `docs/archive/reports/D1_REVIEW_REPORT_20260413T233604Z.md` -- D1 accepted baseline
- `docs/archive/reports/M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md` -- accepted M10 D2 plan (source of revision)

## Why MPWO Revision Was Required

The MPWO was originally written before D1 execution and before the M10 D2 planning milestone. The M11 section contained 11 tickets (M11-01 through M11-11) based on the pre-D1 planning expectations.

The accepted M10 D2 plan (`M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md`) was produced after D1 landed and grounded in actual repo reality. It identified the following gaps in the original MPWO M11 section:

1. **Child execution independence was not first-class.** The D1 interim `CHILD_AGENT_FALLBACK_MAX_TOKENS = 4_096` needed to be superseded by a first-class child `max_tokens` setting, not preserved indefinitely.
2. **No standalone ticket for child execution override wiring.** The original M11-09 (schema updates) buried `allow_fallback` and `budget_usd` without a dedicated ticket for `max_tokens` override wiring across all three spawn paths.
3. **Budget/limit mechanism distinction was implicit.** The MPWO did not clearly distinguish `max_tokens` (token count), `max_budget_usd` (per-loop USD), and `SessionBudget` (cross-session USD).
4. **D1-established child model/provider independence was undocumented** in the M11 section, creating ambiguity about what D2 actually needs to add vs what already works.

The revised M10 plan added a new ticket (M11-09: child execution override wiring), renumbered subsequent tickets, and expanded the invariants and risk register. The MPWO must reflect this to remain the sole canonical execution authority.

## Summary of Changes Made to MPWO

### Header (lines 1-8)
- Added revision date (`2026-04-14`) and reference to the accepted M10 plan.
- Updated scope line to note D1 is complete and M11 is revised per accepted M10 plan.
- Added D1 status line referencing the D1 review report.

### Section 1: Execution Order Summary
- Replaced old M11-09 through M11-11 with new M11-09 through M11-12.
- New M11-09: `Child execution override wiring (max_tokens + spawn-time settings)`
- Old M11-09 renumbered to M11-10 (Schema updates)
- Old M11-10 renumbered to M11-11 (QueryEvent variants)
- Old M11-11 renumbered to M11-12 (D2 test suite)

### Section 2: Dependency Graph
- Updated M11 dependency edges to reflect the new ticket and renumbering.
- Added edges: M11-08 → M11-09, M11-09 → M11-10, M11-09 → M11-11.
- Updated M12 dependency from M11-11 to M11-12.

### Section 6: Milestone 11 Actionable Breakdown (complete rewrite)
- Added preamble documenting:
  - D1-established child independence (model, provider already work)
  - D2 child independence extensions (max_tokens, allow_fallback, budget_usd)
  - Three independent budget/limit mechanisms with explicit non-conflation rule
  - Recommended serial execution order from M10 plan
- All 11 existing tickets revised with enhanced detail from the M10 plan:
  - M11-01: Added derive traits, explicit non-goal about modifying existing types.
  - M11-02: Added `#[serde(default)]` guidance, explicit non-goal about modifying `ModelRegistry` methods.
  - M11-03: Added `provider_supports_capability()` function, explicit non-goals.
  - M11-04: Enhanced with full method signatures and independence note.
  - M11-05: Added `ModelRegistry` helper guidance and secondary file target.
  - M11-06: Added `parking_lot::RwLock` type guidance and setter methods.
  - M11-07: Enhanced with full method signatures and independence note.
  - M11-08: Added `ToolContext` to file list, enhanced steps, added non-goal about `CHILD_AGENT_FALLBACK_MAX_TOKENS` being M11-09 scope.
  - M11-10 (was M11-09): Clarified as remaining schema updates after M11-09, added M11-09 dependency.
  - M11-11 (was M11-10): Added M11-09 dependency.
  - M11-12 (was M11-11): Added child execution override tests (step 6), added `agent_tool.rs` to file list.
- **New ticket inserted: TASK-M11-09 — Child execution override wiring.** Full ticket with:
  - Rationale (supersedes D1 interim)
  - Exact fields: `AgentInput.max_tokens`, `AgentSpec.max_tokens`, `AgentRunParams.max_tokens_override`
  - All three spawn paths covered (foreground, background, team runner)
  - Backward compatibility guarantee (absent = 4096)
  - Child execution independence summary table
  - Explicit non-goals

### Section 9: Global Rules — D2 Boundary
- Rule 13: Added `max_tokens_override` to forbidden-until-M11 list, added note about `CHILD_AGENT_FALLBACK_MAX_TOKENS` supersession.
- New Rule 16: Do not conflate budget/limit mechanisms.
- Communication rules renumbered to 17-18.

### M12 cross-references
- Updated all M11-11 references in M12 tickets to M11-12.

## M11 Ladder Before vs After

### Before (11 tickets):
```
M11-01  TrustDomain enum
M11-02  ModelEntry extension
M11-03  Capability enum and matching
M11-04  HealthCache implementation
M11-05  resolve_provider_with_fallback()
M11-06  CostTracker extension
M11-07  SessionBudget implementation
M11-08  Budget + cancellation wiring
M11-09  Schema updates (allow_fallback, budget_usd)          ← OLD
M11-10  QueryEvent variants                                   ← OLD
M11-11  D2 test suite + workspace validation                  ← OLD
```

### After (12 tickets):
```
M11-01  TrustDomain enum
M11-02  ModelEntry extension
M11-03  Capability enum and matching
M11-04  HealthCache implementation
M11-05  resolve_provider_with_fallback()
M11-06  CostTracker extension
M11-07  SessionBudget implementation
M11-08  Budget + cancellation wiring
M11-09  Child execution override wiring (max_tokens)          ← NEW
M11-10  Schema updates (allow_fallback, budget_usd)           ← was M11-09
M11-11  QueryEvent variants                                   ← was M11-10
M11-12  D2 test suite + workspace validation                  ← was M11-11
```

### Key dependency changes:
- M11-08 → M11-09 (child override depends on budget wiring being in place)
- M11-09 → M11-10 (schema updates depend on child override wiring pattern)
- M11-09 → M11-11 (query events depend on child override wiring)
- M11-10 → M11-12, M11-11 → M11-12 (test suite depends on all above)

## Wording Kept Intentionally Unchanged

1. **Section 2A (Hosted Ollama non-regression invariant):** Kept verbatim. No wording change needed — the invariant applies to all M7-M12 tickets including the new M11-09.
2. **Sections 3-5 (M7, M8, M9 detailed tickets):** Kept verbatim. These are D1 tickets, all complete and accepted. No revision needed.
3. **Section 7 (M12 actionable breakdown):** Kept verbatim except updating M11-11 references to M11-12.
4. **Section 8 (Open Verification Items):** Kept verbatim. All verification items remain applicable.
5. **Section 9 Global Rules (items 1-12, 14-15):** Kept verbatim. Only D2 Boundary (item 13) and Communication (items 17-18) were updated.

## Verdict

**UPDATED**

The MPWO has been revised to align with the accepted M10 D2 implementation plan. The revision is minimal and authoritative: one new ticket inserted, three tickets renumbered, enhanced detail on all M11 tickets, and explicit child execution independence documentation added. No wording was weakened. No scope was widened beyond what the M10 plan supports.

## Ready-for-Codex Statement

**Yes. M11-01 (TrustDomain enum) can proceed against the revised MPWO.**

Basis:
- The revised MPWO is the sole canonical execution authority for M11.
- M11-01 has no code dependencies beyond M10 (complete).
- M11-01's ticket definition in the revised MPWO is fully specified: single file (`crates/api/src/provider_types.rs`), single enum + single match function, explicit non-goals, validation command.
- The M11-01 ticket is unchanged from the original MPWO except for enhanced derive traits and explicit non-goal wording — no structural change.
- The recommended serial execution order places M11-01 first.
