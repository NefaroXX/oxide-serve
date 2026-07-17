use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn start_server() -> u16 {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();

    thread::spawn(move || {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(move || {
                        oxide_serve::handle_client(stream);
                    });
                }
                Err(_) => break,
            }
        }
    });

    port
}

fn send_request(port: u16, request: &str) -> String {
    let mut stream = TcpStream::connect(("127.0.0.1", port)).unwrap();
    stream.set_read_timeout(Some(Duration::from_secs(2))).unwrap();
    stream.write_all(request.as_bytes()).unwrap();

    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();
    response
}

#[test]
fn test_server_responds_200_on_root() {
    let port = start_server();
    thread::sleep(Duration::from_millis(50));

    let response = send_request(port, "GET / HTTP/1.1\r\nHost: localhost\r\n\r\n");

    assert!(response.starts_with("HTTP/1.1 200 OK"));
    assert!(response.contains("Content-Type: text/html; charset=UTF-8"));
    assert!(response.contains("Welcome to Oxide Serve"));
    assert!(!response.contains("<script"));
}

#[test]
fn test_server_responds_404_on_unknown_route() {
    let port = start_server();
    thread::sleep(Duration::from_millis(50));

    let response = send_request(port, "GET /missing HTTP/1.1\r\nHost: localhost\r\n\r\n");

    assert!(response.starts_with("HTTP/1.1 404 Not Found"));
    assert!(response.contains("404"));
    assert!(response.contains("Return home"));
}

#[test]
fn test_server_responds_400_on_bad_utf8() {
    let port = start_server();
    thread::sleep(Duration::from_millis(50));

    let mut stream = TcpStream::connect(("127.0.0.1", port)).unwrap();
    stream.set_read_timeout(Some(Duration::from_secs(2))).unwrap();
    stream.write_all(&[0xff, 0xfe, 0x00, 0x01]).unwrap();

    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();

    assert!(response.starts_with("HTTP/1.1 400 Bad Request"));
}
