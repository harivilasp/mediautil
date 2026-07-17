# Architecture

`mediautil` is split into three main modules:

- `src/cli.rs`: clap definitions and user-facing command shape
- `src/commands.rs`: command behavior and validation
- `src/external.rs`: external binary discovery and process execution

Keep this separation intact. Parsing code should not run media operations, and command behavior should not duplicate process execution logic.

## Native Operations

Native Rust handles lightweight, deterministic operations:

- image resize, crop, convert, and icon creation via `image`
- data URI and base64 encoding via `base64` and MIME detection
- QR generation via `qrcode`
- QR reading via `rqrr`

These paths should have unit tests for core behavior and functional CLI tests using temporary files.

## External Operations

OCR and PDF workflows delegate to established local tools:

- `tesseract`
- `pdftotext`
- `qpdf`
- `pdfcrop`
- `mutool`
- `magick`

The wrapper should:

- fail with the exact missing tool name
- avoid shell interpolation
- pass arguments as structured process arguments
- return stdout for text-producing commands
- write files only where the user explicitly requested

Functional tests should use fake executables on `PATH` to verify arguments without requiring contributors to install every native dependency.

## Adding A Command

1. Add the CLI shape in `src/cli.rs`.
2. Add behavior in `src/commands.rs`.
3. Add external-tool support in `src/external.rs` only if the command wraps a binary.
4. Add functional tests in `tests/cli.rs`.
5. Add a README example.
6. Run the full quality gate from `CONTRIBUTING.md`.
