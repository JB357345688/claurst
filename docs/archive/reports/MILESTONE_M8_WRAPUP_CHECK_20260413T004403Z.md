# Milestone 8 Wrap-Up Check

## Milestone
`8`

## Timestamp UTC
`2026-04-13T00:44:03Z`

## Branch
`feature/provider-resolution-seam`

## Repo State Summary
- `git branch --show-current`: `feature/provider-resolution-seam`
- `git diff --name-only`: empty
- `git diff --cached --name-only`: empty
- Current tracked state is clean: yes
- `git status --short` shows substantial untracked workspace noise under:
  - `.codex/`
  - `docs/Current/`
  - `docs/Orchestrator_planning/`
  - `docs/archive/provider_orchestrator/`
  - `docs/archive/reports/`
  - `src-rust/target/`
- Review basis note:
  - tracked closure basis is clean
  - some authority/supporting documents used by this audit are untracked working-tree artifacts because the prompt explicitly required using them
  - where a report was absent or not commit-anchored, git history and live current code were used directly
- `git log --oneline --decorate -n 40` shows the full M8 closeout chain ending at:
  - `b5b6dd4 (HEAD -> feature/provider-resolution-seam) TASK-M8-11 reconcile M8 workspace validation and formatting`

## Authority Reviewed
- Primary authority:
  - `AGENTS.md`
  - `docs/Current/MPWO_WORK_ORDER_PACK.md`
- Secondary authority only:
  - `docs/Orchestrator_planning/IMPLEMENTATION_PLAN_MPWO.md`
- Milestone prerequisite evidence:
  - `docs/archive/reports/TASK-M7-07_CLOSEOUT_REPORT_20260412T040059Z.md`
- M8 milestone evidence read:
  - `docs/archive/reports/TASK-M8-07_M8-08_RECONCILIATION_REPORT_20260412T160309Z.md`
  - `docs/archive/reports/TASK-M8-08_COMMIT_VERIFICATION_REPORT_20260412T152849Z.md`
  - `docs/archive/reports/TASK-M8-08_POST_M8-07_NONREGRESSION_REPORT_20260412T230523Z.md`
  - `docs/archive/reports/TASK-M8-07_COMMIT_VERIFICATION_REPORT_20260412T225608Z.md`
  - `docs/archive/reports/TASK-M8-09_COMMIT_VERIFICATION_REPORT_20260412T232152Z.md`
  - `docs/archive/reports/TASK-M8-10_COMMIT_VERIFICATION_REPORT_20260412T235810Z.md`
  - `docs/archive/reports/TASK-M8-11_COMMIT_VERIFICATION_REPORT_20260413T003542Z.md`
  - `docs/archive/reports/TASK-M8-11_CLOSEOUT_REPORT_20260413T003451Z.md`
  - `docs/archive/reports/TASK-M8-11_REVIEW_REPORT_20260413T002956Z.md`
  - `docs/archive/reports/TASK-M8-11_FORMAT_SCOPE_RECONCILIATION_REPORT_20260413T001700Z.md`
  - all located M8-01 through M8-06 closeout / commit-verification reports
- Missing report note:
  - no standalone `TASK-M8-01_COMMIT_VERIFICATION_REPORT_*` was found in-repo
  - `TASK-M8-01` commit presence was verified directly with `git show --stat --oneline 3f9b783` plus the committed closeout report path

## Milestone Ticket Ledger

| Ticket | Short objective | Declared dependency tickets from MPWO | Current status on this branch | Commit hash / subject | Evidence used |
|---|---|---|---|---|---|
| `M8-01` | Add `provider_registry` and `model_registry` to `ToolContext` | Milestone-wide base: `M7-07` complete | `verified closed` | `3f9b783` `TASK-M8-01 add provider and model registries to ToolContext` | direct `git show`, `TASK-M8-01_CLOSEOUT_REPORT_20260412T050544Z.md`, live current `src-rust/crates/tools/src/lib.rs:216-234` |
| `M8-02` | Populate startup `ToolContext` with root registries | `M7-07`, `M8-01` | `verified closed` | `fe060af` `TASK-M8-02 wire root registries into startup ToolContext` | `TASK-M8-02_COMMIT_VERIFICATION_REPORT_20260412T061134Z.md`, live current `src-rust/crates/cli/src/main.rs:729-741` |
| `M8-03` | Add `provider` field to `AgentInput` and schema | `M7-07` | `verified closed` | `f4dc962` `TASK-M8-03 add provider field to AgentTool input schema` | `TASK-M8-03_COMMIT_VERIFICATION_REPORT_20260412T100130Z.md`, live current `src-rust/crates/query/src/agent_tool.rs:133-152,181-228` |
| `M8-04` | Replace foreground AgentTool Anthropic-only path with shared seam | `M7-07`, `M8-01`, `M8-03` | `verified closed` | `b5249a3` `TASK-M8-04 wire foreground provider resolution through shared seam` | `TASK-M8-04_COMMIT_VERIFICATION_REPORT_20260412T113834Z.md`, live current `src-rust/crates/query/src/agent_tool.rs:272-321,390-415` |
| `M8-05` | Replace background AgentTool Anthropic-only path with shared seam | ticket text: `M8-04`; graph also lists `M8-01` and `M8-03`; milestone base `M7-07` | `verified closed` | `5d246b2` `TASK-M8-05 wire background provider resolution through shared seam` | `TASK-M8-05_COMMIT_VERIFICATION_REPORT_20260412T121007Z.md`, live current `src-rust/crates/query/src/agent_tool.rs:428-445` |
| `M8-06` | Migrate `AgentRunFn` positional seam to `AgentRunParams` | `M7-07` | `verified closed` | `ea9da37` `TASK-M8-06 replace AgentRunFn positional seam with AgentRunParams` | `TASK-M8-06_COMMIT_VERIFICATION_REPORT_20260412T145426Z.md`, live current `src-rust/crates/tools/src/team_tool.rs:36-50,68-74` |
| `M8-07` | Add `provider` / `model` to `AgentSpec` and TeamCreate schema | `M7-07` | `verified closed` | `5d472cf` `TASK-M8-07 add provider/model fields to team spec schema` | `TASK-M8-07_COMMIT_VERIFICATION_REPORT_20260412T225608Z.md`, live current `src-rust/crates/tools/src/team_tool.rs:156-171,213-252` |
| `M8-08` | Replace `init_team_swarm_runner()` Anthropic-only producer path with shared seam | `M7-07`, `M8-06` | `verified closed` | `1d531da` `TASK-M8-08 wire team runner producer through shared provider seam` | `TASK-M8-08_CLOSEOUT_REPORT_20260412T152403Z.md`, `TASK-M8-08_COMMIT_VERIFICATION_REPORT_20260412T152849Z.md`, `TASK-M8-08_POST_M8-07_NONREGRESSION_REPORT_20260412T230523Z.md`, live current `src-rust/crates/query/src/agent_tool.rs:572-721` |
| `M8-09` | Wire `TeamCreateTool::execute()` to pass provider/model overrides through `AgentRunParams` | `M7-07`, `M8-06`, `M8-07` | `verified closed` | `2fd7732` `TASK-M8-09 wire team spec provider/model into override fields` | `TASK-M8-09_COMMIT_VERIFICATION_REPORT_20260412T232152Z.md`, live current `src-rust/crates/tools/src/team_tool.rs:389-421` |
| `M8-10` | Add worker provider-resolution tests | graph: `M8-04`, `M8-05`, `M8-08`, `M8-09`; ticket text preconditions list `M8-04`, `M8-05`, `M8-08`; milestone base `M7-07` | `verified closed` | `1056eb3` `TASK-M8-10 add agent tool provider seam coverage tests` | `TASK-M8-10_COMMIT_VERIFICATION_REPORT_20260412T235810Z.md`, `TASK-M8-10_CLOSEOUT_REPORT_20260412T235709Z.md` |
| `M8-11` | Final workspace validation for Milestone 8 | `M7-07`, all `M8-01` through `M8-10` | `verified closed` | `b5b6dd4` `TASK-M8-11 reconcile M8 workspace validation and formatting` | `TASK-M8-11_FORMAT_SCOPE_RECONCILIATION_REPORT_20260413T001700Z.md`, `TASK-M8-11_REVIEW_REPORT_20260413T002956Z.md`, `TASK-M8-11_CLOSEOUT_REPORT_20260413T003451Z.md`, `TASK-M8-11_COMMIT_VERIFICATION_REPORT_20260413T003542Z.md` |

## Dependency Graph Reconciliation
- Milestone-wide prerequisite from MPWO:
  - all M8 tickets require `M7-07` complete
  - evidence used: `TASK-M7-07_CLOSEOUT_REPORT_20260412T040059Z.md`, which records commit `b8cc827` and passing `fmt`, `build`, `test`, and `clippy`
- Top-level MPWO graph declares these M8 edges:
  - `M8-01 -> M8-02`
  - `M8-01 -> M8-04`
  - `M8-01 -> M8-05`
  - `M8-03 -> M8-04`
  - `M8-03 -> M8-05`
  - `M8-06 -> M8-08`
  - `M8-07 -> M8-09`
  - `M8-06 -> M8-09`
  - `M8-08 -> M8-10`
  - `M8-04 -> M8-10`
  - `M8-05 -> M8-10`
  - `M8-09 -> M8-10`
  - `M8-10 -> M8-11`
- Ticket-text preconditions add two extra clarifications:
  - `M8-05` explicitly says `M8-04` complete
  - `M8-11` explicitly requires all `M8-01` through `M8-10` complete
- Ticket-text preconditions also differ from the graph in one place:
  - `M8-10` preconditions list `M8-04`, `M8-05`, and `M8-08`
  - the top-level graph additionally includes `M8-09 -> M8-10`
- Current branch history satisfies the stricter union of those dependency statements:
  - `3f9b783` `M8-01`
  - `fe060af` `M8-02`
  - `f4dc962` `M8-03`
  - `b5249a3` `M8-04`
  - `5d246b2` `M8-05`
  - `ea9da37` `M8-06`
  - `1d531da` `M8-08`
  - `5d472cf` `M8-07`
  - `2fd7732` `M8-09`
  - `1056eb3` `M8-10`
  - `b5b6dd4` `M8-11`
- Result:
  - no unresolved dependency-edge violation remains at milestone closure time
  - the milestone is dependency-clean on the current branch state

## Sequencing / Recovery Notes
- Earlier `M8-07` / `M8-08` numeric-order drift is now resolved at milestone level: yes
- Why it is resolved:
  - `TASK-M8-07_M8-08_RECONCILIATION_REPORT_20260412T160309Z.md` established that MPWO does **not** declare `M8-07 -> M8-08`
  - MPWO makes `M8-07` and `M8-08` sibling tickets, with `M8-09` as the connector that first activates TeamCreate override transport
  - `M8-08` landed first at `1d531da` without trespassing into `team_tool.rs`
  - `M8-07` then landed cleanly at `5d472cf` in `team_tool.rs`
  - `TASK-M8-08_POST_M8-07_NONREGRESSION_REPORT_20260412T230523Z.md` verified that `M8-08` remained valid after retroactive `M8-07`
  - current `HEAD` still preserves the accepted `M8-08` contract live:
    - `init_team_swarm_runner()` still destructures `AgentRunParams` and consumes `provider_override` / `model_override` at `src-rust/crates/query/src/agent_tool.rs:577-619`
    - it still routes through `resolve_provider_identity(...)` and `materialize_provider(...)` at `src-rust/crates/query/src/agent_tool.rs:615-641`
    - it still propagates `provider_registry`, `model_registry`, and resolved model into child `QueryConfig` at `src-rust/crates/query/src/agent_tool.rs:685-695`
    - it still preserves the selected provider into the nested runner context at `src-rust/crates/query/src/agent_tool.rs:698-699`
  - `M8-09` then activated the transport seam by wiring `spec.provider` / `spec.model` into `AgentRunParams` at `src-rust/crates/tools/src/team_tool.rs:389-421`
  - `M8-10` added tests only
  - `M8-11` made formatting-only fixes inside the approved three-file recovery surface and then passed all workspace validations
- `M8-08` remains valid after `M8-07`, `M8-09`, `M8-10`, and `M8-11`: yes
- `M8-11` validation-only closure is sufficient for formal milestone closure: yes
  - MPWO defines `M8-11` as validation-only with no exact code targets and Definition of Done `All four validation commands pass`
  - the report trail shows:
    - initial format failure identified
    - scope reconciled as a permitted M8-local three-file formatting-only recovery
    - `cargo fmt --all -- --check`, `cargo build --workspace`, `cargo test --workspace`, and `cargo clippy --workspace --all-targets` all passed before closeout
    - commit verification at `b5b6dd4` confirms current `HEAD` matches that reviewed basis

## Invariant Status
- Standing non-regression invariant reviewed:
  - hosted Ollama compatibility baseline from commit `5f8dfe1`
  - specifically:
    - `normalize_ollama_api_base(...)`
    - `AuthStore::load().api_key_for(ProviderId::OLLAMA)`
    - environment-first precedence for `AuthStore::api_key_for("ollama")`
- Invariant preserved across the accepted M8 basis: yes
- Basis:
  - MPWO section `2A` makes this invariant controlling across remaining M7-M12 seam work
  - every reviewed M8 closeout / verification artifact explicitly records `Hosted Ollama compatibility baseline preserved`
  - no current tracked drift exists
  - `M8-10` is test-only and `M8-11` is formatting-only, so no late-M8 semantic drift re-opened the Ollama path

## Formal Closure Assessment
- Are all MPWO-defined M8 tickets complete on the branch?
  - yes
  - `M8-01` through `M8-11` all have landed commits in branch history, and their owned current-head surfaces still match the accepted basis
- Are all tickets that should be commit-verified actually commit-verified?
  - yes on current audit basis
  - dedicated commit-verification reports exist for `M8-02` through `M8-11`
  - `M8-01` lacks a standalone commit-verification report artifact, but this audit directly verified the closeout commit and current live code, so no commit-presence gap remains
- Is current tracked state clean?
  - yes
- Is the hosted-Ollama invariant preserved across the accepted M8 basis?
  - yes
- Is there any remaining known issue that prevents saying `Milestone 8 is formally closed`?
  - no blocking issue remains
- Historical notes that do **not** block closure:
  - `M8-08` MPWO wording inconsistency about preserved client parameter vs `AnthropicClient::new()` removal remains a text note, but branch acceptance explicitly resolved it in favor of the preserved-client-parameter interpretation
  - no standalone `M8-01` commit-verification report file exists, but this wrap-up audit filled that evidence gap directly from git history

## Outstanding Items
- No blocking outstanding items remain for Milestone 8 closure.
- Non-blocking process note:
  - if the branch wants artifact symmetry, a standalone `TASK-M8-01_COMMIT_VERIFICATION_REPORT` could be added later, but it is not required by MPWO and it is not needed to justify closure on current evidence.

## Final Recommendation
`M8 FORMALLY CLOSED`

Clean next move:
- begin next-ticket preflight
- because AGENTS requires one ticket at a time and the current prompt forbids entering the next milestone now, that preflight should happen only on explicit user instruction
