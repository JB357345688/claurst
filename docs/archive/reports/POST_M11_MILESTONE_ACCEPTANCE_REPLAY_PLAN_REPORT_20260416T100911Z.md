# 1. Ticket ID

`POST-M11-MILESTONE-ACCEPTANCE-REPLAY-PLAN`

# 2. Timestamp UTC

`20260416T100911Z`

# 3. Branch / HEAD / worktree summary

- Preflight verdict: `READY-FOR-SPLIT-REPLAY`
- Current branch: `feature/provider-resolution-seam`
- Current `HEAD`: `038f3c20e01a96eec6397d506b477a461166f762`
- `HEAD` subject: `feat(query): add session-scoped HealthCache reuse`
- `HEAD` exactly matches the user-supplied accepted `POST-M11-ARCH-01A` commit: `yes`
- Current tracked worktree noise:
  - modified tracked files: `6`
  - deleted tracked files: `1`
  - untracked paths: `298`
- Representative tracked noise outside this ticket/replay basis:
  - `.gitignore`
  - `docs/Current/TASK-M11-10_SPLIT_PLAN_AND_TEMPORARY_AUTHORITY_AMENDMENT.md` (deleted in worktree)
  - `src-rust/crates/api/src/providers/google.rs`
  - `src-rust/crates/core/src/effort.rs`
  - `src-rust/crates/core/src/lib.rs`
  - `src-rust/crates/core/src/remote_settings.rs`
  - `src-rust/crates/core/src/system_prompt.rs`
- Representative untracked noise:
  - `.codex/`
  - `docs/Current/HOSTED_OLLAMA_IMPLEMENTATION_PLAN.md`
  - `docs/Current/MPWO_WORK_ORDER_PACK_pre_M10_revision.md`
  - `docs/Current/RFC_PROVIDER_AWARE_WORKER_FABRIC_v3.1.md`
  - `docs/archive/provider_orchestrator/`
  - many `docs/archive/reports/*.md`
  - `src-rust/target/`
- Drift found:
  - no structural drift in the active authority path
  - substantial worktree noise means broad replay failures will need explicit attribution before they are treated as feature-lane regressions
- Blockers:
  - none for planning
  - later replay must treat `cargo fmt` / `cargo clippy` and any broad workspace failure carefully because the branch is not scope-clean

# 4. Authority and evidence reviewed

## Controlling authority

- `AGENTS.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md`

## Live contextual docs reviewed

- `docs/Current/D1_REVIEW_REPORT_20260413T233604Z.md`
- `docs/Current/M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md`
- `docs/Current/IMPLEMENTATION_PLAN_MPWO.md`
- `docs/Current/MPWO_WORK_ORDER_PACK_pre_M10_revision.md`
- `docs/Current/HOSTED_OLLAMA_IMPLEMENTATION_PLAN.md`
- `docs/Current/RFC_PROVIDER_AWARE_WORKER_FABRIC_v3.1.md`

## Accepted archive evidence reviewed for scope / acceptance reconstruction

- `docs/archive/reports/MILESTONE_M8_WRAPUP_CHECK_20260413T004403Z.md`
- `docs/archive/reports/TASK-M7-05_CLOSEOUT_REPORT_20260411T232128Z.md`
- `docs/archive/reports/TASK-M7-06_CLOSEOUT_REPORT_20260412T005128Z.md`
- `docs/archive/reports/TASK-M7-07_CLOSEOUT_REPORT_20260412T040059Z.md`
- `docs/archive/reports/TASK-M8-10_CLOSEOUT_REPORT_20260412T235709Z.md`
- `docs/archive/reports/TASK-M8-11_CLOSEOUT_REPORT_20260413T003451Z.md`
- `docs/archive/reports/TASK-M9-01_EXECUTION_REPORT_20260413T050804Z.md`
- `docs/archive/reports/TASK-M9-05_EXECUTION_REPORT_20260413T083402Z.md`
- `docs/archive/reports/TASK-M9-10_EXECUTION_REPORT_20260413T140203Z.md`
- `docs/archive/reports/TASK-M9-12_CLOSEOUT_REPORT_20260413T153943Z.md`
- `docs/archive/reports/TASK-M9-12_FINAL_EXECUTION_RERUN_REPORT_20260413T164651Z.md`
- `docs/archive/reports/TASK_CHILD_MAX_TOKENS_D1_INTERIM_CLOSEOUT_REPORT_20260413T163701Z.md`
- `docs/archive/reports/MPWO_REVISION_REPORT_20260414T003218Z.md`
- `docs/archive/reports/TASK-M11-08_AUTHORITY_RECONCILIATION_REPORT_20260415T002331Z.md`
- `docs/archive/reports/TASK-M11-10B_SPLIT_AUTHORITY_REPORT_20260415T031240Z.md`
- `docs/archive/reports/TASK-M11-08R_CLOSEOUT_REPORT_20260415T004401Z.md`
- `docs/archive/reports/TASK-M11-08B_CLOSEOUT_REPORT_20260415T011952Z.md`
- `docs/archive/reports/TASK-M11-09_CLOSEOUT_REPORT_20260415T014220Z.md`
- `docs/archive/reports/TASK-M11-10A_CLOSEOUT_REPORT_20260415T024141Z.md`
- `docs/archive/reports/TASK-M11-10B1_CLOSEOUT_REPORT_20260415T034453Z.md`
- `docs/archive/reports/TASK-M11-10B2_CLOSEOUT_REPORT_20260415T041344Z.md`
- `docs/archive/reports/TASK-M11-11_CLOSEOUT_REPORT_20260415T053856Z.md`
- `docs/archive/reports/TASK-M11-12_GATE_CLARIFICATION_REPORT_20260415T060015Z.md`
- `docs/archive/reports/TASK-M11-12_CLOSEOUT_REPORT_20260415T065244Z.md`
- `docs/archive/reports/POST_M11_REPO_ASSESSMENT_20260415T071321Z.md`
- `docs/archive/reports/POST_M11_01_CLOSEOUT_REPORT_20260415T074353Z.md`
- `docs/archive/reports/POST_M11_M12_AUTH_RECON_VERIFICATION_REPORT_20260416T062420Z.md`
- `docs/archive/reports/M12_RECUT_PREFLIGHT_REPORT_20260416T064259Z.md`
- `docs/archive/reports/M12_CLOSEOUT_REPORT_20260416T065308Z.md`
- `docs/archive/reports/POST_M11_ARCH_01A_VERIFICATION_REPORT_20260416T080436Z.md`
- `docs/archive/reports/POST_M11_ARCH_01A_CLOSEOUT_REPORT_20260416T081839Z.md`
- `docs/archive/reports/POST_M11_ARCH_02_PREFLIGHT_REPORT_20260416T084418Z.md`
- `docs/archive/reports/POST_M11_ARCH_03_PREFLIGHT_REPORT_20260416T091621Z.md`

## Live source/test surfaces inspected

- `src-rust/Cargo.toml`
- `src-rust/crates/tools/src/lib.rs`
- `src-rust/crates/tools/src/team_tool.rs`
- `src-rust/crates/cli/src/main.rs`
- `src-rust/crates/query/src/lib.rs`
- `src-rust/crates/query/src/agent_tool.rs`
- `src-rust/crates/query/src/agent_tool_tests.rs`
- `src-rust/crates/query/src/provider_resolution.rs`
- `src-rust/crates/query/src/provider_resolution_tests.rs`
- `src-rust/crates/query/src/session_budget.rs`
- `src-rust/crates/query/src/health_cache.rs`
- `src-rust/crates/api/src/provider_types.rs`
- `src-rust/crates/core/src/lib.rs`

## Verified commands / probe shapes for this planning pass

- branch / `HEAD` / `git status`
- `git log --oneline`
- repo/report archive milestone search with `rg`
- direct source inspection with `sed` / `nl`
- live structural seam probes with `rg`
- no replay execution beyond read-only inspection and command mapping

# 5. Historical milestone inventory from M7 onward

## D1 ladder

- `TASK-M7-01` through `TASK-M7-04`
  - Type: `runtime/code`
  - Accepted intent: extract provider-resolution and provider-materialization seam from `query/lib.rs` into a reusable shared seam
  - Live runtime implication on current `HEAD`: `yes`
  - Current representation: `provider_resolution.rs`, root registry-backed path in `query/lib.rs`, hosted-Ollama normalization/materialization logic
- `TASK-M7-05`
  - Type: `tests/validation`
  - Accepted intent: `P1-P12` precedence-matrix coverage for `resolve_provider_identity()`
  - Live implication: `yes`
- `TASK-M7-06`
  - Type: `tests/validation`
  - Accepted intent: `materialize_provider()` auth/api-base/no-credentials coverage
  - Live implication: `yes`
- `TASK-M7-07`
  - Type: `tests/validation`
  - Accepted intent: milestone-wide workspace validation gate for the seam extraction
  - Live implication: `partially`
  - Current issue: exact historical `fmt` / `clippy` meaning is now contaminated by unrelated worktree noise and later repo debt

- `TASK-M8-01` through `TASK-M8-09`
  - Type: `runtime/code`
  - Accepted intent: carry provider/model registries and provider/model overrides through `ToolContext`, `AgentTool`, `AgentRunParams`, and TeamCreate
  - Live implication: `yes`
  - Current representation: `ToolContext.provider_registry`, `ToolContext.model_registry`, `AgentSpec.provider/model`, `AgentRunParams.provider_override/model_override`, TeamCreate wiring
- `TASK-M8-10`
  - Type: `tests/validation`
  - Accepted intent: local provider-seam coverage for worker paths
  - Live implication: `yes`
- `TASK-M8-11`
  - Type: `tests/validation`
  - Accepted intent: milestone-wide workspace validation gate after worker propagation landed
  - Live implication: `partially`, same caution as `M7-07`

- `TASK-M9-01`
  - Type: `tests/validation`
  - Accepted intent: prove all `P1-P12` rows are covered and passing
  - Live implication: `yes`
- `TASK-M9-02`
  - Type: `tests/validation`
  - Accepted intent: materialization coverage expansion
  - Live implication: `yes`
- `TASK-M9-03` through `TASK-M9-10`
  - Type: `tests/validation`
  - Accepted intent: prove explicit-provider routing, parent inheritance, conflict detection, missing-registry behavior, root legacy fallback behavior, root no-legacy-fallback behavior, TeamCreate mixed-provider dispatch, and no-key explicit-openai spawn behavior
  - Live implication: `yes`
- `TASK-M9-11`
  - Type: `runtime/code`
  - Accepted intent: remove remaining hardcoded Anthropic construction from agent-tool paths
  - Live implication: `yes`
- `TASK-M9-12`
  - Type: `tests/validation`
  - Accepted intent: D1 closeout gate with workspace regression commands plus conditional live smoke
  - Live implication: `yes`, but exact historical gate must now be replayed with current-branch caveats

## D2 / M11 authority ladder

- `M10_D2_IMPLEMENTATION_PLAN`
  - Type: `docs/authority`
  - Accepted role: planning-only; defines original D2 ladder
  - Live implication: `no runtime replay`
- Original unsplit / revised `M11-01` through `M11-07`
  - Type: `runtime/code`
  - Accepted role: D2 substrate
  - Live implication: `yes`
  - Current representation:
    - `TrustDomain`
    - `ModelEntry` / capability matching
    - `HealthCache`
    - `resolve_provider_with_fallback()`
    - `CostTracker`
    - `SessionBudget`
- Original revised `M11-08`
  - Type: `superseded/split/recast`
  - Accepted role now represented by: `08R` plus `08B`
  - Live implication: `yes`, but only through the split path
- Original revised `M11-09`
  - Type: `runtime/code`
  - Accepted role: child `max_tokens` override wiring
  - Live implication: `yes`
  - Accepted closeout is the split-path `TASK-M11-09`
- Original revised `M11-10`
  - Type: `superseded/split/recast`
  - Accepted role now represented by: `10A`, `10B1`, `10B2`
  - Live implication: `yes`, but only through the split path
- Original revised `M11-11`
  - Type: `runtime/code`
  - Accepted role: `QueryEvent` expansion / observability
  - Live implication: `yes`
- Original revised `M11-12`
  - Type: `tests/validation`
  - Accepted role: split-path D2 validation gate
  - Live implication: `yes`, but current replay must use the clarified accepted gate from `TASK-M11-12_GATE_CLARIFICATION_REPORT_20260415T060015Z.md`

## Accepted split-path D2 ladder

- `TASK-M11-08R`
  - Type: `runtime/code`
  - Accepted role: root `SessionBudget` wiring and root cancellation observation
- `TASK-M11-08B`
  - Type: `runtime/code`
  - Accepted role: inherited parent session-budget propagation through query-owned session-id seam
- `TASK-M11-09`
  - Type: `runtime/code`
  - Accepted role: child `max_tokens` override wiring
- `TASK-M11-10A`
  - Type: `runtime/code`
  - Accepted role: child `allow_fallback` wiring
- `TASK-M11-10B1`
  - Type: `runtime/code`
  - Accepted role: layered child-budget seam
- `TASK-M11-10B2`
  - Type: `runtime/code`
  - Accepted role: child/team `budget_usd` carriage
- `TASK-M11-11`
  - Type: `runtime/code`
  - Accepted role: `QueryEvent` / observability expansion
- `TASK-M11-12`
  - Type: `tests/validation`
  - Accepted role: split-path D2 validation gate

## Post-M11 outcomes

- `POST-M11-01`
  - Type: `docs/authority`
  - Accepted role: current-authority consolidation only
  - Not a replay target
- Later post-M11 cleanup commits between `560b54f` and `2def737`
  - Type: `repo hygiene / test organization`
  - Accepted role: cleanup and file-layout changes only
  - Not milestone-acceptance replay targets, but they changed where some current tests now live
- `POST-M11-M12-AUTH-RECON`
  - Type: `docs/authority`
  - Accepted role: authority correction only
  - Not a runtime replay target
- `M12`
  - Type: `docs/authority`
  - Accepted role: audit-satisfied closure; no implementation delta
  - Not a runtime replay target
- `POST-M11-ARCH-01A`
  - Type: `runtime/code`
  - Accepted role: query-owned session-scoped `HealthCache` reuse
  - Live implication: `yes`
- `POST-M11-ARCH-02`
  - Type: `deferred/not-implemented-by-design`
  - Accepted role: remain deferred
- `POST-M11-ARCH-03`
  - Type: `deferred/not-implemented-by-design`
  - Accepted role: remain deferred

# 6. Replayability classification matrix

| Historical item | Replayability | Why |
| --- | --- | --- |
| `M7-01`..`M7-04` | `INDIRECTLY-VERIFIABLE-ON-CURRENT-HEAD` | The original extraction commits are historical, but current `provider_resolution` tests plus root seam tests still exercise the extracted behavior and its hosted-Ollama invariants. |
| `M7-05` | `DIRECT-REPLAYABLE` | The current `provider_resolution` test module still contains the `P1-P12` tests and the filter command still works. |
| `M7-06` | `DIRECT-REPLAYABLE` | Current `provider_resolution` tests still contain the materialization coverage added in `M7-06`. |
| `M7-07` | `DIRECT-REPLAYABLE` | The exact historical workspace commands still exist, but on current `HEAD` `fmt` / `clippy` must be recorded as informational because unrelated worktree noise can contaminate results. |
| `M8-01`..`M8-09` | `INDIRECTLY-VERIFIABLE-ON-CURRENT-HEAD` | Current structural `rg` probes plus `agent_tool` / TeamCreate tests prove the wiring is still present, but the original landing steps are historical. |
| `M8-10` | `DIRECT-REPLAYABLE` | The local worker-path tests still exist under the modernized test layout and still run through the same `cargo test -p claurst-query -- agent_tool` filter. |
| `M8-11` | `DIRECT-REPLAYABLE` | The workspace validation commands still exist, but current replay must treat `fmt` / `clippy` as informational for the same reason as `M7-07`. |
| `M9-01` | `DIRECT-REPLAYABLE` | The exact `P1-P12` audit can still be rerun from the same `provider_resolution` test filter. |
| `M9-02` | `DIRECT-REPLAYABLE` | Current `provider_resolution` tests still cover auth-store / api-base / no-credentials materialization behavior. |
| `M9-03`..`M9-10` | `DIRECT-REPLAYABLE` | Current `agent_tool`, `provider_registry_none`, `provider_registry_some_resolution_failure`, and TeamCreate tests still exercise the accepted D1 validation claims directly. |
| `M9-11` | `INDIRECTLY-VERIFIABLE-ON-CURRENT-HEAD` | The accepted claim is structural: hardcoded Anthropic construction was removed from agent-tool paths. Current best replay is `rg` plus agent-tool/root tests, not a single dedicated regression test. |
| `M9-12` | `DIRECT-REPLAYABLE` | The historical broad regression commands still exist; the manual smoke remains conditional on credentials and network. |
| `M10` | `DOCS-ONLY-RECORD` | Planning-only milestone. There is no runtime replay to perform. |
| Original `M11-01` | `DIRECT-REPLAYABLE` | `TrustDomain` still has direct current tests in `claurst-api`. |
| Original `M11-02` / `M11-03` | `DIRECT-REPLAYABLE` | Current `provider_resolution` tests exercise model metadata and capability matching on live code. |
| Original `M11-04` | `DIRECT-REPLAYABLE` | `health_cache` tests remain live and targeted. |
| Original `M11-05` | `DIRECT-REPLAYABLE` | Fallback tests remain live in `provider_resolution` and still exercise same-domain / cross-domain / health-filtered behavior. |
| Original `M11-06` | `INDIRECTLY-VERIFIABLE-ON-CURRENT-HEAD` | Cost tracker behavior is directly testable in `claurst-core`, but the original M11 acceptance was a substrate claim that now matters mainly through downstream budget/event behavior. |
| Original `M11-07` | `DIRECT-REPLAYABLE` | `session_budget` tests remain live and targeted. |
| Original unsplit `M11-08` | `SUPERSEDED-BY-LATER-ACCEPTED-PATH` | It must not be replayed literally. The accepted runtime meaning is the split path `08R` + `08B`. |
| Accepted `08R` | `INDIRECTLY-VERIFIABLE-ON-CURRENT-HEAD` | Current best replay is structural root wiring probes plus `session_budget` / budget-event tests and package build. There is no dedicated current CLI regression test for `--budget-usd`. |
| Accepted `08B` | `DIRECT-REPLAYABLE` | Current `session_budget` and `agent_tool` tests directly exercise inherited parent budget reuse and child cancel-token behavior. |
| Accepted `09` | `DIRECT-REPLAYABLE` | Current `agent_tool` tests directly exercise child `max_tokens` override. |
| Accepted `10A` | `DIRECT-REPLAYABLE` | Current `agent_tool` and `provider_resolution` tests directly exercise fallback-enabled child resolution. |
| Accepted `10B1` | `DIRECT-REPLAYABLE` | Current `session_budget` and `agent_tool` tests directly exercise layered child-budget semantics. |
| Accepted `10B2` | `DIRECT-REPLAYABLE` | Current `agent_tool` tests plus structural TeamCreate probes directly exercise child/team `budget_usd` carriage. |
| Accepted `11` | `DIRECT-REPLAYABLE` | Current observability tests directly exercise `WorkerProviderResolved`, `WorkerBudgetExceeded`, and `SessionBudgetExceeded`. |
| Accepted `12` | `DIRECT-REPLAYABLE` | Current accepted gate is the clarified split-path build/api/query validation gate, not the obsolete literal pre-split wording. |
| `POST-M11-01` | `DOCS-ONLY-RECORD` | Authority consolidation only. |
| `POST-M11-M12-AUTH-RECON` | `DOCS-ONLY-RECORD` | Authority correction only. |
| `M12` | `DOCS-ONLY-RECORD` | Closed by audit; no implementation or runtime delta to rerun. |
| `POST-M11-ARCH-01A` | `DIRECT-REPLAYABLE` | Current `health_cache`, `provider_resolution`, `agent_tool`, and full `claurst-query` tests directly exercise the accepted session-scoped cache reuse behavior. |
| `POST-M11-ARCH-02` | `DEFERRED-NO-REPLAY` | Explicitly remain deferred by accepted preflight. Replay should record that disposition only. |
| `POST-M11-ARCH-03` | `DEFERRED-NO-REPLAY` | Explicitly remain deferred by accepted preflight. Replay should record that disposition only. |

# 7. Milestone-to-command mapping

| Milestone / ticket | Accepted claim being checked | Exact current command(s) | Expected pass condition | Verification type | Evidence to capture |
| --- | --- | --- | --- | --- | --- |
| `M7-01`..`M7-04` | Shared provider-resolution seam still owns identity/materialization behavior | `cd src-rust && cargo test -p claurst-query -- provider_resolution` and `cd src-rust && cargo test -p claurst-query -- provider_registry_none` and `cd src-rust && cargo test -p claurst-query -- provider_registry_some_resolution_failure` | all targeted tests pass | indirect | capture passing counts and note live tests covering `resolve_provider_identity()`, `materialize_provider()`, root legacy path, and root registry-backed no-legacy-fallback path |
| `M7-05` | `P1-P12` precedence matrix still passes | `cd src-rust && cargo test -p claurst-query -- provider_resolution` | command passes; output includes the precedence tests | direct | capture pass count and named rows `p1_...` through `p12_...` |
| `M7-06` | `materialize_provider()` auth/api-base coverage still passes | `cd src-rust && cargo test -p claurst-query -- provider_resolution` | command passes; output includes `materialize_provider_*` tests | direct | capture the three original `materialize_provider_*` names plus current added no-auth / api-base cases |
| `M7-07` | historical D1 seam workspace gate | `cd src-rust && cargo build --workspace`, `cd src-rust && cargo test --workspace`, `cd src-rust && cargo fmt --all -- --check`, `cd src-rust && cargo clippy --workspace --all-targets -- -D warnings` | build and workspace tests ideally pass; `fmt` / `clippy` results must be recorded even if unrelated noise fails them | direct | capture exit status for each command and explicitly tag `fmt` / `clippy` as informational on current dirty branch |
| `M8-01` / `M8-02` | registries are still carried on `ToolContext` and populated from CLI root setup | `rg -n "provider_registry|model_registry" src-rust/crates/tools/src/lib.rs src-rust/crates/cli/src/main.rs` | output shows fields in `ToolContext` and root initialization/wiring in CLI | indirect | capture matching lines from `tools/src/lib.rs` and `cli/src/main.rs` |
| `M8-03` / `M8-06` / `M8-07` / `M8-09` | provider/model and later child settings still flow through `AgentInput`, `AgentSpec`, and `AgentRunParams` | `rg -n "provider_override|model_override|max_tokens_override|allow_fallback|budget_usd" src-rust/crates/tools/src/team_tool.rs src-rust/crates/query/src/agent_tool.rs` | output shows fields and carriage points still live | indirect | capture matching lines from `team_tool.rs` and `agent_tool.rs` |
| `M8-04` / `M8-05` / `M8-08` | foreground/background/team-runner worker paths still route through the shared seam | `cd src-rust && cargo test -p claurst-query -- agent_tool` | command passes | indirect | capture passing subtests for explicit provider, parent inheritance, mixed-provider TeamCreate, and missing-registry error path |
| `M8-10` | worker provider-resolution tests still pass | `cd src-rust && cargo test -p claurst-query -- agent_tool` | command passes; worker seam tests all pass | direct | capture `agent_tool_errors_when_provider_registry_missing`, `agent_explicit_provider_routes_to_openai_provider`, `agent_parent_inherits_provider_openai_dispatch` |
| `M8-11` | historical worker-propagation workspace gate | same command set as `M7-07` | same interpretation as `M7-07` | direct | same evidence as `M7-07`, but report it as the M8 formal gate replay |
| `M9-01` | all precedence rows are still covered and passing | `cd src-rust && cargo test -p claurst-query -- provider_resolution` | command passes | direct | capture `P1-P12` names and counts |
| `M9-02` | expanded materialization coverage still passes | `cd src-rust && cargo test -p claurst-query -- provider_resolution` | command passes | direct | capture `materialize_provider_*` names and counts |
| `M9-03` | explicit provider routing still dispatches correctly | `cd src-rust && cargo test -p claurst-query -- agent_tool` | command passes | direct | capture `agent_explicit_provider_routes_to_openai_provider` |
| `M9-04` | parent provider inheritance still dispatches correctly | `cd src-rust && cargo test -p claurst-query -- agent_tool` | command passes | direct | capture `agent_parent_inherits_provider_openai_dispatch` |
| `M9-05` | provider/model conflict detection still rejects mismatches | `cd src-rust && cargo test -p claurst-query -- explicit_provider_conflicts` | command passes | direct | capture `p3_...` and `p5_...` |
| `M9-06` | missing worker registry still fails loudly | `cd src-rust && cargo test -p claurst-query -- agent_tool_errors_when_provider_registry_missing` | command passes | direct | capture the exact test name and its error-string expectation |
| `M9-07` | root missing registry still uses the legacy path | `cd src-rust && cargo test -p claurst-query -- provider_registry_none` | command passes | direct | capture `provider_registry_none_uses_legacy_anthropic_client_path` |
| `M9-08` | root registry-backed resolution failure still does not fall back to legacy Anthropic path | `cd src-rust && cargo test -p claurst-query -- provider_registry_some_resolution_failure` | command passes | direct | capture `provider_registry_some_resolution_failure_does_not_fallback_to_legacy_anthropic` |
| `M9-09` | TeamCreate mixed-provider dispatch still works per agent | `cd src-rust && cargo test -p claurst-query -- teamcreate_mixed_providers_per_agent_dispatch` | command passes | direct | capture the exact test name |
| `M9-10` | explicit OpenAI child spawn still works without real keys/network when registry is mocked | `cd src-rust && cargo test -p claurst-query -- agent_explicit_provider_routes_to_openai_provider` | command passes | direct | capture the exact test name and note it still uses the isolated auth helper / fake registry path |
| `M9-11` | hardcoded Anthropic construction stayed out of agent-tool runtime paths | `rg -n "AnthropicClient::new|provider_registry not available in ToolContext|resolve_provider_identity\\(|materialize_provider\\(|resolve_provider_with_fallback\\(" src-rust/crates/query/src/agent_tool.rs src-rust/crates/query/src/lib.rs` and `cd src-rust && cargo test -p claurst-query -- agent_tool` | `rg` shows seam calls in `agent_tool.rs` rather than hardcoded construction; `agent_tool` tests pass | indirect | capture matching `rg` lines and note remaining `AnthropicClient::new` occurrences are in non-agent-tool / legacy-root code paths |
| `M9-12` | D1 final regression gate and conditional smoke | same broad commands as `M7-07`, plus the conditional smoke command in Section 8 | broad commands recorded; smoke either passes or is recorded as not run with reason | direct for automation, conditional for smoke | capture command exit codes, smoke stdout if run, or environment blocker note if not run |
| `M10` | planning-only milestone remains accepted context, not runtime work | no runtime command; record `docs/archive/reports/M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md` | report cited | docs-only | cite the report path only |
| Original `M11-01` | trust-domain classification still matches accepted intent | `cd src-rust && cargo test -p claurst-api -- trust_domain` | command passes | direct | capture trust-domain test count and names |
| Original `M11-02` / `M11-03` | model metadata and capability matching still support the D2 seam | `cd src-rust && cargo test -p claurst-query -- provider_resolution` | command passes | direct | capture capability / fallback-related test names present in the run |
| Original `M11-04` | `HealthCache` behavior still passes | `cd src-rust && cargo test -p claurst-query -- health_cache` | command passes | direct | capture count and `health_cache_*` names |
| Original `M11-05` | same-domain fallback logic still works and cross-domain fallback is still prohibited | `cd src-rust && cargo test -p claurst-query -- provider_resolution` | command passes; fallback tests pass | direct | capture `fallback_disabled_returns_suggestion_text`, `fallback_same_domain_returns_healthy_cloud_candidate`, `fallback_cross_domain_is_prohibited` |
| Original `M11-06` | cost tracking substrate still behaves correctly | `cd src-rust && cargo test -p claurst-core -- cost_tracker` | command passes | indirect | capture `test_cost_tracker`, `test_cost_tracker_cumulative`, `test_cost_tracker_initial_zero` |
| Original `M11-07` | `SessionBudget` substrate still behaves correctly | `cd src-rust && cargo test -p claurst-query -- session_budget` | command passes | direct | capture count and `session_budget_*` / child-scope test names |
| Accepted `08R` | root session budget exists and emits root budget-exceeded events | `cd src-rust && cargo test -p claurst-query -- session_budget_exceeded_event_emits_only_on_new_cancellation` and `cd src-rust && cargo build --workspace` | both commands pass | indirect | capture the event test name and build result; note remaining CLI-entry proof gap |
| Accepted `08B` | inherited parent session-budget accounting and child cancel-token behavior still work | `cd src-rust && cargo test -p claurst-query -- session_budget` and `cd src-rust && cargo test -p claurst-query -- agent_tool` | both commands pass | direct | capture `child_token_is_cancelled_with_root`, `child_session_budget_reuses_inherited_budget_when_child_limit_absent` |
| Accepted `09` | child `max_tokens` override still works | `cd src-rust && cargo test -p claurst-query -- agent_tool` | command passes | direct | capture `agent_tool_respects_max_tokens_override` |
| Accepted `10A` | child fallback wiring still works | `cd src-rust && cargo test -p claurst-query -- agent_tool` and `cd src-rust && cargo test -p claurst-query -- provider_resolution` | both commands pass | direct | capture `agent_tool_allow_fallback_uses_same_domain_provider` and the fallback tests |
| Accepted `10B1` | layered child-budget seam still preserves parent accounting while adding child-local cap | `cd src-rust && cargo test -p claurst-query -- session_budget` and `cd src-rust && cargo test -p claurst-query -- agent_tool` | both commands pass | direct | capture `child_scope_records_against_local_and_parent_budget`, `descendant_scope_chains_all_active_budget_caps`, `child_session_budget_wraps_parent_when_child_limit_present` |
| Accepted `10B2` | child/team `budget_usd` carriage still reaches runtime | `rg -n "provider_override|model_override|max_tokens_override|allow_fallback|budget_usd" src-rust/crates/tools/src/team_tool.rs src-rust/crates/query/src/agent_tool.rs` and `cd src-rust && cargo test -p claurst-query -- teamcreate_mixed_providers_per_agent_dispatch` and `cd src-rust && cargo test -p claurst-query -- agent_tool` | `rg` shows fields/wiring; tests pass | direct | capture relevant `rg` lines plus `worker_budget_exceeded_event_reports_child_limit` and TeamCreate test output |
| Accepted `11` | query events and TeamCreate observability still match accepted semantics | `rg -n "WorkerProviderResolved|WorkerBudgetExceeded|SessionBudgetExceeded|teamcreate_observability_is_sanitized_and_emitted|session_budget_exceeded_event_emits_only_on_new_cancellation" src-rust/crates/query/src/lib.rs src-rust/crates/query/src/agent_tool.rs` and `cd src-rust && cargo test -p claurst-query -- teamcreate_observability_is_sanitized_and_emitted` and `cd src-rust && cargo test -p claurst-query -- session_budget_exceeded_event_emits_only_on_new_cancellation` | `rg` shows live variants/tests; tests pass | direct | capture `rg` lines and the two exact test names |
| Accepted `12` | split-path D2 validation gate still passes on current `HEAD` | `cd src-rust && cargo build --workspace`, `cd src-rust && cargo test -p claurst-api`, `cd src-rust && cargo test -p claurst-query` | all three commands pass | direct | capture exit codes and package pass counts; record `fmt` / `clippy` separately as informational only |
| `POST-M11-01` | post-M11 authority consolidation remains accepted history only | no runtime command; cite `docs/archive/reports/POST_M11_01_CLOSEOUT_REPORT_20260415T074353Z.md` | report cited | docs-only | cite report path only |
| `POST-M11-M12-AUTH-RECON` and `M12` | M12 remains audit-only and closed without implementation replay | no runtime command; cite `docs/archive/reports/POST_M11_M12_AUTH_RECON_VERIFICATION_REPORT_20260416T062420Z.md`, `docs/archive/reports/M12_RECUT_PREFLIGHT_REPORT_20260416T064259Z.md`, and `docs/archive/reports/M12_CLOSEOUT_REPORT_20260416T065308Z.md` | reports cited | docs-only | cite report paths and the `M12-SATISFIED-BY-AUDIT` / `READY-TO-CLOSE-NO-COMMIT` verdicts |
| `POST-M11-ARCH-01A` | session-scoped health-cache reuse still works and remains query-owned | `cd src-rust && cargo test -p claurst-query -- health_cache`, `cd src-rust && cargo test -p claurst-query -- provider_resolution`, `cd src-rust && cargo test -p claurst-query -- agent_tool`, `cd src-rust && cargo test -p claurst-query` | all commands pass | direct | capture counts and the tests `fallback_same_session_reuses_registered_health_cache`, `fallback_session_scopes_do_not_share_cached_health`, `child_and_team_fallback_share_session_health_cache` |
| `POST-M11-ARCH-02` | remain deferred | no runtime command; cite `docs/archive/reports/POST_M11_ARCH_02_PREFLIGHT_REPORT_20260416T084418Z.md` | report cited | deferred | record `REMAIN-DEFERRED` only |
| `POST-M11-ARCH-03` | remain deferred | no runtime command; cite `docs/archive/reports/POST_M11_ARCH_03_PREFLIGHT_REPORT_20260416T091621Z.md` | report cited | deferred | record `REMAIN-DEFERRED` only |

# 8. Proposed ordered replay sequence

## Recommendation

Run the later replay as two passes.

Why split is better on this branch:

- Pass A gives direct attribution per historical seam before any broad workspace gate can be polluted by today’s dirty worktree.
- Pass B still provides broad current-branch confidence, but its failures can be interpreted in light of Pass A instead of collapsing into a generic workspace failure.
- The live OpenAI smoke is conditional and should stay at the end, after deterministic local evidence is already captured.

## Pass A: targeted seam replay and structural probes

1. `git branch --show-current`
2. `git rev-parse HEAD`
3. `git status --short --branch`
4. `rg -n "provider_registry|model_registry" src-rust/crates/tools/src/lib.rs src-rust/crates/cli/src/main.rs`
5. `rg -n "provider_override|model_override|max_tokens_override|allow_fallback|budget_usd" src-rust/crates/tools/src/team_tool.rs src-rust/crates/query/src/agent_tool.rs`
6. `rg -n "AnthropicClient::new|provider_registry not available in ToolContext|resolve_provider_identity\\(|materialize_provider\\(|resolve_provider_with_fallback\\(" src-rust/crates/query/src/agent_tool.rs src-rust/crates/query/src/lib.rs`
7. `rg -n "WorkerProviderResolved|WorkerBudgetExceeded|SessionBudgetExceeded|teamcreate_observability_is_sanitized_and_emitted|session_budget_exceeded_event_emits_only_on_new_cancellation" src-rust/crates/query/src/lib.rs src-rust/crates/query/src/agent_tool.rs`
8. `cd src-rust && cargo test -p claurst-query -- provider_resolution`
9. `cd src-rust && cargo test -p claurst-query -- explicit_provider_conflicts`
10. `cd src-rust && cargo test -p claurst-query -- agent_tool`
11. `cd src-rust && cargo test -p claurst-query -- provider_registry_none`
12. `cd src-rust && cargo test -p claurst-query -- provider_registry_some_resolution_failure`
13. `cd src-rust && cargo test -p claurst-query -- teamcreate_observability_is_sanitized_and_emitted`
14. `cd src-rust && cargo test -p claurst-query -- session_budget_exceeded_event_emits_only_on_new_cancellation`
15. `cd src-rust && cargo test -p claurst-api -- trust_domain`
16. `cd src-rust && cargo test -p claurst-core -- cost_tracker`
17. `cd src-rust && cargo test -p claurst-query -- health_cache`
18. `cd src-rust && cargo test -p claurst-query -- session_budget`

## Pass B: broad current-branch regression and historical-gate probes

19. `cd src-rust && cargo build --workspace`
20. `cd src-rust && cargo test -p claurst-api`
21. `cd src-rust && cargo test -p claurst-query`
22. `cd src-rust && cargo test --workspace`
23. `cd src-rust && cargo fmt --all -- --check`
24. `cd src-rust && cargo clippy --workspace --all-targets -- -D warnings`

## Pass B conditional smoke

Run only if a valid `OPENAI_API_KEY` is available and the environment has outbound network access:

25. `cd src-rust && cargo run -q -p claurst -- --provider openai --model gpt-4o-mini --max-tokens 1024 --verbose --allowed-tools Agent --max-turns 4 -p "You are running a smoke test. Do not answer directly. Your first and only tool call must be Agent. Spawn exactly one child agent with description 'smoke test', provider 'openai', and model 'gpt-4o-mini'. Use the child prompt 'Reply with exactly CHILD_OK and nothing else.' Wait for it to finish, then reply with exactly PARENT_OK: CHILD_OK."`

Expected smoke pass text:

- `PARENT_OK: CHILD_OK`

If the smoke cannot be run:

- record `NOT RUN` with the exact reason
- cite `docs/archive/reports/TASK_CHILD_MAX_TOKENS_D1_INTERIM_CLOSEOUT_REPORT_20260413T163701Z.md` and `docs/archive/reports/TASK-M9-12_FINAL_EXECUTION_RERUN_REPORT_20260413T164651Z.md`
- do not treat missing credentials or blocked network as a runtime regression

## Record-only items to include at the end of the replay report

- `M10` planning-only milestone recorded from `docs/archive/reports/M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md`
- `POST-M11-01` authority consolidation recorded from `docs/archive/reports/POST_M11_01_CLOSEOUT_REPORT_20260415T074353Z.md`
- `POST-M11-M12-AUTH-RECON`, `M12_RECUT_PREFLIGHT`, and `M12` closeout recorded as audit-only acceptance with no runtime replay
- `POST-M11-ARCH-02` and `POST-M11-ARCH-03` recorded as `REMAIN-DEFERRED`

# 9. Replay gaps / limitations

- The branch is not worktree-clean. Any broad replay failure in `cargo build`, `cargo test --workspace`, `cargo fmt`, or `cargo clippy` must be checked against the unrelated tracked modifications before it is attributed to the provider-resolution lane.
- The historical D1 workspace gates (`M7-07`, `M8-11`, `M9-12`) can still be rerun literally, but their current meaning is weaker than it was on clean accepted D1 commits because the branch has moved forward and is dirty.
- The original unsplit `M11-08` and original unsplit `M11-10` must not be replayed literally. Current confidence must be reconstructed through the accepted split path (`08R`, `08B`, `10A`, `10B1`, `10B2`) and the later gate clarification.
- There is no dedicated current CLI integration test for root `--budget-usd`. `08R` therefore remains only indirectly verifiable through:
  - live wiring probes
  - `session_budget` tests
  - `SessionBudgetExceeded` event test
  - accepted closeout evidence
- `M12` cannot produce new runtime proof on current `HEAD` because it closed as an audit decision. The replay can only confirm that the accepted audit basis still stands.
- `POST-M11-ARCH-02` and `POST-M11-ARCH-03` are intentionally deferred. Replay must record their final disposition, not invent runtime verification steps for them.
- The current test layout is not identical to historical ticket-era file placement. Some historical inline tests now live in `agent_tool_tests.rs` and `provider_resolution_tests.rs`. The replay must follow current file reality, not obsolete path assumptions.
- Live smoke remains environment-dependent. Lack of credentials or network access leaves a real replay gap for live external-provider proof, but not for local seam coverage.

# 10. Final recommendation

`READY-FOR-SPLIT-REPLAY`

Rationale:

- A split replay gives the highest confidence with the lowest ambiguity on this dirty branch.
- Pass A yields milestone-attributable evidence for the M7/M8/M9 and split-M11 seams.
- Pass B reconstructs broad current-branch regression confidence and the old workspace gates without pretending those gates are still perfectly isolated from unrelated worktree drift.
- M12 and the deferred architecture items are correctly handled as record-only outcomes, not fake runtime reenactments.

# 11. Exact next-step recommendation for the actual replay pass

1. Run Pass A exactly as listed above and stop immediately if any targeted seam test fails.
2. If Pass A passes, run Pass B and record `cargo build`, package tests, and `cargo test --workspace` as the broad confidence layer.
3. Record `cargo fmt --all -- --check` and `cargo clippy --workspace --all-targets -- -D warnings` as historical-gate probes, but do not treat them as automatic feature-lane failures unless the failing paths are actually within the provider-resolution lane.
4. Attempt the live smoke only if the environment supports it; otherwise record `NOT RUN` with exact credential/network reason and cite the archived successful smoke evidence.
5. End the replay report with explicit record-only entries for:
   - `M10`
   - `POST-M11-01`
   - `POST-M11-M12-AUTH-RECON`
   - `M12`
   - `POST-M11-ARCH-02`
   - `POST-M11-ARCH-03`
6. If any broad command fails only because of unrelated dirty files, mark that as a replay limitation, not as a provider-resolution milestone regression.
