# TASK-M9-02 Preflight Report

## Ticket
`TASK-M9-02`

## Timestamp UTC
`20260413T054655Z`

## Branch
`feature/provider-resolution-seam`

## Verdict
`READY-WITH-NOTES`

## Repo State Summary
- `git branch --show-current` -> `feature/provider-resolution-seam`
- `git status --short --branch` -> branch header matched expectation; no tracked modifications were present; no staged changes were present; substantial untracked workspace noise exists under `.codex/`, `docs/Current/`, `docs/Orchestrator_planning/`, `docs/archive/provider_orchestrator/`, `docs/archive/reports/`, and `src-rust/target/`
- `git diff --name-only` -> empty
- `git diff --cached --name-only` -> empty
- `git log --oneline --decorate -n 20` -> `HEAD` is `b5b6dd4 (HEAD -> feature/provider-resolution-seam) TASK-M8-11 reconcile M8 workspace validation and formatting`
- Repo state is compatible with starting `TASK-M9-02` on tracked files because the tracked worktree is clean; existing untracked noise should still be called out later as review-basis noise

## Authority Reviewed
- `AGENTS.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md`
- `docs/archive/reports/TASK-M8-11_REVIEW_REPORT_20260413T002956Z.md`
- `docs/archive/reports/TASK-M9-01_EXECUTION_REPORT_20260413T050804Z.md`

## Dependency Baseline Confirmed
- `TASK-M8-11` is the accepted tracked baseline on this branch:
  - `HEAD` commit is the `TASK-M8-11` reconciliation commit `b5b6dd4`
  - the tracked worktree is clean
  - the `TASK-M8-11` review report records `PASS` on `feature/provider-resolution-seam`
- `TASK-M9-01` is already complete and does not need reopening:
  - the `TASK-M9-01` execution report records `PASS / COMPLETE WITHOUT SOURCE CHANGE`
  - no newer tracked commit was added after `TASK-M8-11`
- Hosted Ollama compatibility remains a preserved background invariant only:
  - MPWO invariant section `2A` still governs this ticket
  - current preflight found no need to reopen or widen hosted-Ollama behavior
  - `Hosted Ollama compatibility baseline preserved`

## Exact M9-02 Contract
- Ticket: `TASK-M9-02`
- Objective: verify `materialize_provider()` tests cover auth-store refresh, `api_base` override, and missing credentials
- Allowed file surface for later execution: `src-rust/crates/query/src/provider_resolution.rs` local `#[cfg(test)]` module only
- Later execution steps authorized by MPWO:
  1. audit existing materialize tests from `M7-06`
  2. add missing coverage for:
     - auth-store key takes priority over registry
     - `api_base` for `lmstudio` / `llamacpp`
     - error on no credentials
- Do not add network-dependent tests without `#[ignore]`
- Later validation target: tests pass
- Dependency: `M8-11`

## Verified Files / Symbols / Commands
- Files verified:
  - `AGENTS.md`
  - `docs/Current/MPWO_WORK_ORDER_PACK.md`
  - `docs/archive/reports/TASK-M8-11_REVIEW_REPORT_20260413T002956Z.md`
  - `docs/archive/reports/TASK-M9-01_EXECUTION_REPORT_20260413T050804Z.md`
  - `src-rust/crates/query/src/provider_resolution.rs`
  - adjacent inspection only:
    - `src-rust/crates/api/src/registry.rs`
    - `src-rust/crates/core/src/auth_store.rs`
    - `src-rust/crates/api/src/provider.rs`
    - `src-rust/crates/api/src/providers/openai_compat.rs`
    - `src-rust/crates/api/src/providers/openai_compat_providers.rs`
    - `src-rust/crates/core/src/provider_id.rs`
    - `src-rust/crates/query/src/agent_tool.rs`
- Symbols verified:
  - `materialize_provider(...)`
  - local `#[cfg(test)] mod tests`
  - `build_ollama_provider(...)`
  - `normalize_ollama_api_base(...)`
  - `claurst_api::registry::runtime_provider_for(...)`
  - `AuthStore::load().api_key_for(...)`
  - `ProviderRegistry::register(...)`
  - `LlmProvider::{id,name,health_check,capabilities}`
- Commands verified:
  - `git branch --show-current`
  - `git status --short --branch`
  - `git diff --name-only`
  - `git diff --cached --name-only`
  - `git log --oneline --decorate -n 20`
  - read-only `rg`, `nl`, and `sed` inspections over the files above

## Current Code Reality
- `materialize_provider(...)` exists at [src-rust/crates/query/src/provider_resolution.rs](/home/jordi/claurst/src-rust/crates/query/src/provider_resolution.rs:157).
- Current branch structure inside `materialize_provider(...)`:
  - special `ollama` branch returns `build_ollama_provider(provider_configs)` immediately at lines `162-169`
  - non-ollama branch computes `runtime_provider_for(&identity.provider_id)` at line `173`
  - registry fallback is read at lines `175-179`
  - `api_base` override logic exists at lines `181-203` for:
    - `ollama`
    - `"lmstudio" | "lm-studio"`
    - `"llamacpp" | "llama-cpp"`
  - provider selection order is `runtime_provider.or(registry_provider)` at lines `205-207`
  - missing-credentials error is `ProviderResolutionError::NoCredentials(identity.provider_id.clone())` at line `207`
- Local test module exists at [src-rust/crates/query/src/provider_resolution.rs](/home/jordi/claurst/src-rust/crates/query/src/provider_resolution.rs:254).
- Existing local helper utilities inside that module:
  - `assert_identity(...)`
  - `assert_provider_model_conflict(...)`
  - `provider_identity(...)`
- Existing materialize tests currently use:
  - `ProviderRegistry::new()`
  - `ProviderRegistry::register(...)`
  - real `OpenAiProvider::new(...)`
  - plain `HashMap<String, ProviderConfig>`
- No dedicated mock `LlmProvider` exists in this module or nearby query/api test code; current materialize tests do not yet define a test double.
- Adjacent production code confirms the relevant seam behavior:
  - `runtime_provider_for(...)` calls `AuthStore::load()` and `api_key_for(...)` on each invocation in [src-rust/crates/api/src/registry.rs](/home/jordi/claurst/src-rust/crates/api/src/registry.rs:74)
  - `AuthStore::api_key_for(...)` is stored-first for non-ollama providers and env-first only for `ollama` in [src-rust/crates/core/src/auth_store.rs](/home/jordi/claurst/src-rust/crates/core/src/auth_store.rs:137)
  - `LlmProvider` exposes `id()` and `name()` but no direct `base_url` getter in [src-rust/crates/api/src/provider.rs](/home/jordi/claurst/src-rust/crates/api/src/provider.rs:50)
  - `OpenAiCompatProvider::health_check(...)` has a no-key early-return path based on base-url locality in [src-rust/crates/api/src/providers/openai_compat.rs](/home/jordi/claurst/src-rust/crates/api/src/providers/openai_compat.rs:718)

## Existing Materialize Test Audit
- `materialize_provider_returns_openai_target_from_happy_path`
  - Covers: registry-backed success path for `openai`
  - Current assertions: `provider_id`, `model_id`, `resolution_source`, and returned provider `id() == "openai"`
  - M9-02-equivalent coverage: none for auth-store precedence; this test only proves registry success
  - Weakness: it does not distinguish registry provider selection from runtime/auth-store provider selection
- `materialize_provider_returns_no_credentials_for_unknown_provider`
  - Covers: `NoCredentials(...)` error result when runtime provider and registry provider are both absent
  - Current assertions: error variant and provider string for `some-fake-provider`
  - M9-02-equivalent coverage: partial for the missing-credentials branch
  - Weakness: it uses an unknown provider ID, so it does not prove the same error path for a known provider whose auth-store lookup and registry fallback are both empty
- `materialize_provider_accepts_ollama_api_base_override`
  - Covers: `ollama` materialization success with `provider_configs["ollama"].api_base`
  - Current assertions: success plus `provider_id`, `model_id`, `resolution_source`, and returned provider `id() == "ollama"`
  - M9-02-equivalent coverage: overlapping only in the general theme of `api_base` override; not equivalent to the required `lmstudio` / `llamacpp` cases
  - Weakness: it does not assert that the override or normalization actually changed the constructed provider
- Adjacent but not counted as materialize tests:
  - `normalize_ollama_api_base_rewrites_hosted_api_root`
  - `normalize_ollama_api_base_rewrites_hosted_api_v1_root`
  - `normalize_ollama_api_base_appends_v1_for_plain_roots`
  - These preserve the hosted-Ollama baseline but do not call `materialize_provider(...)`

## M9-02 Coverage Matrix
| Required case | Existing test name(s) | Status | Basis for classification | Likely assertion needed if follow-up execution is required |
|---|---|---|---|---|
| auth-store key takes priority over registry | `materialize_provider_returns_openai_target_from_happy_path` | `MISSING` | Current tests never seed auth-backed runtime resolution and never distinguish runtime-provider selection from registry-provider selection. The production order is `runtime_provider.or(registry_provider)`, but no current test proves it. | Isolate provider auth state, seed auth-backed `openai` credentials, register a registry provider with the same `id` but a distinct `name()`, call `materialize_provider(...)`, and assert the returned provider is the runtime/auth-backed one rather than the registry double. |
| api_base override for lmstudio | `materialize_provider_accepts_ollama_api_base_override` | `MISSING` | Existing override coverage is only for `ollama`; no test exercises `"lmstudio"` or `"lm-studio"` branches in the override match arm. | Materialize `lm-studio` or `lmstudio` with a remote-looking override base, isolate `LM_STUDIO_HOST` and key env, then assert an observable override effect such as `health_check()` returning `ProviderStatus::Unavailable { reason: "No API key configured" }`, which only occurs on the remote/no-key path. |
| api_base override for llamacpp | `materialize_provider_accepts_ollama_api_base_override` | `MISSING` | Existing override coverage does not touch `"llamacpp"` or `"llama-cpp"` branches. | Materialize `llama-cpp` or `llamacpp` with a remote-looking override base, isolate `LLAMA_CPP_HOST` and key env, then assert the same remote/no-key `health_check()` behavior or equivalent observable override effect. |
| error on no credentials | `materialize_provider_returns_no_credentials_for_unknown_provider` | `PARTIAL` | The final `NoCredentials` branch is already exercised, but only through an unknown provider ID. That is weaker than a known-provider no-auth/no-registry case, which is the more directly relevant materialize seam. | Use a known runtime-capable provider such as `openai` with isolated auth state and an empty registry, then assert `Err(ProviderResolutionError::NoCredentials("openai"))`. |

## Mocking / Fixture Feasibility
- Can current test helpers drive auth-store-backed credential resolution?
  - Not directly inside `provider_resolution.rs` today.
  - Feasible without widening production code:
    - env-backed auth is already sufficient for non-ollama runtime resolution because `runtime_provider_for(...)` calls `AuthStore::load().api_key_for(...)`
    - temp-`HOME` isolation and env guards already exist as patterns in [src-rust/crates/query/src/agent_tool.rs](/home/jordi/claurst/src-rust/crates/query/src/agent_tool.rs:769) and [src-rust/crates/core/src/auth_store.rs](/home/jordi/claurst/src-rust/crates/core/src/auth_store.rs:154)
  - If the follow-up wants to prove disk-backed `AuthStore::load()` refresh specifically, a local copied helper plus temp `HOME` is sufficient.
- Can current fixtures assert `api_base` on constructed providers?
  - Not directly.
  - The returned provider is held as `Arc<dyn LlmProvider>`, and the trait exposes no `base_url` accessor.
  - Direct exact-string assertion of the constructed base URL is therefore not available from the current local test module alone.
  - Indirect behavioral assertion is feasible via existing trait methods, especially the remote/no-key early-return behavior of `OpenAiCompatProvider::health_check(...)`.
- Do current surfaces allow missing-credentials assertions without network calls?
  - Yes.
  - Empty auth state plus empty registry reaches `NoCredentials(...)` before any network operation.
- Does `#[ignore]` appear necessary?
  - No.
  - All required M9-02 branches appear unit-testable without live network dependency if env/home isolation is used and `api_base` assertions rely on non-network observable behavior.

## Likely Smallest Edit Surface For Execution
- Expected smallest correct edit surface remains:
  - `src-rust/crates/query/src/provider_resolution.rs`
  - local `#[cfg(test)] mod tests` only
- Production code changes appear unnecessary based on current read-only evidence.
- Likely test additions can stay local by adding:
  - one env/home isolation helper
  - one lightweight test-only `LlmProvider` double for registry-vs-runtime distinction, if the follow-up wants a strong auth-store precedence assertion
  - 3 to 4 new `materialize_provider_*` tests, depending on whether the existing missing-credentials test is retained as supporting coverage or replaced with a stronger known-provider case
- Recommended stable materialize-focused filter prefix for later execution:
  - `materialize_provider_`
- Candidate names that match the current code reality:
  - `materialize_provider_prefers_auth_store_provider_over_registry`
  - `materialize_provider_applies_lm_studio_api_base_override`
  - `materialize_provider_applies_llama_cpp_api_base_override`
  - `materialize_provider_returns_no_credentials_for_known_provider_without_auth`

## Validation Readiness
- Validation command for later execution was intentionally not run in this preflight session.
- Based on current read-only evidence, the ticket is validation-ready after local test additions only.
- No production seam widening is currently indicated.
- Existing clean tracked state makes later patch isolation straightforward if execution stays inside the local test module.

## Drift Found
- Minor path-prefix drift only:
  - MPWO file references use `crates/query/src/provider_resolution.rs`
  - repo reality is `src-rust/crates/query/src/provider_resolution.rs`
  - this is non-blocking and already consistent with the repository layout
- Minor provider-ID wording drift only:
  - ticket wording says `lmstudio` / `llamacpp`
  - canonical provider constants in repo are `lm-studio` / `llama-cpp`
  - `materialize_provider(...)` currently accepts both alias spellings, so this is non-blocking
- No structural drift found for:
  - file path presence
  - `materialize_provider(...)` symbol presence
  - function signature presence
  - local `#[cfg(test)]` module presence
  - available provider-registry seam

## Blockers
- None.

## Notes
- The main note for later execution is assertion shape, not scope.
- Auth-store precedence is currently untested, but the code seam exists and can be exercised locally.
- `api_base` override for `lm-studio` and `llama-cpp` is currently untested, and direct base-url introspection is unavailable through the public trait surface; follow-up tests should therefore use an observable indirect behavior rather than widen production code.
- The existing `ollama` override and normalization tests should be treated as preserved background coverage only; M9-02 does not need to reopen hosted-Ollama behavior unless a local test change would interact with it.
