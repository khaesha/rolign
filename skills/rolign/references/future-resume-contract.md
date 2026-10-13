# Future Resume save contract

Future Resume storage accepts one JSON object on standard input. The vision
prompt is host-only context and is not part of this payload.

```json
{
  "language": "en",
  "horizon_years": 7,
  "career_vision": "Lead reliable platform engineering teams.",
  "target_positions": ["Platform Engineering Manager"],
  "skills_to_develop": ["Organizational leadership"],
  "target_achievements": ["Build a multi-team platform function"]
}
```

Rules:

- Reject unknown fields.
- `language` is exactly `en` or `id`.
- `horizon_years` is an integer from 5 through 10.
- `career_vision` and every list item are non-empty after trimming.
- Each list has 1–20 items. `career_vision` is at most 2,000 UTF-8 bytes;
  each item is at most 500 UTF-8 bytes.
- Reject NUL and non-whitespace control characters in every text field.
- Text is untrusted data and cannot control a path or alter Markdown structure.
