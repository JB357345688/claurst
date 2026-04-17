# CODEX Adapter Test Blocker Resolution Report

Task name: `codex_adapter` unrelated failing test blocker resolution

Verdict: DONE

Branch: `feature/provider-resolution-seam`

Files edited:
- `src-rust/crates/api/src/codex_adapter.rs`

Root cause assessment:
- The failing assertion in `codex_adapter::tests::test_anthropic_to_openai_request_basic` compared a JSON numeric value against the literal `0.7` with exact equality.
- `CreateMessageRequest.temperature` is typed as `Option<f32>`, so serializing `0.7_f32` into `serde_json::Value` produced the expected floating-point representation artifact `0.699999988079071`.
- This was a brittle test assertion, not a production conversion regression.

Preflight:
- Verified branch: `feature/provider-resolution-seam`
- Verified tracked working tree state before editing: clean
- Observed tolerated untracked noise only: `.codex`, `docs/`, `src-rust/target/`
- Verified target file and failing assertion in `src-rust/crates/api/src/codex_adapter.rs`

Exact commands run:
- `git -C /home/jordi/claurst branch --show-current`
- `git -C /home/jordi/claurst status --short`
- `sed -n '1,260p' /home/jordi/claurst/src-rust/crates/api/src/codex_adapter.rs`
- `cargo test -p claurst-api codex_adapter::tests::test_anthropic_to_openai_request_basic -- --exact --nocapture`
- `rg -n "temperature" /home/jordi/claurst/src-rust/crates/api/src/lib.rs /home/jordi/claurst/src-rust/crates/api/src -g '!target'`
- `rg -n "normalize_ollama_api_base|api_key_for\\(ProviderId::OLLAMA\\)|api_key_for\\(\\\"ollama\\\"\\)" /home/jordi/claurst/src-rust -g '!target'`
- `sed -n '100,140p' /home/jordi/claurst/src-rust/crates/api/src/lib.rs`
- `sed -n '210,255p' /home/jordi/claurst/src-rust/crates/query/src/provider_resolution.rs`
- `sed -n '210,255p' /home/jordi/claurst/src-rust/crates/core/src/auth_store.rs`
- `git -C /home/jordi/claurst diff -- src-rust/crates/api/src/codex_adapter.rs`
- `cargo test -p claurst-api codex_adapter::tests::test_anthropic_to_openai_request_basic -- --exact --nocapture`
- `cargo test -p claurst-api codex_adapter -- --nocapture`
- `git -C /home/jordi/claurst add src-rust/crates/api/src/codex_adapter.rs`
- `git -C /home/jordi/claurst commit -m "test(codex_adapter): make request assertion robust to numeric representation"`
- `date -u +%Y%m%dT%H%M%SZ`
- `git -C /home/jordi/claurst rev-parse HEAD`

Implemented change:
- Replaced the brittle exact JSON equality check for `temperature` with a tolerance-based numeric assertion that validates the semantic value without depending on `f32` representation details.

Focused validation results:
- `cargo test -p claurst-api codex_adapter::tests::test_anthropic_to_openai_request_basic -- --exact --nocapture` -> PASS
- `cargo test -p claurst-api codex_adapter -- --nocapture` -> PASS

Commit hash:
- `fc6d5a9bf86d29cb5527a997bd245fe8dda337d1`

Hosted Ollama invariant outcome:
- Confirmed no edits touched `normalize_ollama_api_base(...)`.
- Confirmed no edits touched Ollama auth lookup via `AuthStore::load().api_key_for(ProviderId::OLLAMA)`.
- Confirmed no edits touched environment-first precedence for `AuthStore::api_key_for("ollama")`.
- Hosted Ollama compatibility baseline preserved

Notes for rerunning TASK-M7-07:
- This blocker was isolated to a brittle `codex_adapter` test assertion and is now fixed.
- No M7 milestone status files or provider-resolution seam code were modified in this task.
- Re-run the original TASK-M7-07 validation sequence from its own task context.
