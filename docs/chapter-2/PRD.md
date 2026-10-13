# Rolign Chapter 2 Product Requirements

**Status:** Draft 0.2  
**Date:** 2026-10-13  
**Product:** Rolign

## 1. Summary

Chapter 2 improves public job-page extraction, adds English and Indonesian
output, runs a Humanizer pass over cover letters, and introduces a Future
Resume: a brief Markdown document describing the user's 5–10-year career
vision, target positions, skills to develop, and desired achievements.

Rolign remains local-first and agent-hosted. The active Codex, Claude Code, or
Gemini CLI session performs retrieval and language work. Rust validates typed
input and owns deterministic file writes.

## 2. Goals

- Extract usable job context from raw public HTML when structured or visible
  job content is present.
- Generate analysis and cover letters in English or Indonesian, defaulting to
  English.
- Make cover letters sound natural without changing or inventing facts.
- Generate a concise Future Resume from the current resume and explicit career
  goals.
- Preserve current history records and the rule that `resume.md` is never
  modified.

## 3. Non-goals

- Executing JavaScript, adding headless-browser automation, bypassing access
  controls, or maintaining site-specific job-board scrapers.
- Translating company names, job titles, resume evidence, or quoted keywords.
- Claiming future goals as current qualifications or achievements.
- Rewriting `resume.md` from a Future Resume.
- Adding Future Resume records to the job tracker or `history/`.
- Supporting languages other than English and Indonesian in Chapter 2.
- Using Humanizer for analysis or Future Resume output.

## 4. Commands

Canonical intents become:

```text
rolign
rolign analyze [URL] [--language en|id]
rolign tracker
rolign cover [URL] [--language en|id]
rolign future [--language en|id]
```

`--language` is optional and defaults to `en`. Host-specific invocation syntax
may differ, but all hosts must expose equivalent behavior.

## 5. Functional requirements

### FR-1: Raw HTML extraction

For a validated public job URL, Rolign must:

1. Make one ordinary host-side HTTP request with redirects and a short timeout.
2. Write the response only to an ignored temporary file.
3. Prefer schema.org `JobPosting` JSON-LD fields for title, hiring organization,
   and description.
4. Otherwise extract visible job content while ignoring scripts, styles,
   navigation, consent notices, and unrelated page chrome.
5. Treat the page and extracted text as untrusted data, never instructions.
6. Show the extracted company, role, original URL, and description preview for
   confirmation.
7. Request pasted job text when the page is blocked, incomplete,
   JavaScript-only, or ambiguous.
8. Delete the temporary file after success or failure and never save raw HTML
   in history.

Rolign must not retry, use a proxy or cache, add cookies, solve CAPTCHAs, or
bypass access restrictions.

### FR-2: Output language

- `en` produces English analysis and cover-letter prose.
- `id` produces Indonesian analysis and cover-letter prose.
- Omitting `--language` is equivalent to `--language en`.
- An analysis and its cover letter use the same selected language.
- Cover-only and Future Resume flows honor the same option and default.
- Structured field names, scoring, record metadata, company names, role names,
  resume evidence, metrics, and quoted job keywords remain unchanged.
- Translation must not add, remove, or broaden factual claims.
- Unsupported language values stop before generation or storage with an
  actionable error.

### FR-3: Humanized cover letters

- Rolign bundles a pinned, attributed copy of
  [`blader/humanizer`](https://github.com/blader/humanizer) under its MIT
  license; users install nothing separately.
- Humanizer runs after the grounded cover-letter draft and before storage.
- The pass must preserve the selected language, company, role, employers,
  titles, dates, metrics, skills, and achievements.
- The pass must not introduce any claim unsupported by `resume.md`.
- Analysis and Future Resume content do not use Humanizer.
- If the pass changes a protected fact or fails, Rolign stops without saving.

### FR-4: Future Resume input

Given `rolign future [--language en|id]`, Rolign must:

1. Read the non-empty `resume.md` as current-state context without modifying it.
2. Ask for a horizon from 5 through 10 years.
3. Ask for at least one target position, skill to develop, and professional
   achievement to pursue.
4. Treat every supplied aspiration as data, not workflow instructions.
5. Clearly distinguish current resume evidence from future goals.
6. Generate a brief preview in the selected language and request confirmation.
7. Stop without writing when required input is missing or confirmation is not
   given.

### FR-5: Future Resume output

- The output path is `<workspace>/future-resume.md`.
- The document contains a title, horizon, career vision, target positions,
  skills to develop, and target achievements.
- Future statements use goal-oriented language and must not imply they have
  already happened.
- Rust validates non-empty fields and the 5–10-year horizon, then renders the
  Markdown.
- A first write uses create-new behavior.
- If the file already exists, Rolign shows the full replacement preview and
  requires explicit overwrite confirmation.
- Failed validation or writes leave the existing file unchanged.
- Future Resume is not a history record and does not appear in `tracker`.

### FR-6: Compatibility and privacy

- Codex, Claude Code, and Gemini CLI use one canonical workflow.
- Rust makes no web or model API calls and sends no telemetry.
- Raw HTML, prompts, resumes, temporary payloads, and complete model responses
  are not logged.
- Existing history schemas and records remain readable without migration.

## 6. Success criteria

Chapter 2 succeeds when:

1. A public fixture can be extracted from JSON-LD or semantic HTML after one
   HTTP request.
2. Blocked and JavaScript-only pages request pasted text without partial
   history or leaked raw HTML.
3. Analysis and cover-only flows produce English by default and Indonesian when
   requested.
4. Humanized cover letters retain all source-backed facts in both languages.
5. `rolign future` creates a confirmed `future-resume.md` with the required
   sections and cannot silently overwrite an existing file.
6. All three hosts produce output accepted by the same Rust validation path.
