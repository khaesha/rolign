# Rolign

Local-first resume-to-job workflow for Codex, Claude Code, and Gemini CLI.

The active AI host handles analysis and drafting. The Rust core will validate
results and save local Markdown history. Rolign does not call model APIs,
retrieve job pages, or send telemetry itself.

## Quick start

Requirements: stable Rust plus one authenticated host CLI. Install and sign in
to your preferred host before opening this repository:

- [Codex CLI](https://help.openai.com/en/articles/11096431)
- [Claude Code](https://code.claude.com/docs/en/slash-commands)
- [Gemini CLI](https://geminicli.com/docs/get-started/)

```bash
git clone https://github.com/khaesha/rolign.git
cd rolign
cp resume.md.example resume.md
cargo test
```

Edit `resume.md` with your own Markdown resume. Rolign never changes it.

## Core commands

The development boundary accepts one JSON request on standard input. It reads
`resume.md` for saved job records, writes Markdown history under `history/`,
and can write a separate `future-resume.md`.

```bash
cargo run -- save --workspace . < tests/fixtures/analysis-request.json
cargo run -- list --workspace .
cargo run -- validate-url https://example.test/job/123
cargo run -- save-future --workspace . < tests/fixtures/future-request-en.json
```

## Use Rolign

Start an authenticated host from this project, trust the workspace, and approve
normal local tool operations.

| Host | Invocation |
| --- | --- |
| Codex | `$rolign analyze [URL] [--language en|id]` |
| Claude Code | `/rolign analyze [URL] [--language en|id]` |
| Gemini CLI | `/rolign analyze [URL] [--language en|id]` |

All hosts also support `cover [URL]` and `tracker`. Tracker accepts a follow-up
`open <record-id>` request. Gemini users should run `/commands reload` after
pulling adapter changes or restart Gemini CLI. Each host validates URLs, uses
its browser/web capability or one ordinary HTTP request, asks for confirmation,
and falls back to pasted text without saving a premature record.

`future [--language en|id] [--] [VISION...]` collects explicit career goals,
shows a preview, and asks separately before replacing an existing
`future-resume.md`. It does not rewrite `resume.md` or add a tracker record.

Commands:

```text
rolign
rolign analyze [URL] [--language en|id]
rolign cover [URL] [--language en|id]
rolign tracker
rolign future [--language en|id] [--] [VISION...]
```

English is default. `id` selects Indonesian prose; structured job-save payloads
and existing history records stay compatible.

## Privacy and limits

- `resume.md`, `history/`, `future-resume.md`, and `.rolign-tmp/` are ignored by Git. Do not commit
  resumes, saved records, temporary payloads, or credentials.
- The selected host receives the resume and job description needed for the
  requested workflow. Rust makes no model calls, web requests, telemetry, or
  background network activity.
- Public job-page retrieval is attempted once only. Login-gated, incomplete,
  or unavailable pages require pasted job text.
- Future Resume aspirations are not current qualifications. Use only goals you
  explicitly provide; the workflow does not infer missing aspirations.
- Rolign bundles the MIT-licensed [Humanizer attribution](skills/rolign/vendor/humanizer/ATTRIBUTION.md)
  for its cover-letter-only editing pass. Humanizer never processes analysis or
  Future Resume content.
- MVP scope excludes PDF/DOCX input, resume rewriting, job submission,
  databases, cloud sync, a standalone UI, JavaScript page execution, and
  bypassing login or anti-bot controls.

See [MVP verification](docs/MVP-VERIFICATION.md) for release evidence and the
remaining authenticated-host release gate.

## Development

```bash
cargo fmt --check
cargo check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

`resume.md`, `history/`, `future-resume.md`, and `.rolign-tmp/` are intentionally ignored because
they can contain personal data.
