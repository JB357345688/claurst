use std::{collections::HashMap, sync::Arc};

use claurst_api::{LlmProvider, ModelRegistry, ProviderRegistry};
use claurst_core::{ProviderId, config::ProviderConfig};

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
