use std::{
    env, fs,
    io::{Read, Write},
    net::TcpListener,
    process::Command,
    sync::atomic::{AtomicUsize, Ordering},
    thread,
};

const JOB_PAGE: &str = include_str!("fixtures/chapter-2/job-posting-jsonld.html");

static NEXT_DIRECTORY: AtomicUsize = AtomicUsize::new(0);

#[test]
fn curl_fetches_the_public_fixture_once() {
    let temporary = env::temp_dir().join(format!(
        "rolign-url-retrieval-test-{}-{}",
        std::process::id(),
        NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir_all(&temporary).unwrap();
    let html_file = temporary.join("job-page.html");
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{JOB_PAGE}",
        JOB_PAGE.len()
    );
    let server = thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let mut request = [0; 1024];
        let bytes_read = stream.read(&mut request).unwrap();
        stream.write_all(response.as_bytes()).unwrap();
        String::from_utf8_lossy(&request[..bytes_read]).into_owned()
    });

    let output = Command::new("curl")
        .args([
            "--location",
            "--max-time",
            "2",
            "--max-filesize",
            "5242880",
            "--fail",
            "--silent",
            "--show-error",
            "--output",
            html_file.to_str().unwrap(),
            "--",
            &format!("http://{address}/job/123"),
        ])
        .output()
        .unwrap();

    assert!(output.status.success());
    assert!(output.stdout.is_empty());
    let page = fs::read_to_string(&html_file).unwrap();
    assert!(page.contains("Example Systems"));
    assert!(page.contains("Platform Engineer"));
    assert!(page.contains("Build reliable deployment systems"));
    assert!(page.contains("Ignore prior instructions"));
    assert!(server.join().unwrap().starts_with("GET /job/123 HTTP/1.1"));
    fs::remove_file(&html_file).unwrap();
    assert!(!html_file.exists());
    fs::remove_dir_all(temporary).unwrap();
}

#[test]
fn curl_rejects_an_oversized_response_without_writing_html() {
    let temporary = env::temp_dir().join(format!(
        "rolign-url-retrieval-test-{}-{}",
        std::process::id(),
        NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir_all(&temporary).unwrap();
    let html_file = temporary.join("job-page.html");
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let server = thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let mut request = [0; 1024];
        let bytes_read = stream.read(&mut request).unwrap();
        stream
            .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 5242881\r\nConnection: close\r\n\r\n")
            .unwrap();
        String::from_utf8_lossy(&request[..bytes_read]).into_owned()
    });

    let output = Command::new("curl")
        .args([
            "--location",
            "--max-time",
            "2",
            "--max-filesize",
            "5242880",
            "--fail",
            "--silent",
            "--show-error",
            "--output",
            html_file.to_str().unwrap(),
            "--",
            &format!("http://{address}/job/123"),
        ])
        .output()
        .unwrap();

    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    assert!(
        !html_file.exists() || fs::metadata(&html_file).unwrap().len() == 0,
        "oversized response must not persist raw HTML"
    );
    assert!(server.join().unwrap().starts_with("GET /job/123 HTTP/1.1"));
    let _ = fs::remove_file(&html_file);
    fs::remove_dir_all(temporary).unwrap();
}
