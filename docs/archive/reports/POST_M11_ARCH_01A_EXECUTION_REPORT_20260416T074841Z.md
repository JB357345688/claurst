# 1. Ticket ID

`POST-M11-ARCH-01A — Session-scoped HealthCache ownership in claurst-query`

# 2. Timestamp UTC

`20260416T074841Z`

# 3. Files changed

- `src-rust/crates/query/src/health_cache.rs`
- `src-rust/crates/query/src/lib.rs`
- `src-rust/crates/query/src/agent_tool.rs`
- `src-rust/crates/query/src/provider_resolution_tests.rs`
- `src-rust/crates/query/src/agent_tool_tests.rs`
- `docs/archive/reports/POST_M11_ARCH_01A_EXECUTION_REPORT_20260416T074841Z.md`

# 4. Change summary

- Added a query-owned session health-cache registry in `claurst-query`, keyed by `session_id`, with refcounted registration/cleanup guards.
- Registered a session health cache for every `run_query_loop()` lifetime, without changing query-loop call signatures or `ToolContext`.
- Replaced the two live child/team fallback-resolution `HealthCache::new()` sites in `agent_tool.rs` with session-cache retrieval by `ctx.session_id`.
- Added targeted tests for registration/cleanup, sequential same-session reuse, session isolation, and child/team live-seam reuse.

# 5. Session-scoped HealthCache design implemented

- `health_cache.rs` now owns the session seam:
  - `register_session_health_cache(session_id, &Arc<HealthCache>)`
  - `session_health_cache_for_session(session_id)`
  - `session_health_cache_or_new(session_id)`
  - `with_registered_session_health_cache(session_id, future)`
- The registry is crate-local to `claurst-query` and keyed by `session_id`.
- Nested registrations preserve the first active cache owner for that session key and only adjust a registration count.
- `run_query_loop()` now wraps the existing budget registration seam inside `with_registered_session_health_cache(&tool_ctx.session_id, ...)`.

# 6. Exact ownership/lifetime behavior now in repo

- A fresh `Arc<HealthCache>` is created and registered when a query loop starts for a given `session_id`.
- That cache is visible to live fallback-resolution sites through `session_health_cache_for_session(session_id)` for the duration of the registered query scope.
- If nested query loops or sibling child/team runs reuse the same `session_id`, they reuse the same registered `HealthCache` owner for that session instead of replacing it.
- When the last registration guard for that `session_id` drops, the registry entry is removed.
- The live fallback seams in `agent_tool.rs` and the team runner now call `resolve_provider_with_fallback(..., health_cache.as_ref(), ...)` using the session-owned cache when present.
- Outside an active registered query session, those seams fall back to a fresh unregistered cache rather than widening `ToolContext` or changing signatures.

# 7. Tests added/updated

- `health_cache.rs`
  - added registration visibility test
  - added registration cleanup test
  - added nested registration owner-preservation test
  - added scoped registration helper cleanup test
- `provider_resolution_tests.rs`
  - added sequential same-session reuse test proving cached provider health is reused within TTL
  - added session-isolation test proving different session IDs do not share cached health
- `agent_tool_tests.rs`
  - added live child/team reuse test proving agent-tool fallback and team-runner fallback share the same session cache and avoid a second health probe
  - updated the tracking test provider to count health checks in addition to message invocations

# 8. Validation performed and results

- `cargo test -p claurst-query health_cache`
  - passed
  - 12 passed, 0 failed
- `cargo test -p claurst-query provider_resolution`
  - passed
  - 31 passed, 0 failed
- `cargo test -p claurst-query agent_tool`
  - passed
  - 10 passed, 0 failed
- `cargo test -p claurst-query`
  - passed
  - 145 passed, 0 failed
- Forbidden-file check:
  - `git diff --name-only -- src-rust/crates/tools/src/lib.rs src-rust/crates/tools/src/team_tool.rs src-rust/crates/query/src/session_budget.rs`
  - returned no paths
- Repo-state note:
  - `docs/Current/TASK-M11-10_SPLIT_PLAN_AND_TEMPORARY_AUTHORITY_AMENDMENT.md` was already shown as deleted in repo status before this execution pass and was not modified by this ticket

# 9. Diff summary

`git diff --stat -- src-rust/crates/query/src/health_cache.rs src-rust/crates/query/src/lib.rs src-rust/crates/query/src/agent_tool.rs src-rust/crates/query/src/provider_resolution_tests.rs src-rust/crates/query/src/agent_tool_tests.rs`

- `src-rust/crates/query/src/agent_tool.rs` | 17 ++-
- `src-rust/crates/query/src/agent_tool_tests.rs` | 131 ++++++++++++++++++++-
- `src-rust/crates/query/src/health_cache.rs` | 125 +++++++++++++++++++-
- `src-rust/crates/query/src/lib.rs` | 27 +++--
- `src-rust/crates/query/src/provider_resolution_tests.rs` | 108 ++++++++++++++++-
- Total: 5 files changed, 383 insertions(+), 25 deletions(-)

# 10. Explicit out-of-scope confirmations

- No `ToolContext.health_cache` field was introduced.
- `HealthCache` was not moved out of `claurst-query`.
- No neutral/shared-crate relocation was done.
- No process-global always-on cache behavior was introduced; visibility is session-keyed and registration-scoped.
- `resolve_provider_with_fallback()` selection semantics were not changed.
- Trust-domain filtering, health-aware candidate ordering, capability filtering, `allow_fallback` gating, and provider/model conflict behavior remain unchanged.
- Hosted-Ollama/provider-materialization locality remains unchanged; the cache still stores only provider health status.
- Root registry-backed dispatch was not redesigned.
- `provider_resolution.rs` was not touched.
- `src-rust/crates/tools/src/lib.rs`, `src-rust/crates/tools/src/team_tool.rs`, and `src-rust/crates/query/src/session_budget.rs` were not modified.
- No TeamCreate cancellation redesign, ARCH-02/ARCH-03 work, or in-flight duplicate-probe coalescing was added.

# 11. Risks / notes

- The new registry is static crate-local state, but entries exist only while query-session registration guards are alive; it is not a single shared cache for all sessions.
- Concurrent cold misses within the same session can still duplicate health probes because in-flight coalescing remains explicitly out of scope for this ticket.
- Direct `AgentTool`/team-runner execution outside a registered query-loop session uses a fresh local cache, preserving narrow behavior without widening ownership into `ToolContext`.

# 12. Final execution verdict

- `PASS`
- The repo now has real session-scoped `HealthCache` reuse for child/team fallback paths inside `claurst-query`.
- No forbidden source files were modified for this ticket.
- Validation passed.
- Ready for review on the active unstaged diff for this ticket.
