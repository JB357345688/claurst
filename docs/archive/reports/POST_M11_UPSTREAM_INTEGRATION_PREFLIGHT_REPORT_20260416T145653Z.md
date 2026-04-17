# 1. Ticket ID

`POST-M11-UPSTREAM-INTEGRATION-PREFLIGHT`

# 2. Timestamp UTC

`20260416T145653Z`

# 3. Scope / mode confirmation

- Mode: planning-only preflight for fresh-upstream integration.
- This pass did not patch source, cherry-pick, rebase, create the integration branch, clean the worktree, stash, or commit.
- The only artifact created in this pass is this report under `docs/archive/reports/`.
- Preflight verdict: `READY-FOR-LATER-FRESH-UPSTREAM-INTEGRATION-WITH-NOTES`.

# 4. Authority reviewed

Controlling authority re-read:

- `AGENTS.md`
- `docs/Current/MPWO_WORK_ORDER_PACK.md`

Accepted evidence requested by the prompt and verified present locally:

- `docs/archive/reports/POST_M11_MILESTONE_ACCEPTANCE_REPLAY_PLAN_REPORT_20260416T100911Z.md`
- `docs/archive/reports/POST_M11_MILESTONE_ACCEPTANCE_REPLAY_PASS_A_REPORT_20260416T110927Z.md`
- `docs/archive/reports/POST_M11_MILESTONE_ACCEPTANCE_REPLAY_PASS_B_REPORT_20260416T135621Z.md`
- `docs/archive/reports/M12_RECUT_PREFLIGHT_REPORT_20260416T064259Z.md`
- `docs/archive/reports/M12_CLOSEOUT_REPORT_20260416T065308Z.md`
- `docs/archive/reports/POST_M11_ARCH_01A_CLOSEOUT_REPORT_20260416T081839Z.md`
- `docs/archive/reports/POST_M11_ARCH_02_PREFLIGHT_REPORT_20260416T084418Z.md`
- `docs/archive/reports/POST_M11_ARCH_03_PREFLIGHT_REPORT_20260416T091621Z.md`

Verified files / symbols / inspections used in this preflight:

- Repo state and lineage:
  - `git branch --show-current`
  - `git rev-parse HEAD`
  - `git status --short --branch`
  - `git remote -v`
  - `git branch -vv`
  - `git fetch origin --prune`
  - `git fetch upstream --prune`
  - `git ls-remote --symref upstream HEAD`
  - `git remote show upstream`
  - `git log --oneline --decorate --graph --max-count=100 --all`
  - `git merge-base HEAD upstream/main`
  - `git merge-base HEAD origin/rfc/provider-aware-worker-fabric`
  - `git rev-list --left-right --count upstream/main...HEAD`
  - `git rev-list --left-right --count origin/rfc/provider-aware-worker-fabric...HEAD`
  - `git rev-list --left-right --count upstream/main...origin/rfc/provider-aware-worker-fabric`
- Commit / diff inspection:
  - `git diff --stat upstream/main...HEAD`
  - `git diff --name-status upstream/main...HEAD`
  - `git log --reverse --oneline $(git merge-base HEAD upstream/main)..HEAD`
  - `git log --reverse --oneline $(git merge-base HEAD upstream/main)..upstream/main`
  - `git log --reverse --oneline f8541bc12e057f063230a55dfd885b741327b141..HEAD`
  - focused `git show --stat`, `git show --unified=0`, `git log --reverse --name-only`, `git grep`, and `sed` probes
- Live source/test seams inspected:
  - `src-rust/crates/query/src/agent_tool.rs`
  - `src-rust/crates/query/src/provider_resolution.rs`
  - `src-rust/crates/query/src/session_budget.rs`
  - `src-rust/crates/query/src/health_cache.rs`
  - `src-rust/crates/query/src/lib.rs`
  - `src-rust/crates/tools/src/team_tool.rs`
  - `src-rust/crates/tools/src/lib.rs`
  - `src-rust/crates/cli/src/main.rs`
  - `src-rust/crates/api/src/provider_types.rs`
  - `src-rust/crates/api/src/model_registry.rs`
  - `src-rust/crates/core/src/lib.rs`
  - `src-rust/crates/tui/src/app.rs`

Drift found:

- No structural drift in the live authority path.
- The worktree is intentionally noisy and not review-clean.
- `origin/rfc/provider-aware-worker-fabric` is real and useful for lineage only; it is not the fresh integration base.

Blockers:

- None for planning.
- Later execution should not use the current dirty worktree as the review basis for the integration pass.

# 5. Current branch / HEAD / worktree summary

- Current branch: `feature/provider-resolution-seam`
- Current `HEAD`: `038f3c20e01a96eec6397d506b477a461166f762`
- `HEAD` subject: `feat(query): add session-scoped HealthCache reuse`
- `HEAD` matches the accepted post-M11 `ARCH-01A` closeout commit named in the prompt: `yes`
- Local `main`: `acae92611815439ec7981293810da0b90059e70f`
- `main` tracks `origin/main` and is not the planned fresh integration base

Current dirty worktree summary from `git status --short --branch`:

- Tracked modified:
  - `.gitignore`
  - `src-rust/crates/api/src/providers/google.rs`
  - `src-rust/crates/core/src/effort.rs`
  - `src-rust/crates/core/src/lib.rs`
  - `src-rust/crates/core/src/remote_settings.rs`
  - `src-rust/crates/core/src/system_prompt.rs`
- Tracked deleted:
  - `docs/Current/TASK-M11-10_SPLIT_PLAN_AND_TEMPORARY_AUTHORITY_AMENDMENT.md`
- Large untracked noise, including:
  - `.codex`
  - `src-rust/.codex`
  - `src-rust/target/`
  - `docs/archive/reports/*`
  - `docs/archive/provider_orchestrator/`
  - `docs/Orchestrator_planning/`

Planning implication:

- The committed feature lineage is still inspectable and coherent.
- The current worktree state is not a safe basis for direct in-place integration work.

# 6. Remotes and refs inspected

- `origin`
  - fetch: `git@github.com:JB357345688/claurst.git`
  - push: `git@github.com:JB357345688/claurst.git`
- `upstream`
  - fetch: `https://github.com/Kuberwastaken/claurst.git`
  - push: `https://github.com/Kuberwastaken/claurst.git`

Read-only remote confirmation:

- `git fetch origin --prune`: completed
- `git fetch upstream --prune`: completed
- `git ls-remote --symref upstream HEAD` confirmed:
  - `HEAD -> refs/heads/main`
  - remote `HEAD` commit `8359d9ccfe56708729f2696fb1e060da85affe4d`
- `git remote show upstream` confirmed:
  - remote `HEAD branch: main`
  - tracked branch: `main`

Candidate fresh base used for this plan:

- Upstream ref: `upstream/main`
- Upstream commit: `8359d9ccfe56708729f2696fb1e060da85affe4d`

Local alias relevance:

- `local main` and `origin/main` are relevant only as historical/common-ancestor context.
- They are irrelevant as fresh integration bases for this pass.
- `origin/rfc/provider-aware-worker-fabric` is lineage context only and is not a candidate base.

# 7. Branch-lineage context

- `origin/rfc/provider-aware-worker-fabric` currently points to `f8541bc12e057f063230a55dfd885b741327b141`.
- `HEAD` is directly descended from that RFC lineage branch.
- The RFC lineage branch itself is only two commits ahead of the old common ancestor:
  - `968c44721d704eb6c3cca8e205ac8256a666daf0` `docs: add RFC for Provider-Aware Worker Fabric`
  - `f8541bc12e057f063230a55dfd885b741327b141` `docs: add v2 RFC for Provider-Aware Worker Fabric`
- The first feature-lane runtime commit after the RFC lineage is:
  - `a09b3daefe887f2794c9fc2154afd8ebc8b3ec27` `Establish provider resolution seam baseline`

Lineage conclusion:

- The RFC branch is a historical fork point plus docs context.
- The real feature lane begins after that RFC branch and must be replayed selectively onto `upstream/main`.

# 8. Merge-base and divergence summary

Verified merge-bases:

| Pair | Merge-base |
| --- | --- |
| `HEAD` vs `upstream/main` | `acae92611815439ec7981293810da0b90059e70f` |
| `HEAD` vs `origin/rfc/provider-aware-worker-fabric` | `f8541bc12e057f063230a55dfd885b741327b141` |
| `upstream/main` vs `origin/rfc/provider-aware-worker-fabric` | `acae92611815439ec7981293810da0b90059e70f` |

Verified divergence counts:

| Comparison | Left-only | Right-only | Meaning |
| --- | ---: | ---: | --- |
| `upstream/main...HEAD` | `48` | `63` | `upstream/main` has moved far past the old base; the feature branch also contains a long local stack |
| `origin/rfc/provider-aware-worker-fabric...HEAD` | `0` | `61` | the current branch is simply ahead of the RFC lineage |
| `upstream/main...origin/rfc/provider-aware-worker-fabric` | `48` | `2` | the RFC branch has only the two docs lineage commits beyond the old common ancestor |

Planning implication:

- Whole-branch rebase or merge would combine `48` upstream-side commits with `63` feature-side commits and would drag non-feature docs/cleanup/noise across the boundary.

# 9. Accepted baseline and evidence chain re-read

All evidence files named in the prompt were available locally. No requested evidence had to be treated as unavailable.

Re-read evidence conclusions:

- `POST_M11_MILESTONE_ACCEPTANCE_REPLAY_PLAN_REPORT_20260416T100911Z.md`
  - confirms `HEAD=038f3c2`
  - treats the branch as replay-ready with explicit dirty-worktree caveats
- `POST_M11_MILESTONE_ACCEPTANCE_REPLAY_PASS_A_REPORT_20260416T110927Z.md`
  - targeted seam replay passed on the accepted branch
  - confirms provider-resolution, agent-tool, health-cache, session-budget, trust-domain, and cost-tracker seam confidence
- `POST_M11_MILESTONE_ACCEPTANCE_REPLAY_PASS_B_REPORT_20260416T135621Z.md`
  - broad build/test replay passed on the accepted branch
  - `cargo fmt --all -- --check` passed
  - `cargo clippy --workspace --all-targets -- -D warnings` failed outside the provider lane and was explicitly treated as a historical-gate issue, not a lane regression
  - conditional OpenAI smoke passed when prerequisites were present
- `M12_RECUT_PREFLIGHT_REPORT_20260416T064259Z.md`
  - `M12` is satisfied by audit
  - no live implementation delta identified
- `M12_CLOSEOUT_REPORT_20260416T065308Z.md`
  - `M12` closed as audit-only / no implementation delta
- `POST_M11_ARCH_01A_CLOSEOUT_REPORT_20260416T081839Z.md`
  - accepted post-M11 code delta after `M12` is `038f3c2`
  - scope is query-owned session-scoped `HealthCache` reuse only
- `POST_M11_ARCH_02_PREFLIGHT_REPORT_20260416T084418Z.md`
  - `POST-M11-ARCH-02` remains deferred
- `POST_M11_ARCH_03_PREFLIGHT_REPORT_20260416T091621Z.md`
  - `POST-M11-ARCH-03` remains deferred

Accepted baseline conclusions for fresh integration planning:

- Accepted runtime baseline to carry: `b157924e130fdf71c09a3787b47dd5eb1f31d542`
- Accepted record-only post-baseline doc/audit milestone: `63595c387ac8fd2f5adbf9cf75d45a724153c3db`
- Accepted post-baseline runtime extension to carry: `038f3c20e01a96eec6397d506b477a461166f762`
- Required preserved dispositions:
  - `M12` remains audit-only / no implementation delta
  - `POST-M11-ARCH-02` remains deferred
  - `POST-M11-ARCH-03` remains deferred

# 10. Candidate carry-forward commits table

These are the accepted feature-lane commits whose behavior should survive the fresh-upstream integration. The recommended transport is grouped manual reconstruction against `upstream/main`, not literal whole-stack cherry-pick.

Carry-forward docs/authority commits required for integration correctness: `none`.

| Commit | Bucket | Primary surfaces | Carry forward? | Notes |
| --- | --- | --- | --- | --- |
| `a09b3da` | D1 foundation | `query/lib.rs`, `query/provider_resolution.rs` | `yes` | establish provider-resolution seam baseline |
| `5881983` | D1 foundation | `query/lib.rs` | `yes` | wire root query loop through the seam |
| `5f8dfe1` | D1 foundation | `api/error_handling.rs`, `core/auth_store.rs`, `query/provider_resolution.rs` | `yes` | hosted-Ollama / auth materialization compatibility |
| `d76e8fb` | D1 validation | `query/provider_resolution.rs` | `yes` | `P1-P12` provider-identity coverage |
| `73e9104` | D1 validation | `query/provider_resolution.rs` | `yes` | materialize-provider coverage |
| `3f9b783` | M8 carrier seam | `tools/lib.rs`, `cli/main.rs` | `yes` | add provider/model registries to `ToolContext` |
| `fe060af` | M8 carrier seam | `cli/main.rs` | `yes` | wire root registries into startup |
| `f4dc962` | M8 carrier seam | `query/agent_tool.rs` | `yes` | add provider field to AgentTool input |
| `b5249a3` | M8 carrier seam | `query/agent_tool.rs` | `yes` | foreground shared provider seam |
| `5d246b2` | M8 carrier seam | `query/agent_tool.rs` | `yes` | background shared provider seam |
| `ced6005` | M8 carrier seam | `query/compact.rs`, `query/lib.rs` | `yes` | provider-aware compaction hardening |
| `ea9da37` | M8 carrier seam | `tools/team_tool.rs` | `yes` | replace positional runner seam with `AgentRunParams` |
| `1d531da` | M8 carrier seam | `query/agent_tool.rs` | `yes` | injected Team runner through shared provider seam |
| `5d472cf` | M8 carrier seam | `tools/team_tool.rs` | `yes` | add provider/model fields to team spec |
| `2fd7732` | M8 carrier seam | `tools/team_tool.rs` | `yes` | wire team spec provider/model overrides |
| `1056eb3` | M8 validation | `query/agent_tool.rs` | `yes` | provider seam coverage on agent tool path |
| `662b29a` | M9 validation | `query/provider_resolution.rs` | `yes` | expanded materialize-provider coverage |
| `c28ef22` | M9 validation | `query/agent_tool.rs` | `yes` | explicit provider routes to OpenAI |
| `2f1f169` | M9 validation | `query/agent_tool.rs` | `yes` | child inherits parent provider |
| `dfc4be4` | M9 validation | `query/lib.rs` | `yes` | missing registry keeps legacy Anthropics path only in root case |
| `63a8485` | M9 validation | `query/lib.rs` | `yes` | registry-present resolution failure does not fall back to legacy |
| `5e77652` | M9 validation | `query/agent_tool.rs` | `yes` | TeamCreate mixed-provider dispatch coverage |
| `af97a87` | M9 runtime | `query/agent_tool.rs`, `query/cron_scheduler.rs`, `query/lib.rs` | `yes` | remove hardcoded Anthropic child path |
| `6b362a0` | M9 closeout/runtime | `cli/main.rs`, `query/agent_tool.rs`, `query/lib.rs`, `query/provider_resolution.rs` | `yes` | includes D1-safe child max-token behavior and provider-auth test lock consolidation |
| `dc772aa` | D2 runtime | `api/provider_types.rs` | `yes` | add `TrustDomain` |
| `fe21969` | D2 runtime | `api/model_registry.rs` | `yes` | add capability fields to `ModelEntry` |
| `828b08e` | D2 runtime | `query/provider_resolution.rs` | `yes` | capability matching helpers |
| `eb26773` | D2 runtime | `query/health_cache.rs`, `query/lib.rs` | `yes` | add `HealthCache` |
| `1472024` | D2 runtime | `query/provider_resolution.rs` | `yes` | same-domain provider fallback |
| `cf8201f` | D2 runtime | `core/lib.rs` | `yes` | `CostTracker` attribution fields |
| `0942e4a` | D2 runtime | `query/session_budget.rs`, `query/lib.rs` | `yes` | base `SessionBudget` utility |
| `25518ca` | D2 runtime | `cli/main.rs`, `query/agent_tool.rs`, `query/lib.rs` | `yes` | root session-budget wiring |
| `b4ad28e` | D2 runtime | `query/agent_tool.rs`, `query/lib.rs`, `query/session_budget.rs` | `yes` | propagate budget into child query loops |
| `4ef9547` | D2 runtime | `query/agent_tool.rs`, `tools/team_tool.rs` | `yes` | child max-token override wiring |
| `ea046c5` | D2 runtime | `query/agent_tool.rs`, `tools/team_tool.rs` | `yes` | child same-domain fallback wiring |
| `3812df0` | D2 runtime | `query/lib.rs`, `query/session_budget.rs` | `yes` | layered child-budget seam |
| `bfabfd5` | D2 runtime | `query/agent_tool.rs`, `tools/team_tool.rs` | `yes` | child budget carriage |
| `0c9dac4` | D2 runtime | `query/agent_tool.rs`, `query/lib.rs`, `query/session_budget.rs`, `tui/app.rs` | `yes` | worker/budget observability events |
| `b157924` | D2 validation | `api/provider_types.rs`, `query/agent_tool.rs`, `query/lib.rs` | `yes` | accepted split-path D2 validation coverage |
| `038f3c2` | Post-M11 runtime | `query/health_cache.rs`, `query/lib.rs`, `query/agent_tool.rs`, provider/agent tests | `yes` | accepted `ARCH-01A` session-scoped `HealthCache` reuse |

# 11. Candidate do-not-carry commits / file surfaces table

These commits and surfaces should remain excluded from the fresh integration unless later live evidence proves one specific skipped behavior is actually required.

| Commit or surface | Type | Do not carry? | Why excluded |
| --- | --- | --- | --- |
| `968c447`, `f8541bc` | RFC lineage docs | `yes` | lineage context only, no runtime feature content |
| `255e3c7` | docs cleanup | `yes` | obsolete RFC cleanup, not integration correctness |
| `8657678` | `.gitignore` | `yes` | local ignore noise, explicitly listed as exclusion hypothesis |
| `780cb72` | workspace rustfmt baseline | `yes` | broad formatting sweep, unrelated to provider lane |
| `fc6d5a9`, `f8eb130`, `7f57749` | unrelated test alignment | `yes` | not part of accepted provider-resolution lane |
| `b8cc827` | clippy refactor | `yes` | pure cleanup in `run_query_loop`; final replay should simply keep code clean |
| `b5b6dd4` | formatting reconciliation | `yes` | pure formatting / wrapping changes only |
| `560b54f` | docs authority consolidation | `yes` | record-only authority state, not runtime integration content |
| `7fef4a3` | formatting cleanup | `yes` | non-functional normalization in fallback/cache modules |
| `0f66f7f`, `d802b37`, `7c979f5` | core lint cleanup | `yes` | outside accepted provider lane scope |
| `4a9a97f`, `20c3c27`, `8b20182` | query lint cleanup | `yes` | non-functional cleanup only |
| `03a0357`, `d07600e`, `f841967` | API lint / transformer cleanup | `yes` | unrelated to fresh integration target; increases conflict surface |
| `2def737` | query test-module reorg | `yes` | organizational only; do not force old test-file layout onto fresh upstream |
| `63595c3` | `M12` doc/audit closeout | `yes` | accepted record-only milestone; `M12` stays audit-only / no code delta |
| `POST-M11-ARCH-02` | deferred architecture | `yes` | must remain deferred |
| `POST-M11-ARCH-03` | deferred architecture | `yes` | must remain deferred |
| Current dirty `.gitignore` worktree state | worktree noise | `yes` | not accepted carry-forward content |
| Current dirty `docs/Current/TASK-M11-10_SPLIT_PLAN_AND_TEMPORARY_AUTHORITY_AMENDMENT.md` deletion | worktree noise | `yes` | authority artifact is already demoted; do not use the dirty deletion as integration input |
| Current dirty `src-rust/crates/api/src/providers/google.rs` | worktree noise | `yes` | exclusion hypothesis confirmed for current uncommitted state |
| Current dirty `src-rust/crates/core/src/effort.rs` | worktree noise | `yes` | exclusion hypothesis confirmed for current uncommitted state |
| Current dirty `src-rust/crates/core/src/lib.rs` | worktree noise | `yes` | do not carry the dirty worktree state; only replay the accepted committed delta from `cf8201f` |
| Current dirty `src-rust/crates/core/src/remote_settings.rs` | worktree noise | `yes` | exclusion hypothesis confirmed for current uncommitted state |
| Current dirty `src-rust/crates/core/src/system_prompt.rs` | worktree noise | `yes` | exclusion hypothesis confirmed for current uncommitted state |
| `docs/archive/reports/*` | evidence artifacts | `yes` | record-only; do not integrate as code |
| `.codex`, `src-rust/.codex`, `src-rust/target/` | local/generated noise | `yes` | local or generated, never part of the carry-forward set |

# 12. Integration method comparison

| Method | Pros | Main risks in this repo | Assessment |
| --- | --- | --- | --- |
| Cherry-pick the exact accepted commits onto fresh upstream | preserves original commit identities; easy to audit when a commit is truly isolated | there are too many commits; several are cleanup-only; several high-value commits hit files that have drifted heavily on upstream; `2def737` would force an obsolete test layout | workable only as a limited tactic for isolated commits, not as the primary method |
| Selective manual replay / reconstruction of only the minimal accepted feature patchset | can adapt to current upstream code; can exclude docs/cleanup/noise; keeps scope aligned to accepted provider lane | requires careful code review and per-file reconstruction discipline | `safest overall` |
| Rebase `feature/provider-resolution-seam` onto `upstream/main` | retains full local history mechanically | would drag `63` feature-side commits, docs-only commits, cleanup commits, and worktree ambiguity across `48` upstream commits; very high conflict volume | `reject` |
| Merge the current feature branch into a fresh upstream branch | minimal planning effort | worst scope control; imports entire noisy branch wholesale; violates accepted-scope intent | `reject` |
| Hybrid: manual replay as primary, with one-off cherry-picks only for isolated single-file commits | can save time on a few clean commits | still requires manual review at every hotspot and must not become whole-stack cherry-picking by accident | acceptable support tactic only |

# 13. Recommended method

Recommended method: `selective manual replay / reconstruction on top of upstream/main@8359d9c`.

Why this is safest:

- The accepted carry-forward set is real, but the branch history also contains record-only docs, cleanup-only commits, and an obsolete test-module reorg.
- Upstream has materially drifted in the same high-value files that the provider-resolution lane touches, especially:
  - `src-rust/crates/query/src/agent_tool.rs`
  - `src-rust/crates/query/src/lib.rs`
  - `src-rust/crates/cli/src/main.rs`
  - `src-rust/crates/api/src/model_registry.rs`
  - `src-rust/crates/core/src/lib.rs`
  - `src-rust/crates/tui/src/app.rs`
- Upstream also now contains adjacent features that must be preserved while integrating the provider lane:
  - background-agent handling
  - worktree isolation
  - custom OpenAI-compatible provider support and provider quirks
  - broader provider registry / model registry churn

Why whole-branch rebase/merge is not acceptable:

- It would import the entire `63`-commit feature-side divergence, not just the accepted provider-resolution lane.
- It would drag record-only authority commits, lint tranches, unrelated cleanup, `.gitignore` drift, and obsolete test layout changes.
- It would make review basis isolation impossible against the current noisy worktree.

Practical recommendation:

- Use the accepted commits in section 10 as reconstruction references.
- If a later execution pass finds one of the small single-file commits applies cleanly and saves time, that can be used as a narrow support tactic.
- Do not use commit transport mechanics as an excuse to carry excluded commits from section 11.

# 14. Likely conflict hotspots

True feature-lane conflict risk:

- `src-rust/crates/query/src/agent_tool.rs`
  - Upstream already changed this file for worktree isolation and background-agent handling.
  - The provider lane adds explicit provider/model overrides, same-domain fallback, child budget carriage, worker observability, and session-scoped health-cache reuse.
  - This file is the single highest-risk manual integration seam.
- `src-rust/crates/query/src/lib.rs`
  - Upstream already has `provider_registry`, `model_registry`, and `max_budget_usd`.
  - The provider lane adds root provider resolution/materialization, session-budget registration, new `QueryEvent` variants, and root no-legacy-fallback behavior.
  - The integration must preserve upstream behavior while layering in the accepted lane.
- `src-rust/crates/cli/src/main.rs`
  - Upstream changed registry loading and interactive refresh flows.
  - The provider lane adds root `session_budget` wiring and a root cancel-token seam.
  - Manual review is required to preserve both upstream model/provider refresh behavior and the accepted budget seam.
- `src-rust/crates/tools/src/team_tool.rs`
  - File-overlap conflict risk is lower than `agent_tool.rs`, but semantic integration risk is still high.
  - The provider lane expands the injected runner contract to `AgentRunParams` and adds team-level provider/model/max-token/fallback/budget fields.
  - The later pass must keep `ARCH-03` deferred and must not accidentally redesign TeamDelete cancellation ownership.
- `src-rust/crates/api/src/provider_types.rs`
  - Upstream provider catalog has drifted.
  - The provider lane adds `TrustDomain`.
  - Fresh integration must preserve newer upstream providers and current custom-provider behavior.
- `src-rust/crates/api/src/model_registry.rs`
  - Upstream has current provider/model catalog drift.
  - The provider lane adds capability-bearing `ModelEntry` fields and fallback selection helpers.
- `src-rust/crates/query/src/provider_resolution.rs`
  - This file does not exist on `upstream/main`.
  - It must be added in a way that respects current upstream provider quirks and the expanded upstream provider catalog.
- `src-rust/crates/query/src/health_cache.rs`
  - This file does not exist on `upstream/main`.
  - Low direct conflict, but high call-site integration importance.
- `src-rust/crates/query/src/session_budget.rs`
  - This file does not exist on `upstream/main`.
  - Must coexist with upstream `max_budget_usd` semantics; do not regress the existing per-loop cap while adding the accepted shared-session seam.
- `src-rust/crates/core/src/lib.rs`
  - The provider lane extends `CostTracker`.
  - Upstream core has moved, so a literal cherry-pick is unlikely to be the cleanest integration path.
- `src-rust/crates/tui/src/app.rs`
  - The provider lane adds worker/budget observability event handling.
  - Upstream TUI has drifted substantially.
- `src-rust/crates/query/src/compact.rs`
  - `ced6005` is smaller than the seams above but still matters because provider-aware compaction is part of the accepted lane.

Upstream-adjacent drift most likely to intersect the accepted provider lane:

- query provider resolution / materialization seams
- agent tool worker path
- team tool carrier surfaces
- session budget seam
- health-cache seam
- API model/provider surfaces

Current unrelated worktree noise that should not be mistaken for feature-lane conflict input:

- `.gitignore`
- `docs/Current/TASK-M11-10_SPLIT_PLAN_AND_TEMPORARY_AUTHORITY_AMENDMENT.md`
- current dirty `google.rs` / core-file worktree edits
- `docs/archive/reports/*`
- `.codex`
- `src-rust/.codex`
- `src-rust/target/`

# 15. Proposed exact fresh-upstream integration sequence

This is the later execution sequence. It was not performed in this pass.

1. Start from a clean checkout or fresh worktree rooted at this repo, not from the current dirty worktree.
2. Fetch `upstream`, verify `upstream/main` still points to the intended base, and anchor the branch start at `upstream/main` commit `8359d9ccfe56708729f2696fb1e060da85affe4d`.
3. Create the later integration branch from that base only.
   - Suggested starting label if a new milestone naming scheme is introduced: `m13/...`
   - Do not branch from `local main`, `origin/main`, or `feature/provider-resolution-seam`.
4. Reconstruct the D1 provider-resolution foundation first, using these commits as behavioral references:
   - `a09b3da`
   - `5881983`
   - `5f8dfe1`
5. Reconstruct the M8 provider/model carriage and Team/Agent seam next:
   - `3f9b783`
   - `fe060af`
   - `f4dc962`
   - `b5249a3`
   - `5d246b2`
   - `ced6005`
   - `ea9da37`
   - `1d531da`
   - `5d472cf`
   - `2fd7732`
6. Reconstruct the accepted D1 validation/behavior layer:
   - `d76e8fb`
   - `73e9104`
   - `1056eb3`
   - `662b29a`
   - `c28ef22`
   - `2f1f169`
   - `dfc4be4`
   - `63a8485`
   - `5e77652`
   - `af97a87`
   - `6b362a0`
7. Reconstruct the accepted D2 / split-path runtime layer:
   - `dc772aa`
   - `fe21969`
   - `828b08e`
   - `eb26773`
   - `1472024`
   - `cf8201f`
   - `0942e4a`
   - `25518ca`
   - `b4ad28e`
   - `4ef9547`
   - `ea046c5`
   - `3812df0`
   - `bfabfd5`
   - `0c9dac4`
   - `b157924`
8. Reconstruct the accepted post-M11 extension:
   - `038f3c2`
9. Explicitly skip the commits and surfaces in section 11.
10. Do not carry any docs/authority/report artifacts as code changes.
11. Do not reopen `M12`, `POST-M11-ARCH-02`, or `POST-M11-ARCH-03`.
12. Manual-review checkpoints during the later execution pass must include at least:
   - `src-rust/crates/query/src/agent_tool.rs`
   - `src-rust/crates/query/src/lib.rs`
   - `src-rust/crates/cli/src/main.rs`
   - `src-rust/crates/tools/src/team_tool.rs`
   - `src-rust/crates/api/src/provider_types.rs`
   - `src-rust/crates/api/src/model_registry.rs`
   - `src-rust/crates/query/src/provider_resolution.rs`
   - `src-rust/crates/query/src/session_budget.rs`
   - `src-rust/crates/query/src/health_cache.rs`
   - `src-rust/crates/core/src/lib.rs`
   - `src-rust/crates/tui/src/app.rs`
13. Special manual-review rules for the later pass:
   - combine upstream `AgentTool` worktree/background features with the provider lane; do not drop either side
   - preserve upstream `max_budget_usd` semantics while layering in accepted `SessionBudget`
   - adapt provider materialization to current upstream provider quirks and custom-provider support
   - do not force the old test layout from `2def737`; place tests wherever the fresh integration branch layout is most correct
14. After the reconstruction lands, run the full replay/validation sequence in section 16 before any review/closure decision.

# 16. Proposed exact post-integration validation / replay sequence

Targeted replay equivalent to accepted Pass A:

1. `cd src-rust && cargo test -p claurst-query -- provider_resolution`
2. `cd src-rust && cargo test -p claurst-query -- explicit_provider_conflicts`
3. `cd src-rust && cargo test -p claurst-query -- agent_tool`
4. `cd src-rust && cargo test -p claurst-query -- provider_registry_none`
5. `cd src-rust && cargo test -p claurst-query -- provider_registry_some_resolution_failure`
6. `cd src-rust && cargo test -p claurst-query -- teamcreate_observability_is_sanitized_and_emitted`
7. `cd src-rust && cargo test -p claurst-query -- session_budget_exceeded_event_emits_only_on_new_cancellation`
8. `cd src-rust && cargo test -p claurst-api -- trust_domain`
9. `cd src-rust && cargo test -p claurst-core -- cost_tracker`
10. `cd src-rust && cargo test -p claurst-query -- health_cache`
11. `cd src-rust && cargo test -p claurst-query -- session_budget`

Broad replay equivalent to accepted Pass B:

12. `cd src-rust && cargo build --workspace`
13. `cd src-rust && cargo test -p claurst-api`
14. `cd src-rust && cargo test -p claurst-query`
15. `cd src-rust && cargo test --workspace`
16. `cd src-rust && cargo fmt --all -- --check`
17. `cd src-rust && cargo clippy --workspace --all-targets -- -D warnings`

Conditional live smoke:

18. Only if environment prerequisites are satisfied:
   - `OPENAI_API_KEY` present
   - outbound OpenAI reachability confirmed
   - then rerun the accepted parent/child smoke equivalent from Pass B

Attribution rules for the later pass:

- If `cargo clippy` fails outside the reconstructed provider lane, report it as a historical-gate probe result with file-level lane attribution rather than silently broadening scope.
- If a targeted replay fails, stop and attribute the failure before widening the integration.
- Do not reinterpret `M12` as an implementation ticket.
- Keep these accepted dispositions explicit in the later pass:
  - `M12` remains audit-only / no implementation delta
  - `POST-M11-ARCH-02` remains deferred
  - `POST-M11-ARCH-03` remains deferred

# 17. Blockers / limitations / unknowns

- The current worktree is noisy enough that a later integration pass should begin from a clean worktree or fresh clone/worktree, not from the current checkout state.
- This pass verified repo reality and remote state only. It did not compile or test against a newly created fresh-upstream integration branch.
- `upstream/main` confirmation required unrestricted network access in this environment because sandbox DNS resolution could not reach GitHub. The confirmation itself succeeded.
- The skipped cleanup commits are excluded intentionally. If a later reconstruction reveals one narrowly required behavioral detail hiding in an excluded commit, that detail should be re-evaluated explicitly rather than broadening the whole carry-forward set.
- `src-rust/crates/query/src/agent_tool.rs` is the most integration-sensitive file because upstream and the provider lane both changed it heavily for different reasons.
- `src-rust/crates/tools/src/team_tool.rs` has lower direct upstream file-drift risk, but it is still a semantic hotspot because its runner contract must match the reconstructed query-side runner.

# 18. Final verdict

`READY-FOR-LATER-FRESH-UPSTREAM-INTEGRATION-WITH-NOTES`

Exact planning outcome:

- Fresh base to use: `upstream/main` at `8359d9ccfe56708729f2696fb1e060da85affe4d`
- Safest method: selective manual replay / reconstruction of the accepted provider-resolution lane
- Exact committed carry-forward behavior set: the commits in section 10
- Exact excluded set: the commits and worktree surfaces in section 11
- Required preserved dispositions:
  - `M12` stays audit-only / no implementation delta
  - `POST-M11-ARCH-02` stays deferred
  - `POST-M11-ARCH-03` stays deferred

This preflight found no planning blocker that would prevent a later clean fresh-upstream integration pass, provided that later pass starts from `upstream/main`, uses a clean review basis, and follows the selective carry-forward boundary above.
