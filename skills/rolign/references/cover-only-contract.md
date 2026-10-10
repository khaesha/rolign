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

- `kind` must be `cover-letter-only`; `ai_host` must be `codex`.
- `company`, `job_title`, `job_description`, and `cover_letter.body` must be
  non-empty.
- `job_url` is `null` or a valid HTTP(S) URL.
- `analysis` must be `null`; do not include categories, a score, or analysis
  findings.
