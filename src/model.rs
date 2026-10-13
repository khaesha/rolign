use serde::{Deserialize, Serialize};
use url::Url;

use crate::score::{self, CategoryScores};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum RecordKind {
    Analysis,
    CoverLetterOnly,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AiHost {
    Codex,
    Claude,
    Gemini,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    En,
    Id,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SaveRequest {
    pub kind: RecordKind,
    pub company: String,
    pub job_title: String,
    pub job_url: Option<String>,
    pub job_description: String,
    pub ai_host: AiHost,
    pub analysis: Option<AnalysisInput>,
    pub cover_letter: CoverLetterInput,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnalysisInput {
    pub categories: CategoryScores,
    pub strengths: Vec<EvidenceMatch>,
    pub gaps: Vec<Gap>,
    pub missing_keywords: Vec<String>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EvidenceMatch {
    pub finding: String,
    pub evidence: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Gap {
    pub finding: String,
    pub explanation: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CoverLetterInput {
    pub body: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FutureResumeRequest {
    pub language: Language,
    pub horizon_years: u8,
    pub career_vision: String,
    pub target_positions: Vec<String>,
    pub skills_to_develop: Vec<String>,
    pub target_achievements: Vec<String>,
}

#[derive(Debug)]
pub struct FutureResumeDraft {
    pub language: Language,
    pub horizon_years: u8,
    pub career_vision: String,
    pub target_positions: Vec<String>,
    pub skills_to_develop: Vec<String>,
    pub target_achievements: Vec<String>,
}

#[derive(Debug)]
pub struct RecordDraft {
    pub kind: RecordKind,
    pub company: String,
    pub job_title: String,
    pub job_url: Option<String>,
    pub job_description: String,
    pub ai_host: AiHost,
    pub resume: String,
    pub analysis: Option<Analysis>,
    pub cover_letter: String,
}

#[derive(Debug)]
pub struct Analysis {
    pub categories: CategoryScores,
    pub total: u8,
    pub strengths: Vec<EvidenceMatch>,
    pub gaps: Vec<Gap>,
    pub missing_keywords: Vec<String>,
    pub recommendations: Vec<String>,
}

pub fn validate_request(request: SaveRequest, resume: String) -> Result<RecordDraft, String> {
    let company = required("company", request.company)?;
    let job_title = required("job_title", request.job_title)?;
    let job_description = required("job_description", request.job_description)?;
    let cover_letter = required("cover_letter.body", request.cover_letter.body)?;
    let job_url = request.job_url.map(validate_url).transpose()?;

    let analysis = match (request.kind.clone(), request.analysis) {
        (RecordKind::Analysis, Some(analysis)) => Some(validate_analysis(analysis)?),
        (RecordKind::Analysis, None) => {
            return Err("analysis records require analysis data".to_owned());
        }
        (RecordKind::CoverLetterOnly, Some(_)) => {
            return Err("cover-letter-only records must not include analysis data".to_owned());
        }
        (RecordKind::CoverLetterOnly, None) => None,
    };

    Ok(RecordDraft {
        kind: request.kind,
        company,
        job_title,
        job_url,
        job_description,
        ai_host: request.ai_host,
        resume,
        analysis,
        cover_letter,
    })
}

pub fn validate_future_request(request: FutureResumeRequest) -> Result<FutureResumeDraft, String> {
    if !(5..=10).contains(&request.horizon_years) {
        return Err("horizon_years must be between 5 and 10".to_owned());
    }
    Ok(FutureResumeDraft {
        language: request.language,
        horizon_years: request.horizon_years,
        career_vision: future_text("career_vision", request.career_vision, 2_000)?,
        target_positions: future_list("target_positions", request.target_positions)?,
        skills_to_develop: future_list("skills_to_develop", request.skills_to_develop)?,
        target_achievements: future_list("target_achievements", request.target_achievements)?,
    })
}

fn future_list(field: &str, values: Vec<String>) -> Result<Vec<String>, String> {
    if !(1..=20).contains(&values.len()) {
        return Err(format!("{field} must contain between 1 and 20 items"));
    }
    values
        .into_iter()
        .enumerate()
        .map(|(index, value)| future_text(&format!("{field}[{index}]"), value, 500))
        .collect()
}

fn future_text(field: &str, value: String, maximum_bytes: usize) -> Result<String, String> {
    if value.len() > maximum_bytes {
        return Err(format!("{field} exceeds {maximum_bytes} UTF-8 bytes"));
    }
    if value.chars().any(|character| {
        character == '\0' || (character.is_control() && !character.is_whitespace())
    }) {
        return Err(format!("{field} contains a control character"));
    }
    required(field, value)
}

fn validate_analysis(analysis: AnalysisInput) -> Result<Analysis, String> {
    let total = score::total(&analysis.categories)?;
    for item in &analysis.strengths {
        required("strength.finding", item.finding.clone())?;
        required("strength.evidence", item.evidence.clone())?;
    }
    for item in &analysis.gaps {
        required("gap.finding", item.finding.clone())?;
        required("gap.explanation", item.explanation.clone())?;
    }
    for keyword in &analysis.missing_keywords {
        required("missing_keywords item", keyword.clone())?;
    }
    for recommendation in &analysis.recommendations {
        required("recommendations item", recommendation.clone())?;
    }
    Ok(Analysis {
        categories: analysis.categories,
        total,
        strengths: analysis.strengths,
        gaps: analysis.gaps,
        missing_keywords: analysis.missing_keywords,
        recommendations: analysis.recommendations,
    })
}

fn required(field: &str, value: String) -> Result<String, String> {
    if value.trim().is_empty() {
        return Err(format!("{field} must not be empty"));
    }
    Ok(value)
}

pub fn validate_url(value: String) -> Result<String, String> {
    let parsed =
        Url::parse(&value).map_err(|_| "job_url must be a valid HTTP(S) URL".to_owned())?;
    if !matches!(parsed.scheme(), "http" | "https") || parsed.host_str().is_none() {
        return Err("job_url must be a valid HTTP(S) URL".to_owned());
    }
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_invalid_url_scheme() {
        assert_eq!(
            validate_url("ftp://example.com/job".to_owned()),
            Err("job_url must be a valid HTTP(S) URL".to_owned())
        );
    }

    #[test]
    fn fixture_request_validates() {
        for fixture in [
            include_str!("../tests/fixtures/analysis-request.json"),
            include_str!("../tests/fixtures/analysis-request-id.json"),
        ] {
            let request = serde_json::from_str(fixture).unwrap();
            let draft = validate_request(request, "# Resume".to_owned()).unwrap();
            assert_eq!(draft.analysis.unwrap().total, 78);
        }
    }

    #[test]
    fn indonesian_cover_fixture_validates() {
        let request =
            serde_json::from_str(include_str!("../tests/fixtures/cover-request-id.json")).unwrap();
        let draft = validate_request(request, "# Resume".to_owned()).unwrap();
        assert!(draft.analysis.is_none());
    }

    #[test]
    fn rejects_unknown_request_fields() {
        let mut request: serde_json::Value =
            serde_json::from_str(include_str!("../tests/fixtures/analysis-request.json")).unwrap();
        request
            .as_object_mut()
            .unwrap()
            .insert("unexpected".to_owned(), serde_json::Value::Null);
        assert!(serde_json::from_value::<SaveRequest>(request).is_err());
    }

    #[test]
    fn future_fixtures_validate_at_language_and_horizon_bounds() {
        for fixture in [
            include_str!("../tests/fixtures/future-request-en.json"),
            include_str!("../tests/fixtures/future-request-id.json"),
        ] {
            let request = serde_json::from_str(fixture).unwrap();
            let draft = validate_future_request(request).unwrap();
            assert_eq!(draft.horizon_years, 7);
        }
        for horizon_years in [5, 10] {
            let request = FutureResumeRequest {
                language: Language::En,
                horizon_years,
                career_vision: "Lead teams.".to_owned(),
                target_positions: vec!["Manager".to_owned()],
                skills_to_develop: vec!["Leadership".to_owned()],
                target_achievements: vec!["Build systems".to_owned()],
            };
            assert!(validate_future_request(request).is_ok());
        }
    }

    #[test]
    fn future_validation_rejects_invalid_shape_and_text() {
        let base = || FutureResumeRequest {
            language: Language::En,
            horizon_years: 7,
            career_vision: "Lead teams.".to_owned(),
            target_positions: vec!["Manager".to_owned()],
            skills_to_develop: vec!["Leadership".to_owned()],
            target_achievements: vec!["Build systems".to_owned()],
        };
        for horizon_years in [0, 4, 11] {
            let mut request = base();
            request.horizon_years = horizon_years;
            assert!(validate_future_request(request).is_err());
        }
        let mut empty = base();
        empty.target_positions.clear();
        assert!(validate_future_request(empty).is_err());
        let mut empty_vision = base();
        empty_vision.career_vision.clear();
        assert!(validate_future_request(empty_vision).is_err());
        for field in [
            "target_positions",
            "skills_to_develop",
            "target_achievements",
        ] {
            let mut request = base();
            match field {
                "target_positions" => request.target_positions = vec![String::new()],
                "skills_to_develop" => request.skills_to_develop = vec![String::new()],
                "target_achievements" => request.target_achievements = vec![String::new()],
                _ => unreachable!(),
            }
            assert!(validate_future_request(request).is_err());
        }
        let mut too_many = base();
        too_many.target_positions = vec!["Manager".to_owned(); 21];
        assert!(validate_future_request(too_many).is_err());
        let mut control = base();
        control.career_vision = "bad\0text".to_owned();
        assert!(validate_future_request(control).is_err());
        let mut oversized = base();
        oversized.career_vision = "x".repeat(2_001);
        assert!(validate_future_request(oversized).is_err());
        let mut long_item = base();
        long_item.skills_to_develop = vec!["x".repeat(501)];
        assert!(validate_future_request(long_item).is_err());

        let mut unknown: serde_json::Value =
            serde_json::from_str(include_str!("../tests/fixtures/future-request-en.json")).unwrap();
        unknown
            .as_object_mut()
            .unwrap()
            .insert("unexpected".to_owned(), serde_json::Value::Null);
        assert!(serde_json::from_value::<FutureResumeRequest>(unknown).is_err());

        let mut literal = base();
        literal.target_positions = vec!["../../future-resume.md --replace".to_owned()];
        assert!(validate_future_request(literal).is_ok());
    }
}
