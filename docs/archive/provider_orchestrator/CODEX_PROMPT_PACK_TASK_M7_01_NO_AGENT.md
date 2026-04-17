# Codex Prompt Pack — TASK-M7-01 (No Agent / No Repo Skill Required)

This pack is designed for **fresh Codex sessions per ticket**.

Use it with:
- the full `MPWO_WORK_ORDER_PACK.md` attached or present in repo root
- **no assumption that any repo-local agent/skill is installed**
- only the **active ticket excerpt foregrounded** in the session prompt
- a mandatory **preflight pass** before edits
- a mandatory **validation pass** after edits
- human review before moving to the next dependency edge

This version is fully self-contained. The global rules are embedded directly in the prompts.

---

## 1) Recommended Session Sequence

### Session A — Preflight only
Goal: verify repo reality against the ticket before any edits.

### Session B — Execute only
Goal: implement exactly TASK-M7-01, no more.

### Session C — Review only
Goal: audit the diff against the ticket and global rules.

Do **not** carry one long Codex thread across multiple tickets.

---

## 2) Embedded Global Rules Block

Paste or keep this rules block directly in each fresh Codex session. Do not rely on any installed repo-local skill.

```md
GLOBAL RULES FOR THIS SESSION

You are executing exactly one ticket from `MPWO_WORK_ORDER_PACK.md`.

Operating rules:
1. Work only on the active ticket.
2. Do not expand scope beyond listed files.
3. Do not rewrite adjacent systems.
4. Do not replace explicit behavior with a cleaner abstraction unless the ticket requires it.
5. Do not silently fix unrelated issues.
6. Do not add comments, docstrings, or type annotations to untouched code.
7. Do not add error handling, fallbacks, or validation beyond the ticket.
8. Do not continue past a failed validation command.
9. Do not skip validation commands.
10. Do not guess when repo reality conflicts with the ticket.
11. Do not invent types, modules, helpers, or abstractions not specified in the ticket.
12. Do not silently resolve uncertainty; verify first.
13. During M7 and M8, do not introduce any D2 concepts. Forbidden until M11:
   - TrustDomain
   - HealthCache
   - SessionBudget
   - Capability
   - allow_fallback
   - budget_usd
   - resolve_provider_with_fallback()
14. Do not remove `client: &AnthropicClient` from `run_query_loop()`.
15. Do not remove legacy compatibility paths.
16. If blocked, report:
   - what you expected
   - what you found
   - what you tried
   - why you stopped
17. When complete, report:
   - ticket id
   - files changed
   - validations run
   - validation results
   - deviations from ticket
   - blockers

Execution discipline:
- First verify the exact files, symbols, and assumptions referenced by the ticket.
- If drift is only line-number drift, adapt.
- If drift is structural, stop and report.
- Never perform adjacent cleanup.
- Never continue into the next ticket.
```

---

## 3) Active Ticket Summary — TASK-M7-01

### Objective
Create `crates/query/src/provider_resolution.rs` and define the scaffolding for:
- `ResolutionSource`
- `ProviderIdentity`
- `ExecutionTarget`
- `ProviderResolutionError`
- `KNOWN_PROVIDERS`
- stub signatures for:
  - `resolve_provider_identity(...)`
  - `materialize_provider(...)`

### Allowed file changes
- `crates/query/src/provider_resolution.rs` — new file
- `crates/query/src/lib.rs` — module declaration and pub use only
- `crates/query/Cargo.toml` — only if `thiserror` is missing

### Explicitly forbidden
- implementing function bodies
- touching any other file
- introducing any D2 concepts
- editing existing `lib.rs` logic beyond module wiring

### Validation command
```bash
cd src-rust && cargo check -p claurst-query
```

### Verification items relevant to this ticket
- Verify `thiserror` presence in `crates/query/Cargo.toml`
- Verify `KNOWN_PROVIDERS` against both:
  - inline known-providers list in `lib.rs`
  - `provider_from_key()` match arms in `registry.rs`
- Verify `ProviderRegistry`, `ModelRegistry`, and `LlmProvider` are importable from `claurst_api`
- Verify `lib.rs` has the expected `pub mod` / `pub use` area near the top before inserting anything

---

## 4) Session A — Preflight Prompt

```text
You are working on exactly one ticket from MPWO_WORK_ORDER_PACK.md: TASK-M7-01.

Apply these session rules first. Do not assume any repo-local agent or skill is installed:

GLOBAL RULES FOR THIS SESSION

You are executing exactly one ticket from `MPWO_WORK_ORDER_PACK.md`.

Operating rules:
1. Work only on the active ticket.
2. Do not expand scope beyond listed files.
3. Do not rewrite adjacent systems.
4. Do not replace explicit behavior with a cleaner abstraction unless the ticket requires it.
5. Do not silently fix unrelated issues.
6. Do not add comments, docstrings, or type annotations to untouched code.
7. Do not add error handling, fallbacks, or validation beyond the ticket.
8. Do not continue past a failed validation command.
9. Do not skip validation commands.
10. Do not guess when repo reality conflicts with the ticket.
11. Do not invent types, modules, helpers, or abstractions not specified in the ticket.
12. Do not silently resolve uncertainty; verify first.
13. During M7 and M8, do not introduce any D2 concepts. Forbidden until M11:
   - TrustDomain
   - HealthCache
   - SessionBudget
   - Capability
   - allow_fallback
   - budget_usd
   - resolve_provider_with_fallback()
14. Do not remove `client: &AnthropicClient` from `run_query_loop()`.
15. Do not remove legacy compatibility paths.
16. If blocked, report:
   - what you expected
   - what you found
   - what you tried
   - why you stopped
17. When complete, report:
   - ticket id
   - files changed
   - validations run
   - validation results
   - deviations from ticket
   - blockers

Execution discipline:
- First verify the exact files, symbols, and assumptions referenced by the ticket.
- If drift is only line-number drift, adapt.
- If drift is structural, stop and report.
- Never perform adjacent cleanup.
- Never continue into the next ticket.

Do not edit anything yet.

Task:
Perform a preflight verification pass for TASK-M7-01 only.

What to verify:
1. `crates/query/src/provider_resolution.rs` does not already exist.
2. `crates/query/src/lib.rs` has the expected `pub mod` / `pub use` area near the top so the new module can be added there cleanly.
3. `claurst_api::LlmProvider`, `claurst_api::ProviderRegistry`, and `claurst_api::ModelRegistry` are importable from `claurst-query`.
4. `crates/query/Cargo.toml` already has `thiserror`; if not, report that it must be added.
5. `KNOWN_PROVIDERS` must be built from the union of:
   - the inline known-providers list in `crates/query/src/lib.rs`
   - all `provider_from_key()` match arms in the relevant registry source
6. Confirm the union list to use for TASK-M7-01.

Hard rules:
- Do not edit files.
- Do not suggest broader refactors.
- Do not move to M7-02.
- If repo reality conflicts structurally with the ticket, stop.

Return exactly these sections:
- Preflight verdict
- Verified paths and symbols
- Provider list confirmation
- Required adjustments before execution
- Blockers
```

---

## 5) Session B — Execute Prompt

```text
You are working on exactly one ticket from MPWO_WORK_ORDER_PACK.md: TASK-M7-01.

Apply these session rules first. Do not assume any repo-local agent or skill is installed:

GLOBAL RULES FOR THIS SESSION

You are executing exactly one ticket from `MPWO_WORK_ORDER_PACK.md`.

Operating rules:
1. Work only on the active ticket.
2. Do not expand scope beyond listed files.
3. Do not rewrite adjacent systems.
4. Do not replace explicit behavior with a cleaner abstraction unless the ticket requires it.
5. Do not silently fix unrelated issues.
6. Do not add comments, docstrings, or type annotations to untouched code.
7. Do not add error handling, fallbacks, or validation beyond the ticket.
8. Do not continue past a failed validation command.
9. Do not skip validation commands.
10. Do not guess when repo reality conflicts with the ticket.
11. Do not invent types, modules, helpers, or abstractions not specified in the ticket.
12. Do not silently resolve uncertainty; verify first.
13. During M7 and M8, do not introduce any D2 concepts. Forbidden until M11:
   - TrustDomain
   - HealthCache
   - SessionBudget
   - Capability
   - allow_fallback
   - budget_usd
   - resolve_provider_with_fallback()
14. Do not remove `client: &AnthropicClient` from `run_query_loop()`.
15. Do not remove legacy compatibility paths.
16. If blocked, report:
   - what you expected
   - what you found
   - what you tried
   - why you stopped
17. When complete, report:
   - ticket id
   - files changed
   - validations run
   - validation results
   - deviations from ticket
   - blockers

Execution discipline:
- First verify the exact files, symbols, and assumptions referenced by the ticket.
- If drift is only line-number drift, adapt.
- If drift is structural, stop and report.
- Never perform adjacent cleanup.
- Never continue into the next ticket.

Assume preflight has already been run. Re-check anything you need before editing.

Implement TASK-M7-01 exactly as specified.

Required changes:
1. Create `crates/query/src/provider_resolution.rs`.
2. Add these imports:
   - `use std::sync::Arc;`
   - `use claurst_api::{LlmProvider, ProviderRegistry, ModelRegistry};`
   - `use claurst_core::ProviderId;`
3. Define `KNOWN_PROVIDERS` as the verified union list from preflight.
4. Add:
   - `ResolutionSource`
   - `ProviderIdentity`
   - `ExecutionTarget`
   - `ProviderResolutionError`
5. Add stub functions only:
   - `resolve_provider_identity(...) -> Result<ProviderIdentity, ProviderResolutionError>` with `todo!("M7-02")`
   - `materialize_provider(...) -> Result<ExecutionTarget, ProviderResolutionError>` with `todo!("M7-03")`
6. In `crates/query/src/lib.rs` add:
   - `pub mod provider_resolution;`
   - `pub use provider_resolution::*;`
7. Only if needed, add `thiserror` to `crates/query/Cargo.toml`.
8. Run the required validation command:
   - `cd src-rust && cargo check -p claurst-query`

Hard constraints:
- Do not implement function bodies.
- Do not add extra helpers.
- Do not touch any file outside the ticket scope.
- Do not introduce D2 concepts.
- Do not continue if validation fails.
- Do not begin M7-02.

Final report format:
- Ticket
- Files changed
- Exact changes made
- Validation commands run
- Validation results
- Deviations from ticket
- Blockers
```

---

## 6) Session C — Review Prompt

```text
Review the current diff only against TASK-M7-01 from MPWO_WORK_ORDER_PACK.md.

Apply these session rules first. Do not assume any repo-local agent or skill is installed:

GLOBAL RULES FOR THIS SESSION

You are executing exactly one ticket from `MPWO_WORK_ORDER_PACK.md`.

Operating rules:
1. Work only on the active ticket.
2. Do not expand scope beyond listed files.
3. Do not rewrite adjacent systems.
4. Do not replace explicit behavior with a cleaner abstraction unless the ticket requires it.
5. Do not silently fix unrelated issues.
6. Do not add comments, docstrings, or type annotations to untouched code.
7. Do not add error handling, fallbacks, or validation beyond the ticket.
8. Do not continue past a failed validation command.
9. Do not skip validation commands.
10. Do not guess when repo reality conflicts with the ticket.
11. Do not invent types, modules, helpers, or abstractions not specified in the ticket.
12. Do not silently resolve uncertainty; verify first.
13. During M7 and M8, do not introduce any D2 concepts. Forbidden until M11:
   - TrustDomain
   - HealthCache
   - SessionBudget
   - Capability
   - allow_fallback
   - budget_usd
   - resolve_provider_with_fallback()
14. Do not remove `client: &AnthropicClient` from `run_query_loop()`.
15. Do not remove legacy compatibility paths.
16. If blocked, report:
   - what you expected
   - what you found
   - what you tried
   - why you stopped
17. When complete, report:
   - ticket id
   - files changed
   - validations run
   - validation results
   - deviations from ticket
   - blockers

Execution discipline:
- First verify the exact files, symbols, and assumptions referenced by the ticket.
- If drift is only line-number drift, adapt.
- If drift is structural, stop and report.
- Never perform adjacent cleanup.
- Never continue into the next ticket.

Audit for:
1. Scope compliance
2. Exact file compliance
3. Correct provider union list
4. Stub-only compliance (`todo!("M7-02")`, `todo!("M7-03")`)
5. No D2 leakage
6. `lib.rs` only changed for module wiring
7. Validation command was run and passed

Do not broaden the review beyond TASK-M7-01.
Do not rewrite code unless needed to correct a ticket violation.

Return exactly:
- Review verdict: PASS or FAIL
- Violations
- Minimal corrective actions
- Ready for human review: YES or NO
```

---

## 7) Optional One-Shot Wrapper Prompt

Use this only if you want preflight + execute in one fresh session but still in two explicit phases.

```text
You are executing TASK-M7-01 from MPWO_WORK_ORDER_PACK.md.

Do not assume any repo-local agent or skill is installed. Apply these rules in-session:

GLOBAL RULES FOR THIS SESSION

You are executing exactly one ticket from `MPWO_WORK_ORDER_PACK.md`.

Operating rules:
1. Work only on the active ticket.
2. Do not expand scope beyond listed files.
3. Do not rewrite adjacent systems.
4. Do not replace explicit behavior with a cleaner abstraction unless the ticket requires it.
5. Do not silently fix unrelated issues.
6. Do not add comments, docstrings, or type annotations to untouched code.
7. Do not add error handling, fallbacks, or validation beyond the ticket.
8. Do not continue past a failed validation command.
9. Do not skip validation commands.
10. Do not guess when repo reality conflicts with the ticket.
11. Do not invent types, modules, helpers, or abstractions not specified in the ticket.
12. Do not silently resolve uncertainty; verify first.
13. During M7 and M8, do not introduce any D2 concepts. Forbidden until M11:
   - TrustDomain
   - HealthCache
   - SessionBudget
   - Capability
   - allow_fallback
   - budget_usd
   - resolve_provider_with_fallback()
14. Do not remove `client: &AnthropicClient` from `run_query_loop()`.
15. Do not remove legacy compatibility paths.
16. If blocked, report:
   - what you expected
   - what you found
   - what you tried
   - why you stopped
17. When complete, report:
   - ticket id
   - files changed
   - validations run
   - validation results
   - deviations from ticket
   - blockers

Execution discipline:
- First verify the exact files, symbols, and assumptions referenced by the ticket.
- If drift is only line-number drift, adapt.
- If drift is structural, stop and report.
- Never perform adjacent cleanup.
- Never continue into the next ticket.

Work in two phases:

Phase 1: preflight verification only.
- Verify all ticket assumptions.
- If there is structural drift, stop and report.
- If preflight passes, state that execution is safe.

Phase 2: implement exactly TASK-M7-01.
- Make only the allowed file changes.
- Add stub functions only.
- Run `cd src-rust && cargo check -p claurst-query`.
- Stop on failure.

Never move to M7-02.
Never perform adjacent cleanup.
Never introduce D2 concepts.

Final output sections:
- Preflight verdict
- Files changed
- Validation results
- Deviations from ticket
- Blockers
```

---

## 8) Human Gate Before M7-02

Do not start M7-02 until a human confirms all of the following:
- `provider_resolution.rs` exists and contains only the requested scaffolding
- `lib.rs` wiring is minimal and correct
- `KNOWN_PROVIDERS` was built from the verified union, not guessed
- `cargo check -p claurst-query` passed
- no unrelated files changed

---

## 9) Recommended Local Run Pattern

```bash
# Fresh session A: preflight
codex
# paste Session A prompt

# Fresh session B: execution
codex
# paste Session B prompt

# Fresh session C: review
codex
# paste Session C prompt
```

If you keep the full work order pack in the repo, point Codex at it explicitly and keep only TASK-M7-01 foregrounded in the active prompt.
