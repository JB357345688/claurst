Claurst – Provider-Aware Worker Orchestration
Implementation Overview and Roadmap Baseline

North Star
Land the Provider-Aware Worker Orchestration RFC into the real Claurst codebase in a controlled, phased way, starting with the first real production seam and avoiding speculative D2 work before the code is ready.

What we are aiming to achieve from this point onwards
We are no longer in brainstorm or RFC-development mode.
We have completed the design and reconciliation stages far enough to move into implementation planning.

The practical goal now is:

1. Extract and stabilize a shared production provider-resolution seam from the current root query path.
2. Use that seam to complete D1:
   - shared provider identity/materialization path
   - worker inheritance of provider selection
   - explicit provider pinning semantics
   - fail-loud behavior where required
   - removal of inappropriate Anthropic-only assumptions in worker paths
3. Only after D1 is genuinely landed, begin D2 at the real seam:
   - trust-domain classification
   - fallback candidate enumeration
   - same-domain fallback behavior
   - capability/health-based candidate filtering
4. Retire temporary surrogate/spec-proxy test logic once a real production D2 seam exists.

Clean phase boundary
The project has crossed from:
- RFC development / architecture design
into:
- implementation planning and staged landing

This matters because the next tasks should not be more speculative RFC writing.
They should be implementation-planning tasks based on repo reality.

Completed roadmap so far

Milestone 0 – Environment unblocked
Status: Complete
Outcome:
- Correct VM identified
- Guest disk expanded successfully
- Rust toolchain installed successfully
- Native dependency issue resolved (`libasound2-dev`)
- `cargo test --no-run` works
- targeted provider-related tests run successfully
Meaning:
- The earlier “Codex cannot work because Rust tooling is unavailable” blocker is closed.

Milestone 1 – Feature ideation / architecture exploration
Status: Complete
Outcome:
- Brainstorming documents reviewed
- “Provider-aware worker fabric” identified as the most valuable next architectural capability
Meaning:
- The problem statement and strategic direction were established.

Milestone 2 – RFC drafting and refinement
Status: Complete enough
Outcome:
- RFC evolved through v1, v2, v3, and v3.1
- v3.1 is the current architecture baseline
- Main ambiguities were resolved:
  - explicit provider pinning semantics tightened
  - Anthropic special-case ambiguity removed
  - D2 trust-domain scope made more honest and bounded
Meaning:
- Architecture intent is now stable enough to implement against.

Milestone 3 – Adversarial review cycle
Status: Complete
Outcome:
- v2 review surfaced real weaknesses
- v3/v3.1 responded to those weaknesses
- RFC quality improved from “promising” to “usable as implementation intent”
Meaning:
- We are not missing another major RFC review pass before implementation starts.

Milestone 4 – Reality check against current checkout
Status: Complete
Outcome:
- Reconciliation/discovery pass run against the real codebase
- Found that RFC v3.1 is ahead of current implementation
- Confirmed D2 production surfaces do not honestly exist yet in this checkout
- Confirmed prior Codex D2 micro-patch was only a surrogate/spec-proxy test, not a real production D2 regression lock
Meaning:
- Repo reality is now understood; implementation can begin from facts, not assumptions.

Milestone 5 – First real seam identified
Status: Complete
Outcome:
- Current provider selection/materialization logic appears to live inline in the root query path
- Worker/team paths do not yet share a proper reusable provider-resolution seam
- This inline root logic is the first real production extraction target
Meaning:
- We now know where implementation must begin.

Where we are now
Current state:
- Design baseline exists
- Repo reality has been reconciled
- Environment is ready
- First real production seam has been identified

Therefore the current project phase is:
Implementation Planning / D1 Landing Preparation

What the next tasks should be from now until finish

Milestone 6 – D1 implementation plan
Status: Next
Goal:
Produce a concrete implementation plan for landing D1 in the current repo.
What this milestone should answer:
- exact function/module extraction target
- proposed shared provider-resolution seam
- preserved invariants
- call sites to migrate
- minimum test plan
- risk points and rollback boundaries
Recognition marker:
We have a repo-grounded D1 implementation plan, not just an RFC.

Milestone 7 – D1 seam extraction
Status: Proposed next coding milestone
Goal:
Extract current root provider identity/materialization logic into a shared production seam without changing policy.
Scope:
- no D2 fallback yet
- no trust-domain subsystem yet
- no speculative architecture additions
Recognition marker:
Provider resolution is no longer inline-only in the root path.

Milestone 8 – D1 worker propagation
Status: After seam extraction
Goal:
Make Agent / TeamCreate / worker paths use the shared provider-resolution seam.
Target outcomes:
- worker inheritance of provider selection
- explicit provider pinning works consistently
- Anthropic-only assumptions removed where inappropriate
- fail-loud behavior aligned with RFC intent
Recognition marker:
Root and worker paths are using the same core provider-resolution flow.

Milestone 9 – D1 validation and closeout
Status: After D1 propagation
Goal:
Prove D1 is actually landed.
Validation themes:
- explicit provider pinning
- worker inheritance
- known conflict/error behavior
- regression tests around migrated provider flow
Recognition marker:
D1 can be declared complete and shippable independently.

Milestone 10 – D2 implementation planning
Status: After D1 completion
Goal:
Re-evaluate the now-real seam and design the narrowest honest D2 landing sequence.
Focus:
- trust-domain classification
- fallback candidate enumeration
- same-domain fallback policy
- capability/health filtering
- any cancellation/budget considerations that genuinely belong at this layer
Recognition marker:
D2 has an implementation plan attached to a real production seam, not a hypothetical one.

Milestone 11 – D2 landing
Status: Later phase
Goal:
Implement D2 at the real seam created by D1.
Important constraint:
- avoid inventing a broad policy/config system unless genuinely required
Recognition marker:
Fallback/candidate enumeration exists in production code, not just RFC text.

Milestone 12 – Surrogate test retirement / replacement
Status: After real D2 seam exists
Goal:
Remove or replace any temporary RFC-anchor/spec-proxy test logic once real production D2 behavior is testable.
Recognition marker:
No surrogate D2 logic remains as a stand-in for missing production behavior.

Milestone 13 – Final integration and project closeout
Status: Final
Goal:
Close the loop between RFC intent and landed implementation.
Outputs:
- implementation complete
- tests aligned to real behavior
- temporary anchors retired
- final documentation/update notes produced
Recognition marker:
Provider-aware worker orchestration is no longer an RFC initiative; it is a landed codebase capability.

Summary of the roadmap in one line
We have completed:
environment unblock -> architecture design -> RFC refinement -> adversarial review -> repo reconciliation

We are now entering:
implementation planning -> D1 seam extraction -> D1 completion -> D2 landing -> surrogate cleanup -> final closeout

Critical planning principle from this point onwards
Do not let implementation jump directly to D2.
D1 seam extraction and D1 propagation are the necessary bridge between the RFC and the current codebase.

What Opus should produce next
Using this overview, produce a detailed implementation plan that:
- treats RFC v3.1 as the design baseline
- treats the reconciliation report as the source of repo reality
- frames the project as entering a new implementation phase
- defines the work in milestone order
- clearly separates:
  - D1 seam extraction
  - D1 propagation/completion
  - D1 validation
  - D2 planning
  - D2 implementation
  - surrogate test retirement
- includes risks, assumptions, dependencies, and acceptance gates for each milestone
- keeps scope disciplined and grounded in the current checkout