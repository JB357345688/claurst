use std::{collections::HashMap, sync::Arc};

use claurst_api::{LlmProvider, ModelRegistry, ProviderRegistry};
use claurst_core::{AuthStore, ProviderId, config::ProviderConfig};

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
                claurst_api::providers::openai_compat_providers::ollama()
                    .with_base_url(base_url),
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
        ProviderIdentity, ProviderResolutionError, ResolutionSource, materialize_provider,
        normalize_ollama_api_base, resolve_provider_identity,
    };
    use std::{collections::HashMap, sync::Arc};

    use claurst_api::{ModelRegistry, OpenAiProvider, ProviderRegistry};
    use claurst_core::config::ProviderConfig;

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
        assert_provider_model_conflict(
            "openai",
            "anthropic/claude-sonnet-4-20250514",
            "anthropic",
        );
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
}
