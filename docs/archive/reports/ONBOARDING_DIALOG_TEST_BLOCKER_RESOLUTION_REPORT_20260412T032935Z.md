# ONBOARDING_DIALOG_TEST_BLOCKER Resolution Report

- Task name: `ONBOARDING_DIALOG_TEST_BLOCKER`
- Verdict: `DONE`
- Branch: `feature/provider-resolution-seam`

## Files Edited

- `src-rust/crates/tui/src/onboarding_dialog.rs`
- `docs/archive/reports/ONBOARDING_DIALOG_TEST_BLOCKER_RESOLUTION_REPORT_20260412T032935Z.md` (report only, not staged)

## Root Cause Assessment

- The failing test `onboarding_dialog::tests::onboarding_defaults_hidden` asserted that `OnboardingDialogState::new()` should default to `OnboardingPage::Welcome`.
- In current code, `OnboardingDialogState::new()` delegates to `Default`, and `OnboardingPage` explicitly derives its default as `ProviderSetup`.
- Visible onboarding flows are still controlled by explicit method calls:
  - `show()` sets `Welcome`
  - `show_provider_setup()` sets `ProviderSetup`
- Because the hidden initial state is only a container default and not the visible onboarding entrypoint, this was an outdated test expectation rather than a product-behavior regression.

## Exact Commands Run

```bash
cd /home/jordi/claurst
git branch --show-current
git status --short
git diff --quiet
git diff --cached --quiet
sed -n '1,220p' src-rust/crates/tui/src/onboarding_dialog.rs
sed -n '500,535p' src-rust/crates/tui/src/onboarding_dialog.rs
cd /home/jordi/claurst/src-rust && cargo test -p claurst-tui onboarding_dialog::tests::onboarding_defaults_hidden -- --exact --nocapture
rg -n "OnboardingDialogState::new\\(|show_provider_setup\\(|show\\(" /home/jordi/claurst/src-rust/crates/tui -g '*.rs'
rg -n "ProviderSetup|Welcome" /home/jordi/claurst/src-rust/crates/tui/src/onboarding_dialog.rs
rg -n "onboarding_dialog\\.|show_provider_setup\\(|show\\(\\)" /home/jordi/claurst/src-rust/crates/tui/src/app.rs /home/jordi/claurst/src-rust/crates/tui/src -g '*.rs'
sed -n '1648,1695p' /home/jordi/claurst/src-rust/crates/tui/src/app.rs
cd /home/jordi/claurst/src-rust && cargo test -p claurst-tui onboarding_dialog::tests::onboarding_defaults_hidden -- --exact --nocapture
cd /home/jordi/claurst/src-rust && cargo test -p claurst-tui onboarding_dialog -- --nocapture
rg -n "normalize_ollama_api_base|api_key_for\\(ProviderId::OLLAMA\\)|api_key_for\\(\"ollama\"\\)" /home/jordi/claurst/src-rust/crates/query/src/provider_resolution.rs /home/jordi/claurst/src-rust/crates/core/src/auth_store.rs
git diff -- src-rust/crates/tui/src/onboarding_dialog.rs
git status --short
git -C /home/jordi/claurst commit -m "test(onboarding): align default-page assertion with current behavior" -- src-rust/crates/tui/src/onboarding_dialog.rs
```

## Focused Validation Results

- `cargo test -p claurst-tui onboarding_dialog::tests::onboarding_defaults_hidden -- --exact --nocapture`
  - before fix: `FAIL`
  - failure signature:
    - left: `ProviderSetup`
    - right: `Welcome`
  - after fix: `PASS`
- `cargo test -p claurst-tui onboarding_dialog -- --nocapture`
  - `PASS`
  - result: 7 onboarding-dialog tests passed

## Commit

- Commit created: `f8eb130`
- Message: `test(onboarding): align default-page assertion with current behavior`

## Hosted Ollama Invariant Outcome

Hosted Ollama compatibility baseline preserved

Verification basis:

- No M7 seam files were edited.
- `normalize_ollama_api_base(...)` remains present in `src-rust/crates/query/src/provider_resolution.rs`.
- `AuthStore::load().api_key_for(ProviderId::OLLAMA)` remains present in `src-rust/crates/query/src/provider_resolution.rs`.
- Environment-first precedence for `AuthStore::api_key_for("ollama")` remains intact in `src-rust/crates/core/src/auth_store.rs`.

## Notes For Rerunning TASK-M7-07

- The previously blocking onboarding test failure is resolved with a narrow test-only change.
- This task did not run full workspace validation.
- `TASK-M7-07` should now be rerun from its own validation-first flow.
