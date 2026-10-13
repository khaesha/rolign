const CANONICAL_SKILL: &str = include_str!("../skills/rolign/SKILL.md");
const CODEX_ADAPTER: &str = include_str!("../.codex/skills/rolign/SKILL.md");
const CLAUDE_ADAPTER: &str = include_str!("../.claude/skills/rolign/SKILL.md");
const GEMINI_ADAPTER: &str = include_str!("../.gemini/commands/rolign.toml");
const README: &str = include_str!("../README.md");
const ANALYSIS_CONTRACT: &str = include_str!("../skills/rolign/references/analysis-contract.md");
const COVER_ONLY_CONTRACT: &str =
    include_str!("../skills/rolign/references/cover-only-contract.md");
const COVER_RULES: &str = include_str!("../skills/rolign/references/cover-letter-rules.md");
const JOB_PAGE: &str = include_str!("fixtures/public-job-page.html");

#[test]
fn host_adapters_delegate_to_the_canonical_skill() {
    for (adapter, host) in [
        (CODEX_ADAPTER, "codex"),
        (CLAUDE_ADAPTER, "claude"),
        (GEMINI_ADAPTER, "gemini"),
    ] {
        assert!(adapter.contains("canonical"));
        assert!(adapter.contains(host));
        assert!(!adapter.contains("Manual analysis"));
        assert!(!adapter.contains("## Job input"));
    }
    assert!(CODEX_ADAPTER.contains("../../../skills/rolign/SKILL.md"));
    assert!(CLAUDE_ADAPTER.contains("../../../skills/rolign/SKILL.md"));
    assert!(CLAUDE_ADAPTER.contains("$ARGUMENTS"));
    assert!(GEMINI_ADAPTER.contains("skills/rolign/SKILL.md"));
    assert!(GEMINI_ADAPTER.contains("{{args}}"));
    assert!(CLAUDE_ADAPTER.contains("<rolign_args>"));
    assert!(GEMINI_ADAPTER.contains("<rolign_args>"));
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
        "Make exactly one ordinary HTTP request",
        "curl --location --max-time 20 --max-filesize 5242880 --fail --silent --show-error --output",
        "<retrieved_page_data>",
        "original URL",
        "confirm or correct",
        "ask for pasted company, title, and complete job",
        "Do not make a second retrieval attempt",
        "Prefer a schema.org",
        "`JobPosting` JSON-LD object",
        "main` or `article",
        "not print raw HTML",
        "Delete the temporary HTML file",
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
    assert!(ANALYSIS_CONTRACT.contains("`codex`, `claude`, or `gemini`"));
    assert!(COVER_RULES.contains("only facts supported by `resume.md`"));
}

#[test]
fn cover_only_contract_excludes_analysis() {
    for field in [
        "cover-letter-only",
        "job_description",
        "analysis\": null",
        "cover_letter",
    ] {
        assert!(COVER_ONLY_CONTRACT.contains(field));
    }
    assert!(COVER_ONLY_CONTRACT.contains("do not include categories, a score, or analysis"));
    assert!(COVER_ONLY_CONTRACT.contains("`codex`, `claude`, or `gemini`"));
}

#[test]
fn tracker_workflow_renders_and_opens_only_listed_records() {
    for instruction in [
        "Markdown table",
        "Score, Job link, and ID",
        "show `—` for a missing score or job URL",
        "corrupt-record warnings",
        "open <record-id>",
        "exact returned ID",
        "compact header",
        "date, company, role, and original URL",
        "Never render `resume-snapshot.md`",
        "Never accept a path or ID",
    ] {
        assert!(CANONICAL_SKILL.contains(instruction));
    }
}

#[test]
fn readme_uses_standard_cargo_commands() {
    assert!(!README.contains("rtk"));
    assert!(!CANONICAL_SKILL.contains("rtk"));
    assert!(README.contains("cargo run -- save --workspace ."));
    assert!(README.contains("cargo fmt --check"));
}
