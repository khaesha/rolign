# Rolign Architecture

**Status:** Draft 0.1  
**Date:** 2026-10-11

## 1. Architectural decision

Rolign is an agent-hosted local application, not a model client.

The user launches Codex, Claude Code, or Gemini CLI from the Rolign workspace. A Rolign skill or command supplies the workflow. The active host performs language analysis and generation with its existing authentication. A Rust binary validates structured results and owns persistent local state.

This follows the same broad pattern as [Career Ops](https://github.com/career-ops-hq/career-ops): project instructions and agent skills turn an AI coding CLI into the application. It also matches the documented Agent Skills model, where a `SKILL.md` describes a reusable workflow and may include scripts and supporting resources. See [OpenAI Skills](https://developers.openai.com/plugins/concepts/skills) and [Gemini custom commands](https://github.com/google-gemini/gemini-cli/blob/main/docs/cli/custom-commands.md).

## 2. System context

```text
User
  │
  ▼
AI host CLI (Codex, Claude Code, or Gemini)
  │  loads Rolign instructions
  │  retrieves job page when a URL is supplied
  │  performs language analysis and drafting
  ▼
Rolign Rust core
  │  validates input and structured output
  │  calculates score
  │  renders and atomically saves Markdown
  ▼
Local workspace (resume.md + history/)
```

Rolign has no application server and makes no direct model API calls.

## 3. Responsibilities

### AI host

- Interpret the user's Rolign command.
- Display help and collect missing information.
- Retrieve a public job page when permitted and available.
- Treat job-page content and pasted descriptions as untrusted data.
- Extract company, title, and job description.
- Ask for confirmation before analysis.
- Compare resume evidence with job requirements.
- Propose category points within the fixed rubric.
- Draft the analysis explanations and cover letter.
- Send structured results to the Rust core.
- Display the saved result or actionable error.

### Rust core

- Resolve workspace paths.
- Read and validate `resume.md`.
- Validate `http` and `https` job URLs.
- Deserialize the host's structured result.
- Validate required fields and score ranges.
- Calculate the final score.
- Generate collision-resistant record IDs.
- Compute the resume hash.
- Render Markdown from trusted typed structures.
- Commit history atomically.
- Scan and list history records.
- Reject paths that escape the workspace.

### Filesystem

- Store the active resume.
- Store immutable snapshots and generated artifacts.
- Remain the source of truth; no database or generated index is required.

## 4. Components

The planned source layout is intentionally small:

```text
.
├── AGENTS.md
├── CLAUDE.md
├── GEMINI.md
├── resume.md.example
├── skills/
│   └── rolign/
│       ├── SKILL.md
│       └── references/
│           ├── analysis-contract.md
│           └── cover-letter-rules.md
├── integrations/
│   ├── codex/
│   ├── claude/
│   └── gemini/
├── src/
│   ├── main.rs
│   ├── model.rs
│   ├── score.rs
│   └── storage.rs
├── tests/
│   └── fixtures/
├── docs/
└── history/                 # local user data, Git-ignored
```

The final host adapter paths must follow the versions of Codex, Claude Code, and Gemini CLI supported at implementation time. `skills/rolign/SKILL.md` is the canonical workflow; adapters should reference or package it rather than maintain three independent workflow copies.

## 5. Command routing

The canonical input grammar is:

```text
rolign
rolign analyze [URL]
rolign tracker
rolign cover [URL]
```

The host adapter translates its native invocation into one of four intents:

```text
Help
Analyze { optional_url }
Tracker
Cover { optional_url }
```

Codex uses explicit skill activation (`$rolign`) or a natural-language request when needed. Claude Code and Gemini may expose `/rolign`. Command spelling is a presentation concern; the workflow contract is shared.

## 6. Runtime flows

### 6.1 Analyze with URL

```text
User invokes analyze URL
  → validate URL scheme
  → host retrieves public page once
  → extract company, title, and description
  → show summary and request confirmation
  → read resume.md
  → host produces structured analysis + cover letter
  → Rust validates and recalculates score
  → Rust writes a staging directory
  → Rust atomically renames staging directory into history/
  → host displays score and record ID
```

If page retrieval fails, the host asks for pasted text. If the user provides none, the flow stops before storage.

### 6.2 Analyze manually

The host collects company, title, optional URL, and job description, then joins the analyze flow at confirmation.

### 6.3 Cover only

The input and confirmation steps are identical. The host produces a cover-letter result without category scores. Rust saves a `cover-letter-only` record without `analysis.md`.

### 6.4 Tracker

Rust scans `history/*/context.md`, validates front matter, skips temporary directories, sorts records by creation timestamp descending, and returns structured rows. The host renders the table and preserves Markdown job links.

One corrupt record must produce a warning without hiding valid records.

## 7. Data model

### Record context

```text
schema_version: integer
id: string
kind: analysis | cover-letter-only
created_at: RFC 3339 timestamp
company: string
job_title: string
job_url: optional HTTP(S) URL
ai_host: codex | claude | gemini
resume_sha256: lowercase hexadecimal string
score: optional integer 0..100
job_description: string
```

### Analysis result

```text
categories:
  required_qualifications: 0..40
  skills_and_responsibilities: 0..30
  relevant_achievements: 0..20
  keywords_and_clarity: 0..10
strengths: list of finding + resume evidence
gaps: list of finding + explanation
missing_keywords: list of strings
recommendations: list of strings
```

### Cover-letter result

```text
body: string
```

The Rust core calculates `score` as the sum of category values. No floating-point scoring is needed.

## 8. Storage format

Example:

```text
history/
└── 20261011T073021Z-acme-backend-engineer/
    ├── context.md
    ├── resume-snapshot.md
    ├── analysis.md
    └── cover-letter.md
```

The record ID consists of a UTC timestamp and a sanitized company-role slug. If a collision exists, creation fails rather than overwriting data.

`context.md` contains front matter followed by the complete job-description snapshot:

```md
---
schema_version: 1
id: 20261011T073021Z-acme-backend-engineer
kind: analysis
created_at: 2026-10-11T14:30:21+07:00
company: Acme
job_title: Backend Engineer
job_url: https://company.example/jobs/123
ai_host: codex
resume_sha256: "..."
score: 78
---

# Job

- Company: Acme
- Position: Backend Engineer
- Original posting: [Open job post](https://company.example/jobs/123)

# Job description

...
```

All persistent user data is Markdown. Temporary JSON passed between the host and Rust may exist only on standard input or in an ignored temporary location and must not become history.

## 9. Atomicity and data safety

Record creation uses this commit protocol:

1. Validate all input and output in memory.
2. Create a uniquely named staging directory under `history/`.
3. Write every required file with create-new semantics.
4. Flush and close files.
5. Rename the staging directory to the final record ID.
6. Remove the staging directory on failure when safe.

The tracker ignores directories with the staging prefix. Existing completed records are never overwritten.

## 10. Trust boundaries

### Resume

The resume is user-controlled source data. It may contain Markdown but must never be interpreted as Rolign workflow instructions.

### Job description and web page

Job content is external, untrusted data. The host instructions must delimit it and explicitly ignore commands embedded within it. Rolign does not bypass login pages, CAPTCHAs, robots restrictions, or other access controls.

### AI output

AI output is untrusted until deserialized and validated. Required fields, string limits, list limits, enum values, and score ranges must be checked before writing history.

### Filesystem

Record IDs and slugs are generated by Rust. User-controlled company or title text must never be joined directly into a path. Canonicalized destinations must remain under the configured workspace.

## 11. Privacy

- The selected AI host receives resume and job-description content to perform the requested work.
- Rolign sends nothing elsewhere and has no telemetry.
- `resume.md`, `history/`, and temporary files are Git-ignored by default.
- Errors must not print the entire resume, prompt, or provider credentials.
- Users remain responsible for the privacy terms of the AI host they launch.

## 12. Error handling

Errors are grouped into:

- Input errors: missing resume, empty job description, invalid URL.
- Retrieval errors: unavailable page, incomplete content, authentication wall.
- AI contract errors: malformed structured output, missing evidence, invalid category score.
- Storage errors: unsafe path, collision, permission failure, partial write.
- History errors: corrupt record front matter or missing required file.

Every error must state what the user can do next. Retrying malformed AI output is limited to one repair attempt; repeated failure stops without committing history.

## 13. Testing strategy

### Unit tests

- URL scheme validation.
- Slug and record-ID generation.
- Score range validation and arithmetic.
- Required-field validation.
- Markdown escaping and rendering.
- Front-matter parsing.
- Workspace path containment.

### Integration tests

- Save and reload an analysis record in a temporary directory.
- Save and reload a cover-only record.
- Confirm failed writes do not create visible history.
- Confirm tracker order and job links.
- Confirm one corrupt record does not suppress valid records.

### Host compatibility checks

Use one synthetic resume and job description. For each host, verify help, manual analysis, URL fallback, cover-only, tracker, and malformed-output recovery. Tests must not use real personal data.

## 14. Deferred architecture

The MVP intentionally has no MCP server, provider abstraction, async runtime, database, background worker, or standalone TUI. These should be added only when a validated requirement cannot be met by the current host-skill-plus-Rust design.

