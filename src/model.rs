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

fn validate_url(value: String) -> Result<String, String> {
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
        let request =
            serde_json::from_str(include_str!("../tests/fixtures/analysis-request.json")).unwrap();
        let draft = validate_request(request, "# Resume".to_owned()).unwrap();
        assert_eq!(draft.analysis.unwrap().total, 78);
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
}
