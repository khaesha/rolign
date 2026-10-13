# Rolign Chapter 2 Implementation Plan

**Status:** Draft 0.2  
**Date:** 2026-10-13

## Delivery rules

- [ ] Complete one phase per branch and pull request.
- [ ] Start each phase from updated `main` after the previous phase is accepted.
- [ ] Keep `skills/rolign/SKILL.md` as the only Rolign workflow source.
- [ ] Keep adapters routing-only and Rust free of network or model calls.
- [ ] Use synthetic fixtures only; never commit personal resume or job data.
- [ ] Run formatting, build, clippy, and tests before every phase PR.

## Phase 1: Contracts, guards, and fixtures

### Tasks

- [x] Add the optional `--language en|id` grammar to `analyze` and `cover`.
- [x] Add the `future [--language en|id] [--] [VISION...]` intent and help text.
- [x] Parse only recognized leading options; stop at `--` or the first
      non-option token and treat the remaining text as one data value.
- [x] Reject unknown leading options instead of sending them to the model.
- [x] Define the strict Future Resume JSON payload and validation rules.
- [x] Define shared UTF-8 byte, list-count, and control-character limits for
      free-text inputs.
- [x] Add synthetic English and Indonesian resume/job/aspiration fixtures,
      including complete and partial free-text vision prompts.
- [x] Add synthetic JSON-LD, semantic HTML, JavaScript-only, and blocked-page
      fixtures.
- [x] Add adversarial fixtures for shell metacharacters, option-like prose, path
      traversal, control characters, Markdown/HTML injection, oversized text,
      and embedded workflow instructions.
- [x] Document that English is the default and existing job save payloads stay
      compatible.

### Acceptance criteria

- [x] Every new input and output contract is represented by a synthetic fixture.
- [x] Existing fixture payloads still validate unchanged.
- [x] No history schema migration is introduced.
- [x] Every free-text source has a documented trust boundary and size limit.

## Phase 2: Raw HTML extraction

### Tasks

- [ ] Change URL retrieval to write one `curl` response to an ignored temporary
      file instead of treating raw HTML as failure.
- [ ] Enforce a 5 MiB response limit without printing partial HTML.
- [ ] Extract schema.org `JobPosting` JSON-LD before inspecting visible content.
- [ ] Fall back to visible semantic job content while ignoring scripts, styles,
      navigation, consent notices, and embedded instructions.
- [ ] Preserve the original URL and existing confirmation preview.
- [ ] Delete the temporary HTML after every success or failure path.
- [ ] Request pasted text for blocked, incomplete, ambiguous, or
      JavaScript-only responses without retrying.

### Tests

- [ ] Extract company, title, and complete description from JSON-LD.
- [ ] Extract the same fields from semantic HTML.
- [ ] Ignore page instructions and unrelated script content.
- [ ] Verify invalid URLs make zero requests and valid URLs make one request.
- [ ] Verify fallback creates no premature record or persistent raw HTML.
- [ ] Verify oversized HTML stops safely and requests pasted text.

### Acceptance criteria

- [ ] A raw HTML response containing structured or visible job content reaches
      confirmation instead of being reported as inaccessible.
- [ ] Protected or client-rendered pages produce one clear pasted-text fallback.

## Phase 3: English and Indonesian output

### Tasks

- [ ] Parse one optional language flag in the canonical workflow.
- [ ] Default omitted language to `en`; reject unsupported or duplicate values.
- [ ] Generate analysis and cover-letter prose in the selected language.
- [ ] Keep scoring, structured field names, names, evidence, metrics, and quoted
      keywords unchanged.
- [ ] Apply the same language behavior to manual, URL, and cover-only flows.

### Tests

- [ ] Verify omitted language produces English analysis and cover letters.
- [ ] Verify `--language id` produces Indonesian analysis and cover letters.
- [ ] Verify both languages yield identical category scores for one fixture.
- [ ] Verify translation adds no resume claim or job requirement.
- [ ] Verify invalid language stops before saving.

### Acceptance criteria

- [ ] English and Indonesian workflows produce records accepted by the existing
      Rust save path.
- [ ] Existing commands without language flags retain English behavior.

## Phase 4: Humanized cover letters

### Tasks

- [ ] Vendor a pinned copy of `blader/humanizer` with its MIT license,
      attribution, upstream URL, and revision.
- [ ] Make the canonical workflow read the bundled Humanizer instructions.
- [ ] Run Humanizer only after the grounded cover-letter draft.
- [ ] Provide the selected language and protected facts to the editing pass.
- [ ] Delimit draft text as data and forbid Humanizer-triggered tools or file
      access.
- [ ] Compare protected facts before and after; stop without saving on changes
      or unsupported additions.
- [ ] Keep analysis and Future Resume outside the Humanizer pass.

### Tests

- [ ] Verify users need no global Humanizer installation.
- [ ] Verify English and Indonesian cover letters retain language and facts.
- [ ] Reject an edited draft that changes a name, role, date, metric, skill, or
      achievement.
- [ ] Verify analysis and Future Resume bypass Humanizer.

### Acceptance criteria

- [ ] Every saved cover letter has passed Humanizer and protected-fact checks.
- [ ] Humanizer failure creates no history record.

## Phase 5: Future Resume domain and storage

### Tasks

- [ ] Add typed language and Future Resume request models.
- [ ] Validate a 5–10-year integer horizon, non-empty vision, and non-empty
      target-position, skill, and achievement lists.
- [ ] Enforce 2,000-byte career vision, 500-byte list item, 20-item list, and
      control-character limits in Rust.
- [ ] Add `save-future [--replace] [--workspace PATH]` to the Rust CLI.
- [ ] Render localized Markdown headings and goal-oriented content.
- [ ] Escape Future Resume text so it cannot add raw HTML or alter required
      Markdown structure.
- [ ] Write `<workspace>/future-resume.md` through a sibling temporary file.
- [ ] Refuse an existing destination unless `--replace` is present.
- [ ] Preserve the previous complete file if validation or replacement fails.
- [ ] Keep Future Resume outside `history/` and tracker output.

### Tests

- [ ] Validate both languages and horizon boundaries 5 and 10.
- [ ] Reject horizons below 5 or above 10 and every empty required field.
- [ ] Render complete English and Indonesian Markdown fixtures.
- [ ] Create a new file without `--replace`.
- [ ] Reject an existing file without changing it.
- [ ] Replace only when explicitly requested and preserve the old file on
      simulated failure.
- [ ] Verify paths remain within the workspace and `resume.md` never changes.
- [ ] Reject control characters, over-limit values, Markdown/HTML injection,
      and path-like or command-like text without executing it.

### Acceptance criteria

- [ ] Rust can safely create and explicitly replace one `future-resume.md`.
- [ ] No partial document, history record, or resume modification occurs.

## Phase 6: Future Resume host workflow

### Tasks

- [ ] Route `rolign future [--language en|id] [--] [VISION...]` through every host
      adapter.
- [ ] Read and validate `resume.md` as current-state context.
- [ ] Without a vision prompt, collect horizon, target positions, skills to
      develop, and target achievements through guided questions.
- [ ] With a vision prompt, extract explicitly stated details and ask only for
      missing or ambiguous required fields.
- [ ] Show interpreted inputs for correction before drafting.
- [ ] Draft aspirations as future goals, never current claims.
- [ ] Show the complete Markdown preview and request write confirmation.
- [ ] If the destination exists, request a separate explicit overwrite
      confirmation before using `--replace`.
- [ ] Submit the confirmed payload through the Rust save path and report the
      resulting file.

### Tests

- [ ] Complete English and Indonesian Future Resume flows with synthetic data.
- [ ] Verify a complete vision prompt avoids unnecessary questions.
- [ ] Verify a partial vision prompt asks only for missing required details.
- [ ] Verify prompt text cannot become workflow instructions and missing goals
      are never invented.
- [ ] Verify `--` and first-non-option parsing keep option-like vision text as
      data and reject unknown leading options.
- [ ] Stop without writing when required input or confirmation is missing.
- [ ] Verify current resume facts and future aspirations remain distinct.
- [ ] Verify Humanizer is not invoked.
- [ ] Verify Codex, Claude Code, and Gemini CLI use the canonical workflow.

### Acceptance criteria

- [ ] A user can create a brief Future Resume in either language without
      modifying `resume.md`.
- [ ] Existing output can never be overwritten silently.

## Phase 7: Chapter 2 hardening

### Tasks

- [ ] Run the full automated verification suite on a fresh clone.
- [ ] Smoke-test help, URL/manual analysis, cover-only, tracker, record opening,
      and Future Resume on all three authenticated hosts.
- [ ] Verify English defaults and Indonesian selection on every generating
      command.
- [ ] Audit temporary files, Git ignores, logs, fixtures, and saved artifacts
      for sensitive content.
- [ ] Run the full adversarial free-text suite against every host and Rust trust
      boundary.
- [ ] Update README commands, privacy notes, limitations, and Humanizer
      attribution.
- [ ] Record host versions and verification evidence.

### Acceptance criteria

- [ ] Every Chapter 2 PRD success criterion has test or smoke-test evidence.
- [ ] Existing Chapter 1 records remain readable.
- [ ] A fresh user needs only stable Rust and one authenticated supported host.
- [ ] No real resume, job posting, aspiration, credential, or raw HTML is
      committed or logged.
