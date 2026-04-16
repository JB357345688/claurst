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
                supports_required_capabilities(
                    Some(entry),
                    provider_caps,
                    DEFAULT_REQUIRED_CAPABILITIES,
                )
            })
            .collect::<Vec<_>>();
        family_matches.sort_by(|a, b| (*b.info.id).cmp(&*a.info.id));

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

        if let Ok(target) =
            materialize_provider(&fallback_identity, provider_registry, provider_configs)
        {
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
#[path = "provider_resolution_tests.rs"]
mod tests;
