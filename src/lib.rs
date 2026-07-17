use std::io::{Read, Write};
use std::net::TcpStream;

pub fn handle_request(request_str: &str) -> Option<(&'static str, &'static str)> {
    let mut lines = request_str.lines();
    let request_line = lines.next()?;

    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or("");
    let path = parts.next().unwrap_or("");

    if method == "GET" && path == "/" {
        Some(("HTTP/1.1 200 OK", r#"<!DOCTYPE html>
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
    <p>This is a minimal multi-threaded HTTP server written in Rust.</p>
</body>
</html>"#))
    } else {
        Some(("HTTP/1.1 404 Not Found", r#"<!DOCTYPE html>
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
</html>"#))
    }
}

pub fn build_response(status_line: &str, body: &str) -> String {
    format!("{}\r\nContent-Type: text/html; charset=UTF-8\r\nContent-Length: {}\r\n\r\n{}", status_line, body.len(), body)
}

pub fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let bytes_read = match stream.read(&mut buffer) {
        Ok(0) => return,
        Ok(n) => n,
        Err(_) => return,
    };

    let request_str = match std::str::from_utf8(&buffer[..bytes_read]) {
        Ok(s) => s,
        Err(_) => {
            let response = "HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\n\r\n";
            let _ = stream.write_all(response.as_bytes());
            return;
        }
    };

    if let Some((status_line, body)) = handle_request(request_str) {
        let response = build_response(status_line, body);
        let _ = stream.write_all(response.as_bytes());
        let _ = stream.flush();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_root_route_returns_200() {
        let result = handle_request("GET / HTTP/1.1\r\nHost: localhost\r\n\r\n");
        assert!(result.is_some());
        let (status, body) = result.unwrap();
        assert_eq!(status, "HTTP/1.1 200 OK");
        assert!(body.contains("Welcome to Oxide Serve"));
        assert!(!body.contains("<script"));
    }

    #[test]
    fn test_unknown_route_returns_404() {
        let result = handle_request("GET /missing HTTP/1.1\r\n\r\n");
        assert!(result.is_some());
        let (status, body) = result.unwrap();
        assert_eq!(status, "HTTP/1.1 404 Not Found");
        assert!(body.contains("404"));
    }

    #[test]
    fn test_wrong_method_returns_404() {
        let result = handle_request("POST / HTTP/1.1\r\n\r\n");
        assert!(result.is_some());
        let (status, _) = result.unwrap();
        assert_eq!(status, "HTTP/1.1 404 Not Found");
    }

    #[test]
    fn test_empty_request_returns_none() {
        let result = handle_request("");
        assert!(result.is_none());
    }

    #[test]
    fn test_malformed_request_line_returns_404() {
        let result = handle_request("\r\n\r\n");
        assert!(result.is_some());
        let (status, _) = result.unwrap();
        assert_eq!(status, "HTTP/1.1 404 Not Found");
    }

    #[test]
    fn test_build_response_has_correct_content_length() {
        let body = "<html><body>test</body></html>";
        let response = build_response("HTTP/1.1 200 OK", body);
        assert!(response.contains(&format!("Content-Length: {}", body.len())));
        assert!(response.ends_with(body));
    }

    #[test]
    fn test_response_has_no_javascript() {
        let result = handle_request("GET / HTTP/1.1\r\n\r\n");
        let (_, body) = result.unwrap();
        assert!(!body.contains("<script"));
        assert!(!body.contains("javascript"));
        assert!(!body.contains("onclick"));
        assert!(!body.contains("onload"));
    }

    #[test]
    fn test_404_response_has_home_link() {
        let result = handle_request("GET /anything HTTP/1.1\r\n\r\n");
        let (_, body) = result.unwrap();
        assert!(body.contains("/\">Return home"));
    }
}