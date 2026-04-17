# TASK-M7-07 Blocker Audit Report

- Ticket ID: `TASK-M7-07`
- Verdict: `READY-AFTER-HOUSEKEEPING`
- Branch: `feature/provider-resolution-seam`
- No source files were edited, no files were staged, and no commits were created during this blocker audit. Only this report file was created under `docs/archive/reports/`.

## Working Tree Summary

- Verified from `git status --short --branch`:
  - `## feature/provider-resolution-seam`
  - `?? docs/`
  - `?? src-rust/target/`
- Verified from `git diff --cached --name-status`: no staged tracked changes.
- Verified from `git diff --name-status`: no unstaged tracked changes.
- Existing untracked noise under `docs/` and `src-rust/target/` was tolerated and left untouched.

## Authority Basis

Repo authority re-read:

- `/home/jordi/claurst/AGENTS.md`
- `/home/jordi/claurst/docs/Current/MPWO_WORK_ORDER_PACK.md`

Exact rule basis that makes unrelated formatting drift out of scope for `TASK-M7-07`:

- `AGENTS.md` scope discipline:
  - do not expand scope beyond the ticket’s listed files and required symbols
  - do not silently fix unrelated issues
- `AGENTS.md` validation discipline:
  - do not continue past failed validation
  - if validation fails, stop and report
  - do separate corrective patch phase only for narrow, ticket-local issues instead of widening scope
- `TASK-M7-07` in MPWO:
  - exact code targets: none; validation-only ticket
  - if the error is in an unrelated file, do not fix it; report it
  - do not modify any file not already modified in `M7-01` through `M7-06`
  - do not add new features or surrounding cleanup
  - if more than 3 files need fixes, escalate
- Standing hosted-Ollama invariant:
  - later seam-validation tickets must remain neutral to the accepted hosted-Ollama baseline from `5f8dfe1`

## Exact Rustfmt Command Run

```bash
cd /home/jordi/claurst/src-rust && cargo fmt --all -- --check
```

Evidence collection note:

- The exact command above was rerun for blocker evidence.
- For classification only, the same `cargo fmt --all -- --check` output was normalized through read-only path-extraction passes to enumerate unique surfaced files.

## Rustfmt File Classification Summary

- Unique surfaced files: `179`
- Inside authorized M7 fix scope: `1`
- Ambiguous: `1`
- Outside authorized M7 fix scope: `177`

Authorized M7 fix scope used for classification:

- `src-rust/crates/query/src/provider_resolution.rs`
- the already-modified seam section of `src-rust/crates/query/src/lib.rs`

## Full Classified File List From Rustfmt

### Inside Authorized M7 Fix Scope

```text
src-rust/crates/query/src/provider_resolution.rs
```

### Ambiguous

```text
src-rust/crates/query/src/lib.rs
```

Reason for `ambiguous` classification:

- `src-rust/crates/query/src/lib.rs` is only partially authorized under `TASK-M7-07`: the already-modified seam section.
- Rustfmt did not report only seam-local hunks. It reported many hunk positions across the file, including both seam-adjacent lines and many unrelated regions.

### Outside Authorized M7 Fix Scope

```text
src-rust/crates/acp/src/lib.rs
src-rust/crates/api/src/cch.rs
src-rust/crates/api/src/codex_adapter.rs
src-rust/crates/api/src/error_handling.rs
src-rust/crates/api/src/lib.rs
src-rust/crates/api/src/model_registry.rs
src-rust/crates/api/src/provider.rs
src-rust/crates/api/src/provider_error.rs
src-rust/crates/api/src/provider_types.rs
src-rust/crates/api/src/providers/anthropic.rs
src-rust/crates/api/src/providers/azure.rs
src-rust/crates/api/src/providers/bedrock.rs
src-rust/crates/api/src/providers/codex.rs
src-rust/crates/api/src/providers/cohere.rs
src-rust/crates/api/src/providers/copilot.rs
src-rust/crates/api/src/providers/google.rs
src-rust/crates/api/src/providers/message_normalization.rs
src-rust/crates/api/src/providers/openai.rs
src-rust/crates/api/src/providers/openai_compat.rs
src-rust/crates/api/src/providers/openai_compat_providers.rs
src-rust/crates/api/src/providers/request_options.rs
src-rust/crates/api/src/registry.rs
src-rust/crates/api/src/stream_parser.rs
src-rust/crates/api/src/transform.rs
src-rust/crates/api/src/transformers/anthropic.rs
src-rust/crates/api/src/transformers/openai_chat.rs
src-rust/crates/bridge/src/lib.rs
src-rust/crates/cli/src/codex_oauth_flow.rs
src-rust/crates/cli/src/main.rs
src-rust/crates/cli/src/oauth_flow.rs
src-rust/crates/commands/src/lib.rs
src-rust/crates/commands/src/named_commands.rs
src-rust/crates/core/src/attachments.rs
src-rust/crates/core/src/auth_store.rs
src-rust/crates/core/src/bash_classifier.rs
src-rust/crates/core/src/claudemd.rs
src-rust/crates/core/src/cloud_session.rs
src-rust/crates/core/src/crypto_utils.rs
src-rust/crates/core/src/effort.rs
src-rust/crates/core/src/feature_gates.rs
src-rust/crates/core/src/format_utils.rs
src-rust/crates/core/src/git_utils.rs
src-rust/crates/core/src/keybindings.rs
src-rust/crates/core/src/lib.rs
src-rust/crates/core/src/lsp.rs
src-rust/crates/core/src/mcp_templates.rs
src-rust/crates/core/src/memdir.rs
src-rust/crates/core/src/message_utils.rs
src-rust/crates/core/src/migrations.rs
src-rust/crates/core/src/oauth_config.rs
src-rust/crates/core/src/output_styles.rs
src-rust/crates/core/src/prompt_history.rs
src-rust/crates/core/src/provider_id.rs
src-rust/crates/core/src/ps_classifier.rs
src-rust/crates/core/src/remote_session.rs
src-rust/crates/core/src/remote_settings.rs
src-rust/crates/core/src/session_share.rs
src-rust/crates/core/src/session_storage.rs
src-rust/crates/core/src/session_tracing.rs
src-rust/crates/core/src/settings_sync.rs
src-rust/crates/core/src/skill_discovery.rs
src-rust/crates/core/src/status_notices.rs
src-rust/crates/core/src/system_prompt.rs
src-rust/crates/core/src/team_memory_sync.rs
src-rust/crates/core/src/tips.rs
src-rust/crates/core/src/token_budget.rs
src-rust/crates/core/src/truncate.rs
src-rust/crates/core/src/update_check.rs
src-rust/crates/core/src/voice.rs
src-rust/crates/core/tests/parity_smoke.rs
src-rust/crates/core/tests/test_mcp_templates.rs
src-rust/crates/mcp/src/connection_manager.rs
src-rust/crates/mcp/src/lib.rs
src-rust/crates/mcp/src/oauth.rs
src-rust/crates/mcp/src/registry.rs
src-rust/crates/plugins/src/hooks.rs
src-rust/crates/plugins/src/lib.rs
src-rust/crates/plugins/src/loader.rs
src-rust/crates/plugins/src/manifest.rs
src-rust/crates/plugins/src/marketplace.rs
src-rust/crates/plugins/src/registry.rs
src-rust/crates/query/src/agent_tool.rs
src-rust/crates/query/src/auto_dream.rs
src-rust/crates/query/src/away_summary.rs
src-rust/crates/query/src/command_queue.rs
src-rust/crates/query/src/compact.rs
src-rust/crates/query/src/context_analyzer.rs
src-rust/crates/query/src/coordinator.rs
src-rust/crates/query/src/cron_scheduler.rs
src-rust/crates/query/src/session_memory.rs
src-rust/crates/query/src/skill_prefetch.rs
src-rust/crates/tools/src/apply_patch.rs
src-rust/crates/tools/src/ask_user.rs
src-rust/crates/tools/src/bash.rs
src-rust/crates/tools/src/batch_edit.rs
src-rust/crates/tools/src/brief.rs
src-rust/crates/tools/src/bundled_skills.rs
src-rust/crates/tools/src/computer_use.rs
src-rust/crates/tools/src/config_tool.rs
src-rust/crates/tools/src/cron.rs
src-rust/crates/tools/src/enter_plan_mode.rs
src-rust/crates/tools/src/exit_plan_mode.rs
src-rust/crates/tools/src/file_edit.rs
src-rust/crates/tools/src/file_read.rs
src-rust/crates/tools/src/file_write.rs
src-rust/crates/tools/src/formatter.rs
src-rust/crates/tools/src/glob_tool.rs
src-rust/crates/tools/src/grep_tool.rs
src-rust/crates/tools/src/lib.rs
src-rust/crates/tools/src/lsp_tool.rs
src-rust/crates/tools/src/mcp_resources.rs
src-rust/crates/tools/src/notebook_edit.rs
src-rust/crates/tools/src/powershell.rs
src-rust/crates/tools/src/pty_bash.rs
src-rust/crates/tools/src/remote_trigger.rs
src-rust/crates/tools/src/repl_tool.rs
src-rust/crates/tools/src/send_message.rs
src-rust/crates/tools/src/skill_tool.rs
src-rust/crates/tools/src/sleep.rs
src-rust/crates/tools/src/tasks.rs
src-rust/crates/tools/src/team_tool.rs
src-rust/crates/tools/src/todo_write.rs
src-rust/crates/tools/src/tool_search.rs
src-rust/crates/tools/src/web_fetch.rs
src-rust/crates/tools/src/web_search.rs
src-rust/crates/tools/src/worktree.rs
src-rust/crates/tui/src/agents_view.rs
src-rust/crates/tui/src/app.rs
src-rust/crates/tui/src/bridge_state.rs
src-rust/crates/tui/src/bypass_permissions_dialog.rs
src-rust/crates/tui/src/context_viz.rs
src-rust/crates/tui/src/desktop_upsell_startup.rs
src-rust/crates/tui/src/device_auth_dialog.rs
src-rust/crates/tui/src/dialog_select.rs
src-rust/crates/tui/src/dialogs.rs
src-rust/crates/tui/src/diff_viewer.rs
src-rust/crates/tui/src/elicitation_dialog.rs
src-rust/crates/tui/src/export_dialog.rs
src-rust/crates/tui/src/feedback_survey.rs
src-rust/crates/tui/src/figures.rs
src-rust/crates/tui/src/hooks_config_menu.rs
src-rust/crates/tui/src/image_paste.rs
src-rust/crates/tui/src/invalid_config_dialog.rs
src-rust/crates/tui/src/key_input_dialog.rs
src-rust/crates/tui/src/kitty_image.rs
src-rust/crates/tui/src/lib.rs
src-rust/crates/tui/src/mcp_view.rs
src-rust/crates/tui/src/memory_file_selector.rs
src-rust/crates/tui/src/memory_update_notification.rs
src-rust/crates/tui/src/message_copy.rs
src-rust/crates/tui/src/messages/markdown.rs
src-rust/crates/tui/src/messages/markdown_enhanced.rs
src-rust/crates/tui/src/messages/mod.rs
src-rust/crates/tui/src/model_picker.rs
src-rust/crates/tui/src/notifications.rs
src-rust/crates/tui/src/onboarding_dialog.rs
src-rust/crates/tui/src/overage_upsell.rs
src-rust/crates/tui/src/overlays.rs
src-rust/crates/tui/src/plugin_views.rs
src-rust/crates/tui/src/privacy_screen.rs
src-rust/crates/tui/src/prompt_input.rs
src-rust/crates/tui/src/render.rs
src-rust/crates/tui/src/rustle.rs
src-rust/crates/tui/src/session_branching.rs
src-rust/crates/tui/src/session_browser.rs
src-rust/crates/tui/src/settings_screen.rs
src-rust/crates/tui/src/stats_dialog.rs
src-rust/crates/tui/src/tasks_overlay.rs
src-rust/crates/tui/src/theme_colors.rs
src-rust/crates/tui/src/theme_screen.rs
src-rust/crates/tui/src/transcript_turn.rs
src-rust/crates/tui/src/virtual_list.rs
src-rust/crates/tui/src/voice_capture.rs
src-rust/crates/tui/src/voice_mode_notice.rs
src-rust/crates/tui/tests/diff_viewer.rs
src-rust/crates/tui/tests/markdown_enhancements.rs
src-rust/crates/tui/tests/render_snapshots.rs
```

## Whether Rustfmt Drift Overlaps Authorized M7 Files

- `src-rust/crates/query/src/provider_resolution.rs`: yes
  - rustfmt reported hunk positions at lines `1`, `185`, `255`, and `373`
  - classification: inside authorized M7 fix scope
- `src-rust/crates/query/src/lib.rs`: yes
  - rustfmt reported hunk positions at many locations across the file, including:
    - early unrelated regions: `19`, `39`, `61`, `159`, `179`, `189`, `197`, `206`, `285`, `298`, `365`, `382`, `412`, `424`, `488`, `571`, `613`, `622`, `687`
    - seam-adjacent regions: `830`, `839`, `904`, `922`, `932`, `954`, `973`
    - later unrelated regions: `1078`, `1086`, `1109`, `1180`, `1360`, `1676`, `1723`, `1789`, `1897`, `1915`, `1919`, `1933`, `1942`, `1953`
  - classification: ambiguous, because the returned drift is not confined to the already-modified seam section

Conclusion:

- The fmt blocker is not entirely outside M7 scope.
- But it is overwhelmingly outside M7 scope:
  - `1` file clearly inside scope
  - `1` file ambiguous because rustfmt reported whole-file drift, not just seam-local drift
  - `177` files clearly outside scope
- Under current authority, this is not a valid `TASK-M7-07` corrective patch candidate.

## Hosted Ollama Invariant Status

Hosted Ollama compatibility baseline preserved

- No code changes were made.
- Nothing weakened, bypassed, or replaced:
  - `normalize_ollama_api_base(...)`
  - Ollama auth lookup via `AuthStore::load().api_key_for(ProviderId::OLLAMA)`
  - the accepted hosted Ollama compatibility baseline from `5f8dfe1`

## Recommended Next Governance Action

`SEPARATE_HOUSEKEEPING_TICKET_REQUIRED`

Justification:

- The formatting blocker spans `179` unique files across `acp`, `api`, `bridge`, `cli`, `commands`, `core`, `mcp`, `plugins`, `query`, `tools`, and `tui`.
- `177` of those files are clearly outside the authorized `TASK-M7-07` fix scope.
- The only clearly in-scope file is `src-rust/crates/query/src/provider_resolution.rs`.
- `src-rust/crates/query/src/lib.rs` is not safely actionable under `TASK-M7-07` as a pure seam-local fix because rustfmt reported broad whole-file drift, not only the already-modified seam section.
- The clean governance move is to establish a separate formatting-housekeeping baseline outside `TASK-M7-07`, then rerun `TASK-M7-07` against that accepted baseline.

## Notes

- This audit did not run `cargo build --workspace`, `cargo test --workspace`, or `cargo clippy --workspace --all-targets`.
- This audit reran only the required rustfmt check for blocker evidence.
- No source files were edited.
- No files were staged or committed.
