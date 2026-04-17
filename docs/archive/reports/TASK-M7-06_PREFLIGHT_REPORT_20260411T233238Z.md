# TASK-M7-06 Preflight Report

- Ticket ID: `TASK-M7-06`
- Verdict: `READY`
- Current branch: `feature/provider-resolution-seam`
- No source edits were made. The only artifact created by this preflight is this report file.

## Git Working Tree Summary

State verified before report creation:

- Branch: `feature/provider-resolution-seam`
- Staged changes: none
- Unstaged tracked changes: none
- Untracked files: present
- Untracked file count before report creation: `19347`
- Untracked summary before report creation:
  - `docs/Current`: `5`
  - `docs/Orchestrator_planning`: `5`
  - `docs/archive`: `45`
  - `src-rust/target`: `19292`

Notes:

- `git status --short --branch` showed only untracked `docs/` and `src-rust/target/`.
- No git state was altered via staging, committing, checkout, reset, or similar operations.

## Baseline Commit Verification Results

All required baseline commits are present in current branch history.

| Requested baseline | Verified commit | Subject | In current branch history |
|---|---|---|---|
| `a09b3da` | `a09b3daefe887f2794c9fc2154afd8ebc8b3ec27` | `Establish provider resolution seam baseline` | Yes |
| `5881983` | `58819832c1385d64d0e8f9c4d68ff18f5a96fd05` | `TASK-M7-04 wire run_query_loop through provider resolution seam` | Yes |
| `255e3c7` | `255e3c7391eb1b02e79188bdf37792ccc86a7544` | `Cleanup remove obsolete provider worker fabric RFCs` | Yes |
| `5f8dfe1` | `5f8dfe1edd3b0b2c3c064b463948080bcc5b188b` | `Fix hosted Ollama compatibility on provider seam` | Yes |
| `d76e8fb731473b5abf09c05ce885a0c4721233b9` | `d76e8fb731473b5abf09c05ce885a0c4721233b9` | `test(provider_resolution): add P1-P12 resolve_provider_identity coverage` | Yes |
| `865767881c683bb70c5bd253740ae0c5107b3752` | `865767881c683bb70c5bd253740ae0c5107b3752` | `chore(gitignore): ignore local root prompt files` | Yes |

Blockers from baseline verification: none.

## Verified Authority Sources

- `/home/jordi/claurst/AGENTS.md`
- `/home/jordi/claurst/docs/Current/MPWO_WORK_ORDER_PACK.md`

## TASK-M7-06 Contract Verified From MPWO

- Objective: write unit tests covering `materialize_provider()` materialization behavior: auth-store refresh, `api_base` overrides, and missing-credentials error.
- Preconditions:
  - TASK-M7-03 complete: `materialize_provider()` implemented.
  - TASK-M7-05 complete: test module exists.
- Exact code target:
  - Extend `#[cfg(test)]` module in `src-rust/crates/query/src/provider_resolution.rs`.
- Strict constraints:
  - Do not modify production code.
  - Do not add network-dependent tests without `#[ignore]`.
  - Do not test `resolve_provider_identity()` here.
- Definition of done:
  - At least 2 materialize tests exist: happy path + error path.
  - Tests pass or are `#[ignore]`-gated with clear explanation.
- Validation command:
  - `cd src-rust && cargo test -p claurst-query -- provider_resolution`
- Stop / escalate condition:
  - If `ProviderRegistry` cannot be constructed in tests without real API keys, mark all materialize tests as `#[ignore]` and document the limitation.

## Verified Files

Minimal file set inspected for this preflight:

- `src-rust/crates/query/src/provider_resolution.rs`
- `src-rust/crates/query/src/lib.rs`
- `src-rust/crates/api/src/registry.rs`
- `src-rust/crates/api/src/lib.rs`
- `src-rust/crates/api/src/provider.rs`
- `src-rust/crates/api/src/providers/openai.rs`
- `src-rust/crates/api/src/providers/openai_compat.rs`
- `src-rust/crates/api/src/providers/openai_compat_providers.rs`
- `src-rust/crates/core/src/lib.rs`
- `src-rust/crates/core/src/auth_store.rs`
- `src-rust/crates/query/Cargo.toml`
- `src-rust/Cargo.toml`

## Verified Symbols

- `materialize_provider` exists at `src-rust/crates/query/src/provider_resolution.rs:157`
  - Signature:
    - `pub fn materialize_provider(identity: &ProviderIdentity, registry: &ProviderRegistry, provider_configs: &HashMap<String, ProviderConfig>) -> Result<ExecutionTarget, ProviderResolutionError>`
- `ProviderIdentity` exists at `src-rust/crates/query/src/provider_resolution.rs:58`
- `ExecutionTarget` exists at `src-rust/crates/query/src/provider_resolution.rs:65`
- `ProviderResolutionError::NoCredentials(String)` exists at `src-rust/crates/query/src/provider_resolution.rs:89-90`
- Existing `#[cfg(test)] mod tests` is present at `src-rust/crates/query/src/provider_resolution.rs:255`
- P1-P12 `resolve_provider_identity()` coverage is present in the existing test module at `src-rust/crates/query/src/provider_resolution.rs:335-471`
- `ProviderRegistry` lives at `src-rust/crates/api/src/registry.rs:21`
  - Public constructor: `ProviderRegistry::new()` at `src-rust/crates/api/src/registry.rs:84`
  - Public insert/register path: `ProviderRegistry::register(&mut self, Arc<dyn LlmProvider>)` at `src-rust/crates/api/src/registry.rs:92`
  - Default/empty construction path: `impl Default for ProviderRegistry { Self::new() }` at `src-rust/crates/api/src/registry.rs:389-392`
  - Additional public helper: `with_openai_if_key_set()` at `src-rust/crates/api/src/registry.rs:175`
- `ProviderRegistry` is publicly re-exported by `claurst_api` at `src-rust/crates/api/src/lib.rs:72`
- `OpenAiProvider` is publicly re-exported by `claurst_api` at `src-rust/crates/api/src/lib.rs:77`
- `OpenAiProvider::new(api_key: String)` exists at `src-rust/crates/api/src/providers/openai.rs:52`
- `ProviderConfig` lives in `claurst_core::config` at `src-rust/crates/core/src/lib.rs:669`
  - `api_base: Option<String>` at `src-rust/crates/core/src/lib.rs:673`
  - `Default` implementation at `src-rust/crates/core/src/lib.rs:688-698`
- `openai_compat_providers::ollama()` exists at `src-rust/crates/api/src/providers/openai_compat_providers.rs:19`
- `OpenAiCompatProvider::with_base_url(...)` exists at `src-rust/crates/api/src/providers/openai_compat.rs:129`
- `LlmProvider::id()` exists at `src-rust/crates/api/src/provider.rs:50`

## Preconditions Status

### TASK-M7-03 complete

Status: verified.

Evidence:

- `materialize_provider()` is implemented in `src-rust/crates/query/src/provider_resolution.rs:157-215`.
- Query crate wiring calls the seam from `src-rust/crates/query/src/lib.rs:860` and `:872`.

### TASK-M7-05 complete

Status: verified.

Evidence:

- Existing `#[cfg(test)] mod tests` starts at `src-rust/crates/query/src/provider_resolution.rs:255`.
- P1-P12 tests are present at `src-rust/crates/query/src/provider_resolution.rs:335-471`.
- Baseline coverage commit `d76e8fb731473b5abf09c05ce885a0c4721233b9` is present in branch history.

### Hosted Ollama compatibility baseline still present

Status: verified.

Evidence:

- Baseline commit `5f8dfe1edd3b0b2c3c064b463948080bcc5b188b` is present in branch history.
- `build_ollama_provider()` still applies `normalize_ollama_api_base(...)` to configured overrides at `src-rust/crates/query/src/provider_resolution.rs:218-228`.
- `build_ollama_provider()` still reads Ollama auth via `AuthStore::load().api_key_for(ProviderId::OLLAMA)` at `src-rust/crates/query/src/provider_resolution.rs:230-231`.
- Hosted/root normalization helper remains present at `src-rust/crates/query/src/provider_resolution.rs:237-252`.
- Hosted Ollama normalization tests remain present at `src-rust/crates/query/src/provider_resolution.rs:299-333`.
- `AuthStore::api_key_for("ollama")` still uses environment-first precedence at `src-rust/crates/core/src/auth_store.rs:137-147`, with explicit tests at `src-rust/crates/core/src/auth_store.rs:220-249`.

## Scope And Target Inspection

### `materialize_provider(...)` signature and location

- File: `src-rust/crates/query/src/provider_resolution.rs`
- Line: `157`
- Signature:
  - `pub fn materialize_provider(identity: &ProviderIdentity, registry: &ProviderRegistry, provider_configs: &HashMap<String, ProviderConfig>) -> Result<ExecutionTarget, ProviderResolutionError>`

### `ProviderRegistry` public API shape

- Location: `src-rust/crates/api/src/registry.rs`
- Public constructor: yes, `ProviderRegistry::new()` at `:84`
- Builder type: no separate builder type found
- Insert/add/register method: yes, `register(&mut self, Arc<dyn LlmProvider>) -> &mut Self` at `:92`
- Empty/default construction path:
  - `ProviderRegistry::new()` at `:84`
  - `ProviderRegistry::default()` via `impl Default` at `:389-392`

### Can a registry containing an OpenAI provider be built from public API?

Yes.

Evidence:

- `ProviderRegistry` is publicly re-exported from `claurst_api` at `src-rust/crates/api/src/lib.rs:72`.
- `OpenAiProvider` is publicly re-exported from `claurst_api` at `src-rust/crates/api/src/lib.rs:77`.
- `OpenAiProvider::new(api_key: String)` is public at `src-rust/crates/api/src/providers/openai.rs:52`.
- `ProviderRegistry::register(...)` accepts `Arc<dyn LlmProvider>` at `src-rust/crates/api/src/registry.rs:92`.

Real credentials required: no. Any string can be passed to `OpenAiProvider::new(...)`.

Network access required to construct: no. Construction only builds a `reqwest::Client`; no request is sent until provider methods like `create_message`, `list_models`, or `health_check` are called.

### Where provider config types live

- `ProviderConfig` lives at `src-rust/crates/core/src/lib.rs:669-686`
- Public access path: `claurst_core::config::ProviderConfig` and re-exported via `claurst_core` at `src-rust/crates/core/src/lib.rs:78`

### Can an Ollama config with `api_base: Some("http://custom:11434")` be built from public API?

Yes.

Evidence:

- `ProviderConfig.api_base: Option<String>` exists at `src-rust/crates/core/src/lib.rs:673`.
- `ProviderConfig` has a public `Default` implementation at `src-rust/crates/core/src/lib.rs:688-698`.
- `materialize_provider()` special-cases `ProviderId::OLLAMA` and delegates to `build_ollama_provider(provider_configs)` at `src-rust/crates/query/src/provider_resolution.rs:162-168`.
- `build_ollama_provider()` reads `provider_configs["ollama"].api_base` and applies `with_base_url(normalize_ollama_api_base(...))` at `src-rust/crates/query/src/provider_resolution.rs:223-227`.

### Is `ProviderResolutionError::NoCredentials(...)` directly assertable from tests?

Yes, via pattern matching.

Evidence:

- Variant exists at `src-rust/crates/query/src/provider_resolution.rs:89-90`.
- `materialize_provider()` constructs it directly at `src-rust/crates/query/src/provider_resolution.rs:208`.
- The enum does not derive `PartialEq`, so the practical assertion style is `matches!(..., ProviderResolutionError::NoCredentials(provider) if ...)`, not `assert_eq!`.

### Is the existing test module still the correct edit location?

Yes.

Evidence:

- Existing `#[cfg(test)] mod tests` remains in `src-rust/crates/query/src/provider_resolution.rs:255`.
- MPWO target is to extend this test module, and no structural mismatch was found.

### Exact files and symbols TASK-M7-06 would need to touch if executed

Expected source touch set if executed:

- `src-rust/crates/query/src/provider_resolution.rs`
  - existing `#[cfg(test)] mod tests`
  - test imports in that module
  - new test functions for:
    - registry provider found
    - missing credentials
    - Ollama `api_base` override

Expected additional symbols imported/used in that same file:

- `materialize_provider`
- `ProviderIdentity`
- `ProviderResolutionError`
- `ResolutionSource`
- `ProviderRegistry`
- `OpenAiProvider`
- `ProviderConfig`
- `HashMap`
- `Arc`

No production-code file changes appear necessary from the current public API surface.

## Per-Test Feasibility Assessment

### A. Registry provider found

Classification: `runnable as normal unit test`

Assessment:

- `ProviderIdentity { provider_id: "openai", ... }` is constructible from current public types in `provider_resolution.rs`.
- `ProviderRegistry` containing an OpenAI provider is constructible from public API via `ProviderRegistry::new()` + `register(Arc::new(OpenAiProvider::new(...)))`.
- Real API keys are not required to construct the provider.
- Network activity is not required unless the test explicitly calls provider methods that hit the network, which MPWO does not require.

Note:

- `materialize_provider()` consults `runtime_provider_for(&identity.provider_id)` before falling back to the passed registry (`src-rust/crates/query/src/provider_resolution.rs:173-179`, `src-rust/crates/api/src/registry.rs:73-80`).
- If ambient OpenAI credentials exist in env or `~/.claurst/auth.json`, the returned provider could come from runtime credentials rather than the injected registry. The MPWO assertion `Ok(target)` with `target.provider_id == "openai"` is still feasible and normal; a strict provenance assertion would require env/home isolation not specified by MPWO.

### B. No credentials error path

Classification: `runnable as normal unit test`

Assessment:

- Empty registry construction is available via `ProviderRegistry::new()` or `ProviderRegistry::default()`.
- `materialize_provider()` can be invoked with provider id `some-fake-provider` and empty `provider_configs`.
- `runtime_provider_for("some-fake-provider")` returns `None` under current code because no env-var mapping/provider factory exists for that provider id.
- The error path is directly assertable with `matches!(..., ProviderResolutionError::NoCredentials(provider) if provider == "some-fake-provider")`.

Real credentials required: no.

Network activity required: no.

### C. `api_base` override for Ollama

Classification: `runnable as normal unit test`

Assessment:

- `ProviderIdentity` with `provider_id: "ollama"` is constructible from current public types.
- `provider_configs["ollama"].api_base = Some("http://custom:11434")` is constructible from public API using `ProviderConfig`.
- `materialize_provider()` succeeds on the Ollama branch without network activity because it only constructs the provider object.
- No real credentials are required for success; `AuthStore::load().api_key_for("ollama")` is optional and only adds a key if present.
- Success-only assertion is realistic. Verifying the actual base URL through the returned `Arc<dyn LlmProvider>` is not realistic from current public API without inspecting internal concrete type state or adding new test-only seams.

Conclusion:

- None of the three required MPWO tests appear to require `#[ignore]` under current repo reality.

## Verified Commands

- Verified command from MPWO:
  - `cd src-rust && cargo test -p claurst-query -- provider_resolution`

Command plausibility against repo reality:

- `src-rust/` exists.
- `src-rust/Cargo.toml` is a workspace manifest.
- `src-rust/crates/query/Cargo.toml` declares package `name = "claurst-query"`.
- Cargo workspace metadata includes `claurst-query` as a workspace member.
- Query crate exports `pub mod provider_resolution;` from `src-rust/crates/query/src/lib.rs:19`, so the filter string `provider_resolution` is plausible for the unit tests in that module.

Validation execution status:

- Not run during this preflight, by instruction.

## Drift Found

- Drift classification: `none`

Rationale:

- TASK-M7-06 target file, preconditions, test module location, registry/config APIs, and validation command all match repo reality closely enough to execute without reinterpretation.
- No structural drift was found that would require stopping or escalating.

## Blockers

- None.

## Notes For Execution Phase

- Keep scope to `src-rust/crates/query/src/provider_resolution.rs` test module only.
- No production code change is indicated by current public APIs.
- `ProviderResolutionError::NoCredentials(...)` should be asserted with `matches!`, not `assert_eq!`.
- The OpenAI happy-path test can be a normal unit test, but it should follow MPWO's minimum assertion surface because ambient OpenAI credentials could satisfy the runtime-provider path before the injected registry path.
- The Ollama `api_base` override test can also be a normal unit test; success-only assertion is the realistic contract without peeking into concrete provider internals.
- Review basis should account for large unrelated untracked noise under `src-rust/target/` and existing untracked docs content.
