# Contributing to oxide-serve

Thanks for your interest! This project aims to be a minimal, educational HTTP server. Contributions that preserve that spirit are welcome.

## Guiding Principles

- **Zero external dependencies** — no new crate dependencies unless absolutely unavoidable
- **Simplicity over features** — every added line should justify its existence
- **Educational clarity** — code should be readable and teach something about HTTP or systems programming
- **No JavaScript** — the server serves zero JS; keep it that way

## How to Contribute

### Bug Reports

Open an issue with:
- A clear description of the problem
- Steps to reproduce
- Expected vs actual behavior

### Feature Requests

Open an issue describing the feature and why it fits `oxide-serve`'s scope. Not all features will be accepted — if the feature requires a dependency or significantly increases complexity, it likely belongs in a fork or a downstream project.

### Pull Requests

1. Fork the repository
2. Create a branch (`git checkout -b my-feature`)
3. Make your changes
4. Ensure it builds cleanly:
   ```bash
   cargo build --release
   cargo clippy -- -D warnings
   cargo fmt --check
   ```
5. Commit with a clear message
6. Open a PR against `main`

### Commit Style

Use conventional commits:
- `feat:` — new capability
- `fix:` — bug fix
- `docs:` — documentation only
- `refactor:` — code change with no behavior change
- `test:` — adding or updating tests
- `chore:` — build, CI, or tooling

## Code of Conduct

All contributors must follow the [Code of Conduct](CODE_OF_CONDUCT.md).