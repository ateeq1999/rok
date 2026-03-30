use std::env;
use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let port = args.get(1).map(|p| p.as_str()).unwrap_or("8080");
    let addr = format!("0.0.0.0:{}", port);

    let doc_dir = env::current_dir()
        .map(|p| p.join("docs"))
        .unwrap_or_else(|_| Path::new("docs").to_path_buf());

    println!("📖 Serving rok documentation at http://localhost:{}", port);
    println!("Press Ctrl+C to stop\n");

    let listener = TcpListener::bind(&addr).expect("Failed to bind to port");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0u8; 1024];
                if let Ok(bytes_read) = stream.read(&mut buffer) {
                    let request = String::from_utf8_lossy(&buffer[..bytes_read]);

                    let (status, content_type, body) = if request.starts_with("GET / ")
                        || request.starts_with("GET /index.html")
                    {
                        serve_file(&doc_dir.join("index.html"), "text/html")
                    } else if request.contains("GET /api") {
                        serve_file(&doc_dir.join("api.html"), "text/html")
                    } else if request.contains("GET /") {
                        let path = request
                            .split_whitespace()
                            .nth(1)
                            .unwrap_or("/")
                            .trim_start_matches('/');

                        let file_path = doc_dir.join(path);
                        if file_path.exists() && file_path.is_file() {
                            let ext = path.rsplit('.').next().unwrap_or("");
                            let content_type = match ext {
                                "css" => "text/css",
                                "js" => "application/javascript",
                                "json" => "application/json",
                                "html" => "text/html",
                                "md" => "text/markdown",
                                "1" => "text/plain",
                                _ => "text/plain",
                            };
                            serve_file(&file_path, content_type)
                        } else {
                            (404, "text/html", not_found())
                        }
                    } else {
                        (404, "text/html", not_found())
                    };

                    let response = format!(
                        "HTTP/1.1 {} OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
                        status,
                        content_type,
                        body.len(),
                        body
                    );

                    let _ = stream.write_all(response.as_bytes());
                }
            }
            Err(e) => {
                eprintln!("Connection error: {}", e);
            }
        }
    }
}

fn serve_file(path: &Path, content_type: &str) -> (u16, &str, String) {
    match fs::read_to_string(path) {
        Ok(content) => (200, content_type, content),
        Err(_) => (404, "text/html", not_found()),
    }
}

fn not_found() -> String {
    r#"<!DOCTYPE html>
<html><head><title>404 Not Found</title></head>
<body><h1>404 Not Found</h1><p>The requested file was not found.</p></body></html>"#
        .to_string()
}
