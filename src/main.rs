use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    // Read the request into a buffer
    let mut buffer = [0; 1024];
    let bytes_read = match stream.read(&mut buffer) {
        Ok(0) => return, // connection closed
        Ok(n) => n,
        Err(_) => return, // read error, ignore
    };

    // Try to interpret request as UTF-8; if invalid, just return without panic
    let request_str = match std::str::from_utf8(&buffer[..bytes_read]) {
        Ok(s) => s,
        Err(_) => {
            // Send a simple 400 Bad Request response
            let response = "HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\n\r\n";
            let _ = stream.write_all(response.as_bytes());
            return;
        }
    };

    // Parse the request line (e.g., "GET / HTTP/1.1")
    let mut lines = request_str.lines();
    let request_line = match lines.next() {
        Some(line) => line,
        None => return,
    };
    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or("");
    let path = parts.next().unwrap_or("");

    // Determine response based on method and path
    let (status_line, body) = if method == "GET" && path == "/" {
        (
            "HTTP/1.1 200 OK",
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Oxide Serve</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 2rem; }
        h1 { color: #2c3e50; }
    </style>
</head>
<body>
    <h1>Welcome to Oxide Serve!</h1>
    <p>This is a minimal multi‑threaded HTTP server written in Rust.</p>
</body>
</html>"#,
        )
    } else {
        (
            "HTTP/1.1 404 Not Found",
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>404 - Not Found | Oxide Serve</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 2rem; text-align: center; padding-top: 4rem; }
        h1 { color: #e74c3c; font-size: 3rem; margin-bottom: 0.5rem; }
        p { color: #555; font-size: 1.1rem; }
        a { color: #3498db; text-decoration: none; }
        a:hover { text-decoration: underline; }
    </style>
</head>
<body>
    <h1>404</h1>
    <p>The requested resource was not found.</p>
    <p><a href="/">Return home</a></p>
</body>
</html>"#,
        )
    };

    // Build the full HTTP response
    let response = format!(
        "{}\r\nContent-Type: text/html; charset=UTF-8\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        body.as_bytes().len(),
        body
    );

    let _ = stream.write_all(response.as_bytes());
    let _ = stream.flush();
}

fn main() -> std::io::Result<()> {
    // Bind to all interfaces on port 8080
    let listener = TcpListener::bind("0.0.0.0:8080")?;
    println!("Oxide Serve listening on 0.0.0.0:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Spawn a new thread for each connection
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
    Ok(())
}