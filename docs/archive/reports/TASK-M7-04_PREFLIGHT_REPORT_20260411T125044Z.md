# TASK-M7-04 Preflight Report

- Ticket ID: `TASK-M7-04`
- Verdict: Conditional pass.
- Safe to execute as written: No, not safely reviewable as written against the current git state. Technically safe to implement once the review basis is made explicit so accepted prior-ticket content is treated as baseline and unrelated staged diffs are excluded from the M7-04 delta.

## Verified Files, Symbols, Commands

- Authority verified: `docs/Current/MPWO_WORK_ORDER_PACK.md`
- Target file verified: `src-rust/crates/query/src/lib.rs`
- Helper module verified: `src-rust/crates/query/src/provider_resolution.rs`
- Registry helper verified: `src-rust/crates/api/src/registry.rs`
- Config type verified: `src-rust/crates/core/src/lib.rs`
- Commands used for verification: `rg`, `nl -ba`, `git status --short`, `git diff --`, `git diff --cached`, `find`, `date`
- Verified live anchor: `if let Some(ref registry) = config.provider_registry {` in `run_query_loop()`
- Verified live anchor: inline resolution block still present in `run_query_loop()`
- Verified live anchor: anthropic filter `.filter(|p| *p != "anthropic")` still present
- Verified live anchor: `use_provider_dispatch` still present
- Verified live anchor: inline materialization block still present
- Verified live anchor: final provider selection `let provider = runtime_provider.or(registry_provider);` still present
- Verified live anchor: capability-shaping block remains inline
- Verified live anchor: no-registry Anthropic path remains separate and unchanged in structure
- Verified helper: `resolve_provider_identity()` exists with accepted M7-02 shape
- Verified helper: `materialize_provider()` exists in the live worktree with accepted M7-03 shape
- Verified helper: `runtime_provider_for()` exists
- Verified type: `tool_ctx.config.provider_configs` is `HashMap<String, ProviderConfig>`
- Verified local error-handling style: provider-path failures log and return `QueryOutcome::Error(ClaudeError::Api(...))`; raw client failures log and return `QueryOutcome::Error(e)`
- Verified downstream identifier usage count: `provider_id_str` and `model_id_str` are referenced at 19 later line sites in the current block

## Drift Found

- Line-number drift only:
- Work-order paths `crates/...` map to live repo paths `src-rust/crates/...`
- Registry block entry is live line 864 instead of the pack’s 862
- Anthropic filter is live line 865 instead of the pack’s 863
- `use_provider_dispatch` is live lines 933-934 instead of the pack’s 931-932
- Materialization block spans live lines 937-982 instead of the pack’s 934-980 / 937-977
- Capability shaping starts at live line 996 instead of the pack’s 994
- Structural drift in the M7-04 target block: none
- Repo-state drift affecting review basis:
- `src-rust/crates/query/src/lib.rs` already has staged changes
- `src-rust/crates/api/src/registry.rs` already has staged unrelated changes
- `src-rust/crates/query/src/provider_resolution.rs` is staged as the new module, but the M7-03 body is only present in the current unstaged worktree diff

## Blockers

- Review-basis blocker: the active patch is not scope-clean for M7-04
- Accepted prior-ticket content is not fully anchored as baseline in current git state
- Unrelated staged `registry.rs` changes would contaminate M7-04 review unless explicitly excluded
- The work order’s stop condition on downstream identifier sites is exceeded in the live file shape, so those updates must be handled deliberately within the same ticket rather than assumed trivial

## Conclusion

- `TASK-M7-04` is structurally ready in code terms
- `TASK-M7-04` is not ready to execute as a clean closure/review pass until the review basis is made explicit
