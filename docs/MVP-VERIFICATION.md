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

## Fresh-clone check

Use a new temporary clone with only synthetic data:

1. Clone the release branch or `main` into an empty directory.
2. Copy `resume.md.example` to `resume.md`.
3. Run the automated checks above.
4. Run `cargo run -- validate-url https://example.test/job/123`.
5. Save `tests/fixtures/analysis-request.json`, then list the saved record.

The fixture resume, job text, job page, and cover letter are synthetic. No test
uses a personal resume or a live/private job posting.

## Privacy and ignore audit

Verify that Git ignores `resume.md`, `history/`, and `.rolign-tmp/`, and review
tracked files for credentials, personal contact details, personal resumes, and
live/private job URLs. The expected tracked fixture URL is
`https://example.test/job/123`.

## PRD success evidence

| Criterion | Evidence |
| --- | --- |
| Supported host invocation | Project adapters and static contract tests cover Codex, Claude Code, and Gemini CLI routing. |
| URL and manual analysis | Canonical workflow and synthetic URL fixture cover validation, one retrieval attempt, confirmation, and pasted fallback. |
| Cover-only and tracker | CLI/storage tests cover cover-only persistence, nullable score, sorting, links, and corrupt records. |
| Resume snapshots and atomic failures | Storage tests cover immutable snapshots, collisions, and invisible failed writes. |
| Cross-host arithmetic | Rust calculates every total; `ai_host` is metadata only. |

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
