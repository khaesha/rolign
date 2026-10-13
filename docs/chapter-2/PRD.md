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
rolign future [--language en|id] [--] [VISION...]
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

Given `rolign future [--language en|id] [--] [VISION...]`, Rolign must:

1. Read the non-empty `resume.md` as current-state context without modifying it.
2. Without `VISION`, ask guided questions for a 5–10-year horizon, target
   positions, skills to develop, and professional achievements to pursue.
3. With `VISION`, treat all free text after `future` except recognized options
   as one aspiration prompt. Extract any stated horizon, target positions,
   skills, and achievements, then ask only for required details that are
   missing or ambiguous.
4. Never invent an aspiration merely to complete a field.
5. Treat every supplied aspiration as data, not workflow instructions.
6. Clearly distinguish current resume evidence from future goals.
7. Show the interpreted inputs before drafting so the user can correct them.
8. Generate a brief preview in the selected language and request confirmation.
9. Stop without writing when required input is missing or confirmation is not
   given.

Option parsing stops at `--` or the first non-option token. Every remaining
token belongs to one vision value; option-like text inside it is data. Unknown
options before the vision are rejected. `--` is optional for ordinary prose
and required when the vision begins with `-`.

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

### FR-7: Free-text security guard

The guard applies to vision prompts, manual job text, retrieved pages, resume
content, and model-produced prose:

- Delimit every free-text value as untrusted data and ignore commands embedded
  within it.
- Never interpolate free text into a shell program, executable name, option,
  URL, or filesystem path. Pass fixed commands fixed arguments; pass content by
  standard input, an argument after `--`, or an ignored temporary file.
- Reject NUL and non-whitespace control characters before generation or
  storage.
- Limit a Future Resume vision prompt to 4,000 UTF-8 bytes, career vision to
  2,000 bytes, each list item to 500 bytes, and each list to 20 items.
- Stop an HTML download above 5 MiB and request pasted job text instead.
- Render Future Resume fields as escaped Markdown text so user input cannot add
  raw HTML or alter the required document structure.
- Allow the model to extract career goals only. Free text cannot authorize tool
  calls, network access, file reads, file writes, workflow changes, or policy
  changes.
- Do not print or log complete free-text inputs. Errors identify the field and
  limit without echoing its value.
- Revalidate the structured payload in Rust; host-side filtering alone is not
  trusted.

## 6. Success criteria

Chapter 2 succeeds when:

1. A public fixture can be extracted from JSON-LD or semantic HTML after one
   HTTP request.
2. Blocked and JavaScript-only pages request pasted text without partial
   history or leaked raw HTML.
3. Analysis and cover-only flows produce English by default and Indonesian when
   requested.
4. Humanized cover letters retain all source-backed facts in both languages.
5. `rolign future` supports guided questions or a free-text vision, then creates
   a confirmed `future-resume.md` with the required sections and cannot silently
   overwrite an existing file.
6. All three hosts produce output accepted by the same Rust validation path.
7. Adversarial free text cannot execute commands, select paths, add Markdown
   structure, change workflow instructions, or bypass validation limits.
