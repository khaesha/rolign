const CANONICAL_SKILL: &str = include_str!("../skills/rolign/SKILL.md");
const CODEX_ADAPTER: &str = include_str!("../.codex/skills/rolign/SKILL.md");
const ANALYSIS_CONTRACT: &str = include_str!("../skills/rolign/references/analysis-contract.md");
const COVER_RULES: &str = include_str!("../skills/rolign/references/cover-letter-rules.md");

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
