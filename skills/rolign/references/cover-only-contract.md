# Cover-only save contract

Send one JSON object to `rolign save` on standard input for a confirmed cover
letter. Rust rejects unknown fields.

```json
{
  "kind": "cover-letter-only",
  "company": "non-empty company name",
  "job_title": "non-empty title",
  "job_url": "https://example.test/job-or-null",
  "job_description": "confirmed complete job description",
  "ai_host": "codex",
  "analysis": null,
  "cover_letter": { "body": "draft cover letter" }
}
```

Rules:

- Read `output-language-rules.md`. `cover_letter.body` uses the selected
  language; an omitted language option means English.
- `kind` must be `cover-letter-only`; `ai_host` must match the active adapter:
  `codex`, `claude`, or `gemini`.
- `company`, `job_title`, `job_description`, and `cover_letter.body` must be
  non-empty.
- `job_url` is `null` or a valid HTTP(S) URL.
- `analysis` must be `null`; do not include categories, a score, or analysis
  findings.
