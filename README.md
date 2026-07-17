# oxide-serve

A minimal, multi-threaded HTTP server built from scratch using **only the Rust standard library** — zero external dependencies.

`oxide-serve` was created as a learning resource and a foundation for understanding how HTTP servers work under the hood. No frameworks, no async runtimes, no macros — just `std::net::TcpListener`, `std::net::TcpStream`, and `std::thread`.

## Features

- **Zero dependencies** — built entirely on the Rust standard library
- **Multi-threaded** — each connection gets its own thread; a slow request never blocks the server
- **Manual request parsing** — reads raw bytes off the wire and parses HTTP by hand
- **Zero-JS frontend** — serves a clean HTML/CSS landing page with no JavaScript
- **Graceful error handling** — invalid UTF-8, dropped connections, and malformed requests never panic
- **Local-network ready** — binds to `0.0.0.0:8080` by default

## Quick Start

```bash
# Clone and run
git clone https://github.com/NefaroXX/oxide-serve.git
cd oxide-serve
cargo run --release
```

The server starts on `http://0.0.0.0:8080`. Open it in a browser — you should see the landing page.

## Project Structure

```
oxide-serve/
├── Cargo.toml          # Package metadata and dependencies (empty)
├── LICENSE             # MIT license
├── README.md           # This file
├── CONTRIBUTING.md     # Contribution guidelines
├── CODE_OF_CONDUCT.md  # Community standards
└── src/
    └── main.rs         # Single-file server implementation (~100 lines)
```

## Why?

Most production Rust web servers use async runtimes like `tokio` and frameworks like `axum` or `actix`. Those are excellent tools, but they abstract away the underlying mechanics of HTTP. `oxide-serve` exists to show what happens at the bare-metal level:

- How a `TcpListener` accepts connections
- How raw bytes become an HTTP request
- How a valid HTTP response is constructed byte-by-byte
- How threads provide concurrency without an async runtime

## Usage

### Routes

| Route | Response |
|---|---|
| `GET /` | `200 OK` — HTML landing page |
| anything else | `404 Not Found` — error page |

### Configuration

The bind address is hard-coded to `0.0.0.0:8080`. To change it, edit the `bind()` call in `src/main.rs`.

## License

MIT — see [LICENSE](LICENSE).

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) first.