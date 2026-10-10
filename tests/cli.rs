use std::{
    env, fs,
    io::Write,
    process::{Command, Stdio},
    sync::atomic::{AtomicUsize, Ordering},
};

static NEXT_DIRECTORY: AtomicUsize = AtomicUsize::new(0);

#[test]
fn validates_http_urls_without_creating_history() {
    let output = Command::new(env!("CARGO_BIN_EXE_rolign"))
        .args(["validate-url", "https://example.test/job/123"])
        .output()
        .unwrap();

    assert!(output.status.success());
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&output.stdout).unwrap()["url"],
        "https://example.test/job/123"
    );
}

#[test]
fn rejects_invalid_urls_before_retrieval() {
    let output = Command::new(env!("CARGO_BIN_EXE_rolign"))
        .args(["validate-url", "ftp://example.test/job/123"])
        .output()
        .unwrap();

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("valid HTTP(S) URL"));
}

#[test]
fn saves_fixture_input_and_lists_it() {
    let workspace = env::temp_dir().join(format!(
        "rolign-cli-test-{}-{}",
        std::process::id(),
        NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir_all(&workspace).unwrap();
    fs::write(workspace.join("resume.md"), "# Synthetic resume\n").unwrap();

    let mut save = Command::new(env!("CARGO_BIN_EXE_rolign"))
        .args(["save", "--workspace", workspace.to_str().unwrap()])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    save.stdin
        .take()
        .unwrap()
        .write_all(include_bytes!("fixtures/analysis-request.json"))
        .unwrap();
    let saved = save.wait_with_output().unwrap();
    assert!(saved.status.success());
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&saved.stdout).unwrap()["score"],
        78
    );

    let listed = Command::new(env!("CARGO_BIN_EXE_rolign"))
        .args(["list", "--workspace", workspace.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(listed.status.success());
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&listed.stdout).unwrap()["records"]
            .as_array()
            .unwrap()
            .len(),
        1
    );

    fs::remove_dir_all(workspace).unwrap();
}

#[test]
fn saves_cover_only_fixture_and_lists_a_null_score() {
    let workspace = env::temp_dir().join(format!(
        "rolign-cli-cover-test-{}-{}",
        std::process::id(),
        NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir_all(&workspace).unwrap();
    fs::write(workspace.join("resume.md"), "# Synthetic resume\n").unwrap();

    let mut save = Command::new(env!("CARGO_BIN_EXE_rolign"))
        .args(["save", "--workspace", workspace.to_str().unwrap()])
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();
    save.stdin
        .take()
        .unwrap()
        .write_all(include_bytes!("fixtures/cover-request.json"))
        .unwrap();
    assert!(save.wait().unwrap().success());

    let listed = Command::new(env!("CARGO_BIN_EXE_rolign"))
        .args(["list", "--workspace", workspace.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(listed.status.success());
    let record =
        &serde_json::from_slice::<serde_json::Value>(&listed.stdout).unwrap()["records"][0];
    assert_eq!(record["kind"], "cover-letter-only");
    assert!(record["score"].is_null());
    assert!(record["job_url"].is_null());

    fs::remove_dir_all(workspace).unwrap();
}

#[test]
fn malformed_input_creates_no_history() {
    let workspace = env::temp_dir().join(format!(
        "rolign-cli-invalid-test-{}-{}",
        std::process::id(),
        NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir_all(&workspace).unwrap();
    fs::write(workspace.join("resume.md"), "# Synthetic resume\n").unwrap();

    let mut save = Command::new(env!("CARGO_BIN_EXE_rolign"))
        .args(["save", "--workspace", workspace.to_str().unwrap()])
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();
    save.stdin
        .take()
        .unwrap()
        .write_all(br#"{"kind":"analysis"}"#)
        .unwrap();
    assert!(!save.wait().unwrap().success());
    assert!(!workspace.join("history").exists());

    fs::remove_dir_all(workspace).unwrap();
}
