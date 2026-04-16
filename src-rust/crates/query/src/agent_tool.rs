// AgentTool: spawn a sub-agent to handle a complex sub-task.
//
// Lives in cc-query (not cc-tools) to avoid a circular dependency:
//   cc-tools would need cc-query, but cc-query already needs cc-tools.
//
// The AgentTool creates a nested query loop with its own context, enabling
// the model to delegate complex work to specialized sub-agents. Each sub-agent:
//   - Runs its own agentic loop
//   - Has access to all tools (except AgentTool itself, preventing infinite recursion)
//   - Returns its final output as the tool result
//
// New capabilities (TS parity):
//   - `isolation: "worktree"` — run the agent in a dedicated git worktree so
//     file edits don't conflict with the parent checkout or sibling agents.
//   - `run_in_background: true` — fire-and-forget; returns agent_id immediately.
//     Use poll_background_agent() to check completion status.

use async_trait::async_trait;
use claurst_core::types::Message;
use claurst_tools::{PermissionLevel, Tool, ToolContext, ToolResult};
use dashmap::DashMap;
use once_cell::sync::Lazy;
use serde::Deserialize;
use serde_json::{json, Value};
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::Arc;
use tokio_util::sync::CancellationToken;
use tracing::{debug, info, warn};

use crate::provider_resolution::{
    resolve_provider_identity, resolve_provider_with_fallback, ExecutionTarget, KNOWN_PROVIDERS,
};
use crate::{run_query_loop, session_budget_for_session, HealthCache, QueryConfig, QueryOutcome};

// ---------------------------------------------------------------------------
// Background agent registry
// ---------------------------------------------------------------------------

/// Registry of in-flight background agents.
/// Maps agent_id -> oneshot receiver that resolves to the agent's final output.
static BACKGROUND_AGENTS: Lazy<DashMap<String, tokio::sync::oneshot::Receiver<String>>> =
    Lazy::new(DashMap::new);

/// Poll a background agent's result.
///
/// Returns `None` if still running, `Some(result_text)` when done (or errored).
/// After returning `Some`, the entry is removed from the registry.
pub fn poll_background_agent(agent_id: &str) -> Option<String> {
    if let Some(mut entry) = BACKGROUND_AGENTS.get_mut(agent_id) {
        match entry.try_recv() {
            Ok(result) => {
                drop(entry);
                BACKGROUND_AGENTS.remove(agent_id);
                Some(result)
            }
            Err(tokio::sync::oneshot::error::TryRecvError::Empty) => None,
            Err(_) => {
                // Sender dropped - treat as agent error/cancellation.
                drop(entry);
                BACKGROUND_AGENTS.remove(agent_id);
                Some("[Agent error or cancelled]".to_string())
            }
        }
    } else {
        None
    }
}

// ---------------------------------------------------------------------------
// Worktree isolation helpers
// ---------------------------------------------------------------------------

fn find_git_root(start: &Path) -> Option<PathBuf> {
    let mut dir = start.to_path_buf();
    loop {
        if dir.join(".git").exists() {
            return Some(dir);
        }
        if !dir.pop() {
            return None;
        }
    }
}

async fn create_worktree(git_root: &Path, agent_id: &str) -> Option<PathBuf> {
    let worktree_dir = std::env::temp_dir().join(format!("claude-agent-{}", agent_id));
    let output = tokio::process::Command::new("git")
        .args([
            "worktree",
            "add",
            "--detach",
            worktree_dir.to_str().unwrap_or_default(),
            "HEAD",
        ])
        .current_dir(git_root)
        .output()
        .await
        .ok()?;
    if output.status.success() {
        Some(worktree_dir)
    } else {
        warn!(
            "git worktree add failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        None
    }
}

async fn remove_worktree(git_root: &Path, worktree_dir: &Path) {
    let _ = tokio::process::Command::new("git")
        .args([
            "worktree",
            "remove",
            "--force",
            worktree_dir.to_str().unwrap_or_default(),
        ])
        .current_dir(git_root)
        .output()
        .await;
}

// ---------------------------------------------------------------------------
// AgentTool
// ---------------------------------------------------------------------------

pub struct AgentTool;

// D1-safe interim fallback for spawned child agents; this is not the final
// parent/child max_tokens policy.
const CHILD_AGENT_FALLBACK_MAX_TOKENS: u32 = 4_096;
pub(crate) const TOOL_OBSERVABILITY_EVENTS_KEY: &str = "query_observability_events";
pub(crate) const TEAM_RUNNER_OBSERVABILITY_PREFIX: &str = "\n\n[[CLAURST_QUERY_OBS:";
pub(crate) const TEAM_RUNNER_OBSERVABILITY_SUFFIX: &str = "]]";

fn inherited_session_budget(session_id: &str) -> Option<Arc<crate::SessionBudget>> {
    session_budget_for_session(session_id)
}

fn inherited_child_cancel_token(
    session_budget: Option<&Arc<crate::SessionBudget>>,
) -> CancellationToken {
    session_budget
        .map(|budget| budget.child_cancel_token())
        .unwrap_or_default()
}

fn child_session_budget(
    inherited_budget: Option<Arc<crate::SessionBudget>>,
    budget_usd: Option<f64>,
) -> Option<Arc<crate::SessionBudget>> {
    match (inherited_budget, budget_usd) {
        (Some(parent_budget), Some(usd)) => Some(Arc::new(crate::SessionBudget::child_scope(
            parent_budget,
            usd,
        ))),
        (None, Some(usd)) => Some(Arc::new(crate::SessionBudget::new(usd))),
        (inherited_budget, None) => inherited_budget,
    }
}

fn worker_provider_resolved_event(
    agent_id: &str,
    requested_provider_id: &str,
    requested_model_id: &str,
    target: &ExecutionTarget,
) -> Value {
    json!({
        "type": "worker_provider_resolved",
        "agent_id": agent_id,
        "provider_id": target.provider_id,
        "model_id": target.model_id,
        "was_fallback": target.provider_id != requested_provider_id
            || target.model_id != requested_model_id,
    })
}

fn worker_budget_exceeded_event(
    agent_id: &str,
    session_budget: Option<&Arc<crate::SessionBudget>>,
    budget_usd: Option<f64>,
) -> Option<Value> {
    let session_budget = session_budget?;
    budget_usd?;

    if session_budget.is_limit_exceeded() {
        Some(json!({
            "type": "worker_budget_exceeded",
            "agent_id": agent_id,
            "cost_usd": session_budget.spent_usd(),
            "limit_usd": session_budget.limit_usd(),
        }))
    } else {
        None
    }
}

fn build_observability_metadata(events: Vec<Value>) -> Option<Value> {
    if events.is_empty() {
        None
    } else {
        Some(json!({
            TOOL_OBSERVABILITY_EVENTS_KEY: events,
        }))
    }
}

fn attach_observability(mut result: ToolResult, events: Vec<Value>) -> ToolResult {
    if let Some(metadata) = build_observability_metadata(events) {
        result = result.with_metadata(metadata);
    }
    result
}

fn encode_team_runner_observability(content: String, events: Vec<Value>) -> String {
    match build_observability_metadata(events) {
        Some(metadata) => format!(
            "{}{}{}{}",
            content, TEAM_RUNNER_OBSERVABILITY_PREFIX, metadata, TEAM_RUNNER_OBSERVABILITY_SUFFIX
        ),
        None => content,
    }
}

#[derive(Debug, Deserialize)]
struct AgentInput {
    /// Short description of the agent's task (used for logging).
    description: String,
    /// The complete task prompt to send as the first user message.
    prompt: String,
    /// Optional: which tools to make available (defaults to all minus AgentTool).
    #[serde(default)]
    tools: Option<Vec<String>>,
    /// Optional: system prompt override for the sub-agent.
    #[serde(default)]
    system_prompt: Option<String>,
    /// Optional: max turns for the sub-agent (default 10).
    #[serde(default)]
    max_turns: Option<u32>,
    /// Optional: max tokens override for the child query loop.
    #[serde(default)]
    max_tokens: Option<u32>,
    /// Optional: model override for this sub-agent.
    #[serde(default)]
    model: Option<String>,
    /// Optional: explicit provider override (e.g., "openai", "google").
    #[serde(default)]
    provider: Option<String>,
    /// Optional: allow same-domain provider fallback for this sub-agent.
    #[serde(default)]
    allow_fallback: Option<bool>,
    /// Optional: child-local cumulative USD cap for this sub-agent subtree.
    #[serde(default)]
    budget_usd: Option<f64>,
    /// Set to "worktree" to run the agent in an isolated git worktree.
    /// Omit (or set to null) for shared working directory.
    #[serde(default)]
    isolation: Option<String>,
    /// If true, start the agent in the background and return agent_id immediately.
    /// Default: false (wait for completion).
    #[serde(default)]
    run_in_background: bool,
}

#[async_trait]
impl Tool for AgentTool {
    fn name(&self) -> &str {
        claurst_core::constants::TOOL_NAME_AGENT
    }

    fn description(&self) -> &str {
        "Launch a new agent to handle complex, multi-step tasks autonomously. \
         The agent runs its own agentic loop with access to tools and returns \
         its final result. Use this to delegate sub-tasks, run parallel \
         workstreams, or handle tasks that require many tool calls."
    }

    fn permission_level(&self) -> PermissionLevel {
        // The agent inherits parent permissions; no extra level required.
        PermissionLevel::None
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "description": {
                    "type": "string",
                    "description": "Short description of the agent's task (3-5 words)"
                },
                "prompt": {
                    "type": "string",
                    "description": "The complete task for the agent to perform"
                },
                "tools": {
                    "type": "array",
                    "items": { "type": "string" },
                    "description": "List of tool names to make available. Defaults to all tools."
                },
                "system_prompt": {
                    "type": "string",
                    "description": "Optional system prompt override for the sub-agent"
                },
                "max_turns": {
                    "type": "number",
                    "description": "Maximum number of turns for the sub-agent (default 10)"
                },
                "max_tokens": {
                    "type": "integer",
                    "description": "Optional max tokens override for the child agent. Defaults to 4096 when omitted."
                },
                "model": {
                    "type": "string",
                    "description": "Optional model to use for this agent"
                },
                "provider": {
                    "type": "string",
                    "description": "Explicit provider to use for this agent (e.g., 'openai', 'google'). When omitted, inherits from parent session."
                },
                "allow_fallback": {
                    "type": "boolean",
                    "description": "Allow same-domain provider fallback for this child agent. Defaults to false when omitted."
                },
                "budget_usd": {
                    "type": "number",
                    "description": "Optional cumulative USD cap for this child subtree. When omitted, the child only inherits the parent shared session budget."
                },
                "isolation": {
                    "type": "string",
                    "enum": ["worktree"],
                    "description": "Set to \"worktree\" to run the agent in an isolated git worktree. \
                                    Prevents file-edit conflicts when multiple agents run in parallel."
                },
                "run_in_background": {
                    "type": "boolean",
                    "description": "If true, the agent starts immediately and this call returns an \
                                    agent_id without waiting for completion. Poll with poll_background_agent \
                                    to retrieve the result. Default: false."
                }
            },
            "required": ["description", "prompt"]
        })
    }

    async fn execute(&self, input: Value, ctx: &ToolContext) -> ToolResult {
        let params: AgentInput = match serde_json::from_value(input) {
            Ok(p) => p,
            Err(e) => return ToolResult::error(format!("Invalid input: {}", e)),
        };

        info!(description = %params.description, "Spawning sub-agent");

        // Build the tool list for the sub-agent.
        // Always exclude AgentTool itself to prevent unbounded recursion.
        let all = claurst_tools::all_tools();
        let agent_tools: Vec<Box<dyn Tool>> = if let Some(ref allowed) = params.tools {
            all.into_iter()
                .filter(|t| allowed.contains(&t.name().to_string()))
                .collect()
        } else {
            all.into_iter()
                .filter(|t| t.name() != claurst_core::constants::TOOL_NAME_AGENT)
                .collect()
        };

        // Resolve model: explicit override > default.
        let model = params
            .model
            .filter(|m| !m.is_empty())
            .unwrap_or_else(|| claurst_core::constants::DEFAULT_MODEL.to_string());

        let explicit_provider = params.provider.as_deref().filter(|p| !p.is_empty());
        let parent_provider = ctx.config.provider.as_deref().filter(|p| !p.is_empty());
        let model_has_provider_prefix = matches!(
            model.split_once('/'),
            Some((provider_id, _)) if KNOWN_PROVIDERS.contains(&provider_id)
        );
        let provider_hint = if explicit_provider.is_some() {
            explicit_provider
        } else if model_has_provider_prefix {
            None
        } else {
            parent_provider
        };
        let requested_identity =
            match resolve_provider_identity(provider_hint, &model, ctx.model_registry.as_deref()) {
                Ok(identity) => identity,
                Err(e) => return ToolResult::error(format!("Provider resolution failed: {}", e)),
            };

        let registry = match ctx.provider_registry.as_ref() {
            Some(registry) => registry,
            None => {
                return ToolResult::error(
                    "Cannot spawn sub-agent: provider_registry not available in ToolContext"
                        .to_string(),
                )
            }
        };

        let allow_fallback = params.allow_fallback.unwrap_or(false);
        let health_cache = HealthCache::new();
        let target = match resolve_provider_with_fallback(
            provider_hint,
            &model,
            ctx.model_registry.as_deref(),
            registry,
            &ctx.config.provider_configs,
            &health_cache,
            allow_fallback,
        )
        .await
        {
            Ok(target) => target,
            Err(e) => return ToolResult::error(format!("Provider resolution failed: {}", e)),
        };

        let system_prompt = params.system_prompt.unwrap_or_else(|| {
            let mut prompt = "You are a specialized AI agent helping with a specific sub-task. \
             Complete the task thoroughly and return your findings."
                .to_string();

            // Append plugin-contributed agent definitions so the sub-agent
            // is aware of any specialised agents declared by plugins.
            if let Some(registry) = claurst_plugins::global_plugin_registry() {
                let mut agent_defs = String::new();
                for agent_dir in registry.all_agent_paths() {
                    if let Ok(entries) = std::fs::read_dir(&agent_dir) {
                        for entry in entries.flatten() {
                            let p = entry.path();
                            if p.extension().is_some_and(|e| e == "md") {
                                if let Ok(content) = std::fs::read_to_string(&p) {
                                    let name =
                                        p.file_stem().and_then(|s| s.to_str()).unwrap_or("agent");
                                    agent_defs.push_str(&format!(
                                        "\n\n## Agent: {}\n{}",
                                        name,
                                        content.trim()
                                    ));
                                }
                            }
                        }
                    }
                }
                if !agent_defs.is_empty() {
                    prompt.push_str("\n\nThe following specialized agents are available:");
                    prompt.push_str(&agent_defs);
                }
            }

            prompt
        });

        // -----------------------------------------------------------------------
        // Determine working directory - optionally isolate in a git worktree.
        // -----------------------------------------------------------------------
        let use_isolation = params.isolation.as_deref() == Some("worktree");
        let agent_id = uuid::Uuid::new_v4().to_string();
        let provider_event = worker_provider_resolved_event(
            &agent_id,
            &requested_identity.provider_id,
            &requested_identity.model_id,
            &target,
        );

        let (working_dir_str, worktree_path, git_root): (String, Option<PathBuf>, Option<PathBuf>) =
            if use_isolation {
                let git_root = find_git_root(&ctx.working_dir);
                if let Some(ref root) = git_root {
                    if let Some(wt) = create_worktree(root, &agent_id).await {
                        let wd = wt.display().to_string();
                        (wd, Some(wt), git_root)
                    } else {
                        warn!(
                            agent_id = %agent_id,
                            "Worktree creation failed; running agent in shared working directory"
                        );
                        (ctx.working_dir.display().to_string(), None, None)
                    }
                } else {
                    warn!(
                        agent_id = %agent_id,
                        "No git root found; isolation=worktree ignored"
                    );
                    (ctx.working_dir.display().to_string(), None, None)
                }
            } else {
                (ctx.working_dir.display().to_string(), None, None)
            };

        let session_budget =
            child_session_budget(inherited_session_budget(&ctx.session_id), params.budget_usd);

        let query_config = QueryConfig {
            model: target.model_id.clone(),
            max_tokens: params.max_tokens.unwrap_or(CHILD_AGENT_FALLBACK_MAX_TOKENS),
            max_turns: params.max_turns.unwrap_or(10),
            system_prompt: Some(system_prompt),
            append_system_prompt: None,
            output_style: ctx.config.effective_output_style(),
            output_style_prompt: ctx.config.resolve_output_style_prompt(),
            working_directory: Some(working_dir_str),
            thinking_budget: None,
            temperature: None,
            tool_result_budget: 50_000,
            effort_level: None,
            command_queue: None,
            skill_index: None,
            session_budget: session_budget.clone(),
            max_budget_usd: None,
            fallback_model: None,
            provider_registry: Some(registry.clone()),
            agent_name: None,
            agent_definition: None,
            model_registry: ctx.model_registry.clone(),
        };

        let mut foreground_ctx = ctx.clone();
        foreground_ctx.config.provider = Some(target.provider_id.clone());
        // -----------------------------------------------------------------------
        // Background mode: spawn and return agent_id immediately.
        // -----------------------------------------------------------------------
        if params.run_in_background {
            let (tx, rx) = tokio::sync::oneshot::channel::<String>();
            BACKGROUND_AGENTS.insert(agent_id.clone(), rx);

            // Re-create the tool list inside the closure so it is owned and Send.
            let agent_tools_bg: Vec<Box<dyn Tool>> = claurst_tools::all_tools()
                .into_iter()
                .filter(|t| t.name() != claurst_core::constants::TOOL_NAME_AGENT)
                .collect();

            let ctx_bg = foreground_ctx.clone();
            let config_bg = query_config.clone();
            let cost_tracker_bg = ctx.cost_tracker.clone();
            let description_bg = params.description.clone();
            let prompt_bg = params.prompt.clone();
            let agent_id_bg = agent_id.clone();

            tokio::spawn(async move {
                let cancel = inherited_child_cancel_token(config_bg.session_budget.as_ref());
                let mut messages = vec![Message::user(prompt_bg)];
                let outcome = run_query_loop(
                    None,
                    &mut messages,
                    &agent_tools_bg,
                    &ctx_bg,
                    &config_bg,
                    cost_tracker_bg,
                    None,
                    cancel,
                    None,
                )
                .await;

                // Cleanup worktree if one was created.
                if let (Some(root), Some(wt)) = (git_root, worktree_path) {
                    remove_worktree(&root, &wt).await;
                }

                let result_text = format_outcome(outcome);
                debug!(
                    agent_id = %agent_id_bg,
                    description = %description_bg,
                    "Background agent completed"
                );
                let _ = tx.send(result_text);
            });

            return attach_observability(
                ToolResult::success(
                serde_json::json!({
                    "agent_id": agent_id,
                    "status": "running",
                    "message": format!(
                        "Agent '{}' started in background. Use poll_background_agent with agent_id '{}' to check status.",
                        params.description, agent_id
                    )
                })
                .to_string(),
                ),
                vec![provider_event],
            );
        }

        // -----------------------------------------------------------------------
        // Synchronous mode: run the sub-agent loop and wait for completion.
        // -----------------------------------------------------------------------
        let mut messages = vec![Message::user(params.prompt)];
        let cancel = inherited_child_cancel_token(query_config.session_budget.as_ref());

        let outcome = run_query_loop(
            None,
            &mut messages,
            &agent_tools,
            &foreground_ctx,
            &query_config,
            ctx.cost_tracker.clone(),
            None, // no event forwarding for sub-agents
            cancel,
            None, // no pending message queue for sub-agents
        )
        .await;

        // Cleanup worktree if one was created.
        if let (Some(root), Some(wt)) = (git_root, worktree_path) {
            remove_worktree(&root, &wt).await;
        }

        let mut observability_events = vec![provider_event];
        if let Some(event) =
            worker_budget_exceeded_event(&agent_id, session_budget.as_ref(), params.budget_usd)
        {
            observability_events.push(event);
        }

        match outcome {
            QueryOutcome::EndTurn { message, usage } => {
                let text = message.get_all_text();
                debug!(
                    description = %params.description,
                    output_tokens = usage.output_tokens,
                    "Sub-agent completed"
                );
                attach_observability(ToolResult::success(text), observability_events)
            }
            QueryOutcome::MaxTokens {
                partial_message, ..
            } => {
                let text = partial_message.get_all_text();
                attach_observability(
                    ToolResult::success(format!("{}\n\n[Note: Agent hit max_tokens limit]", text)),
                    observability_events,
                )
            }
            QueryOutcome::Cancelled => attach_observability(
                ToolResult::error("Sub-agent was cancelled".to_string()),
                observability_events,
            ),
            QueryOutcome::Error(e) => attach_observability(
                ToolResult::error(format!("Sub-agent error: {}", e)),
                observability_events,
            ),
            QueryOutcome::BudgetExceeded {
                cost_usd,
                limit_usd,
            } => attach_observability(
                ToolResult::error(format!(
                    "Sub-agent stopped: budget ${:.4} exceeded (limit ${:.4})",
                    cost_usd, limit_usd
                )),
                observability_events,
            ),
        }
    }
}

// ---------------------------------------------------------------------------
// Helper: convert a QueryOutcome into a result string for background agents
// ---------------------------------------------------------------------------

fn format_outcome(outcome: QueryOutcome) -> String {
    match outcome {
        QueryOutcome::EndTurn { message, .. } => message.get_all_text(),
        QueryOutcome::MaxTokens {
            partial_message, ..
        } => format!(
            "{}\n\n[Note: Agent hit max_tokens limit]",
            partial_message.get_all_text()
        ),
        QueryOutcome::Cancelled => "[Agent was cancelled]".to_string(),
        QueryOutcome::Error(e) => format!("[Agent error: {}]", e),
        QueryOutcome::BudgetExceeded {
            cost_usd,
            limit_usd,
        } => format!(
            "[Agent stopped: budget ${:.4} exceeded (limit ${:.4})]",
            cost_usd, limit_usd
        ),
    }
}

// ---------------------------------------------------------------------------
// Team swarm runner injection
// ---------------------------------------------------------------------------
//
// Called once at process startup (e.g. from main.rs) to inject a real agent
// runner into cc-tools so that TeamCreateTool can spawn sub-agents via
// run_query_loop without creating a circular crate dependency.

/// Register the cc-query-backed agent runner with cc-tools.
///
/// After this call, `TeamCreateTool` will actually invoke `run_query_loop` for
/// each agent instead of returning stub output.
///
/// # Panics
/// Panics if the runner was already registered.
pub fn init_team_swarm_runner() {
    let runner: claurst_tools::AgentRunFn =
        Arc::new(|params: claurst_tools::team_tool::AgentRunParams| {
            // We must return a Pin<Box<dyn Future<...> + Send>>.
            Box::pin(async move {
                let claurst_tools::team_tool::AgentRunParams {
                    description,
                    prompt,
                    tools,
                    system_prompt,
                    max_turns,
                    max_tokens_override,
                    allow_fallback,
                    budget_usd,
                    ctx,
                    provider_override,
                    model_override,
                } = params;

                let registry = match ctx.provider_registry.clone() {
                    Some(registry) => registry,
                    None => {
                        return format!(
                            "[Agent '{}' failed: provider_registry not available in ToolContext]",
                            description
                        )
                    }
                };

                // Build the tool list, filtering to the allowlist if provided.
                let all = claurst_tools::all_tools();
                let agent_tools: Vec<Box<dyn claurst_tools::Tool>> =
                    if let Some(ref allowed) = tools {
                        all.into_iter()
                            .filter(|t| allowed.contains(&t.name().to_string()))
                            .collect()
                    } else {
                        all.into_iter()
                            .filter(|t| t.name() != claurst_core::constants::TOOL_NAME_AGENT)
                            .collect()
                    };

                let model = model_override
                    .filter(|m| !m.is_empty())
                    .unwrap_or_else(|| claurst_core::constants::DEFAULT_MODEL.to_string());
                let requested_identity = match resolve_provider_identity(
                    provider_override.as_deref().filter(|p| !p.is_empty()),
                    &model,
                    ctx.model_registry.as_deref(),
                ) {
                    Ok(identity) => identity,
                    Err(e) => {
                        return format!(
                            "[Agent '{}' provider resolution failed: {}]",
                            description, e
                        )
                    }
                };

                let health_cache = HealthCache::new();
                let target = match resolve_provider_with_fallback(
                    provider_override.as_deref().filter(|p| !p.is_empty()),
                    &model,
                    ctx.model_registry.as_deref(),
                    &registry,
                    &ctx.config.provider_configs,
                    &health_cache,
                    allow_fallback,
                )
                .await
                {
                    Ok(target) => target,
                    Err(e) => {
                        return format!(
                            "[Agent '{}' provider resolution failed: {}]",
                            description, e
                        )
                    }
                };
                let provider_event = worker_provider_resolved_event(
                    &description,
                    &requested_identity.provider_id,
                    &requested_identity.model_id,
                    &target,
                );

                let system_prompt = system_prompt.unwrap_or_else(|| {
                    "You are a specialized AI agent helping with a specific sub-task. \
                     Complete the task thoroughly and return your findings."
                        .to_string()
                });

                let session_budget =
                    child_session_budget(inherited_session_budget(&ctx.session_id), budget_usd);

                let query_config = QueryConfig {
                    model: target.model_id.clone(),
                    max_tokens: max_tokens_override.unwrap_or(CHILD_AGENT_FALLBACK_MAX_TOKENS),
                    max_turns: max_turns.unwrap_or(10),
                    system_prompt: Some(system_prompt),
                    working_directory: Some(ctx.working_dir.display().to_string()),
                    output_style: ctx.config.effective_output_style(),
                    output_style_prompt: ctx.config.resolve_output_style_prompt(),
                    session_budget: session_budget.clone(),
                    provider_registry: Some(registry.clone()),
                    model_registry: ctx.model_registry.clone(),
                    ..Default::default()
                };

                let mut runner_ctx = (*ctx).clone();
                runner_ctx.config.provider = Some(target.provider_id.clone());

                let cancel = inherited_child_cancel_token(query_config.session_budget.as_ref());
                let mut messages = vec![Message::user(prompt)];
                let outcome = run_query_loop(
                    None,
                    &mut messages,
                    &agent_tools,
                    &runner_ctx,
                    &query_config,
                    ctx.cost_tracker.clone(),
                    None,
                    cancel,
                    None,
                )
                .await;

                let mut observability_events = vec![provider_event];
                if let Some(event) =
                    worker_budget_exceeded_event(&description, session_budget.as_ref(), budget_usd)
                {
                    observability_events.push(event);
                }

                encode_team_runner_observability(format_outcome(outcome), observability_events)
            }) as Pin<Box<dyn std::future::Future<Output = String> + Send>>
        });

    claurst_tools::register_agent_runner(runner);
}

#[cfg(test)]
#[path = "agent_tool_tests.rs"]
mod tests;
