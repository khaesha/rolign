const ROLIGN_SKILL: &str = include_str!("../skills/rolign/SKILL.md");
const INTEGRATION: &str = include_str!("../skills/rolign/references/humanizer-integration.md");
const VENDOR_SKILL: &str = include_str!("../skills/rolign/vendor/humanizer/SKILL.md");
const ATTRIBUTION: &str = include_str!("../skills/rolign/vendor/humanizer/ATTRIBUTION.md");
const LICENSE: &str = include_str!("../skills/rolign/vendor/humanizer/LICENSE");
const FACTS: &str = include_str!("fixtures/humanizer/protected-facts.json");
const ENGLISH_DRAFT: &str = include_str!("fixtures/humanizer/cover-letter-en.md");
const INDONESIAN_DRAFT: &str = include_str!("fixtures/humanizer/cover-letter-id.md");
const CHANGED_FACT: &str = include_str!("fixtures/humanizer/changed-protected-fact.md");
const ADDED_CLAIM: &str = include_str!("fixtures/humanizer/added-unsupported-claim.md");

#[test]
fn vendored_humanizer_is_pinned_and_mit_licensed() {
    assert!(VENDOR_SKILL.contains("version: \"3.1.0\""));
    assert!(ATTRIBUTION.contains("225a6f39ac85f76ee48dbad772ea4abe4ed6c9d8"));
    assert!(ATTRIBUTION.contains("https://github.com/blader/humanizer"));
    assert!(LICENSE.starts_with("MIT License"));
    assert!(LICENSE.contains("Copyright (c) 2025 Siqi Chen"));
}

#[test]
fn workflow_uses_bundled_humanizer_only_for_cover_letters() {
    for instruction in [
        "references/humanizer-integration.md",
        "bundled editing\n   pass to the cover letter only",
        "Humanizer never processes analysis prose or Future Resume content",
    ] {
        assert!(ROLIGN_SKILL.contains(instruction));
    }
    for instruction in [
        "do not install, invoke, or rely on a global Humanizer skill",
        "<cover_letter_draft>",
        "<protected_facts>",
        "must not use tools, read or write files, access the\n   network",
        "Return only the final cover\n   letter body in the selected language",
        "verify every protected fact appears unchanged",
        "adds, removes, or broadens any factual\n   claim",
        "stop without creating a payload or\n   history record",
    ] {
        assert!(INTEGRATION.contains(instruction));
    }
}

#[test]
fn protected_facts_survive_english_and_indonesian_drafts() {
    let facts: serde_json::Value = serde_json::from_str(FACTS).unwrap();
    for (language, draft) in [("en", ENGLISH_DRAFT), ("id", INDONESIAN_DRAFT)] {
        for fact in facts[language].as_array().unwrap() {
            assert!(draft.contains(fact.as_str().unwrap()));
        }
    }
    assert!(INDONESIAN_DRAFT.contains("Saya melamar"));
}

#[test]
fn unsafe_humanizer_edits_have_rejection_fixtures() {
    assert!(!CHANGED_FACT.contains("2024"));
    assert!(CHANGED_FACT.contains("2025"));
    assert!(ADDED_CLAIM.contains("50-person team"));
}
