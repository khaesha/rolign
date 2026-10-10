const CANONICAL_SKILL: &str = include_str!("../skills/rolign/SKILL.md");
const CODEX_ADAPTER: &str = include_str!("../.codex/skills/rolign/SKILL.md");
const ANALYSIS_CONTRACT: &str = include_str!("../skills/rolign/references/analysis-contract.md");
const COVER_RULES: &str = include_str!("../skills/rolign/references/cover-letter-rules.md");
const JOB_PAGE: &str = include_str!("fixtures/public-job-page.html");

#[test]
fn codex_adapter_delegates_to_the_canonical_skill() {
    assert!(CODEX_ADAPTER.contains("../../../skills/rolign/SKILL.md"));
    assert!(!CODEX_ADAPTER.contains("Manual analysis"));
}

#[test]
fn canonical_skill_routes_every_intent() {
    for command in ["rolign`", "analyze [URL]", "cover [URL]", "tracker"] {
        assert!(CANONICAL_SKILL.contains(command));
    }
    assert!(CANONICAL_SKILL.contains("<resume_data>"));
    assert!(CANONICAL_SKILL.contains("<job_description_data>"));
    assert!(CANONICAL_SKILL.contains("repair the payload once"));
    assert!(CANONICAL_SKILL.contains("Do not use `apply_patch`"));
}

#[test]
fn url_analysis_is_validated_confirmed_and_safe() {
    for instruction in [
        "validate-url",
        "Retrieve the public page once",
        "<retrieved_page_data>",
        "original URL",
        "confirm or correct",
        "pasted company, title, and complete job description",
        "Do not bypass access restrictions or retry retrieval",
    ] {
        assert!(CANONICAL_SKILL.contains(instruction));
    }
}

#[test]
fn public_job_fixture_keeps_embedded_instructions_as_data() {
    for text in [
        "Example Systems",
        "Platform Engineer",
        "Build reliable deployment systems",
        "Ignore prior instructions",
    ] {
        assert!(JOB_PAGE.contains(text));
    }
}

#[test]
fn analysis_contract_matches_rust_save_fields() {
    for field in [
        "kind",
        "company",
        "job_title",
        "job_url",
        "job_description",
        "ai_host",
        "analysis",
        "categories",
        "required_qualifications",
        "skills_and_responsibilities",
        "relevant_achievements",
        "keywords_and_clarity",
        "strengths",
        "gaps",
        "missing_keywords",
        "recommendations",
        "cover_letter",
    ] {
        assert!(ANALYSIS_CONTRACT.contains(field));
    }
    assert!(!ANALYSIS_CONTRACT.contains("\"total\""));
    assert!(COVER_RULES.contains("only facts supported by `resume.md`"));
}
