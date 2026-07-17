from __future__ import annotations

import os
import platform
import subprocess
import sys
import tarfile
import tempfile
import urllib.request
import zipfile
from pathlib import Path

VERSION = "0.1.2"
REPO = os.environ.get("MEDIAUTIL_REPO", "harivilasp/mediautil")


def _target() -> tuple[str, str]:
    machine = platform.machine().lower()
    arch = "aarch64" if machine in {"arm64", "aarch64"} else "x86_64"
    system = platform.system().lower()
    if system == "darwin":
        return f"{arch}-apple-darwin", "tar.gz"
    if system == "linux":
        return "x86_64-unknown-linux-gnu", "tar.gz"
    if system == "windows":
        return "x86_64-pc-windows-msvc", "zip"
    raise RuntimeError(f"unsupported platform: {system}")


def _bin_path() -> Path:
    name = "mediautil.exe" if platform.system().lower() == "windows" else "mediautil"
    return Path(__file__).with_name("vendor") / name


def _asset_url() -> str:
    target, suffix = _target()
    return f"https://github.com/{REPO}/releases/download/v{VERSION}/mediautil-{target}.{suffix}"


def _ensure_binary() -> Path:
    binary = _bin_path()
    if binary.exists():
        return binary

    binary.parent.mkdir(parents=True, exist_ok=True)
    target, suffix = _target()
    with tempfile.TemporaryDirectory() as tmp:
        archive = Path(tmp) / f"mediautil-{target}.{suffix}"
        urllib.request.urlretrieve(_asset_url(), archive)
        if suffix == "zip":
            with zipfile.ZipFile(archive) as zip_file:
                zip_file.extractall(binary.parent)
        else:
            with tarfile.open(archive, "r:gz") as tar:
                for member in tar.getmembers():
                    destination = (binary.parent / member.name).resolve()
                    if not destination.is_relative_to(binary.parent.resolve()):
                        raise RuntimeError(f"unsafe archive path: {member.name}")
                tar.extractall(binary.parent)

    if platform.system().lower() != "windows":
        binary.chmod(0o755)
    return binary


def main() -> None:
    binary = _ensure_binary()
    completed = subprocess.run([str(binary), *sys.argv[1:]], check=False)
    raise SystemExit(completed.returncode)


__all__ = ["main"]
