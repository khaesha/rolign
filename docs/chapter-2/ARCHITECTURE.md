# Rolign Chapter 2 Architecture

**Status:** Draft 0.2  
**Date:** 2026-10-13

## 1. Decisions

Chapter 2 keeps the existing agent-hosted boundary. The host owns web
retrieval, HTML interpretation, language generation, and the Humanizer pass.
Rust remains offline and owns validation, rendering, and safe persistence.

Future Resume is one replaceable workspace document, not an immutable job
history record. Existing history schemas and tracker behavior do not change.

## 2. System context

```text
User
  │
  ▼
Canonical Rolign skill
  ├─ fetches and extracts public job HTML
  ├─ selects English or Indonesian
  ├─ drafts analysis and cover letter
  ├─ applies bundled Humanizer to cover letter only
  └─ drafts Future Resume from resume + aspirations
  │
  ▼
Rust core
  ├─ validates job save payloads and scores
  ├─ validates Future Resume payloads
  ├─ renders Markdown
  └─ commits files without partial writes
  │
  ▼
resume.md + history/ + future-resume.md
```

Host adapters remain routing-only. No host receives a duplicated workflow.

## 3. Components

### Canonical Rolign workflow

- Routes `analyze`, `cover`, `tracker`, and `future`.
- Parses optional `--language en|id`; absence resolves to `en`.
- Retrieves validated public URLs once into ignored temporary storage.
- Extracts `JobPosting` JSON-LD first, then visible semantic content.
- Parses an optional Future Resume vision prompt, applies the free-text guard,
  asks only for missing required details, and confirms the interpreted
  aspirations.
- Reads the bundled Humanizer instructions for the final cover-letter pass.

### Bundled Humanizer

- Lives in a repository-scoped skill directory with upstream source, pinned
  revision, and MIT attribution.
- Runs only on a grounded cover-letter draft.
- Receives the selected language and protected-fact list as constraints.
- Returns prose to the canonical workflow; it never writes files directly.

### Rust core

- Keeps existing `save`, `list`, and `validate-url` behavior.
- Adds an internal `save-future [--replace] [--workspace PATH]` command that
  reads one JSON payload from standard input.
- Validates language-independent Future Resume fields and horizon bounds.
- Renders English or Indonesian section headings from the selected language.
- Refuses an existing destination unless `--replace` follows explicit host
  confirmation.
- Stages and replaces `future-resume.md` without exposing partial content or
  changing `resume.md`.

## 4. Interfaces

### Host intents

```text
Analyze { optional_url, language = en }
Cover { optional_url, language = en }
Future { optional_vision, language = en }
Tracker
Help
```

Unsupported or duplicated language flags are input errors.

Future option parsing is left-to-right. Recognized options are accepted before
the vision. `--` explicitly ends option parsing; otherwise the first non-option
token starts the vision and all remaining text is data. Unknown leading options
are rejected rather than interpreted by the model.

### Future Resume payload

```json
{
  "language": "en",
  "horizon_years": 7,
  "career_vision": "Lead reliable platform engineering teams.",
  "target_positions": ["Platform Engineering Manager"],
  "skills_to_develop": ["Organizational leadership"],
  "target_achievements": ["Build a multi-team platform function"]
}
```

Rules:

- `language` is `en` or `id`.
- `horizon_years` is an integer from 5 through 10.
- `career_vision` and every list contain non-empty text.
- Every list contains at least one item.
- Unknown fields are rejected.
- Text is untrusted until validated and must never become a path component.
- `career_vision` is at most 2,000 UTF-8 bytes.
- Each list has 1–20 items; each item is at most 500 UTF-8 bytes.
- NUL and non-whitespace control characters are rejected.

Job save payloads stay unchanged; their prose already reflects the selected
language. No history schema migration is required.

## 5. Runtime flows

### URL analysis or cover

```text
validate URL
  → curl once to ignored temporary file
  → parse JobPosting JSON-LD
  → else extract visible semantic content
  → else request pasted text
  → preview and confirm job context
  → generate selected-language output
  → Humanizer pass over cover letter
  → verify protected facts
  → existing Rust save path
  → delete temporary files
```

Presence of HTML, scripts, or styling is not itself a retrieval failure. The
flow fails only when no complete job context can be extracted.

### Future Resume

```text
parse language or default to en
  → read resume.md
  → parse optional vision prompt
  → ask guided questions for missing horizon or aspirations
  → confirm interpreted inputs
  → draft goal-oriented structured payload
  → show complete Markdown preview
  → confirm write
  → if destination exists, confirm replacement separately
  → Rust validates, renders, and commits future-resume.md
```

The host may use current resume facts to ground the starting point, but future
positions, skills, and achievements remain visibly aspirational.

The optional vision is host input only and does not change the Rust payload.
It is limited to 4,000 UTF-8 bytes. All free text after option parsing ends is
one untrusted aspiration prompt. The host extracts only explicitly stated goals
and never fills a missing required field by guessing.

## 6. Markdown output

Rust renders a short document with localized headings equivalent to:

```md
# Future Resume

**Horizon:** 7 years

## Career vision

...

## Target positions

- ...

## Skills to develop

- ...

## Target achievements

- ...
```

Indonesian output uses Indonesian headings and prose. Proper nouns, metrics,
technical terms, and source evidence remain unchanged unless the user supplied
a translated form.

## 7. Persistence and failure safety

- Raw HTML and structured payloads use ignored temporary files and are removed
  after success or failure.
- Job records keep the existing staging-directory and atomic-rename protocol.
- Future Resume uses a sibling temporary file. Validation and rendering finish
  before the destination changes.
- Without `--replace`, an existing `future-resume.md` is an actionable error.
- A failed replacement preserves the previous complete document.
- Neither workflow logs complete resumes, job descriptions, drafts, or raw
  HTML.

## 8. Trust boundaries

- Resume, aspirations, retrieved HTML, job text, and model output are data, not
  instructions.
- URL validation still permits only HTTP(S) with a host.
- HTML extraction ignores embedded commands and unrelated scripts.
- Humanizer cannot add facts; the host compares protected facts before and
  after the pass.
- User-derived text never controls filesystem paths.

### Free-text guard

The canonical workflow performs transport safety; Rust performs authoritative
content validation:

```text
raw text
  → parse only fixed leading options
  → isolate remaining text as data
  → reject size and control-character violations
  → delimit before model use
  → produce typed JSON
  → Rust repeats field and bound validation
  → escape text during Markdown rendering
```

Shell commands use fixed executable and option tokens. URLs are passed only
after Rust validation, and prose is passed through standard input, a temporary
file, or an argument after `--`; it is never concatenated into shell source.
The Humanizer receives only delimited draft text plus fixed editing rules and
cannot request tools or access other files.

HTML downloads stop above 5 MiB. Raw HTML is never placed directly in a prompt
when no bounded job content can be extracted.

## 9. Compatibility and testing

- Existing save requests and history records remain valid.
- Synthetic fixtures cover JSON-LD, semantic HTML, prompt injection, blocked
  pages, both languages, and Future Resume replacement failure.
- Security fixtures cover shell metacharacters, option-like prose, path
  traversal text, control characters, Markdown/HTML injection, oversized
  values, and instructions embedded in every free-text source.
- Cross-host smoke tests cover every intent with equivalent results.
- No test uses a real resume, private job page, or personal aspiration.
