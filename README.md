# Rolign

Local-first resume-to-job workflow for Codex, Claude Code, and Gemini CLI.

The active AI host handles analysis and drafting. The Rust core will validate
results and save local Markdown history. Rolign does not call model APIs,
retrieve job pages, or send telemetry itself.

## Status

Foundation only. Rolign commands and history storage arrive in later phases.

## Development

```bash
rtk cargo fmt --check
rtk cargo check
rtk cargo clippy --all-targets --all-features -- -D warnings
rtk cargo test
```

Copy `resume.md.example` to `resume.md` only when a later phase requires a
resume. `resume.md`, `history/`, and `.rolign-tmp/` are intentionally ignored
because they can contain personal data.
