# Rolign

Local-first resume-to-job workflow for Codex, Claude Code, and Gemini CLI.

The active AI host handles analysis and drafting. The Rust core will validate
results and save local Markdown history. Rolign does not call model APIs,
retrieve job pages, or send telemetry itself.

## Status

Rust storage foundation. Host-facing Rolign workflows arrive in later phases.

## Core commands

The development boundary accepts one JSON request on standard input. It reads
`resume.md` from the workspace and writes Markdown-only records under
`history/`.

```bash
rtk cargo run -- save --workspace . < tests/fixtures/analysis-request.json
rtk cargo run -- list --workspace .
rtk cargo run -- validate-url https://example.test/job/123
```

In Codex, use `$rolign` for help, `$rolign analyze` for manual analysis, or
`$rolign analyze https://example.test/job/123` for URL-first input, or
`$rolign cover [URL]` for a saved draft without scoring. `$rolign tracker`
renders saved analysis and cover-only records; ask to open a returned record ID
to view its saved Markdown files. Codex validates URLs, retrieves a public page
once when available, asks for confirmation, and falls back to pasted text
without saving a premature record.

## Development

```bash
rtk cargo fmt --check
rtk cargo check
rtk cargo clippy --all-targets --all-features -- -D warnings
rtk cargo test
```

Copy `resume.md.example` to `resume.md` before using `save`. `resume.md`,
`history/`, and `.rolign-tmp/` are intentionally ignored because they can
contain personal data.
