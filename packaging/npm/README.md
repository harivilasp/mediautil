# mediautil

`mediautil` is a local-first command-line utility for everyday media chores that should not require uploading private files to web tools. It gives developers and power users one consistent CLI for images, PDFs, OCR, QR codes, icons, data URIs, and base64 workflows.

This npm package installs the `mediautil` Rust binary from the matching GitHub Release.

## Install

```bash
npm install -g @harivilasp/mediautil
```

Other install options:

```bash
cargo install mediautil
pip install mediautil-cli
brew install harivilasp/tap/mediautil
```

## What It Does

- Resize, crop, convert, and inspect images.
- Generate `.ico` files from source images.
- Convert images to `data:` URIs for CSS, HTML, and prototypes.
- Generate and read QR codes.
- Encode and decode files with base64, including PDFs.
- Extract text from images with local Tesseract OCR.
- Extract, split, merge, crop, and convert PDFs through local PDF tools.

## External Tools

Core image, QR, data URI, and base64 commands work without extra tools.

PDF and OCR commands use local command-line tools when needed:

- `tesseract` for OCR
- `pdftotext` from Poppler for PDF text extraction
- `qpdf` for PDF split and merge
- `pdfcrop` for PDF crop
- `mutool` from MuPDF, or ImageMagick `magick`, for PDF conversion

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

## Links

- GitHub: https://github.com/harivilasp/mediautil
- Releases: https://github.com/harivilasp/mediautil/releases
