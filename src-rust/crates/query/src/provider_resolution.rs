use std::{collections::HashMap, sync::Arc};

use claurst_api::{
    LlmProvider, ModelEntry, ModelRegistry, ProviderCapabilities, ProviderRegistry, TrustDomain,
};
use claurst_core::{config::ProviderConfig, AuthStore, ProviderId};

use crate::HealthCache;

pub const KNOWN_PROVIDERS: &[&str] = &[
    "anthropic",
    "openai",
    "google",
    "groq",
    "mistral",
    "deepseek",
    "xai",
    "cohere",
    "perplexity",
    "cerebras",
    "openrouter",
    "togetherai",
    "together-ai",
    "deepinfra",
    "venice",
    "github-copilot",
    "ollama",
    "lmstudio",
    "llamacpp",
    "azure",
    "amazon-bedrock",
    "huggingface",
    "nvidia",
    "fireworks",
    "sambanova",
    "codex",
    "siliconflow",
    "moonshot",
    "zhipu",
    "qwen",
    "nebius",
    "novita",
    "ovhcloud",
    "scaleway",
    "vultr",
    "vultr-ai",
    "baseten",
    "friendli",
    "upstage",
    "stepfun",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Capability {
    ToolCalling,
    Reasoning,
    Vision,
    PdfInput,
    AudioInput,
    StructuredOutput,
}

pub const DEFAULT_REQUIRED_CAPABILITIES: &[Capability] = &[Capability::ToolCalling];

pub fn model_supports_capability(entry: &ModelEntry, cap: &Capability) -> Option<bool> {
    match cap {
        Capability::ToolCalling => Some(entry.tool_calling),
        Capability::Reasoning => Some(entry.reasoning),
        Capability::Vision => Some(entry.vision),
        Capability::PdfInput => entry.pdf_input,
        Capability::AudioInput => entry.audio_input,
        Capability::StructuredOutput => entry.structured_output,
    }
}

pub fn provider_supports_capability(caps: &ProviderCapabilities, cap: &Capability) -> bool {
    match cap {
        Capability::ToolCalling => caps.tool_calling,
        Capability::Reasoning => caps.thinking,
        Capability::Vision => caps.image_input,
        Capability::PdfInput => caps.pdf_input,
        Capability::AudioInput => caps.audio_input,
        Capability::StructuredOutput => caps.structured_output,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolutionSource {
    ExplicitProvider,
    ModelStringPrefix,
    ModelRegistry,
    Default,
}

#[derive(Debug, Clone)]
pub struct ProviderIdentity {
    pub provider_id: String,
    pub model_id: String,
    pub resolution_source: ResolutionSource,
}

#[derive(Clone)]
pub struct ExecutionTarget {
    pub provider_id: String,
    pub model_id: String,
    pub provider: Arc<dyn LlmProvider>,
    pub resolution_source: ResolutionSource,
}

impl std::fmt::Debug for ExecutionTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExecutionTarget")
            .field("provider_id", &self.provider_id)
            .field("model_id", &self.model_id)
            .field("provider", &"<dyn LlmProvider>")
            .field("resolution_source", &self.resolution_source)
            .finish()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ProviderResolutionError {
    #[error("Provider '{0}' not found in registry")]
    ProviderNotFound(String),
    #[error("No provider found for model '{0}'")]
    NoProviderForModel(String),
    #[error("No credentials available for provider '{0}'")]
    NoCredentials(String),
    #[error("Provider/model conflict: explicit provider '{provider}' but model '{model}' belongs to '{model_provider}'")]
    ProviderModelConflict {
        provider: String,
        model: String,
        model_provider: String,
    },
    #[error("Provider '{0}' is unavailable")]
    ProviderUnavailable(String),
    #[error("{0}. Try allow_fallback: true")]
    FallbackDisabled(String),
}

pub fn resolve_provider_identity(
    explicit_provider: Option<&str>,
    model: &str,
    model_registry: Option<&ModelRegistry>,
) -> Result<ProviderIdentity, ProviderResolutionError> {
    if let Some(provider) = explicit_provider {
        let provider_prefix = format!("{provider}/");
        let model_id = if let Some(stripped) = model.strip_prefix(&provider_prefix) {
            stripped.to_string()
        } else {
            if let Some((model_provider, _)) = model.split_once('/') {
                if KNOWN_PROVIDERS.contains(&model_provider) && model_provider != provider {
                    return Err(ProviderResolutionError::ProviderModelConflict {
                        provider: provider.to_string(),
                        model: model.to_string(),
                        model_provider: model_provider.to_string(),
                    });
                }
            }
            model.to_string()
        };

        return Ok(ProviderIdentity {
            provider_id: provider.to_string(),
            model_id,
            resolution_source: ResolutionSource::ExplicitProvider,
        });
    }

    if let Some((model_provider, model_id)) = model.split_once('/') {
        if KNOWN_PROVIDERS.contains(&model_provider) {
            return Ok(ProviderIdentity {
                provider_id: model_provider.to_string(),
                model_id: model_id.to_string(),
                resolution_source: ResolutionSource::ModelStringPrefix,
            });
        }
    }

    if let Some(model_registry) = model_registry {
        if let Some(provider_id) = model_registry.find_provider_for_model(model) {
            return Ok(ProviderIdentity {
                provider_id: provider_id.to_string(),
                model_id: model.to_string(),
                resolution_source: ResolutionSource::ModelRegistry,
            });
        }
    }

    Ok(ProviderIdentity {
        provider_id: "anthropic".to_string(),
        model_id: model.to_string(),
        resolution_source: ResolutionSource::Default,
    })
}

pub fn materialize_provider(
    identity: &ProviderIdentity,
    registry: &ProviderRegistry,
    provider_configs: &HashMap<String, ProviderConfig>,
) -> Result<ExecutionTarget, ProviderResolutionError> {
    if identity.provider_id == ProviderId::OLLAMA {
        return Ok(ExecutionTarget {
            provider_id: identity.provider_id.clone(),
            model_id: identity.model_id.clone(),
            provider: Arc::new(build_ollama_provider(provider_configs)),
            resolution_source: identity.resolution_source.clone(),
        });
    }

    let pid = ProviderId::new(&identity.provider_id);

    let runtime_provider = claurst_api::registry::runtime_provider_for(&identity.provider_id);

    let mut registry_provider = if runtime_provider.is_some() {
        None
    } else {
        registry.get(&pid).cloned()
    };

    if let Some(override_base) = provider_configs
        .get(&identity.provider_id)
        .and_then(|pc| pc.api_base.as_deref())
    {
        let base_url = format!("{}/v1", override_base.trim_end_matches('/'));
        let overridden: Option<Arc<dyn LlmProvider>> = match identity.provider_id.as_str() {
            "ollama" => Some(Arc::new(
                claurst_api::providers::openai_compat_providers::ollama().with_base_url(base_url),
            )),
            "lmstudio" | "lm-studio" => Some(Arc::new(
                claurst_api::providers::openai_compat_providers::lm_studio()
                    .with_base_url(base_url),
            )),
            "llamacpp" | "llama-cpp" => Some(Arc::new(
                claurst_api::providers::openai_compat_providers::llama_cpp()
                    .with_base_url(base_url),
            )),
            _ => None,
        };
        if overridden.is_some() {
            registry_provider = overridden;
        }
    }

    let provider = runtime_provider
        .or(registry_provider)
        .ok_or_else(|| ProviderResolutionError::NoCredentials(identity.provider_id.clone()))?;

    Ok(ExecutionTarget {
        provider_id: identity.provider_id.clone(),
        model_id: identity.model_id.clone(),
        provider,
        resolution_source: identity.resolution_source.clone(),
    })
}

fn supports_required_capabilities(
    entry: Option<&ModelEntry>,
    provider_caps: &ProviderCapabilities,
    required_capabilities: &[Capability],
) -> bool {
    required_capabilities.iter().all(|cap| {
        entry
            .and_then(|entry| model_supports_capability(entry, cap))
            .unwrap_or_else(|| provider_supports_capability(provider_caps, cap))
    })
}

fn select_fallback_model(
    model_registry: Option<&ModelRegistry>,
    primary_identity: &ProviderIdentity,
    candidate_provider_id: &str,
    provider_caps: &ProviderCapabilities,
) -> Option<String> {
    let model_registry = model_registry?;
    let original_family = model_registry
        .get(&primary_identity.provider_id, &primary_identity.model_id)
        .and_then(|entry| entry.family.as_deref());

    if let Some(family) = original_family {
        let mut family_matches = model_registry
            .list_by_provider(candidate_provider_id)
            .into_iter()
            .filter(|entry| entry.family.as_deref() == Some(family))
            .filter(|entry| {
                supports_required_capabilities(Some(entry), provider_caps, DEFAULT_REQUIRED_CAPABILITIES)
            })
            .collect::<Vec<_>>();
        family_matches.sort_by(|a, b| (&*b.info.id).cmp(&*a.info.id));

        if let Some(entry) = family_matches.first() {
            return Some(entry.info.id.to_string());
        }
    }

    let default_model_id = model_registry.best_model_for_provider(candidate_provider_id)?;
    let default_entry = model_registry.get(candidate_provider_id, &default_model_id);
    supports_required_capabilities(default_entry, provider_caps, DEFAULT_REQUIRED_CAPABILITIES)
        .then_some(default_model_id)
}

pub async fn resolve_provider_with_fallback(
    explicit_provider: Option<&str>,
    model: &str,
    model_registry: Option<&ModelRegistry>,
    provider_registry: &ProviderRegistry,
    provider_configs: &HashMap<String, ProviderConfig>,
    health_cache: &HealthCache,
    allow_fallback: bool,
) -> Result<ExecutionTarget, ProviderResolutionError> {
    let identity = resolve_provider_identity(explicit_provider, model, model_registry)?;

    let direct_error = match materialize_provider(&identity, provider_registry, provider_configs) {
        Ok(target) => return Ok(target),
        Err(error) => error,
    };

    if !allow_fallback {
        return Err(ProviderResolutionError::FallbackDisabled(
            direct_error.to_string(),
        ));
    }

    let primary_domain = TrustDomain::for_provider(&identity.provider_id);
    let mut healthy_candidates = Vec::new();
    let mut degraded_candidates = Vec::new();

    for candidate_id in provider_registry.provider_ids() {
        let candidate_provider_id = candidate_id.to_string();
        if candidate_provider_id == identity.provider_id {
            continue;
        }
        if TrustDomain::for_provider(&candidate_provider_id) != primary_domain {
            continue;
        }

        let Some(provider) = provider_registry.get(candidate_id) else {
            continue;
        };

        match health_cache
            .probe_if_stale(&candidate_provider_id, provider.as_ref())
            .await
        {
            claurst_api::ProviderStatus::Healthy => healthy_candidates.push(candidate_provider_id),
            claurst_api::ProviderStatus::Degraded { .. } => {
                degraded_candidates.push(candidate_provider_id)
            }
            claurst_api::ProviderStatus::Unavailable { .. } => {}
        }
    }

    for candidate_provider_id in healthy_candidates
        .into_iter()
        .chain(degraded_candidates.into_iter())
    {
        let candidate_pid = ProviderId::new(candidate_provider_id.as_str());
        let Some(provider) = provider_registry.get(&candidate_pid) else {
            continue;
        };

        let provider_caps = provider.capabilities();
        let Some(candidate_model_id) = select_fallback_model(
            model_registry,
            &identity,
            &candidate_provider_id,
            &provider_caps,
        ) else {
            continue;
        };

        let fallback_identity = ProviderIdentity {
            provider_id: candidate_provider_id.clone(),
            model_id: candidate_model_id,
            resolution_source: ResolutionSource::ModelRegistry,
        };

        if let Ok(target) = materialize_provider(
            &fallback_identity,
            provider_registry,
            provider_configs,
        ) {
            return Ok(target);
        }
    }

    Err(direct_error)
}

fn build_ollama_provider(
    provider_configs: &HashMap<String, ProviderConfig>,
) -> claurst_api::providers::openai_compat::OpenAiCompatProvider {
    let mut provider = claurst_api::providers::openai_compat_providers::ollama();

    if let Some(override_base) = provider_configs
        .get(ProviderId::OLLAMA)
        .and_then(|pc| pc.api_base.as_deref())
    {
        provider = provider.with_base_url(normalize_ollama_api_base(override_base));
    }

    if let Some(key) = AuthStore::load().api_key_for(ProviderId::OLLAMA) {
        provider = provider.with_api_key(key);
    }

    provider
}

fn normalize_ollama_api_base(api_base: &str) -> String {
    let trimmed = api_base.trim_end_matches('/');

    if let Some(root) = trimmed.strip_suffix("/api/v1") {
        return format!("{root}/v1");
    }

    if let Some(root) = trimmed.strip_suffix("/api") {
        return format!("{root}/v1");
    }

    if trimmed.ends_with("/v1") {
        return trimmed.to_string();
    }

    format!("{trimmed}/v1")
}

#[cfg(test)]
mod tests {
    use super::{
        materialize_provider, model_supports_capability, normalize_ollama_api_base,
        provider_supports_capability, resolve_provider_identity, resolve_provider_with_fallback,
        Capability, DEFAULT_REQUIRED_CAPABILITIES, ProviderIdentity, ProviderResolutionError,
        ResolutionSource,
    };
    use std::{
        collections::HashMap,
        ffi::{OsStr, OsString},
        future::Future,
        sync::Arc,
    };

    use async_trait::async_trait;
    use claurst_api::{
        LlmProvider, ModelEntry, ModelInfo, ModelRegistry, OpenAiProvider, ProviderCapabilities,
        ProviderError, ProviderRegistry, ProviderStatus, SystemPromptStyle,
    };
    use claurst_core::{config::ProviderConfig, AuthStore, ModelId, ProviderId, StoredCredential};
    use tempfile::TempDir;
    use crate::HealthCache;

    struct TestProvider {
        id: ProviderId,
        name: String,
        health_status: ProviderStatus,
        capabilities: ProviderCapabilities,
    }

    impl TestProvider {
        fn new(id: &str, name: &str) -> Self {
            Self {
                id: ProviderId::new(id),
                name: name.to_string(),
                health_status: ProviderStatus::Healthy,
                capabilities: ProviderCapabilities {
                    streaming: false,
                    tool_calling: false,
                    thinking: false,
                    image_input: false,
                    pdf_input: false,
                    audio_input: false,
                    video_input: false,
                    caching: false,
                    structured_output: false,
                    system_prompt_style: SystemPromptStyle::SystemMessage,
                },
            }
        }

        fn with_health_status(mut self, health_status: ProviderStatus) -> Self {
            self.health_status = health_status;
            self
        }

        fn with_capabilities(mut self, capabilities: ProviderCapabilities) -> Self {
            self.capabilities = capabilities;
            self
        }
    }

    #[async_trait]
    impl LlmProvider for TestProvider {
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
            panic!("create_message is not used in provider_resolution tests")
        }

        async fn create_message_stream(
            &self,
            _request: claurst_api::ProviderRequest,
        ) -> Result<
            std::pin::Pin<
                Box<
                    dyn futures::Stream<Item = Result<claurst_api::StreamEvent, ProviderError>>
                        + Send,
                >,
            >,
            ProviderError,
        > {
            panic!("create_message_stream is not used in provider_resolution tests")
        }

        async fn list_models(&self) -> Result<Vec<claurst_api::ModelInfo>, ProviderError> {
            Ok(vec![])
        }

        async fn health_check(&self) -> Result<ProviderStatus, ProviderError> {
            Ok(self.health_status.clone())
        }

        fn capabilities(&self) -> ProviderCapabilities {
            self.capabilities.clone()
        }
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
        let _google_genai = EnvGuard::set("GOOGLE_GENERATIVE_AI_API_KEY", None);
        let _ollama = EnvGuard::set("OLLAMA_API_KEY", None);
        let _lm_studio = EnvGuard::set("LM_STUDIO_HOST", None);
        let _llama_cpp = EnvGuard::set("LLAMA_CPP_HOST", None);
        f()
    }

    fn run_async<T>(future: impl Future<Output = T>) -> T {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(future)
    }

    fn assert_unavailable_reason(status: ProviderStatus, expected_reason: &str) {
        assert!(
            matches!(
                status,
                ProviderStatus::Unavailable { ref reason } if reason == expected_reason
            ),
            "unexpected provider status: {status:?}"
        );
    }

    fn assert_identity(
        explicit_provider: Option<&str>,
        model: &str,
        model_registry: Option<&ModelRegistry>,
        expected_provider: &str,
        expected_model: &str,
        expected_source: ResolutionSource,
    ) {
        let identity = resolve_provider_identity(explicit_provider, model, model_registry)
            .expect("resolution should succeed");

        assert_eq!(identity.provider_id, expected_provider);
        assert_eq!(identity.model_id, expected_model);
        assert_eq!(identity.resolution_source, expected_source);
    }

    fn assert_provider_model_conflict(
        explicit_provider: &str,
        model: &str,
        expected_model_provider: &str,
    ) {
        let error = resolve_provider_identity(Some(explicit_provider), model, None)
            .expect_err("resolution should fail with provider/model conflict");

        assert!(matches!(
            error,
            ProviderResolutionError::ProviderModelConflict {
                provider,
                model: error_model,
                model_provider,
            } if provider == explicit_provider
                && error_model == model
                && model_provider == expected_model_provider
        ));
    }

    fn provider_identity(
        provider_id: &str,
        model_id: &str,
        resolution_source: ResolutionSource,
    ) -> ProviderIdentity {
        ProviderIdentity {
            provider_id: provider_id.to_string(),
            model_id: model_id.to_string(),
            resolution_source,
        }
    }

    fn test_model_entry() -> ModelEntry {
        ModelEntry {
            info: ModelInfo {
                id: ModelId::new("test-model"),
                provider_id: ProviderId::new("test-provider"),
                name: "Test Model".to_string(),
                context_window: 128_000,
                max_output_tokens: 4_096,
            },
            cost_input: None,
            cost_output: None,
            cost_cache_read: None,
            cost_cache_write: None,
            tool_calling: false,
            reasoning: false,
            vision: false,
            pdf_input: None,
            audio_input: None,
            structured_output: None,
            max_output_tokens: None,
            family: None,
            status: "active".to_string(),
        }
    }

    fn tool_calling_capabilities() -> ProviderCapabilities {
        ProviderCapabilities {
            streaming: false,
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

    #[test]
    fn normalize_ollama_api_base_rewrites_hosted_api_root() {
        assert_eq!(
            normalize_ollama_api_base("https://ollama.com/api"),
            "https://ollama.com/v1"
        );
        assert_eq!(
            normalize_ollama_api_base("https://ollama.com/api/"),
            "https://ollama.com/v1"
        );
    }

    #[test]
    fn normalize_ollama_api_base_rewrites_hosted_api_v1_root() {
        assert_eq!(
            normalize_ollama_api_base("https://ollama.com/api/v1"),
            "https://ollama.com/v1"
        );
        assert_eq!(
            normalize_ollama_api_base("https://ollama.com/api/v1/"),
            "https://ollama.com/v1"
        );
    }

    #[test]
    fn normalize_ollama_api_base_appends_v1_for_plain_roots() {
        assert_eq!(
            normalize_ollama_api_base("http://localhost:11434"),
            "http://localhost:11434/v1"
        );
        assert_eq!(
            normalize_ollama_api_base("https://ollama.com/v1"),
            "https://ollama.com/v1"
        );
    }

    #[test]
    fn p1_explicit_provider_matches_model_prefix() {
        assert_identity(
            Some("openai"),
            "openai/gpt-4o",
            None,
            "openai",
            "gpt-4o",
            ResolutionSource::ExplicitProvider,
        );
    }

    #[test]
    fn p2_explicit_provider_with_bare_model() {
        assert_identity(
            Some("openai"),
            "gpt-4o",
            None,
            "openai",
            "gpt-4o",
            ResolutionSource::ExplicitProvider,
        );
    }

    #[test]
    fn p3_explicit_provider_conflicts_with_model_prefix() {
        assert_provider_model_conflict("openai", "anthropic/claude-sonnet-4-20250514", "anthropic");
    }

    #[test]
    fn p4_no_provider_with_known_model_prefix() {
        assert_identity(
            None,
            "google/gemini-2.5-flash",
            None,
            "google",
            "gemini-2.5-flash",
            ResolutionSource::ModelStringPrefix,
        );
    }

    #[test]
    fn p5_explicit_provider_conflicts_with_reverse_model_prefix() {
        assert_provider_model_conflict("anthropic", "openai/gpt-4o", "openai");
    }

    #[test]
    fn p6_explicit_anthropic_pin_with_bare_model() {
        assert_identity(
            Some("anthropic"),
            "claude-sonnet-4-20250514",
            None,
            "anthropic",
            "claude-sonnet-4-20250514",
            ResolutionSource::ExplicitProvider,
        );
    }

    #[test]
    fn p7_no_provider_with_unknown_namespace_defaults() {
        assert_identity(
            None,
            "meta-llama/Llama-3.3-70B",
            None,
            "anthropic",
            "meta-llama/Llama-3.3-70B",
            ResolutionSource::Default,
        );
    }

    #[test]
    fn p8_no_provider_bare_model_registry_resolves() {
        let model_registry = ModelRegistry::new();

        assert_identity(
            None,
            "gemini-3-flash-preview",
            Some(&model_registry),
            "google",
            "gemini-3-flash-preview",
            ResolutionSource::ModelRegistry,
        );
    }

    #[test]
    fn p9_no_provider_bare_model_registry_has_no_match() {
        let model_registry = ModelRegistry::new();

        assert_identity(
            None,
            "some-unknown-model",
            Some(&model_registry),
            "anthropic",
            "some-unknown-model",
            ResolutionSource::Default,
        );
    }

    #[test]
    fn p10_no_provider_without_model_registry_defaults() {
        assert_identity(
            None,
            "claude-sonnet-4-20250514",
            None,
            "anthropic",
            "claude-sonnet-4-20250514",
            ResolutionSource::Default,
        );
    }

    #[test]
    fn p11_explicit_provider_with_nested_slash_model() {
        assert_identity(
            Some("openrouter"),
            "openrouter/meta-llama/Llama-3.3-70B",
            None,
            "openrouter",
            "meta-llama/Llama-3.3-70B",
            ResolutionSource::ExplicitProvider,
        );
    }

    #[test]
    fn p12_local_provider_with_bare_model() {
        assert_identity(
            Some("ollama"),
            "llama3",
            None,
            "ollama",
            "llama3",
            ResolutionSource::ExplicitProvider,
        );
    }

    #[test]
    fn default_required_capabilities_contains_tool_calling_capability() {
        assert_eq!(DEFAULT_REQUIRED_CAPABILITIES, &[Capability::ToolCalling]);
    }

    #[test]
    fn model_supports_capability_returns_known_bool_capabilities() {
        let mut entry = test_model_entry();
        entry.tool_calling = true;
        entry.reasoning = false;
        entry.vision = true;

        assert_eq!(
            model_supports_capability(&entry, &Capability::ToolCalling),
            Some(true)
        );
        assert_eq!(
            model_supports_capability(&entry, &Capability::Reasoning),
            Some(false)
        );
        assert_eq!(
            model_supports_capability(&entry, &Capability::Vision),
            Some(true)
        );
    }

    #[test]
    fn model_supports_capability_returns_optional_capability_values() {
        let mut entry = test_model_entry();
        entry.pdf_input = Some(true);
        entry.audio_input = Some(false);
        entry.structured_output = None;

        assert_eq!(
            model_supports_capability(&entry, &Capability::PdfInput),
            Some(true)
        );
        assert_eq!(
            model_supports_capability(&entry, &Capability::AudioInput),
            Some(false)
        );
        assert_eq!(
            model_supports_capability(&entry, &Capability::StructuredOutput),
            None
        );
    }

    #[test]
    fn provider_supports_capability_maps_provider_capability_fields() {
        let caps = ProviderCapabilities {
            streaming: false,
            tool_calling: true,
            thinking: false,
            image_input: true,
            pdf_input: false,
            audio_input: true,
            video_input: false,
            caching: false,
            structured_output: true,
            system_prompt_style: SystemPromptStyle::SystemMessage,
        };

        assert!(provider_supports_capability(&caps, &Capability::ToolCalling));
        assert!(!provider_supports_capability(&caps, &Capability::Reasoning));
        assert!(provider_supports_capability(&caps, &Capability::Vision));
        assert!(!provider_supports_capability(&caps, &Capability::PdfInput));
        assert!(provider_supports_capability(&caps, &Capability::AudioInput));
        assert!(provider_supports_capability(
            &caps,
            &Capability::StructuredOutput
        ));
    }

    #[test]
    fn materialize_provider_returns_openai_target_from_happy_path() {
        let identity = provider_identity("openai", "gpt-4o", ResolutionSource::ExplicitProvider);
        let mut registry = ProviderRegistry::new();
        registry.register(Arc::new(OpenAiProvider::new("test-key".to_string())));

        let target = materialize_provider(&identity, &registry, &HashMap::new())
            .expect("materialization should succeed");

        assert_eq!(target.provider_id, "openai");
        assert_eq!(target.model_id, "gpt-4o");
        assert_eq!(target.resolution_source, ResolutionSource::ExplicitProvider);
        assert_eq!(target.provider.id(), "openai");
    }

    #[test]
    fn materialize_provider_returns_no_credentials_for_unknown_provider() {
        let identity = provider_identity(
            "some-fake-provider",
            "fake-model",
            ResolutionSource::ExplicitProvider,
        );
        let registry = ProviderRegistry::new();

        let error = materialize_provider(&identity, &registry, &HashMap::new())
            .expect_err("materialization should fail without credentials");

        assert!(matches!(
            error,
            ProviderResolutionError::NoCredentials(provider) if provider == "some-fake-provider"
        ));
    }

    #[test]
    fn materialize_provider_accepts_ollama_api_base_override() {
        let identity = provider_identity("ollama", "llama3", ResolutionSource::ExplicitProvider);
        let registry = ProviderRegistry::new();
        let mut provider_configs = HashMap::new();
        provider_configs.insert(
            "ollama".to_string(),
            ProviderConfig {
                api_base: Some("http://custom:11434".to_string()),
                ..Default::default()
            },
        );

        let target = materialize_provider(&identity, &registry, &provider_configs)
            .expect("materialization should succeed for ollama api_base override");

        assert_eq!(target.provider_id, "ollama");
        assert_eq!(target.model_id, "llama3");
        assert_eq!(target.resolution_source, ResolutionSource::ExplicitProvider);
        assert_eq!(target.provider.id(), "ollama");
    }

    #[test]
    fn materialize_provider_prefers_auth_store_provider_over_registry() {
        with_isolated_provider_auth(|| {
            let identity =
                provider_identity("openai", "gpt-4o", ResolutionSource::ExplicitProvider);
            let mut registry = ProviderRegistry::new();
            registry.register(Arc::new(TestProvider::new("openai", "Registry OpenAI")));

            let mut auth_store = AuthStore::default();
            auth_store.set(
                "openai",
                StoredCredential::ApiKey {
                    key: "auth-store-key".to_string(),
                },
            );

            let target = materialize_provider(&identity, &registry, &HashMap::new())
                .expect("materialization should prefer the auth-store runtime provider");

            assert_eq!(target.provider_id, "openai");
            assert_eq!(target.model_id, "gpt-4o");
            assert_eq!(target.resolution_source, ResolutionSource::ExplicitProvider);
            assert_eq!(target.provider.id(), "openai");
            assert_eq!(target.provider.name(), "OpenAI");
            assert_ne!(target.provider.name(), "Registry OpenAI");
        });
    }

    #[test]
    fn materialize_provider_applies_lm_studio_api_base_override() {
        with_isolated_provider_auth(|| {
            let _lm_studio_host = EnvGuard::set("LM_STUDIO_HOST", Some("http://localhost:bad"));
            let identity = provider_identity(
                "lm-studio",
                "local-model",
                ResolutionSource::ExplicitProvider,
            );
            let registry = ProviderRegistry::new();
            let mut provider_configs = HashMap::new();
            provider_configs.insert(
                "lm-studio".to_string(),
                ProviderConfig {
                    api_base: Some("https://example.invalid/lm-studio".to_string()),
                    ..Default::default()
                },
            );

            let target = materialize_provider(&identity, &registry, &provider_configs)
                .expect("materialization should succeed for lm-studio api_base override");
            let status = run_async(target.provider.health_check())
                .expect("health check should yield a provider status");

            assert_eq!(target.provider_id, "lm-studio");
            assert_eq!(target.model_id, "local-model");
            assert_eq!(target.resolution_source, ResolutionSource::ExplicitProvider);
            assert_eq!(target.provider.id(), "lm-studio");
            assert_unavailable_reason(status, "No API key configured");
        });
    }

    #[test]
    fn materialize_provider_applies_llama_cpp_api_base_override() {
        with_isolated_provider_auth(|| {
            let _llama_cpp_host = EnvGuard::set("LLAMA_CPP_HOST", Some("http://localhost:bad"));
            let identity = provider_identity(
                "llama-cpp",
                "local-model",
                ResolutionSource::ExplicitProvider,
            );
            let registry = ProviderRegistry::new();
            let mut provider_configs = HashMap::new();
            provider_configs.insert(
                "llama-cpp".to_string(),
                ProviderConfig {
                    api_base: Some("https://example.invalid/llama-cpp".to_string()),
                    ..Default::default()
                },
            );

            let target = materialize_provider(&identity, &registry, &provider_configs)
                .expect("materialization should succeed for llama-cpp api_base override");
            let status = run_async(target.provider.health_check())
                .expect("health check should yield a provider status");

            assert_eq!(target.provider_id, "llama-cpp");
            assert_eq!(target.model_id, "local-model");
            assert_eq!(target.resolution_source, ResolutionSource::ExplicitProvider);
            assert_eq!(target.provider.id(), "llama-cpp");
            assert_unavailable_reason(status, "No API key configured");
        });
    }

    #[test]
    fn materialize_provider_returns_no_credentials_for_known_provider_without_auth() {
        with_isolated_provider_auth(|| {
            let identity =
                provider_identity("openai", "gpt-4o", ResolutionSource::ExplicitProvider);
            let registry = ProviderRegistry::new();

            let error = materialize_provider(&identity, &registry, &HashMap::new())
                .expect_err("materialization should fail without openai credentials");

            assert!(matches!(
                error,
                ProviderResolutionError::NoCredentials(provider) if provider == "openai"
            ));
        });
    }

    #[test]
    fn fallback_disabled_returns_suggestion_text() {
        with_isolated_provider_auth(|| {
            let model_registry = ModelRegistry::new();
            let registry = ProviderRegistry::new();
            let health_cache = HealthCache::new();

            let error = run_async(resolve_provider_with_fallback(
                Some("openai"),
                "gpt-4o",
                Some(&model_registry),
                &registry,
                &HashMap::new(),
                &health_cache,
                false,
            ))
            .expect_err("fallback-disabled resolution should fail");

            assert!(matches!(
                error,
                ProviderResolutionError::FallbackDisabled(_)
            ));
            assert!(error.to_string().contains("allow_fallback: true"));
        });
    }

    #[test]
    fn fallback_same_domain_returns_healthy_cloud_candidate() {
        with_isolated_provider_auth(|| {
            let model_registry = ModelRegistry::new();
            let health_cache = HealthCache::new();
            let mut registry = ProviderRegistry::new();
            registry.register(Arc::new(
                TestProvider::new("openai", "OpenAI")
                    .with_health_status(ProviderStatus::Degraded {
                        reason: "slow".to_string(),
                    })
                    .with_capabilities(tool_calling_capabilities()),
            ));
            registry.register(Arc::new(
                TestProvider::new("google", "Google")
                    .with_health_status(ProviderStatus::Healthy)
                    .with_capabilities(tool_calling_capabilities()),
            ));

            let target = run_async(resolve_provider_with_fallback(
                Some("anthropic"),
                "claude-sonnet-4-6",
                Some(&model_registry),
                &registry,
                &HashMap::new(),
                &health_cache,
                true,
            ))
            .expect("same-domain fallback should succeed");

            assert_eq!(target.provider_id, "google");
            assert_eq!(target.model_id, "gemini-2.5-pro");
            assert_eq!(target.resolution_source, ResolutionSource::ModelRegistry);
        });
    }

    #[test]
    fn fallback_cross_domain_is_prohibited() {
        with_isolated_provider_auth(|| {
            let model_registry = ModelRegistry::new();
            let health_cache = HealthCache::new();
            let mut registry = ProviderRegistry::new();
            registry.register(Arc::new(
                TestProvider::new("google", "Google")
                    .with_health_status(ProviderStatus::Healthy)
                    .with_capabilities(tool_calling_capabilities()),
            ));

            let error = run_async(resolve_provider_with_fallback(
                Some("lm-studio"),
                "local-model",
                Some(&model_registry),
                &registry,
                &HashMap::new(),
                &health_cache,
                true,
            ))
            .expect_err("cross-domain fallback should not succeed");

            assert!(matches!(
                error,
                ProviderResolutionError::NoCredentials(provider) if provider == "lm-studio"
            ));
        });
    }
}
