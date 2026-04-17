# Repo File Tree Snapshot

Generated at `20260416T154521Z` from `/home/jordi/claurst`.

## Note on `docs/`

`docs/` exists in this repository for working context, reports, plans, RFCs, and other supporting material. It should not be treated as the main product/source tree. The primary implementation surface lives in the root project files, `public/`, `spec/`, and especially `src-rust/`.

## Core Repository Tree

This view intentionally excludes `docs/` so the product/code tree is easier to read. It also excludes common generated/artifact paths: `.git` and `target`.

```text
.
├── .claude
│   └── settings.local.json
├── .codex
├── .envrc
├── .envrc.bak
├── .github
│   └── workflows
│       ├── pages.yml
│       └── release.yml
├── .gitignore
├── AGENTS.md
├── CLAUDE.md
├── GEMINI.md
├── LICENSE.md
├── README.md
├── public
│   ├── Rustle.png
│   ├── claude-files.png
│   ├── leak-tweet.png
│   └── screenshot.png
├── spec
│   ├── 00_overview.md
│   ├── 01_core_entry_query.md
│   ├── 02_commands.md
│   ├── 03_tools.md
│   ├── 04_components_core_messages.md
│   ├── 05_components_agents_permissions_design.md
│   ├── 06_services_context_state.md
│   ├── 07_hooks.md
│   ├── 08_ink_terminal.md
│   ├── 09_bridge_cli_remote.md
│   ├── 10_utils.md
│   ├── 11_special_systems.md
│   ├── 12_constants_types.md
│   ├── 13_rust_codebase.md
│   └── INDEX.md
└── src-rust
    ├── .codex
    ├── Cargo.lock
    ├── Cargo.toml
    ├── LICENSE
    └── crates
        ├── acp
        │   ├── Cargo.toml
        │   └── src
        │       └── lib.rs
        ├── api
        │   ├── Cargo.toml
        │   └── src
        │       ├── auth.rs
        │       ├── cch.rs
        │       ├── codex_adapter.rs
        │       ├── error_handling.rs
        │       ├── lib.rs
        │       ├── model_registry.rs
        │       ├── provider.rs
        │       ├── provider_error.rs
        │       ├── provider_types.rs
        │       ├── providers
        │       │   ├── anthropic.rs
        │       │   ├── azure.rs
        │       │   ├── bedrock.rs
        │       │   ├── codex.rs
        │       │   ├── cohere.rs
        │       │   ├── copilot.rs
        │       │   ├── google.rs
        │       │   ├── message_normalization.rs
        │       │   ├── mod.rs
        │       │   ├── openai.rs
        │       │   ├── openai_compat.rs
        │       │   ├── openai_compat_providers.rs
        │       │   └── request_options.rs
        │       ├── registry.rs
        │       ├── stream_parser.rs
        │       ├── transform.rs
        │       └── transformers
        │           ├── anthropic.rs
        │           ├── mod.rs
        │           └── openai_chat.rs
        ├── bridge
        │   ├── Cargo.toml
        │   └── src
        │       └── lib.rs
        ├── buddy
        │   ├── Cargo.toml
        │   └── src
        │       └── lib.rs
        ├── cli
        │   ├── Cargo.toml
        │   ├── build.rs
        │   └── src
        │       ├── codex_oauth_flow.rs
        │       ├── main.rs
        │       ├── oauth_flow.rs
        │       └── system_prompt.txt
        ├── commands
        │   ├── Cargo.toml
        │   └── src
        │       ├── lib.rs
        │       └── named_commands.rs
        ├── core
        │   ├── Cargo.toml
        │   ├── src
        │   │   ├── analytics.rs
        │   │   ├── attachments.rs
        │   │   ├── auth_store.rs
        │   │   ├── auto_mode.rs
        │   │   ├── bash_classifier.rs
        │   │   ├── claudemd.rs
        │   │   ├── cloud_session.rs
        │   │   ├── codex_oauth.rs
        │   │   ├── context_collapse.rs
        │   │   ├── crypto_utils.rs
        │   │   ├── device_code.rs
        │   │   ├── effort.rs
        │   │   ├── feature_flags.rs
        │   │   ├── feature_gates.rs
        │   │   ├── file_history.rs
        │   │   ├── format_utils.rs
        │   │   ├── git_utils.rs
        │   │   ├── ide.rs
        │   │   ├── keybindings.rs
        │   │   ├── lib.rs
        │   │   ├── lsp.rs
        │   │   ├── mcp_templates.rs
        │   │   ├── memdir.rs
        │   │   ├── message_utils.rs
        │   │   ├── migrations.rs
        │   │   ├── oauth_config.rs
        │   │   ├── output_styles.rs
        │   │   ├── prompt_history.rs
        │   │   ├── provider_id.rs
        │   │   ├── ps_classifier.rs
        │   │   ├── remote_session.rs
        │   │   ├── remote_settings.rs
        │   │   ├── session_share.rs
        │   │   ├── session_storage.rs
        │   │   ├── session_tracing.rs
        │   │   ├── settings_sync.rs
        │   │   ├── skill_discovery.rs
        │   │   ├── snapshot.rs
        │   │   ├── sqlite_storage.rs
        │   │   ├── status_notices.rs
        │   │   ├── system_prompt.rs
        │   │   ├── team_memory_sync.rs
        │   │   ├── tips.rs
        │   │   ├── token_budget.rs
        │   │   ├── truncate.rs
        │   │   ├── update_check.rs
        │   │   └── voice.rs
        │   └── tests
        │       ├── parity_smoke.rs
        │       └── test_mcp_templates.rs
        ├── mcp
        │   ├── Cargo.toml
        │   └── src
        │       ├── connection_manager.rs
        │       ├── lib.rs
        │       ├── oauth.rs
        │       └── registry.rs
        ├── plugins
        │   ├── Cargo.toml
        │   └── src
        │       ├── hooks.rs
        │       ├── lib.rs
        │       ├── loader.rs
        │       ├── manifest.rs
        │       ├── marketplace.rs
        │       ├── plugin.rs
        │       └── registry.rs
        ├── query
        │   ├── Cargo.toml
        │   └── src
        │       ├── agent_tool.rs
        │       ├── agent_tool_tests.rs
        │       ├── auto_dream.rs
        │       ├── away_summary.rs
        │       ├── command_queue.rs
        │       ├── compact.rs
        │       ├── context_analyzer.rs
        │       ├── coordinator.rs
        │       ├── cron_scheduler.rs
        │       ├── health_cache.rs
        │       ├── lib.rs
        │       ├── provider_resolution.rs
        │       ├── provider_resolution_tests.rs
        │       ├── session_budget.rs
        │       ├── session_memory.rs
        │       └── skill_prefetch.rs
        ├── tools
        │   ├── Cargo.toml
        │   └── src
        │       ├── agent_tool.rs
        │       ├── apply_patch.rs
        │       ├── ask_user.rs
        │       ├── bash.rs
        │       ├── batch_edit.rs
        │       ├── brief.rs
        │       ├── bundled_skills.rs
        │       ├── computer_use.rs
        │       ├── config_tool.rs
        │       ├── cron.rs
        │       ├── enter_plan_mode.rs
        │       ├── exit_plan_mode.rs
        │       ├── file_edit.rs
        │       ├── file_read.rs
        │       ├── file_write.rs
        │       ├── formatter.rs
        │       ├── glob_tool.rs
        │       ├── grep_tool.rs
        │       ├── lib.rs
        │       ├── lsp_tool.rs
        │       ├── mcp_auth_tool.rs
        │       ├── mcp_resources.rs
        │       ├── notebook_edit.rs
        │       ├── powershell.rs
        │       ├── pty_bash.rs
        │       ├── remote_trigger.rs
        │       ├── repl_tool.rs
        │       ├── send_message.rs
        │       ├── skill_tool.rs
        │       ├── sleep.rs
        │       ├── synthetic_output.rs
        │       ├── tasks.rs
        │       ├── team_tool.rs
        │       ├── todo_write.rs
        │       ├── tool_search.rs
        │       ├── web_fetch.rs
        │       ├── web_search.rs
        │       └── worktree.rs
        └── tui
            ├── Cargo.toml
            ├── src
            │   ├── agents_view.rs
            │   ├── app.rs
            │   ├── bridge_state.rs
            │   ├── bypass_permissions_dialog.rs
            │   ├── context_viz.rs
            │   ├── desktop_upsell_startup.rs
            │   ├── device_auth_dialog.rs
            │   ├── dialog_select.rs
            │   ├── dialogs.rs
            │   ├── diff_viewer.rs
            │   ├── elicitation_dialog.rs
            │   ├── export_dialog.rs
            │   ├── feedback_survey.rs
            │   ├── figures.rs
            │   ├── hooks_config_menu.rs
            │   ├── image_paste.rs
            │   ├── input.rs
            │   ├── invalid_config_dialog.rs
            │   ├── key_input_dialog.rs
            │   ├── kitty_image.rs
            │   ├── lib.rs
            │   ├── mcp_view.rs
            │   ├── memory_file_selector.rs
            │   ├── memory_update_notification.rs
            │   ├── message_copy.rs
            │   ├── messages
            │   │   ├── markdown.rs
            │   │   ├── markdown_enhanced.rs
            │   │   └── mod.rs
            │   ├── model_picker.rs
            │   ├── notifications.rs
            │   ├── onboarding_dialog.rs
            │   ├── overage_upsell.rs
            │   ├── overlays.rs
            │   ├── plugin_views.rs
            │   ├── privacy_screen.rs
            │   ├── prompt_input.rs
            │   ├── render.rs
            │   ├── rustle.rs
            │   ├── session_branching.rs
            │   ├── session_browser.rs
            │   ├── settings_screen.rs
            │   ├── stats_dialog.rs
            │   ├── tasks_overlay.rs
            │   ├── theme_colors.rs
            │   ├── theme_screen.rs
            │   ├── transcript_turn.rs
            │   ├── virtual_list.rs
            │   ├── voice_capture.rs
            │   └── voice_mode_notice.rs
            └── tests
                ├── diff_viewer.rs
                ├── markdown_enhancements.rs
                └── render_snapshots.rs

37 directories, 251 files
```

## `docs/` Context Tree

This view is separated on purpose. `docs/` is for project context, reports, plans, RFCs, and other supporting artifacts rather than the main runtime/code surface.

```text
docs
├── Current
│   ├── D1_REVIEW_REPORT_20260413T233604Z.md
│   ├── HOSTED_OLLAMA_IMPLEMENTATION_PLAN.md
│   ├── IMPLEMENTATION_PLAN_MPWO.md
│   ├── M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md
│   ├── MPWO_WORK_ORDER_PACK.md
│   ├── MPWO_WORK_ORDER_PACK_pre_M10_revision.md
│   └── RFC_PROVIDER_AWARE_WORKER_FABRIC_v3.1.md
├── Orchestrator_planning
│   ├── Claurst_progress_report.md
│   ├── Codex5.4_FEATURE_BRAINSTORM.md
│   ├── FEATURE_BRAINSTORM.md
│   ├── IMPLEMENTATION_PLAN_MPWO.md
│   └── Prompt for Opus claurst.md
└── archive
    ├── provider_orchestrator
    │   ├── CODEX_PROMPT_PACK_TASK_M7_01_NO_AGENT.md
    │   ├── CODEX_PROMPT_PACK_TASK_M7_02_NO_AGENT.md
    │   ├── CODEX_PROMPT_PACK_TASK_M7_03_NO_AGENT.md
    │   ├── FUNCTIONAL_SPEC.md
    │   ├── Prompt for Opus claurst.md
    │   ├── RFC_PROVIDER_AWARE_WORKER_FABRIC.md
    │   ├── RFC_PROVIDER_AWARE_WORKER_FABRIC_v2.md
    │   ├── RFC_PROVIDER_AWARE_WORKER_FABRIC_v2_REVIEW.md
    │   ├── RFC_PROVIDER_AWARE_WORKER_FABRIC_v3.md
    │   ├── TASK-M11-10_SPLIT_PLAN_AND_TEMPORARY_AUTHORITY_AMENDMENT.md
    │   ├── TASK-M7-01_COMPILE_FIX_REPORT.md
    │   ├── TASK-M7-01_CORRECTIVE_PATCH_REVIEW.md
    │   ├── TASK-M7-01_EXECUTION_REPORT.md
    │   ├── TASK-M7-01_PREFLIGHT_REPORT.md
    │   ├── TASK-M7-02_ACTIVE_UNSTAGED_PATCH_REVIEW.md
    │   ├── TASK-M7-02_EXECUTION_REPORT.md
    │   ├── TASK-M7-02_PATCH_HYGIENE_REPORT.md
    │   ├── TASK-M7-02_REVIEW_REPORT.md
    │   ├── d2_plan.json
    │   ├── d2_plan_summary.md
    │   ├── d2_test_micro_patch_report.md
    │   ├── opus_work_order_prompt.md
    │   └── provider_reconciliation_report.md
    └── reports
        ├── BRANCH_LINEAGE_AND_PUSH_STATUS_REPORT_20260416T142903Z.md
        ├── CODEX_ADAPTER_TEST_BLOCKER_RESOLUTION_REPORT_20260412T031935Z.md
        ├── D1_REVIEW_REPORT_20260413T233604Z.md
        ├── DOCS_FILE_TREE_20260414T011217Z.md
        ├── HOSTED_OLLAMA_COMPATIBILITY_FIX_ACCEPTANCE_REPORT.md
        ├── HOSTED_OLLAMA_COMPATIBILITY_FIX_EXECUTION_REPORT_20260411T222751Z.md
        ├── HOSTED_OLLAMA_COMPATIBILITY_FIX_LIVE_VALIDATION_REPORT.md
        ├── HOSTED_OLLAMA_SMOKE_TEST_REPORT.md
        ├── LOCAL_MULTI_AGENT_ORCHESTRATION_PLAN_20260414T033412Z.md
        ├── LOCAL_MULTI_AGENT_ORCHESTRATION_PLAN_20260414T033412Z_v2.md
        ├── M10_D2_IMPLEMENTATION_PLAN_REPORT_20260413T234613Z.md
        ├── M11_CONVERGENCE_REVIEW_20260415T042825Z.md
        ├── M12_CLOSEOUT_REPORT_20260416T065308Z.md
        ├── M12_RECUT_PREFLIGHT_REPORT_20260416T064259Z.md
        ├── MILESTONE_M8_WRAPUP_CHECK_20260413T004403Z.md
        ├── MPWO_REVISION_REPORT_20260414T003218Z.md
        ├── ONBOARDING_DIALOG_TEST_BLOCKER_RESOLUTION_REPORT_20260412T032935Z.md
        ├── POST_M11_01_CLOSEOUT_REPORT_20260415T074353Z.md
        ├── POST_M11_01_EXECUTION_REPORT_20260415T073149Z.md
        ├── POST_M11_01_PREFLIGHT_REPORT_20260415T072257Z.md
        ├── POST_M11_01_VERIFICATION_REPORT_20260415T073951Z.md
        ├── POST_M11_02_CLOSEOUT_REPORT_20260415T082535Z.md
        ├── POST_M11_02_EXECUTION_REPORT_20260415T075425Z.md
        ├── POST_M11_02_PREFLIGHT_REPORT_20260415T075008Z.md
        ├── POST_M11_02_VERIFICATION_REPORT_20260415T081055Z.md
        ├── POST_M11_03_CLOSEOUT_REPORT_20260415T090527Z.md
        ├── POST_M11_03_EXECUTION_REPORT_20260415T084629Z.md
        ├── POST_M11_03_PREFLIGHT_REPORT_20260415T083548Z.md
        ├── POST_M11_03_VERIFICATION_REPORT_20260415T085443Z.md
        ├── POST_M11_04_CLOSEOUT_REPORT_20260415T102652Z.md
        ├── POST_M11_04_EXECUTION_REPORT_20260415T100440Z.md
        ├── POST_M11_04_PREFLIGHT_REPORT_20260415T093128Z.md
        ├── POST_M11_04_VERIFICATION_REPORT_20260415T101803Z.md
        ├── POST_M11_05_CLOSEOUT_REPORT_20260415T122710Z.md
        ├── POST_M11_05_EXECUTION_REPORT_20260415T115909Z.md
        ├── POST_M11_05_PREFLIGHT_REPORT_20260415T112425Z.md
        ├── POST_M11_05_VERIFICATION_REPORT_20260415T121334Z.md
        ├── POST_M11_06A_CLOSEOUT_REPORT_20260415T130143Z.md
        ├── POST_M11_06A_EXECUTION_REPORT_20260415T125011Z.md
        ├── POST_M11_06A_VERIFICATION_REPORT_20260415T125645Z.md
        ├── POST_M11_06B_CLOSEOUT_REPORT_20260415T132352Z.md
        ├── POST_M11_06B_EXECUTION_REPORT_20260415T131311Z.md
        ├── POST_M11_06B_PREFLIGHT_REPORT_20260415T130723Z.md
        ├── POST_M11_06B_VERIFICATION_REPORT_20260415T131851Z.md
        ├── POST_M11_06C_CLOSEOUT_REPORT_20260415T134926Z.md
        ├── POST_M11_06C_EXECUTION_REPORT_20260415T134121Z.md
        ├── POST_M11_06C_PREFLIGHT_REPORT_20260415T133059Z.md
        ├── POST_M11_06C_VERIFICATION_REPORT_20260415T134524Z.md
        ├── POST_M11_06_PREFLIGHT_REPORT_20260415T124322Z.md
        ├── POST_M11_07A_CLOSEOUT_REPORT_20260415T141904Z.md
        ├── POST_M11_07A_EXECUTION_REPORT_20260415T140906Z.md
        ├── POST_M11_07A_VERIFICATION_REPORT_20260415T141319Z.md
        ├── POST_M11_07B_CLOSEOUT_REPORT_20260415T144345Z.md
        ├── POST_M11_07B_EXECUTION_REPORT_20260415T143313Z.md
        ├── POST_M11_07B_PREFLIGHT_REPORT_20260415T142531Z.md
        ├── POST_M11_07B_VERIFICATION_REPORT_20260415T143903Z.md
        ├── POST_M11_07C_CLOSEOUT_REPORT_20260415T150817Z.md
        ├── POST_M11_07C_EXECUTION_REPORT_20260415T145730Z.md
        ├── POST_M11_07C_PREFLIGHT_REPORT_20260415T145045Z.md
        ├── POST_M11_07C_VERIFICATION_REPORT_20260415T150217Z.md
        ├── POST_M11_07_PREFLIGHT_REPORT_20260415T135950Z.md
        ├── POST_M11_08A_CLOSEOUT_REPORT_20260416T001346Z.md
        ├── POST_M11_08A_EXECUTION_REPORT_20260415T235711Z.md
        ├── POST_M11_08A_EXECUTION_REPORT_20260416T000216Z.md
        ├── POST_M11_08A_PREFLIGHT_REPORT_20260415T233909Z.md
        ├── POST_M11_08A_VERIFICATION_REPORT_20260416T000815Z.md
        ├── POST_M11_08_PREFLIGHT_REPORT_20260415T232456Z.md
        ├── POST_M11_09_PREFLIGHT_REPORT_20260416T002821Z.md
        ├── POST_M11_ARCH_01A_CLOSEOUT_REPORT_20260416T081839Z.md
        ├── POST_M11_ARCH_01A_EXECUTION_REPORT_20260416T074841Z.md
        ├── POST_M11_ARCH_01A_VERIFICATION_REPORT_20260416T080436Z.md
        ├── POST_M11_ARCH_01_PREFLIGHT_REPORT_20260416T072504Z.md
        ├── POST_M11_ARCH_02_PREFLIGHT_REPORT_20260416T084418Z.md
        ├── POST_M11_ARCH_03_PREFLIGHT_REPORT_20260416T091621Z.md
        ├── POST_M11_FULL_REPO_EVALUATION_DEFERRED_ITEMS_AND_M12_20260416T053356Z.md
        ├── POST_M11_M12_AUTH_RECON_CLOSEOUT_REPORT_20260416T063617Z.md
        ├── POST_M11_M12_AUTH_RECON_EXECUTION_REPORT_20260416T061647Z.md
        ├── POST_M11_M12_AUTH_RECON_PREFLIGHT_REPORT_20260416T060529Z.md
        ├── POST_M11_M12_AUTH_RECON_VERIFICATION_REPORT_20260416T062420Z.md
        ├── POST_M11_MILESTONE_ACCEPTANCE_REPLAY_PASS_A_REPORT_20260416T110927Z.md
        ├── POST_M11_MILESTONE_ACCEPTANCE_REPLAY_PASS_A_REPORT_20260416T135153Z.md
        ├── POST_M11_MILESTONE_ACCEPTANCE_REPLAY_PASS_B_REPORT_20260416T112120Z.md
        ├── POST_M11_MILESTONE_ACCEPTANCE_REPLAY_PASS_B_REPORT_20260416T124413Z.md
        ├── POST_M11_MILESTONE_ACCEPTANCE_REPLAY_PASS_B_REPORT_20260416T132128Z.md
        ├── POST_M11_MILESTONE_ACCEPTANCE_REPLAY_PASS_B_REPORT_20260416T135621Z.md
        ├── POST_M11_MILESTONE_ACCEPTANCE_REPLAY_PLAN_REPORT_20260416T100911Z.md
        ├── POST_M11_REPO_ASSESSMENT_20260415T071321Z.md
        ├── POST_M11_UPSTREAM_INTEGRATION_PREFLIGHT_REPORT_20260416T145653Z.md
        ├── POST_M8_05_COMPACTION_HARDENING_CLOSEOUT_REPORT_20260412T130308Z.md
        ├── POST_M8_05_COMPACTION_HARDENING_COMMIT_VERIFICATION_REPORT_20260412T131713Z.md
        ├── POST_M8_05_COMPACTION_HARDENING_COMMIT_VERIFICATION_REPORT_20260412T132915Z.md
        ├── POST_M8_05_COMPACTION_HARDENING_EXECUTION_REPORT_20260412T124625Z.md
        ├── POST_M8_05_COMPACTION_HARDENING_PREFLIGHT_REPORT_20260412T122507Z.md
        ├── POST_M8_05_COMPACTION_HARDENING_REVIEW_REPORT_20260412T125252Z.md
        ├── PRE_M8_01_RECON_REPORT_20260412T041306Z.md
        ├── REPO_FILE_TREE_SNAPSHOT_20260415T065827Z.md
        ├── REPO_FILE_TREE_SNAPSHOT_20260416T045542Z.md
        ├── REPO_FILE_TREE_SNAPSHOT_20260416T154521Z.md
        ├── ROOT_GITIGNORE_HOUSEKEEPING_REPORT_20260411T232304Z.md
        ├── SMOKE_KEYS_RECHECK_REPORT_20260416T134444Z.md
        ├── TASK-M11-01_AUTHORITY_ALIGNMENT_REPORT_20260414T020835Z.md
        ├── TASK-M11-01_CLOSEOUT_REPORT_20260414T022628Z.md
        ├── TASK-M11-01_EXECUTION_REPORT_20260414T021527Z.md
        ├── TASK-M11-01_PREFLIGHT_REPORT_20260414T015011Z.md
        ├── TASK-M11-01_VERIFICATION_REPORT_20260414T022325Z.md
        ├── TASK-M11-02_CLOSEOUT_REPORT_20260414T025746Z.md
        ├── TASK-M11-02_EXECUTION_REPORT_20260414T023634Z.md
        ├── TASK-M11-02_PREFLIGHT_REPORT_20260414T023134Z.md
        ├── TASK-M11-02_VERIFICATION_REPORT_20260414T024804Z.md
        ├── TASK-M11-03_CLOSEOUT_REPORT_20260414T134737Z.md
        ├── TASK-M11-03_EXECUTION_REPORT_20260414T133913Z.md
        ├── TASK-M11-03_PREFLIGHT_REPORT_20260414T133214Z.md
        ├── TASK-M11-03_VERIFICATION_REPORT_20260414T134301Z.md
        ├── TASK-M11-04_CLOSEOUT_REPORT_20260414T141351Z.md
        ├── TASK-M11-04_EXECUTION_REPORT_20260414T140308Z.md
        ├── TASK-M11-04_PREFLIGHT_REPORT_20260414T135634Z.md
        ├── TASK-M11-04_VERIFICATION_REPORT_20260414T141056Z.md
        ├── TASK-M11-05_CLOSEOUT_REPORT_20260414T143657Z.md
        ├── TASK-M11-05_EXECUTION_REPORT_20260414T142928Z.md
        ├── TASK-M11-05_PREFLIGHT_REPORT_20260414T142046Z.md
        ├── TASK-M11-05_VERIFICATION_REPORT_20260414T143304Z.md
        ├── TASK-M11-06_CLOSEOUT_REPORT_20260414T150212Z.md
        ├── TASK-M11-06_EXECUTION_REPORT_20260414T145531Z.md
        ├── TASK-M11-06_PREFLIGHT_REPORT_20260414T145113Z.md
        ├── TASK-M11-06_VERIFICATION_REPORT_20260414T145815Z.md
        ├── TASK-M11-07_CLOSEOUT_REPORT_20260414T151840Z.md
        ├── TASK-M11-07_EXECUTION_REPORT_20260414T151247Z.md
        ├── TASK-M11-07_PREFLIGHT_REPORT_20260414T150920Z.md
        ├── TASK-M11-07_VERIFICATION_REPORT_20260414T151543Z.md
        ├── TASK-M11-08B_AUTHORITY_REPORT_20260415T005148Z.md
        ├── TASK-M11-08B_CLOSEOUT_REPORT_20260415T011952Z.md
        ├── TASK-M11-08B_EXECUTION_REPORT_20260415T010729Z.md
        ├── TASK-M11-08B_PREFLIGHT_REPORT_20260415T005753Z.md
        ├── TASK-M11-08B_VERIFICATION_REPORT_20260415T011238Z.md
        ├── TASK-M11-08R_CLOSEOUT_REPORT_20260415T004401Z.md
        ├── TASK-M11-08R_EXECUTION_REPORT_20260415T002914Z.md
        ├── TASK-M11-08R_FINAL_PATCH_REPORT_20260415T004124Z.md
        ├── TASK-M11-08R_PATCH_REPORT_20260415T003155Z.md
        ├── TASK-M11-08R_VERIFICATION_REPORT_20260415T003838Z.md
        ├── TASK-M11-08_AUTHORITY_RECONCILIATION_REPORT_20260415T002331Z.md
        ├── TASK-M11-08_CORRECTIVE_PREFLIGHT_REPORT_20260414T230240Z.md
        ├── TASK-M11-08_EXECUTION_REPORT_20260414T223857Z.md
        ├── TASK-M11-08_PREFLIGHT_REPORT_20260414T153327Z.md
        ├── TASK-M11-09_CLOSEOUT_REPORT_20260415T014220Z.md
        ├── TASK-M11-09_EXECUTION_REPORT_20260415T013414Z.md
        ├── TASK-M11-09_PREFLIGHT_REPORT_20260415T012710Z.md
        ├── TASK-M11-09_VERIFICATION_REPORT_20260415T013829Z.md
        ├── TASK-M11-10A_CLOSEOUT_REPORT_20260415T024141Z.md
        ├── TASK-M11-10A_EXECUTION_REPORT_20260415T023511Z.md
        ├── TASK-M11-10A_PREFLIGHT_REPORT_20260415T023023Z.md
        ├── TASK-M11-10A_VERIFICATION_REPORT_20260415T023838Z.md
        ├── TASK-M11-10B1_CLOSEOUT_REPORT_20260415T034453Z.md
        ├── TASK-M11-10B1_EXECUTION_REPORT_20260415T033055Z.md
        ├── TASK-M11-10B1_PREFLIGHT_REPORT_20260415T031907Z.md
        ├── TASK-M11-10B1_VERIFICATION_REPORT_20260415T033821Z.md
        ├── TASK-M11-10B2_CLOSEOUT_REPORT_20260415T041344Z.md
        ├── TASK-M11-10B2_EXECUTION_REPORT_20260415T035858Z.md
        ├── TASK-M11-10B2_PREFLIGHT_REPORT_20260415T035305Z.md
        ├── TASK-M11-10B2_VERIFICATION_REPORT_20260415T040840Z.md
        ├── TASK-M11-10B_AUTHORITY_REPORT_20260415T030449Z.md
        ├── TASK-M11-10B_SPLIT_AUTHORITY_REPORT_20260415T031240Z.md
        ├── TASK-M11-10_PREFLIGHT_REPORT_20260415T021006Z.md
        ├── TASK-M11-11_CLOSEOUT_REPORT_20260415T053856Z.md
        ├── TASK-M11-11_EXECUTION_REPORT_20260415T052651Z.md
        ├── TASK-M11-11_PREFLIGHT_REPORT_20260415T045550Z.md
        ├── TASK-M11-11_VERIFICATION_REPORT_20260415T053516Z.md
        ├── TASK-M11-12_CLOSEOUT_REPORT_20260415T065244Z.md
        ├── TASK-M11-12_EXECUTION_REPORT_20260415T061925Z.md
        ├── TASK-M11-12_GATE_CLARIFICATION_REPORT_20260415T060015Z.md
        ├── TASK-M11-12_PATCH_REPORT_20260415T064404Z.md
        ├── TASK-M11-12_PREFLIGHT_REPORT_20260415T054738Z.md
        ├── TASK-M11-12_VERIFICATION_REPORT_20260415T063317Z.md
        ├── TASK-M11_D2_RECON_REPORT_20260414T012649Z.md
        ├── TASK-M7-03_COMPLETION_REPORT.md
        ├── TASK-M7-03_PATCH_HYGIENE_REPORT.md
        ├── TASK-M7-03_PREFLIGHT_REPORT.md
        ├── TASK-M7-03_REVIEW_REPORT.md
        ├── TASK-M7-04_ACCEPTANCE_AND_RFC_CLEANUP_REPORT.md
        ├── TASK-M7-04_BASELINE_HYGIENE_REPORT.md
        ├── TASK-M7-04_EXECUTION_REPORT.md
        ├── TASK-M7-04_PATCH_HYGIENE_REPORT.md
        ├── TASK-M7-04_PREFLIGHT_REPORT_20260411T125044Z.md
        ├── TASK-M7-04_REVIEW_REPORT_20260411T134810Z.md
        ├── TASK-M7-04_review_2026-04-11.md
        ├── TASK-M7-05_CLOSEOUT_REPORT_20260411T232128Z.md
        ├── TASK-M7-05_EXECUTION_REPORT_20260411T230547Z.md
        ├── TASK-M7-05_PREFLIGHT_REPORT_20260411T225856Z.md
        ├── TASK-M7-05_REVIEW_REPORT_20260411T231155Z.md
        ├── TASK-M7-06_CLOSEOUT_REPORT_20260412T005128Z.md
        ├── TASK-M7-06_EXECUTION_REPORT_20260412T004109Z.md
        ├── TASK-M7-06_PREFLIGHT_REPORT_20260411T233238Z.md
        ├── TASK-M7-06_REVIEW_REPORT_20260412T004550Z.md
        ├── TASK-M7-07_BLOCKER_AUDIT_REPORT_20260412T013214Z.md
        ├── TASK-M7-07_CLOSEOUT_REPORT_20260412T040059Z.md
        ├── TASK-M7-07_EXECUTION_REPORT_20260412T012024Z.md
        ├── TASK-M7-07_EXECUTION_REPORT_20260412T012602Z.md
        ├── TASK-M7-07_EXECUTION_REPORT_20260412T031337Z.md
        ├── TASK-M7-07_EXECUTION_REPORT_20260412T032440Z.md
        ├── TASK-M7-07_EXECUTION_REPORT_20260412T033428Z.md
        ├── TASK-M7-07_EXECUTION_REPORT_20260412T034821Z.md
        ├── TASK-M7-07_PREFLIGHT_REPORT_20260412T011415Z.md
        ├── TASK-M7-07_REVIEW_REPORT_20260412T035625Z.md
        ├── TASK-M7-BASELINE_BRANCH_REPORT.md
        ├── TASK-M8-01_CLOSEOUT_REPORT_20260412T050544Z.md
        ├── TASK-M8-01_EXECUTION_REPORT_20260412T045041Z.md
        ├── TASK-M8-01_PREFLIGHT_REPORT_20260412T043020Z.md
        ├── TASK-M8-01_REVIEW_REPORT_20260412T045940Z.md
        ├── TASK-M8-02_CLOSEOUT_REPORT_20260412T060333Z.md
        ├── TASK-M8-02_COMMIT_VERIFICATION_REPORT_20260412T061134Z.md
        ├── TASK-M8-02_EXECUTION_REPORT_20260412T053641Z.md
        ├── TASK-M8-02_PREFLIGHT_REPORT_20260412T052210Z.md
        ├── TASK-M8-02_REVIEW_REPORT_20260412T054519Z.md
        ├── TASK-M8-03_CLOSEOUT_REPORT_20260412T095036Z.md
        ├── TASK-M8-03_COMMIT_VERIFICATION_REPORT_20260412T100130Z.md
        ├── TASK-M8-03_EXECUTION_REPORT_20260412T062822Z.md
        ├── TASK-M8-03_PREFLIGHT_REPORT_20260412T061839Z.md
        ├── TASK-M8-03_REVIEW_REPORT_20260412T094103Z.md
        ├── TASK-M8-04_ADJUDICATION_REPORT_20260412T111215Z.md
        ├── TASK-M8-04_CLOSEOUT_REPORT_20260412T113440Z.md
        ├── TASK-M8-04_COMMIT_VERIFICATION_REPORT_20260412T113834Z.md
        ├── TASK-M8-04_CORRECTIVE_PREFLIGHT_REPORT_20260412T105346Z.md
        ├── TASK-M8-04_EXECUTION_REPORT_20260412T103524Z.md
        ├── TASK-M8-04_IN_DEPTH_VERIFICATION_REPORT.md
        ├── TASK-M8-04_PREFLIGHT_REPORT_20260412T101332Z.md
        ├── TASK-M8-04_REVIEW_REPORT_20260412T103951Z.md
        ├── TASK-M8-05_CLOSEOUT_REPORT_20260412T120456Z.md
        ├── TASK-M8-05_COMMIT_VERIFICATION_REPORT_20260412T121007Z.md
        ├── TASK-M8-05_EXECUTION_REPORT_20260412T115728Z.md
        ├── TASK-M8-05_PREFLIGHT_REPORT_20260412T115058Z.md
        ├── TASK-M8-05_REVIEW_REPORT_20260412T120112Z.md
        ├── TASK-M8-06_CLOSEOUT_REPORT_20260412T145114Z.md
        ├── TASK-M8-06_COMMIT_VERIFICATION_REPORT_20260412T145426Z.md
        ├── TASK-M8-06_EXECUTION_REPORT_20260412T141929Z.md
        ├── TASK-M8-06_PREFLIGHT_REPORT_20260412T135323Z.md
        ├── TASK-M8-06_REVIEW_REPORT_20260412T144536Z.md
        ├── TASK-M8-07_CLOSEOUT_REPORT_20260412T224707Z.md
        ├── TASK-M8-07_COMMIT_VERIFICATION_REPORT_20260412T225608Z.md
        ├── TASK-M8-07_EXECUTION_REPORT_20260412T160750Z.md
        ├── TASK-M8-07_M8-08_RECONCILIATION_REPORT_20260412T160309Z.md
        ├── TASK-M8-07_REVIEW_REPORT_20260412T224256Z.md
        ├── TASK-M8-08_CLOSEOUT_REPORT_20260412T152403Z.md
        ├── TASK-M8-08_COMMIT_VERIFICATION_REPORT_20260412T152849Z.md
        ├── TASK-M8-08_EXECUTION_REPORT_20260412T151649Z.md
        ├── TASK-M8-08_POST_M8-07_NONREGRESSION_REPORT_20260412T230523Z.md
        ├── TASK-M8-08_PREFLIGHT_REPORT_20260412T150803Z.md
        ├── TASK-M8-08_REVIEW_REPORT_20260412T152001Z.md
        ├── TASK-M8-09_CLOSEOUT_REPORT_20260412T232041Z.md
        ├── TASK-M8-09_COMMIT_VERIFICATION_REPORT_20260412T232152Z.md
        ├── TASK-M8-09_EXECUTION_REPORT_20260412T231257Z.md
        ├── TASK-M8-09_PREFLIGHT_REPORT_20260412T230031Z.md
        ├── TASK-M8-09_REVIEW_REPORT_20260412T231632Z.md
        ├── TASK-M8-10_CLOSEOUT_REPORT_20260412T235709Z.md
        ├── TASK-M8-10_COMMIT_VERIFICATION_REPORT_20260412T235810Z.md
        ├── TASK-M8-10_EXECUTION_REPORT_20260412T234003Z.md
        ├── TASK-M8-10_PREFLIGHT_REPORT_20260412T232936Z.md
        ├── TASK-M8-10_REVIEW_REPORT_20260412T235341Z.md
        ├── TASK-M8-11_CLOSEOUT_REPORT_20260413T003451Z.md
        ├── TASK-M8-11_COMMIT_VERIFICATION_REPORT_20260413T003542Z.md
        ├── TASK-M8-11_EXECUTION_REPORT_20260413T001110Z.md
        ├── TASK-M8-11_EXECUTION_REPORT_20260413T002601Z.md
        ├── TASK-M8-11_FORMAT_SCOPE_RECONCILIATION_REPORT_20260413T001700Z.md
        ├── TASK-M8-11_PREFLIGHT_REPORT_20260413T000622Z.md
        ├── TASK-M8-11_REVIEW_REPORT_20260413T002956Z.md
        ├── TASK-M9-01_EXECUTION_REPORT_20260413T050804Z.md
        ├── TASK-M9-01_PREFLIGHT_REPORT_20260413T045856Z.md
        ├── TASK-M9-02_CLOSEOUT_REPORT_20260413T060834Z.md
        ├── TASK-M9-02_COMMIT_VERIFICATION_REPORT_20260413T063215Z.md
        ├── TASK-M9-02_EXECUTION_REPORT_20260413T060834Z.md
        ├── TASK-M9-02_PREFLIGHT_REPORT_20260413T054655Z.md
        ├── TASK-M9-03_CLOSEOUT_REPORT_20260413T070716Z.md
        ├── TASK-M9-03_COMMIT_VERIFICATION_REPORT_20260413T072303Z.md
        ├── TASK-M9-03_EXECUTION_REPORT_20260413T064215Z.md
        ├── TASK-M9-03_PREFLIGHT_REPORT_20260413T061901Z.md
        ├── TASK-M9-04_CLOSEOUT_REPORT_20260413T075000Z.md
        ├── TASK-M9-04_COMMIT_VERIFICATION_REPORT_20260413T075907Z.md
        ├── TASK-M9-04_EXECUTION_REPORT_20260413T075000Z.md
        ├── TASK-M9-04_PREFLIGHT_REPORT_20260413T073825Z.md
        ├── TASK-M9-05_EXECUTION_REPORT_20260413T083402Z.md
        ├── TASK-M9-05_PREFLIGHT_REPORT_20260413T080928Z.md
        ├── TASK-M9-06_EXECUTION_REPORT_20260413T085936Z.md
        ├── TASK-M9-06_PREFLIGHT_REPORT_20260413T084627Z.md
        ├── TASK-M9-07_CLOSEOUT_REPORT_20260413T095809Z.md
        ├── TASK-M9-07_COMMIT_VERIFICATION_REPORT_20260413T115639Z.md
        ├── TASK-M9-07_EXECUTION_REPORT_20260413T095809Z.md
        ├── TASK-M9-07_PREFLIGHT_REPORT_20260413T093641Z.md
        ├── TASK-M9-08_CLOSEOUT_REPORT_20260413T124507Z.md
        ├── TASK-M9-08_COMMIT_VERIFICATION_REPORT_20260413T130029Z.md
        ├── TASK-M9-08_EXECUTION_REPORT_20260413T124507Z.md
        ├── TASK-M9-08_PREFLIGHT_REPORT_20260413T123418Z.md
        ├── TASK-M9-09_CLOSEOUT_REPORT_20260413T132046Z.md
        ├── TASK-M9-09_COMMIT_VERIFICATION_REPORT_20260413T132904Z.md
        ├── TASK-M9-09_EXECUTION_REPORT_20260413T132046Z.md
        ├── TASK-M9-09_PREFLIGHT_REPORT_20260413T130702Z.md
        ├── TASK-M9-10_EXECUTION_REPORT_20260413T140203Z.md
        ├── TASK-M9-10_PREFLIGHT_REPORT_20260413T134257Z.md
        ├── TASK-M9-11_COMMIT_VERIFICATION_REPORT_20260413T150533Z.md
        ├── TASK-M9-11_EXECUTION_REPORT_20260413T145013Z.md
        ├── TASK-M9-11_PREFLIGHT_REPORT_20260413T142100Z.md
        ├── TASK-M9-11_REMEDIATION_CLOSEOUT_REPORT_20260413T144105Z.md
        ├── TASK-M9-11_REMEDIATION_EXECUTION_REPORT_20260413T144105Z.md
        ├── TASK-M9-12_CLOSEOUT_REPORT_20260413T153943Z.md
        ├── TASK-M9-12_COMMIT_VERIFICATION_REPORT_20260413T165501Z.md
        ├── TASK-M9-12_EXECUTION_REPORT_20260413T152113Z.md
        ├── TASK-M9-12_EXECUTION_RERUN_REPORT_20260413T153943Z.md
        ├── TASK-M9-12_FINAL_EXECUTION_RERUN_REPORT_20260413T164651Z.md
        ├── TASK-M9-12_FORMAT_REMEDIATION_CLOSEOUT_REPORT_20260413T153001Z.md
        ├── TASK-M9-12_FORMAT_REMEDIATION_EXECUTION_REPORT_20260413T153001Z.md
        ├── TASK-M9-12_OPENAI_SMOKE_TEST_REPORT_20260413T161005Z.md
        ├── TASK-M9-12_PREFLIGHT_REPORT_20260413T151224Z.md
        ├── TASK_CHILD_MAX_TOKENS_D1_INTERIM_CLOSEOUT_REPORT_20260413T163701Z.md
        ├── TASK_CHILD_MAX_TOKENS_D1_INTERIM_EXECUTION_REPORT_20260413T163701Z.md
        ├── TUI_THINKING_BLOCK_TEST_BLOCKER_RESOLUTION_REPORT_20260412T033941Z.md
        ├── WORKFLOW_AUTOMATION_ASSESSMENT.md
        ├── WORKFLOW_AUTOMATION_ASSESSMENT_V2.md
        └── WORKSPACE_RUSTFMT_BASELINE_REPORT_20260412T014523Z.md

6 directories, 350 files
```
