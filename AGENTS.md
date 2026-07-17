# Agent Instructions

This repository welcomes contributions from AI coding agents. Agents should optimize for boring, reviewable changes.

## First Steps

1. Read `README.md`, `CONTRIBUTING.md`, and `docs/ARCHITECTURE.md`.
2. Inspect the existing command shape before adding flags or subcommands.
3. Keep changes scoped to the requested issue.
4. Do not publish releases, push tags, or modify registry credentials.

## Quality Gate

Run the relevant checks before opening a PR:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo llvm-cov --all-targets --fail-under-lines 80
cargo audit
npm pack --dry-run --prefix packaging/npm
python -m build packaging/pip
```

If a tool is unavailable, install it when reasonable. If a check still cannot run, state the exact command and reason in the PR.

## Test Expectations

For every user-visible behavior change:

- add or update a functional CLI test in `tests/cli.rs`
- cover one failure path
- use fake external binaries for OCR/PDF wrappers when possible
- avoid tests that depend on network access or user-specific files

## Code Style

- Keep parsing in `src/cli.rs`.
- Keep command behavior in `src/commands.rs`.
- Keep process execution helpers in `src/external.rs`.
- Use `anyhow::Context` for actionable errors.
- Do not use shell string interpolation for command execution.
- Prefer native Rust for lightweight media operations.
- Wrap proven local tools for heavyweight OCR/PDF operations.

## Documentation

Update docs when command behavior changes:

- `README.md` for user-facing examples
- `CONTRIBUTING.md` for workflow changes
- `docs/ARCHITECTURE.md` for module/design changes
- `CHANGELOG.md` for release-visible changes

## PR Notes

Agent-authored PRs should include:

- what changed
- why it changed
- exact validation commands and results
- known limitations or follow-up work

Never claim a package was published unless the command completed successfully and the registry URL is available.
