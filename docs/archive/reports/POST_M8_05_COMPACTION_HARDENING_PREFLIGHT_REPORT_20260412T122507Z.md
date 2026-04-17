# POST-M8-05 Shared Compaction/Context-Collapse Provider-Parity Hardening Preflight Report

**Pass Name**

`POST-M8-05 shared compaction/context-collapse provider-parity hardening`

**Timestamp UTC**

`20260412T122507Z`

**Branch**

`feature/provider-resolution-seam`

**Working Tree Summary**

- `git diff --name-only`: no unstaged tracked changes.
- `git diff --cached --name-only`: no staged changes.
- `git status --short`: untracked-only noise is present. Notable items: repo-root `AGENTS.md`, `docs/Current/MPWO_WORK_ORDER_PACK.md`, many historical report files under `docs/archive/reports/`, orchestration docs, `.codex`, and `src-rust/target/`.
- M8-05 is committed at `HEAD` as `5d246b2 TASK-M8-05 wire background provider resolution through shared seam`.
- M8-04 is committed at `HEAD~1` as `b5249a3 TASK-M8-04 wire foreground provider resolution through shared seam`.
- Baseline clean enough for a future tracked-source hardening execution pass: yes, but only with an explicit tracked-diff review basis. Whole-worktree cleanliness is not present because authority files and report noise are untracked.
- Scope readiness for execution: `READY-WITH-NOTES`.

**Authority Reviewed**

- Repo-local `AGENTS.md` at `/home/jordi/claurst/AGENTS.md`: present, readable, untracked, and treated as primary repo-local authority for this pass.
- `/home/jordi/claurst/docs/Current/MPWO_WORK_ORDER_PACK.md`: present, readable, untracked, and used for invariants, M8-04/M8-05 boundaries, and non-regression constraints.
- Relevant MPWO sections rechecked:
  - hosted Ollama invariant: `docs/Current/MPWO_WORK_ORDER_PACK.md:108-132`
  - M8-04 foreground scope and dummy-client guardrail: `docs/Current/MPWO_WORK_ORDER_PACK.md:978-1071`
  - M8-05 background scope: `docs/Current/MPWO_WORK_ORDER_PACK.md:1075-1115`
  - preserved `run_query_loop(client: &AnthropicClient, ...)` and legacy `provider_registry=None` path: `docs/Current/MPWO_WORK_ORDER_PACK.md:2020-2027`
- `Hosted Ollama compatibility baseline preserved`

| Type | Item | Verified outcome |
|---|---|---|
| command | `git branch --show-current` | `feature/provider-resolution-seam` |
| command | `git status --short` | untracked-only noise; no staged or unstaged tracked diffs |
| command | `git log --oneline --decorate -n 20` | M8-05 at `HEAD`; M8-04 at `HEAD~1` |
| file | `src-rust/crates/query/src/agent_tool.rs` | foreground/background sub-agent paths now propagate provider-aware config into `run_query_loop()` |
| file | `src-rust/crates/query/src/lib.rs` | registry-backed branch still exits before shared compaction/context-collapse block |
| file | `src-rust/crates/query/src/compact.rs` | compaction helpers still require `&AnthropicClient`; direct raw stream calls remain |
| file | `src-rust/crates/api/src/lib.rs` | empty-key `AnthropicClient::new()` is allowed, but raw `create_message_stream()` still validates at call time |
| file | `src-rust/crates/query/src/provider_resolution.rs` | existing shared seam already materializes providers and preserves hosted Ollama baseline behavior |

**Original Scope vs Hardening Extension**

- Original MPWO scope for this part of Milestone 8 ended at:
  - M8-04 foreground sub-agent provider resolution in `agent_tool.rs`
  - M8-05 background sub-agent provider resolution in `agent_tool.rs`
- Original MPWO explicitly did not authorize:
  - removing `run_query_loop(client: &AnthropicClient, ...)`
  - removing the legacy `provider_registry=None` Anthropic path
  - entering `init_team_swarm_runner()` / team propagation before M8-08
- This pass is not an M8 ladder ticket. It is an orchestrator-authorized post-M8-05 hardening insertion focused only on shared compaction/context-collapse provider parity.
- This pass is not normal worker/team propagation. It should not silently pull in M8-06, M8-08, later team work, or general provider-branch parity cleanup outside compaction/context-collapse.

**Current Post-M8-05 Control-Flow Recheck**

- Foreground sub-agent entry into `run_query_loop()` is at `src-rust/crates/query/src/agent_tool.rs:483-491`.
- Background sub-agent entry into `run_query_loop()` is at `src-rust/crates/query/src/agent_tool.rs:434-448`.
- Post-M8-05 propagation already present before those calls:
  - provider resolution/materialization: `src-rust/crates/query/src/agent_tool.rs:270-288`
  - child `QueryConfig.model = target.model_id`: `src-rust/crates/query/src/agent_tool.rs:388-390`
  - child `provider_registry: Some(registry.clone())`: `src-rust/crates/query/src/agent_tool.rs:405`
  - child `model_registry: ctx.model_registry.clone()`: `src-rust/crates/query/src/agent_tool.rs:408`
  - child provider hint propagation via `foreground_ctx.config.provider = Some(target.provider_id.clone())`: `src-rust/crates/query/src/agent_tool.rs:411-412`
- `run_query_loop()` chooses the registry-backed provider branch at `src-rust/crates/query/src/lib.rs:874`.
- Inside that branch, provider resolution and materialization still go through the preferred shared seam:
  - `resolve_provider_identity(...)`: `src-rust/crates/query/src/lib.rs:875-879`
  - `materialize_provider(...)`: `src-rust/crates/query/src/lib.rs:887-890`
- That registry-backed branch exits before the shared post-turn compaction block:
  - hard error returns at `src-rust/crates/query/src/lib.rs:881-900`
  - provider stream error return at `src-rust/crates/query/src/lib.rs:989-996`
  - tool-use path `continue`s at `src-rust/crates/query/src/lib.rs:1151-1184`
  - end-turn path returns at `src-rust/crates/query/src/lib.rs:1187-1199`
- The raw Anthropic path begins only after that branch at `src-rust/crates/query/src/lib.rs:1202-1204`.
- Shared compaction/context-collapse begins later at `src-rust/crates/query/src/lib.rs:1371-1452`.
- Explicit reachability result: current post-M8-05 registry-backed runs cannot currently reach shared compaction/context-collapse.
- This conclusion applies to both foreground and background sub-agent paths because both enter the same early-return registry-backed branch with `provider_registry: Some(...)`.
- This is also consistent with the M8-04 dummy-client adjudication: `AnthropicClient::new()` now allows empty keys at construction (`src-rust/crates/api/src/lib.rs:447-463`), but raw `create_message_stream()` still fails on first use with an empty key (`src-rust/crates/api/src/lib.rs:608-651`). Current safety depends on the registry-backed branch never falling into raw compaction.
- Hardening classification: still preventive only. It has parity value for future branch evolution, but it is not a live correctness requirement in the present post-M8-05 control flow.

**Shared Compaction Seam Analysis**

- `src-rust/crates/query/src/compact.rs` still binds shared compaction helpers directly to `&AnthropicClient`:
  - `micro_compact_if_needed(...)`: `src-rust/crates/query/src/compact.rs:263-306`
  - `summarise_head(...)`: `src-rust/crates/query/src/compact.rs:540-664`
  - `compact_conversation(...)`: `src-rust/crates/query/src/compact.rs:669-693`
  - `auto_compact_if_needed(...)`: `src-rust/crates/query/src/compact.rs:697-730`
  - `reactive_compact(...)`: `src-rust/crates/query/src/compact.rs:899-978`
  - `context_collapse(...)`: `src-rust/crates/query/src/compact.rs:987-1088`
- Direct raw `client.create_message_stream(...)` calls relevant to compaction are exactly:
  - `src-rust/crates/query/src/compact.rs:633` inside `summarise_head(...)`
  - `src-rust/crates/query/src/compact.rs:1050` inside `context_collapse(...)`
- The raw-call dependency then fans out transitively through:
  - `micro_compact_if_needed(...)` via `summarise_head(...)` at `src-rust/crates/query/src/compact.rs:293`
  - `compact_conversation(...)` via `summarise_head(...)` at `src-rust/crates/query/src/compact.rs:692`
  - `reactive_compact(...)` via `summarise_head(...)` at `src-rust/crates/query/src/compact.rs:936-937`
- `compact.rs` does not currently have enough local context to become safely registry-aware on its own for sub-agent/provider-backed runs:
  - it does not receive `tool_ctx.config.provider`
  - it does not receive `tool_ctx.config.provider_configs`
  - it does not receive a pre-materialized `ExecutionTarget` / `Arc<dyn LlmProvider>`
- The missing context already exists in `run_query_loop()`:
  - provider hint: `src-rust/crates/query/src/lib.rs:876`
  - provider config map: `src-rust/crates/query/src/lib.rs:890`
  - materialized provider object: `src-rust/crates/query/src/lib.rs:904`
  - provider-option builder available without API-crate changes: `src-rust/crates/query/src/lib.rs:256-332`
- Smallest-safe implication: keep provider resolution/materialization in `run_query_loop()` and pass already-resolved provider execution context into new provider-aware compaction helpers. Do not push new provider-selection logic down into `compact.rs`.

**Corrective / Hardening Options Compared**

- Option A: make `compact.rs` resolve/materialize providers internally whenever `provider_registry` is `Some(...)`.
  - Viability: low for a smallest-safe pass.
  - File count: likely broader than two files because provider hint/config plumbing would need to be threaded through several compact helper signatures and callers.
  - Behavioral risk: moderate. This increases the chance of changing legacy `provider_registry=None` behavior or widening into unrelated compact entrypoints.
  - Legacy preservation: possible, but only with more branching and more signature churn than current repo reality requires.
- Option B: keep provider resolution in `run_query_loop()`, and add targeted provider-aware compaction calls inside the existing registry-backed branch.
  - Viability: high.
  - File count: likely two-file.
  - Behavioral risk: lowest practical option.
  - Legacy preservation: strong. The legacy `provider_registry=None` Anthropic path can remain unchanged, and the `client` parameter can remain intact.
  - Narrow insertion point: after `messages.push(assistant_msg.clone())` at `src-rust/crates/query/src/lib.rs:1132` and before provider-branch tool-use handling at `src-rust/crates/query/src/lib.rs:1134-1184`, which matches the current legacy-path timing where compaction runs before stop/tool-use handling.
- Option C: unify the registry-backed branch with the full shared post-turn tail so provider-backed runs fall through to the same compaction/hook/session-memory logic as the Anthropic path.
  - Viability: technically possible, but broader than this hardening pass should take on.
  - File count: likely broader than two-file once stop hooks, session memory, auto-dream, and provider/tool-use ordering are revalidated.
  - Behavioral risk: high for this scope because the shared tail contains more than compaction.
  - Scope fit: poor. This would widen from compaction/context-collapse hardening into general provider-branch parity.
- Recommended option: `Option B`.

**Recommended Minimal Scope**

- Minimum likely patch classification: `two-file`.
- Minimum likely patch status: `READY-WITH-NOTES`.
- Definitely in scope:
  - `src-rust/crates/query/src/lib.rs`
  - `src-rust/crates/query/src/compact.rs`
- Maybe in scope:
  - existing test sections in those same files only if the narrow refactor forces compile/test fixture adjustments
- Should remain untouched:
  - `src-rust/crates/api/src/lib.rs`
  - `src-rust/crates/query/src/provider_resolution.rs`
  - `src-rust/crates/query/src/agent_tool.rs` for foreground/background M8-04/M8-05 wiring
  - `src-rust/crates/query/src/agent_tool.rs:570-650` (`init_team_swarm_runner()`), which is still the M8-08 boundary
  - `src-rust/crates/query/src/team_tool.rs` because no such file exists in the current repo; do not create it for this hardening
  - CLI startup / root registry construction
  - `src-rust/crates/query/src/cron_scheduler.rs`
- Smallest safe implementation shape for a later execution pass:
  - keep `run_query_loop(client: &AnthropicClient, ...)` unchanged
  - keep the legacy `provider_registry=None` path unchanged
  - do not add fallback behavior
  - in `lib.rs`, add a provider-branch-local compaction decision point rather than full branch unification
  - in `compact.rs`, add provider-aware helper variants that consume already-materialized provider context and reuse existing prompt/summary formatting logic
- Broadening stop condition for a later execution pass:
  - if the only way forward turns out to be removing the `client` parameter from `run_query_loop()` or changing the legacy `provider_registry=None` Anthropic path, that is scope expansion and should stop for re-authorization

**Validation Command**

- Future execution-pass validation command: `cd /home/jordi/claurst/src-rust && cargo check -p claurst-query`
- Validation was not run in this preflight. Reason: the pass was kept read-only except for this report file, and running `cargo check` would write under `src-rust/target/`, violating the requested worktree constraint.

**Drift Found**

- `AGENTS.md` exists but is untracked.
- `docs/Current/MPWO_WORK_ORDER_PACK.md` exists but is untracked.
- Suggested prompt target `src-rust/crates/query/src/team_tool.rs` does not exist in current repo reality; the team runner lives in `src-rust/crates/query/src/agent_tool.rs`.
- MPWO line references for M8-04/M8-05 are line-number-drifted relative to current file layout, but the intended symbols and behaviors remain identifiable and matched the repo reality.

**Blockers**

- No blocker prevents a later hardening execution pass.
- Notes that must be carried into execution:
  - review basis must be explicit because the worktree has extensive untracked noise
  - the authority files themselves are untracked
  - the hardening should remain narrow and avoid full provider-branch parity cleanup

**Verdict**

- Ticket/pass id: `POST-M8-05 shared compaction/context-collapse provider-parity hardening`
- Preflight verdict: `PASS`
- Current post-M8-05 registry-backed runs can reach shared compaction/context-collapse: `No`
- Hardening classification: `preventive only`
- Minimum likely patch: `two-file`
- Execution readiness: `READY-WITH-NOTES`
- Hosted Ollama baseline statement: `Hosted Ollama compatibility baseline preserved`
- Recommended next execution scope:
  - touch only `src-rust/crates/query/src/lib.rs` and `src-rust/crates/query/src/compact.rs`
  - keep provider resolution/materialization on the existing shared seam
  - keep `run_query_loop(client: &AnthropicClient, ...)` and the legacy `provider_registry=None` path intact
