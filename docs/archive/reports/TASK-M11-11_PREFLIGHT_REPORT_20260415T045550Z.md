# TASK-M11-11 Preflight Report

## 1. Ticket ID

`TASK-M11-11`

This preflight is for revised `TASK-M11-11 = QueryEvent expansion / observability`, not the stale MPWO numbering where QueryEvent work appeared as `M11-10`.

## 2. Verdict

`PASS-WITH-NOTES`

Basis:
- live `HEAD` matches the accepted `10B2` closeout commit exactly
- accepted `08R`, `08B`, `09`, `10A`, `10B1`, and `10B2` are all present in current repo reality and on the current ancestry path
- the landed layered `SessionBudget` model from `10B1` / `10B2` is the runtime baseline for event semantics
- the ticket still reads as a single narrow observability ticket on the corrected accepted path
- notes remain around event transport from child runs, payload shape for a cross-path worker identifier, and minimal non-query compile fallout

## 3. Timestamp UTC

`2026-04-15T04:55:50Z`

## 4. Branch / HEAD Observed

- Branch: `feature/provider-resolution-seam`
- HEAD: `bfabfd5934b0afe801f2e7de9f15a7f6258c563b`
- Expected accepted latest HEAD: `bfabfd5934b0afe801f2e7de9f15a7f6258c563b`
- Match: `yes`
- Working tree state: branch matches and `HEAD` matches, but the repo is noisy with unrelated tracked and untracked changes, including `.gitignore`, `docs/Current/`, many `docs/archive/reports/*.md`, `.codex`, and `src-rust/target/`

## 5. Authority Reviewed

- `AGENTS.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md`
- `docs/Current/TASK-M11-10_SPLIT_PLAN_AND_TEMPORARY_AUTHORITY_AMENDMENT.md`
- `docs/archive/reports/M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md`
- `docs/archive/reports/MPWO_REVISION_REPORT_20260414T003218Z.md`
- `docs/archive/reports/TASK-M11-08R_CLOSEOUT_REPORT_20260415T004401Z.md`
- `docs/archive/reports/TASK-M11-08B_CLOSEOUT_REPORT_20260415T011952Z.md`
- `docs/archive/reports/TASK-M11-09_CLOSEOUT_REPORT_20260415T014220Z.md`
- `docs/archive/reports/TASK-M11-10A_CLOSEOUT_REPORT_20260415T024141Z.md`
- `docs/archive/reports/TASK-M11-10B_SPLIT_AUTHORITY_REPORT_20260415T031240Z.md`
- `docs/archive/reports/TASK-M11-10B1_CLOSEOUT_REPORT_20260415T034453Z.md`
- `docs/archive/reports/TASK-M11-10B2_CLOSEOUT_REPORT_20260415T041344Z.md`
- `docs/archive/reports/M11_CONVERGENCE_REVIEW_20260415T042825Z.md`

## 6. Accepted-Baseline Comparison

- `HEAD` exactly equals the accepted `10B2` closeout SHA
- ancestry checks passed for:
- `08R` `25518cac29d34353cb58c8811da1040a3da69247`
- `08B` `b4ad28ee9eb1e434f935fd2de70c4b402a5c74da`
- `09` `4ef9547dab51959f7b39c473f929b81f05ee1134`
- `10A` `ea046c52da82dfd9778f4065bd36b36e28d73c8a`
- `10B1` `3812df04ec87ce0e96f851da2d18ab38b24f0b99`
- `10B2` `bfabfd5934b0afe801f2e7de9f15a7f6258c563b`
- live code confirms the corrected split runtime path:
- inherited shared session-budget lookup via [`agent_tool.rs`](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:134)
- child-local layered budget construction via [`agent_tool.rs`](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:148)
- child/team `max_tokens` carriage via [`agent_tool.rs`](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:421) and [`team_tool.rs`](/home/jordi/claurst/src-rust/crates/tools/src/team_tool.rs:43)
- child `allow_fallback` carriage and resolution via [`agent_tool.rs`](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:332) and [`agent_tool.rs`](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:652)
- child/team `budget_usd` carriage via [`agent_tool.rs`](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:416), [`agent_tool.rs`](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:678), and [`team_tool.rs`](/home/jordi/claurst/src-rust/crates/tools/src/team_tool.rs:45)
- shared-root plus task-local stack registration via [`session_budget.rs`](/home/jordi/claurst/src-rust/crates/query/src/session_budget.rs:98)

## 7. Verified Target Files / Symbols / Commands

Verified files:
- [`src-rust/crates/query/src/lib.rs`](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:399)
- [`src-rust/crates/query/src/agent_tool.rs`](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:148)
- [`src-rust/crates/query/src/session_budget.rs`](/home/jordi/claurst/src-rust/crates/query/src/session_budget.rs:19)
- [`src-rust/crates/query/src/provider_resolution.rs`](/home/jordi/claurst/src-rust/crates/query/src/provider_resolution.rs:302)
- [`src-rust/crates/query/src/health_cache.rs`](/home/jordi/claurst/src-rust/crates/query/src/health_cache.rs)
- [`src-rust/crates/tools/src/team_tool.rs`](/home/jordi/claurst/src-rust/crates/tools/src/team_tool.rs:39)
- [`src-rust/crates/tools/src/lib.rs`](/home/jordi/claurst/src-rust/crates/tools/src/lib.rs:100)
- [`src-rust/crates/cli/src/main.rs`](/home/jordi/claurst/src-rust/crates/cli/src/main.rs:1159)
- [`src-rust/crates/tui/src/app.rs`](/home/jordi/claurst/src-rust/crates/tui/src/app.rs:5442)

Verified symbols / seams:
- `QueryEvent`
- `run_query_loop`
- `resolve_provider_with_fallback(...)`
- `resolve_provider_identity(...)`
- `SessionBudget::new(...)`
- `SessionBudget::child_scope(...)`
- `SessionBudget::check_and_cancel(...)`
- `session_budget_for_session(...)`
- `with_registered_session_budget(...)`
- `AgentInput.allow_fallback`
- `AgentInput.budget_usd`
- `AgentRunParams.max_tokens_override`
- `AgentRunParams.allow_fallback`
- `AgentRunParams.budget_usd`
- `ToolResult.metadata`

Commands run:
- `git rev-parse --abbrev-ref HEAD`
- `git rev-parse HEAD`
- `git status --short --branch`
- `git merge-base --is-ancestor <accepted-sha> HEAD` for all accepted split-baseline SHAs
- targeted `rg` scans across query, tools, cli, tui, and authority docs
- `cd src-rust && cargo check --workspace`

## 8. Exact Scope Confirmation in Current Repo Reality

Scope confirmation: `yes`, with notes.

`TASK-M11-11` can still stay limited to:
- adding the three D2 `QueryEvent` variants
- emitting them at the correct runtime points against the landed split path
- keeping all landed budget, fallback, hosted Ollama, and provider-resolution semantics unchanged

Important scope facts from live code:
- current `QueryEvent` still only contains `Stream`, `ToolStart`, `ToolEnd`, `TurnComplete`, `Status`, `Error`, and `TokenWarning` in [`lib.rs`](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:399)
- same-domain fallback-aware provider resolution is already the landed path in [`provider_resolution.rs`](/home/jordi/claurst/src-rust/crates/query/src/provider_resolution.rs:302)
- child-local layered `budget_usd` semantics are already the landed path in [`agent_tool.rs`](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:148) plus [`session_budget.rs`](/home/jordi/claurst/src-rust/crates/query/src/session_budget.rs:36)
- the distinction between `max_budget_usd` and shared `SessionBudget` remains explicit in [`lib.rs`](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:115)

The ticket does not need:
- UI rendering work
- provider fallback redesign
- hosted Ollama behavior change
- `ToolContext.session_budget`
- `ToolContext.health_cache`
- reopening `10B1` or `10B2`

## 9. Event Seam Findings

Current enum shape:
- `QueryEvent` is currently defined only in [`lib.rs`](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:399)
- no `WorkerProviderResolved`
- no `WorkerBudgetExceeded`
- no `SessionBudgetExceeded`

Current emission sites:
- all current `QueryEvent` sends are in `query::lib.rs`
- there is no direct tool-owned `QueryEvent` emitter path today

Current child/team provider-resolution sites:
- foreground/background `AgentTool` path resolves provider at [`agent_tool.rs`](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:334)
- team-runner injected path resolves provider at [`agent_tool.rs`](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:652)
- same-domain fallback gating remains at [`provider_resolution.rs`](/home/jordi/claurst/src-rust/crates/query/src/provider_resolution.rs:318) through [`provider_resolution.rs`](/home/jordi/claurst/src-rust/crates/query/src/provider_resolution.rs:345)

Current root and child/team budget paths:
- root/shared session-budget recording and cancellation checks happen in the registry-backed query path at [`lib.rs`](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:1169) and in the legacy path at [`lib.rs`](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:1439)
- per-loop `max_budget_usd` remains a separate guard returning `QueryOutcome::BudgetExceeded` at [`lib.rs`](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:1446)
- child/team layered budget objects are created from inherited parent budget plus local `budget_usd` at [`agent_tool.rs`](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:148)
- shared-root registry preservation plus task-local child-scope lookup are implemented at [`session_budget.rs`](/home/jordi/claurst/src-rust/crates/query/src/session_budget.rs:98)

Clean emission assessment:
- `WorkerProviderResolved`: can be emitted narrowly from the existing child/team resolution points without redesigning provider-resolution code; the needed comparison data already exists through `resolve_provider_identity(...)` and the resolved `ExecutionTarget`
- `WorkerBudgetExceeded`: semantically should map to child-local `budget_usd` exceed on the landed `10B1/10B2` layered budget model, not to `max_budget_usd`
- `SessionBudgetExceeded`: semantically should map to shared-session `SessionBudget` exceed on the landed `08R/08B` model, not to child-local `budget_usd` and not to `max_budget_usd`

Important live seam note:
- child loops are currently run with `event_tx = None` in foreground/background/team paths at [`agent_tool.rs`](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:479), [`agent_tool.rs`](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:525), and [`agent_tool.rs`](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:710)
- because of that, child-owned observability cannot reach the parent TUI by simply adding sends inside nested `run_query_loop(...)`
- a narrow bridge from tool/runtime results back into parent `query::lib.rs` is required if these events are meant to surface on the parent event channel

## 10. Anticipated Implementation Shape

Narrowest implementation shape that matches live repo reality:
- widen `QueryEvent` in [`lib.rs`](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:399)
- keep provider-resolution semantics unchanged
- keep layered `SessionBudget` semantics unchanged
- use existing parent tool/result boundary to surface child-runtime observability back into `query::lib.rs`

Most likely narrow seam:
- `AgentTool` and the injected team runner attach structured observability payloads to `ToolResult.metadata` or equivalent narrow result-carried data using the already-existing metadata field in [`tools/src/lib.rs`](/home/jordi/claurst/src-rust/crates/tools/src/lib.rs:101)
- `query::lib.rs` emits the new `QueryEvent` variants after tool execution using that metadata
- `query::lib.rs` emits root/shared `SessionBudgetExceeded` directly at the two post-cost-accounting `check_and_cancel()` sites

Field-shape findings:
- a cross-path stable `agent_id` is not uniformly present today
- foreground/background `AgentTool` has a generated UUID, but the team-runner path currently has `description` / `team/name`, not an equivalent dedicated `agent_id`
- a narrow event payload will either need:
- a worker identifier field based on already-live `description` / team agent name
- or a minimal new worker identifier carriage across both child/team entrypoints
- event payloads that include exact budget `limit_usd` and precise spent values will likely need narrow `SessionBudget` accessors because `budget_usd` and `spent` are private in [`session_budget.rs`](/home/jordi/claurst/src-rust/crates/query/src/session_budget.rs:19)

Budget detection findings:
- `WorkerBudgetExceeded` can be detected without reopening the landed budget seam by observing the current child scope versus its shared root budget after cancellation
- `SessionBudgetExceeded` can be detected without reopening the landed budget seam by observing the shared root `SessionBudget` at the existing `record_cost(...)` plus `check_and_cancel()` sites
- both remain observability-only if implemented as read-only detection plus event emission

Background-path note:
- provider resolution for background agents happens before the background task is launched, so `WorkerProviderResolved` is straightforward there
- budget exceed events for already-detached background runs are not directly visible through the current parent event channel because nested loops do not forward events
- this is still addressable through the same narrow tool/result-carried observability seam, but it should be treated as a design note during implementation

## 11. Anticipated Compile-Fallout Scope

Compile fallout is not fully confined to query-owned files.

Likely ticket-owned code changes:
- [`src-rust/crates/query/src/lib.rs`](/home/jordi/claurst/src-rust/crates/query/src/lib.rs:399)
- [`src-rust/crates/query/src/agent_tool.rs`](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:334)

Likely narrow compile-fallout outside query:
- [`src-rust/crates/tui/src/app.rs`](/home/jordi/claurst/src-rust/crates/tui/src/app.rs:5442) exhaustively matches `QueryEvent`, so widening the enum will likely require at least no-op handling there

Likely no required compile-fallout:
- CLI event consumers in [`main.rs`](/home/jordi/claurst/src-rust/crates/cli/src/main.rs:1159) and [`main.rs`](/home/jordi/claurst/src-rust/crates/cli/src/main.rs:2165) already use wildcard branches for unhandled variants
- bridge forwarding in CLI also has `_ => None`

Assessment:
- the ticket remains single-step and narrow
- but it is unlikely to be strictly query-only at compile time because of the TUI exhaustive match

## 12. Validation Command Run and Result

Command:

```bash
cd src-rust && cargo check --workspace
```

Result:

```text
Compiling claurst v0.0.8 (/home/jordi/claurst/src-rust/crates/cli)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
```

Classification:
- passed
- no pre-existing baseline compile drift observed in the read-only validation probe

## 13. Drift Found

Drift found:
- authority-shape drift: `docs/Current/MPWO_WORK_ORDER_PACK.md` currently contains revision-report content rather than a detailed executable work-order pack; the corrected split authority is still recoverable from the reviewed authority chain and the user’s prompt
- repo-state drift: the working tree contains extensive unrelated tracked and untracked changes, so any future ticket review will need explicit patch-basis hygiene
- planning-shape drift: older planning evidence names `agent_id` in worker events, but live child/team runtime surfaces do not currently expose a uniform cross-path `agent_id`

No material runtime drift found against the accepted corrected split baseline.

## 14. Blockers, If Any

No blocking runtime contradiction found.

Notes that must stay explicit during implementation:
- child-owned events cannot surface to the parent event stream without using an existing narrow return seam because nested child loops currently run with `event_tx = None`
- a uniform worker identifier field should be chosen deliberately before coding
- numeric `SessionBudgetExceeded` / `WorkerBudgetExceeded` payload fields may need narrow `SessionBudget` accessors

These are implementation notes, not preflight blockers.

## 15. Hosted Ollama Invariant Assessment

Assessment: `no regression risk identified from this ticket when kept observability-only`

Basis:
- hosted Ollama compatibility depends on existing provider dispatch and tool-use behavior, not on adding passive event variants
- same-domain fallback behavior remains owned by the already-landed `resolve_provider_with_fallback(...)` seam in [`provider_resolution.rs`](/home/jordi/claurst/src-rust/crates/query/src/provider_resolution.rs:302)
- `TASK-M11-11` does not need to change provider selection policy, provider configs, fallback-domain rules, or tool execution behavior
- root shared `SessionBudget`, inherited parent session-budget accounting, child `max_tokens`, child `allow_fallback`, and layered child `budget_usd` can remain unchanged while adding observability

## 16. Exact Recommendation for Next Step

Proceed to `TASK-M11-11` implementation as one narrow observability ticket on the accepted split baseline.

Recommended implementation guardrails:
- treat the landed layered `SessionBudget` model from `10B1` / `10B2` as the controlling runtime baseline
- emit `WorkerProviderResolved` from the existing child/team resolution points without redesigning provider-resolution policy
- emit `WorkerBudgetExceeded` for child-local `budget_usd` semantics only, not `max_budget_usd`
- emit `SessionBudgetExceeded` for shared-session `SessionBudget` semantics only, not child-local `budget_usd`
- keep budget and fallback behavior unchanged
- expect narrow compile fallout in `tui/src/app.rs`
- keep any child-to-parent event transport narrow by using an existing result seam rather than reopening `ToolContext`, `HealthCache`, or the accepted budget seam
