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
```

These are core commands, not the final `$rolign` workflow. The host adapter
arrives in Phase 2.

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
