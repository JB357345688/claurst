Prompt for Opus: Provider-Aware Worker Orchestration Implementation Plan

Please act as a senior Rust engineer and architect for the Claurst project.

We are transitioning from the architecture design and RFC phase into the disciplined implementation phase for the “Provider-Aware Worker Orchestration” feature.

I have provided three critical context documents:
- Claurst_progress_report.md
- RFC_PROVIDER_AWARE_WORKER_FABRIC_IMPLEMENTATION_RECONCILIATION.md
- RFC_PROVIDER_AWARE_WORKER_FABRIC_v3.1.md

Your Task:
Based on these documents, produce a comprehensive and detailed Implementation Plan starting from Milestone 6 onwards.

Core interpretation rules
1. Treat RFC v3.1 as the absolute source of truth for **target design intent**.
2. Treat the Reconciliation Report as the absolute source of truth for **current implementation reality**.
3. When describing the current codebase, reconciliation wins.
4. When describing the intended end-state, RFC v3.1 wins.
5. If the RFC and current codebase differ, do not invent seams, helpers, modules, or behavior that do not currently exist. Explicitly call the gap out.
6. This is an execution-planning document, not a brainstorming memo and not an RFC rewrite.

Phase framing
Explicitly frame this plan as the start of a new implementation phase.
Make clear that:
- architecture/RFC development is complete enough
- reconciliation is complete enough
- the project is now entering staged implementation
- D1 seam extraction is the first hard prerequisite before any honest D2 landing

Milestone ordering
Define the work strictly in the milestone order established in the Progress Report, starting at Milestone 6.

You must keep these tracks of work clearly separated:
1. D1 Seam Extraction
2. D1 Propagation / Completion
3. D1 Validation
4. D2 Planning
5. D2 Implementation
6. Surrogate Test Retirement / Replacement
7. Final Integration / Closeout

Detailed requirements per milestone
For each milestone include all of the following:
- Milestone Name
- Purpose
- Goals & Scope
- Exact code targets
  - files
  - functions
  - structs / enums / modules
  - if a target is uncertain, explicitly mark it “requires confirmation” rather than guessing
- Preserved Invariants
- Dependencies / Preconditions
- Risks & Assumptions
- Acceptance Gates / Exit Criteria
- Deliverables / Outputs

Mandatory planning content

A. D1 Seam Extraction
You must identify the first real production extraction target from the reconciliation report.
Describe:
- the exact current inline provider-resolution/materialization location
- what should be extracted
- what must remain behaviorally unchanged during extraction
- what must not be introduced yet
Do not add D2 fallback behavior at this stage.

B. D1 Propagation / Completion
Describe how the new shared seam should be adopted by:
- AgentTool
- TeamCreate worker paths
- any other relevant worker execution paths identified in the reconciliation report

You must explain how this phase handles:
- provider pinning
- parent/worker inheritance
- fail-loud behavior where required
- removal of Anthropic-only assumptions in worker paths

C. D1 Validation
Define the testing/validation strategy required to declare D1 complete.
This should include:
- migrated provider-resolution coverage
- provider pinning/conflict behavior
- worker inheritance validation
- regression protection against reintroducing inline-only or Anthropic-only worker behavior

D. D2 Planning
Do not prematurely implement D2 in this section.
Instead, describe the planning work that happens only after D1 is real.
This should cover:
- trust-domain classification
- fallback candidate enumeration
- same-domain fallback behavior
- capability / health-based filtering
- any honest seam re-evaluation now that D1 exists

E. D2 Implementation
Describe the staged landing of D2 only at the real seam established by D1.
Keep this grounded in the checkout and RFC. Do not invent a broad policy/config system unless genuinely required.

F. Surrogate Test Retirement
Explain:
- whether the current surrogate/spec-proxy test should remain temporarily
- what exact milestone should trigger its removal/replacement
- what real production tests must exist before that surrogate is retired

Scope discipline
- Do not jump directly to D2 implementation.
- D1 seam extraction is a hard prerequisite.
- Do not invent broad policy/config systems prematurely.
- Do not treat the surrogate D2 test as real production D2 coverage.
- Keep the plan tightly grounded in the current checkout.

Output requirements
Output in Markdown.
Use clear top-level sections.
Use highly recognizable milestone names.
Make the roadmap easy to scan and easy to refer back to later.

Required output structure
1. Executive Summary
2. Phase Transition Statement
3. Current State vs Target State
4. Milestone Roadmap (Milestone 6 onward)
5. Detailed Milestone Plans
6. Dependency and Sequencing Summary
7. Risks and Scope Discipline Rules
8. Immediate Next Actions
9. Final Phase Summary

Final instruction
This plan must read like a serious execution guide for the next phase of work.
It should create a clean boundary between completed design/reconciliation work and the upcoming implementation work.
It must identify the first real coding move, the milestone path after that, and the acceptance gates through to finish.