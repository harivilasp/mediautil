class Mediautil < Formula
  desc "Local-first CLI for image/PDF, OCR, QR, data URI, and base64 workflows"
  homepage "https://github.com/harivilasp/mediautil"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/harivilasp/mediautil/releases/download/v0.1.2/mediautil-aarch64-apple-darwin.tar.gz"
      sha256 "30301a21bfc3aaa400da21734e5aef28c42111484e59ef36c4031b3e6ace1808"
    else
      odie "mediautil does not yet provide a prebuilt macOS Intel Homebrew binary"
    end
  end

  on_linux do
    if Hardware::CPU.intel?
      url "https://github.com/harivilasp/mediautil/releases/download/v0.1.2/mediautil-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "aa97e1f8c12c0d955739f837c5bcdaf33d77be70db82ab23ffcab5fe3367c816"
    else
      odie "mediautil does not yet provide a prebuilt Linux ARM Homebrew binary"
    end
  end

  def install
    bin.install "mediautil"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/mediautil --version")
  end
end
