# Workflow Automation Assessment

## 1. Evidence reviewed
- **Authority Documents:** `AGENTS.md`, `docs/Current/MPWO_WORK_ORDER_PACK.md`.
- **Workflow Artifacts:**
  - `docs/archive/reports/TASK-M8-04_PREFLIGHT_REPORT_20260412T101332Z.md`
  - `docs/archive/reports/TASK-M8-04_EXECUTION_REPORT_20260412T103524Z.md`
  - `docs/archive/reports/TASK-M8-04_REVIEW_REPORT_20260412T103951Z.md`
  - `docs/archive/reports/TASK-M8-04_CORRECTIVE_PREFLIGHT_REPORT_20260412T105346Z.md`
  - `docs/archive/reports/TASK-M8-04_ADJUDICATION_REPORT_20260412T111215Z.md`
  - `docs/archive/reports/TASK-M8-04_CLOSEOUT_REPORT_20260412T113440Z.md`
  - `docs/archive/reports/TASK-M8-04_COMMIT_VERIFICATION_REPORT_20260412T113834Z.md`
  - `docs/archive/reports/TASK-M8-11_EXECUTION_REPORT_20260413T002601Z.md`

## 2. Reconstructed current workflow
Based on the evidence, the exact workflow sequence currently being followed is:
1. **Preflight Phase:** The executor AI (Codex) reads the target ticket from `MPWO_WORK_ORDER_PACK.md`, inspects current `git status`, verifies preconditions, and writes a structured `PREFLIGHT_REPORT.md`.
2. **Execution Phase:** Codex implements the code changes, runs the specified `cargo` validation commands, and writes an `EXECUTION_REPORT.md`.
3. **Review Phase (Manual Handoff):** The human gathers the execution report and `git diff`, sends them to an external reviewer (Gemini via WebUI/Android), and records the reviewer's verdict in a `REVIEW_REPORT.md`.
4. **Corrective/Adjudication Phase:** If the review fails (as in M8-04), Codex produces a `CORRECTIVE_PREFLIGHT_REPORT.md` and/or `ADJUDICATION_REPORT.md` to investigate the failure, verify if it falls within the strict ticket scope, and determine the next steps.
5. **Closeout Phase:** Once the review is accepted, Codex verifies the clean ticket state, ensures no out-of-scope files are touched, and writes a `CLOSEOUT_REPORT.md` preparing for commit.
6. **Commit Verification Phase:** The human or agent commits the code. Codex verifies `git log` and `git diff --cached` to confirm the commit matches the closeout basis, writing the final `COMMIT_VERIFICATION_REPORT.md`.

## 3. Manual steps that are good candidates for automation
Without altering the governance model, the following manual and repetitive steps can be safely automated:
- **MPWO Ticket Extraction:** Manually finding and copying the active ticket block from the large `MPWO_WORK_ORDER_PACK.md` file.
- **Deterministic Repo State Gathering:** Asking Codex to execute `git branch`, `git status`, `git diff --name-only`, and `git log` across multiple turns for every single report.
- **Validation Execution & Capture:** Manually running `cargo check`, `cargo test`, `cargo clippy`, and `cargo fmt`, then copying the terminal output back into the chat window for Codex to document.
- **Review Packet Assembly & Handoff:** Copying the `git diff`, the execution report, and the ticket constraints into an external WebUI chat window for the review pass.
- **Report Boilerplate Generation:** Typing out the markdown structural headers (Ticket, Timestamp, Branch, Repo-State Summary) for the 5+ reports generated per ticket.

## 4. Steps that should remain explicit and human-gated
To preserve the rigorous governance of `AGENTS.md`, these controls MUST NOT be automated away:
- **Phase Transitions:** The transition from Execution to Review, and from Review to Closeout, must remain explicit. Automation scripts should pause and await human confirmation before moving to the next phase.
- **Adjudication Decisions:** When a review fails, the decision of whether to apply a corrective patch or issue an adjudication override (e.g., M8-04's dummy client finding) must remain a human-driven, explicitly documented phase.
- **Closeout Authorization:** The final decision to approve a ticket for closeout and prepare the commit.
- **Commit Execution:** The actual execution of `git commit` should remain a discrete step triggered by the human, separating the closeout recommendation from the permanent history change.

## 5. Recommended automation layers
**Low-risk immediate wins:**
- Helper scripts to dump deterministic data (git state, cargo outputs) into JSON sidecars so Codex doesn't waste time running terminal commands just to read the output.
- Scripted extraction of the active MPWO ticket into an isolated text file for focused context.

**Medium-effort improvements:**
- Scaffolding scripts that auto-generate the `.md` report shells with timestamps, git state, and ticket info already filled in. Codex then only needs to write the analytical sections ("Findings", "Deviations", "Verdict").

**Later refinements:**
- A CLI wrapper that completely replaces the WebUI copy/paste loop by sending the review packet (diff + manifest + execution report) directly to the Gemini API and outputting the draft `REVIEW_REPORT.md` locally.

## 6. Report-preserving automation design
Automation will *support* the existing markdown reports, not replace them.
- **Data Gathering & Sidecars:** A local script runs at the start of a phase, executing git commands and validations, and saving the output to `.ticket_context/repo_state.json` and `.ticket_context/validation.json`.
- **Templating:** The script generates a draft `PREFLIGHT_REPORT.md` (or other report) containing all the deterministic data (timestamps, branch, untracked files).
- **Codex Role:** Codex reads the sidecars and the draft markdown, applies its reasoning, and fills in the "Exact Scope Confirmation", "Blockers", and "Verdict" sections of the markdown file. The markdown file remains the source of truth.
- **Review Packet:** A script bundles `repo_state.json` and the `git diff` into a payload sent to the Gemini API. The API returns a generated `REVIEW_REPORT.md` directly into the `docs/archive/reports/` folder.

## 7. Suggested machine-readable support artifacts
These files will live in a temporary `.ticket_context/` directory to feed context to the agents:
- **`ticket_manifest.json`**: The exact MPWO text for the active ticket, stripped of surrounding milestone noise.
- **`repo_state_snapshot.json`**: The exact outputs of `git status --short`, `git branch`, and `git rev-parse HEAD`.
- **`validation_results.json`**: The exit codes and raw stdout/stderr of the required `cargo` commands.
- **`review_packet.json`**: The structured payload combining the ticket manifest, the execution report, and `git diff HEAD`.

## 8. Candidate CLI tooling layout
All scripts should be practical, composable Python or Shell scripts.

**1. `scripts/prep_phase.py`**
- **Purpose:** Replaces the manual querying of git status and ticket extraction.
- **Inputs:** Ticket ID (e.g., `TASK-M8-05`).
- **Outputs:** `.ticket_context/ticket_manifest.json`, `.ticket_context/repo_state_snapshot.json`.
- **Language:** Python.

**2. `scripts/run_validation.sh`**
- **Purpose:** Replaces the manual running and copy/pasting of cargo checks.
- **Inputs:** None (reads `ticket_manifest.json` for required commands).
- **Outputs:** `.ticket_context/validation_results.json`.
- **Language:** Bash.

**3. `scripts/trigger_review.py`**
- **Purpose:** Replaces the manual WebUI copy/paste bottleneck.
- **Inputs:** `review_packet.json` (diff + manifest).
- **Outputs:** Calls Gemini API, streams output directly into `docs/archive/reports/TASK-M8-XX_REVIEW_REPORT.md`.
- **Language:** Python.

## 9. Recommended Codex / Gemini / human partition
- **Codex (Executor & Documenter):** Continues to be the primary execution engine. Uses the JSON sidecars to rapidly write code and fill out the qualitative sections of the Preflight, Execution, Closeout, and Commit Verification markdown reports.
- **Gemini (Strict Reviewer):** Operates as the fresh-context reviewer via API. Consumes the review packet and strictly evaluates the diff against the `AGENTS.md` rules and the ticket manifest to produce the `REVIEW_REPORT.md`.
- **Human (Gatekeeper):** Triggers the scripts, adjudicates any review failures (like the M8-04 dummy client), authorizes the transition to closeout, and manually fires the git commit.

## 10. Minimal-change target workflow
This proposed workflow looks exactly like the current one, but eliminates the manual copy-pasting and terminal scraping:
1. **Human:** Runs `./scripts/prep_phase.py TASK-M8-05`.
2. **Codex:** Reads the generated `.ticket_context` sidecars and writes `PREFLIGHT_REPORT.md`.
3. **Codex:** Executes the code changes in `src-rust/`.
4. **Human:** Runs `./scripts/run_validation.sh`.
5. **Codex:** Reads `validation_results.json` and writes `EXECUTION_REPORT.md`.
6. **Human:** Runs `./scripts/trigger_review.py`.
7. **Gemini API:** Automatically reads the diff and outputs `REVIEW_REPORT.md` (e.g., Verdict: PASS).
8. **Human:** Inspects the review report. Authorizes closeout.
9. **Codex:** Reads the clean repo state and writes `CLOSEOUT_REPORT.md`.
10. **Human:** Runs `git commit -m "TASK-M8-05 ..."`.
11. **Codex:** Reads the new `git log` sidecar and writes `COMMIT_VERIFICATION_REPORT.md`.

## 11. Risks and guardrails
- **Risk:** Automation scripts silently hiding validation failures from Codex, leading to false-positive Execution Reports.
  - **Guardrail:** The `validation_results.json` must explicitly include exit codes. Codex must be prompted to explicitly quote the exit codes in the markdown report.
- **Risk:** The review script accidentally expanding scope by trying to "fix" the code instead of just reviewing it.
  - **Guardrail:** `trigger_review.py` must strictly be a read-only API call that only generates a markdown report. It must have no ability to write to `src-rust/`.
- **Risk:** The preflight script missing untracked files, breaking the review-basis rules seen in M8-04.
  - **Guardrail:** `repo_state_snapshot.json` must explicitly capture the output of `git ls-files --others --exclude-standard` to guarantee untracked noise is passed to the AI for documentation.

## 12. Final recommendation

1. **The single best manual bottleneck to automate first:**
   The manual compilation and formatting of the Review Packet for WebUI copy-pasting. Moving the Review phase to a local CLI script that calls the Gemini API will save the most context-switching time.

2. **The safest first automation script to build:**
   `scripts/trigger_review.py`. It is read-only, does not touch the execution flow, does not modify `AGENTS.md`, and perfectly preserves the formal phase boundaries while outputting the required `REVIEW_REPORT.md`.

3. **Sample one-ticket flow:**
   - **Manual step:** `./scripts/prep_phase.py TASK-M8-05`
   - **Codex writes:** `PREFLIGHT_REPORT.md`
   - **Codex edits:** `agent_tool.rs`
   - **Manual step:** `./scripts/run_validation.sh`
   - **Codex writes:** `EXECUTION_REPORT.md`
   - **Manual step:** `./scripts/trigger_review.py` -> outputs `REVIEW_REPORT.md`
   - **Human:** Reads review, types "looks good, prep closeout"
   - **Codex writes:** `CLOSEOUT_REPORT.md`
   - **Human:** `git commit`
   - **Codex writes:** `COMMIT_VERIFICATION_REPORT.md`