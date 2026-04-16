use std::future::Future;
use std::sync::Arc;
use std::time::{Duration, Instant};

use claurst_api::{LlmProvider, ProviderStatus};
use dashmap::mapref::entry::Entry;
use dashmap::DashMap;
use once_cell::sync::Lazy;

const DEFAULT_TTL: Duration = Duration::from_secs(30);
const PROBE_TIMEOUT: Duration = Duration::from_secs(5);
const PROBE_TIMEOUT_REASON: &str = "health check timed out";
const PROBE_FAILURE_REASON: &str = "health check failed";

static SESSION_HEALTH_CACHE_REGISTRY: Lazy<DashMap<String, (Arc<HealthCache>, usize)>> =
    Lazy::new(DashMap::new);

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

#[derive(Debug)]
pub struct SessionHealthCacheRegistration {
    session_id: String,
}

impl Drop for SessionHealthCacheRegistration {
    fn drop(&mut self) {
        if let Entry::Occupied(mut entry) =
            SESSION_HEALTH_CACHE_REGISTRY.entry(self.session_id.clone())
        {
            let should_remove = {
                let (_, registrations) = entry.get_mut();
                *registrations -= 1;
                *registrations == 0
            };
            if should_remove {
                entry.remove();
            }
        }
    }
}

pub fn register_session_health_cache(
    session_id: &str,
    cache: &Arc<HealthCache>,
) -> SessionHealthCacheRegistration {
    match SESSION_HEALTH_CACHE_REGISTRY.entry(session_id.to_string()) {
        Entry::Occupied(mut entry) => {
            let (_, registrations) = entry.get_mut();
            *registrations += 1;
        }
        Entry::Vacant(entry) => {
            entry.insert((cache.clone(), 1));
        }
    }

    SessionHealthCacheRegistration {
        session_id: session_id.to_string(),
    }
}

pub fn session_health_cache_for_session(session_id: &str) -> Option<Arc<HealthCache>> {
    SESSION_HEALTH_CACHE_REGISTRY
        .get(session_id)
        .map(|entry| entry.value().0.clone())
}

pub fn session_health_cache_or_new(session_id: &str) -> Arc<HealthCache> {
    session_health_cache_for_session(session_id).unwrap_or_else(|| Arc::new(HealthCache::new()))
}

pub async fn with_registered_session_health_cache<Fut, T>(session_id: &str, future: Fut) -> T
where
    Fut: Future<Output = T>,
{
    let cache = Arc::new(HealthCache::new());
    let _registration = register_session_health_cache(session_id, &cache);
    future.await
}

#[cfg(test)]
mod tests {
    use super::{
        register_session_health_cache, session_health_cache_for_session,
        with_registered_session_health_cache, HealthCache, PROBE_FAILURE_REASON,
        PROBE_TIMEOUT_REASON,
    };
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

    #[test]
    fn health_cache_registration_exposes_cache_for_session() {
        let cache = Arc::new(HealthCache::new());
        let _registration = register_session_health_cache("health-cache-visible", &cache);

        let inherited = session_health_cache_for_session("health-cache-visible")
            .expect("health cache must register");

        assert!(Arc::ptr_eq(&cache, &inherited));
    }

    #[test]
    fn health_cache_registration_releases_when_last_guard_drops() {
        let cache = Arc::new(HealthCache::new());

        {
            let _registration = register_session_health_cache("health-cache-release", &cache);
            assert!(session_health_cache_for_session("health-cache-release").is_some());
        }

        assert!(session_health_cache_for_session("health-cache-release").is_none());
    }

    #[test]
    fn health_cache_nested_registration_preserves_initial_owner() {
        let root = Arc::new(HealthCache::new());
        let child = Arc::new(HealthCache::new());
        let _root_registration = register_session_health_cache("health-cache-nested-owner", &root);

        {
            let _child_registration =
                register_session_health_cache("health-cache-nested-owner", &child);
            let inherited = session_health_cache_for_session("health-cache-nested-owner")
                .expect("root health cache must stay visible");
            assert!(Arc::ptr_eq(&root, &inherited));
        }

        let inherited = session_health_cache_for_session("health-cache-nested-owner")
            .expect("root health cache must remain registered");
        assert!(Arc::ptr_eq(&root, &inherited));
    }

    #[tokio::test]
    async fn with_registered_session_health_cache_registers_and_cleans_up() {
        with_registered_session_health_cache("health-cache-scope", async {
            assert!(session_health_cache_for_session("health-cache-scope").is_some());
        })
        .await;

        assert!(session_health_cache_for_session("health-cache-scope").is_none());
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
