# 1. Ticket ID

`POST-M11-ARCH-01A — Session-scoped HealthCache ownership in claurst-query`

# 2. Timestamp UTC

`20260416T080436Z`

# 3. Verification verdict

`PASS-WITH-NOTES`

# 4. Branch / HEAD / worktree summary

- Branch: `feature/provider-resolution-seam`
- HEAD: `63595c387ac8fd2f5adbf9cf75d45a724153c3db`
- Worktree state: dirty outside this ticket
- Unrelated tracked noise is present in `.gitignore`, `docs/Current/TASK-M11-10_SPLIT_PLAN_AND_TEMPORARY_AUTHORITY_AMENDMENT.md`, `src-rust/crates/api/src/providers/google.rs`, and multiple `src-rust/crates/core/*` files.
- Unrelated untracked noise is present in many `docs/archive/reports/*` artifacts plus `src-rust/target/`.
- Ticket-local review basis is still explicit and narrow:
  - current code diff under `src-rust/crates/query/src/` is limited to the five approved query files
  - the execution artifact is the untracked file `docs/archive/reports/POST_M11_ARCH_01A_EXECUTION_REPORT_20260416T074841Z.md`
- No source edits were made in this verification pass.

# 5. Files verified

- `src-rust/crates/query/src/health_cache.rs`
- `src-rust/crates/query/src/lib.rs`
- `src-rust/crates/query/src/agent_tool.rs`
- `src-rust/crates/query/src/provider_resolution_tests.rs`
- `src-rust/crates/query/src/agent_tool_tests.rs`
- `src-rust/crates/query/src/provider_resolution.rs` — verified untouched
- `src-rust/crates/tools/src/lib.rs` — `ToolContext` shape inspected
- `src-rust/crates/tools/src/team_tool.rs` — verified untouched
- `src-rust/crates/query/src/session_budget.rs` — verified untouched
- `docs/archive/reports/POST_M11_ARCH_01A_EXECUTION_REPORT_20260416T074841Z.md`

# 6. Scope verification result

- `git diff --name-only -- src-rust/crates/query/src` returned exactly:
  - `src-rust/crates/query/src/agent_tool.rs`
  - `src-rust/crates/query/src/agent_tool_tests.rs`
  - `src-rust/crates/query/src/health_cache.rs`
  - `src-rust/crates/query/src/lib.rs`
  - `src-rust/crates/query/src/provider_resolution_tests.rs`
- That matches the approved in-scope source set exactly.
- `git diff --name-only -- src-rust/crates/tools/src/lib.rs src-rust/crates/tools/src/team_tool.rs src-rust/crates/query/src/session_budget.rs src-rust/crates/query/src/provider_resolution.rs` returned no paths.
- The execution artifact exists as `?? docs/archive/reports/POST_M11_ARCH_01A_EXECUTION_REPORT_20260416T074841Z.md`.
- Scope conclusion:
  - ticket-local source diff is limited to the approved query files
  - no forbidden source file was modified by this ticket
  - broader repo noise exists, but it is distinguishable from this ticket’s edit set

# 7. Structural verification result

- A query-owned session registry/helper exists inside `claurst-query`:
  - `SESSION_HEALTH_CACHE_REGISTRY` in `src-rust/crates/query/src/health_cache.rs:15`
  - `register_session_health_cache(...)` at `src-rust/crates/query/src/health_cache.rs:100`
  - `session_health_cache_for_session(...)` at `src-rust/crates/query/src/health_cache.rs:119`
  - `session_health_cache_or_new(...)` at `src-rust/crates/query/src/health_cache.rs:125`
  - `with_registered_session_health_cache(...)` at `src-rust/crates/query/src/health_cache.rs:129`
- Ownership is keyed by `session_id`:
  - registry keying uses `session_id.to_string()` in `health_cache.rs:104` and lookup by `session_id` in `health_cache.rs:119`
- Registration/cleanup is tied to query-session lifetime:
  - `run_query_loop()` now wraps the query loop with `with_registered_session_health_cache(&tool_ctx.session_id, ...)` in `src-rust/crates/query/src/lib.rs:878`
  - cleanup is driven by `SessionHealthCacheRegistration::drop()` in `src-rust/crates/query/src/health_cache.rs:83`
- Live child/team fallback sites no longer construct fresh `HealthCache::new()` for the registered path:
  - `AgentTool` uses `inherited_session_health_cache(&ctx.session_id)` in `src-rust/crates/query/src/agent_tool.rs:144` and at the child fallback site in `agent_tool.rs:410`
  - the team runner uses the same helper in `src-rust/crates/query/src/agent_tool.rs:765`
- Fallback to a fresh local cache still exists outside an active registered query scope:
  - `session_health_cache_or_new()` falls back to `Arc::new(HealthCache::new())` in `src-rust/crates/query/src/health_cache.rs:126`
- `ToolContext` shape is unchanged:
  - `src-rust/crates/tools/src/lib.rs:216-235` contains no `health_cache` field
- No process-global unconditional cache behavior was introduced:
  - the registry is static storage, but visibility is gated behind explicit session registration and cleaned on last-guard drop

# 8. Lifetime / isolation verification result

- Registration visibility is covered:
  - `health_cache_registration_exposes_cache_for_session()` in `src-rust/crates/query/src/health_cache.rs:296`
- Cleanup on last guard drop is covered:
  - `health_cache_registration_releases_when_last_guard_drops()` in `src-rust/crates/query/src/health_cache.rs:307`
- Nested registration preserves the first active owner for the same session id:
  - `health_cache_nested_registration_preserves_initial_owner()` in `src-rust/crates/query/src/health_cache.rs:319`
  - implementation preserves the initial cache by incrementing only the registration count for occupied entries in `src-rust/crates/query/src/health_cache.rs:104-112`
- Registration helper scope cleanup is covered:
  - `with_registered_session_health_cache_registers_and_cleans_up()` in `src-rust/crates/query/src/health_cache.rs:338`
- Same-session reuse is covered:
  - `fallback_same_session_reuses_registered_health_cache()` in `src-rust/crates/query/src/provider_resolution_tests.rs:739`
  - verifies sequential same-session reuse causes one health probe only
- Session isolation is covered:
  - `fallback_session_scopes_do_not_share_cached_health()` in `src-rust/crates/query/src/provider_resolution_tests.rs:786`
  - verifies different session IDs trigger separate health probes
- Child/team live-seam reuse is covered:
  - `child_and_team_fallback_share_session_health_cache()` in `src-rust/crates/query/src/agent_tool_tests.rs:488`
  - verifies child agent and team runner share one session cache and no leak remains after the scope ends
- Isolation conclusion:
  - no cross-session leak is evident in code or tests

# 9. Behavioral non-regression result

- `src-rust/crates/query/src/provider_resolution.rs` is untouched by this ticket.
- Trust-domain filtering is unchanged in `resolve_provider_with_fallback()`:
  - `TrustDomain::for_provider(...)` filter remains at `src-rust/crates/query/src/provider_resolution.rs:328-339`
- Health-aware candidate ordering is unchanged:
  - healthy candidates are still accumulated before degraded candidates at `provider_resolution.rs:329-360`
- Capability filtering is unchanged:
  - fallback model selection still routes through `select_fallback_model(...)` and `DEFAULT_REQUIRED_CAPABILITIES` in `provider_resolution.rs:266-303`
- `allow_fallback` gating/defaults are unchanged:
  - `resolve_provider_with_fallback()` still short-circuits with `FallbackDisabled` when false at `provider_resolution.rs:322-326`
  - `AgentTool` still defaults `allow_fallback` with `unwrap_or(false)` at `src-rust/crates/query/src/agent_tool.rs:409`
- Provider/model conflict behavior is unchanged:
  - `resolve_provider_identity(...)` code path is untouched
- Hosted-Ollama/provider-materialization locality is unchanged:
  - `materialize_provider(...)`, `build_ollama_provider(...)`, and auth/base-URL logic remain untouched in `provider_resolution.rs`
- Root registry-backed dispatch semantics are unchanged:
  - only query-loop session registration wrapping changed in `lib.rs`; no root provider-resolution redesign occurred
- `SessionBudget` ownership design is unchanged:
  - `src-rust/crates/query/src/session_budget.rs` is untouched
  - `run_query_loop()` still uses the existing `with_registered_session_budget(...)` seam inside the new health-cache wrapper
- Fresh full-crate tests passed, which supports the non-regression conclusion.

# 10. Out-of-scope verification result

- Verified not added:
  - `health_cache` was not added to `ToolContext`
- Verified not modified:
  - `src-rust/crates/tools/src/lib.rs`
  - `src-rust/crates/tools/src/team_tool.rs`
  - `src-rust/crates/query/src/session_budget.rs`
  - `src-rust/crates/query/src/provider_resolution.rs`
- Verified not introduced:
  - in-flight probe coalescing
  - duplicate-probe suppression claims
  - ARCH-02 / ARCH-03 work
  - TeamCreate cancellation redesign
  - neutral/shared-crate relocation
  - process-global cache baseline semantics
- Verified not reopened:
  - M11 runtime behavior
  - M12 authority/audit work

# 11. Test verification result

- Re-ran required filters:
  - `cargo test -p claurst-query health_cache`
    - passed
    - 12 passed, 0 failed
  - `cargo test -p claurst-query provider_resolution`
    - passed
    - 31 passed, 0 failed
  - `cargo test -p claurst-query agent_tool`
    - passed
    - 10 passed, 0 failed
- Also ran:
  - `cargo test -p claurst-query`
    - passed
    - 145 passed, 0 failed
- Note:
  - cargo briefly reported package-cache/artifact lock waits before tests started, but all runs completed successfully without intervention

# 12. Prompt-safety / ambiguity check

- No material prompt-safety failure found.
- The implementation could be glanced at and mistaken for a process-global cache because `SESSION_HEALTH_CACHE_REGISTRY` is a static, but the surrounding API makes the intended lifetime sufficiently clear:
  - registration is explicit via `with_registered_session_health_cache(...)`
  - lookup is session-keyed
  - fallback to fresh local cache is explicit in `session_health_cache_or_new(...)`
  - cleanup is explicit in the registration guard `Drop`
  - tests assert registration visibility, cleanup, nested-owner preservation, same-session reuse, and cross-session isolation
- The code does not read as an endorsement of `ToolContext` carriage:
  - `ToolContext` is unchanged
  - the retrieval seam is query-owned, not tool-context-owned
- The code does not read as a full duplicate-probe suppression solution:
  - no in-flight map/promise/join/coalescer exists
  - tests assert sequential reuse and isolation only, not concurrent cold-miss suppression

# 13. Any failures or concerns

- No concrete verification failure was found.
- Concern:
  - the overall repo worktree is not scope-clean because unrelated tracked and untracked changes remain present
  - closeout should therefore use an explicit ticket-local review basis rather than the full raw worktree
- Operational note:
  - the execution artifact is still untracked, so any later commit/closeout step should deliberately include the correct report artifacts instead of relying on a blanket add

# 14. Final recommendation on whether the ticket is ready for conditional commit + closeout

- Recommendation: `YES, CONDITIONALLY`
- This ticket is ready for conditional commit + closeout if the review/commit basis is explicitly limited to:
  - `src-rust/crates/query/src/health_cache.rs`
  - `src-rust/crates/query/src/lib.rs`
  - `src-rust/crates/query/src/agent_tool.rs`
  - `src-rust/crates/query/src/provider_resolution_tests.rs`
  - `src-rust/crates/query/src/agent_tool_tests.rs`
  - `docs/archive/reports/POST_M11_ARCH_01A_EXECUTION_REPORT_20260416T074841Z.md`
  - `docs/archive/reports/POST_M11_ARCH_01A_VERIFICATION_REPORT_20260416T080436Z.md`
- No corrective patch is warranted from this verification pass.
