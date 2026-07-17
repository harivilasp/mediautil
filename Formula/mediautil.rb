class Mediautil < Formula
  desc "Local-first CLI for image/PDF utilities, OCR, QR codes, icons, data URIs, and base64"
  homepage "https://github.com/harivilasp/mediautil"
  url "https://github.com/harivilasp/mediautil/archive/refs/tags/v0.1.1.tar.gz"
  sha256 "f63713649c35afd68e3f2752d716eb6babeadc36a3ca9a53f863dc946f640636"
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
