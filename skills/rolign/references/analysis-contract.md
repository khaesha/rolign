# Analysis save contract

Send one JSON object to `rolign save` on standard input. Rust rejects unknown
fields and recalculates the score; never include a `total` key.

```json
{
  "kind": "analysis",
  "company": "non-empty company name",
  "job_title": "non-empty title",
  "job_url": "https://example.test/job-or-null",
  "job_description": "confirmed complete job description",
  "ai_host": "codex",
  "analysis": {
    "categories": {
      "required_qualifications": 0,
      "skills_and_responsibilities": 0,
      "relevant_achievements": 0,
      "keywords_and_clarity": 0
    },
    "strengths": [{ "finding": "match", "evidence": "resume evidence" }],
    "gaps": [{ "finding": "gap", "explanation": "why it is weak or missing" }],
    "missing_keywords": ["keyword"],
    "recommendations": ["grounded next step"]
  },
  "cover_letter": { "body": "draft cover letter" }
}
```

Rules:

- `kind` must be `analysis`; `ai_host` must match the active adapter:
  `codex`, `claude`, or `gemini`.
- `company`, `job_title`, `job_description`, and `cover_letter.body` must be
  non-empty.
- `job_url` is `null` or a valid HTTP(S) URL.
- Category ranges are required qualifications 0–40, skills and
  responsibilities 0–30, relevant achievements 0–20, and keywords and
  clarity 0–10.
- Every strength has non-empty `finding` and resume-backed `evidence`. Every
  gap has non-empty `finding` and `explanation`. List entries cannot be empty.
- Empty lists are allowed when the confirmed evidence supports none.
