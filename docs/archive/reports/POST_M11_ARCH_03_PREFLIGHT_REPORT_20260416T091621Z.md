# POST-M11-ARCH-03 Preflight Report

## 1. Ticket ID
`POST-M11-ARCH-03`

## 2. Verdict
`REMAIN-DEFERRED`

## 3. Timestamp UTC
`20260416T091621Z`

## 4. Branch / HEAD / worktree summary
- Branch: `feature/provider-resolution-seam`
- HEAD: `038f3c20e01a96eec6397d506b477a461166f762`
- `HEAD` matches the accepted `POST-M11-ARCH-01A` commit named in the prompt.
- No staged changes were present.
- Tracked worktree noise is present in unrelated paths: `.gitignore`, `docs/Current/TASK-M11-10_SPLIT_PLAN_AND_TEMPORARY_AUTHORITY_AMENDMENT.md`, `src-rust/crates/api/src/providers/google.rs`, `src-rust/crates/core/src/effort.rs`, `src-rust/crates/core/src/lib.rs`, `src-rust/crates/core/src/remote_settings.rs`, `src-rust/crates/core/src/system_prompt.rs`.
- Untracked noise is very large: `27219` paths, dominated by `src-rust/target` (`26897` paths), plus many archive reports and planning files. This would materially confuse later scope attribution if not called out.
- Drift verdict: no structural ticket drift found in the inspected cancellation seam itself; only unrelated worktree noise.
- Blockers: none for preflight.

## 5. Authority reviewed
- Repo authority: `AGENTS.md`
- Live current authority: `docs/Current/MPWO_WORK_ORDER_PACK.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md` explicitly says it is the single active authority artifact in `docs/Current/` and the sole current-authority artifact there (`docs/Current/MPWO_WORK_ORDER_PACK.md:5-6`, `:22-23`, `:30-37`).
- `rg --files docs/Current` found `MPWO_WORK_ORDER_PACK.md`, `MPWO_WORK_ORDER_PACK_pre_M10_revision.md`, `IMPLEMENTATION_PLAN_MPWO.md`, `D1_REVIEW_REPORT_...`, `M10_D2_IMPLEMENTATION_PLAN_REPORT_...`, `HOSTED_OLLAMA_IMPLEMENTATION_PLAN.md`, and `RFC_PROVIDER_AWARE_WORKER_FABRIC_v3.1.md`. The live pack itself marks only `MPWO_WORK_ORDER_PACK.md` active and the rest historical or non-controlling.
- Nothing in the live pack forbids a preflight assessment of a deferred post-M11 architecture item. The live constraint is only that accepted M11 runtime semantics must not be casually reopened (`docs/Current/MPWO_WORK_ORDER_PACK.md:22-26`).
- Verified commands and searches used in this preflight:
- `git branch --show-current`
- `git rev-parse HEAD`
- `git status --short --branch`
- `git diff --name-only`
- `git diff --cached --name-only`
- `git ls-files --others --exclude-standard | wc -l`
- `git ls-files --others --exclude-standard | cut -d/ -f1-2 | sort | uniq -c | sort -nr | head -n 20`
- `rg --files docs/Current`
- `rg -n "CancellationToken|ACTIVE_TEAMS|cancelled\\(|TeamDelete|register_agent_runner|run_agent|child_cancel_token|SessionBudgetExceeded|TeamCreate|cancel"` over the Rust tree
- direct source inspection of the files named in the prompt plus directly relevant call sites and tests
- focused verification runs:
- `cargo test -p claurst-query teamcreate_`
- `cargo test -p claurst-query session_budget_`
- `cargo test -p claurst-query child_token_is_cancelled_with_root`

## 6. Current live cancellation-seam findings
- `claurst-tools` intentionally stays independent from `claurst-query`; `team_tool.rs` documents the injected runner boundary and the circular-dependency reason for it (`src-rust/crates/tools/src/team_tool.rs:8-19`). `src-rust/crates/tools/src/lib.rs:88` only re-exports `register_agent_runner`, `AgentRunFn`, `TeamCreateTool`, and `TeamDeleteTool`; it adds no extra cancellation logic.
- TeamCreate currently creates one fresh outer `CancellationToken` per agent, stores the vector in the process-global `ACTIVE_TEAMS`, and keys it by the sanitized final team name (`src-rust/crates/tools/src/team_tool.rs:83-90`, `:391-398`).
- TeamCreate does not pass those outer tokens into the injected query runner. Each worker future simply races `run_agent(...)` against the outer token and returns `"[Agent cancelled by TeamDelete]"` if the outer token wins (`src-rust/crates/tools/src/team_tool.rs:432-456`).
- TeamDelete currently removes the token vector from `ACTIVE_TEAMS`, cancels each token, counts them, and then removes the team directory (`src-rust/crates/tools/src/team_tool.rs:559-616`).
- Query-side child cancellation is created from `SessionBudget`, not from TeamCreate’s outer tokens. `SessionBudget::child_scope` uses `parent.child_cancel_token()` for its root token, `check_and_cancel()` cancels the scope token when spend crosses the limit, and `child_cancel_token()` returns a child of that root (`src-rust/crates/query/src/session_budget.rs:36-63`).
- The query-side helper `inherited_child_cancel_token()` returns `session_budget.child_cancel_token()` when a budget exists and otherwise a default token (`src-rust/crates/query/src/agent_tool.rs:148-154`).
- The injected TeamCreate runner in `claurst-query` builds a child session budget from the parent session registration, then derives an inner query-loop cancel token from that budget and passes it to `run_query_loop()` (`src-rust/crates/query/src/agent_tool.rs:798-830`).
- The root query loop also receives a cancel token derived from the session budget when one exists; the CLI helper `root_query_cancel_token()` does the same `budget.child_cancel_token()` mapping (`src-rust/crates/cli/src/main.rs:104-109`, `:1129-1139`).
- Inside `run_query_loop()`, cancellation is checked before each turn and inside provider-streaming `tokio::select!` loops (`src-rust/crates/query/src/lib.rs:960-963`, `:1241-1245`, `:1586-1590`).
- `run_query_loop()` does not wrap tool execution itself in a cancel-aware `select!`; it directly awaits `execute_tool(...)` (`src-rust/crates/query/src/lib.rs:1478-1516`). That matters for any long-running tool, not TeamCreate alone.
- Current observability only distinguishes `WorkerProviderResolved`, `WorkerBudgetExceeded`, and `SessionBudgetExceeded` events (`src-rust/crates/query/src/lib.rs:400-447`, `:456-470`, `:598-613`). There is no worker-cancellation event and no event that distinguishes TeamDelete cancellation from other cancellation causes.
- TeamCreate output sanitization only extracts provider/budget observability payloads from child results; it does not encode a cancellation-source event (`src-rust/crates/query/src/lib.rs:499-595`).
- Existing tests cover TeamCreate provider routing, shared HealthCache reuse, child-budget helper behavior, TeamCreate observability sanitization, and session-budget event emission (`src-rust/crates/query/src/agent_tool_tests.rs:487-707`, `src-rust/crates/query/src/lib.rs:2780-2879`).
- No direct TeamDelete test or direct `ACTIVE_TEAMS` cancellation-path test was found in the current tree.

## 7. Concrete problem assessment
- TeamDelete’s current outer token path appears to stop the work it claims to stop in the direct TeamCreate path. The outer token resolves the `tokio::select!` in TeamCreate and drops the `run_agent(...)` future (`src-rust/crates/tools/src/team_tool.rs:439-454`). I found no live-code evidence of an orphaned task, stuck `join_all`, or leaked TeamCreate worker specific to that seam.
- Query-side inner cancellation covers a different trigger domain. It handles budget-driven or caller-provided query-loop cancellation inside `run_query_loop()` and provider streaming (`src-rust/crates/query/src/agent_tool.rs:798-830`, `src-rust/crates/query/src/lib.rs:960-963`, `:1241-1245`, `:1586-1590`).
- The two layers are therefore complementary in ownership and trigger source:
- Outer layer: named-team control plane for `TeamDelete`
- Inner layer: session-budget and query-loop runtime control plane
- I did not find a concrete present defect showing that TeamDelete cannot stop a TeamCreate worker, nor a code path proving the two layers create a leak, orphan, or stuck-work condition.
- I did find two narrower gaps:
- The cancellation source is not observable through the current `QueryEvent` model. TeamDelete cancellation, budget-driven cancellation, and generic query-loop cancellation ultimately collapse into output strings rather than a typed event.
- There is no direct TeamDelete regression test.
- Those gaps are real, but they do not prove that TeamCreate outer-cancellation ownership is wrong. They point to test/observability coverage, not to a required redesign of the outer-versus-inner ownership model.
- The one stronger runtime limitation visible in live code is broader than this ticket: parent query cancellation is not checked while any tool is executing because `execute_tool(...)` is awaited directly in `run_query_loop()` (`src-rust/crates/query/src/lib.rs:1478-1516`). That is a general long-running-tool cancellation seam, not a TeamCreate-specific outer-cancellation defect, and it would require broader changes than `POST-M11-ARCH-03`.

## 8. Design-space comparison
- Option A: remain deferred. Owning layer: none. Touched files: none. Reopens accepted seams: no. Coupling risk: low. Crate-boundary risk: none. Regression risk: lowest. Observability stays imperfect, but no current runtime defect is left unaddressed. Does not force `POST-M11-ARCH-02` or broader redesign.
- Option B: narrow clarification/helper without ownership redesign. Owning layer: mostly `claurst-query` tests or observability helpers, possibly plus TeamCreate tests. Likely touched files: `src-rust/crates/query/src/agent_tool_tests.rs`, `src-rust/crates/query/src/lib.rs`, possibly `src-rust/crates/tools/src/team_tool.rs` if a TeamDelete test seam is added. Reopens accepted seams: minimally if test-only, somewhat if runtime observability is changed. Coupling risk: low to medium. Crate-boundary risk: low if it avoids new cross-crate cancel handles. Regression risk: low to medium. Observability improves, but this does not justify `ARCH-03` as a cancellation-ownership redesign ticket.
- Option C: unify outer TeamDelete cancellation with query-side cancellation for TeamCreate only. Owning layer: both `claurst-tools` and `claurst-query`. Likely touched files: `src-rust/crates/tools/src/team_tool.rs`, `src-rust/crates/query/src/agent_tool.rs`, and likely `ToolContext` or `AgentRunParams` plumbing. Reopens accepted seams: yes, especially TeamCreate/TeamDelete behavior and the injected runner seam. Coupling risk: medium to high. Crate-boundary risk: high because `claurst-tools` cannot depend on `claurst-query` and currently relies on injection specifically to avoid that (`src-rust/crates/tools/src/team_tool.rs:8-19`). Regression risk: medium to high. Observability could improve, but the change would pressure broader API seam changes. It does not force `ARCH-02`, but it clearly pressures broader runtime-seam redesign.
- Option D: broader shared or neutral cancellation abstraction. Owning layer: new shared abstraction plus both crates. Likely touched files: `src-rust/crates/tools/src/team_tool.rs`, `src-rust/crates/tools/src/lib.rs`, `src-rust/crates/query/src/agent_tool.rs`, `src-rust/crates/query/src/session_budget.rs`, `src-rust/crates/query/src/lib.rs`, likely additional crate wiring. Reopens accepted seams: yes, broadly. Coupling risk: high. Crate-boundary risk: highest. Regression risk: high. Observability could become cleaner, but this is architecture expansion without current proof. This is not justified by live repo evidence.
- Option E: broader root tool-execution cancellation propagation. Owning layer: `claurst-query` and possibly `ToolContext`. Likely touched files: `src-rust/crates/query/src/lib.rs`, `src-rust/crates/tools/src/lib.rs`, call sites in `src-rust/crates/cli/src/main.rs`, and any long-running tool that should respect a parent cancel handle. Reopens accepted seams: yes, and not only TeamCreate. Coupling risk: medium to high. Crate-boundary risk: medium. Regression risk: medium to high. Observability and interrupt behavior could improve for all long-running tools, but this is a broader ticket than `POST-M11-ARCH-03`.
- Best current option: Option A. The only plausible narrower work today is Option B, but that is a separate test or observability clarification track, not the deferred TeamCreate outer-cancellation redesign itself.

## 9. Risk / interaction analysis
- TeamCreate / TeamDelete behavior: any redesign would directly alter `ACTIVE_TEAMS`, cancellation races, output strings, and directory-cleanup timing in `src-rust/crates/tools/src/team_tool.rs:391-616`.
- `AgentRunParams` / injected runner seam: a real unification would likely need new cancel plumbing through `AgentRunParams` or `ToolContext`, reopening the boundary documented in `src-rust/crates/tools/src/team_tool.rs:8-19` and exercised in `src-rust/crates/query/src/agent_tool.rs:706-844`.
- `SessionBudget` cancellation behavior: changing ownership or parentage of tokens would risk the current child-scope semantics in `src-rust/crates/query/src/session_budget.rs:36-63`.
- `ARCH-01A` HealthCache seam: the accepted session-scoped HealthCache reuse lives in the same injected runner path (`src-rust/crates/query/src/agent_tool.rs:765-775`) and is covered by tests (`src-rust/crates/query/src/agent_tool_tests.rs:487-565`). Touching the runner for cancellation redesign risks incidental regressions here.
- `ToolContext` API surface: any attempt to let `claurst-tools` observe parent query cancellation would likely require adding a generic cancel handle to `ToolContext`. That is materially broader than this ticket.
- Hosted-Ollama and provider materialization locality: cancellation redesign in the injected runner would sit adjacent to provider-resolution and provider-materialization logic in `src-rust/crates/query/src/agent_tool.rs:748-790`. It would not directly reopen `ARCH-02`, but it would increase risk in the same seam that `ARCH-02` explicitly leaves deferred.
- Existing tests: current coverage proves provider routing, shared HealthCache reuse, TeamCreate observability sanitization, and session-budget helper behavior. It does not cover TeamDelete cancellation. Any runtime change here would need new TeamDelete tests and must keep the currently passing provider/budget/observability tests green.
- Specific evaluation of the current outer-cancellation model:
- TeamDelete’s outer token currently stops the right TeamCreate work in practice as coded. It cancels the per-agent token and the worker future stops via TeamCreate’s `tokio::select!`.
- Query-side child/session-budget cancellation covers different ground. It controls nested query-loop stop behavior and budget propagation, not named-team deletion.
- The present two-layer model creates benign redundancy when both layers fire near the same time. The ambiguous part is source attribution, not stoppability.
- The separation is materially useful because it preserves `claurst-tools` independence from `claurst-query`.
- A redesign would pressure either a neutral cancellation abstraction or a broader API seam change. Neither is justified by current evidence.

## 10. Validation expectations if later implemented
- Add a direct TeamDelete cancellation test that runs a deliberately blocking TeamCreate worker and asserts that `TeamDelete` cancels the worker and unblocks TeamCreate deterministically.
- Add a direct regression test for cancellation-source semantics if runtime behavior is intentionally changed. That test must spell out expected output or expected typed event for:
- TeamDelete-driven cancellation
- shared session-budget cancellation
- any deliberate precedence rule when both happen together
- Preserve and rerun current TeamCreate/provider/budget observability tests:
- `cargo test -p claurst-query teamcreate_`
- `cargo test -p claurst-query session_budget_`
- `cargo test -p claurst-query child_token_is_cancelled_with_root`
- If any runtime seam around `ToolContext` or the injected runner changes, add focused non-regression tests for:
- `child_and_team_fallback_share_session_health_cache`
- `teamcreate_mixed_providers_per_agent_dispatch`
- `teamcreate_observability_is_sanitized_and_emitted`
- If a later ticket tries to solve parent-interrupt propagation rather than TeamDelete semantics, validation must expand beyond TeamCreate because that is a broader query-loop/tool-execution cancellation change.

## 11. Recommended next-step decision
`REMAIN-DEFERRED`

The live repo after `ARCH-01A` does not show a concrete current defect that justifies reopening TeamCreate outer-cancellation ownership now. The direct TeamDelete stop path is present and coherent, the inner query/session-budget path serves a different trigger domain, and the sharper issues visible today are either:
- a test/observability gap, or
- a broader parent-tool cancellation limitation in `run_query_loop()` that is not TeamCreate-specific.

Neither of those is a sound reason to spend the deferred architecture ticket on an ownership redesign.

## 12. Exact follow-on boundary
Not applicable. No implementation ticket is justified under `POST-M11-ARCH-03` at this time.

## 13. Explicit out-of-scope list
- No need found to reopen `POST-M11-ARCH-01A`.
- No need found to reopen `POST-M11-ARCH-02`.
- No need found to reopen accepted M11 runtime tickets.
- No need found to reopen M12.
- No need found to broaden into unrelated docs cleanup or worktree cleanup.
- No need found to redesign provider resolution.
- No need found to redesign session-budget ownership wholesale.

## 14. Risks / notes
- Review-basis risk for any future follow-on remains high until unrelated worktree noise is isolated. The preflight itself is still valid because it was read-only apart from this report.
- The current runtime path has an untested outer-cancellation seam for TeamDelete. That is worth noting for future bug triage, but it is not enough by itself to justify architecture redesign.
- If future evidence does surface, the most likely valid ticket will not be “unify everything.” It will either be:
- a narrow TeamDelete test or observability ticket, or
- a broader query-loop tool-execution cancellation ticket that explicitly covers more than TeamCreate.

## 15. Final recommendation
Keep `POST-M11-ARCH-03` deferred.

Current live repo evidence supports the accepted split:
- TeamDelete owns named-team stop control in `claurst-tools`
- query/session-budget logic owns nested runtime cancellation in `claurst-query`

That split is currently good enough for the accepted runtime path. The repo does not presently justify a TeamCreate outer-cancellation redesign, and any attempt to force unification now would reopen broader seams than this ticket can safely carry.
