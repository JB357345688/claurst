# Local Multi-Agent Orchestration Plan v2

ENVIRONMENT_FINDINGS

- Host is `Ubuntu 24.04.4 LTS` on `Linux 6.8.0-101-generic x86_64`.
- Active and login shell are both `/bin/bash`; `zsh` and `pwsh` are not installed.
- AI CLIs detected: `codex` at `/home/jordi/.npm-global/bin/codex` version `0.120.0`; `claude` at `/home/jordi/.npm-global/bin/claude` version `2.1.96`; `gemini` at `/home/jordi/.npm-global/bin/gemini` version `0.37.0`.
- Runtime tools confirmed: `bash 5.2.21`, `python3 3.12.3` at `/usr/bin/python3` with no `python` shim, `jq 1.7`, `sed 4.9`, `awk 5.2.1`, `timeout 9.4`, `tee 9.4`, `mktemp 9.4`, `flock 2.39.3`.
- Relevant local CLI surfaces re-verified for this machine:
  - `codex exec`: stdin prompt input is supported only when no prompt is supplied or when `-` is supplied as the prompt placeholder; supports `--sandbox`, `--json`, `--output-schema`, `-o`, `--ephemeral`, and `-c/--config`.
  - `codex review`: exists, but its local CLI help does not expose explicit sandbox or approval controls.
  - Top-level `codex`: exposes `--search`; because local Codex also indicates web search can be enabled or disabled via config, deterministic local orchestration must set search mode explicitly instead of inheriting defaults.
  - `claude`: supports `-p`, `--output-format json`, `--permission-mode`, `--effort`, `--bare`, `--no-session-persistence`.
  - `gemini`: supports `--prompt`, `--output-format json`, `--approval-mode`, `--sandbox`, `--model`.

WORKSPACE_FINDINGS

- Workspace root is `/home/jordi/claurst` on branch `feature/provider-resolution-seam`.
- The worktree is already dirty: `.gitignore` is modified and there are many untracked docs/report artifacts plus `src-rust/target/`; the controller must treat run-start state as a baseline, not assume a clean repo.
- Top-level workflow files already exist: `AGENTS.md`, `CLAUDE.md`, `GEMINI.md`, `README.md`, `docs/Current`, `docs/Orchestrator_planning`, `docs/archive/reports`, `docs/archive/provider_orchestrator`, `src-rust`.
- No existing shell automation layer exists here; `rg --files -g '*.sh' -g '*.bash' -g '*.zsh' -g '*.ps1'` returned no scripts.
- Repo conventions are strong and should be reused: durable markdown reports belong in `docs/archive/reports`; prompt packs/RFCs already live under `docs/archive/provider_orchestrator`; planning docs live in `docs/Current` and `docs/Orchestrator_planning`.
- `.gitignore` is minimal and does not yet ignore an orchestration runtime directory; that is implementation-run work, not part of this planning document.
- `.codex` and `src-rust/.codex` are zero-byte marker files, not config directories; `.claude/settings.local.json` exists and currently grants only a few local bash reads plus one skill.

ARCHITECTURE_DECISION

- Use `Bash 5.2` as the controller runtime. Do not use POSIX `sh`, PowerShell, or a Python-led controller.
- Reason: this machine is Linux-first, Bash is the active/login shell, `flock`/`timeout`/`mktemp`/`jq` are present, and a pure Bash state machine keeps bootstrap simple, auditable, and local. `python3` remains optional for future helpers, not a hard dependency.
- Canonical runtime root should be repo-local `.orchestrator/` once ignored by git; first bootstrap runs should use `/tmp/claurst-orchestrator/claurst/` with the exact same structure to avoid polluting this already-dirty checkout.
- Durable human-facing summaries should always be emitted as `.md` files under `docs/archive/reports/` to match repo authority.
- Machine state and handoff packets should be `JSON`; prompt artifacts should be `Markdown`; raw agent streams should be `JSONL` when supported and plain stdout/stderr logs otherwise; repo diffs should be plain `.patch` plus `.txt` stats.
- Default tool choice on this machine should be `codex` for both roles.
- Reviewer default must be defined portably as explicit model plus explicit reasoning override, not by assuming pre-created profiles:
  - model: `gpt-5.4`
  - reasoning effort override: `high`
  - sandbox: `read-only`
  - approval policy: `never`
  - web search: `disabled`
  - session persistence: disabled via `--ephemeral`
- Drafter default must also be portable and explicit:
  - model: `gpt-5.4-mini`
  - reasoning effort override: `medium`
  - sandbox: `workspace-write`
  - approval policy: `never`
  - web search: `disabled`
  - session persistence: disabled via `--ephemeral`
- Named Codex profiles are optional convenience only. If present, they may layer additional defaults, but the controller must still set the deterministic fields explicitly on each invocation.
- For deterministic local orchestration, use `codex exec` for both reviewer and drafter stages. Do not use `codex review` as the default automated reviewer primitive because its local CLI surface does not expose explicit sandbox or approval controls.
- Fallback order should be `claude` first, then `gemini`. Both are secondary adapters and must also set approval/search behavior explicitly rather than inheriting ambient settings.

STATE_AND_HANDOFF_SPEC

- Run ID format: `YYYYMMDDTHHMMSSZ_<branch_slug>_<head7>`. Cycle IDs: `0001`, `0002`, and so on.
- At run start the controller writes `baseline_manifest.json` and `baseline_repo_state.json`; every later review is against the run baseline, not against global repo cleanliness.
- Each stage has exactly two controller-built inputs: `*_packet.json` for machines and `*_prompt.md` for humans/agents. The prompt is a rendered view of the packet, never the source of truth.
- Every rendered prompt starts with a deterministic header containing `run_id`, `cycle`, `stage`, `workspace`, `git_head`, `provider`, `model`, `sandbox`, `approval`, `reasoning`, `web_search`, referenced input files, and SHA256 digests of those inputs.
- Reviewer stage 0 produces `reviewer_blueprint.json`; each later reviewer pass produces `review_decision.json`.
- The controller converts `reviewer_blueprint.json` or `review_decision.json` into `drafter_packet.json`; the drafter never receives free-form prior chat only.
- The drafter returns `draft_result.json`; the controller independently captures actual repo deltas into `worktree_snapshot.json`, `worktree_diff.patch`, and `validation_results.json` before sending anything back to the reviewer.
- Reviewer input is always normalized as: objective + baseline scope + actual diff + actual validation results + drafter self-report. Reviewer critique is therefore grounded in real artifacts, not the drafter’s narrative.
- One repo-level lock only: `locks/repo.lock` acquired via `flock -n`. The design is intentionally single-controller and sequential for determinism.
- All writes are atomic: write to `mktemp` under the target directory, validate JSON with `jq -e`, then `mv` into place.
- `resolved_config.json` must record the effective stage settings, not just requested settings. At minimum record:
  - provider adapter
  - binary path
  - model
  - reasoning effort
  - sandbox
  - approval policy
  - web search mode
  - ephemeral or persistence mode
  - whether an optional profile overlay was applied
- Loop terminates when reviewer returns `approve`, when `ORCH_MAX_CYCLES` is reached, when the diff hash stops changing across two draft attempts, when a mandatory human checkpoint is triggered, or when a required tool is unavailable.

CONTROLLER_FLOW

1. `detect-env`: discover CLI tools, versions, shells, helper binaries; write `tool_inventory.json`.
2. `acquire-lock`: take `locks/repo.lock`; fail fast if another controller owns the repo.
3. `init-run`: create run directories, immutable `run_manifest.json`, and mutable `state/run_status.json`.
4. `snapshot-baseline`: capture branch, HEAD, dirty state, changed/untracked files, and a baseline manifest excluding `.git`, runtime dir, and configured noise such as `src-rust/target`.
5. `resolve-config`: choose reviewer/drafter adapters from installed tools; validate local CLI truth for stdin handling, sandbox behavior, approval behavior, and web search behavior; write `resolved_config.json`.
6. `render-blueprint`: build `blueprint/reviewer_packet.json` and `blueprint/reviewer_prompt.md`.
7. `run-blueprint`: invoke reviewer in deterministic read-only mode; validate and store `blueprint/reviewer_blueprint.json`.
8. If mode is `dry-run`, stop after writing prompts and `planned_invocations.json`. If mode is `plan-only`, stop after stage 7.
9. For each cycle: build `cycles/<id>/drafter_packet.json` and `drafter_prompt.md` from the blueprint plus latest review decision.
10. Enforce human write gate if `ORCH_REQUIRE_HUMAN_ON_FIRST_WRITE=1` and this is the first mutating drafter pass.
11. Invoke drafter; store raw logs plus `draft_result.json`.
12. Snapshot actual repo delta vs baseline; write `worktree_snapshot.json`, `worktree_diff.patch`, `worktree_diff_stat.txt`, and `changed_files.txt`.
13. Run deterministic validation commands from the reviewer blueprint; write `validation_results.json` and raw logs.
14. Build `cycles/<id>/reviewer_packet.json` and `reviewer_prompt.md` from objective, diff, validations, and draft result.
15. Invoke reviewer in deterministic read-only mode; validate and store `review_decision.json`.
16. If reviewer says `approve`, render final report and exit `success`. If reviewer says `revise`, continue to next cycle. If reviewer says `block` or `need_human`, set status `paused` and stop.
17. `finalize`: always write `state/run_status.json` and a human-readable summary report in `docs/archive/reports/`.

FAILURE_AND_RECOVERY_RULES

- If the requested provider CLI is missing, auto-fallback to the next installed adapter; if no adapter is available for a required role, fail before stage 6.
- If a stage command exits nonzero or times out, retry up to `ORCH_STAGE_RETRIES` with the exact same prompt and packet; store each attempt under `attempt-001`, `attempt-002`, and never overwrite prior logs.
- If an agent returns malformed JSON or violates the output contract, treat the stage as failed; do not auto-rewrite or “repair” the JSON in-controller.
- If validation commands fail and `ORCH_STOP_ON_VALIDATION_FAIL=1`, stop before reviewer and mark the run `paused`; otherwise include the failures in reviewer input and let the reviewer decide.
- If the repo is dirty at start and `ORCH_ALLOW_DIRTY=0`, refuse active draft/full-loop modes; allow only `dry-run`, `plan-only`, or `review-only`.
- If the repo is dirty at start and `ORCH_ALLOW_DIRTY=1`, the controller must still review only `cycle_delta` against `baseline_manifest.json`, never the entire worktree.
- If two consecutive draft cycles produce the same diff hash, stop as stagnation rather than looping forever.
- On `INT` or `TERM`, trap the signal, mark `run_status.json` as `interrupted`, record the active stage, and release the lock.
- Resume works only from the last complete JSON checkpoint in `state/run_status.json`; there is no hidden session resume.
- If the selected adapter cannot make sandbox mode, approval behavior, or web search mode explicit for the intended stage, fail before launch rather than inheriting ambient config.
- For Codex specifically:
  - automated reviewer runs must be `read-only`, `approval_policy="never"`, and `web_search="disabled"`
  - automated drafter runs must set sandbox, approval, and web search explicitly
  - do not rely on default profile contents, global config defaults, or interactive approval prompts for automated runs
- Narrow bootstrap exception:
  - if a one-time manual reconnaissance pass is needed before controller implementation, it may use `codex exec` with `approval_policy="on-request"` and `--sandbox read-only`
  - this exception is for harmless environment inspection only
  - it must keep `web_search="disabled"`
  - it must not permit workspace writes or dangerous sandbox bypass

PROPOSED_DIRECTORY_TREE

- `scripts/orchestrator/orch-loop.sh`
- `scripts/orchestrator/lib/common.sh`
- `scripts/orchestrator/lib/detect.sh`
- `scripts/orchestrator/lib/lock.sh`
- `scripts/orchestrator/lib/state.sh`
- `scripts/orchestrator/lib/render.sh`
- `scripts/orchestrator/lib/git_snapshot.sh`
- `scripts/orchestrator/lib/validate.sh`
- `scripts/orchestrator/lib/provider_codex.sh`
- `scripts/orchestrator/lib/provider_claude.sh`
- `scripts/orchestrator/lib/provider_gemini.sh`
- `scripts/orchestrator/prompts/reviewer_blueprint.md`
- `scripts/orchestrator/prompts/reviewer_critique.md`
- `scripts/orchestrator/prompts/drafter_execute.md`
- `scripts/orchestrator/schemas/reviewer_blueprint.schema.json`
- `scripts/orchestrator/schemas/review_decision.schema.json`
- `scripts/orchestrator/schemas/draft_result.schema.json`
- `scripts/orchestrator/schemas/run_status.schema.json`
- `.orchestrator/locks/repo.lock`
- `.orchestrator/runs/<run_id>/run_manifest.json`
- `.orchestrator/runs/<run_id>/resolved_config.json`
- `.orchestrator/runs/<run_id>/tool_inventory.json`
- `.orchestrator/runs/<run_id>/baseline/baseline_repo_state.json`
- `.orchestrator/runs/<run_id>/baseline/baseline_manifest.json`
- `.orchestrator/runs/<run_id>/blueprint/reviewer_packet.json`
- `.orchestrator/runs/<run_id>/blueprint/reviewer_prompt.md`
- `.orchestrator/runs/<run_id>/blueprint/reviewer_blueprint.json`
- `.orchestrator/runs/<run_id>/cycles/0001/drafter_packet.json`
- `.orchestrator/runs/<run_id>/cycles/0001/drafter_prompt.md`
- `.orchestrator/runs/<run_id>/cycles/0001/draft_result.json`
- `.orchestrator/runs/<run_id>/cycles/0001/worktree_snapshot.json`
- `.orchestrator/runs/<run_id>/cycles/0001/worktree_diff.patch`
- `.orchestrator/runs/<run_id>/cycles/0001/worktree_diff_stat.txt`
- `.orchestrator/runs/<run_id>/cycles/0001/validation_results.json`
- `.orchestrator/runs/<run_id>/cycles/0001/reviewer_packet.json`
- `.orchestrator/runs/<run_id>/cycles/0001/reviewer_prompt.md`
- `.orchestrator/runs/<run_id>/cycles/0001/review_decision.json`
- `.orchestrator/runs/<run_id>/logs/controller.log`
- `.orchestrator/runs/<run_id>/logs/<stage>.stdout.log`
- `.orchestrator/runs/<run_id>/logs/<stage>.stderr.log`
- `.orchestrator/runs/<run_id>/logs/<stage>.events.jsonl`
- `.orchestrator/runs/<run_id>/state/run_status.json`
- `.orchestrator/runs/<run_id>/state/planned_invocations.json`
- `docs/archive/reports/ORCH_<run_id>_LOOP_REPORT.md`

PROPOSED_FILE_CONTRACTS

- `run_manifest.json`: immutable metadata; keys `run_id`, `workspace_root`, `branch`, `head`, `started_at`, `mode`, `controller_script`, `baseline_dirty`, `reports_dir`.
- `resolved_config.json`: resolved adapter/config values; keys `reviewer`, `drafter`, `max_cycles`, `allow_dirty`, `retry_limit`, `timeout_sec`, `validation_mode`, `excluded_paths`; record env var names used, never secret values.
- `tool_inventory.json`: one object per discovered tool; keys `name`, `path`, `version`, `installed`, `capabilities`.
- `baseline_repo_state.json`: run-start repo snapshot; keys `status_short`, `tracked_modified`, `untracked`, `head`, `branch`.
- `baseline_manifest.json`: file-hash baseline for scope enforcement; each entry stores `path`, `sha256`, `size`, `tracked`.
- `reviewer_packet.json`: exact reviewer input; keys `role`, `objective`, `mode`, `baseline_paths`, `diff_artifacts`, `validation_artifacts`, `prior_decision`, `required_output_schema`.
- `reviewer_prompt.md`: rendered reviewer packet; header with digests and explicit instruction to emit JSON only matching the named schema.
- `reviewer_blueprint.json`: reviewer stage 0 output; keys `status`, `objective`, `allowed_paths`, `forbidden_paths`, `acceptance_criteria`, `validation_commands`, `draft_plan`, `review_focus`, `stop_conditions`.
- `drafter_packet.json`: exact drafter handoff; keys `objective`, `allowed_paths`, `required_changes`, `forbidden_changes`, `validation_commands`, `review_findings`, `output_contract`.
- `drafter_prompt.md`: rendered drafter packet with the same digest header and no hidden context.
- `draft_result.json`: drafter output; keys `status`, `summary`, `files_touched`, `commands_run`, `validations_requested`, `risks`, `needs_human`.
- `worktree_snapshot.json`: controller-owned truth after a draft; keys `changed_files_vs_baseline`, `untracked_vs_baseline`, `diff_hash`, `patch_path`, `diff_stat_path`.
- `validation_results.json`: controller-run command results; array of `{cmd, cwd, exit_code, stdout_log, stderr_log, duration_sec}` plus `overall_status`.
- `review_decision.json`: reviewer critique output; keys `decision`, `summary`, `blocking_findings`, `required_edits`, `validation_findings`, `next_action`, `terminate`.
- `run_status.json`: mutable checkpoint; keys `run_id`, `current_stage`, `current_cycle`, `last_completed_stage`, `attempt`, `final_status`, `resume_hint`.
- `*.events.jsonl`, `*.stdout.log`, `*.stderr.log`: raw audit artifacts only; never parsed as truth unless explicitly declared by the adapter.
- `docs/archive/reports/ORCH_<run_id>_LOOP_REPORT.md`: durable markdown report summarizing environment, config, cycle history, validation summary, final verdict, and resume/failure reason if not successful.

PROPOSED_AGENT_INVOCATION_PATTERN

- Externalize these keys: `ORCH_WORKSPACE`, `ORCH_STATE_ROOT`, `ORCH_REPORTS_DIR`, `ORCH_MODE`, `ORCH_MAX_CYCLES`, `ORCH_ALLOW_DIRTY`, `ORCH_STAGE_RETRIES`, `ORCH_REQUIRE_HUMAN_ON_FIRST_WRITE`, `ORCH_STOP_ON_VALIDATION_FAIL`, `ORCH_REVIEWER_PROVIDER`, `ORCH_REVIEWER_BIN`, `ORCH_REVIEWER_MODEL`, `ORCH_REVIEWER_PROFILE`, `ORCH_REVIEWER_REASONING_EFFORT`, `ORCH_REVIEWER_SANDBOX`, `ORCH_REVIEWER_APPROVAL`, `ORCH_REVIEWER_WEB_SEARCH`, `ORCH_REVIEWER_TIMEOUT_SEC`, and the matching `ORCH_DRAFTER_*` set.
- Profiles are optional convenience only:
  - if `ORCH_REVIEWER_PROFILE` or `ORCH_DRAFTER_PROFILE` is set, include it
  - otherwise omit `--profile`
  - regardless, still pass explicit config overrides for reasoning effort, approval policy, and web search mode
- Codex reviewer pattern:
  - `/usr/bin/timeout "$ORCH_REVIEWER_TIMEOUT_SEC" "$ORCH_REVIEWER_BIN" exec --cd "$ORCH_WORKSPACE" --model "$ORCH_REVIEWER_MODEL" ${ORCH_REVIEWER_PROFILE:+--profile "$ORCH_REVIEWER_PROFILE"} --sandbox read-only --ephemeral --json --output-schema "$SCHEMA_REVIEW" -o "$LAST_JSON" -c approval_policy="never" -c model_reasoning_effort="$ORCH_REVIEWER_REASONING_EFFORT" -c web_search="disabled" - < "$PROMPT_MD" > "$EVENTS_JSONL" 2> "$STDERR_LOG"`
- Codex drafter pattern:
  - `/usr/bin/timeout "$ORCH_DRAFTER_TIMEOUT_SEC" "$ORCH_DRAFTER_BIN" exec --cd "$ORCH_WORKSPACE" --model "$ORCH_DRAFTER_MODEL" ${ORCH_DRAFTER_PROFILE:+--profile "$ORCH_DRAFTER_PROFILE"} --sandbox "$ORCH_DRAFTER_SANDBOX" --ephemeral --json --output-schema "$SCHEMA_DRAFT" -o "$LAST_JSON" -c approval_policy="never" -c model_reasoning_effort="$ORCH_DRAFTER_REASONING_EFFORT" -c web_search="disabled" - < "$PROMPT_MD" > "$EVENTS_JSONL" 2> "$STDERR_LOG"`
- Codex stdin rule:
  - any `codex exec` example that feeds the prompt from stdin must pass `-` as the prompt placeholder
  - do not rely on a bare redirect without `-`
- Codex reviewer rule:
  - do not use `codex review` for the automated reviewer stage
  - make reviewer approval, sandbox, and web search behavior explicit on every invocation
- Codex approval rule:
  - do not use `--full-auto` or `--dangerously-bypass-approvals-and-sandbox` for the normal automated loop
  - first-write authorization belongs to the controller’s human checkpoint, not to interactive approvals inside `codex exec`
- Codex search rule:
  - do not pass `--search` for deterministic local runs
  - set `-c web_search="disabled"` explicitly
  - only a future explicit run mode may opt into `live` or `cached`
- Claude fallback pattern:
  - `PROMPT="$(cat "$PROMPT_MD")"; /usr/bin/timeout "$TIMEOUT" "$BIN" --bare --no-session-persistence --add-dir "$ORCH_WORKSPACE" -p "$PROMPT" --output-format json --model "$MODEL" --permission-mode "$PERMISSION_MODE" --effort "$EFFORT" > "$FINAL_JSON" 2> "$STDERR_LOG"`
  - treat `--permission-mode plan` or the closest explicit mode as the reviewer equivalent; do not inherit ambient permission behavior
- Gemini fallback pattern:
  - `PROMPT="$(cat "$PROMPT_MD")"; /usr/bin/timeout "$TIMEOUT" "$BIN" --prompt "$PROMPT" --output-format json --model "$MODEL" --approval-mode "$APPROVAL_MODE" $GEMINI_SANDBOX_FLAG > "$FINAL_JSON" 2> "$STDERR_LOG"`
  - use explicit `--approval-mode plan` for review-style runs when available; do not inherit the Gemini default approval mode
- Gemini reasoning should be treated as provider-default because this installed CLI does not expose an effort flag.
- Mode handling should be adapter-independent: `dry-run` renders packets/prompts and `planned_invocations.json` only; `plan-only` runs reviewer blueprint only; `review-only` builds reviewer packet from current baseline/diff and skips drafter; `draft-only` requires an existing blueprint/decision packet and skips reviewer; `full-loop` runs blueprint plus draft/review cycles.

BOOTSTRAP_PLAN

- First implementation run should target `/tmp/claurst-orchestrator/claurst/`, not repo-local `.orchestrator/`, because this checkout is already dirty and `.gitignore` does not yet ignore runtime artifacts.
- First bootstrap mode should be `plan-only` with the Codex reviewer only.
- Default bootstrap reviewer behavior must be deterministic and non-interactive:
  - `codex exec`
  - `--sandbox read-only`
  - `-c approval_policy="never"`
  - `-c model_reasoning_effort="high"`
  - `-c web_search="disabled"`
  - `--ephemeral`
- No write-capable drafter invocation, no commit behavior, and no repo mutation beyond future controller-script creation itself.
- The first active write-capable run should still default to `ORCH_REQUIRE_HUMAN_ON_FIRST_WRITE=1`, `ORCH_MAX_CYCLES=1`, and `ORCH_ALLOW_DIRTY=0`.
- Only after the bootstrap run proves stable should the implementation add a repo-local `.orchestrator/` path and a matching `.gitignore` rule.
- Manual bootstrap scout exception:
  - only if the deterministic bootstrap reviewer cannot complete harmless environment inspection
  - use `codex exec` with `--sandbox read-only`, `-c approval_policy="on-request"`, and `-c web_search="disabled"`
  - use it strictly for environment inspection, not drafting or repo writes
- `claude` adapter should be added after the `codex` path is stable; `gemini` should be added last, after explicit confirmation that extension/session behavior can be isolated enough for deterministic use.
- The bootstrap report file should be `docs/archive/reports/ORCH_<run_id>_BOOTSTRAP_REPORT.md`.

IMPLEMENTATION_STEPS_FOR_NEXT_RUN

1. Create `scripts/orchestrator/` with `orch-loop.sh`, core Bash libs, prompt templates, and JSON schemas.
2. Implement environment detection, lock acquisition, run manifest creation, baseline snapshotting, and `planned_invocations.json`.
3. Implement the Codex adapter first, using `codex exec` for both reviewer and drafter stages.
4. Build explicit config handling before stage execution:
   - stdin prompt placeholder `-`
   - explicit `approval_policy`
   - explicit `model_reasoning_effort`
   - explicit `web_search`
   - optional `--profile`, never required
5. Implement prompt rendering and JSON contract validation with `jq -e`; reject malformed agent outputs immediately.
6. Implement baseline-vs-current repo delta capture so the controller can operate safely even when the repo starts dirty.
7. Implement reviewer blueprint mode and `plan-only` end-to-end before adding any mutating draft path.
8. Implement the draft path, validation runner, and review cycle loop with `ORCH_REQUIRE_HUMAN_ON_FIRST_WRITE=1` hard-enabled.
9. Add final markdown report generation under `docs/archive/reports/`.
10. After the Codex path is proven, add `.orchestrator/` ignore rules and switch canonical runtime root from `/tmp/...` to repo-local.
11. Then add `claude` adapter, then `gemini` adapter, without changing the controller state model or file contracts.

TERMINAL_SUMMARY_REQUIREMENTS

- Source file chosen: print the exact source plan path used for the amendment.
- New v2 file path: print the exact sibling `_v2.md` file path.
- Sections changed: list the sections amended in the v2 document.
- Key amendments applied: summarize the Codex stdin correction, explicit reviewer approval and sandbox behavior, portable model plus reasoning configuration, default web-search disablement, and invocation cleanup.
- Unresolved uncertainties: list any local CLI or model-catalog details that should be re-checked before the later controller implementation run.
