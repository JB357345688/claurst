# Claurst — Feature Brainstorm

> Grounded proposals for meaningful improvements to the agentic harness.
> Each entry includes rationale, architectural fit, and implementation surface.
> Ordered by estimated impact, not difficulty.

---

## 1. Adaptive Context Windowing

**Problem:** The current auto-compact fires at a flat 90% threshold and uses a fixed "keep last 10 messages" strategy. This treats a 3-message debugging session the same as a 200-message refactoring marathon. Compaction destroys tool-result context that the model may need moments later.

**Proposal:** Replace the static compaction trigger with a signal-weighted system:

- **Recency decay** — weight messages by how recently they were referenced (tool results that produced follow-up tool calls are "hot" and should survive longer)
- **Semantic clustering** — group messages by topic/file-path affinity before summarizing, so compaction produces per-topic summaries rather than one flat blob
- **Tiered eviction** — evict tool results first (cheapest to regenerate), then thinking blocks, then assistant text, then user messages (most expensive to lose)
- **Anchor pinning** — let the model or user mark messages as "pinned" (never compacted) via a `pin` content block or `/pin N` command

**Where it fits:** `crates/query/src/compact.rs`. The `group_messages_for_compact()` function already groups by API round — extend with affinity scoring. Add a `CompactionStrategy` enum to `QueryConfig`.

---

## 2. Speculative Tool Execution

**Problem:** The model often chains predictable tool sequences: `Glob → Read → Edit → Read (verify)`. Each step requires a full API round-trip. Four tool calls = four streaming responses, burning tokens on "now I'll read the file" filler text.

**Proposal:** Detect common tool-chain patterns and pre-execute likely next steps:

- After a `Glob` that returns 1-3 files, speculatively `Read` them and inject the content as a prefilled assistant context attachment
- After an `Edit`, speculatively run the project's configured formatter and linter (if `config.formatter` is set) and append diagnostics
- After a `Bash` test command that fails, speculatively read the failing test file

Implementation: A `SpeculativeExecutor` that runs in a background task during API streaming. Results are held in a `PrefetchCache` and attached to the next API request if the model's actual tool call matches. Misses are discarded silently.

**Where it fits:** New module `crates/query/src/speculative.rs`. Hooks into the tool dispatch path in `run_query_loop()` after each `ToolEnd` event.

---

## 3. Structured Undo / Session Checkpoints

**Problem:** The current `SnapshotManager` tracks individual file before/after states, and `/undo` reverts one file at a time. But agentic work is transactional — a single "task" might touch 8 files, run migrations, and update configs. Undoing one file leaves the project in an inconsistent state.

**Proposal:** Introduce transaction-level checkpoints:

- **Auto-checkpoint** before each model turn that contains tool_use blocks
- **Named checkpoints** via `/checkpoint [name]` command
- **Atomic rollback** via `/rollback [checkpoint]` that reverts all files changed since that point
- **Checkpoint diff** via `/checkpoint diff [name]` showing all changes since
- Store checkpoints as lightweight git stashes (or in-memory if not a git repo)

**Where it fits:** Extend `crates/core/src/snapshot.rs` to support grouped snapshots. Add `/checkpoint` and `/rollback` commands to `crates/commands/`. The `ToolContext.record_file_change()` path already captures before/after — add a checkpoint-id tag.

---

## 4. Live Test Feedback Loop

**Problem:** The model writes code, then runs tests, then reads failures, then fixes — each step is a separate tool call. Test failures are the single most valuable signal for code correctness, but the model treats them like any other tool output.

**Proposal:** A dedicated test-watcher mode:

- `/test-watch [command]` starts a background file watcher (via `notify` crate)
- On any file write by a tool, automatically re-run the test command
- Inject test results directly into the conversation as a system-level `<test-results>` block (not a tool result — it shouldn't count against tool-result budget)
- Color-code pass/fail in the TUI with a persistent test status bar
- Model gets a special system prompt section: "Tests are running automatically. After each edit, wait for the test result before proceeding."

**Where it fits:** New module `crates/query/src/test_watcher.rs`. Uses `notify` for filesystem events. Integrates with the `QueryEvent` channel to push `TestResult` events to the TUI. Config via `QueryConfig.test_command`.

---

## 5. Retrieval-Augmented Tool Selection

**Problem:** With 42+ built-in tools plus MCP tools, the tool list in the system prompt can consume 5-10K tokens. Most tasks only need 5-8 tools. The model wastes attention on irrelevant tool schemas.

**Proposal:** Dynamic tool selection based on conversation context:

- **Phase 1:** Heuristic filtering — if the user's message mentions "git", include worktree tools; if it mentions "test", include Bash; if it mentions a file path, include Read/Edit/Write. Always include a core set (Bash, Read, Edit, Write, Glob, Grep).
- **Phase 2:** Deferred tool loading — send tool names + one-line descriptions first; when the model calls `ToolSearch`, return full schemas for the requested tools only (this already exists via `ToolSearchTool` but isn't the default behavior)
- **Phase 3:** Per-agent tool profiles — agent definitions (`AgentDefinition`) already have a `tools` field; auto-detect agent type from the task and apply the appropriate profile

**Where it fits:** The tool filtering already happens in `run_query_loop()` when building the API request. Add a `ToolSelector` trait with `HeuristicSelector` and `DeferredSelector` implementations. The `ToolSearchTool` already exists — make it the default path for non-core tools.

---

## 6. Cross-Session Learning

**Problem:** autoDream consolidates memories after sessions, but the model starts every session cold — it re-discovers project conventions, test commands, build steps. The CLAUDE.md file helps but is static and manually maintained.

**Proposal:** Automatic project profile generation:

- After every 10 sessions in the same project, generate a `project_profile.json` containing:
  - Detected language/framework (from file extensions, package files)
  - Build command (from successful Bash runs of `cargo build`, `npm run build`, etc.)
  - Test command (from successful test runs)
  - Lint command (from successful lint runs)
  - Common file patterns accessed
  - Frequently edited directories
- Inject this profile into the system prompt as structured context
- Make it editable via `/project-profile` command

**Where it fits:** New module `crates/core/src/project_profile.rs`. Hooks into `session_storage.rs` to analyze historical tool calls. Injected via the attachment pipeline in `crates/core/src/attachments.rs`.

---

## 7. Differential Streaming

**Problem:** When the model edits a large file, the Edit tool sends the entire `old_string` and `new_string` in the tool call. The model must generate the full `old_string` verbatim — often hundreds of tokens reproducing existing code just to identify the edit location.

**Proposal:** A line-addressed edit mode that eliminates redundant output:

- `EditByLine` tool: accepts `file_path`, `start_line`, `end_line`, `new_content`
- The model only generates the replacement content, not the matching context
- Validation: read lines `start_line..end_line`, verify they exist, apply replacement
- Combine with the existing `ApplyPatchTool` for unified diff format as an alternative

This could reduce Edit tool token usage by 40-60% on large files.

**Where it fits:** New tool in `crates/tools/src/edit_by_line.rs`. Register alongside `FileEditTool`. The model can choose between context-match (`Edit`) and line-addressed (`EditByLine`) based on file size.

---

## 8. Sandboxed Execution Environments

**Problem:** The `BashTool` runs commands directly on the host. Dangerous commands are classified by `bash_classifier.rs` and blocked, but this is a deny-list approach — novel dangerous commands can slip through.

**Proposal:** Optional containerized execution:

- **Lightweight:** `bwrap` (bubblewrap) sandboxing on Linux — mount project dir read-write, everything else read-only, no network by default
- **Medium:** `podman`/`docker` container per session with the project mounted
- **Heavy:** Full VM isolation (for computer-use scenarios)

Config: `sandbox_mode: "none" | "bwrap" | "container"` in settings.json. The `PtyBashTool` already has persistent shell state — extend it with a sandbox context.

**Where it fits:** `crates/tools/src/pty_bash.rs` gets a `SandboxBackend` trait. `BwrapBackend` wraps commands in `bwrap` invocations. Container backend spawns a persistent container and exec's into it.

---

## 9. Conversation Branching with Visual DAG

**Problem:** `/fork` and `/rewind` exist but conversations are still linear. The user can't easily explore two approaches in parallel and compare results.

**Proposal:** Full conversation DAG:

- Every `/fork` creates a named branch from the current message
- Branches are visualized in the TUI as a tree (like `git log --graph`)
- `/switch [branch]` to jump between branches
- `/merge [branch]` to cherry-pick tool results from another branch into the current one
- `/compare [branch]` shows a side-by-side diff of file states between branches
- Backed by the existing session storage — each branch is a separate JSONL file with a parent pointer

**Where it fits:** Extend `crates/tui/src/session_branching.rs` (which already exists but is minimal). Add branch metadata to `SessionSummary`. The session browser (`session_browser.rs`) gets a tree view mode.

---

## 10. Agent Observability Dashboard

**Problem:** When the coordinator spawns multiple background agents, the user has no visibility into what each agent is doing until it completes. The `tasks_overlay.rs` shows task status but not live agent activity.

**Proposal:** A real-time agent dashboard:

- Split-pane TUI mode showing all active agents and their current tool call
- Per-agent token usage, elapsed time, and tool call count
- Ability to send a message to a running agent (`SendMessage` already exists in the protocol)
- Ability to cancel a specific agent
- Aggregate cost tracking across all agents
- Post-completion summary with per-agent contribution breakdown

**Where it fits:** `crates/tui/src/agents_view.rs` already has `AgentInfo` and `AgentStatus` types. Extend with live streaming from the `BACKGROUND_AGENTS` registry. Add a `QueryEvent::AgentUpdate` variant for real-time status.

---

## 11. Intelligent File Watching

**Problem:** The model often reads a file, edits it, then the user manually modifies the same file outside Claurst. On the next turn, the model's context has a stale version and may produce conflicting edits.

**Proposal:** Background file watcher for session-touched files:

- Track all files read or written during the session (already in `FileHistory`)
- Watch those files for external modifications (via `notify` crate)
- On external change, inject a system message: `"[File externally modified: src/main.rs — re-read before editing]"`
- Optional: auto-re-read and diff against the model's last known version

**Where it fits:** New module `crates/query/src/file_watcher.rs`. Reads the `FileHistory` to know which files to watch. Pushes notifications through the `CommandQueue`.

---

## 12. Cost-Aware Planning

**Problem:** The model has no awareness of token costs. A "simple" refactoring request might spawn 5 sub-agents each burning 100K tokens. The user only discovers the cost after the fact.

**Proposal:** Budget-aware agentic planning:

- Before spawning sub-agents, the coordinator estimates total cost based on task complexity and historical averages
- Present estimate to user: "This task will likely cost ~$2.40 across 3 agents. Proceed?"
- Per-agent budget caps (already in `QueryConfig.max_budget_usd`) enforced automatically
- Cost breakdown in the `/cost` command: per-agent, per-tool, per-turn
- Configurable session budget with warning thresholds

**Where it fits:** Extend `CostTracker` in `crates/core/src/cost.rs` with per-agent tracking. Add cost estimation heuristics to the coordinator system prompt. The `QueryOutcome::BudgetExceeded` path already exists.

---

## 13. Semantic Code Search (Tree-sitter Integration)

**Problem:** `GrepTool` does regex text search. Finding "all implementations of trait X" or "all callers of function Y" requires the model to grep, read, grep again — multiple round trips for what a language-aware tool could answer in one call.

**Proposal:** A `SemanticSearch` tool backed by tree-sitter:

- Parse files with tree-sitter grammars (the `tree_sitter_bash` feature flag already exists)
- Index symbol definitions, references, and call sites
- Queries: "find all implementations of trait `Tool`", "find all callers of `run_query_loop`", "find all structs with field `session_id`"
- Output: structured results with file path, line number, and surrounding context
- Incremental indexing — only re-parse changed files

**Where it fits:** New crate `claurst-semantic` or module in `crates/tools/`. The `tree_sitter_bash` and `tree_sitter_bash_shadow` feature flags suggest tree-sitter integration was already planned. Extend to Rust, TypeScript, Python, Go grammars.

---

## 14. Persistent Agent Workers (Daemon Mode)

**Problem:** Every sub-agent starts cold — creates a new API client, builds a tool list, has no context from previous runs. For recurring tasks (CI monitoring, PR reviews, code quality checks), this is wasteful.

**Proposal:** Long-lived agent workers:

- `/daemon start [name] [prompt]` — start a persistent background agent
- The daemon runs in a separate tokio task with its own conversation history
- Daemons can be triggered by events: file changes, git push, cron schedule, or explicit `/daemon trigger [name]`
- Daemons share the session's MCP servers and permission rules
- `/daemon list` / `/daemon stop [name]` / `/daemon logs [name]`
- Daemons survive session restarts (state persisted to SQLite)

**Where it fits:** Builds on `crates/query/src/cron_scheduler.rs` (which already handles scheduled triggers). Add a `DaemonRegistry` with persistent state. The `BACKGROUND_AGENTS` DashMap provides the runtime pattern.

---

## 15. Multi-Modal Context Injection

**Problem:** The model can read images via `FileReadTool` (base64 encoding), but there's no structured way to inject screenshots, diagrams, or terminal recordings into the conversation context without manual file paths.

**Proposal:** Rich context injection:

- **Terminal screenshot** — `/screenshot` captures current terminal state as an image and injects it (useful for showing the model a visual bug)
- **Clipboard paste** — Ctrl+V pastes clipboard images directly into the prompt (the `native_clipboard_image` feature flag exists but isn't wired)
- **URL screenshot** — `WebScreenshotTool` that takes a URL, renders it headlessly (via `chromiumoxide` or similar), and injects the screenshot
- **Diagram rendering** — Accept Mermaid/PlantUML in model output and render to inline terminal graphics (Kitty protocol — `kitty_image.rs` already exists)

**Where it fits:** `crates/tui/src/image_paste.rs` already exists. Wire the `native_clipboard_image` feature flag. Add `WebScreenshotTool` to `crates/tools/`. Diagram rendering in `crates/tui/src/messages/`.

---

## 16. Collaborative Sessions

**Problem:** Claurst is single-user. Two developers can't work on the same codebase with shared context — each runs their own session with duplicated discovery work.

**Proposal:** Shared session mode:

- Session owner starts with `/share --collab` which opens a WebSocket server
- Second user connects with `claurst --join [session-id]`
- Both users see the same conversation and can send messages
- File edits are conflict-checked (lock file while tool is executing)
- Each user gets their own permission prompts
- Conversation shows user attribution: `[alice]: fix the tests` / `[bob]: add error handling`

**Where it fits:** Builds on the bridge protocol in `crates/bridge/`. The JWT auth and device fingerprinting already support multi-device identity. Add a `CollabServer` that multiplexes `QueryEvent` streams.

---

## 17. Self-Correction via Assertion Tools

**Problem:** The model writes code and moves on. Bugs are only caught when tests run (if they exist). Many projects have no tests for the code being modified.

**Proposal:** Inline assertion tools that the model uses to verify its own work:

- `AssertFileContains { path, pattern }` — verify a file contains expected content after editing
- `AssertBashSucceeds { command }` — verify a command exits 0
- `AssertNoRegressions { test_command }` — run tests and assert no new failures vs baseline
- `AssertTypeChecks { path }` — run the language's type checker on a specific file

These are wrappers around existing tools but with the key difference: **failures are injected back as errors the model must fix before continuing**, rather than informational output.

**Where it fits:** New tools in `crates/tools/src/assertions.rs`. System prompt instructs the model to use assertion tools after significant changes. Hook into `PostToolUse` to enforce fix-before-continue.

---

## 18. Effort-Adaptive Token Budgets

**Problem:** The `EffortLevel` (Low/Medium/High/Max) only affects thinking budget and temperature. A "Low" effort question like "what does this function do?" still gets the full tool suite and context window, wasting resources.

**Proposal:** Effort level controls the entire resource envelope:

| Resource | Low | Medium | High | Max |
|----------|-----|--------|------|-----|
| Tools available | Read, Grep, Glob | Core set (10) | Full set (42) | Full + experimental |
| Max turns | 1 | 5 | 10 | 30 |
| Context target | 8K | 32K | 128K | 200K |
| Auto-compact | Aggressive | Normal | Conservative | Off |
| Sub-agents | Disabled | 1 max | 5 max | Unlimited |
| Thinking budget | 0 | 2K | 8K | 32K |

**Where it fits:** Extend `EffortLevel` in `crates/core/src/effort.rs` with methods like `max_turns()`, `tool_budget()`, `context_target()`. Apply in `QueryConfig::from_config()`.

---

## Summary Matrix

| # | Feature | Impact | Complexity | Crate(s) Affected |
|---|---------|--------|------------|-------------------|
| 1 | Adaptive Context Windowing | High | Medium | query |
| 2 | Speculative Tool Execution | High | High | query |
| 3 | Structured Undo / Checkpoints | High | Medium | core, commands |
| 4 | Live Test Feedback Loop | High | Medium | query, tui |
| 5 | Retrieval-Augmented Tool Selection | Medium | Low | query |
| 6 | Cross-Session Learning | Medium | Medium | core |
| 7 | Differential Streaming | Medium | Low | tools |
| 8 | Sandboxed Execution | Medium | High | tools |
| 9 | Conversation Branching DAG | Medium | High | tui, core |
| 10 | Agent Observability Dashboard | Medium | Medium | tui, query |
| 11 | Intelligent File Watching | Medium | Low | query |
| 12 | Cost-Aware Planning | Medium | Low | core, query |
| 13 | Semantic Code Search | High | High | new crate |
| 14 | Persistent Agent Workers | Medium | High | query |
| 15 | Multi-Modal Context Injection | Low | Medium | tui, tools |
| 16 | Collaborative Sessions | Low | High | bridge |
| 17 | Self-Correction Assertions | High | Low | tools |
| 18 | Effort-Adaptive Token Budgets | Medium | Low | core, query |

### Recommended first batch (high impact, low-medium complexity):
1. **#5 — Retrieval-Augmented Tool Selection** — immediate token savings, low risk
2. **#17 — Self-Correction Assertions** — directly improves code quality
3. **#12 — Cost-Aware Planning** — user trust and budget control
4. **#7 — Differential Streaming** — reduces output tokens by 40-60% on edits
5. **#11 — Intelligent File Watching** — prevents stale-context bugs

---

*Generated 2026-04-07 from architectural analysis of the Claurst codebase.*
