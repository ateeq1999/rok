use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::Path;

pub fn serve(port: &str) -> i32 {
    let addr = format!("0.0.0.0:{}", port);
    let doc_dir = Path::new("docs");

    println!("Serving rok documentation at http://localhost:{}", port);
    println!("Press Ctrl+C to stop\n");

    let listener = match TcpListener::bind(&addr) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Failed to bind to port {}: {}", port, e);
            return 1;
        }
    };

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0u8; 2048];
                if let Ok(bytes_read) = stream.read(&mut buffer) {
                    let request = String::from_utf8_lossy(&buffer[..bytes_read]);
                    let response = route_request(&request, doc_dir);
                    let _ = stream.write_all(response.as_bytes());
                }
            }
            Err(e) => eprintln!("Connection error: {}", e),
        }
    }
    0
}

fn route_request(request: &str, doc_dir: &Path) -> String {
    if request.starts_with("GET / ") || request.starts_with("GET /index.html") {
        serve_file(&doc_dir.join("index.html"))
    } else if request.contains("GET /api") {
        serve_file(&doc_dir.join("api.html"))
    } else if request.starts_with("GET /") {
        let path = request
            .split_whitespace()
            .nth(1)
            .unwrap_or("/")
            .trim_start_matches('/');
        let file_path = doc_dir.join(path);
        if file_path.exists() && file_path.is_file() {
            serve_file(&file_path)
        } else {
            not_found()
        }
    } else {
        not_found()
    }
}

fn serve_file(path: &Path) -> String {
    match std::fs::read_to_string(path) {
        Ok(content) => {
            let content_type = match path.extension().and_then(|e| e.to_str()).unwrap_or("") {
                "css" => "text/css",
                "js" => "application/javascript",
                "json" => "application/json",
                "html" | "md" => "text/html",
                _ => "text/plain",
            };
            format!(
                "HTTP/1.1 200 OK\r\nContent-Type: {}; charset=utf-8\r\nContent-Length: {}\r\n\r\n{}",
                content_type,
                content.len(),
                content
            )
        }
        Err(_) => not_found(),
    }
}

fn not_found() -> String {
    "HTTP/1.1 404 Not Found\r\nContent-Type: text/html\r\nContent-Length: 171\r\n\r\n\
     <!DOCTYPE html><html><head><title>404</title></head>\
     <body><h1>404 Not Found</h1><p>The requested file was not found.</p></body></html>"
        .to_string()
}
