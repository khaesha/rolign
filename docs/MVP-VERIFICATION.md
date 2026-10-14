# MVP verification

## Automated checks

Run from the repository root:

```bash
cargo fmt --check
cargo check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

These checks cover URL validation, score calculation, required fields, path
containment, Markdown rendering, atomic history writes, cover-only records,
tracker ordering, and corrupt-record warnings.

Phase 7 evidence, 2026-10-13: a fresh clone of `main` at `6267f99` passed
formatting, build, and clippy. `cargo test` passed 49 tests across six suites
when its local HTTP fixture listener was permitted by the sandbox.

## Fresh-clone check

Use a new temporary clone with only synthetic data:

1. Clone the release branch or `main` into an empty directory.
2. Copy `resume.md.example` to `resume.md`.
3. Run the automated checks above.
4. Run `cargo run -- validate-url https://example.test/job/123`.
5. Save `tests/fixtures/analysis-request.json`, then list the saved record.

The fixture resume, job text, job page, and cover letter are synthetic. No test
uses a personal resume or a live/private job posting.

Phase 7 evidence, 2026-10-13: a fresh clone of `main` copied only
`resume.md.example`, validated `https://example.test/job/123`, saved and listed
the synthetic analysis fixture with score 78, created the English Future Resume,
and explicitly replaced it with the Indonesian fixture.

## Privacy and ignore audit

Verify that Git ignores `resume.md`, `history/`, `future-resume.md`, and
`.rolign-tmp/`, and review
tracked files for credentials, personal contact details, personal resumes, and
live/private job URLs. The expected tracked fixture URL is
`https://example.test/job/123`.

Phase 7 evidence, 2026-10-13: all four paths were ignored. The tracked-content
scan found no credential markers, private keys, or tracked personal artifacts;
fixture references use only `example.test` and synthetic text.

## PRD success evidence

| Criterion | Evidence |
| --- | --- |
| Supported host invocation | Project adapters and static contract tests cover Codex, Claude Code, and Gemini CLI routing. |
| URL and manual analysis | Canonical workflow and synthetic URL fixture cover validation, one retrieval attempt, confirmation, and pasted fallback. |
| Cover-only and tracker | CLI/storage tests cover cover-only persistence, nullable score, sorting, links, and corrupt records. |
| Resume snapshots and atomic failures | Storage tests cover immutable snapshots, collisions, and invisible failed writes. |
| Cross-host arithmetic | Rust calculates every total; `ai_host` is metadata only. |
| Chapter 1 compatibility | A frozen schema-v1 synthetic history record is listed by the current tracker test. |
| Future Resume safety | Rust tests cover English/Indonesian rendering, explicit replacement, unchanged resume/history, and escaped free text. |

## Chapter 2 host evidence

| Host | Version | Evidence | Status |
| --- | --- | --- | --- |
| Codex | 0.162.1 | User-tested help, URL analysis, cover-only, tracker, record opening, and Future Resume workflows on 2026-10-14. | Passed |
| Claude Code | 2.1.275 | Installed; no authenticated smoke run requested. | Pending manual run |
| Gemini CLI | 0.59.0 | Installed; no authenticated smoke run requested. | Pending manual run |

Claude Code and Gemini CLI remain pending by explicit product decision. The
full Chapter 2 host matrix remains a release gate. Record only compact
pass/fail summaries and versions; do not retain prompts, resumes, job text,
raw HTML, credentials, or model responses as verification evidence.

## Release gate

Before tagging `v0.1.0`, authenticate Codex, Claude Code, and Gemini CLI and
run this synthetic host matrix for each:

- Help, manual analysis, URL fallback using `https://example.test/job/123`,
  cover-only, tracker, and `open <record-id>`.
- A malformed save payload followed by exactly one repair attempt.
- Confirm every accepted record is written by the same Rust `rolign save` path.

Record the successful host versions and dates in the release PR. Merge first,
then create `v0.1.0` from the resulting `main` commit. Do not tag while any
host check is incomplete.
