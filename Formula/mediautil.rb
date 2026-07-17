class Mediautil < Formula
  desc "Local media utility shell for images, PDFs, OCR, QR codes, data URIs, and base64"
  homepage "https://github.com/harivilasp/mediautil"
  url "https://github.com/harivilasp/mediautil/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "c9f9596ff97684733f5b331484223d8dcb7f85011b8d58be6061099fd4c91e64"
  license "MIT"
  head "https://github.com/harivilasp/mediautil.git", branch: "main"

  depends_on "rust" => :build
  depends_on "qpdf" => :recommended
  depends_on "poppler" => :recommended
  depends_on "tesseract" => :recommended
  depends_on "mupdf-tools" => :optional
  depends_on "imagemagick" => :optional

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    assert_match "tesseract", shell_output("#{bin}/mediautil doctor")
  end
end
