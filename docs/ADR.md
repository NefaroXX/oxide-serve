# Architecture Decision Records

This directory logs architecture decisions made for oxide-serve.

## ADR-001: Zero-dependency HTTP server

- **Date:** 2026-07-17
- **Status:** Accepted

### Context

The project's goal is to demonstrate how an HTTP server works at the
bare-metal level. Using external runtimes or frameworks would obscure the
underlying mechanics.

### Decision

Use only the Rust standard library — `std::net::TcpListener`,
`std::net::TcpStream`, `std::io::Read`/`Write`, and `std::thread`.
No external crates.

### Consequences

- The implementation is ~100 lines of straightforward, single-file code.
- No dependency vulnerabilities or update burden.
- Not suitable for production workloads (no back-pressure, no async I/O).
- Easy to understand for learners.

---

## ADR-002: Thread-per-connection concurrency

- **Date:** 2026-07-17
- **Status:** Accepted

### Context

The server must handle concurrent clients without blocking. The standard
library does not include an async runtime.

### Decision

Spawn one `std::thread` per incoming `TcpStream`. This is the simplest
correct approach that leverages only `std::thread`.

### Consequences

- A slow client cannot block other clients.
- Thread overhead is acceptable for educational/demo workloads.
- Not suitable for high-concurrency production use (thousands of
  simultaneous connections).
- No thread-pool overhead or tuning knobs.

---

## ADR-003: Zero-JS frontend

- **Date:** 2026-07-17
- **Status:** Accepted

### Context

The landing page should be simple, secure, and universally renderable.
JavaScript introduces complexity, attack surface, and rendering delays.

### Decision

Serve purely static HTML and CSS. No `<script>` tags, no event handler
attributes, no external resources.

### Consequences

- The page renders in every browser, including text-mode and assistive
  technologies, without any runtime evaluation.
- No XSS surface from injected scripts.
- Styling changes require a server restart (content is baked into the
  binary).
