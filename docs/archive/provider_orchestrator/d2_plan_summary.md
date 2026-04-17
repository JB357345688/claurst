# D2 Micro-Patch Plan Pass

## A. Executive Judgment

**Recommended next move:** `2) implementation micro-patch only`

Interpretation:
- Narrowest safe move is a **test-only D2 micro-patch**
- **No production behavior change**
- **No RFC edit required**

Why:
- RFC v3.1 already explicitly documents the D2 limitation that custom/unknown providers default to `TrustDomain::Cloud`
- RFC v3.1 also settles explicit provider pinning semantics, including `provider: "anthropic"` as a true pin
- Adding implementation guardrails now would widen scope into the explicitly deferred custom trust-domain/config contract

## B. Proposed Scope

**Exact recommended next step:**
- Add **one narrow regression/unit test** that locks the documented D2 behavior for custom/unknown providers:
  - unrecognized provider IDs default to `TrustDomain::Cloud`
  - D2 fallback logic treats them consistently with the current built-in-only trust-domain model

**Why this is the minimum safe scope:**
- The RFC already documents the limitation clearly
- A test prevents spec/implementation drift
- A production patch here would effectively invent a new trust-domain policy surface that v3.1 explicitly defers

**Conclusion on the known D2 limitation:**
- Recommendation: **documentation + regression test**
- Not documentation-only, because the behavior is subtle and easy to regress
- Not implementation guardrails, because that would exceed v3.1 scope

**Conclusion on explicit provider pinning semantics:**
- **Fully settled in RFC v3.1**
- No further RFC clarification appears necessary for pinning semantics

**Conclusion on provider/model compatibility edge cases:**
- The only compatibility edge that still clearly deserves test coverage is:
  - reverse-direction explicit pin conflict behavior
  - example: `provider: "anthropic"` + `model: "openai/gpt-4.1"`
- In RFC terms, this is already settled; the remaining question is whether the implementation/tests already lock it

## C. Intended Touchpoints

Likely files/modules/tests if implementation proceeds:

- `src-rust/crates/query/src/provider_resolution.rs`
  - primary likely location for `TrustDomain::for_provider()`
  - primary likely location for fallback-candidate selection logic
  - most likely place for the added regression test if tests are colocated

- `src-rust/crates/query/tests/`
  - possible home for a focused fallback/regression test if this is tested at integration level

- `src-rust/crates/query/src/agent_tool.rs`
  - inspect only if fallback wiring or worker resolution path needs confirmation

- `src-rust/crates/tools/src/team_tool.rs`
  - inspect only if TeamCreate shares the same D2 fallback path under test

- `src-rust/crates/api/src/model_registry.rs`
  - inspect only if fallback model-selection or compatibility behavior is implicated

## D. Risks

**If we patch too little:**
- The RFC's documented custom-provider Cloud-default behavior could drift from implementation
- A self-hosted OpenAI-compatible endpoint could be treated inconsistently during D2 fallback
- Future contributors may incorrectly "fix" the behavior ad hoc because it looks surprising

**If we over-engineer it:**
- We accidentally introduce a new custom trust-domain subsystem
- We add config like `trust_domain`, exclusion lists, or local/cloud heuristics that the RFC explicitly defers
- We widen a micro-patch into an architectural change that breaks v3.1 intent

## E. Gating Questions

These should be confirmed before coding:

1. Should the regression lock the **full current documented behavior** that unknown/custom providers may participate as **Cloud fallback candidates**, or only the narrower fact that they classify as `TrustDomain::Cloud`?

2. Is the intended D2 behavior for unpinned workers with `provider: None`:
   - fallback allowed only when `allow_fallback: true`
   - or fallback allowed by default after normal resolution fails?

3. Do you want the next patch to also include a **small D1 regression test** for the reverse explicit-pin conflict case:
   - `provider: "anthropic"` + `model: "openai/gpt-4.1"`
   even though that is not itself D2?

## F. If Implementation Proceeds

Narrow mechanical execution plan:

1. Inspect `TrustDomain::for_provider()` in `provider_resolution`.
2. Inspect fallback-candidate enumeration in the D2 fallback path.
3. Add one focused regression test using:
   - one built-in Cloud provider
   - one unknown/custom provider ID
4. Assert only the behavior documented in RFC v3.1:
   - unknown/custom provider defaults to `TrustDomain::Cloud`
   - fallback treatment matches the built-in-only trust-domain model
5. Do **not** add:
   - new config fields
   - custom trust-domain overrides
   - fallback exclusion mechanisms
   - locality heuristics
6. Separately verify whether D1 test coverage already includes:
   - reverse explicit pin conflict
   - `provider: "anthropic"` true-pin semantics
7. Run targeted provider-resolution tests first, then broader query tests only if needed

## Bottom Line

The minimum safe next step is:

**Add one narrow regression test and nothing else.**

That preserves v3.1 intent, avoids redesign, and locks the one D2 limitation that is now documented but still easy to regress.
