# Testing

The goal is to catch broken workflows before users do. Tests should exercise the CLI the way a user runs it.

## Layers

- Unit tests in `src/commands.rs` cover small pure behaviors.
- Functional tests in `tests/cli.rs` execute the compiled binary with temporary files.
- Fake external-tool tests verify OCR/PDF command wiring without requiring heavyweight local installs.
- Coverage is enforced with `cargo llvm-cov --all-targets --fail-under-lines 80`.

## Functional Tests

Use `assert_cmd` to run `mediautil` and `tempfile` for isolated input/output files.

Prefer this pattern:

```rust
Command::cargo_bin("mediautil")?
    .args(["base64", "encode"])
    .arg(&input)
    .args(["--output"])
    .arg(&output)
    .assert()
    .success();
```

## External Tool Tests

Use fake executables on `PATH` for wrappers around tools such as `tesseract`, `qpdf`, and `pdftotext`. Verify both the user-visible output and the arguments passed to the external binary.

Live integration tests against real external tools can be added later behind explicit opt-in environment variables.

## Manual Smoke Test

Before a release, run at least:

```bash
cargo build --release
target/release/mediautil --help
target/release/mediautil doctor
target/release/mediautil qr gen "smoke" /tmp/mediautil-smoke.png
target/release/mediautil qr read /tmp/mediautil-smoke.png
```
