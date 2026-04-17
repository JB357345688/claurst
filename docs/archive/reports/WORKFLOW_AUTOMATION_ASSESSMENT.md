# Workflow Assessment and Automation Proposal

Based on an extensive review of the authority documents (`AGENTS.md`, `docs/Current/MPWO_WORK_ORDER_PACK.md`) and the real-world execution artifacts from `TASK-M8-04`, here is a workflow assessment and automation proposal tailored specifically to your repository.

## 1. Evidence Reviewed
- **Authority Documents:** `AGENTS.md`, `docs/Current/MPWO_WORK_ORDER_PACK.md`.
- **Workflow Artifacts (TASK-M8-04):**
  - `...PREFLIGHT_REPORT...`
  - `...EXECUTION_REPORT...`
  - `...REVIEW_REPORT...`
  - `...CORRECTIVE_PREFLIGHT_REPORT...`
  - `...ADJUDICATION_REPORT...`
  - `...CLOSEOUT_REPORT...`
  - `...COMMIT_VERIFICATION_REPORT...`

## 2. Reconstructed Current Workflow
Your current workflow enforces an incredibly strict, highly disciplined process, but does so entirely through conversational markdown and manual handoffs:
1. **Preflight:** Codex reads the MPWO, checks `git status`/`diff`, verifies preconditions, and writes a Preflight Report.
2. **Execution:** Codex writes the code, runs the cargo validation commands, and writes an Execution Report.
3. **Manual Handoff:** You copy the Execution Report and git diff, paste them into ChatGPT/Claude WebUI.
4. **Review (WebUI):** The orchestrator reviews the execution against the MPWO rules, generating a Review Report.
5. **Corrective Loop:** If the review fails (as it did for M8-04 regarding the dummy `AnthropicClient`), a Corrective Preflight and Adjudication Report are generated to decide if the failure is actually in scope, followed by fixes if needed.
6. **Closeout:** Codex verifies the accepted state and writes a Closeout Report.
7. **Commit Verification:** Codex stages, commits (or verifies the commit), and writes a Commit Verification Report.

## 3. Friction Points and Inefficiencies
- **The "Report Explosion":** Generating 5–7 verbose markdown reports per ticket wastes massive amounts of token bandwidth and execution time on boilerplate (timestamps, branch names, repeating the ticket goal).
- **Manual Orchestration Bottleneck:** Copy-pasting raw diffs and markdown reports from the local terminal to a WebUI breaks flow state and is highly prone to context truncation or missed files.
- **AI doing deterministic work:** Codex is spending expensive inference tokens running `git status`, `git branch`, and checking if files exist in the preflight and commit verification stages. These are deterministic bash operations.
- **Redundant Verification:** Closeout and Commit Verification are functionally the same step but split into two AI prompts.

## 4. Automation Opportunities

**Quick Wins:**
- Script the Preflight step. `git diff`, `git branch`, and `cargo check` should be gathered by a bash script and fed *into* the prompt, rather than asking the AI to gather them.
- Merge Closeout and Commit Verification into a single human-gated bash script.

**Medium-Effort Improvements:**
- Shift handoffs from Markdown to JSON sidecars (`ticket_state.json`) so the Orchestrator doesn't have to parse conversational text.
- Use a CLI wrapper to hit the Gemini API for the Review pass automatically, printing the pass/fail to your terminal.

**Later-Stage Improvements:**
- An end-to-end local task runner that pipelines the Execution -> Review -> Corrective loop, only halting to ask you "Review passed. Commit? [y/N]".

## 5. Recommended Target Workflow
1. **Automated Ticket Bundle (Local Script):** A script parses `MPWO_WORK_ORDER_PACK.md` for the current ticket, grabs `git status`, and outputs an `active_ticket.json`.
2. **Execution (Codex):** You prompt Codex: *"Execute the ticket in `active_ticket.json`."* Codex modifies the code, runs validation, and outputs an `execution_summary.json`.
3. **Automated Review (Gemini via CLI):** A script pipes `git diff` + `active_ticket.json` + `execution_summary.json` to Gemini. Gemini strictly validates scope and outputs a `review.json` with a PASS/FAIL verdict.
4. **Human Gate:** The script pauses. You review Gemini's verdict and the diff. You press `y`.
5. **Automated Closeout (Local Script):** The script runs `git add .`, `git commit -m "TASK-M8-XX ..."`, and updates the workspace state.

## 6. Task Partitioning by Agent
- **Codex (Executor):** File modification, compilation, and terminal execution. It should *only* write code and run `cargo`. It should not write reports.
- **Gemini (Reviewer / Orchestrator):** Strict compliance checking. Gemini excels at high-context reasoning. Feed it the diff and the MPWO rules, and have it enforce the `AGENTS.md` scope discipline.
- **Human (Gatekeeper):** Final arbiter of ambiguous scope (e.g., the Adjudication step in M8-04) and the final commit approver.

## 7. Suggested Artifacts and Machine-Readable Contracts
Stop using Markdown for machine-to-machine handoffs. Use these sidecars in a `.work_order/` hidden directory:
- **`manifest.json`**: Extracted ticket constraints, expected files, and validation commands.
- **`execution.json`**: Codex's output (Files touched, `cargo check` exit code, deviations).
- **`review.json`**: Gemini's output (`{"verdict": "PASS|FAIL", "violations": [], "suggested_fix": ""}`).

## 8. Concrete Automation Plan
- **Phase 1 (Today):** Stop making Codex write Preflight and Closeout reports. Write a 20-line bash script that dumps `git status`, `git diff`, and the ticket text into your clipboard.
- **Phase 2 (Next Week):** Implement `scripts/review_trigger.py` to send the local diff directly to Gemini via API, completely eliminating the ChatGPT WebUI copy-paste loop.
- **Phase 3 (Polished):** Move to the full JSON sidecar flow.

## 9. Candidate CLI Tooling Layout

**1. `scripts/bundle_ticket.py`**
- **Purpose:** Replaces the AI Preflight pass.
- **Inputs:** Ticket ID (e.g., `TASK-M8-05`).
- **Outputs:** Generates `.work_order/manifest.json` by parsing the MPWO and gathering repo state.
- **Language:** Python.

**2. `scripts/review_diff.py`**
- **Purpose:** Replaces the manual WebUI review pass.
- **Inputs:** Reads `.work_order/manifest.json` and `git diff`. Sends them to the Gemini API.
- **Outputs:** Prints a strict PASS/FAIL to the terminal and saves `.work_order/review.json`.
- **Language:** Python.

**3. `scripts/closeout.sh`**
- **Purpose:** Replaces the Closeout and Commit Verification reports.
- **Inputs:** None (reads from `.work_order/`).
- **Outputs:** Runs `git add`, `git commit`, and clears the `.work_order` directory.
- **Language:** Shell.

## 10. Recommended Prompt/Handoff Strategy
- **Minimize Token Waste:** Stop asking Codex to "reconfirm authority" or print the branch name in every prompt. Assume context is established by the injected JSON.
- **Codex Prompt:** *"Implement the requirements in `manifest.json`. Run the validation command. When tests pass, write your summary to `execution.json`."*
- **Gemini Prompt:** *"You are a strict compliance checker. Read `manifest.json` and the provided `git diff`. Answer only in JSON. Does the diff violate any 'Strict Constraints'? Does the diff touch out-of-scope files? Verdict: PASS or FAIL."*

## 11. Risks and Guardrails
- **Over-automation Risk:** If the review script automatically triggers Codex to fix a failure, they can get stuck in an infinite loop (e.g., if the MPWO instructions are contradictory).
- **Guardrail:** The `review_diff.py` script must **halt and exit** on a FAIL verdict. A human must decide whether to issue a Corrective Execution or an Adjudication override (as you correctly did manually for M8-04).

## 12. Final Recommendation
**The single highest-value next change:** Eliminate the WebUI copy-paste bottleneck and the verbose markdown reports.

**The simplest possible automation stack for this repo:**
Create a single script: `./scripts/ticket.sh TASK-M8-05`.
1. It greps the MPWO for the ticket block and saves it to a temp file.
2. You use Codex in your CLI/IDE to implement the fix.
3. You run `./scripts/review.sh` which grabs `git diff HEAD`, pipes it to Gemini via CLI/API, and prints `PASS/FAIL`.
4. If it passes, you run `./scripts/commit.sh` which stages and commits.

This preserves your exact, rigorous `AGENTS.md` discipline while removing the massive token and time overhead of having LLMs write administrative markdown files.