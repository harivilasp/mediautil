# Contributing

`mediautil` is meant to be boring, local, scriptable infrastructure. Changes should keep the CLI predictable and easy to automate.

## Development

Install the normal Rust toolchain plus optional release verification tools:

```bash
cargo install cargo-llvm-cov
python -m pip install --upgrade build
```

Run the Rust quality gate:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo llvm-cov --all-targets --fail-under-lines 80
cargo audit
```

Package checks:

```bash
npm pack --dry-run --prefix packaging/npm
python -m build packaging/pip
```

The CI workflow runs these checks on Linux, macOS, and Windows where applicable.

## Test Policy

Every user-visible command should have at least one functional CLI test. Prefer tests that execute the binary with real files over tests that only call internals.

External tools such as `tesseract`, `qpdf`, and `pdftotext` should be tested with fake binaries where possible, so contributors can run the suite without installing large native dependencies. Add live-tool tests only when the behavior cannot be simulated.

When adding a command, include:

- success-path CLI test
- input validation or failure-path test
- fake external-tool test if the command wraps another binary
- README example if the command is user-facing

## Command Design

- Keep commands composable: file in, file or stdout out.
- Use clear failure messages with the missing tool or bad input named directly.
- Do not add network calls to core media operations.
- Prefer native Rust for lightweight operations.
- Wrap established local tools for heavyweight OCR/PDF behavior.

## Architecture

See `docs/ARCHITECTURE.md` for module responsibilities. Keep command parsing, execution, and external process handling separated so tests can target behavior cleanly.

## AI Agent Contributions

AI agents should follow `AGENTS.md`. Maintainers should expect agent-authored PRs to include exact validation output and docs updates for user-visible changes.

## Release Policy

Releases are tag-driven. Maintainers cut tags on a regular cycle when CI is green and the changelog/release notes are ready.

Automated release jobs build GitHub assets and publish registry packages only when the matching credentials are configured.

Use `docs/PUBLISHING.md` as the release checklist.
