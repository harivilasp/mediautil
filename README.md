# mediautil

`mediautil` is a local-first command-line utility for everyday media chores that should not require uploading private files to web tools. It gives developers and power users one consistent CLI for images, PDFs, OCR, QR codes, icons, data URIs, and base64 workflows.

## What It Does

- Resize, crop, convert, and inspect images.
- Generate `.ico` files from source images.
- Convert images to `data:` URIs for CSS, HTML, and prototypes.
- Generate and read QR codes.
- Encode and decode files with base64, including PDFs.
- Extract text from images with local Tesseract OCR.
- Extract, split, merge, crop, and convert PDFs through local PDF tools.

Native Rust handles lightweight operations such as images, QR codes, base64, and data URIs. Heavyweight OCR/PDF commands wrap proven local tools so the UX stays simple while the underlying behavior stays dependable.

## Install

Choose one:

```bash
cargo install mediautil
```

```bash
npm install -g @harivilasp/mediautil
```

```bash
pip install mediautil-cli
```

```bash
brew install harivilasp/tap/mediautil
```

From source:

```bash
git clone https://github.com/harivilasp/mediautil.git
cd mediautil
cargo install --path .
```

## External Tools

Core image, QR, data URI, and base64 commands work without extra tools.

PDF and OCR commands use local command-line tools when needed:

- `tesseract` for `mediautil ocr`
- `pdftotext` from Poppler for `mediautil pdf text`
- `qpdf` for `mediautil pdf split` and `mediautil pdf merge`
- `pdfcrop` for `mediautil pdf crop`
- `mutool` from MuPDF, or ImageMagick `magick`, for `mediautil pdf convert`

Check your machine:

```bash
mediautil doctor
```

## Examples

```bash
mediautil image resize input.png output.webp --width 1200 --height 800
mediautil image crop input.png crop.png --x 20 --y 20 --width 400 --height 300
mediautil image icon logo.png favicon.ico --size 256
mediautil image data-uri logo.png
```

```bash
mediautil qr gen "https://example.com" qr.png
mediautil qr read qr.png
```

```bash
mediautil base64 encode report.pdf --output report.pdf.b64
mediautil base64 decode report.pdf.b64 --output report.pdf
```

```bash
mediautil ocr screenshot.png --lang eng
mediautil pdf text document.pdf
mediautil pdf split document.pdf page-1.pdf --pages 1
mediautil pdf merge chapter1.pdf chapter2.pdf --output book.pdf
mediautil pdf crop document.pdf cropped.pdf
mediautil pdf convert document.pdf page-%d.png
```

## Command Model

- Inputs are explicit positional arguments.
- Generated files are explicit positional or `--output` arguments.
- Text-like results such as OCR, PDF text, base64, and data URIs go to stdout unless an output flag exists.
- Missing external tools are reported directly by name.

## Quality

The test suite includes unit tests, functional CLI tests, and fake external-tool tests for OCR/PDF wrappers. Live validation has also been run against real Tesseract, Poppler, QPDF, `pdfcrop`, MuPDF, and ImageMagick tools.

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

## Contributing

See `CONTRIBUTING.md`, `AGENTS.md`, `docs/ARCHITECTURE.md`, and `docs/TESTING.md` before adding a new command. New commands should include functional CLI tests and clear missing-tool behavior when they depend on external software.
