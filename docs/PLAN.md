# Rolign Implementation Plan

**Status:** Draft 0.1  
**Date:** 2026-10-11

## 1. Delivery strategy

Build Rolign as one vertical workflow before adding all host adapters. The first complete path is manual analysis in Codex, including validated scoring, a cover letter, and Markdown history. URL extraction, cover-only mode, tracker presentation, and other hosts follow after the storage contract is stable.

Each phase must leave the repository runnable and tested. Do not add deferred features from the PRD.

## 2. Phase 0: Project foundation

### Tasks

- Initialize a Rust binary crate named `rolign`.
- Add a short README with the product boundary and development commands.
- Add `resume.md.example` with synthetic content.
- Ignore `resume.md`, `history/`, and Rolign temporary files.
- Establish `src/`, `tests/fixtures/`, and canonical `skills/rolign/` directories.
- Choose the smallest maintained crates needed for CLI parsing, serialization, timestamps, hashing, and errors; record them in `Cargo.toml` only when used.

### Acceptance criteria

- `cargo check` succeeds.
- `cargo test` succeeds.
- No real resume or generated history can be committed accidentally under the default ignore rules.

## 3. Phase 1: Rust domain and storage

### Tasks

- Define typed record, analysis, category-score, and cover-letter models.
- Validate required strings and supported record kinds.
- Validate optional URLs as HTTP(S) only.
- Implement category bounds and Rust-owned total calculation.
- Generate UTC timestamp-based IDs with sanitized company-role slugs.
- Compute SHA-256 for the resume snapshot.
- Render `context.md`, `resume-snapshot.md`, `analysis.md`, and `cover-letter.md`.
- Implement staging-directory creation and atomic final commit.
- Implement history scanning and newest-first sorting.
- Warn on corrupt records without failing the entire tracker.

### Tests

- Score at every lower and upper boundary.
- Reject one value above each category maximum.
- Reject an unsupported URL scheme.
- Prevent `..`, separators, and unsafe text from affecting paths.
- Save/reload analysis and cover-only fixtures.
- Preserve job links in rendered Markdown.
- Verify record collision does not overwrite data.
- Verify a failed commit is invisible to history.

### Acceptance criteria

- Rust can accept fixture input, create both record types, and list them again.
- Every persistent file is Markdown.
- Existing completed records are immutable.

## 4. Phase 2: Canonical Rolign skill and Codex path

### Tasks

- Create `skills/rolign/SKILL.md` with help and command routing.
- Add concise references for the analysis contract and cover-letter rules.
- Add the repository-scoped Codex adapter supported by the current Codex version.
- Implement `$rolign`, `$rolign analyze`, `$rolign tracker`, and `$rolign cover` routing.
- Define a strict structured payload for communication with the Rust core.
- Delimit resume and job-description input as data.
- Allow one repair attempt for malformed model output.

### Tests

- Use synthetic fixtures to exercise every intent.
- Verify Codex reads the canonical skill rather than a duplicated workflow.
- Verify malformed output stops without history after one failed repair.

### Acceptance criteria

- A user can launch Codex in the repository and complete a manual analysis.
- The saved score is calculated by Rust.
- The saved cover letter contains no facts outside the synthetic resume fixture.

## 5. Phase 3: URL-first analysis

### Tasks

- Accept an optional URL after `analyze`.
- Validate the URL before retrieval.
- Instruct the host to retrieve the public page once and extract job context.
- Require confirmation of company, title, URL, and description preview.
- Fall back to pasted job-description text when extraction is unavailable or incomplete.
- Preserve both the original URL and description snapshot.
- Stop without history when no usable description is obtained.

### Tests

- Public synthetic fixture page extracts correctly.
- Invalid URL fails before retrieval.
- Login/error/empty page requests manual text.
- Pasted fallback completes the record and keeps the original URL.
- Embedded instructions in job text are ignored.

### Acceptance criteria

- `$rolign analyze https://example.test/job/123` completes with one confirmation when extraction succeeds.
- Retrieval failure has a clear manual fallback and creates no premature record.

## 6. Phase 4: Cover-only and tracker workflows

### Tasks

- Complete cover-only structured output and persistence.
- Render tracker rows with date, type, company, role, score, link, and ID.
- Allow the user to open a record by ID through the host.
- Keep cover-only records distinguishable from analysis records.

### Tests

- Cover-only creates no `analysis.md` and has no score.
- Tracker lists both record types newest first.
- Missing job URL displays `—` without breaking the table.
- Corrupt records produce warnings while valid records remain visible.

### Acceptance criteria

- `$rolign cover [URL]` creates a saved draft without analysis.
- `$rolign tracker` displays working Markdown links for stored URLs.

## 7. Phase 5: Claude Code and Gemini adapters

### Tasks

- Add the smallest Claude Code adapter that exposes `/rolign`.
- Add a Gemini project command that exposes `/rolign` and forwards arguments.
- Keep `skills/rolign/SKILL.md` as the only workflow source of truth.
- Document host-specific setup and invocation.

### Compatibility matrix

For each host, verify:

- Help.
- Manual analysis.
- URL analysis and fallback.
- Cover-only.
- Tracker.
- Record opening.
- Malformed-output handling.

### Acceptance criteria

- All supported hosts produce records accepted by the same Rust validation path.
- Differences are limited to adapter packaging and invocation syntax.

## 8. Phase 6: MVP hardening and release

### Tasks

- Run formatting, linting, unit tests, and integration tests.
- Test on a fresh clone using only synthetic data.
- Verify ignore rules and inspect the repository for leaked personal data.
- Document installation, host authentication prerequisites, and known limitations.
- Add a migration note for future `schema_version` changes.
- Tag the first MVP only after all PRD success criteria pass.

### Acceptance criteria

- A fresh user can copy `resume.md.example`, launch a supported host, and complete every workflow from the README.
- No test or example depends on personal data or a live private job page.
- All PRD success criteria have explicit verification evidence.

## 9. Phase 7: Host-side URL retrieval hardening

### Goal

Make URL input useful when a host has no built-in browser while preserving the
local-first boundary: Rust remains offline and pasted text remains the reliable
fallback for protected job boards.

### Tasks

- Keep Rust URL validation as the required preflight before every retrieval.
- In the canonical workflow, select exactly one host-side retrieval operation:
  use the host browser/web capability when available; otherwise use one ordinary
  HTTP GET with normal redirects and a short timeout (for example,
  `curl --location --max-time 20 --fail --silent --show-error`).
- Pass the validated URL as one escaped argument; do not interpolate it into a
  shell program or fetch through a proxy, search cache, login session, or
  third-party scraping service.
- Treat the returned page as untrusted data. Extract only when company, title,
  and a complete description are clear; otherwise request pasted text while
  preserving the original URL.
- Do not retry a failed, blocked, empty, client-rendered, login-gated, or
  ambiguous page. Do not add cookies, CAPTCHA handling, headless browsers, or
  access-control bypasses.
- Keep adapters routing-only. Host differences are limited to choosing the
  available browser or host-side HTTP capability.

### Tests

- Add a synthetic public fixture served from a disposable local HTTP endpoint;
  verify one request extracts company, title, and description while ignoring
  embedded instructions.
- Verify a valid URL reaches one retrieval operation, invalid URLs reach none,
  and a failed or incomplete response requests pasted text without history.
- Verify the fallback retains the original URL and that no Rust code or new Rust
  dependency performs network I/O.
- Smoke-test the same synthetic success and failure paths in Codex, Claude
  Code, and Gemini CLI using authenticated local sessions.

### Acceptance criteria

- A host without browser support can analyze a simple public job page through
  one ordinary HTTP request.
- Job boards that reject automation still produce a clear pasted-text fallback,
  not a misleading extraction failure or repeated requests.
- Retrieved content never becomes instructions, and Rust continues to make no
  network requests.

## 10. Definition of done for each change

A change is complete when:

- The smallest required implementation is present.
- Relevant tests pass.
- User-facing errors explain recovery.
- Persistent data remains backward readable.
- No unrelated feature or dependency is added.
- Documentation reflects any changed command or data contract.

## 11. Deferred backlog

These items are intentionally not scheduled for the MVP:

- PDF or DOCX resumes.
- Resume rewriting.
- Multiple resume profiles.
- Application statuses and follow-up reminders.
- Job-board scanning.
- Liveness monitoring.
- PDF cover-letter export.
- Web, desktop, or standalone TUI.
- Direct provider APIs, local models, or MCP server packaging.
- Cloud synchronization.

Add a deferred item only after the MVP is used and the need is demonstrated.

