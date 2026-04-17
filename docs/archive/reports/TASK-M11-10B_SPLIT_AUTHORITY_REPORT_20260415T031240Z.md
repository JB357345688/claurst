# TASK-M11-10B Split Authority Report

## ticket id

`TASK-M11-10B`

## timestamp UTC

`2026-04-15T03:12:40Z`

## authority inputs reviewed

- `AGENTS.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md`
- `docs/Current/TASK-M11-10_SPLIT_PLAN_AND_TEMPORARY_AUTHORITY_AMENDMENT.md`
- `docs/archive/reports/M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md`
- `docs/archive/reports/MPWO_REVISION_REPORT_20260414T003218Z.md`
- `docs/archive/reports/TASK-M11-08R_CLOSEOUT_REPORT_20260415T004401Z.md`
- `docs/archive/reports/TASK-M11-08B_CLOSEOUT_REPORT_20260415T011952Z.md`
- `docs/archive/reports/TASK-M11-09_CLOSEOUT_REPORT_20260415T014220Z.md`
- `docs/archive/reports/TASK-M11-10_PREFLIGHT_REPORT_20260415T021006Z.md`
- `docs/archive/reports/TASK-M11-10A_CLOSEOUT_REPORT_20260415T024141Z.md`
- `docs/archive/reports/TASK-M11-10B_AUTHORITY_REPORT_20260415T030449Z.md`

## current baseline checked

- Branch observed: `feature/provider-resolution-seam`
- HEAD observed: `ea046c52da82dfd9778f4065bd36b36e28d73c8a`
- Expected accepted branch / HEAD from current authority context: matched
- Accepted corrected path treated as fixed baseline:
  - `08R` root `SessionBudget` wiring
  - `08B` child/team inherited parent session-budget propagation via the accepted query-owned session-id keyed seam
  - `09` child `max_tokens` override wiring
  - `10A` child `allow_fallback` wiring
- Governing conclusion carried forward from `TASK-M11-10B_AUTHORITY_REPORT_20260415T030449Z.md`:
  - unsplit `10B` is not sufficient as one ticket on the accepted corrected path
  - child `budget_usd` means an additional child-local cumulative USD cap for the child subtree while preserving parent shared-session accounting

## why unsplit `TASK-M11-10B` cannot proceed

Unsplit `10B` cannot proceed as one ticket because the accepted corrected path contains two different problems that do not share the same narrow execution boundary:

1. The first problem is query-owned seam definition.
   - The current runtime supports only one live `SessionBudget` per child loop path.
   - The accepted `08B` seam inherits budget by shared `session_id`.
   - The current registry behavior replaces the stored budget for that session key when a new one is registered.
   - Therefore child-specific budget semantics are not a field-carriage problem first; they are a query-owned seam problem first.

2. The second problem is child/team schema and runtime carriage.
   - After the seam is defined, child/team entrypoints still need schema fields and runtime wiring.
   - Those entrypoints span both `agent_tool.rs` and `team_tool.rs`.

3. Doing both in one ticket would blur the authority boundary.
   - It would combine seam-definition work with child/team carriage work.
   - It would make preflight and review less scope-clean.
   - It would encourage drift back toward the already-rejected assumption that child `budget_usd` is a narrow wiring pass.

4. Child `budget_usd` must remain distinct from all of the following:
   - `max_budget_usd`
   - shared inherited parent `SessionBudget`
   - `max_tokens`

Conclusion:

- `TASK-M11-10B1` must own the query-owned child budget seam problem first.
- `TASK-M11-10B2` must not begin until `10B1` is accepted.

## temporary authority for `TASK-M11-10B1`

### ticket name

`TASK-M11-10B1`

### purpose

Resolve the query-owned child budget seam on the accepted corrected path before any child/team `budget_usd` schema carriage begins.

### owned problem

`10B1` owns the first unresolved problem only:

- how child `budget_usd` is represented in the query-owned runtime seam
- how child-local budget enforcement is added without breaking accepted `08B` parent shared-session accounting
- how nested child inheritance behaves once a child-local budget exists

### intended meaning of child `budget_usd`

For this corrected path, child `budget_usd` means:

- an additional child-local cumulative USD cap for the spawned child subtree
- while preserving accounting against the inherited parent shared `SessionBudget`

It does **not** mean:

- `max_budget_usd`
- replacing the inherited parent shared `SessionBudget`
- a synonym for `max_tokens`

### required seam outcome

`10B1` must define and authorize the minimal query-owned seam that makes the above meaning implementable.

That seam must ensure:

- child-local cumulative USD cap exists as a distinct concept
- parent shared-session accounting from `08B` remains preserved
- descendant child runs inherit the correct combined semantics from the accepted seam
- no ticket-local solution depends on adding concrete query-owned budget/cache fields to `ToolContext`

### likely owned file scope

Default expected owned file scope:

- `src-rust/crates/query/src/session_budget.rs`
- `src-rust/crates/query/src/lib.rs`

Possible narrow query-side fallout only if preflight proves it unavoidable:

- `src-rust/crates/query/src/agent_tool.rs`

### whether it is query-owned only

`yes`

`10B1` is query-owned only. It is the next executable budget ticket. It should not require `team_tool.rs` carriage work if the seam boundary is kept clean.

### explicitly excluded scope

`10B1` must not include:

- child/team schema fields on `AgentInput`, `AgentSpec`, or `AgentRunParams`
- `team_tool.rs` carriage work
- reinterpretation of child `budget_usd` as `max_budget_usd`
- any `ToolContext.session_budget`
- any `ToolContext.health_cache`
- `HealthCache` plumbing or fallback-policy changes
- reopening `10A`
- reopening `09`
- TeamCreate outer-cancellation redesign unless later evidence proves it unavoidable
- QueryEvent expansion
- downstream test-suite expansion beyond seam-local validation

## temporary authority for `TASK-M11-10B2`

### ticket name

`TASK-M11-10B2`

### purpose

Add child/team schema and runtime carriage for `budget_usd` only after `10B1` has been accepted.

### dependency on accepted `10B1`

`10B2` is blocked on accepted `10B1`.

It must not begin until:

- `10B1` preflight is complete
- `10B1` execution is complete
- `10B1` verification passes
- `10B1` closeout is accepted

### required schema/runtime fields

After `10B1` is accepted, `10B2` owns the narrow carriage work for:

- `AgentInput.budget_usd`
- `AgentSpec.budget_usd`
- `AgentRunParams.budget_usd`

And the runtime wiring for:

- foreground AgentTool child runs
- background AgentTool child runs
- cc-query-backed team-runner child loops

That carriage must target the seam accepted by `10B1`, not invent a new interpretation.

### expected owned file scope

Default expected owned file scope:

- `src-rust/crates/query/src/agent_tool.rs`
- `src-rust/crates/tools/src/team_tool.rs`

Possible narrow query-side fallout only if required by the already-accepted `10B1` seam:

- `src-rust/crates/query/src/lib.rs`
- `src-rust/crates/query/src/session_budget.rs`

### explicitly excluded scope

`10B2` must not include:

- redesign of the `10B1` seam itself
- reinterpretation of child `budget_usd` as `max_budget_usd`
- any `ToolContext.session_budget`
- any `ToolContext.health_cache`
- `HealthCache` budget semantics
- reopening `10A`
- reopening `09`
- TeamCreate outer-cancellation redesign unless later evidence proves it unavoidable
- QueryEvent expansion beyond any narrow compile fallout authorized by later ticket authority

## explicit exclusions and guardrails

These guardrails apply to both `10B1` and `10B2`:

1. Child `budget_usd` remains distinct from:
   - `max_budget_usd`
   - shared inherited parent `SessionBudget`
   - `max_tokens`

2. No ticket in this split may add:
   - `ToolContext.session_budget`
   - `ToolContext.health_cache`

3. No ticket in this split may:
   - reopen `10A`
   - reopen `09`
   - silently redesign TeamCreate outer cancellation unless later evidence proves it unavoidable
   - pull `HealthCache` into budget semantics

4. `10B1` is the next executable budget ticket, not `10B2`.

5. `10B2` is blocked until `10B1` is accepted.

6. Neither ticket may drift back toward the rejected pre-correction assumption that child `budget_usd` is a narrow field-only wiring pass on the accepted `08B` seam.

## sequencing impact on `TASK-M11-11` and later

The required sequencing rule is:

- `10B1` preflight
- `10B1` execution
- `10B1` verification
- `10B1` closeout
- then `10B2` preflight
- `10B2` execution
- `10B2` verification
- `10B2` closeout

Only after both tickets close may downstream tickets proceed assuming child `budget_usd` is settled.

Therefore:

- `TASK-M11-10B1` is the next executable budget ticket
- `TASK-M11-10B2` is blocked pending accepted `10B1`
- `TASK-M11-11` and later tickets must not assume child `budget_usd` semantics are settled until both `10B1` and `10B2` are closed

## exact proposed wording snippet for GPT/WebUI to adopt

> `TASK-M11-10B` is split again. Do not execute it as one ticket. TASK-M11-10B1 is the next executable budget ticket and owns the query-owned child budget seam problem first. On the accepted corrected path, child budget_usd means an additional child-local cumulative USD cap for the spawned child subtree while preserving accounting against the inherited parent shared SessionBudget from TASK-M11-08B. Child budget_usd remains distinct from max_budget_usd, shared inherited parent SessionBudget, and max_tokens. TASK-M11-10B1 is query-owned only and should stay focused on the minimal seam work needed in query-owned budget runtime surfaces.`
>
> `TASK-M11-10B2 is blocked until TASK-M11-10B1 is accepted. Only after TASK-M11-10B1 closeout may TASK-M11-10B2 add and wire AgentInput.budget_usd, AgentSpec.budget_usd, and AgentRunParams.budget_usd through child/team runtime entrypoints. Neither ticket may add ToolContext.session_budget or ToolContext.health_cache, reopen TASK-M11-10A or TASK-M11-09, pull HealthCache into budget semantics, or redesign TeamCreate outer cancellation unless later evidence proves it unavoidable. Downstream tickets, including TASK-M11-11, must not assume child budget_usd is settled until both TASK-M11-10B1 and TASK-M11-10B2 are closed.`

## whether the split resolves current authority ambiguity

`yes`

## notes / risks

- This split resolves the current authority ambiguity by separating query-owned seam definition from child/team carriage.
- The main residual risk is future prompt drift that tries to collapse `10B1` and `10B2` back together for convenience.
- A second residual risk is semantic drift that tries to treat child `budget_usd` as either:
  - `max_budget_usd`, or
  - a replacement for the inherited parent shared `SessionBudget`
  Both remain invalid on the accepted corrected path.
- If later preflight on `10B1` proves that a tiny query-side construction adjustment in `agent_tool.rs` is unavoidable, that does not change the split rule: the ticket still remains query-owned first, and `10B2` remains blocked on accepted `10B1`.
