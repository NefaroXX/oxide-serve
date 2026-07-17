# Architecture Decision Records

This directory logs architecture decisions made for oxide-serve.

## ADR-001: Zero-dependency HTTP server

- Date: 2026-07-17
- Status: Accepted

Use only the Rust standard library. No external crates.

## ADR-002: Thread-per-connection concurrency

- Date: 2026-07-17
- Status: Accepted

Spawn one std::thread per incoming TcpStream.

## ADR-003: Zero-JS frontend

- Date: 2026-07-17
- Status: Accepted

Serve purely static HTML and CSS. No script tags.
