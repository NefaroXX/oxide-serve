# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Repository community-health files: SECURITY.md, issue templates, pull request
  template, dependabot config, FUNDING.yml, and CI workflow.
- Architecture Decision Record (docs/ADR.md).
- Split server logic into oxide_serve library crate with handle_request
  and handle_client public API.
- Integration tests for request parsing, response formatting, and server
  lifecycle.

## [0.1.0] - 2026-07-17

### Added
- Initial release: a minimal, multi-threaded HTTP server with zero external
  dependencies.
- GET / returns 200 OK with a zero-JS HTML/CSS landing page.
- All other routes return 404 Not Found.
- Each connection is handled in its own thread.
- Manual HTTP request parsing from raw bytes.
- Graceful handling of invalid UTF-8, dropped connections, and read errors.
- MIT license with community documentation.
