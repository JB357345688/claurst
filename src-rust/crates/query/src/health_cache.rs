use std::time::{Duration, Instant};

use claurst_api::{LlmProvider, ProviderStatus};
use dashmap::DashMap;

const DEFAULT_TTL: Duration = Duration::from_secs(30);
const PROBE_TIMEOUT: Duration = Duration::from_secs(5);
const PROBE_TIMEOUT_REASON: &str = "health check timed out";
const PROBE_FAILURE_REASON: &str = "health check failed";

#[derive(Debug, Default)]
pub struct HealthCache {
    cache: DashMap<String, (ProviderStatus, Instant)>,
    ttl: Duration,
}

impl HealthCache {
    pub fn new() -> Self {
        Self {
            cache: DashMap::new(),
            ttl: DEFAULT_TTL,
        }
    }

    #[cfg(test)]
    fn with_ttl(ttl: Duration) -> Self {
        Self {
            cache: DashMap::new(),
            ttl,
        }
    }

    pub fn get(&self, provider_id: &str) -> Option<ProviderStatus> {
        let entry = self.cache.get(provider_id)?;
        if entry.value().1.elapsed() < self.ttl {
            Some(entry.value().0.clone())
        } else {
            None
        }
    }

    pub fn insert(&self, provider_id: &str, status: ProviderStatus) {
        self.cache
            .insert(provider_id.to_string(), (status, Instant::now()));
    }

    pub async fn probe_if_stale(
        &self,
        provider_id: &str,
        provider: &dyn LlmProvider,
    ) -> ProviderStatus {
        if let Some(status) = self.get(provider_id) {
            return status;
        }

        let status = match tokio::time::timeout(PROBE_TIMEOUT, provider.health_check()).await {
            Ok(Ok(status)) => status,
            Ok(Err(_)) => ProviderStatus::Unavailable {
                reason: PROBE_FAILURE_REASON.to_string(),
            },
            Err(_) => ProviderStatus::Unavailable {
                reason: PROBE_TIMEOUT_REASON.to_string(),
            },
        };

        self.insert(provider_id, status.clone());
        status
    }
}

#[cfg(test)]
mod tests {
    use super::{HealthCache, PROBE_FAILURE_REASON, PROBE_TIMEOUT_REASON};
    use async_trait::async_trait;
    use claurst_api::{
        LlmProvider, ModelInfo, ProviderCapabilities, ProviderError, ProviderRequest,
        ProviderResponse, ProviderStatus, StreamEvent, SystemPromptStyle,
    };
    use claurst_core::ProviderId;
    use futures::stream;
    use std::pin::Pin;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::time::{Duration, Instant};

    struct TestProvider {
        id: ProviderId,
        health_status: Option<ProviderStatus>,
        fail_health_check: bool,
        delay: Duration,
        health_calls: Arc<AtomicUsize>,
    }

    impl TestProvider {
        fn healthy() -> Self {
            Self {
                id: ProviderId::new("test-provider"),
                health_status: Some(ProviderStatus::Healthy),
                fail_health_check: false,
                delay: Duration::ZERO,
                health_calls: Arc::new(AtomicUsize::new(0)),
            }
        }

        fn failing() -> Self {
            Self {
                fail_health_check: true,
                ..Self::healthy()
            }
        }

        fn delayed(delay: Duration) -> Self {
            Self {
                delay,
                ..Self::healthy()
            }
        }
    }

    #[async_trait]
    impl LlmProvider for TestProvider {
        fn id(&self) -> &ProviderId {
            &self.id
        }

        fn name(&self) -> &str {
            "Test Provider"
        }

        async fn create_message(
            &self,
            _request: ProviderRequest,
        ) -> Result<ProviderResponse, ProviderError> {
            panic!("create_message is not used in health_cache tests")
        }

        async fn create_message_stream(
            &self,
            _request: ProviderRequest,
        ) -> Result<
            Pin<Box<dyn futures::Stream<Item = Result<StreamEvent, ProviderError>> + Send>>,
            ProviderError,
        > {
            Ok(Box::pin(stream::empty()))
        }

        async fn list_models(&self) -> Result<Vec<ModelInfo>, ProviderError> {
            Ok(vec![])
        }

        async fn health_check(&self) -> Result<ProviderStatus, ProviderError> {
            self.health_calls.fetch_add(1, Ordering::SeqCst);
            if !self.delay.is_zero() {
                tokio::time::sleep(self.delay).await;
            }
            if self.fail_health_check {
                Err(ProviderError::AuthFailed {
                    provider: self.id.clone(),
                    message: PROBE_FAILURE_REASON.to_string(),
                })
            } else {
                Ok(self
                    .health_status
                    .clone()
                    .unwrap_or(ProviderStatus::Unavailable {
                        reason: "missing test status".to_string(),
                    }))
            }
        }

        fn capabilities(&self) -> ProviderCapabilities {
            ProviderCapabilities {
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
            }
        }
    }

    #[test]
    fn health_cache_miss_returns_none() {
        let cache = HealthCache::new();

        assert!(cache.get("missing-provider").is_none());
    }

    #[test]
    fn health_cache_hit_returns_cached_status() {
        let cache = HealthCache::new();
        cache.insert(
            "provider-a",
            ProviderStatus::Degraded {
                reason: "slow".to_string(),
            },
        );

        assert!(matches!(
            cache.get("provider-a"),
            Some(ProviderStatus::Degraded { reason }) if reason == "slow"
        ));
    }

    #[test]
    fn health_cache_expired_entry_returns_none() {
        let cache = HealthCache::with_ttl(Duration::from_millis(10));
        cache.cache.insert(
            "provider-a".to_string(),
            (
                ProviderStatus::Healthy,
                Instant::now() - Duration::from_secs(1),
            ),
        );

        assert!(cache.get("provider-a").is_none());
    }

    #[tokio::test]
    async fn health_cache_probe_if_stale_caches_successful_result() {
        let cache = HealthCache::new();
        let provider = TestProvider::healthy();
        let calls = provider.health_calls.clone();

        let first = cache.probe_if_stale("provider-a", &provider).await;
        let second = cache.probe_if_stale("provider-a", &provider).await;

        assert!(matches!(first, ProviderStatus::Healthy));
        assert!(matches!(second, ProviderStatus::Healthy));
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn health_cache_probe_if_stale_maps_provider_error_to_unavailable() {
        let cache = HealthCache::new();
        let provider = TestProvider::failing();

        let status = cache.probe_if_stale("provider-a", &provider).await;

        assert!(matches!(
            status,
            ProviderStatus::Unavailable { reason } if reason == PROBE_FAILURE_REASON
        ));
    }

    #[tokio::test]
    async fn health_cache_probe_if_stale_times_out_as_unavailable() {
        let cache = HealthCache::new();
        let provider = TestProvider::delayed(Duration::from_secs(6));

        let status = cache.probe_if_stale("provider-a", &provider).await;

        assert!(matches!(
            status,
            ProviderStatus::Unavailable { reason } if reason == PROBE_TIMEOUT_REASON
        ));
    }
}
