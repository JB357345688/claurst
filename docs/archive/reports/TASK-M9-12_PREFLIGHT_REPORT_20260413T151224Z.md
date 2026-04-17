# TASK-M9-12 Preflight Report

## Ticket
`TASK-M9-12 — Full regression + D1 completion declaration`

## Timestamp UTC
`2026-04-13T15:12:24Z`

## Branch
Expected: `feature/provider-resolution-seam`

Observed: `feature/provider-resolution-seam`

## Verdict
`READY-WITH-NOTES`

Hosted Ollama compatibility baseline preserved.

## Repo State Summary
- `git branch --show-current` -> `feature/provider-resolution-seam`
- `git status --short --branch` -> branch matches expectation; no tracked modified files; no staged tracked files; substantial unrelated untracked workspace/report noise remains, including `docs/Current/`, `docs/archive/reports/`, and `src-rust/target/`
- `git diff --name-only` -> no output
- `git diff --cached --name-only` -> no output
- `git log --oneline --decorate -n 20` -> `HEAD` is `af97a87 (feature/provider-resolution-seam) TASK-M9-11 remove hardcoded anthropic construction from agent tool paths`
- Most recent source-changing chain relevant to M9 readiness is present in history: `af97a87` (`M9-11`), `5e77652` (`M9-09`), `63a8485` (`M9-08`), `dfc4be4` (`M9-07`), `2f1f169` (`M9-04`), `c28ef22` (`M9-03`), `662b29a` (`M9-02`), plus accepted baseline `b5b6dd4` (`M8-11`)
- Review-basis note: the tracked worktree is clean for source files, but the repository contains broad untracked report/build artifacts that should remain outside any later closure-review basis

## Authority Reviewed
- `AGENTS.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md`
- MPWO section `2A. Standing Non-Regression Invariant — Hosted Ollama Compatibility`
- MPWO section `TASK-M9-12 — Full regression + D1 completion declaration`

## Dependency Baseline Confirmed
- `M8-11` accepted baseline is present in branch history as `b5b6dd4 TASK-M8-11 reconcile M8 workspace validation and formatting`
- `M9-01` execution artifact exists and closes as `PASS / COMPLETE WITHOUT SOURCE CHANGE`
- `M9-02` commit verification artifact exists and closes as `VERIFIED`
- `M9-03` commit verification artifact exists and closes as `VERIFIED`
- `M9-04` commit verification artifact exists and closes as `VERIFIED`
- `M9-05` execution artifact exists and closes as `PASS / COMPLETE WITHOUT SOURCE CHANGE`
- `M9-06` execution artifact exists and closes as `PASS / COMPLETE WITHOUT SOURCE CHANGE`
- `M9-07` commit verification artifact exists and closes as `VERIFIED`
- `M9-08` commit verification artifact exists and closes as `VERIFIED`
- `M9-09` commit verification artifact exists and closes as `VERIFIED`
- `M9-10` execution artifact exists and closes as `PASS / COMPLETE WITHOUT SOURCE CHANGE`
- `M9-11` commit verification artifact exists and closes as `VERIFIED`
- Current branch reality is compatible with starting `M9-12` after accepted `M8-11`; no read-only evidence suggests `M9-01` through `M9-11` need reopening
- Hosted Ollama remains a preserved background invariant only; no preflight evidence requires reopening that baseline

## Exact M9-12 Contract
- Objective: run full validation suite and declare D1 complete
- Files: entire workspace
- Required later execution order:
  - `cd src-rust && cargo fmt --all -- --check`
  - `cd src-rust && cargo build --workspace`
  - `cd src-rust && cargo test --workspace`
  - `cd src-rust && cargo clippy --workspace --all-targets`
  - manual smoke: run Claurst with `--provider openai`, spawn an agent, confirm it uses OpenAI, if possible in the test environment
  - document: `D1 is complete. Provider-resolution seam is landed. Workers inherit parent providers.`
- Do not start D2 work
- Validation target: all commands pass and D1 is declared shippable
- Dependency: `M9-01` through `M9-11`

## Verified Files / Symbols / Commands
- Verified files:
  - `AGENTS.md`
  - `docs/Current/MPWO_WORK_ORDER_PACK.md`
  - `src-rust/Cargo.toml`
  - `src-rust/Cargo.lock`
  - `src-rust/crates/cli/Cargo.toml`
  - `src-rust/crates/query/Cargo.toml`
  - `src-rust/crates/cli/src/main.rs`
  - `src-rust/crates/query/src/lib.rs`
  - `src-rust/crates/query/src/agent_tool.rs`
  - recent M9 report artifacts under `docs/archive/reports/`
- Verified workspace member manifests:
  - `src-rust/crates/acp/Cargo.toml`
  - `src-rust/crates/api/Cargo.toml`
  - `src-rust/crates/bridge/Cargo.toml`
  - `src-rust/crates/buddy/Cargo.toml`
  - `src-rust/crates/cli/Cargo.toml`
  - `src-rust/crates/commands/Cargo.toml`
  - `src-rust/crates/core/Cargo.toml`
  - `src-rust/crates/mcp/Cargo.toml`
  - `src-rust/crates/plugins/Cargo.toml`
  - `src-rust/crates/query/Cargo.toml`
  - `src-rust/crates/tools/Cargo.toml`
  - `src-rust/crates/tui/Cargo.toml`
- Verified symbols / runtime surfaces:
  - CLI `provider` flag in `src-rust/crates/cli/src/main.rs`
  - `build_tools_with_mcp(...)` pushes `claurst_query::AgentTool`
  - `claurst_query::init_team_swarm_runner()` is still initialized before tool execution
  - root query loop still dispatches through `resolve_provider_identity(...)` and `materialize_provider(...)`
  - `AgentTool` still accepts optional `provider`
  - `AgentTool` still inherits parent provider when no explicit provider is supplied
  - `AgentTool` still requires `provider_registry` and materializes a registry-backed provider target
  - committed tests proving explicit OpenAI routing and parent-provider inheritance remain present in `src-rust/crates/query/src/agent_tool.rs`
- Verified commands:
  - `git branch --show-current`
  - `git status --short --branch`
  - `git diff --name-only`
  - `git diff --cached --name-only`
  - `git log --oneline --decorate -n 20`
  - Read-only filesystem inspection commands only; the `M9-12` validation commands were not run in this session

## Workspace Validation Readiness Audit
| Command | Status | Basis | Known Caveat |
|---|---|---|---|
| `cd src-rust && cargo fmt --all -- --check` | `PLAUSIBLE` | `src-rust/Cargo.toml` exists as the Rust workspace root, all member manifests listed in the workspace are present, and no tracked source diffs are pending before the formatting gate | Must be run from `src-rust`, not repo root; no pinned `rust-toolchain*` file was found, so `rustfmt` availability depends on the host toolchain |
| `cd src-rust && cargo build --workspace` | `PLAUSIBLE` | Workspace manifest, member manifests, and `Cargo.lock` are present; recent M9 ticket validations compiled nearby targets successfully and current tracked worktree is clean | Untracked `src-rust/target/` exists, but that is normal build noise rather than structural drift |
| `cd src-rust && cargo test --workspace` | `PLAUSIBLE` | Recent M9 execution / verification artifacts show repeated passing cargo test runs on affected query/agent surfaces; no missing workspace members or manifest drift is visible | Recurrent non-blocking warning was reported across recent M9 runs: unused import `Role` in `src-rust/crates/query/src/compact.rs`; transient cargo file-lock waits were also observed but resolved without intervention |
| `cd src-rust && cargo clippy --workspace --all-targets` | `RISK` | Command shape matches the current workspace layout and no obvious manifest drift blocks it; no read-only evidence suggests `clippy` would target missing members | `clippy` component presence cannot be proven without execution, no pinned `rust-toolchain*` file was found, and `--all-targets` is broader than the targeted validations recorded in recent M9 reports |

## Manual Smoke Test Feasibility
- CLI/runtime path for `--provider openai` appears present:
  - yes; `src-rust/crates/cli/src/main.rs` still defines the `--provider` flag and wires provider selection into runtime config
- Agent spawn appears reachable from current CLI/runtime surfaces:
  - yes; `build_tools_with_mcp(...)` adds `AgentTool`, `init_team_swarm_runner()` is called during CLI startup, and `AgentTool` still resolves explicit or inherited providers through `provider_registry`
- Evidence that the OpenAI path is specifically still wired:
  - root query dispatch still uses the shared provider-resolution seam
  - committed tests remain present for `agent_explicit_provider_routes_to_openai_provider` and `agent_parent_inherits_provider_openai_dispatch`
- Feasibility classification:
  - conditionally feasible
- Visible limitations before execution:
  - requires usable OpenAI credentials and outbound network access at runtime
  - this preflight session did not verify credential presence
  - current sandbox/tool environment is network-restricted, so the smoke step is not provably runnable here without additional runtime access
  - model-driven tool use may need an explicit prompt that clearly requests sub-agent delegation to make the spawn step deterministic
- Conclusion:
  - the repo/runtime surface needed for the smoke test exists
  - the work-order phrasing `if possible in the test environment` is materially relevant

## Likely Execution Shape
- `M9-12` is ready to run as a validation-only ticket, not a feature-implementation ticket
- Cargo commands should be executed exactly from `src-rust/`; `/home/jordi/claurst` is not the Rust workspace root
- The likely order in the work order is already correct and should remain explicit in the later execution report
- Manual smoke is best treated as conditionally feasible:
  - repo/runtime wiring exists
  - environment prerequisites may block it
- If any validation step fails, the follow-on work should be a narrow ticket-local corrective patch only; no D2 scope should be entered
- If the manual smoke step cannot be performed because credentials or egress are unavailable, that limitation should be documented explicitly against the work-order clause `if possible in the test environment`

## Drift Found
- Minor layout drift only:
  - Rust workspace root is `src-rust/`, not repository root
  - broad untracked report/build artifacts are present in the repo state
  - no `rust-toolchain.toml`, `rust-toolchain`, `.rustfmt.toml`, or `clippy.toml` was found at `src-rust/`
- No structural drift found:
  - no missing workspace manifest
  - no missing member manifests
  - no command-shape mismatch versus current workspace layout
  - no missing obvious CLI/provider surface for the later manual smoke step

## Blockers
- None on read-only preflight evidence for the workspace validation commands
- Conditional runtime limitation only:
  - the manual OpenAI smoke step depends on credentials and network egress that were not verified in this session
  - this is a note, not a structural blocker to starting `M9-12` as written

## Notes
- Recent M9 reports repeatedly call out one pre-existing non-blocking warning in `src-rust/crates/query/src/compact.rs`; this should be expected noise unless the full-workspace clippy pass escalates it
- Recent reports also note occasional transient cargo file-lock waits that resolved without intervention
- `docs/Current/MPWO_WORK_ORDER_PACK.md` is present and used as controlling ticket authority in this session even though `docs/Current/` appears in untracked repo-state noise
- No source file was edited in this preflight session
- No tests were added or modified
- No `M9-12` validation command was executed in this session
- No D2 / M10 / M11 / M12 work was started
