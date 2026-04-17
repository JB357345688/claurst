# Prompt for Opus: MPWO Work Order Pack Generation

Please act as a Senior Technical Program Manager and Lead Architect for the Claurst project. We have a finalized implementation plan for the Multi-Provider Worker Orchestration (MPWO) feature, spanning Milestones 7 through 13.

I am providing the complete contents of `IMPLEMENTATION_PLAN_MPWO.md` as your source of truth.

**Your Task:**
Translate the implementation plan into a highly detailed, foolproof **"Work Order Pack"** designed specifically to be executed by lesser-skilled AI coding agents (or junior developers).

Lesser-skilled LLMs are prone to hallucinating missing context, over-engineering solutions, or straying from the exact scope. Your work order pack must prevent this by being exhaustively explicit.

Please structure the work order pack as a series of atomic, sequential tickets. For each task/ticket, you MUST provide:

1. **Ticket ID & Title** (e.g., `TASK-M7-01: Create provider_resolution.rs`)
2. **Context & Objective:** A very brief explanation of *why* this is being done, tied back to the milestone.
3. **Exact Code Targets:**
    *   File path(s) to create or modify.
    *   Exact struct names, function signatures, and enums to implement.
    *   Specific line ranges to extract or replace (based on the provided plan).
4. **Step-by-Step Instructions:** A literal, mechanical checklist of what code to write or move.
5. **Strict Constraints (The "Do NOT" List):**
    *   Explicitly list things the agent might be tempted to do but MUST NOT do (e.g., "Do NOT add TrustDomain yet", "Do NOT remove the client: &AnthropicClient parameter", "Do NOT change surrounding logic").
6. **Validation / Acceptance Criteria:**
    *   Exactly what the agent needs to compile, run, or test to prove the task is complete.
    *   Specific `cargo` commands to run.

**Scope to Cover:**
Please generate these detailed work orders for **Milestone 7** and **Milestone 8** first, as they are the immediate coding tasks (D1 Seam Extraction and D1 Worker Propagation). Then provide a slightly higher-level (but still actionable) breakdown for Milestones 9, 11, and 12.

Make the output format clean Markdown so it can be easily copy-pasted to the executing agents.

---
*(Include the contents of IMPLEMENTATION_PLAN_MPWO.md when sending this to Opus)*