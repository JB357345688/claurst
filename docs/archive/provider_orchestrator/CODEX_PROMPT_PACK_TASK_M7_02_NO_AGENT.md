# Codex Prompt Pack — TASK-M7-02 (No Agent / No Repo Skill Required)

This pack is designed for **fresh Codex sessions per ticket**.

Use it with:
- the full `MPWO_WORK_ORDER_PACK.md` attached or present in repo root
- **no assumption that any repo-local agent/skill is installed**
- only the **active ticket excerpt foregrounded** in the session prompt
- a mandatory **preflight pass** before edits
- a mandatory **validation pass** after edits
- human review before moving to the next dependency edge

This version is fully self-contained. The global rules are embedded directly in the prompts.

It also bakes in the main lesson from TASK-M7-01: if validation fails on a **narrow compile-shape mismatch**, prefer the **smallest ticket-compatible corrective patch** rather than widening the change set.

---

## 1) Recommended Session Sequence

### Session A — Preflight only
Goal: verify repo reality against the ticket before any edits.

### Session B — Execute only
Goal: implement exactly TASK-M7-02, no more.

### Session C — Review only
Goal: audit the diff against the ticket and global rules.

### Session D — Corrective patch only (use only if validation fails)
Goal: apply the smallest patch required to satisfy the ticket's validation command.

### Session E — Corrective patch review only (use only if Session D was needed)
Goal: confirm the corrective patch stayed narrow and ticket-compliant.

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
- If validation fails on a narrow compile-shape issue, prefer the smallest corrective patch that preserves the ticket's intended public shape and behavior.
```

---

## 3) Active Ticket Summary — TASK-M7-02

### Objective
Implement the pure provider-identity resolution function by replacing the `todo!("M7-02")` body of `resolve_provider_identity()` in `crates/query/src/provider_resolution.rs`.

### Source logic to match
Extract and reproduce the inline decision tree currently living in `crates/query/src/lib.rs` around the block described in the work order as `lib.rs:854-926`, with one intentional behavioral change:
- **remove the old filter that skipped explicit `"anthropic"`**
- explicit `"anthropic"` is now a real pin just like any other explicit provider

### Allowed file changes
- `crates/query/src/provider_resolution.rs` — function body implementation only

### Explicitly forbidden
- modifying `crates/query/src/lib.rs` yet — that belongs to M7-04
- changing `KNOWN_PROVIDERS` unless repo reality proves M7-01's verified union is stale
- adding D2 concepts
- adding side effects, network calls, auth-store lookups, or provider construction
- adding fallback behavior beyond the ticket's default-to-`"anthropic"`
- moving on to M7-03 or M7-04

### Preconditions
- TASK-M7-01 completed successfully
- `provider_resolution.rs` already exists
- `resolve_provider_identity(...)` currently exists as the stub created by M7-01

### Required precedence behavior
Implement the precedence described in the work order:
- **P1**: explicit provider + matching `"provider/model"` prefix → strip prefix, source `ExplicitProvider`
- **P2**: explicit provider + bare model → provider stays explicit, source `ExplicitProvider`
- **P3/P5**: explicit provider + different known-provider prefix → `ProviderModelConflict`
- **P4**: no explicit provider + known-provider prefix → source `ModelStringPrefix`
- **P7**: no explicit provider + unknown namespace prefix → fall through to registry/default
- **P8**: no explicit provider + bare model + model registry hit → source `ModelRegistry`
- **P9**: no explicit provider + bare model + no registry hit → default provider `"anthropic"`, source `Default`
- **P6 / anthropic change**: explicit `"anthropic"` + bare model must now honor the explicit provider pin

### Validation command
```bash
cd src-rust && cargo check -p claurst-query
```

### Verification items relevant to this ticket
- Re-read the actual inline decision tree in `crates/query/src/lib.rs` before editing, because M7-02 depends on matching repo reality rather than stale line numbers.
- Confirm `resolve_provider_identity(...)` still exists as a stub in `crates/query/src/provider_resolution.rs`.
- Confirm `ModelRegistry::find_provider_for_model()` still exists with the usable signature expected by the ticket.
- Confirm the verified `KNOWN_PROVIDERS` set from M7-01 is still current before relying on it.
- Confirm the old anthropic-skip logic still exists in the inline tree so the intentional behavioral change is precise.

---

## 4) Session A — Preflight Prompt

```text
You are working on exactly one ticket from MPWO_WORK_ORDER_PACK.md: TASK-M7-02.

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
- If validation fails on a narrow compile-shape issue, prefer the smallest corrective patch that preserves the ticket's intended public shape and behavior.

Do not edit anything yet.

Task:
Perform a preflight verification pass for TASK-M7-02 only.

What to verify:
1. `crates/query/src/provider_resolution.rs` exists and still contains the `resolve_provider_identity(...)` stub created by M7-01.
2. The inline decision tree described in the work order is still present in `crates/query/src/lib.rs` near the cited block, even if line numbers have drifted.
3. Identify the exact live branches that correspond to:
   - explicit provider + matching prefix
   - explicit provider + mismatched known-provider prefix
   - no explicit provider + known-provider prefix
   - no explicit provider + unknown namespace prefix
   - model registry lookup
   - final default provider path
4. Confirm whether the old explicit-provider path still contains the `anthropic` skip/filter behavior that M7-02 is supposed to remove.
5. Confirm `ModelRegistry::find_provider_for_model()` still exists and note its callable signature as actually found.
6. Confirm the verified `KNOWN_PROVIDERS` set from M7-01 is still current enough to reuse.
7. Confirm no extra file edits are needed beyond `crates/query/src/provider_resolution.rs` for the M7-02 implementation itself.

Hard rules:
- Do not edit files.
- Do not suggest broader refactors.
- Do not move to M7-03 or M7-04.
- If repo reality conflicts structurally with the ticket, stop.

Return exactly these sections:
- Preflight verdict
- Verified paths and symbols
- Live branch mapping
- Anthropic-pin confirmation
- Required adjustments before execution
- Blockers
```

---

## 5) Session B — Execute Prompt

```text
You are working on exactly one ticket from MPWO_WORK_ORDER_PACK.md: TASK-M7-02.

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
- If validation fails on a narrow compile-shape issue, prefer the smallest corrective patch that preserves the ticket's intended public shape and behavior.

Preconditions already expected to be true from preflight:
- `provider_resolution.rs` exists.
- `resolve_provider_identity(...)` exists as the M7-01 stub.
- The current inline logic in `lib.rs` is still the source of truth for the branch behavior.
- `ModelRegistry::find_provider_for_model()` is available.
- The verified `KNOWN_PROVIDERS` set from M7-01 remains valid unless preflight explicitly found drift.

Task:
Implement TASK-M7-02 only.

Required action:
1. Open `crates/query/src/provider_resolution.rs`.
2. Replace only the `todo!("M7-02")` body of `resolve_provider_identity()`.
3. Translate the inline decision tree from `crates/query/src/lib.rs` into this pure function.
4. Preserve the ticket's intended precedence:
   - explicit provider + matching prefix => strip prefix and use explicit provider
   - explicit provider + mismatched known-provider prefix => `ProviderModelConflict`
   - explicit provider + bare model => use explicit provider as-is
   - no explicit provider + known-provider prefix => split and use model-string prefix
   - no explicit provider + unknown namespace prefix => fall through
   - no explicit provider + bare model + registry hit => use registry provider
   - no explicit provider + bare model + no registry hit => default to `"anthropic"`
5. Apply the one intentional behavior change from the work order:
   - do NOT filter out explicit `"anthropic"`
   - explicit `"anthropic"` must now behave as a true pin
6. Keep the function pure:
   - no I/O
   - no network
   - no auth-store access
   - no provider construction
7. Do not touch `lib.rs` yet.
8. Do not change `KNOWN_PROVIDERS` unless preflight proved the M7-01 verified union is stale.
9. Do not change other functions, types, or module structure unless a narrow compile fix is strictly required.

Validation:
- Run exactly this command:
  `cd src-rust && cargo check -p claurst-query`
- Stop immediately on failure.
- Do not continue into M7-03 or M7-04.

Final report format:
- Ticket
- Files changed
- What was implemented
- Validation commands run
- Validation results
- Deviations from ticket, if any
- Blockers, if any
```

---

## 6) Session C — Review Prompt

```text
You are reviewing exactly one ticket from MPWO_WORK_ORDER_PACK.md: TASK-M7-02.

Apply the same global rules used for execution. Do not assume any repo-local agent or skill is installed.

Review the current diff only against TASK-M7-02.

Check only:
1. Scope compliance — was the implementation kept inside `crates/query/src/provider_resolution.rs`?
2. Was only the `resolve_provider_identity()` implementation changed, aside from any strictly necessary compile-shape fix?
3. Does the implemented precedence match the ticket:
   - explicit provider + matching prefix
   - explicit provider + mismatched known-provider prefix
   - explicit provider + bare model
   - no explicit provider + known-provider prefix
   - no explicit provider + unknown namespace prefix fallthrough
   - model registry lookup
   - default to `"anthropic"`
4. Was the old explicit-`anthropic` skip removed so anthropic is a true explicit-provider pin?
5. Were any forbidden behaviors introduced:
   - side effects
   - I/O
   - provider construction
   - D2 concepts
   - extra fallback logic
6. Was `lib.rs` left untouched for this ticket?
7. Was the validation command actually run, and did it pass?

Do not rewrite code unless needed to fix a ticket violation.

Return:
- pass/fail
- exact violations
- minimal corrective actions
```

---

## 7) Session D — Corrective Patch Prompt (Only If Validation Fails)

```text
You are working on a minimal corrective patch required to complete TASK-M7-02 after a failed validation run.

Known rule:
- This is not a new ticket.
- This is a narrow corrective patch only.
- Scope must stay inside the files already touched for TASK-M7-02 unless the compile failure proves otherwise.

Apply these constraints:
1. Do not widen scope.
2. Do not start M7-03 or M7-04.
3. Do not change public behavior beyond what is required to satisfy TASK-M7-02.
4. Prefer the smallest ticket-compatible patch.
5. If the failure is a narrow trait/derive/typing issue, preserve the intended ticket shape and behavior while fixing compileability.
6. Re-run the ticket validation command and stop immediately on failure.

Validation command:
`cd src-rust && cargo check -p claurst-query`

Final report:
- Files changed
- Exact corrective fix applied
- Validation result
- Any remaining blocker
```

---

## 8) Session E — Corrective Patch Review Prompt (Only If Session D Was Needed)

```text
Review the current diff only for the TASK-M7-02 corrective patch.

Check:
1. Was the fix limited strictly to the validation-blocking issue?
2. Did the patch preserve TASK-M7-02 behavior and precedence?
3. Was scope kept narrow?
4. Did `cargo check -p claurst-query` pass?
5. Is there any simpler ticket-compatible alternative that would have been better?

Return:
- pass/fail
- any scope violations
- any simpler alternative if the patch was overbuilt
```

---

## 9) One-Shot Two-Phase Variant (Use Only If You Want a Single Session)

This is less preferred than separate fresh sessions, but still usable.

```text
Work only on TASK-M7-02 from MPWO_WORK_ORDER_PACK.md.

Phase 1 — preflight only:
- verify the live inline branch logic in `crates/query/src/lib.rs`
- verify `resolve_provider_identity()` stub exists in `crates/query/src/provider_resolution.rs`
- verify `ModelRegistry::find_provider_for_model()` signature
- verify whether the old anthropic skip/filter is still present
- if structural drift exists, stop and report without editing

Phase 2 — execute only if preflight passes:
- replace only the `todo!("M7-02")` body of `resolve_provider_identity()`
- implement the exact precedence required by the ticket
- remove the old anthropic skip behavior by honoring explicit `"anthropic"`
- keep the function pure
- do not modify `lib.rs`
- run `cd src-rust && cargo check -p claurst-query`
- stop immediately on failure

Global rules:
- no scope expansion
- no adjacent cleanup
- no D2 concepts
- no extra fallbacks
- no side effects
- no continuation into M7-03 or M7-04

Final output:
- preflight verdict
- files changed
- validation result
- blockers or deviations
```

---

## 10) Human Gate Before M7-03

Do **not** move to TASK-M7-03 until all of the following are true:
- TASK-M7-02 preflight passed
- TASK-M7-02 implementation completed
- `cargo check -p claurst-query` passed
- review passed
- any corrective patch, if needed, also passed review
- human acceptance given
