# Codex Prompt Pack — TASK-M7-03 (No Agent / No Repo Skill Required)

This pack is designed for **fresh Codex sessions per ticket**.

Use it with:
- the full `MPWO_WORK_ORDER_PACK.md` attached or present in repo root
- **no assumption that any repo-local agent/skill is installed**
- only the **active ticket excerpt foregrounded** in the session prompt
- a mandatory **preflight pass** before edits
- a mandatory **validation pass** after edits
- human review before moving to the next dependency edge

This version is fully self-contained. The global rules are embedded directly in the prompts.

It also carries forward the main lesson from TASK-M7-01: if validation fails on a **narrow compile-shape mismatch**, prefer the **smallest ticket-compatible corrective patch** rather than widening the change set.

---

## 1) Recommended Session Sequence

### Session A — Preflight only
Goal: verify repo reality against the ticket before any edits.

### Session B — Execute only
Goal: implement exactly TASK-M7-03, no more.

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

## 3) Active Ticket Summary — TASK-M7-03

### Objective
Implement the side-effectful provider materialization function by replacing the `todo!("M7-03")` body of `materialize_provider()` in `crates/query/src/provider_resolution.rs`.

### Source logic to match
Extract and reproduce the inline construction and runtime selection logic currently living in `crates/query/src/lib.rs` around the block described in the work order as `lib.rs:937-977`.

This ticket is intentionally **side-effectful**:
- it may read the auth store via `runtime_provider_for()`
- it may construct OpenAI-compatible providers for exact local-provider override cases

### Allowed file changes
- `crates/query/src/provider_resolution.rs` — function body implementation and imports only

### Explicitly forbidden
- modifying `crates/query/src/lib.rs` yet — that belongs to M7-04
- changing `runtime_provider_for()` in `crates/api/src/registry.rs`
- adding health checks, capability checks, or any D2 logic
- adding fallback behavior beyond the ticket's explicit `NoCredentials(...)` error path
- adding new provider aliases beyond the exact existing match arms in the inline source block
- moving on to M7-04

### Preconditions
- TASK-M7-01 completed successfully
- `provider_resolution.rs` already exists
- `materialize_provider(...)` currently exists as the stub created by M7-01

Ticket note:
- The work order only requires M7-01 as a formal prerequisite.
- If you are executing the milestone strictly in sequence, it is still reasonable to wait until M7-02 has cleared its own review gate before running M7-03.

### Required implementation behavior
Implement the sequence described in the work order:
- auth-store refresh via `claurst_api::registry::runtime_provider_for(&identity.provider_id)`
- registry lookup only when the runtime/auth-store lookup did not produce a provider
- `api_base` override handling using the exact local/OpenAI-compatible provider aliases already present in the inline source block
- final provider selection as `runtime_provider.or(registry_provider)`
- return `ExecutionTarget` when a provider exists
- otherwise return `ProviderResolutionError::NoCredentials(identity.provider_id.clone())`

### Validation command
```bash
cd src-rust && cargo check -p claurst-query
```

### Verification items relevant to this ticket
- Re-read the actual inline materialization block in `crates/query/src/lib.rs` before editing, because M7-03 depends on matching repo reality rather than stale line numbers.
- Confirm `materialize_provider(...)` still exists as a stub in `crates/query/src/provider_resolution.rs`.
- Confirm `claurst_api::registry::runtime_provider_for()` still exists and note its callable signature as actually found.
- Confirm `ProviderRegistry::get()` still yields the shape expected by the ticket, including whether `.cloned()` remains appropriate.
- Confirm `claurst_core::config::ProviderConfig` still exposes the `api_base` path the inline code uses.
- Confirm `claurst_api::providers::openai_compat_providers` still exists, and verify the exact builder names and `.with_base_url()` pattern used in the inline source block.
- Confirm the exact local-provider alias match arms still present in the inline block before copying them.
- Confirm no extra file edits are needed beyond `crates/query/src/provider_resolution.rs` for the M7-03 implementation itself.

---

## 4) Session A — Preflight Prompt

```text
You are working on exactly one ticket from MPWO_WORK_ORDER_PACK.md: TASK-M7-03.

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
Perform a preflight verification pass for TASK-M7-03 only.

What to verify:
1. `crates/query/src/provider_resolution.rs` exists and still contains the `materialize_provider(...)` stub created by M7-01.
2. The inline materialization block described in the work order is still present in `crates/query/src/lib.rs` near the cited block, even if line numbers have drifted.
3. Identify the exact live steps that correspond to:
   - auth-store refresh via `runtime_provider_for()`
   - registry lookup fallback when runtime provider is absent
   - `api_base` override detection from `provider_configs`
   - exact local-provider alias arms used for override reconstruction
   - final `runtime_provider.or(registry_provider)` selection
   - `NoCredentials(...)` error path
4. Confirm `claurst_api::registry::runtime_provider_for()` still exists and note its callable signature as actually found.
5. Confirm `ProviderRegistry::get()` still supports the lookup pattern expected by the ticket.
6. Confirm `ProviderConfig` still exposes the field path used for `api_base` override handling.
7. Confirm `claurst_api::providers::openai_compat_providers` still exists and that the exact builder pattern used in the inline block is still available.
8. Record the exact provider alias match arms present in the inline block for the override rebuild path.
9. Confirm no extra file edits are needed beyond `crates/query/src/provider_resolution.rs` for the M7-03 implementation itself.

Hard rules:
- Do not edit files.
- Do not suggest broader refactors.
- Do not move to M7-04.
- If repo reality conflicts structurally with the ticket, stop.

Return exactly these sections:
- Preflight verdict
- Verified paths and symbols
- Live step mapping
- Exact override alias set
- Required adjustments before execution
- Blockers
```

---

## 5) Session B — Execute Prompt

```text
You are working on exactly one ticket from MPWO_WORK_ORDER_PACK.md: TASK-M7-03.

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
- `materialize_provider(...)` exists as the M7-01 stub.
- The current inline logic in `lib.rs` is still the source of truth for the materialization behavior.
- `claurst_api::registry::runtime_provider_for()` is available.
- `ProviderRegistry::get()` still supports the expected lookup path.
- The exact override alias match arms have been verified from the inline source block.

Task:
Implement TASK-M7-03 only.

Required action:
1. Open `crates/query/src/provider_resolution.rs`.
2. Add only the imports required by the ticket:
   - `use claurst_core::config::ProviderConfig;`
   - `use std::collections::HashMap;`
   plus any other import that is strictly required to mirror the already-existing inline source behavior as found during preflight.
3. Replace only the `todo!("M7-03")` body of `materialize_provider()`.
4. Implement the exact ticket-required sequence:
   - call `claurst_api::registry::runtime_provider_for(&identity.provider_id)` first
   - if that returns `Some`, auth-store/runtime provider takes priority
   - otherwise construct the registry lookup path and use `registry.get(&pid).cloned()` or the equivalent verified by preflight
   - inspect `provider_configs.get(&identity.provider_id)` for `api_base` override handling
   - if override is present and the provider id matches one of the exact verified local-provider aliases from the inline source block, rebuild using the same `openai_compat_providers` builder arms and `.with_base_url()` pattern as the inline logic
   - perform final selection as `runtime_provider.or(registry_provider)`
   - return `Ok(ExecutionTarget { ... })` when a provider exists
   - otherwise return `Err(ProviderResolutionError::NoCredentials(identity.provider_id.clone()))`
5. Keep the implementation limited to the inline behavior already present in `lib.rs`.
6. Do not add new provider aliases beyond the exact verified match arms.
7. Do not add D2 concepts, health checks, capability checks, or extra fallback behavior.
8. Do not modify `lib.rs` yet.
9. Do not touch `registry.rs`.
10. Do not change other functions, types, or module structure unless a narrow compile fix is strictly required.

Validation:
- Run exactly this command:
  `cd src-rust && cargo check -p claurst-query`
- Stop immediately on failure.
- Do not continue into M7-04.

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
You are reviewing exactly one ticket from MPWO_WORK_ORDER_PACK.md: TASK-M7-03.

Apply the same global rules used for execution. Do not assume any repo-local agent or skill is installed.

Review the current diff only against TASK-M7-03.

Check only:
1. Scope compliance — was the implementation kept inside `crates/query/src/provider_resolution.rs`?
2. Were changes limited to the `materialize_provider()` implementation and the minimal imports needed for this ticket, aside from any strictly necessary compile-shape fix?
3. Does the implementation follow the ticket-required sequence:
   - auth-store refresh via `runtime_provider_for()`
   - registry lookup only when runtime provider is absent
   - `api_base` override detection
   - exact verified local-provider alias match arms only
   - final `runtime_provider.or(registry_provider)` selection
   - `NoCredentials(...)` error when no provider materializes
4. Were any forbidden behaviors introduced:
   - changes to `lib.rs`
   - changes to `registry.rs`
   - new provider aliases not present in the inline source block
   - D2 concepts
   - health or capability logic
   - extra fallback behavior
5. Was the inline source behavior mirrored rather than reimagined into a new abstraction?
6. Was the validation command actually run, and did it pass?

Do not rewrite code unless needed to fix a ticket violation.

Return:
- pass/fail
- exact violations
- minimal corrective actions
```

---

## 7) Session D — Corrective Patch Prompt (Only If Validation Fails)

```text
You are working on a minimal corrective patch required to complete TASK-M7-03 after a failed validation run.

Known rule:
- This is not a new ticket.
- This is a narrow corrective patch only.
- Scope must stay inside the files already touched for TASK-M7-03 unless the compile failure proves otherwise.

Apply these constraints:
1. Do not widen scope.
2. Do not start M7-04.
3. Do not change public behavior beyond what is required to satisfy TASK-M7-03.
4. Prefer the smallest ticket-compatible patch.
5. If the failure is a narrow trait/derive/typing/import issue, preserve the intended ticket shape and behavior while fixing compileability.
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
Review the current diff only for the TASK-M7-03 corrective patch.

Check:
1. Was the fix limited strictly to the validation-blocking issue?
2. Did the patch preserve TASK-M7-03 behavior and ordering?
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
Work only on TASK-M7-03 from MPWO_WORK_ORDER_PACK.md.

Phase 1 — preflight only:
- verify the live inline materialization logic in `crates/query/src/lib.rs`
- verify `materialize_provider()` stub exists in `crates/query/src/provider_resolution.rs`
- verify `runtime_provider_for()` signature
- verify `ProviderRegistry::get()` lookup shape
- verify `ProviderConfig` api_base path
- verify `openai_compat_providers` builder names and exact override alias match arms
- if structural drift exists, stop and report without editing

Phase 2 — execute only if preflight passes:
- replace only the `todo!("M7-03")` body of `materialize_provider()`
- add only the ticket-required imports and any other strictly necessary imports already implied by the verified inline source block
- implement auth-store refresh, registry lookup, exact override reconstruction, final provider selection, and `NoCredentials(...)` error
- do not modify `lib.rs`
- do not modify `registry.rs`
- do not add new aliases or D2 concepts
- run `cd src-rust && cargo check -p claurst-query`
- stop immediately on failure

Global rules:
- no scope expansion
- no adjacent cleanup
- no D2 concepts
- no extra fallbacks
- no continuation into M7-04

Final output:
- preflight verdict
- files changed
- validation result
- blockers or deviations
```

---

## 10) Human Gate Before M7-04

Do **not** move to TASK-M7-04 until all of the following are true:
- TASK-M7-03 preflight passed
- TASK-M7-03 implementation completed
- `cargo check -p claurst-query` passed
- review passed
- any corrective patch, if needed, also passed review
- if you are running M7 sequentially, TASK-M7-02 has also cleared its own acceptance gate
- human acceptance given
