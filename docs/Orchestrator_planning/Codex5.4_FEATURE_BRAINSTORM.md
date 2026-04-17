# Claurst Harness Feature Brainstorm for Codex 5.4

> Grounded in `FUNCTIONAL_SPEC.md` and the live checkout under `src-rust/crates/`.
> The most important implementation fact from the scan is that the root query loop is already multi-provider, but sub-agents and teams are still mostly Anthropic-bound and string-aggregated.

## Scan Summary

- The real workspace lives under `src-rust/crates/`, not the spec's shortened `crates/` paths.
- `src-rust/crates/query/src/lib.rs` already contains a capable provider-dispatch path inside `run_query_loop()`, including local OpenAI-compatible overrides for Ollama, LM Studio, and llama.cpp.
- `src-rust/crates/query/src/agent_tool.rs` and the injected team runner still construct an `AnthropicClient` directly and fail when `ANTHROPIC_API_KEY` is absent, which means multi-provider support does not actually extend to sub-agents.
- `src-rust/crates/tools/src/team_tool.rs` is useful but shallow: it runs `join_all`, writes `config.json`/`results.json`, and returns one aggregated string. It has no dependency graph, no artifact bus, and no live campaign state.
- `src-rust/crates/query/src/coordinator.rs` defines coordinator policy and `ScratchpadGate`, but those seams are much thinner than the functional spec suggests.
- `src-rust/crates/query/src/cron_scheduler.rs` is already a viable substrate for a KAIROS-style always-on loop.
- `src-rust/crates/tools/src/remote_trigger.rs` is cloud-shaped and currently posts with an empty token, so open-source cross-session orchestration is still incomplete.
- `src-rust/crates/tools/src/lib.rs` exposes `PermissionLevel` and `ToolContext`, but permission enforcement is still tool-local rather than centrally lease-based.

## 1. Provider-Aware Worker Fabric

### Feature Name & Description

Turn sub-agents and teams into first-class multi-provider workers. Today the parent loop can route to Anthropic, OpenAI, Google, Ollama, LM Studio, and other providers, but spawned agents do not inherit that capability. The highest-impact fix is to make provider selection, capability matching, and fallback logic work identically for root sessions, `Agent`, and `TeamCreate`.

This unlocks mixed swarms such as:

- local Ollama/LM Studio workers for wide code search and cheap bulk summarization
- Anthropic/OpenAI workers for synthesis or patch generation
- provider-specific workers selected by tool-calling, vision, or reasoning support

### Crate Topology

- Modify `src-rust/crates/query/src/agent_tool.rs`
- Modify `src-rust/crates/tools/src/team_tool.rs`
- Modify `src-rust/crates/query/src/lib.rs`
- Create `src-rust/crates/query/src/agent_scheduler.rs`
- Modify `src-rust/crates/api/src/registry.rs`
- Modify `src-rust/crates/api/src/model_registry.rs`
- Modify `src-rust/crates/api/src/provider_types.rs`

### Tool Interfaces

No new top-level tool is strictly required. The cleanest path is to extend the existing `Agent` and `TeamCreate` schemas.

`Agent` additions:

```json
{
  "provider": { "type": "string", "description": "Preferred provider id, e.g. anthropic, openai, ollama" },
  "required_capabilities": {
    "type": "array",
    "items": { "type": "string", "enum": ["tool_calling", "thinking", "image_input", "pdf_input"] }
  },
  "budget_usd": { "type": "number" },
  "access_profile": { "type": "string", "enum": ["full", "read-only", "search-only"] },
  "runtime_id": { "type": "string", "description": "Optional runtime created by RuntimeOrchestrate" }
}
```

`TeamCreate.agents[]` additions:

```json
{
  "provider": { "type": "string" },
  "model": { "type": "string" },
  "required_capabilities": {
    "type": "array",
    "items": { "type": "string" }
  },
  "access_profile": { "type": "string", "enum": ["full", "read-only", "search-only"] },
  "budget_usd": { "type": "number" },
  "runtime_id": { "type": "string" }
}
```

Permission impact:

- `Agent` stays `PermissionLevel::None`
- `TeamCreate` stays `PermissionLevel::Write`

### Execution Model

- Factor the provider-resolution logic out of `run_query_loop()` into a reusable scheduler helper so child agents use the same dispatch path as the root session.
- Replace the direct `AnthropicClient::new(...)` path in `src-rust/crates/query/src/agent_tool.rs` with a provider-aware execution target built from `QueryConfig.provider_registry` and `QueryConfig.model_registry`.
- Partition `ToolContext.cost_tracker` by `agent_id` so the coordinator can enforce per-worker budgets and compare spend across providers.
- Add health-aware fallback: if a requested local endpoint is unavailable, the worker can degrade to another provider that satisfies the same capability set.

## 2. Campaign DAG and Artifact Bus

### Feature Name & Description

Add a real coordinator execution plane for large refactors, legacy migrations, and C/C++ reverse-engineering. The current team model is "spawn N agents, wait, concatenate text". That is too weak for tasks that need staged decomposition, intermediate artifacts, verification gates, and resumability.

The replacement should be a campaign DAG:

- stage 1: symbol extraction, index build, or architectural inventory
- stage 2: parallel worker lanes per subsystem or binary
- stage 3: patch synthesis or decompilation notes
- stage 4: verifier agents and final merge review

Each stage should publish typed artifacts instead of only prose.

### Crate Topology

- Create `src-rust/crates/query/src/campaign.rs`
- Create `src-rust/crates/query/src/artifact_bus.rs`
- Modify `src-rust/crates/query/src/coordinator.rs`
- Modify `src-rust/crates/query/src/lib.rs`
- Modify `src-rust/crates/tools/src/team_tool.rs`
- Modify `src-rust/crates/tools/src/tasks.rs`
- Modify `src-rust/crates/tools/src/lib.rs`
- Modify `src-rust/crates/tui/src/agents_view.rs`
- Modify `src-rust/crates/tui/src/tasks_overlay.rs`

### Tool Interfaces

New tool: `CampaignCreate`

- `PermissionLevel::Write`

```json
{
  "type": "object",
  "properties": {
    "name": { "type": "string" },
    "objective": { "type": "string" },
    "artifact_dir": { "type": "string" },
    "stages": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "id": { "type": "string" },
          "prompt": { "type": "string" },
          "depends_on": { "type": "array", "items": { "type": "string" } },
          "agents": {
            "type": "array",
            "items": {
              "type": "object",
              "properties": {
                "name": { "type": "string" },
                "role": { "type": "string" },
                "provider": { "type": "string" },
                "model": { "type": "string" },
                "tools": { "type": "array", "items": { "type": "string" } },
                "access_profile": { "type": "string" },
                "isolation": { "type": "string", "enum": ["shared", "worktree", "runtime"] }
              },
              "required": ["name"]
            }
          },
          "outputs": { "type": "array", "items": { "type": "string" } }
        },
        "required": ["id", "prompt"]
      }
    }
  },
  "required": ["name", "objective", "stages"]
}
```

New tool: `ArtifactQuery`

- `PermissionLevel::ReadOnly`

```json
{
  "type": "object",
  "properties": {
    "campaign_id": { "type": "string" },
    "stage_id": { "type": "string" },
    "artifact_type": { "type": "string" },
    "producer": { "type": "string" },
    "limit": { "type": "integer" }
  },
  "required": ["campaign_id"]
}
```

### Execution Model

- `CampaignCreate` should register stages in the existing task store and persist campaign state under `.claurst/campaigns/<campaign_id>/`.
- `run_query_loop()` should treat campaign stages as long-lived background children rather than one-shot tool calls.
- `ToolContext` should gain campaign-scoped handles such as `campaign_id`, `stage_id`, and `artifact_bus`.
- For large reverse-engineering tasks, early stages can publish symbol maps, call graphs, or disassembly notes that later workers consume through `ArtifactQuery` instead of repeating expensive Grep/Read loops.

## 3. KAIROS Tick Engine

### Feature Name & Description

Implement the missing always-on subsystem by reusing existing Rust primitives instead of waiting for Anthropic-internal infrastructure. The repo already has the right pieces: background Tokio tasks, cron-style scheduling, command injection, away summaries, and concise output styles. What is missing is the orchestrator that turns those parts into a budgeted, proactive loop.

The OSS version of KAIROS should do four things well:

- run short, budget-capped observation turns in the background
- write append-only daily logs
- watch selected external signals like PR status, CI, or scheduled prompts
- surface only actionable deltas into the foreground session

### Crate Topology

- Create `src-rust/crates/query/src/kairos.rs`
- Modify `src-rust/crates/query/src/lib.rs`
- Modify `src-rust/crates/query/src/cron_scheduler.rs`
- Modify `src-rust/crates/tools/src/remote_trigger.rs`
- Create `src-rust/crates/tools/src/push_notification.rs`
- Create `src-rust/crates/tools/src/send_user_file.rs`
- Create `src-rust/crates/tools/src/subscribe_pr.rs`
- Modify `src-rust/crates/tools/src/lib.rs`
- Modify `src-rust/crates/commands/src/lib.rs`

### Tool Interfaces

New tool: `PushNotification`

- `PermissionLevel::None` for TUI-local notifications

```json
{
  "type": "object",
  "properties": {
    "title": { "type": "string" },
    "body": { "type": "string" },
    "severity": { "type": "string", "enum": ["info", "warning", "critical"] },
    "channel": { "type": "string", "enum": ["tui", "desktop", "log"] }
  },
  "required": ["title", "body"]
}
```

New tool: `SendUserFile`

- `PermissionLevel::ReadOnly`

```json
{
  "type": "object",
  "properties": {
    "path": { "type": "string" },
    "summary": { "type": "string" },
    "tail_lines": { "type": "integer" }
  },
  "required": ["path"]
}
```

New tool: `SubscribePR`

- `PermissionLevel::ReadOnly`

```json
{
  "type": "object",
  "properties": {
    "repo": { "type": "string" },
    "pr_number": { "type": "integer" },
    "poll_interval_sec": { "type": "integer" },
    "filters": {
      "type": "array",
      "items": { "type": "string", "enum": ["checks", "reviews", "comments", "mergeability"] }
    }
  },
  "required": ["repo", "pr_number"]
}
```

### Execution Model

- Start a `kairos.rs` background supervisor from the same runtime that currently starts cron jobs.
- Reuse `run_query_loop()` for each KAIROS tick, but feed it a minimal tool set and force `OutputStyle::Concise`.
- Enforce the spec's short blocking budget by hard-capping wall-clock tick duration and token spend per tick.
- Write daily logs to `~/.claurst/kairos/YYYY-MM-DD.log`.
- Deliver user-facing deltas through `CommandQueue` and the TUI event stream instead of writing into the live conversation immediately.

## 4. ULTRAPLAN-OSS Local Teleport Runtime

### Feature Name & Description

Build an open-source replacement for the missing CCR-backed ULTRAPLAN flow. Claurst already knows how to:

- create isolated worktrees
- override local provider base URLs
- run background agent loops
- serialize and restore session bundles with `/teleport`

What it does not have is a runtime manager that can launch a local or remote execution capsule, bind a model endpoint to it, poll status, and "teleport" the result back into the coordinator. That is the right feature for home-lab orchestration, local model deployments, and heavy long-running reviews from the TUI.

### Crate Topology

- Create `src-rust/crates/query/src/runtime_manager.rs`
- Create `src-rust/crates/query/src/ultraplan.rs`
- Create `src-rust/crates/tools/src/runtime_orchestrator.rs`
- Modify `src-rust/crates/query/src/agent_tool.rs`
- Modify `src-rust/crates/api/src/registry.rs`
- Modify `src-rust/crates/api/src/providers/openai_compat_providers.rs`
- Modify `src-rust/crates/tools/src/lib.rs`
- Modify `src-rust/crates/commands/src/lib.rs`
- Create `src-rust/crates/tui/src/runtime_view.rs`

### Tool Interfaces

New tool: `RuntimeOrchestrate`

- `PermissionLevel::Dangerous`

```json
{
  "type": "object",
  "properties": {
    "action": { "type": "string", "enum": ["create", "start", "stop", "status", "attach", "destroy"] },
    "backend": { "type": "string", "enum": ["worktree", "docker", "podman", "ssh", "libvirt", "ollama", "lmstudio", "llamacpp"] },
    "name": { "type": "string" },
    "working_dir": { "type": "string" },
    "provider_id": { "type": "string" },
    "model": { "type": "string" },
    "api_base": { "type": "string" },
    "image_or_host": { "type": "string" },
    "mounts": { "type": "array", "items": { "type": "string" } },
    "env": { "type": "object", "additionalProperties": { "type": "string" } },
    "start_command": { "type": "string" },
    "approval_scope": { "type": "string" }
  },
  "required": ["action", "backend", "name"]
}
```

Optional management tool: `RuntimeExec`

- `PermissionLevel::Execute`

```json
{
  "type": "object",
  "properties": {
    "runtime_id": { "type": "string" },
    "command": { "type": "string" },
    "cwd": { "type": "string" },
    "timeout_sec": { "type": "integer" }
  },
  "required": ["runtime_id", "command"]
}
```

### Execution Model

- `RuntimeOrchestrate` should register ephemeral runtimes in a shared manager stored off `ToolContext`.
- `run_query_loop()` should be able to resolve `runtime_id` into both a working directory and a provider endpoint before dispatching model requests or tool executions.
- Extend the existing `/teleport` command instead of replacing it. For example, keep `export/import/link` intact and add a new `runtime` subcommand family for ULTRAPLAN-style execution.
- Extend `/ultrareview` to optionally target a runtime so heavyweight scans can run outside the foreground session.

## 5. Semantic and Binary Index Service

### Feature Name & Description

Claurst is still too text-centric for massive legacy trees and reverse-engineering work. `Grep`, `Glob`, `Read`, and `LspTool` are useful, but they force repeated search loops. A background semantic and binary index would let workers ask higher-level questions in one tool call:

- "find callers of `run_query_loop`"
- "list all implementations of trait `Tool`"
- "show vtable-like symbols and imports for this ELF"
- "build a call graph for these C++ translation units"

This matters most for large Rust/C++ monorepos and stripped or partially documented binaries.

### Crate Topology

- Create `src-rust/crates/analysis/`
- Create `src-rust/crates/analysis/src/lib.rs`
- Create `src-rust/crates/analysis/src/index.rs`
- Create `src-rust/crates/analysis/src/binary.rs`
- Create `src-rust/crates/tools/src/semantic_search.rs`
- Create `src-rust/crates/tools/src/binary_inspect.rs`
- Modify `src-rust/crates/tools/src/lsp_tool.rs`
- Modify `src-rust/crates/tools/src/lib.rs`
- Modify `src-rust/crates/query/src/lib.rs`

### Tool Interfaces

New tool: `SemanticSearch`

- `PermissionLevel::ReadOnly`

```json
{
  "type": "object",
  "properties": {
    "query": { "type": "string" },
    "language": { "type": "string" },
    "scope": { "type": "string" },
    "relation": {
      "type": "string",
      "enum": ["definitions", "references", "callers", "callees", "implementations", "types"]
    },
    "limit": { "type": "integer" }
  },
  "required": ["query"]
}
```

New tool: `BinaryInspect`

- `PermissionLevel::Execute`

```json
{
  "type": "object",
  "properties": {
    "path": { "type": "string" },
    "mode": { "type": "string", "enum": ["symbols", "imports", "strings", "callgraph", "decompile_stub"] },
    "function": { "type": "string" },
    "backend": { "type": "string", "enum": ["llvm", "objdump", "radare2", "ghidra-headless"] }
  },
  "required": ["path", "mode"]
}
```

### Execution Model

- Prewarm the index when the repo exceeds a size threshold or when the user starts a campaign/coordinator session.
- Attach an `analysis_cache` handle to `ToolContext` so all workers share the same index instead of rebuilding it.
- Publish symbol maps and binary artifacts into the campaign artifact bus so reverse-engineering becomes staged and resumable rather than conversationally repetitive.

## Recommended Delivery Order

1. Provider-Aware Worker Fabric
2. Campaign DAG and Artifact Bus
3. ULTRAPLAN-OSS Local Teleport Runtime
4. KAIROS Tick Engine
5. Semantic and Binary Index Service

That order front-loads the highest architectural leverage:

- first fix the child-agent/provider mismatch
- then give the coordinator a real execution graph
- then attach runtimes and long-running remote-ish backends
- then add proactive background automation
- then specialize for large codebases and reverse engineering
