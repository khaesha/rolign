# Rolign

Local-first resume-to-job workflow for Codex, Claude Code, and Gemini CLI.

The active AI host handles analysis and drafting. The Rust core will validate
results and save local Markdown history. Rolign does not call model APIs,
retrieve job pages, or send telemetry itself.

## Status

Codex, Claude Code, and Gemini CLI adapters share one local workflow and Rust
validation path.

## Core commands

The development boundary accepts one JSON request on standard input. It reads
`resume.md` from the workspace and writes Markdown-only records under
`history/`.

```bash
rtk cargo run -- save --workspace . < tests/fixtures/analysis-request.json
rtk cargo run -- list --workspace .
rtk cargo run -- validate-url https://example.test/job/123
```

## Host usage

Install and authenticate the host CLI, start it from this project, and trust the
workspace before approving normal local tool operations.

| Host | Invocation |
| --- | --- |
| Codex | `$rolign analyze [URL]` |
| Claude Code | `/rolign analyze [URL]` |
| Gemini CLI | `/rolign analyze [URL]` |

All hosts also support `cover [URL]` and `tracker`. Tracker accepts a follow-up
`open <record-id>` request. Gemini users should run `/commands reload` after
pulling adapter changes or restart Gemini CLI. Each host validates URLs,
retrieves a public page once when available, asks for confirmation, and falls
back to pasted text without saving a premature record.

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
