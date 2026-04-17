1. Short execution summary

I added one test-only patch in [registry.rs](/home/jordi/claurst/src-rust/crates/api/src/registry.rs). It is an executable-spec regression around the existing registry/health/capability seams, because this checkout does not yet contain production D2 fallback/trust-domain code (`provider_resolution.rs`, `TrustDomain`, `allow_fallback`, candidate enumeration, or `ProviderModelConflict` are absent).

2. Exact files changed

- [registry.rs](/home/jordi/claurst/src-rust/crates/api/src/registry.rs)

3. Why this test location is the narrowest correct place

- The requested D2 candidate-enumeration layer does not exist in production code in this checkout.
- [registry.rs](/home/jordi/claurst/src-rust/crates/api/src/registry.rs) is the closest existing seam that already owns:
  - provider registration
  - provider IDs
  - provider health
  - provider capabilities
- That let me add a test-only executable spec without changing production behavior or inventing new config/policy surfaces.

4. The patch diff

```diff
diff --git a/src-rust/crates/api/src/registry.rs b/src-rust/crates/api/src/registry.rs
index cc245bd..203dd0e 100644
--- a/src-rust/crates/api/src/registry.rs
+++ b/src-rust/crates/api/src/registry.rs
@@ -386,8 +386,212 @@ impl ProviderRegistry {
     }
 }

-impl Default for ProviderRegistry {
-    fn default() -> Self {
-        Self::new()
-    }
-}
+impl Default for ProviderRegistry {
+    fn default() -> Self {
+        Self::new()
+    }
+}
+
+#[cfg(test)]
+mod tests {
+    use super::*;
+    use crate::provider::{LlmProvider, ModelInfo};
+    use crate::provider_error::ProviderError;
+    use crate::provider_types::{
+        ProviderCapabilities, ProviderRequest, ProviderResponse, ProviderStatus, StreamEvent,
+        SystemPromptStyle,
+    };
+    use async_trait::async_trait;
+    use claurst_core::ModelId;
+    use futures::stream;
+    use std::collections::BTreeSet;
+    use std::pin::Pin;
+
+    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
+    enum TestTrustDomain {
+        Local,
+        Cloud,
+    }
+
+    fn trust_domain_for_provider(provider_id: &str) -> TestTrustDomain {
+        match provider_id {
+            ProviderId::OLLAMA | ProviderId::LM_STUDIO | ProviderId::LLAMA_CPP => {
+                TestTrustDomain::Local
+            }
+            _ => TestTrustDomain::Cloud,
+        }
+    }
+
+    async fn cloud_fallback_candidate_ids(
+        registry: &ProviderRegistry,
+        require_tool_calling: bool,
+    ) -> BTreeSet<String> {
+        let mut candidates = BTreeSet::new();
+
+        for provider_id in registry.provider_ids() {
+            if trust_domain_for_provider(provider_id) != TestTrustDomain::Cloud {
+                continue;
+            }
+
+            let provider = registry
+                .get(provider_id)
+                .expect("registered provider should be retrievable");
+
+            if !matches!(provider.health_check().await, Ok(ProviderStatus::Healthy)) {
+                continue;
+            }
+
+            if require_tool_calling && !provider.capabilities().tool_calling {
+                continue;
+            }
+
+            candidates.insert(provider_id.to_string());
+        }
+
+        candidates
+    }
+
+    #[derive(Clone)]
+    struct TestProvider {
+        id: ProviderId,
+        name: &'static str,
+        health: ProviderStatus,
+        capabilities: ProviderCapabilities,
+    }
+
+    impl TestProvider {
+        fn new(
+            id: impl Into<ProviderId>,
+            name: &'static str,
+            health: ProviderStatus,
+            tool_calling: bool,
+        ) -> Self {
+            Self {
+                id: id.into(),
+                name,
+                health,
+                capabilities: ProviderCapabilities {
+                    streaming: true,
+                    tool_calling,
+                    thinking: false,
+                    image_input: false,
+                    pdf_input: false,
+                    audio_input: false,
+                    video_input: false,
+                    caching: false,
+                    structured_output: false,
+                    system_prompt_style: SystemPromptStyle::SystemMessage,
+                },
+            }
+        }
+
+        fn unused_error(&self, message: &str) -> ProviderError {
+            ProviderError::Other {
+                provider: self.id.clone(),
+                message: message.to_string(),
+                status: None,
+                body: None,
+            }
+        }
+    }
+
+    #[async_trait]
+    impl LlmProvider for TestProvider {
+        fn id(&self) -> &ProviderId {
+            &self.id
+        }
+
+        fn name(&self) -> &str {
+            self.name
+        }
+
+        async fn create_message(
+            &self,
+            _request: ProviderRequest,
+        ) -> Result<ProviderResponse, ProviderError> {
+            Err(self.unused_error("create_message is unused in registry tests"))
+        }
+
+        async fn create_message_stream(
+            &self,
+            _request: ProviderRequest,
+        ) -> Result<
+            Pin<Box<dyn futures::Stream<Item = Result<StreamEvent, ProviderError>> + Send>>,
+            ProviderError,
+        > {
+            Ok(Box::pin(stream::empty()))
+        }
+
+        async fn list_models(&self) -> Result<Vec<ModelInfo>, ProviderError> {
+            Ok(vec![ModelInfo {
+                id: ModelId::new("test-model"),
+                provider_id: self.id.clone(),
+                name: "Test Model".to_string(),
+                context_window: 8192,
+                max_output_tokens: 1024,
+            }])
+        }
+
+        async fn health_check(&self) -> Result<ProviderStatus, ProviderError> {
+            Ok(self.health.clone())
+        }
+
+        fn capabilities(&self) -> ProviderCapabilities {
+            self.capabilities.clone()
+        }
+    }
+
+    #[tokio::test]
+    async fn unknown_custom_providers_default_to_cloud_and_enter_cloud_candidate_scope() {
+        // D2 candidate enumeration is not implemented in production in this
+        // checkout. This test anchors the RFC v3.1 rule to the existing
+        // registry/health/capability seams so the built-in-only trust-domain
+        // behavior stays explicit and local.
+        let mut registry = ProviderRegistry::new();
+        registry
+            .register(Arc::new(TestProvider::new(
+                ProviderId::OPENAI,
+                "OpenAI",
+                ProviderStatus::Healthy,
+                true,
+            )))
+            .register(Arc::new(TestProvider::new(
+                "custom-vllm",
+                "Custom vLLM",
+                ProviderStatus::Healthy,
+                true,
+            )))
+            .register(Arc::new(TestProvider::new(
+                ProviderId::OLLAMA,
+                "Ollama",
+                ProviderStatus::Healthy,
+                true,
+            )))
+            .register(Arc::new(TestProvider::new(
+                "custom-no-tools",
+                "Custom No Tools",
+                ProviderStatus::Healthy,
+                false,
+            )))
+            .register(Arc::new(TestProvider::new(
+                "custom-unhealthy",
+                "Custom Unhealthy",
+                ProviderStatus::Unavailable {
+                    reason: "down".to_string(),
+                },
+                true,
+            )));
+
+        let candidates = cloud_fallback_candidate_ids(&registry, true).await;
+
+        assert_eq!(
+            trust_domain_for_provider("custom-vllm"),
+            TestTrustDomain::Cloud
+        );
+        assert!(candidates.contains(ProviderId::OPENAI));
+        assert!(candidates.contains("custom-vllm"));
+        assert!(!candidates.contains(ProviderId::OLLAMA));
+        assert!(!candidates.contains("custom-no-tools"));
+        assert!(!candidates.contains("custom-unhealthy"));
+    }
+}
```

5. Tests run

- Attempted: `cargo test -p claurst-api unknown_custom_providers_default_to_cloud_and_enter_cloud_candidate_scope`
- Could not run: `cargo` is not installed or not available on PATH in this environment (`/bin/bash: cargo: command not found`)

6. Optional D1 cleanup I found but did not include

- I found no existing reverse explicit-pin conflict coverage.
- I also found no `ProviderModelConflict` implementation in this checkout.
- Current code still special-cases Anthropic in [lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs#L863) with `filter(|p| *p != "anthropic")`.

That means the optional D1 cleanup is larger than “add one missing test”; it is currently an implementation gap plus a missing test, so I left it out.

7. Confirmation that `RFC_PROVIDER_AWARE_WORKER_FABRIC_v3.1.md` was left untouched

- I did not edit [RFC_PROVIDER_AWARE_WORKER_FABRIC_v3.1.md](/home/jordi/claurst/RFC_PROVIDER_AWARE_WORKER_FABRIC_v3.1.md).
- It already appears as untracked in this worktree, but I made no changes to it.

8. If any doc change was truly required

- No doc change was required.
- [RFC_PROVIDER_AWARE_WORKER_FABRIC_v3.2.md](/home/jordi/claurst/RFC_PROVIDER_AWARE_WORKER_FABRIC_v3.2.md) was not created.
