# mediautil

`mediautil` is a local-first media utility shell for common developer and power-user workflows:

- image crop, resize, convert, `.ico`, and data URI output
- OCR from images with Tesseract
- PDF text extraction, split, merge, crop, and conversion through local PDF tools
- QR code generation and reading
- base64 encode/decode for any file, including PDFs

The CLI is a small Rust binary. Native Rust handles image, QR, base64, and data URI paths. PDF and OCR operations intentionally wrap established local tools so the command surface stays consistent without hiding heavyweight dependencies.

## Status

This project is pre-1.0. The command shape is intended to be stable enough for early users, but releases may still refine flags and defaults while the community validates real workflows.

## Install From Source

```bash
cargo install --path .
```

Published packages are scaffolded for GitHub Releases, crates.io, npm, PyPI, and Homebrew. Registry publishing is enabled by release automation once repository credentials are configured.

## External Tools

Run:

```bash
mediautil doctor
```

Optional integrations:

- `tesseract` for `mediautil ocr`
- `pdftotext` for `mediautil pdf text`
- `qpdf` for `mediautil pdf split` and `mediautil pdf merge`
- `pdfcrop` for `mediautil pdf crop`
- `mutool` or `magick` for `mediautil pdf convert`

Core commands that do not need external tools work out of the box.

## Examples

```bash
mediautil image resize input.png output.webp --width 1200 --height 800
mediautil image crop input.png crop.png --x 20 --y 20 --width 400 --height 300
mediautil image icon logo.png favicon.ico --size 256
mediautil image data-uri logo.png

mediautil qr gen "https://example.com" qr.png
mediautil qr read qr.png

mediautil base64 encode report.pdf --output report.pdf.b64
mediautil base64 decode report.pdf.b64 --output report.pdf

mediautil ocr screenshot.png --lang eng
mediautil pdf text document.pdf
mediautil pdf split document.pdf page-1.pdf --pages 1
mediautil pdf merge chapter1.pdf chapter2.pdf --output book.pdf
```

## Command Model

The command model is intentionally simple:

- input files are explicit positional arguments
- generated files are explicit positional or `--output` arguments
- stdout is used for text-like output such as OCR, PDF text, base64, and data URIs
- heavyweight PDF/OCR behavior delegates to local tools and reports missing tools directly

## Quality

The test suite includes unit tests and functional CLI tests. Functional tests execute the compiled binary against real temporary files, including fake external binaries for OCR/PDF adapters. This keeps contributor setup light while still testing the command wiring.

Required checks:

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

## Release Channels

This repository includes packaging scaffolding for:

- GitHub Releases: `.github/workflows/release.yml`
- npm: `packaging/npm`
- PyPI: `packaging/pip`
- Homebrew: `Formula/mediautil.rb`

Publishing requires registry credentials and real GitHub release assets. See `docs/PUBLISHING.md`.

## Contributing

See `CONTRIBUTING.md`, `AGENTS.md`, `docs/ARCHITECTURE.md`, and `docs/TESTING.md` before adding a new command. New commands should include functional CLI tests and clear missing-tool behavior when they depend on external software.
