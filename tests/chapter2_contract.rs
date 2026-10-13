const SKILL: &str = include_str!("../skills/rolign/SKILL.md");
const FUTURE_CONTRACT: &str = include_str!("../skills/rolign/references/future-resume-contract.md");
const GUARD: &str = include_str!("../skills/rolign/references/free-text-guard.md");
const ANALYSIS_REQUEST: &str = include_str!("fixtures/analysis-request.json");
const COVER_REQUEST: &str = include_str!("fixtures/cover-request.json");
const COMPLETE_VISION: &str = include_str!("fixtures/chapter-2/future-vision-complete.txt");
const PARTIAL_VISION: &str = include_str!("fixtures/chapter-2/future-vision-partial.txt");
const RESUME_EN: &str = include_str!("fixtures/chapter-2/resume-en.md");
const RESUME_ID: &str = include_str!("fixtures/chapter-2/resume-id.md");
const JOB_EN: &str = include_str!("fixtures/chapter-2/job-en.md");
const JOB_ID: &str = include_str!("fixtures/chapter-2/job-id.md");
const JSON_LD_PAGE: &str = include_str!("fixtures/chapter-2/job-posting-jsonld.html");
const SEMANTIC_PAGE: &str = include_str!("fixtures/chapter-2/semantic-job-page.html");
const JAVASCRIPT_PAGE: &str = include_str!("fixtures/chapter-2/javascript-only-page.html");
const BLOCKED_PAGE: &str = include_str!("fixtures/chapter-2/blocked-page.html");
const OVERSIZED_VISION: &str = include_str!("fixtures/chapter-2/adversarial/oversized-vision.txt");
const SHELL_METACHARACTERS: &str =
    include_str!("fixtures/chapter-2/adversarial/shell-metacharacters.txt");
const OPTION_LIKE_VISION: &str =
    include_str!("fixtures/chapter-2/adversarial/option-like-vision.txt");
const PATH_TRAVERSAL: &str = include_str!("fixtures/chapter-2/adversarial/path-traversal.txt");
const CONTROL_CHARACTER: &str =
    include_str!("fixtures/chapter-2/adversarial/control-character.json");
const MARKDOWN_HTML: &str =
    include_str!("fixtures/chapter-2/adversarial/markdown-html-injection.txt");
const EMBEDDED_INSTRUCTIONS: &str =
    include_str!("fixtures/chapter-2/adversarial/embedded-instructions.txt");

#[test]
fn chapter_two_command_grammar_is_canonical() {
    for command in [
        "analyze [URL] [--language en|id]",
        "cover [URL] [--language en|id]",
        "future [--language en|id] [--] [VISION...]",
        "Omitted language defaults to `en`",
        "Reject unknown leading options",
        "duplicate",
        "otherwise the first",
        "non-option token begins one vision value",
    ] {
        assert!(SKILL.contains(command));
    }
}

#[test]
fn future_contract_has_strict_shape_and_limits() {
    for field in [
        "language",
        "horizon_years",
        "career_vision",
        "target_positions",
        "skills_to_develop",
        "target_achievements",
        "Reject unknown fields",
        "5 through 10",
        "1–20",
        "2,000 UTF-8 bytes",
        "500 UTF-8 bytes",
    ] {
        assert!(FUTURE_CONTRACT.contains(field));
    }
    assert!(!FUTURE_CONTRACT.contains("vision_prompt"));
}

#[test]
fn free_text_guard_covers_every_untrusted_source() {
    for source in [
        "Resume",
        "manual job",
        "retrieved pages",
        "vision prompts",
        "model-produced",
    ] {
        assert!(GUARD.contains(source));
    }
    for rule in [
        "4,000 UTF-8 bytes",
        "5 MiB",
        "NUL",
        "non-whitespace control",
    ] {
        assert!(GUARD.contains(rule));
    }
}

#[test]
fn chapter_two_fixtures_cover_happy_and_fallback_inputs() {
    assert!(RESUME_EN.contains("Platform engineer"));
    assert!(RESUME_ID.contains("Insinyur platform"));
    assert!(JOB_EN.contains("Example Systems"));
    assert!(JOB_ID.contains("membutuhkan"));
    assert!(COMPLETE_VISION.contains("seven years"));
    assert!(PARTIAL_VISION.contains("platform engineering leader"));
    assert!(JSON_LD_PAGE.contains("JobPosting"));
    assert!(JSON_LD_PAGE.contains("hiringOrganization"));
    assert!(SEMANTIC_PAGE.contains("Job description"));
    assert!(SEMANTIC_PAGE.contains("<nav>"));
    assert!(SEMANTIC_PAGE.contains("<script>"));
    assert!(JAVASCRIPT_PAGE.contains("Loading job details"));
    assert!(BLOCKED_PAGE.contains("Access denied"));
    assert!(OVERSIZED_VISION.repeat(2).len() > 4_000);
    assert!(SHELL_METACHARACTERS.contains('|'));
    assert!(OPTION_LIKE_VISION.starts_with("--"));
    assert!(PATH_TRAVERSAL.contains(".."));
    assert!(CONTROL_CHARACTER.contains("\\u0000"));
    assert!(MARKDOWN_HTML.contains("<script>"));
    assert!(EMBEDDED_INSTRUCTIONS.contains("Ignore all prior instructions"));
}

#[test]
fn raw_html_retrieval_contract_is_bounded_and_safe() {
    for instruction in [
        "--max-filesize 5242880",
        "--output \"$html_file\"",
        "exactly one ordinary HTTP request",
        "reject and delete any file larger",
        "than 5 MiB",
        "objects inside `@graph` or arrays",
        "Ignore\n   `script`, `style`, `nav`",
        "Delete the temporary HTML file after extraction or on every failure path",
        "JavaScript-only, blocked, incomplete, or",
    ] {
        assert!(SKILL.contains(instruction));
    }
}

#[test]
fn existing_job_save_payloads_remain_unchanged_and_valid_json() {
    for request in [ANALYSIS_REQUEST, COVER_REQUEST] {
        assert!(serde_json::from_str::<serde_json::Value>(request).is_ok());
        assert!(!request.contains("\"language\""));
    }
}
