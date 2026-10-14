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

- [x] Change URL retrieval to write one `curl` response to an ignored temporary
      file instead of treating raw HTML as failure.
- [x] Enforce a 5 MiB response limit without printing partial HTML.
- [x] Extract schema.org `JobPosting` JSON-LD before inspecting visible content.
- [x] Fall back to visible semantic job content while ignoring scripts, styles,
      navigation, consent notices, and embedded instructions.
- [x] Preserve the original URL and existing confirmation preview.
- [x] Delete the temporary HTML after every success or failure path.
- [x] Request pasted text for blocked, incomplete, ambiguous, or
      JavaScript-only responses without retrying.

### Tests

- [x] Extract company, title, and complete description from JSON-LD.
- [x] Extract the same fields from semantic HTML.
- [x] Ignore page instructions and unrelated script content.
- [x] Verify invalid URLs make zero requests and valid URLs make one request.
- [x] Verify fallback creates no premature record or persistent raw HTML.
- [x] Verify oversized HTML stops safely and requests pasted text.

### Acceptance criteria

- [x] A raw HTML response containing structured or visible job content reaches
      confirmation instead of being reported as inaccessible.
- [x] Protected or client-rendered pages produce one clear pasted-text fallback.

## Phase 3: English and Indonesian output

### Tasks

- [x] Parse one optional language flag in the canonical workflow.
- [x] Default omitted language to `en`; reject unsupported or duplicate values.
- [x] Generate analysis and cover-letter prose in the selected language.
- [x] Keep scoring, structured field names, names, evidence, metrics, and quoted
      keywords unchanged.
- [x] Apply the same language behavior to manual, URL, and cover-only flows.

### Tests

- [x] Verify omitted language produces English analysis and cover letters.
- [x] Verify `--language id` produces Indonesian analysis and cover letters.
- [x] Verify both languages yield identical category scores for one fixture.
- [x] Verify translation adds no resume claim or job requirement.
- [x] Verify invalid language stops before saving.

### Acceptance criteria

- [x] English and Indonesian workflows produce records accepted by the existing
      Rust save path.
- [x] Existing commands without language flags retain English behavior.

## Phase 4: Humanized cover letters

### Tasks

- [x] Vendor a pinned copy of `blader/humanizer` with its MIT license,
      attribution, upstream URL, and revision.
- [x] Make the canonical workflow read the bundled Humanizer instructions.
- [x] Run Humanizer only after the grounded cover-letter draft.
- [x] Provide the selected language and protected facts to the editing pass.
- [x] Delimit draft text as data and forbid Humanizer-triggered tools or file
      access.
- [x] Compare protected facts before and after; stop without saving on changes
      or unsupported additions.
- [x] Keep analysis and Future Resume outside the Humanizer pass.

### Tests

- [x] Verify users need no global Humanizer installation.
- [x] Verify English and Indonesian cover letters retain language and facts.
- [x] Reject an edited draft that changes a name, role, date, metric, skill, or
      achievement.
- [x] Verify analysis and Future Resume bypass Humanizer.

### Acceptance criteria

- [x] Every saved cover letter has passed Humanizer and protected-fact checks.
- [x] Humanizer failure creates no history record.

## Phase 5: Future Resume domain and storage

### Tasks

- [x] Add typed language and Future Resume request models.
- [x] Validate a 5–10-year integer horizon, non-empty vision, and non-empty
      target-position, skill, and achievement lists.
- [x] Enforce 2,000-byte career vision, 500-byte list item, 20-item list, and
      control-character limits in Rust.
- [x] Add `save-future [--replace] [--workspace PATH]` to the Rust CLI.
- [x] Render localized Markdown headings and goal-oriented content.
- [x] Escape Future Resume text so it cannot add raw HTML or alter required
      Markdown structure.
- [x] Write `<workspace>/future-resume.md` through a sibling temporary file.
- [x] Refuse an existing destination unless `--replace` is present.
- [x] Preserve the previous complete file if validation or replacement fails.
- [x] Keep Future Resume outside `history/` and tracker output.

### Tests

- [x] Validate both languages and horizon boundaries 5 and 10.
- [x] Reject horizons below 5 or above 10 and every empty required field.
- [x] Render complete English and Indonesian Markdown fixtures.
- [x] Create a new file without `--replace`.
- [x] Reject an existing file without changing it.
- [x] Replace only when explicitly requested and preserve the old file on
      simulated failure.
- [x] Verify paths remain within the workspace and `resume.md` never changes.
- [x] Reject control characters, over-limit values, Markdown/HTML injection,
      and path-like or command-like text without executing it.

### Acceptance criteria

- [x] Rust can safely create and explicitly replace one `future-resume.md`.
- [x] No partial document, history record, or resume modification occurs.

## Phase 6: Future Resume host workflow

### Tasks

- [x] Route `rolign future [--language en|id] [--] [VISION...]` through every host
      adapter.
- [x] Read and validate `resume.md` as current-state context.
- [x] Without a vision prompt, collect horizon, target positions, skills to
      develop, and target achievements through guided questions.
- [x] With a vision prompt, extract explicitly stated details and ask only for
      missing or ambiguous required fields.
- [x] Show interpreted inputs for correction before drafting.
- [x] Draft aspirations as future goals, never current claims.
- [x] Show the complete Markdown preview and request write confirmation.
- [x] If the destination exists, request a separate explicit overwrite
      confirmation before using `--replace`.
- [x] Submit the confirmed payload through the Rust save path and report the
      resulting file.

### Tests

- [x] Complete English and Indonesian Future Resume flows with synthetic data.
- [x] Verify a complete vision prompt avoids unnecessary questions.
- [x] Verify a partial vision prompt asks only for missing required details.
- [x] Verify prompt text cannot become workflow instructions and missing goals
      are never invented.
- [x] Verify `--` and first-non-option parsing keep option-like vision text as
      data and reject unknown leading options.
- [x] Stop without writing when required input or confirmation is missing.
- [x] Verify current resume facts and future aspirations remain distinct.
- [x] Verify Humanizer is not invoked.
- [x] Verify Codex, Claude Code, and Gemini CLI use the canonical workflow.

### Acceptance criteria

- [x] A user can create a brief Future Resume in either language without
      modifying `resume.md`.
- [x] Existing output can never be overwritten silently.

## Phase 7: Chapter 2 hardening

### Tasks

- [x] Run the full automated verification suite on a fresh clone.
- [ ] Smoke-test help, URL/manual analysis, cover-only, tracker, record opening,
      and Future Resume on all three authenticated hosts.
- [ ] Verify English defaults and Indonesian selection on every generating
      command.
- [x] Audit temporary files, Git ignores, logs, fixtures, and saved artifacts
      for sensitive content.
- [ ] Run the full adversarial free-text suite against every host and Rust trust
      boundary.
- [x] Update README commands, privacy notes, limitations, and Humanizer
      attribution.
- [x] Record host versions and verification evidence.

### Acceptance criteria

- [ ] Every Chapter 2 PRD success criterion has test or smoke-test evidence.
- [x] Existing Chapter 1 records remain readable.
- [x] A fresh user needs only stable Rust and one authenticated supported host.
- [x] No real resume, job posting, aspiration, credential, or raw HTML is
      committed or logged.

### Future Resume refinement

- [x] Default sparse Future Resume horizons to five years.
- [x] Resolve a Product/Project Manager vision to Product Manager primary role.
- [x] Recommend editable role-relevant capabilities and achievements.
- [x] Render a localized resume-style future profile without current-fact claims.
- [x] Expand the profile and recommendations with practical career-advisor
      rationale and development guidance.
- [x] Add non-sensitive progress stages and tracker record-opening guidance.
- [x] Complete Codex user smoke testing; defer Claude Code and Gemini CLI smoke
      testing to the remaining release gate.
