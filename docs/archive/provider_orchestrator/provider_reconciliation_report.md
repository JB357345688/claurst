**1. Executive Summary**
Provider resolution in production lives today as an inline block inside [lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs#L660), not in a shared helper or dedicated `provider_resolution` module. Root sessions are genuinely multi-provider because CLI wiring builds both `ProviderRegistry` and `ModelRegistry` at startup in [main.rs](/home/jordi/claurst/src-rust/crates/cli/src/main.rs#L608) and [main.rs](/home/jordi/claurst/src-rust/crates/cli/src/main.rs#L711), then hands them to `run_query_loop()` in [main.rs](/home/jordi/claurst/src-rust/crates/cli/src/main.rs#L1113).

D1 is only partially landed. The root loop can resolve and dispatch providers, but workers do not inherit that path. `AgentTool` and the injected team runner still construct Anthropic clients directly in [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs#L229) and [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs#L528), and `AgentTool` explicitly strips shared registries from child `QueryConfig` in [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs#L339).

D2 is not present in real production orchestration. Repo search found no production implementation for `TrustDomain`, `allow_fallback`, `ProviderModelConflict`, or `provider_resolution`; the only D2-like logic is the test-only surrogate called out in [d2_test_micro_patch_report.md](/home/jordi/claurst/d2_test_micro_patch_report.md#L3).

**2. Current Production Provider-Resolution Flow**
Root provider selection is handled inline in `run_query_loop()` starting at the provider-dispatch block in [lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs#L854). The actual decision order is visible in code, not abstracted elsewhere: explicit `config.provider` except `"anthropic"` first, then `provider/model` parsing, then `ModelRegistry::find_provider_for_model()` in [model_registry.rs](/home/jordi/claurst/src-rust/crates/api/src/model_registry.rs#L208), then default to Anthropic in [lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs#L863).

The orchestration is scattered overall. Root resolution is centralized in one inline block, but sub-agents and teams bypass it. `AgentTool` accepts only a `model` override in [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs#L129), creates an `AnthropicClient` directly in [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs#L242), and passes a child `QueryConfig` with `provider_registry: None` and `model_registry: None` in [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs#L356). Team workers are even narrower: `AgentRunFn` has no provider or model parameter in [team_tool.rs](/home/jordi/claurst/src-rust/crates/tools/src/team_tool.rs#L47), `AgentSpec` has no provider or model fields in [team_tool.rs](/home/jordi/claurst/src-rust/crates/tools/src/team_tool.rs#L172), and `TeamCreate` ultimately calls that runner in [team_tool.rs](/home/jordi/claurst/src-rust/crates/tools/src/team_tool.rs#L415).

Anthropic is still explicitly special-cased. Root resolution filters out `config.provider == "anthropic"` in [lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs#L863), conditionally falls back to provider-dispatch only when the raw Anthropic client is unusable in [lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs#L931), and still creates a fresh Anthropic client for session-memory extraction in [lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs#L1599).

**3. D1 Implementation Mapping**
- `Shared provider resolution path`: `PARTIAL`. Root has a real path in [lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs#L854), but it is inline and not reusable by workers.
- `Explicit provider pinning semantics`: `PARTIAL`. Non-Anthropic `config.provider` acts like a pin in [lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs#L863), but Anthropic is not treated as a true pin, and there is no worker-level provider surface.
- `Parent/worker provider inheritance`: `MISSING`. Child agents discard both registries in [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs#L356), and TeamCreate’s runner interface has no provider propagation surface in [team_tool.rs](/home/jordi/claurst/src-rust/crates/tools/src/team_tool.rs#L47).
- `Fail-loud vs fail-open`: `PARTIAL`. Non-Anthropic root dispatch fails loudly when no provider can be materialized, but Anthropic still has a raw-client escape hatch in [lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs#L931). Worker paths hard-fail on missing `ANTHROPIC_API_KEY` in [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs#L229) and [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs#L528).
- `Removal of Anthropic special-casing`: `MISSING`. The special case remains in both root and worker paths.
- `Provider/model conflict handling`: `MISSING`. There is no production `ProviderModelConflict`; model-family heuristics exist in [model_registry.rs](/home/jordi/claurst/src-rust/crates/api/src/model_registry.rs#L214), but explicit conflict detection does not.

**4. D2 Implementation Mapping**
- `Trust-domain classification`: `MISSING`. No production `TrustDomain` exists; only the surrogate test/report discusses it in [d2_test_micro_patch_report.md](/home/jordi/claurst/d2_test_micro_patch_report.md#L3).
- `Fallback candidate enumeration`: `MISSING`. There is no production candidate-scoping layer; the closest real production primitives are provider inventory in [registry.rs](/home/jordi/claurst/src-rust/crates/api/src/registry.rs#L21) and provider runtime materialization in [registry.rs](/home/jordi/claurst/src-rust/crates/api/src/registry.rs#L73).
- `Same-domain fallback behavior`: `MISSING`. The only fallback I found is model fallback on overloaded/rate-limit conditions inside the root loop, not provider fallback.
- `allow_fallback`: `MISSING`. No production hits.
- `Health/capability-based fallback filtering`: `PARTIAL`. Provider health exists in [registry.rs](/home/jordi/claurst/src-rust/crates/api/src/registry.rs#L132), and capability shaping exists in [lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs#L994), but neither is used to enumerate provider fallback candidates.
- `Cancellation/budget behavior for provider orchestration`: `PARTIAL`. Generic cancellation and budget guards exist in [lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs#L718) and [lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs#L1390), and team cancellation exists in [team_tool.rs](/home/jordi/claurst/src-rust/crates/tools/src/team_tool.rs#L369), but there is no provider-aware fallback budget/cancellation layer.

**5. Real D2 Seam Identification**
The first real production seam is the existing inline provider-selection and provider-materialization block in [lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs#L854). That is where root sessions already combine `config.provider`, `provider/model` parsing, `ModelRegistry`, auth-store runtime materialization, and local-provider overrides. D2 does not have a real home until that block becomes a reusable query-layer seam that worker paths can call too.

`crates/api/src/registry.rs` is not the seam. It owns provider inventory, health, and materialization primitives, but not orchestration policy. TeamCreate is also not the right first seam because it currently has no provider-resolution surface of its own and just delegates to a runner in [team_tool.rs](/home/jordi/claurst/src-rust/crates/tools/src/team_tool.rs#L76).

**6. Surrogate Test Judgment**
Keep the surrogate test temporarily, but treat it as an RFC-anchor/spec-proxy only. That matches the report’s own framing in [d2_test_micro_patch_report.md](/home/jordi/claurst/d2_test_micro_patch_report.md#L11). It is useful because it records intended D2 semantics against real registry primitives without mutating production behavior.

It should be replaced, not canonized. The removal condition is specific: remove or rewrite it once real production fallback candidate enumeration exists at the query/orchestration layer adjacent to the current `run_query_loop()` provider-resolution block.

**7. Reconciliation Matrix**

| RFC concept | Expected location per RFC | Actual current location in code | Status | Notes / evidence |
|---|---|---|---|---|
| Shared provider resolution path | Query-level shared helper/module | Inline root-only block in [lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs#L854) | PARTIAL | Real logic exists, but only root uses it |
| Explicit provider pinning | Shared resolution path plus worker inputs | Root inline logic in [lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs#L863) | PARTIAL | Anthropic is still exempted from true-pin behavior |
| Parent/worker provider inheritance | Agent/Team runner using shared query path | Child config strips registries in [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs#L356); Team runner has no provider/model params in [team_tool.rs](/home/jordi/claurst/src-rust/crates/tools/src/team_tool.rs#L47) | MISSING | Workers do not inherit root provider selection |
| Anthropic special-casing removed | None | Still present in [lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs#L863), [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs#L229), [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs#L528) | MISSING | Root and workers remain Anthropic-biased |
| Provider/model conflict handling | Shared resolution helper | No production implementation found | MISSING | No `ProviderModelConflict` hit in production search |
| Trust-domain classification | Query/provider-resolution layer | None | MISSING | Only surrogate/test artifact exists |
| Fallback candidate enumeration | Query/provider-resolution layer | None | MISSING | Registry has primitives, not orchestration |
| Health/capability fallback filter | Query/provider-resolution using registry/caps | Health in [registry.rs](/home/jordi/claurst/src-rust/crates/api/src/registry.rs#L132), capabilities in [lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs#L994) | PARTIAL | Building blocks exist, fallback logic does not |
| Provider-aware cancellation/budget | Worker orchestration layer | Generic guards in [lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs#L1390) and [team_tool.rs](/home/jordi/claurst/src-rust/crates/tools/src/team_tool.rs#L369) | PARTIAL | No provider-aware policy |

**8. Recommended Next Step**
`3. D2 seam-extraction prep`

That is the correct next move because the missing piece is not “one more D2 rule”; it is the absence of a real shared production seam where D2 could live. The narrowest honest next step is to extract the existing root provider-resolution/materialization logic from [lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs#L854) into a reusable query-layer seam and prove that workers can call it without changing policy yet. Until that exists, any D2 work remains either speculative or surrogate-only.

**9. Exact Files / Functions / Commands Used as Evidence**
Key files and functions:
- [lib.rs](/home/jordi/claurst/src-rust/crates/query/src/lib.rs#L660)
- [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs#L221)
- [agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs#L517)
- [team_tool.rs](/home/jordi/claurst/src-rust/crates/tools/src/team_tool.rs#L47)
- [team_tool.rs](/home/jordi/claurst/src-rust/crates/tools/src/team_tool.rs#L168)
- [registry.rs](/home/jordi/claurst/src-rust/crates/api/src/registry.rs#L21)
- [model_registry.rs](/home/jordi/claurst/src-rust/crates/api/src/model_registry.rs#L208)
- [openai_compat_providers.rs](/home/jordi/claurst/src-rust/crates/api/src/providers/openai_compat_providers.rs#L17)
- [main.rs](/home/jordi/claurst/src-rust/crates/cli/src/main.rs#L608)
- [FUNCTIONAL_SPEC.md](/home/jordi/claurst/FUNCTIONAL_SPEC.md#L340)
- [Codex5.4_FEATURE_BRAINSTORM.md](/home/jordi/claurst/Codex5.4_FEATURE_BRAINSTORM.md#L21)
- [d2_test_micro_patch_report.md](/home/jordi/claurst/d2_test_micro_patch_report.md#L3)

Commands used:
```bash
rg --files /home/jordi/claurst | rg '(^|/)(FUNCTIONAL_SPEC\.md|FEATURE_BRAINSTORM\.md|Codex5\.4_FEATURE_BRAINSTORM\.md|RFC_PROVIDER_AWARE_WORKER_FABRIC.*\.md|d2_test_micro_patch_report\.md)$'
rg --files /home/jordi/claurst/src-rust | rg 'crates/query/src/(lib|agent_tool|coordinator)\.rs$|crates/tools/src/team_tool\.rs$|crates/api/src/(registry|model_registry|provider|provider_types)\.rs$|crates/api/src/providers/openai_compat_providers\.rs$'
rg -n 'TrustDomain|allow_fallback|ProviderModelConflict|SessionBudget|HealthCache|resolve_provider_identity|materialize_provider|resolve_provider_with_fallback|provider_resolution' /home/jordi/claurst/src-rust/crates/query /home/jordi/claurst/src-rust/crates/api /home/jordi/claurst/src-rust/crates/tools
rg -n 'run_query_loop|provider_registry|model_registry|filter\\(\\|p\\| \\*p != "anthropic"\\)|AnthropicClient::new|find_provider_for_model|TeamCreate|AgentRunFn' /home/jordi/claurst/src-rust/crates/query /home/jordi/claurst/src-rust/crates/api /home/jordi/claurst/src-rust/crates/tools /home/jordi/claurst/src-rust/crates/cli
nl -ba /home/jordi/claurst/src-rust/crates/query/src/lib.rs | sed -n '660,985p'
nl -ba /home/jordi/claurst/src-rust/crates/query/src/lib.rs | sed -n '985,1010p;1388,1406p;1596,1611p'
nl -ba /home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs | sed -n '120,610p'
nl -ba /home/jordi/claurst/src-rust/crates/tools/src/team_tool.rs | sed -n '40,545p'
nl -ba /home/jordi/claurst/src-rust/crates/api/src/model_registry.rs | sed -n '200,265p'
nl -ba /home/jordi/claurst/src-rust/crates/api/src/registry.rs | sed -n '1,390p'
nl -ba /home/jordi/claurst/src-rust/crates/api/src/providers/openai_compat_providers.rs | sed -n '15,60p'
nl -ba /home/jordi/claurst/src-rust/crates/cli/src/main.rs | sed -n '600,730p;1108,1128p'
nl -ba /home/jordi/claurst/FUNCTIONAL_SPEC.md | sed -n '333,349p;614,641p'
nl -ba /home/jordi/claurst/Codex5.4_FEATURE_BRAINSTORM.md | sed -n '4,10p;21,31p;79,84p'
nl -ba /home/jordi/claurst/d2_test_micro_patch_report.md | sed -n '1,40p'
```

No code, tests, or RFC files were edited in this pass.
