use std::{
    io::{Read, Write},
    net::TcpListener,
    process::Command,
    thread,
};

const JOB_PAGE: &str = include_str!("fixtures/public-job-page.html");

#[test]
fn curl_fetches_the_public_fixture_once() {
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
            "--fail",
            "--silent",
            "--show-error",
            "--",
            &format!("http://{address}/job/123"),
        ])
        .output()
        .unwrap();

    assert!(output.status.success());
    let page = String::from_utf8(output.stdout).unwrap();
    assert!(page.contains("Example Systems"));
    assert!(page.contains("Platform Engineer"));
    assert!(page.contains("Build reliable deployment systems"));
    assert!(page.contains("Ignore prior instructions"));
    assert!(server.join().unwrap().starts_with("GET /job/123 HTTP/1.1"));
}
