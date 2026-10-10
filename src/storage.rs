use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use time::{OffsetDateTime, format_description::well_known::Rfc3339};
use url::Url;

use crate::{
    model::{AiHost, Analysis, RecordDraft, RecordKind},
    score::CategoryScores,
};

const SCHEMA_VERSION: u8 = 1;
const STAGING_PREFIX: &str = ".staging-";

#[derive(Debug, Serialize)]
pub struct SaveResult {
    pub id: String,
    pub score: Option<u8>,
}

#[derive(Debug, Serialize)]
pub struct ListResult {
    pub records: Vec<TrackerRecord>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct TrackerRecord {
    pub id: String,
    pub kind: RecordKind,
    pub created_at: String,
    pub company: String,
    pub job_title: String,
    pub job_url: Option<String>,
    pub score: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ContextFrontMatter {
    schema_version: u8,
    id: String,
    kind: RecordKind,
    created_at: String,
    company: String,
    job_title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    job_url: Option<String>,
    ai_host: AiHost,
    resume_sha256: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    score: Option<u8>,
}

pub fn workspace_root(workspace: &Path) -> Result<PathBuf, String> {
    let root = workspace
        .canonicalize()
        .map_err(|error| format!("workspace is unavailable: {error}"))?;
    if !root.is_dir() {
        return Err("workspace must be a directory".to_owned());
    }
    Ok(root)
}

pub fn read_resume(workspace: &Path) -> Result<String, String> {
    let workspace = workspace_root(workspace)?;
    let resume_path = workspace.join("resume.md");
    let canonical_resume = resume_path.canonicalize().map_err(|_| {
        "resume.md is missing; copy resume.md.example and add your resume".to_owned()
    })?;
    ensure_within(&workspace, &canonical_resume, "resume.md")?;
    if !canonical_resume.is_file() {
        return Err("resume.md must be a file".to_owned());
    }
    let resume = fs::read_to_string(&canonical_resume)
        .map_err(|error| format!("cannot read resume.md: {error}"))?;
    if resume.trim().is_empty() {
        return Err("resume.md is empty; add your resume before saving a record".to_owned());
    }
    Ok(resume)
}

pub fn save(workspace: &Path, draft: &RecordDraft) -> Result<SaveResult, String> {
    let workspace = workspace_root(workspace)?;
    let history = history_dir(&workspace, true)?;
    let now = OffsetDateTime::now_utc();
    let id = record_id(now, &draft.company, &draft.job_title);
    let final_dir = history.join(&id);
    if final_dir.exists() {
        return Err(format!(
            "record {id} already exists; no files were overwritten"
        ));
    }

    let staging = history.join(format!("{STAGING_PREFIX}{id}"));
    fs::create_dir(&staging).map_err(|error| format!("cannot create staging record: {error}"))?;

    let result: Result<(), String> = (|| {
        let created_at = now
            .format(&Rfc3339)
            .map_err(|error| format!("cannot format timestamp: {error}"))?;
        let score = draft.analysis.as_ref().map(|analysis| analysis.total);
        let context = ContextFrontMatter {
            schema_version: SCHEMA_VERSION,
            id: id.clone(),
            kind: draft.kind.clone(),
            created_at,
            company: draft.company.clone(),
            job_title: draft.job_title.clone(),
            job_url: draft.job_url.clone(),
            ai_host: draft.ai_host.clone(),
            resume_sha256: resume_hash(&draft.resume),
            score,
        };

        write_new(
            &staging.join("context.md"),
            &render_context(&context, &draft.job_description)?,
        )?;
        write_new(&staging.join("resume-snapshot.md"), &draft.resume)?;
        if let Some(analysis) = &draft.analysis {
            write_new(&staging.join("analysis.md"), &render_analysis(analysis))?;
        }
        write_new(
            &staging.join("cover-letter.md"),
            &render_cover_letter(&draft.cover_letter),
        )?;
        File::open(&staging)
            .and_then(|directory| directory.sync_all())
            .map_err(|error| format!("cannot flush staging record: {error}"))?;
        fs::rename(&staging, &final_dir)
            .map_err(|error| format!("cannot commit record: {error}"))?;
        File::open(&history)
            .and_then(|directory| directory.sync_all())
            .map_err(|error| format!("cannot flush history directory: {error}"))?;
        Ok(())
    })();

    if result.is_err() && staging.exists() {
        let _ = fs::remove_dir_all(&staging);
    }
    result?;
    Ok(SaveResult {
        id,
        score: draft.analysis.as_ref().map(|analysis| analysis.total),
    })
}

pub fn list(workspace: &Path) -> Result<ListResult, String> {
    let root = workspace_root(workspace)?;
    let history = root.join("history");
    if !history.exists() {
        return Ok(ListResult {
            records: Vec::new(),
            warnings: Vec::new(),
        });
    }
    let history = history_dir(&root, false)?;
    let mut records = Vec::new();
    let mut warnings = Vec::new();

    for entry in fs::read_dir(&history).map_err(|error| format!("cannot read history: {error}"))? {
        let entry = entry.map_err(|error| format!("cannot inspect history: {error}"))?;
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.starts_with(STAGING_PREFIX) || !entry.path().is_dir() {
            continue;
        }
        let path = match entry.path().canonicalize() {
            Ok(path) => path,
            Err(error) => {
                warnings.push(format!("skipped corrupt record {name}: {error}"));
                continue;
            }
        };
        if let Err(error) = ensure_within(&history, &path, &name) {
            warnings.push(format!("skipped corrupt record {name}: {error}"));
            continue;
        }
        match read_tracker_record(&path) {
            Ok(record) => records.push(record),
            Err(error) => warnings.push(format!("skipped corrupt record {name}: {error}")),
        }
    }
    records.sort_by(|left, right| right.created_at.cmp(&left.created_at));
    Ok(ListResult { records, warnings })
}

fn history_dir(workspace: &Path, create: bool) -> Result<PathBuf, String> {
    let history = workspace.join("history");
    if create {
        fs::create_dir_all(&history)
            .map_err(|error| format!("cannot create history directory: {error}"))?;
    }
    let canonical_history = history
        .canonicalize()
        .map_err(|error| format!("history directory is unavailable: {error}"))?;
    ensure_within(workspace, &canonical_history, "history")?;
    if !canonical_history.is_dir() {
        return Err("history must be a directory".to_owned());
    }
    Ok(canonical_history)
}

fn ensure_within(root: &Path, path: &Path, label: &str) -> Result<(), String> {
    if path.starts_with(root) {
        Ok(())
    } else {
        Err(format!("{label} must remain within the workspace"))
    }
}

fn write_new(path: &Path, content: &str) -> Result<(), String> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| format!("cannot write {}: {error}", path.display()))?;
    file.write_all(content.as_bytes())
        .and_then(|()| file.sync_all())
        .map_err(|error| format!("cannot flush {}: {error}", path.display()))
}

fn record_id(now: OffsetDateTime, company: &str, job_title: &str) -> String {
    format!(
        "{:04}{:02}{:02}T{:02}{:02}{:02}Z-{}",
        now.year(),
        now.month() as u8,
        now.day(),
        now.hour(),
        now.minute(),
        now.second(),
        slug(&format!("{company}-{job_title}"))
    )
}

fn slug(value: &str) -> String {
    let mut output = String::new();
    let mut previous_dash = false;
    for character in value.chars() {
        if character.is_ascii_alphanumeric() {
            output.push(character.to_ascii_lowercase());
            previous_dash = false;
        } else if !previous_dash {
            output.push('-');
            previous_dash = true;
        }
    }
    let output = output.trim_matches('-');
    let output: String = output.chars().take(80).collect();
    if output.is_empty() {
        "job".to_owned()
    } else {
        output
    }
}

fn resume_hash(resume: &str) -> String {
    format!("{:x}", Sha256::digest(resume.as_bytes()))
}

fn render_context(context: &ContextFrontMatter, job_description: &str) -> Result<String, String> {
    let front_matter = serde_yaml::to_string(context)
        .map_err(|error| format!("cannot render context metadata: {error}"))?;
    let original_posting = context
        .job_url
        .as_deref()
        .map(|url| format!("[Open job post](<{url}>)"))
        .unwrap_or_else(|| "—".to_owned());
    Ok(format!(
        "---\n{front_matter}---\n\n# Job\n\n- Company: {}\n- Position: {}\n- Original posting: {original_posting}\n\n# Job description\n\n{job_description}\n",
        escape_markdown(&context.company),
        escape_markdown(&context.job_title),
    ))
}

fn render_analysis(analysis: &Analysis) -> String {
    let CategoryScores {
        required_qualifications,
        skills_and_responsibilities,
        relevant_achievements,
        keywords_and_clarity,
    } = analysis.categories;
    let mut output = format!(
        "# Resume-job analysis\n\n## Score\n\n**{} / 100**\n\n- Required qualifications: {required_qualifications} / 40\n- Skills and responsibilities: {skills_and_responsibilities} / 30\n- Relevant achievements: {relevant_achievements} / 20\n- Keywords and clarity: {keywords_and_clarity} / 10\n\n## Strong matches\n\n",
        analysis.total
    );
    render_evidence_list(&mut output, &analysis.strengths);
    output.push_str("\n## Missing or weak requirements\n\n");
    render_gap_list(&mut output, &analysis.gaps);
    output.push_str("\n## Important missing keywords\n\n");
    render_string_list(&mut output, &analysis.missing_keywords);
    output.push_str("\n## Recommendations\n\n");
    render_string_list(&mut output, &analysis.recommendations);
    output.push_str("\nGuidance only; this is not a portal ATS score or hiring prediction.\n");
    output
}

fn render_cover_letter(body: &str) -> String {
    format!("# Cover letter draft\n\n{body}\n")
}

fn render_evidence_list(output: &mut String, values: &[crate::model::EvidenceMatch]) {
    if values.is_empty() {
        output.push_str("- None recorded.\n");
        return;
    }
    for value in values {
        output.push_str(&format!(
            "- {} — Evidence: {}\n",
            escape_markdown(&value.finding),
            escape_markdown(&value.evidence)
        ));
    }
}

fn render_gap_list(output: &mut String, values: &[crate::model::Gap]) {
    if values.is_empty() {
        output.push_str("- None recorded.\n");
        return;
    }
    for value in values {
        output.push_str(&format!(
            "- {} — {}\n",
            escape_markdown(&value.finding),
            escape_markdown(&value.explanation)
        ));
    }
}

fn render_string_list(output: &mut String, values: &[String]) {
    if values.is_empty() {
        output.push_str("- None recorded.\n");
        return;
    }
    for value in values {
        output.push_str(&format!("- {}\n", escape_markdown(value)));
    }
}

fn escape_markdown(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('*', "\\*")
        .replace('_', "\\_")
        .replace('[', "\\[")
        .replace(']', "\\]")
        .replace('`', "\\`")
        .replace('#', "\\#")
}

fn read_tracker_record(directory: &Path) -> Result<TrackerRecord, String> {
    let context = fs::read_to_string(directory.join("context.md"))
        .map_err(|error| format!("missing context.md: {error}"))?;
    let metadata = parse_context(&context)?;
    validate_context(&metadata)?;
    for file in ["resume-snapshot.md", "cover-letter.md"] {
        if !directory.join(file).is_file() {
            return Err(format!("missing {file}"));
        }
    }
    if metadata.kind == RecordKind::Analysis && !directory.join("analysis.md").is_file() {
        return Err("missing analysis.md".to_owned());
    }
    Ok(TrackerRecord {
        id: metadata.id,
        kind: metadata.kind,
        created_at: metadata.created_at,
        company: metadata.company,
        job_title: metadata.job_title,
        job_url: metadata.job_url,
        score: metadata.score,
    })
}

fn parse_context(content: &str) -> Result<ContextFrontMatter, String> {
    let normalized = content.replace("\r\n", "\n");
    let front_matter = normalized
        .strip_prefix("---\n")
        .and_then(|rest| rest.split_once("\n---\n").map(|(front, _)| front))
        .ok_or_else(|| "invalid front matter".to_owned())?;
    serde_yaml::from_str(front_matter).map_err(|error| format!("invalid front matter: {error}"))
}

fn validate_context(context: &ContextFrontMatter) -> Result<(), String> {
    if context.schema_version != SCHEMA_VERSION || context.id.trim().is_empty() {
        return Err("unsupported or incomplete metadata".to_owned());
    }
    if context.company.trim().is_empty()
        || context.job_title.trim().is_empty()
        || context.resume_sha256.len() != 64
        || !context
            .resume_sha256
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit())
    {
        return Err("incomplete metadata".to_owned());
    }
    OffsetDateTime::parse(&context.created_at, &Rfc3339)
        .map_err(|_| "invalid creation timestamp".to_owned())?;
    if let Some(url) = &context.job_url {
        let parsed = Url::parse(url).map_err(|_| "invalid job URL".to_owned())?;
        if !matches!(parsed.scheme(), "http" | "https") || parsed.host_str().is_none() {
            return Err("invalid job URL".to_owned());
        }
    }
    match context.kind {
        RecordKind::Analysis if context.score.is_none() => {
            Err("analysis record is missing score".to_owned())
        }
        RecordKind::Analysis if context.score.is_some_and(|score| score > 100) => {
            Err("analysis record has invalid score".to_owned())
        }
        RecordKind::CoverLetterOnly if context.score.is_some() => {
            Err("cover-only record must not have score".to_owned())
        }
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use std::{
        env, fs,
        sync::atomic::{AtomicUsize, Ordering},
    };

    use super::*;
    use crate::model::{SaveRequest, validate_request};

    static NEXT_DIRECTORY: AtomicUsize = AtomicUsize::new(0);

    struct TestWorkspace(PathBuf);

    impl TestWorkspace {
        fn new() -> Self {
            let directory = env::temp_dir().join(format!(
                "rolign-test-{}-{}",
                std::process::id(),
                NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir_all(&directory).unwrap();
            fs::write(directory.join("resume.md"), "# Synthetic resume\n").unwrap();
            Self(directory)
        }

        fn draft(&self) -> RecordDraft {
            let request: SaveRequest =
                serde_json::from_str(include_str!("../tests/fixtures/analysis-request.json"))
                    .unwrap();
            validate_request(request, read_resume(&self.0).unwrap()).unwrap()
        }
    }

    impl Drop for TestWorkspace {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn saves_and_lists_an_analysis_record_with_job_link() {
        let workspace = TestWorkspace::new();
        let result = save(&workspace.0, &workspace.draft()).unwrap();
        let directory = workspace.0.join("history").join(&result.id);
        assert!(directory.join("analysis.md").is_file());
        assert!(
            fs::read_to_string(directory.join("context.md"))
                .unwrap()
                .contains("https://example.test/jobs/platform")
        );
        assert_eq!(list(&workspace.0).unwrap().records.len(), 1);
    }

    #[test]
    fn cover_only_omits_analysis_file() {
        let workspace = TestWorkspace::new();
        let request: SaveRequest =
            serde_json::from_str(include_str!("../tests/fixtures/cover-request.json")).unwrap();
        let draft = validate_request(request, read_resume(&workspace.0).unwrap()).unwrap();
        let result = save(&workspace.0, &draft).unwrap();
        assert!(result.score.is_none());
        assert!(
            !workspace
                .0
                .join("history")
                .join(result.id)
                .join("analysis.md")
                .exists()
        );
    }

    #[test]
    fn collision_does_not_overwrite_existing_record() {
        let workspace = TestWorkspace::new();
        let draft = workspace.draft();
        save(&workspace.0, &draft).unwrap();
        assert!(save(&workspace.0, &draft).is_err());
        assert_eq!(list(&workspace.0).unwrap().records.len(), 1);
    }

    #[test]
    fn corrupt_records_warn_without_hiding_valid_records() {
        let workspace = TestWorkspace::new();
        save(&workspace.0, &workspace.draft()).unwrap();
        let corrupt = workspace.0.join("history").join("broken");
        fs::create_dir(&corrupt).unwrap();
        fs::write(corrupt.join("context.md"), "not front matter").unwrap();
        let result = list(&workspace.0).unwrap();
        assert_eq!(result.records.len(), 1);
        assert_eq!(result.warnings.len(), 1);
    }

    #[test]
    fn tracker_sorts_newest_first_and_ignores_staging_directories() {
        let workspace = TestWorkspace::new();
        let saved = save(&workspace.0, &workspace.draft()).unwrap();
        let history = workspace.0.join("history");
        let original = history.join(&saved.id);
        let older = history.join("older-record");
        fs::create_dir(&older).unwrap();
        for file in [
            "context.md",
            "resume-snapshot.md",
            "analysis.md",
            "cover-letter.md",
        ] {
            fs::copy(original.join(file), older.join(file)).unwrap();
        }
        let context = fs::read_to_string(older.join("context.md")).unwrap();
        let newer_timestamp = list(&workspace.0).unwrap().records[0].created_at.clone();
        fs::write(
            older.join("context.md"),
            context
                .replace(&saved.id, "older-record")
                .replace(&newer_timestamp, "2000-01-01T00:00:00Z"),
        )
        .unwrap();
        fs::create_dir(history.join(".staging-interrupted")).unwrap();

        let result = list(&workspace.0).unwrap();
        assert_eq!(result.records.len(), 2);
        assert_eq!(result.records[0].id, saved.id);
        assert!(result.warnings.is_empty());
    }

    #[test]
    fn failed_history_write_creates_no_record() {
        let workspace = TestWorkspace::new();
        fs::write(workspace.0.join("history"), "not a directory").unwrap();
        assert!(save(&workspace.0, &workspace.draft()).is_err());
        assert!(workspace.0.join("history").is_file());
    }

    #[test]
    fn unsafe_text_becomes_safe_slug() {
        assert_eq!(slug("../ACME\\Platform Engineer"), "acme-platform-engineer");
    }

    #[test]
    fn markdown_escapes_dynamic_text() {
        assert_eq!(escape_markdown("# [Rust]*"), "\\# \\[Rust\\]\\*");
    }
}
