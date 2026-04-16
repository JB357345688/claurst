use super::AgentTool;
use async_trait::async_trait;
use claurst_api::{
    LlmProvider, ModelRegistry, ProviderCapabilities, ProviderError, ProviderRegistry,
    ProviderStatus, StreamEvent, SystemPromptStyle,
};
use claurst_core::config::{Config, PermissionMode};
use claurst_core::permissions::AutoPermissionHandler;
use claurst_core::{ContentBlock, ProviderId, UsageInfo};
use claurst_tools::{TeamCreateTool, Tool, ToolContext};
use futures::stream;
use parking_lot::Mutex;
use serde_json::{json, Value};
use std::ffi::{OsStr, OsString};
use std::pin::Pin;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, OnceLock};
use tempfile::TempDir;

fn make_tool_context(
    provider_registry: Option<Arc<ProviderRegistry>>,
    parent_provider: Option<&str>,
) -> ToolContext {
    make_tool_context_with_model_registry(provider_registry, None, parent_provider)
}

fn make_tool_context_with_model_registry(
    provider_registry: Option<Arc<ProviderRegistry>>,
    model_registry: Option<Arc<ModelRegistry>>,
    parent_provider: Option<&str>,
) -> ToolContext {
    let config = Config {
        provider: parent_provider.map(str::to_string),
        ..Default::default()
    };

    ToolContext {
        working_dir: std::env::temp_dir(),
        permission_mode: PermissionMode::Default,
        permission_handler: Arc::new(AutoPermissionHandler {
            mode: PermissionMode::Default,
        }),
        cost_tracker: claurst_core::cost::CostTracker::new(),
        session_id: "agent-tool-test".to_string(),
        file_history: Arc::new(Mutex::new(claurst_core::file_history::FileHistory::new())),
        current_turn: Arc::new(AtomicUsize::new(0)),
        non_interactive: true,
        mcp_manager: None,
        config,
        provider_registry,
        model_registry,
    }
}

struct TrackingStreamingProvider {
    id: ProviderId,
    name: String,
    message_id: String,
    model_name: String,
    invocations: Arc<AtomicUsize>,
    health_checks: Arc<AtomicUsize>,
    observed_max_tokens: Arc<Mutex<Vec<u32>>>,
    response_text: String,
}

impl TrackingStreamingProvider {
    fn new(
        provider_id: &str,
        provider_name: &str,
        message_id: &str,
        model_name: &str,
        invocations: Arc<AtomicUsize>,
        observed_max_tokens: Arc<Mutex<Vec<u32>>>,
        response_text: impl Into<String>,
    ) -> Self {
        Self::new_with_health_counter(
            provider_id,
            provider_name,
            message_id,
            model_name,
            invocations,
            Arc::new(AtomicUsize::new(0)),
            observed_max_tokens,
            response_text,
        )
    }

    fn new_with_health_counter(
        provider_id: &str,
        provider_name: &str,
        message_id: &str,
        model_name: &str,
        invocations: Arc<AtomicUsize>,
        health_checks: Arc<AtomicUsize>,
        observed_max_tokens: Arc<Mutex<Vec<u32>>>,
        response_text: impl Into<String>,
    ) -> Self {
        Self {
            id: ProviderId::new(provider_id),
            name: provider_name.to_string(),
            message_id: message_id.to_string(),
            model_name: model_name.to_string(),
            invocations,
            health_checks,
            observed_max_tokens,
            response_text: response_text.into(),
        }
    }
}

#[async_trait]
impl LlmProvider for TrackingStreamingProvider {
    fn id(&self) -> &ProviderId {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    async fn create_message(
        &self,
        _request: claurst_api::ProviderRequest,
    ) -> Result<claurst_api::ProviderResponse, ProviderError> {
        panic!("create_message is not used by agent_tool streaming dispatch")
    }

    async fn create_message_stream(
        &self,
        request: claurst_api::ProviderRequest,
    ) -> Result<
        Pin<Box<dyn futures::Stream<Item = Result<StreamEvent, ProviderError>> + Send>>,
        ProviderError,
    > {
        self.invocations.fetch_add(1, Ordering::SeqCst);
        self.observed_max_tokens.lock().push(request.max_tokens);
        Ok(Box::pin(stream::iter(vec![
            Ok(StreamEvent::MessageStart {
                id: self.message_id.clone(),
                model: self.model_name.clone(),
                usage: UsageInfo::default(),
            }),
            Ok(StreamEvent::ContentBlockStart {
                index: 0,
                content_block: ContentBlock::Text {
                    text: String::new(),
                },
            }),
            Ok(StreamEvent::TextDelta {
                index: 0,
                text: self.response_text.clone(),
            }),
            Ok(StreamEvent::ContentBlockStop { index: 0 }),
            Ok(StreamEvent::MessageDelta {
                stop_reason: Some(claurst_api::provider_types::StopReason::EndTurn),
                usage: Some(UsageInfo {
                    output_tokens: 1,
                    ..UsageInfo::default()
                }),
            }),
            Ok(StreamEvent::MessageStop),
        ])))
    }

    async fn list_models(&self) -> Result<Vec<claurst_api::ModelInfo>, ProviderError> {
        Ok(vec![])
    }

    async fn health_check(&self) -> Result<ProviderStatus, ProviderError> {
        self.health_checks.fetch_add(1, Ordering::SeqCst);
        Ok(ProviderStatus::Healthy)
    }

    fn capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities {
            streaming: true,
            tool_calling: true,
            thinking: false,
            image_input: false,
            pdf_input: false,
            audio_input: false,
            video_input: false,
            caching: false,
            structured_output: false,
            system_prompt_style: SystemPromptStyle::SystemMessage,
        }
    }
}

fn make_tracking_openai_registry(response_text: &str) -> (Arc<ProviderRegistry>, Arc<AtomicUsize>) {
    let (registry, invocations, _) = make_tracking_openai_registry_with_tokens(response_text);
    (registry, invocations)
}

fn make_tracking_openai_registry_with_health(
    response_text: &str,
) -> (Arc<ProviderRegistry>, Arc<AtomicUsize>, Arc<AtomicUsize>) {
    let invocations = Arc::new(AtomicUsize::new(0));
    let health_checks = Arc::new(AtomicUsize::new(0));
    let observed_max_tokens = Arc::new(Mutex::new(Vec::new()));
    let mut registry = ProviderRegistry::new();
    registry.register(Arc::new(
        TrackingStreamingProvider::new_with_health_counter(
            "openai",
            "OpenAI",
            "tracking-openai-message",
            "gpt-4o",
            invocations.clone(),
            health_checks.clone(),
            observed_max_tokens,
            response_text,
        ),
    ));
    (Arc::new(registry), invocations, health_checks)
}

fn make_tracking_openai_registry_with_tokens(
    response_text: &str,
) -> (
    Arc<ProviderRegistry>,
    Arc<AtomicUsize>,
    Arc<Mutex<Vec<u32>>>,
) {
    let invocations = Arc::new(AtomicUsize::new(0));
    let observed_max_tokens = Arc::new(Mutex::new(Vec::new()));
    let mut registry = ProviderRegistry::new();
    registry.register(Arc::new(TrackingStreamingProvider::new(
        "openai",
        "OpenAI",
        "tracking-openai-message",
        "gpt-4o",
        invocations.clone(),
        observed_max_tokens.clone(),
        response_text,
    )));
    (Arc::new(registry), invocations, observed_max_tokens)
}

type MixedTrackingRegistry = (
    Arc<ProviderRegistry>,
    Arc<AtomicUsize>,
    Arc<AtomicUsize>,
    Arc<Mutex<Vec<u32>>>,
    Arc<Mutex<Vec<u32>>>,
);

fn make_mixed_tracking_registry(
    openai_response_text: &str,
    google_response_text: &str,
) -> MixedTrackingRegistry {
    let openai_invocations = Arc::new(AtomicUsize::new(0));
    let google_invocations = Arc::new(AtomicUsize::new(0));
    let openai_max_tokens = Arc::new(Mutex::new(Vec::new()));
    let google_max_tokens = Arc::new(Mutex::new(Vec::new()));
    let mut registry = ProviderRegistry::new();
    registry.register(Arc::new(TrackingStreamingProvider::new(
        "openai",
        "OpenAI",
        "tracking-openai-message",
        "gpt-4o",
        openai_invocations.clone(),
        openai_max_tokens.clone(),
        openai_response_text,
    )));
    registry.register(Arc::new(TrackingStreamingProvider::new(
        "google",
        "Google",
        "tracking-google-message",
        "gemini-2.5-flash",
        google_invocations.clone(),
        google_max_tokens.clone(),
        google_response_text,
    )));
    (
        Arc::new(registry),
        openai_invocations,
        google_invocations,
        openai_max_tokens,
        google_max_tokens,
    )
}

struct EnvGuard {
    key: &'static str,
    original: Option<OsString>,
}

impl EnvGuard {
    fn set(key: &'static str, value: Option<&str>) -> Self {
        let original = std::env::var_os(key);
        match value {
            Some(value) => std::env::set_var(key, value),
            None => std::env::remove_var(key),
        }
        Self { key, original }
    }

    fn set_os(key: &'static str, value: Option<&OsStr>) -> Self {
        let original = std::env::var_os(key);
        match value {
            Some(value) => std::env::set_var(key, value),
            None => std::env::remove_var(key),
        }
        Self { key, original }
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        match &self.original {
            Some(value) => std::env::set_var(self.key, value),
            None => std::env::remove_var(self.key),
        }
    }
}

fn with_isolated_provider_auth<T>(f: impl FnOnce() -> T) -> T {
    let _lock = crate::provider_auth_test_lock().lock().unwrap();
    let home = TempDir::new().unwrap();
    let _home = EnvGuard::set_os("HOME", Some(home.path().as_os_str()));
    let _anthropic = EnvGuard::set("ANTHROPIC_API_KEY", None);
    let _openai = EnvGuard::set("OPENAI_API_KEY", None);
    let _google = EnvGuard::set("GOOGLE_API_KEY", None);
    f()
}

fn run_agent_tool(input: Value, ctx: &ToolContext) -> claurst_tools::ToolResult {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(AgentTool.execute(input, ctx))
}

fn init_team_swarm_runner_once() {
    static INIT: OnceLock<()> = OnceLock::new();
    INIT.get_or_init(super::init_team_swarm_runner);
}

fn run_team_create_tool(input: Value, ctx: &ToolContext) -> claurst_tools::ToolResult {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(TeamCreateTool.execute(input, ctx))
}

fn split_encoded_team_output(output: &str) -> (String, Value) {
    let start = output
        .rfind(super::TEAM_RUNNER_OBSERVABILITY_PREFIX)
        .expect("team output should include query observability suffix");
    let payload_start = start + super::TEAM_RUNNER_OBSERVABILITY_PREFIX.len();
    let payload_end = output.len() - super::TEAM_RUNNER_OBSERVABILITY_SUFFIX.len();

    (
        output[..start].to_string(),
        serde_json::from_str(&output[payload_start..payload_end])
            .expect("team observability payload should be valid JSON"),
    )
}

#[test]
fn agent_tool_errors_when_provider_registry_missing() {
    let ctx = make_tool_context(None, None);

    let result = run_agent_tool(
        json!({
            "description": "missing-registry",
            "prompt": "verify missing registry"
        }),
        &ctx,
    );

    assert!(result.is_error);
    assert!(
        result
            .content
            .contains("provider_registry not available in ToolContext"),
        "unexpected error: {}",
        result.content
    );
}

#[test]
fn agent_explicit_provider_routes_to_openai_provider() {
    with_isolated_provider_auth(|| {
        let sentinel = "openai provider sentinel";
        let (registry, invocations) = make_tracking_openai_registry(sentinel);
        let ctx = make_tool_context(Some(registry), None);

        let result = run_agent_tool(
            json!({
                "description": "explicit-provider",
                "prompt": "explicit provider success",
                "provider": "openai",
                "model": "gpt-4o",
                "max_turns": 1
            }),
            &ctx,
        );

        assert!(!result.is_error, "unexpected error: {}", result.content);
        assert_eq!(invocations.load(Ordering::SeqCst), 1);
        assert_eq!(result.content, sentinel);
    });
}

#[test]
fn agent_parent_inherits_provider_openai_dispatch() {
    with_isolated_provider_auth(|| {
        let sentinel = "inherited openai provider sentinel";
        let (registry, invocations) = make_tracking_openai_registry(sentinel);
        let ctx = make_tool_context(Some(registry), Some("openai"));

        let result = run_agent_tool(
            json!({
                "description": "parent-provider",
                "prompt": "parent provider success",
                "max_turns": 1
            }),
            &ctx,
        );

        assert!(!result.is_error, "unexpected error: {}", result.content);
        assert_eq!(invocations.load(Ordering::SeqCst), 1);
        assert_eq!(result.content, sentinel);
    });
}

#[test]
fn agent_tool_respects_max_tokens_override() {
    with_isolated_provider_auth(|| {
        let sentinel = "max tokens sentinel";
        let (registry, invocations, observed_max_tokens) =
            make_tracking_openai_registry_with_tokens(sentinel);
        let ctx = make_tool_context(Some(registry), None);

        let result = run_agent_tool(
            json!({
                "description": "max-tokens-override",
                "prompt": "use the provided max token limit",
                "provider": "openai",
                "model": "gpt-4o",
                "max_turns": 1,
                "max_tokens": 321
            }),
            &ctx,
        );

        assert!(!result.is_error, "unexpected error: {}", result.content);
        assert_eq!(invocations.load(Ordering::SeqCst), 1);
        assert_eq!(&*observed_max_tokens.lock(), &[321]);
    });
}

#[test]
fn agent_tool_allow_fallback_uses_same_domain_provider() {
    with_isolated_provider_auth(|| {
        let sentinel = "same-domain fallback sentinel";
        let (registry, google_invocations, _) = make_tracking_openai_registry_with_tokens(sentinel);
        let ctx = make_tool_context_with_model_registry(
            Some(registry),
            Some(Arc::new(ModelRegistry::new())),
            None,
        );

        let result = run_agent_tool(
            json!({
                "description": "same-domain-fallback",
                "prompt": "fallback to another cloud provider",
                "provider": "anthropic",
                "model": "claude-sonnet-4-6",
                "allow_fallback": true,
                "max_turns": 1
            }),
            &ctx,
        );

        assert!(!result.is_error, "unexpected error: {}", result.content);
        assert_eq!(google_invocations.load(Ordering::SeqCst), 1);
        assert_eq!(result.content, sentinel);
    });
}

#[test]
fn child_and_team_fallback_share_session_health_cache() {
    with_isolated_provider_auth(|| {
        init_team_swarm_runner_once();

        let sentinel = "shared session fallback sentinel";
        let (registry, invocations, health_checks) =
            make_tracking_openai_registry_with_health(sentinel);
        let ctx = make_tool_context_with_model_registry(
            Some(registry),
            Some(Arc::new(ModelRegistry::new())),
            None,
        );

        {
            let cache = Arc::new(crate::HealthCache::new());
            let _registration = crate::register_session_health_cache(&ctx.session_id, &cache);

            let agent_result = run_agent_tool(
                json!({
                    "description": "agent-fallback",
                    "prompt": "return the shared fallback sentinel",
                    "provider": "anthropic",
                    "model": "claude-sonnet-4-6",
                    "allow_fallback": true,
                    "max_turns": 1
                }),
                &ctx,
            );

            assert!(
                !agent_result.is_error,
                "unexpected agent error: {}",
                agent_result.content
            );
            assert_eq!(agent_result.content, sentinel);

            let team_result = run_team_create_tool(
                json!({
                    "team_name": "fallback-team",
                    "task": "return the shared fallback sentinel",
                    "agents": [
                        {
                            "name": "agent-a",
                            "task": "return the shared fallback sentinel",
                            "provider": "anthropic",
                            "model": "claude-sonnet-4-6",
                            "allow_fallback": true,
                            "max_turns": 1
                        }
                    ]
                }),
                &ctx,
            );

            assert!(
                !team_result.is_error,
                "unexpected team error: {}",
                team_result.content
            );

            let payload: Value = serde_json::from_str(&team_result.content)
                .expect("team result should be valid JSON");
            let team_output = payload["results"][0]["output"]
                .as_str()
                .expect("team output should be present");
            let (clean_output, observability) = split_encoded_team_output(team_output);

            assert_eq!(clean_output, sentinel);
            assert_eq!(
                observability["query_observability_events"][0]["provider_id"],
                json!("openai")
            );
        }

        assert_eq!(invocations.load(Ordering::SeqCst), 2);
        assert_eq!(health_checks.load(Ordering::SeqCst), 1);
        assert!(crate::session_health_cache_for_session(&ctx.session_id).is_none());
    });
}

#[test]
fn child_session_budget_reuses_inherited_budget_when_child_limit_absent() {
    let parent = Arc::new(crate::SessionBudget::new(7.5));
    let inherited = super::child_session_budget(Some(parent.clone()), None)
        .expect("inherited session budget should remain available");

    assert!(Arc::ptr_eq(&parent, &inherited));
}

#[test]
fn child_session_budget_wraps_parent_when_child_limit_present() {
    let parent = Arc::new(crate::SessionBudget::new(7.5));
    let child = super::child_session_budget(Some(parent.clone()), Some(2.0))
        .expect("child-local session budget should be created");

    assert_eq!(child.limit_usd(), 2.0);
    assert!(Arc::ptr_eq(&child.shared_budget(), &parent));

    child.record_cost(1.25);
    child.check_and_cancel();

    assert_eq!(child.spent_usd(), 1.25);
    assert_eq!(parent.spent_usd(), 1.25);
    assert!(!child.is_cancelled());
    assert!(!parent.is_cancelled());
}

#[test]
fn worker_budget_exceeded_event_reports_child_limit() {
    let parent = Arc::new(crate::SessionBudget::new(10.0));
    let child = Arc::new(crate::SessionBudget::child_scope(parent, 2.0));
    child.record_cost(2.5);
    child.check_and_cancel();

    let event = super::worker_budget_exceeded_event("worker-a", Some(&child), Some(2.0))
        .expect("child-local budget exceedance should emit an event");

    assert_eq!(event["type"], json!("worker_budget_exceeded"));
    assert_eq!(event["agent_id"], json!("worker-a"));
    assert_eq!(event["cost_usd"], json!(2.5));
    assert_eq!(event["limit_usd"], json!(2.0));
}

#[test]
fn teamcreate_mixed_providers_per_agent_dispatch() {
    with_isolated_provider_auth(|| {
        init_team_swarm_runner_once();

        let openai_sentinel = "team openai provider sentinel";
        let google_sentinel = "team google provider sentinel";
        let (
            registry,
            openai_invocations,
            google_invocations,
            openai_max_tokens,
            google_max_tokens,
        ) = make_mixed_tracking_registry(openai_sentinel, google_sentinel);
        let ctx = make_tool_context(Some(registry), None);

        let result = run_team_create_tool(
            json!({
                "team_name": "mixed-providers-team",
                "task": "return your provider sentinel",
                "agents": [
                    {
                        "name": "agent-a",
                        "task": "return the openai sentinel",
                        "provider": "openai",
                        "model": "gpt-4o",
                        "max_tokens": 321
                    },
                    {
                        "name": "agent-b",
                        "task": "return the google sentinel",
                        "provider": "google",
                        "model": "gemini-2.5-flash",
                        "max_tokens": 654
                    }
                ]
            }),
            &ctx,
        );

        assert!(!result.is_error, "unexpected error: {}", result.content);

        let payload: Value =
            serde_json::from_str(&result.content).expect("team result should be valid JSON");
        let results = payload["results"]
            .as_array()
            .expect("team results should be an array");

        let agent_a_output = results
            .iter()
            .find(|entry| entry["agent"] == "agent-a")
            .and_then(|entry| entry["output"].as_str())
            .expect("agent-a output should be present");
        let agent_b_output = results
            .iter()
            .find(|entry| entry["agent"] == "agent-b")
            .and_then(|entry| entry["output"].as_str())
            .expect("agent-b output should be present");

        let (agent_a_clean_output, agent_a_observability) =
            split_encoded_team_output(agent_a_output);
        let (agent_b_clean_output, agent_b_observability) =
            split_encoded_team_output(agent_b_output);

        assert_eq!(agent_a_clean_output, openai_sentinel);
        assert_eq!(agent_b_clean_output, google_sentinel);
        assert_ne!(agent_a_clean_output, google_sentinel);
        assert_ne!(agent_b_clean_output, openai_sentinel);
        assert_eq!(
            agent_a_observability["query_observability_events"][0]["provider_id"],
            json!("openai")
        );
        assert_eq!(
            agent_a_observability["query_observability_events"][0]["model_id"],
            json!("gpt-4o")
        );
        assert_eq!(
            agent_a_observability["query_observability_events"][0]["was_fallback"],
            json!(false)
        );
        assert_eq!(
            agent_b_observability["query_observability_events"][0]["provider_id"],
            json!("google")
        );
        assert_eq!(
            agent_b_observability["query_observability_events"][0]["model_id"],
            json!("gemini-2.5-flash")
        );
        assert_eq!(
            agent_b_observability["query_observability_events"][0]["was_fallback"],
            json!(false)
        );
        assert_eq!(openai_invocations.load(Ordering::SeqCst), 1);
        assert_eq!(google_invocations.load(Ordering::SeqCst), 1);
        assert_eq!(&*openai_max_tokens.lock(), &[321]);
        assert_eq!(&*google_max_tokens.lock(), &[654]);
    });
}
