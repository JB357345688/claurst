# Adversarial Engineering Review: `RFC_PROVIDER_AWARE_WORKER_FABRIC_v2.md`

## 1. Executive Verdict

**Overall verdict:** `decent RFC`, not implementation-ready.

**D1 verdict:** fundamentally the right scope, but **not implementation-ready yet**.

**D2 verdict:** directionally right, but **still under-specified and not implementation-ready**.

Blunt version: v2 is much better than v1 because it finally targets the real problem instead of inventing a subsystem. But it still leaves enough ambiguity that two competent engineers could ship materially different behavior, especially around provider/model precedence, Anthropic fallback, and D2 fallback/cancellation semantics.

## 2. What v2 Gets Right

- It identifies the real defect correctly: worker paths hardcode `AnthropicClient` and drop `provider_registry` / `model_registry`, while the root loop already has multi-provider routing. That is the actual architectural gap, and the current code confirms it in `/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:230`, `/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:356`, `/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:541`, and `/home/jordi/claurst/src-rust/crates/query/src/lib.rs:862`.
- It narrows scope versus v1 in the correct direction. v1 bundled scheduler, capability matcher, fallback engine, health probe, budgets, and access profiles into one story. v2 cuts D1 down to “extract the existing resolution path and propagate registries” (`RFC_PROVIDER_AWARE_WORKER_FABRIC_v2.md:45-51`, `:87-89`). That is the right move.
- It explicitly fixes both `AgentTool` and `TeamCreateTool` in D1. v1 deferred the team path operationally; v2 no longer does (`RFC_PROVIDER_AWARE_WORKER_FABRIC_v2.md:606-610`). That is necessary because the bug exists in both paths today, and the functional spec already treats the team runner as a first-class part of agent orchestration (`FUNCTIONAL_SPEC.md:521-522`).
- It is more honest than v1 about behavioral change. v1 claimed additive/backward-compatible behavior and “zero new tools / internal plumbing only”; v2 correctly admits the behavior change and visible schema deltas (`RFC_PROVIDER_AWARE_WORKER_FABRIC_v2.md:776-783`, `:828-836`).
- It removes v1’s worst overreach. The shift from v1’s `WorkerScheduler` / per-call `FallbackEngine` / `HealthProbe` stack (`RFC_PROVIDER_AWARE_WORKER_FABRIC.md:291-362`, `:422-509`) to v2’s “extract shared resolution first, then add spawn-time fallback later” is a real improvement.
- It correctly demotes access profiles out of this RFC. That was coupling two unrelated concerns in v1.
- It fixes the soft-budget language. The current code already enforces budget after usage is recorded at the end of a model response (`/home/jordi/claurst/src-rust/crates/query/src/lib.rs:1390-1404`), and v2 finally describes budgeting as soft rather than pretending otherwise (`RFC_PROVIDER_AWARE_WORKER_FABRIC_v2.md:519-523`).

## 3. Current Behavior vs Intended Behavior vs RFC Claims

**Current Claurst behavior**

- Root loop supports provider routing through `ProviderRegistry`, with precedence implemented inline in `run_query_loop()` (`/home/jordi/claurst/src-rust/crates/query/src/lib.rs:854-980`).
- Worker paths do not inherit that. They construct Anthropic clients directly and set child `QueryConfig.provider_registry = None` and `model_registry = None` (`/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:229-249`, `:339-360`, `:517-599`).
- Budget checks already fire after a model response is completed and usage is recorded, not before API calls (`/home/jordi/claurst/src-rust/crates/query/src/lib.rs:1390-1404`).
- `ModelRegistry` already contains some model-level metadata today: `tool_calling`, `reasoning`, `vision`, pricing, context window (`/home/jordi/claurst/src-rust/crates/api/src/model_registry.rs:22-44`).

**Intended future behavior in v2**

- D1: workers use the same provider resolution path as root, inherit registries, and can explicitly set provider/model.
- D2: add budgets, health-aware spawn-time fallback, and trust-domain safety rails.

**Claims v2 makes but does not fully support**

- It says `resolve_provider()` is an extraction of current root logic with unchanged behavior, but its signature is too narrow to preserve the current root path.
- It says explicit providers are pinned by default, but it also preserves an Anthropic fallback path that would silently unpin behavior.
- It says D2 trust-domain fallback is safe, but it does not fully specify the config surface or custom-provider contract needed to make that true.

## 4. Critical Blocking Issues

### Blocker 1: Provider/model precedence is still contradictory

**Why it matters**

This is the core behavior of D1. If precedence is ambiguous, D1 is not a handoff doc. It is a debate prompt.

**What is contradictory**

- v2 problem statement says current root priority is:
  `provider/model string` -> `config.provider` -> `ModelRegistry` -> default (`RFC_PROVIDER_AWARE_WORKER_FABRIC_v2.md:59-64`).
- v2 `resolve_provider()` says the unchanged priority is:
  `provider_override` -> `provider/model string` -> `ModelRegistry` -> registry default (`RFC_PROVIDER_AWARE_WORKER_FABRIC_v2.md:321-325`).
- Current code actually gives `config.provider` priority before a provider prefix in the model string (`/home/jordi/claurst/src-rust/crates/query/src/lib.rs:857-875`).
- v2 acceptance criteria say `resolve_provider()` must be identical to current inline logic (`RFC_PROVIDER_AWARE_WORKER_FABRIC_v2.md:978`), but also say conflicting `provider=openai` + `model=claude-opus-4-6` must return an explicit error (`:983`). Current inline logic does not define that error path.

**Risk created**

Two engineers can legitimately implement:

- “explicit provider wins”
- “provider prefix in model wins”
- “conflict errors”
- “current root behavior preserved”

Those are not the same implementation.

**Minimum correction required**

Add one normative precedence/conflict matrix for D1. It needs exact outcomes for:

- explicit `provider` + bare `model`
- explicit `provider` + `provider/model` model string
- explicit `provider` + conflicting model owner
- no explicit provider + `provider/model` string
- no explicit provider + model-registry hit
- no explicit provider + no registry

Also add an explicit error variant for provider/model conflict. Right now the proposed error enum does not include one (`RFC_PROVIDER_AWARE_WORKER_FABRIC_v2.md:338-348`), but the failure table assumes it exists (`:823`).

### Blocker 2: The Anthropic fallback escape hatch is too lenient and internally inconsistent

**Why it matters**

This is your suspected issue A, and it is real. It is more than a style problem. It directly threatens the main invariant D1 is supposed to establish.

**What is wrong**

- The root-session “after” pseudocode falls back to the Anthropic client on any `resolve_provider()` error (`RFC_PROVIDER_AWARE_WORKER_FABRIC_v2.md:545-552`).
- D1 design principle says explicit provider is pinned by default (`:291`).
- Failure mode F10 preserves a worker-path fallback when `ProviderRegistry` is missing (`:825-826`).
- OQ1 explicitly keeps the Anthropic client as a fallback for “legacy code paths” (`:1043-1048`).
- But the AgentTool pseudocode says missing registry is an error: `No ProviderRegistry available` (`:575-579`).

**Risk created**

If implemented literally, these behaviors are possible:

- `provider=openai` + missing OpenAI credentials silently routes to Anthropic.
- a worker missing inherited registry silently re-enters the old Anthropic-only path.
- different call sites fail open vs fail closed.

That destroys provider pinning, hides broken plumbing, and makes production bugs hard to diagnose.

**Minimum correction required**

State this explicitly:

- Root session may use `client: &AnthropicClient` only when `provider_registry` is `None`.
- If `provider_registry` is present and `resolve_provider()` returns an error, the call fails. No silent Anthropic fallback.
- Worker paths never silently fall back to Anthropic when registry/context propagation is missing. Missing registry in a worker path is an invariant/configuration error.

That is the minimum safe rule set.

### Blocker 3: `resolve_provider()` as specified cannot preserve current root behavior

**Why it matters**

v2 sells D1 as a safe extraction of existing logic. The proposed interface does not actually carry enough information to do that.

**What is missing**

Current root dispatch does more than choose `(provider_id, model_id)`:

- it re-materializes a provider from the auth store so `/connect` updates can take effect immediately (`/home/jordi/claurst/src-rust/crates/query/src/lib.rs:937-949`)
- it applies per-provider `api_base` overrides for local providers (`:952-977`)

But the proposed `resolve_provider()` signature only receives:

- `model`
- `provider_override`
- `registry`
- `model_registry`

See `RFC_PROVIDER_AWARE_WORKER_FABRIC_v2.md:331-336`.

There is also an internal inconsistency:

- the diagram shows `resolve_provider(&effective_model, &config, &tool_ctx.config)` (`RFC_PROVIDER_AWARE_WORKER_FABRIC_v2.md:237-240`)
- the actual signature does not take either config object

**Risk created**

If an engineer follows the signature literally, D1 regresses existing root behavior around:

- runtime credential refresh after `/connect`
- local-provider `api_base` override handling

That is exactly the kind of hidden regression an “extract the code” RFC is supposed to prevent.

**Minimum correction required**

Either:

- split the design into `resolve_provider_identity()` and `materialize_provider_instance()`

or:

- extend the contract so provider materialization has access to `provider_configs` and the runtime-auth-store rules

The RFC must say which one.

### Blocker 4: D2’s fallback and trust-domain control surface is not actually specified

**Why it matters**

This is your suspected issue C, and it is real.

**What is missing**

- The RFC repeatedly references `allow_cross_domain_fallback` (`RFC_PROVIDER_AWARE_WORKER_FABRIC_v2.md:220`, `:446`, `:798`, `:997`).
- That field is not present in the Agent schema (`:664-680`).
- It is not present in the TeamCreate schema (`:690-696`).
- It is not present in `AgentRunParams` (`:720-733`).

So the internal logic references a control surface that the external interface never defines.

**Risk created**

Two engineers can implement completely different escape hatches:

- per-worker tool field
- session-level config
- hidden internal flag
- no escape hatch at all

That is not acceptable for a security-adjacent policy surface.

**Minimum correction required**

For D2, define exactly where `allow_cross_domain_fallback` lives:

- Agent field?
- TeamCreate agent field?
- global/session config?
- all of the above with precedence?

If you do not want to define it now, delete it from D2 and forbid cross-domain fallback entirely.

### Blocker 5: D2 custom-local-provider trust-domain support is hand-wavy, not implementable

**Why it matters**

This is your suspected issue D, and it is also real.

**What is wrong**

v2 says provider-id-based trust domain is imperfect and suggests a config example for local OpenAI-compatible endpoints (`RFC_PROVIDER_AWARE_WORKER_FABRIC_v2.md:1070-1088`).

But the current config and registry model do not match that example:

- current `ProviderConfig` has `api_key`, `api_base`, `enabled`, whitelists, and `options` (`/home/jordi/claurst/src-rust/crates/core/src/lib.rs:667-686`)
- it does not have `type`
- it does not have `trust_domain`
- it does not use `base_url`; the existing field is `api_base`
- the registry only knows hardcoded provider IDs in `provider_from_key()` (`/home/jordi/claurst/src-rust/crates/api/src/registry.rs:26-70`)

So the example config in OQ6 is not describing an existing extension point. It is describing another feature.

**Risk created**

An engineer implementing D2 will either:

- silently ignore custom local endpoints
- invent ad hoc config parsing
- conflate arbitrary provider IDs with built-in providers

**Minimum correction required**

Pick one:

- D2 only supports built-in provider IDs for trust-domain classification; custom provider trust-domain override is explicitly deferred.
- or D2 expands `ProviderConfig` and provider registration to support custom provider type + `trust_domain`, and the RFC documents that contract fully.

Right now it does neither.

### Blocker 6: D2 capability matching depends on metadata semantics the RFC does not pin down

**Why it matters**

This is your suspected issue E. It is real.

**What is under-specified**

v2 says D2 capability matching must be model-level, with provider-level fallback when model data is absent (`RFC_PROVIDER_AWARE_WORKER_FABRIC_v2.md:299`, `:388-390`, `:1050-1052`).

But the current `ModelRegistry` state is mixed:

- model-level metadata already exists today (`tool_calling`, `reasoning`, `vision`) in `/home/jordi/claurst/src-rust/crates/api/src/model_registry.rs:22-44`
- the models.dev parse only fills some fields and defaults `vision` to `false` (`:458-486`)
- there is no data for `pdf_input`, `audio_input`, `video_input`, or `structured_output`

The RFC does not define:

- whether “unknown” means false
- whether “unknown” falls back to provider-level capability per-field or per-model
- whether stale bundled data outranks provider claims
- how to rank fallback targets when several models are eligible

**Risk created**

Different engineers will either over-filter or over-permit models. Both are bad.

**Minimum correction required**

Define a field-by-field unknown-data policy for D2. For each capability:

- `KnownTrue` => eligible
- `KnownFalse` => ineligible
- `Unknown` => either fallback to provider-level flag or reject

Also define how a fallback provider chooses a model. “Find another provider in the same trust domain” is not enough. The doc must say whether to use `best_model_for_provider()`, a cost-aware selector, or something else.

### Blocker 7: D2 cancellation propagation is not specified tightly enough for current worker architecture

**Why it matters**

This is the real core of your suspected issue F.

**What is vague**

The budget timing itself is mostly fine. v2 now says checks happen after a completed API turn and overshoot by at most one round-trip (`RFC_PROVIDER_AWARE_WORKER_FABRIC_v2.md:519-523`), which matches current root-loop behavior closely enough.

The problem is cancellation propagation:

- v2 says session budget exceeded means “cancellation tokens fired for all workers” (`:821-822`, `:994`)
- current TeamCreate uses one token set around the outer future join (`/home/jordi/claurst/src-rust/crates/tools/src/team_tool.rs:369-425`)
- current agent runner creates a separate internal token for `run_query_loop()` (`/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:383-395`, `:586-599`)

The RFC never defines:

- which token is authoritative
- how root session budget cancellation reaches TeamCreate workers
- how it reaches background agents
- whether nested workers inherit/cascade cancellation
- whether cancellation interrupts tool execution or only the next model request

**Risk created**

D2 budget enforcement can look correct on paper while failing to stop the actual work.

**Minimum correction required**

Add one explicit cancellation propagation section for D2:

- who owns the session budget
- where tokens live
- how TeamCreate, background agents, and nested agents subscribe
- the precise stop point: during stream, after stream, before next tool call, before next API call

Without that, D2 is not ready.

## 5. Non-Blocking but Important Issues

- The RFC under-describes the current starting point of `ModelRegistry`. It is not a blank slate; some model-level metadata already exists. That matters for scope estimates and migration planning.
- `ctx.worker_scheduler_registry` in the AgentTool snippet (`RFC_PROVIDER_AWARE_WORKER_FABRIC_v2.md:576`) does not match the proposed `ToolContext` fields (`:764-770`). That is a document bug, but it signals sloppiness in the exact handoff surface.
- Rollout task `1.8` says “Add `provider` and `model` to Agent/TeamCreate input schemas” (`RFC_PROVIDER_AWARE_WORKER_FABRIC_v2.md:925`). `model` already exists on `Agent` today (`/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:144-146`, `:200-203`). `model` is new only for TeamCreate.
- D2 never defines how `ProviderStatus::Degraded` should behave in fallback selection. Healthy vs Degraded vs Unavailable ordering matters.
- D2 does not define interaction with the existing `fallback_model` path already present in `QueryConfig` and used by the root loop today (`FUNCTIONAL_SPEC.md:334-338`, `/home/jordi/claurst/src-rust/crates/query/src/lib.rs:1299-1319`). If that remains active, “spawn-time only” is not the whole story.
- The D1 LoC estimate is probably low once you account for preserving runtime auth refresh and `api_base` override behavior.

## 6. Validate or Reject Suspected Issues A-G

| Issue | Verdict | Why |
|---|---|---|
| A. Missing-provider fallback in worker paths may be too lenient | **Valid** | v2 is internally inconsistent here, and preserving Anthropic fallback in worker paths would hide a broken invariant. Root fallback on any resolution error is also too lenient. |
| B. Provider/model precedence is still not fully locked down | **Valid** | The RFC contradicts itself and the current code. AC1 and AC6 cannot both be true without a new explicit matrix. |
| C. Cross-domain fallback control surface is under-specified | **Valid** | `allow_cross_domain_fallback` is referenced throughout D2 but never added to any schema or config surface. |
| D. Trust-domain override for self-hosted OpenAI-compatible endpoints is under-specified | **Valid** | The proposed config example does not match the current config/registry model, and custom provider registration is not actually specified. |
| E. D2 depends on model metadata completeness | **Valid** | The RFC does not define how partial or stale metadata is handled, and current model metadata is incomplete for the full capability set D2 wants. |
| F. Budget and cancellation semantics may still be too vague | **Partially valid** | Budget check timing is much better specified in v2. Cancellation propagation still is not. |
| G. Minor schema/document consistency issue around `model` field | **Partially valid** | Mostly editorial, but real. `model` already exists for `Agent`, is new for `TeamCreate`, and the rollout table blurs that. There is also a naming inconsistency in the proposed ToolContext access path. |

## 7. Required v2.1 Redlines

These are the shortest edits that materially improve safety.

1. Add a single normative precedence/conflict table for `provider`, `model`, `provider/model`, inherited `config.provider`, `ModelRegistry`, and no-registry cases.
2. Rewrite the fallback semantics so Anthropic fallback is allowed only for root sessions with `provider_registry=None`. All resolution errors with a present registry must fail loudly. Worker paths must fail loudly on missing registry.
3. Fix the `resolve_provider()` contract so it can preserve current root behavior, including runtime auth-store refresh and local `api_base` overrides. Either expand the contract or split resolution from provider materialization.
4. Add the missing D2 control surfaces explicitly: `allow_cross_domain_fallback` placement, precedence, and default. If not ready, remove it from v2 entirely.
5. Either fully specify custom local provider support for trust domains or explicitly defer it. Do not leave a fake example config in OQ6 that the current system cannot represent.
6. Add a D2 subsection for capability metadata semantics: unknown-field handling, provider-level fallback policy, and model selection order inside a fallback provider.
7. Add a D2 subsection for cancellation propagation with exact token ownership and stop points.

## 8. Implementation Safety Judgment

**Would I allow coding to begin from D1 now?** No.

**Would I allow coding to begin from D2 now?** No.

**Would I allow D1 after a small RFC patch?** Yes.

**Would I require a full rewrite?** No.

Practical judgment:

- D1 should start only after a small v2.1 patch that fixes precedence, error/fallback semantics, and the provider-materialization contract.
- D2 should not start from this document. It needs another tightening pass first.

## 9. Smallest Viable v2.1 Patch Set

If you want the minimal patch rather than a rewrite, do this:

- Patch D1 only:
  add the precedence matrix, forbid silent Anthropic fallback in worker paths, and fix the `resolve_provider()` contract so it preserves current root behavior.
- Patch D2 only enough to stop false confidence:
  either remove `allow_cross_domain_fallback`, custom trust-domain override, and capability matching from this RFC for now, or define them properly.
- Keep the rest of v2 intact:
  the narrowed scope, the D1/D2 split, and the “fix both Agent and TeamCreate in D1” decision are all correct.

## 10. Bottom Line

v2 is the first version that is pointed at the real problem. That is a meaningful improvement over v1.

But it is still not a safe implementation handoff. D1 is close and salvageable with a small patch. D2 is still a concept package, not an implementation spec.
