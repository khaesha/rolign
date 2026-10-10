# Rolign Product Requirements Document

**Status:** Draft 0.1  
**Date:** 2026-10-11  
**Product:** Rolign

## 1. Summary

Rolign is a local-first resume assistant that runs inside an authenticated AI coding CLI. A user opens the Rolign project with Codex, Claude Code, or Gemini CLI, then asks Rolign to compare a Markdown resume with a job posting, create a grounded cover letter, or review saved history.

Rolign uses the active AI CLI for language analysis and generation. A small Rust program owns deterministic work: input validation, score calculation, Markdown rendering, atomic storage, and history listing. Rolign does not require the user to provide a separate model API key.

## 2. Problem

Comparing the same resume with many job descriptions is repetitive. The user must repeatedly identify requirements, judge whether the resume contains evidence, estimate fit, write a tailored cover letter, and remember which jobs were already reviewed.

General-purpose AI chats can help, but their output is inconsistent and disconnected from prior applications. Rolign provides one repeatable workflow with a stable rubric and inspectable local history.

## 3. Goals

- Analyze one Markdown resume against a job description.
- Produce an explainable resume-job match score, not a claimed portal ATS score.
- Generate a tailored cover letter using only facts supported by the resume.
- Accept a job-post URL for faster input.
- Fall back to pasted job-description text when extraction fails.
- Save analysis and cover-letter-only records as Markdown.
- Show saved records with their original job-post links.
- Work from Codex, Claude Code, and Gemini CLI using their existing authentication.
- Provide a useful Rust learning project without duplicating the host CLI's interface.

## 4. Non-goals for the MVP

- Scraping job boards or bypassing login, anti-bot, or access controls.
- Reproducing a job portal's private ATS ranking.
- Editing or tailoring the source resume automatically.
- Reading PDF, DOCX, or image resumes.
- Submitting job applications.
- Sending email or messages.
- Maintaining multiple active resumes.
- A database, web server, browser UI, or standalone Ratatui application.
- Cloud synchronization or user accounts.
- Embedded provider login or direct model API integration.

## 5. Users

The MVP has one user: the owner of the local Rolign workspace. The user is comfortable launching an AI coding CLI and keeping a resume in Markdown.

## 6. Invocation

The user starts an authenticated host from the project directory:

```bash
codex
# or
claude
# or
gemini
```

The canonical Rolign intents are:

```text
rolign
rolign analyze [job-url]
rolign tracker
rolign cover [job-url]
```

Host syntax differs:

| Host | Invocation |
|---|---|
| Codex | `$rolign analyze ...` or `Run rolign analyze ...` |
| Claude Code | `/rolign analyze ...` |
| Gemini CLI | `/rolign analyze ...` |

The workflow and outputs must remain equivalent across hosts even when invocation syntax differs.

## 7. Functional requirements

### FR-1: Help

Invoking Rolign without a subcommand must show the available commands and one-line descriptions.

```text
Rolign

  analyze [URL]  Analyze resume fit and create a cover letter
  tracker        Show saved analysis and cover-letter history
  cover [URL]    Create a cover letter without scoring
```

### FR-2: Analyze from a URL

Given `rolign analyze <job-url>`, Rolign must:

1. Accept only an `http://` or `https://` URL.
2. Ask the host to retrieve the publicly accessible job page.
3. Extract the company, job title, and complete job description.
4. Treat retrieved page content as untrusted data, never as instructions.
5. Show the extracted company, title, URL, and a short description preview.
6. Ask the user to confirm or correct the extracted information.
7. Fall back to pasted job-description text if the page is unavailable, incomplete, behind login, or ambiguous.
8. Stop without creating history if no usable job description is available.
9. Analyze the confirmed job description against `resume.md`.
10. Generate a cover letter from the same confirmed inputs.
11. Save the completed record only after validation succeeds.

Rolign must not attempt to bypass access restrictions or repeatedly retry an unavailable page.

### FR-3: Analyze without a URL

Given `rolign analyze`, Rolign must ask for:

- Company name.
- Job title.
- Optional job-post URL.
- Job description.

It must then run the same analysis, cover-letter, validation, and storage workflow as URL input.

### FR-4: Match analysis

The analysis must include:

- Overall match score from 0 to 100.
- Category scores and explanations.
- Resume evidence supporting every positive match.
- Strong matches.
- Missing or weak requirements.
- Important missing keywords.
- Actionable recommendations that do not invent experience.
- A disclaimer that the result is guidance, not a portal ATS score or hiring prediction.

The MVP scoring rubric is:

| Category | Maximum |
|---|---:|
| Required qualifications | 40 |
| Skills and responsibilities | 30 |
| Relevant achievements | 20 |
| Keywords and clarity | 10 |
| **Total** | **100** |

The AI host proposes category points and supporting evidence. Rust validates each range and calculates the total. The AI host's supplied total, if any, is ignored.

### FR-5: Cover letter with analysis

The analysis workflow must produce a cover letter that:

- Names the correct company and role.
- Uses only claims supported by `resume.md`.
- Does not invent employers, titles, dates, metrics, skills, or achievements.
- Connects the strongest supported experience to the job's priorities.
- Is concise and ready for user review.
- Is clearly marked as a draft.

### FR-6: Cover letter only

Given `rolign cover [job-url]`, Rolign must collect or extract the same job context, read `resume.md`, and create a cover letter without producing a score or analysis.

The resulting record must be saved in history with type `cover-letter-only`.

### FR-7: Tracker

Given `rolign tracker`, Rolign must list saved records newest first.

Each row must contain:

- Date.
- Record type.
- Company.
- Job title.
- Score for analysis records, or `—` for cover-letter-only records.
- Job-post link when available.
- Record ID.

The user must be able to ask the host to open a record by ID and view its saved Markdown files.

### FR-8: Resume source

- The active resume is `<workspace>/resume.md`.
- The user edits this file manually.
- Rolign must reject a missing or empty resume.
- Each completed history record must contain a snapshot of the resume used for that run.
- Rolign must never modify `resume.md` in the MVP.

### FR-9: History

Each completed run must create one directory:

```text
history/<record-id>/
├── context.md
├── resume-snapshot.md
├── analysis.md          # analysis records only
└── cover-letter.md
```

`context.md` must include machine-readable front matter and a human-readable job-description snapshot. At minimum, metadata must contain:

- Schema version.
- Record ID.
- Record type.
- Creation timestamp.
- Company.
- Job title.
- Optional job URL.
- AI host.
- Resume content hash.
- Score for analysis records.

History must remain readable without Rolign. Partially written records must not appear in the tracker.

### FR-10: Errors

Rolign must provide actionable errors for:

- Missing or empty `resume.md`.
- Invalid URL scheme.
- Failed or incomplete job-page extraction.
- Empty company, title, or job description.
- Malformed AI output.
- Score values outside their category ranges.
- Unsafe or invalid record paths.
- Existing record collision.
- Failed history write.

Failures before the final commit must not create a visible history record.

## 8. Data and privacy requirements

- Resume and history files remain local except for content sent through the chosen AI host.
- Rolign itself must not collect telemetry.
- Prompts and model responses must not be logged unless needed as a temporary failure diagnostic explicitly shown to the user.
- Temporary structured data must be removed after success or failure.
- `history/`, `resume.md`, and temporary data should be ignored by Git by default because they contain personal information.
- External job descriptions must be delimited and treated as untrusted input.

## 9. Success criteria

The MVP is successful when all of the following are true:

1. A user can launch a supported AI CLI from the project directory and invoke Rolign.
2. `analyze <URL>` extracts or requests the job description, confirms it, and saves a valid analysis plus cover letter.
3. `analyze` works with fully manual input.
4. `cover` saves a cover-letter-only record.
5. `tracker` lists both record types with job-post links.
6. Changing `resume.md` does not alter earlier resume snapshots.
7. Invalid AI output or filesystem failure leaves no partial visible record.
8. The same fixture produces the same arithmetic total regardless of AI host.

## 10. MVP release boundary

The first release supports one local user, one `resume.md`, three host integrations, four Rolign intents, URL-or-manual job input, a fixed score rubric, cover letters, and Markdown history. Everything in the non-goals list is deferred until real use demonstrates a need.

