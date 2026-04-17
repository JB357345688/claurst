# Claurst — Functional Specification & Capability Map

> **Version:** 0.0.8 | **Date:** 2026-04-07 | **Status:** Read-only survey artifact
>
> This document is the primary context layer for AI agents performing further
> development on the Claurst codebase. It maps the Rust implementation against
> the behavioral specification, identifies architectural boundaries, and provides
> precise injection points for new features.

---

## Table of Contents

1. [Codebase Topology](#1-codebase-topology)
2. [Data Flow & Execution Model](#2-data-flow--execution-model)
3. [Tool & Capability Registry](#3-tool--capability-registry)
4. [Subsystem Deep-Dives](#4-subsystem-deep-dives)
   - 4.1 [Agent Orchestration (Coordinator Mode)](#41-agent-orchestration-coordinator-mode)
   - 4.2 [KAIROS (Always-On)](#42-kairos-always-on)
   - 4.3 [ULTRAPLAN](#43-ultraplan)
   - 4.4 [autoDream (Memory Consolidation)](#44-autodream-memory-consolidation)
   - 4.5 [BUDDY (Tamagotchi)](#45-buddy-tamagotchi)
   - 4.6 [Multi-Provider System](#46-multi-provider-system)
   - 4.7 [Speech Modes](#47-speech-modes)
5. [State & Memory Management](#5-state--memory-management)
6. [AI Integration Guide](#6-ai-integration-guide)
7. [Delta Analysis: Spec vs Implementation](#7-delta-analysis-spec-vs-implementation)

---

## 1. Codebase Topology

**Total Rust source:** ~108,800 lines across 130+ `.rs` files in 12 crates.

### 1.1 Workspace Dependency Graph

```
claurst (binary)                          [crates/cli/src/main.rs]
├── claurst-tui                           [crates/tui/]  — 50+ source files, largest crate
│   Uses: ratatui, crossterm, claurst-core, claurst-query, claurst-tools
├── claurst-commands                      [crates/commands/]  — 85+ slash commands
│   Uses: claurst-core, claurst-mcp, claurst-plugins
├── claurst-query                         [crates/query/]  — Core agentic loop
│   ├── claurst-api                       [crates/api/]  — Provider-agnostic API
│   ├── claurst-tools                     [crates/tools/]  — 40+ tool implementations
│   │   └── claurst-mcp                   [crates/mcp/]  — MCP client (JSON-RPC 2.0)
│   └── claurst-plugins                   [crates/plugins/]  — Plugin runtime
├── claurst-bridge                        [crates/bridge/]  — claude.ai remote bridge
├── claurst-buddy                         [crates/buddy/]  — Tamagotchi companion
├── claurst-acp                           [crates/acp/]  — Agent Client Protocol (IDE)
└── claurst-core                          [crates/core/]  — Foundation types & config
```

### 1.2 Crate-by-Crate Breakdown

#### `claurst-core` (crates/core/) — Foundation
**Files:** ~40 source files | **Role:** Types, config, errors, constants, auth, permissions

| Module | Purpose |
|--------|---------|
| `lib.rs` | Error enum (`ClaudeError`), `Message`, `ContentBlock`, `Role` types |
| `config.rs` (in lib.rs) | `Config`, `Settings`, `PermissionMode`, `McpServerConfig`, `AgentDefinition` |
| `constants.rs` (in lib.rs) | `DEFAULT_MODEL`, `APP_VERSION`, `MAX_TURNS_DEFAULT`, tool name constants |
| `permissions.rs` (in lib.rs) | `PermissionHandler` trait, `AutoPermissionHandler`, `InteractivePermissionHandler` |
| `system_prompt.rs` | `SystemPromptSection`, `OutputStyle` enum, dynamic boundary caching |
| `session_storage.rs` | JSONL-based session persistence |
| `sqlite_storage.rs` | SQLite session storage (`SqliteSessionStore`) |
| `auth_store.rs` | `AuthStore` for provider API keys and OAuth tokens |
| `device_code.rs` | RFC 8628 device code flow for OAuth |
| `feature_flags.rs` | `FeatureFlagManager` via GrowthBook integration |
| `feature_gates.rs` | Env-var-based feature gates (`CLAURST_FEATURE_*`) |
| `claudemd.rs` | CLAUDE.md / AGENTS.md hierarchical loading |
| `token_budget.rs` | Token budget calculations |
| `voice.rs` | Voice availability checks, hold-to-talk recording, Whisper STT |
| `git_utils.rs` | Git status, diff, log utilities |
| `memdir.rs` | Memory directory (MEMORY.md) management |
| `keybindings.rs` | `KeybindingResolver`, chord/sequence parsing |
| `snapshot.rs` | `SnapshotManager` for per-session file undo |
| `file_history.rs` | Per-session file modification tracking |
| `migrations.rs` | Settings/model migration logic |
| `skill_discovery.rs` | Filesystem and git URL skill loading |
| `analytics.rs` | First-party event logging |
| `auto_mode.rs` | Auto/YOLO permission mode logic |
| `bash_classifier.rs` | Risk classification for shell commands |
| `context_collapse.rs` | Aggressive context-window reduction |
| `effort.rs` | `EffortLevel` enum (Low/Medium/High/Max) |
| `cloud_session.rs` | Cloud session API integration |
| `remote_session.rs` | Remote session sync |
| `ide.rs` | IDE environment detection |
| `update_check.rs` | Background version check against GitHub releases |
| `output_styles.rs` | Output style file loading |
| `mcp_templates.rs` | MCP resource prompt template rendering |
| `tips.rs` | Contextual tips system |
| `team_memory_sync.rs` | Team/swarm memory synchronization |

**Feature flags (36 total):**
```
Defaults: voice, ultraplan
Dev flags: ultrathink, history_picker, token_budget, message_actions,
  quick_search, away_summary, hook_prompts, kairos_brief, kairos_channels,
  lodestone, agent_triggers, agent_triggers_remote, extract_memories,
  verification_agent, builtin_explore_plan_agents, cached_microcompact,
  compaction_reminders, agent_memory_snapshot, teammem, bash_classifier,
  bridge_mode, mcp_rich_output, connector_text, unattended_retry, new_init,
  powershell_auto_mode, shot_stats, tree_sitter_bash, tree_sitter_bash_shadow,
  prompt_cache_break_detection, native_clipboard_image, ccr_auto_connect,
  ccr_mirror, ccr_remote_setup
```

#### `claurst-api` (crates/api/) — Provider-Agnostic API Client
**Files:** ~25 source files | **Role:** LLM provider abstraction, streaming, transforms

| Module | Purpose |
|--------|---------|
| `lib.rs` | `CreateMessageRequest`, `ThinkingConfig`, `SystemPrompt`, `ApiMessage` |
| `provider.rs` | `LlmProvider` trait, `ModelInfo` struct |
| `provider_types.rs` | `ProviderRequest`, `ProviderResponse`, `StreamEvent`, `ProviderCapabilities` |
| `provider_error.rs` | `ProviderError` type |
| `auth.rs` | `AuthProvider` trait, `LoginFlow` |
| `stream_parser.rs` | `StreamParser` trait, `SseStreamParser`, `JsonLinesStreamParser` |
| `transform.rs` | `MessageTransformer` trait |
| `registry.rs` | `ProviderRegistry` — central provider routing |
| `model_registry.rs` | `ModelRegistry`, `ModelEntry`, `effective_model_for_config()` |
| `error_handling.rs` | `is_context_overflow()`, `parse_error_response()`, `RetryConfig` |
| `cch.rs` | Client attestation header computation |
| `codex_adapter.rs` | Codex/OpenAI Responses API adapter |
| `providers/anthropic.rs` | `AnthropicProvider` — native Anthropic adapter |
| `providers/google.rs` | `GoogleProvider` — Gemini adapter |
| `providers/openai.rs` | `OpenAiProvider` — OpenAI Chat adapter |
| `providers/openai_compat.rs` | `OpenAiCompatProvider` — generic OpenAI-compatible adapter |
| `providers/openai_compat_providers.rs` | Factory functions: ollama, lm_studio, deepseek, groq, xai, openrouter, mistral |
| `providers/azure.rs` | `AzureProvider` — Azure OpenAI adapter |
| `providers/bedrock.rs` | `BedrockProvider` — AWS Bedrock adapter |
| `providers/copilot.rs` | `CopilotProvider` — GitHub Copilot adapter |
| `providers/cohere.rs` | `CohereProvider` — Cohere native adapter |
| `transformers/anthropic.rs` | `AnthropicTransformer` |
| `transformers/openai_chat.rs` | `OpenAiChatTransformer` |

**Core trait:**
```rust
#[async_trait]
pub trait LlmProvider: Send + Sync {
    fn id(&self) -> &ProviderId;
    fn name(&self) -> &str;
    async fn create_message(&self, request: ProviderRequest) -> Result<ProviderResponse, ProviderError>;
    async fn create_message_stream(&self, request: ProviderRequest)
        -> Result<Pin<Box<dyn Stream<Item = Result<StreamEvent, ProviderError>> + Send>>, ProviderError>;
    async fn list_models(&self) -> Result<Vec<ModelInfo>, ProviderError>;
    async fn health_check(&self) -> Result<ProviderStatus, ProviderError>;
    fn capabilities(&self) -> ProviderCapabilities;
}
```

#### `claurst-tools` (crates/tools/) — Tool Implementations
**Files:** 35+ source files | **Role:** All LLM-invokable tools

**Core trait:**
```rust
#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn permission_level(&self) -> PermissionLevel;
    fn input_schema(&self) -> Value;
    async fn execute(&self, input: Value, ctx: &ToolContext) -> ToolResult;
}
```

**`ToolContext` (shared across all invocations):**
```rust
pub struct ToolContext {
    pub working_dir: PathBuf,
    pub permission_mode: PermissionMode,
    pub permission_handler: Arc<dyn PermissionHandler>,
    pub cost_tracker: Arc<CostTracker>,
    pub session_id: String,
    pub file_history: Arc<Mutex<FileHistory>>,
    pub current_turn: Arc<AtomicUsize>,
    pub non_interactive: bool,
    pub mcp_manager: Option<Arc<McpManager>>,
    pub config: Config,
}
```

**Permission levels:**
```rust
pub enum PermissionLevel {
    None,       // Read-only, informational
    ReadOnly,   // Filesystem/network reads
    Write,      // Filesystem writes
    Execute,    // Shell command execution
    Dangerous,  // Bypass sandbox
    Forbidden,  // Unconditionally denied (e.g., `rm -rf /`)
}
```

#### `claurst-query` (crates/query/) — Agentic Loop
**Files:** 12 source files | **Role:** Orchestrates API calls, tool dispatch, compaction

| Module | Purpose |
|--------|---------|
| `lib.rs` | `run_query_loop()`, `QueryConfig`, `QueryOutcome`, `QueryEvent`, post-sampling hooks |
| `agent_tool.rs` | `AgentTool` — sub-agent spawning with worktree isolation and background mode |
| `coordinator.rs` | Coordinator mode: `AgentMode`, tool filtering, `ScratchpadGate`, system prompt |
| `auto_dream.rs` | `AutoDream` — 3-gate memory consolidation daemon |
| `compact.rs` | Auto-compact, micro-compact, context collapse, token warnings |
| `session_memory.rs` | `SessionMemoryExtractor` — post-session fact extraction to CLAUDE.md |
| `command_queue.rs` | `CommandQueue` — shared queue for injecting commands mid-loop |
| `context_analyzer.rs` | Context analysis utilities |
| `away_summary.rs` | Away-mode summary generation |
| `cron_scheduler.rs` | Cron-based scheduled agent triggers |
| `skill_prefetch.rs` | `SkillIndex`, `prefetch_skills()` — skill discovery and listing |

#### `claurst-tui` (crates/tui/) — Terminal UI
**Files:** 50+ source files | **Role:** Full TUI via ratatui + crossterm

| Module | Purpose |
|--------|---------|
| `app.rs` | `App` state struct, main event loop, 85+ slash command definitions |
| `render.rs` | Frame rendering dispatcher |
| `prompt_input.rs` | Multi-line input with vim mode, autocomplete, file drag-and-drop |
| `messages/mod.rs` | Message rendering (2,089 lines), streaming, tool results, thinking |
| `messages/markdown.rs` | Markdown-to-terminal rendering |
| `messages/markdown_enhanced.rs` | Enhanced markdown with syntax highlighting |
| `diff_viewer.rs` | Side-by-side diff viewer |
| `model_picker.rs` | Model selection dialog with effort level |
| `session_browser.rs` | Session history browser |
| `agents_view.rs` | Agent management UI |
| `mcp_view.rs` | MCP server browser |
| `overlays.rs` | Help, history search, rewind, global search overlays |
| `voice_capture.rs` | Voice recording overlay |
| `dialogs.rs` | Permission dialogs, MCP approval |
| `theme_colors.rs` | Theme color system |
| `kitty_image.rs` | Kitty graphics protocol for image display |
| `context_viz.rs` | Context window visualization |
| `tasks_overlay.rs` | Background tasks overlay |
| `plugin_views.rs` | Plugin management UI |
| `rustle.rs` | Rustle the Crab mascot ASCII art |

#### `claurst-commands` (crates/commands/) — Slash Commands
**Files:** 2 source files (lib.rs + named_commands.rs) | **85+ commands**

**Core trait:**
```rust
#[async_trait]
pub trait SlashCommand: Send + Sync {
    fn name(&self) -> &str;
    fn aliases(&self) -> Vec<&str>;
    fn description(&self) -> &str;
    fn help(&self) -> &str;
    fn hidden(&self) -> bool;
    async fn execute(&self, args: &str, ctx: &mut CommandContext) -> CommandResult;
}
```

**Full command list:**
`help`, `clear`, `compact`, `cost`, `exit`/`quit`, `model`, `config`/`settings`,
`color`, `version`, `resume`, `status`, `diff`/`changes`/`review`, `memory`,
`bug`, `usage`, `doctor`, `login`, `logout`, `refresh`, `caveman`, `rocky`,
`normal`, `init`, `hooks`, `mcp`, `permissions`, `plan`, `tasks`, `session`,
`thinking`, `export`, `skills`, `rewind`, `stats`, `files`, `rename`, `effort`,
`summary`, `commit`, `plugin`, `reload-plugins`, `theme`, `output-style`,
`keybindings`, `privacy`, `remote-control`, `remote-env`, `context`, `copy`,
`chrome`, `vim`, `voice`, `upgrade`, `release-notes`, `rate-limit-options`,
`statusline`, `security-review`, `terminal-setup`, `extra-usage`, `fast`,
`think-back`, `think-back-play`, `feedback`/`survey`, `color-set`, `share`,
`teleport`, `btw`, `ctx-viz`, `sandbox-toggle`, `heapdump`, `insights`,
`ultrareview`, `advisor`, `install-slack-app`, `undo`, `providers`, `connect`,
`agent`/`agents`, `search`, `fork`

#### `claurst-mcp` (crates/mcp/) — MCP Client
**Files:** 4 source files | **Role:** JSON-RPC 2.0 client for tool/resource servers

- Stdio and HTTP/SSE transports
- `McpManager` — connection lifecycle, tool discovery, tool execution
- `McpConnectionManager` — exponential-backoff reconnection
- OAuth support for authenticated MCP servers
- Environment variable expansion in server configs

#### `claurst-plugins` (crates/plugins/) — Plugin Runtime
**Files:** 7 source files | **Role:** Discovery, manifest, hooks, marketplace

- `PluginManifest` — TOML manifest parsing
- `HookRegistry` — pre/post tool-use hooks
- `PluginRegistry` — global plugin state
- Capability enforcement (`check_plugin_capability`)
- Marketplace integration for community plugins

#### `claurst-bridge` (crates/bridge/) — Remote Bridge
**Files:** 1 source file | **Role:** JWT auth, long-polling, device fingerprinting

- `JwtClaims` — client-side JWT decode (no signature verification)
- `device_fingerprint()` — SHA-256 of hostname:user:homedir
- `BridgeConfig` — URL, poll intervals, timeouts
- Session lifecycle: register, poll, upload events, deregister
- Work modes: `single-session`, `worktree`, `same-dir`

#### `claurst-buddy` (crates/buddy/) — Tamagotchi Companion
**Files:** 1 source file | **Role:** Deterministic gacha system, ASCII rendering

- `Mulberry32` PRNG (identical to TypeScript)
- `seed_from_user_id()` — FNV-1a 32-bit hash
- 18 species, 5 rarities, 6 eye styles, 8+ hat options
- Stat generation with rarity-scaled floors
- Sprite rendering (5-line-tall, 12-char-wide ASCII art)

#### `claurst-acp` (crates/acp/) — Agent Client Protocol
**Files:** 1 source file | **Role:** JSON-RPC 2.0 over stdio for IDE integration

Methods: `initialize`, `session/create`, `session/message`, `session/list`, `tool/list`, `model/list`

#### `claurst` (crates/cli/) — Binary Entry Point
**Files:** 3 source files | **Role:** CLI argument parsing, bootstrap, mode dispatch

- `McpToolWrapper` — wraps MCP server tools as native `Tool` trait objects
- clap argument parser (mirrors TypeScript `main.tsx` flags)
- Mode dispatch: headless (`--print`) vs interactive TUI
- OAuth flow integration (`oauth_flow.rs`, `codex_oauth_flow.rs`)

---

## 2. Data Flow & Execution Model

### 2.1 User Turn Flow

```
User Input (TUI PromptInput / --print flag)
    │
    ▼
Command Queue (CommandQueue) ── if slash command ──▶ CommandContext.execute()
    │                                                     │
    │ (regular prompt)                                    ▼
    ▼                                              CommandResult
QueryConfig assembled:                            (Message / ConfigChange /
  model, max_tokens, max_turns,                    ClearConversation / Exit / …)
  system_prompt, effort_level,
  tool_result_budget, fallback_model
    │
    ▼
run_query_loop()  ─── [crates/query/src/lib.rs]
    │
    ├─▶ Build CreateMessageRequest
    │     system = static sections + dynamic boundary + CLAUDE.md + attachments
    │     messages = conversation history (tool-result-budgeted)
    │     tools = all_tools() + MCP tools + AgentTool (if not worker)
    │     thinking = ThinkingConfig if budget > 0
    │
    ├─▶ Stream to API via AnthropicClient (or ProviderRegistry dispatch)
    │     SSE events → QueryEvent channel → TUI renders in real-time
    │
    ├─▶ On tool_use content block:
    │     1. Find tool by name in tool registry
    │     2. Check PermissionLevel vs PermissionMode
    │     3. Fire PreToolUse hooks
    │     4. Execute tool.execute(input, ctx)
    │     5. Fire PostToolUse hooks
    │     6. Inject ToolResult into messages
    │     7. Loop back to API call
    │
    ├─▶ On end_turn: return QueryOutcome::EndTurn
    ├─▶ On max_tokens: inject recovery message, retry (up to 3x)
    ├─▶ After each turn: fire PostModelTurn hooks
    ├─▶ Auto-compact check: if context ≥ 90% → compact_conversation()
    └─▶ Budget check: if cost > max_budget_usd → QueryOutcome::BudgetExceeded
```

### 2.2 Auto-Compact Flow

```
should_auto_compact(messages, model) → true when context ≥ 90%
    │
    ▼
compact_conversation():
  1. Keep last 10 messages verbatim
  2. Group older messages by API round
  3. Send summary request (non-agentic, single API call)
  4. Replace conversation head with <compact-summary>
  5. Update AutoCompactState (circuit breaker on 3 failures)
```

### 2.3 Session Persistence

```
Messages ──▶ JSONL file (session_storage.rs)
         ──▶ SQLite row (sqlite_storage.rs)  [faster queries]

Config   ──▶ ~/.claude/settings.json (global)
         ──▶ .claude/settings.json (project)
         ──▶ .claude/settings.local.json (local, gitignored)

Memory   ──▶ ~/.claude/projects/<hash>/memory/MEMORY.md
         ──▶ Individual topic files in memory/
```

---

## 3. Tool & Capability Registry

### 3.1 Complete Tool List (42 tools + conditional)

| # | Tool Name | Struct | Permission | Module |
|---|-----------|--------|------------|--------|
| 1 | `Bash` | `PtyBashTool` | Execute | `pty_bash.rs` |
| 2 | `Read` | `FileReadTool` | ReadOnly | `file_read.rs` |
| 3 | `Edit` | `FileEditTool` | Write | `file_edit.rs` |
| 4 | `Write` | `FileWriteTool` | Write | `file_write.rs` |
| 5 | `BatchEdit` | `BatchEditTool` | Write | `batch_edit.rs` |
| 6 | `ApplyPatch` | `ApplyPatchTool` | Write | `apply_patch.rs` |
| 7 | `Glob` | `GlobTool` | ReadOnly | `glob_tool.rs` |
| 8 | `Grep` | `GrepTool` | ReadOnly | `grep_tool.rs` |
| 9 | `WebFetch` | `WebFetchTool` | Execute | `web_fetch.rs` |
| 10 | `WebSearch` | `WebSearchTool` | Execute | `web_search.rs` |
| 11 | `NotebookEdit` | `NotebookEditTool` | Write | `notebook_edit.rs` |
| 12 | `TaskCreate` | `TaskCreateTool` | None | `tasks.rs` |
| 13 | `TaskGet` | `TaskGetTool` | None | `tasks.rs` |
| 14 | `TaskUpdate` | `TaskUpdateTool` | None | `tasks.rs` |
| 15 | `TaskList` | `TaskListTool` | None | `tasks.rs` |
| 16 | `TaskStop` | `TaskStopTool` | None | `tasks.rs` |
| 17 | `TaskOutput` | `TaskOutputTool` | None | `tasks.rs` |
| 18 | `TodoWrite` | `TodoWriteTool` | Write | `todo_write.rs` |
| 19 | `AskUser` | `AskUserQuestionTool` | None | `ask_user.rs` |
| 20 | `EnterPlanMode` | `EnterPlanModeTool` | None | `enter_plan_mode.rs` |
| 21 | `ExitPlanMode` | `ExitPlanModeTool` | None | `exit_plan_mode.rs` |
| 22 | `PowerShell` | `PowerShellTool` | Execute | `powershell.rs` |
| 23 | `Sleep` | `SleepTool` | None | `sleep.rs` |
| 24 | `CronCreate` | `CronCreateTool` | Execute | `cron.rs` |
| 25 | `CronDelete` | `CronDeleteTool` | Execute | `cron.rs` |
| 26 | `CronList` | `CronListTool` | ReadOnly | `cron.rs` |
| 27 | `EnterWorktree` | `EnterWorktreeTool` | Execute | `worktree.rs` |
| 28 | `ExitWorktree` | `ExitWorktreeTool` | Execute | `worktree.rs` |
| 29 | `ListMcpResources` | `ListMcpResourcesTool` | ReadOnly | `mcp_resources.rs` |
| 30 | `ReadMcpResource` | `ReadMcpResourceTool` | ReadOnly | `mcp_resources.rs` |
| 31 | `ToolSearch` | `ToolSearchTool` | None | `tool_search.rs` |
| 32 | `Brief` | `BriefTool` | None | `brief.rs` |
| 33 | `Config` | `ConfigTool` | Write | `config_tool.rs` |
| 34 | `SendMessage` | `SendMessageTool` | None | `send_message.rs` |
| 35 | `Skill` | `SkillTool` | None | `skill_tool.rs` |
| 36 | `LSP` | `LspTool` | ReadOnly | `lsp_tool.rs` |
| 37 | `REPL` | `ReplTool` | Execute | `repl_tool.rs` |
| 38 | `TeamCreate` | `TeamCreateTool` | None | `team_tool.rs` |
| 39 | `TeamDelete` | `TeamDeleteTool` | None | `team_tool.rs` |
| 40 | `SyntheticOutput` | `SyntheticOutputTool` | None | `synthetic_output.rs` |
| 41 | `McpAuth` | `McpAuthTool` | Execute | `mcp_auth_tool.rs` |
| 42 | `RemoteTrigger` | `RemoteTriggerTool` | Execute | `remote_trigger.rs` |
| — | `Agent` | `AgentTool` | None | `query/agent_tool.rs` |
| — | `ComputerUse` | `ComputerUseTool` | Dangerous | `computer_use.rs` (feature-gated) |

**Note:** `AgentTool` lives in `claurst-query` (not `claurst-tools`) to break a circular dependency. MCP tools are wrapped via `McpToolWrapper` in the CLI crate.

### 3.2 Persistent Shell State

```rust
pub struct ShellState {
    pub cwd: Option<PathBuf>,
    pub env_vars: HashMap<String, String>,
}
// Process-global DashMap<session_id, Arc<Mutex<ShellState>>>
```

Shell state (cwd, environment) persists across Bash tool invocations within a session via a process-global registry. This ensures `cd` and `export` commands stick.

### 3.3 Snapshot/Undo System

```rust
pub struct SnapshotManager { /* per-session, tracks file before/after states */ }
// Process-global DashMap<session_id, Arc<Mutex<SnapshotManager>>>
```

Every file write/edit records a before/after snapshot. The `/undo` command reverts the most recent change.

---

## 4. Subsystem Deep-Dives

### 4.1 Agent Orchestration (Coordinator Mode)

**Location:** `crates/query/src/coordinator.rs` + `crates/query/src/agent_tool.rs`
**Activation:** `CLAURST_COORDINATOR_MODE=1` environment variable
**Status:** Fully implemented

#### Architecture

```
                    Coordinator (AgentMode::Coordinator)
                    ┌─────────────────────────────────┐
                    │  Has: Agent, SendMessage,        │
                    │  TaskStop, TeamCreate, TeamDelete │
                    │  Banned: Bash (delegates to      │
                    │  workers)                        │
                    └──────────┬──────────┬───────────┘
                               │          │
                    ┌──────────▼──┐  ┌────▼──────────┐
                    │  Worker A   │  │  Worker B      │
                    │  (all tools │  │  (all tools    │
                    │  except     │  │  except        │
                    │  coordinator│  │  coordinator   │
                    │  tools)     │  │  tools)        │
                    └─────────────┘  └────────────────┘
```

**Key types:**
```rust
pub enum AgentMode { Coordinator, Worker, Normal }

pub struct ScratchpadGate {
    unlocked: bool,
    unlock_signal: Option<String>,
}
// Gated tools (Write, FileWrite, Edit, FileEdit) blocked until signal phrase appears
```

**AgentTool capabilities:**
- Synchronous or background (`run_in_background: true`) execution
- Git worktree isolation (`isolation: "worktree"`)
- Background agent registry (`BACKGROUND_AGENTS: DashMap`) with polling via `poll_background_agent()`
- Tool allowlist filtering (defaults to all tools minus `Agent` to prevent recursion)
- Model override per sub-agent
- System prompt override per sub-agent
- Plugin-contributed agent definitions injected into default system prompt

**Team swarm runner injection:**
`init_team_swarm_runner()` injects a real `run_query_loop`-backed runner into `claurst-tools` via the `AgentRunFn` callback, breaking the circular dependency between `claurst-query` and `claurst-tools`.

### 4.2 KAIROS (Always-On)

**Status:** Feature-gated (`kairos_brief`, `kairos_channels`), partially implemented

The KAIROS subsystem exists as feature flags and plumbing but the core "always-on ticking" loop is **not yet implemented** in the Rust codebase. What exists:

| Component | Status |
|-----------|--------|
| `kairos_brief` feature flag | Defined in `Cargo.toml` |
| `kairos_channels` feature flag | Defined in `Cargo.toml` |
| Brief output style | Implemented in `system_prompt.rs` (as `OutputStyle::Concise`) |
| Proactive tick loop | **Not implemented** |
| Daily append-only log files | **Not implemented** |
| 15-second blocking budget | **Not implemented** |
| `SendUserFile` tool | **Not implemented** |
| `PushNotification` tool | **Not implemented** |
| `SubscribePR` tool | **Not implemented** |

The spec (`06_services_context_state.md`) describes a `<tick>` prompt system with proactive observation capabilities. The Rust port has the feature gates but no behavioral implementation.

### 4.3 ULTRAPLAN

**Status:** Feature-gated (`ultraplan`), structurally present but **stub implementation**

| Component | Status |
|-----------|--------|
| `ultraplan` feature flag | Defined, in default features |
| `/ultrareview` command | Struct defined (`UltrareviewCommand`) |
| `/teleport` command | Struct defined (`TeleportCommand`) |
| CCR session spin-up | **Not implemented** |
| 3-second polling loop | **Not implemented** |
| Browser-based approval UI | **Not implemented** |
| `__ULTRAPLAN_TELEPORT_LOCAL__` sentinel | **Not implemented** |
| `tengu_ultraplan_model` config | **Not implemented** |

The `/ultrareview` and `/teleport` commands exist as command structs but the actual remote session orchestration (spinning up a Cloud Container Runtime, polling for results, teleporting back) is absent. This requires the Anthropic cloud infrastructure backend which is not available to the open-source build.

### 4.4 autoDream (Memory Consolidation)

**Location:** `crates/query/src/auto_dream.rs`
**Status:** Fully implemented

```rust
pub struct AutoDream {
    config: AutoDreamConfig,    // min_hours: 24.0, min_sessions: 5
    memory_dir: PathBuf,
    conversations_dir: PathBuf,
    lock_file: PathBuf,         // .consolidation_lock
    state_file: PathBuf,        // .consolidation_state.json
}
```

**Three-gate trigger (cheapest-first):**
1. **Time gate:** `hours_since_last >= min_hours` (24h default)
2. **Session gate:** `sessions_with_mtime_after_last >= min_sessions` (5 default)
3. **Lock gate:** No `.consolidation_lock` file (stale after 1 hour)

**Four-phase consolidation prompt:**
1. **Orient:** `ls` memory dir, read `MEMORY.md`, skim topic files
2. **Gather Recent Signal:** Daily logs, drifted memories, narrow transcript grep
3. **Consolidate:** Write/update memory files, convert dates, delete contradictions
4. **Prune and Index:** Keep `MEMORY.md` under 200 lines / ~25KB

The sub-agent gets read-only Bash access. Consolidation is finalized via `finish_consolidation()` which persists state and releases the lock.

### 4.5 BUDDY (Tamagotchi)

**Location:** `crates/buddy/src/lib.rs`
**Status:** Core mechanics implemented, rendering partially implemented

**Implemented:**
- `Mulberry32` PRNG (bit-exact match with TypeScript)
- `seed_from_user_id()` using FNV-1a 32-bit
- 18 species enum with all names
- 5 rarity tiers with weights (60/25/10/4/1)
- 6 eye styles with Unicode glyphs
- 8+ hat options (gated by rarity)
- Stat generation (5 stats: 0-100 with rarity floors)
- Shiny flag (1% probability)
- Rarity stars display

**Not yet implemented:**
- ASCII sprite frames (5×12 character art per species)
- Sprite animation system (idle, reaction frames)
- Speech bubble rendering
- Soul generation prompt (AI-generated name/personality)
- `companion.json` persistence
- TUI integration (rendering beside PromptInput)
- 500ms animation tick

### 4.6 Multi-Provider System

**Location:** `crates/api/`
**Status:** Fully implemented with 12+ providers

**Provider registry hierarchy:**
```
ProviderRegistry
├── AnthropicProvider    (native Anthropic API)
├── GoogleProvider       (Gemini API)
├── OpenAiProvider       (OpenAI Chat API)
├── AzureProvider        (Azure OpenAI)
├── BedrockProvider      (AWS Bedrock)
├── CopilotProvider      (GitHub Copilot)
├── CohereProvider       (Cohere native)
└── OpenAiCompatProvider (generic adapter)
    ├── ollama()
    ├── lm_studio()
    ├── deepseek()
    ├── groq()
    ├── xai()
    ├── openrouter()
    ├── mistral()
    └── (30+ more recognized provider IDs)
```

The `/connect` command triggers the provider configuration flow. Provider selection persists in settings. The `ModelRegistry` fetches model lists from models.dev and maps them to providers.

Provider-specific options are built by `build_provider_options()` which handles:
- Anthropic thinking budget
- Google Gemini thinking config (2.5 vs 3.x)
- OpenAI reasoning effort (o1/o3/o4/GPT-5 models)
- AWS Bedrock reasoning config
- GitHub Copilot model-specific options

### 4.7 Speech Modes

**Location:** `crates/commands/src/lib.rs` (`CavemanCommand`, `RockyCommand`, `NormalCommand`)
**Status:** Fully implemented as system prompt modifiers

Three speech modes activated via slash commands:
- `/caveman` — Extremely terse, primitive language ("save big token")
- `/rocky` — Enthusiastic, motivational ("amaze amaze amaze")
- `/normal` — Deactivate speech mode

These inject output style overrides into the session config via `CommandResult::SpeechMode`. The TUI applies the speech mode as a system prompt suffix via the `OutputStyle` system.

---

## 5. State & Memory Management

### 5.1 Configuration Cascade

```
Priority (highest → lowest):
1. CLI arguments (--model, --permission-mode, etc.)
2. Environment variables (ANTHROPIC_API_KEY, CLAURST_COORDINATOR_MODE, etc.)
3. .claude/settings.local.json (project-local, gitignored)
4. .claude/settings.json (project-shared)
5. ~/.claude/settings.json (global)
```

```rust
pub struct Config {
    pub model: Option<String>,
    pub permission_mode: PermissionMode,
    pub api_key: Option<String>,
    pub provider: Option<String>,
    pub project_dir: Option<PathBuf>,
    pub hooks: HashMap<HookEvent, Vec<HookEntry>>,
    pub mcp_servers: Vec<McpServerConfig>,
    pub theme: Theme,
    pub agents: Vec<AgentDefinition>,
    pub formatter: Option<FormatterConfig>,
    pub skills: Option<SkillsConfig>,
    // ... ~30 more fields
}
```

### 5.2 Session Storage

**JSONL backend** (`session_storage.rs`): Append-only log of messages per session. Fast writes, O(n) reads.

**SQLite backend** (`sqlite_storage.rs`): Indexed storage with `SessionSummary` queries. Used by `/resume`, `/session` commands and the session browser TUI.

### 5.3 Memory System

```
~/.claude/projects/<project_hash>/memory/
├── MEMORY.md              ← Index file (always loaded, max 200 lines)
├── user_role.md           ← Individual memory files with YAML frontmatter
├── feedback_testing.md
├── project_goals.md
└── .consolidation_state.json  ← autoDream state
```

**Memory types:** `user`, `feedback`, `project`, `reference` (defined in auto-memory system prompt, not as Rust types).

**Session memory extraction** (`session_memory.rs`):
- Triggers after 20+ messages in a session
- Calls API with structured extraction prompt
- Parses into `ExtractedMemory` entries with categories: `UserPreference`, `ProjectFact`, `CodePattern`, `Decision`, `Constraint`
- Appends to CLAUDE.md under `## Auto-extracted memories`

### 5.4 Context Management

**Token budget:** Configurable, default 200K window. Auto-compact at 90%. Warning at 80%, critical at 95%.

**Tool-result budgeting:** Oldest tool results truncated when cumulative character count exceeds 50,000 (configurable via `QueryConfig.tool_result_budget`).

**Max-tokens recovery:** Up to 3 retry attempts with a resume prompt when model hits output limit.

### 5.5 Hook System

```rust
pub enum HookEvent {
    PreToolUse,
    PostToolUse,
    PostModelTurn,
    Stop,
}
```

Hooks are shell commands executed synchronously (PreToolUse, PostToolUse, PostModelTurn) or fire-and-forget (Stop). Non-zero exit codes from PostModelTurn hooks inject error messages into the conversation. Exit code > 1 prevents continuation.

---

## 6. AI Integration Guide

### 6.1 Adding a New Tool

**Step 1:** Create `crates/tools/src/my_tool.rs`:
```rust
use async_trait::async_trait;
use claurst_tools::{PermissionLevel, Tool, ToolContext, ToolResult};
use serde_json::{json, Value};

pub struct MyTool;

#[async_trait]
impl Tool for MyTool {
    fn name(&self) -> &str { "MyTool" }
    fn description(&self) -> &str { "Does the thing." }
    fn permission_level(&self) -> PermissionLevel { PermissionLevel::ReadOnly }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "param": { "type": "string", "description": "..." }
            },
            "required": ["param"]
        })
    }
    async fn execute(&self, input: Value, ctx: &ToolContext) -> ToolResult {
        // Implementation
        ToolResult::success("result")
    }
}
```

**Step 2:** Register in `crates/tools/src/lib.rs`:
```rust
pub mod my_tool;
pub use my_tool::MyTool;
// Add to all_tools():
Box::new(MyTool),
```

**Step 3:** Add tool name constant in `crates/core/src/lib.rs` constants section:
```rust
pub const TOOL_NAME_MY_TOOL: &str = "MyTool";
```

### 6.2 Adding a New Slash Command

**Step 1:** Add struct in `crates/commands/src/lib.rs`:
```rust
pub struct MyCommand;
```

**Step 2:** Implement `SlashCommand` trait in `crates/commands/src/named_commands.rs` or `lib.rs`:
```rust
#[async_trait]
impl SlashCommand for MyCommand {
    fn name(&self) -> &str { "mycommand" }
    fn description(&self) -> &str { "Does the thing" }
    async fn execute(&self, args: &str, ctx: &mut CommandContext) -> CommandResult {
        CommandResult::Message("Done!".to_string())
    }
}
```

**Step 3:** Register in the command map (in `crates/commands/src/lib.rs`, `all_commands()` function).

**Step 4:** Add to `PROMPT_SLASH_COMMANDS` in `crates/tui/src/app.rs` for autocomplete.

### 6.3 Adding a New LLM Provider

**Step 1:** Create `crates/api/src/providers/my_provider.rs`:
```rust
pub struct MyProvider { /* config fields */ }

#[async_trait]
impl LlmProvider for MyProvider {
    fn id(&self) -> &ProviderId { &ProviderId::new("my-provider") }
    fn name(&self) -> &str { "My Provider" }
    async fn create_message_stream(&self, request: ProviderRequest)
        -> Result<Pin<Box<dyn Stream<Item = Result<StreamEvent, ProviderError>> + Send>>, ProviderError>
    { /* ... */ }
    // ... other trait methods
}
```

**Step 2:** Add to `crates/api/src/providers/mod.rs` and re-export from `crates/api/src/lib.rs`.

**Step 3:** Register in `ProviderRegistry` construction (in CLI main.rs).

**Step 4:** Add provider ID to `is_openaiish_provider()` if it uses OpenAI-compatible APIs.

### 6.4 Adding a Feature Flag

**Step 1:** Add to `crates/core/Cargo.toml`:
```toml
[features]
my_feature = []
dev_full = ["my_feature", ...]
```

**Step 2:** Gate code with:
```rust
#[cfg(feature = "my_feature")]
fn my_feature_code() { /* ... */ }
```

**Step 3:** Feature flags propagate to dependent crates via Cargo passthrough. Add the feature to any downstream `Cargo.toml` that needs it.

### 6.5 Key Architectural Constraints

1. **No circular dependencies:** `claurst-tools` cannot depend on `claurst-query`. Use callback injection (see `AgentRunFn`).
2. **Process-global registries:** Shell state, snapshot state, hook registries, plugin registries — all use `OnceLock` or `Lazy<DashMap>`. Thread-safe but not test-isolated.
3. **Permission model:** Always declare the correct `PermissionLevel`. The TUI presents permission dialogs based on this.
4. **Tool schemas:** Must be valid JSON Schema objects with `type` and `properties`. Used directly in API requests.
5. **Async everywhere:** All tool execution is async (`#[async_trait]`). Use `tokio::spawn` for background work, `tokio::task::spawn_blocking` for synchronous I/O.

---

## 7. Delta Analysis: Spec vs Implementation

### 7.1 Fully Implemented (Spec Parity)

| Subsystem | Spec Files | Rust Location |
|-----------|-----------|---------------|
| Query/turn loop | 01 | `query/lib.rs` |
| Tool framework + 42 tools | 03 | `tools/` |
| Auto-compact + micro-compact | 01, 06 | `query/compact.rs` |
| Session persistence (JSONL + SQLite) | 06 | `core/session_storage.rs`, `core/sqlite_storage.rs` |
| Settings cascade | 00 | `core/config.rs` (in lib.rs) |
| Permission system | 05, 00 | `core/permissions.rs` (in lib.rs) |
| Slash commands (85+) | 02 | `commands/` |
| TUI with ratatui | 04, 08 | `tui/` |
| Multi-provider API | 12 | `api/` (12+ providers) |
| MCP client | 03 | `mcp/` |
| Coordinator mode | 06 | `query/coordinator.rs` |
| AgentTool (worktree, background) | 03 | `query/agent_tool.rs` |
| autoDream consolidation | 06 | `query/auto_dream.rs` |
| CLAUDE.md loading | 11 | `core/claudemd.rs` |
| Memory directory | 11 | `core/memdir.rs` |
| Session memory extraction | 06 | `query/session_memory.rs` |
| Voice input (cpal + Whisper) | 11 | `core/voice.rs` |
| Keybindings (chords, vim mode) | 11 | `core/keybindings.rs` |
| Plugin system | 11 | `plugins/` |
| Skill discovery | 11 | `core/skill_discovery.rs` |
| Bridge protocol | 09 | `bridge/` |
| ACP for IDE integration | 09 | `acp/` |
| Hook system | 07 | `plugins/hooks.rs`, `query/lib.rs` |
| Buddy PRNG + species/rarity | 11 | `buddy/` |
| Git utilities | 10 | `core/git_utils.rs` |
| Cron scheduling | 06 | `query/cron_scheduler.rs` |
| Speech modes | — | `commands/` |
| Output styles | 11 | `core/system_prompt.rs`, `core/output_styles.rs` |

### 7.2 Partially Implemented (Gaps Identified)

| Subsystem | What's Missing | Spec Ref |
|-----------|---------------|----------|
| **BUDDY rendering** | ASCII sprite frames (5×12 art), animation tick, speech bubbles, soul generation prompt, TUI integration beside PromptInput | 11 §buddy |
| **KAIROS** | Proactive tick loop, daily logs, 15-second blocking budget, `SendUserFile`/`PushNotification`/`SubscribePR` tools | 06 §KAIROS |
| **ULTRAPLAN** | CCR session spin-up, polling loop, browser approval UI, `__ULTRAPLAN_TELEPORT_LOCAL__` sentinel | 06 §ULTRAPLAN |
| **Agent Teams/Swarm** | Color assignments, visual distinction, tmux/iTerm2 pane spawning (current impl uses in-process `TeamCreateTool` with callback injection) | 06 §coordinator |
| **Transcript classifier** | ML-based auto-approval for AFK mode (`TRANSCRIPT_CLASSIFIER` feature flag exists but no implementation) | 05 §permissions |
| **Computer Use ("Chicago")** | Feature-gated, struct exists but `enigo`/`xcap` integration is compile-time conditional; no end-to-end test path | 03 §ComputerUse |
| **GrowthBook integration** | Replaced with env-var-based feature gates; no remote feature flag service | 06 §analytics |
| **Analytics/telemetry** | Module exists (`core/analytics.rs`) but no Datadog or first-party backend integration | 06 §analytics |
| **Undercover mode** | Not implemented (internal Anthropic feature, not relevant for open-source) | 12 §undercover |
| **Fast/Penguin mode** | `/fast` command exists but no API endpoint for penguin mode | 12 §fastMode |

### 7.3 Not Implemented (By Design)

| Subsystem | Reason |
|-----------|--------|
| **Custom Ink renderer** | Replaced by `ratatui` + `crossterm` — equivalent capability, idiomatic Rust |
| **React component tree** | N/A — Rust has no React; TUI uses direct ratatui widget rendering |
| **Bun bundler/runtime** | N/A — compiled Rust binary, no bundler needed |
| **Source map / npm concerns** | N/A — Rust binary distribution |
| **`NATIVE_CLIENT_ATTESTATION`** | `cch.rs` exists for header computation but real attestation requires Anthropic infrastructure |
| **Internal-only tools** (`TungstenTool`, `SuggestBackgroundPRTool`) | Intentionally omitted from open-source build |

### 7.4 Rust-Only Additions (Not in Spec)

| Addition | Location | Purpose |
|----------|----------|---------|
| `ApplyPatchTool` | `tools/apply_patch.rs` | Unified diff patch application |
| `BatchEditTool` | `tools/batch_edit.rs` | Multi-file atomic edits |
| `PtyBashTool` | `tools/pty_bash.rs` | PTY-based shell with persistent state |
| `CodexAdapter` | `api/codex_adapter.rs` | OpenAI Codex/Responses API compatibility |
| `CopilotProvider` | `api/providers/copilot.rs` | GitHub Copilot integration |
| `CohereProvider` | `api/providers/cohere.rs` | Cohere native API |
| `PluginCapabilities` | `plugins/lib.rs` | Capability-based plugin security model |
| `SessionBranching` | `tui/session_branching.rs` | Fork sessions into new branches |
| `Rustle` | `tui/rustle.rs` | Mascot ASCII art |
| 30+ OpenAI-compat providers | `api/providers/openai_compat_providers.rs` | ollama, lm_studio, deepseek, groq, xai, etc. |

---

*End of Functional Specification. This document was generated from a read-only analysis of the Claurst codebase at commit `acae926` (2026-04-07).*
