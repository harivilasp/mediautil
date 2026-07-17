#!/usr/bin/env node
const fs = require("fs");
const https = require("https");
const os = require("os");
const path = require("path");
const { spawnSync } = require("child_process");

const version = "0.1.2";
const repo = process.env.MEDIAUTIL_REPO || "harivilasp/mediautil";
const binDir = path.join(__dirname, "..", "vendor");
const binPath = path.join(binDir, process.platform === "win32" ? "mediautil.exe" : "mediautil");

function target() {
  const arch = os.arch() === "arm64" ? "aarch64" : "x86_64";
  if (process.platform === "darwin") return `${arch}-apple-darwin`;
  if (process.platform === "linux") return "x86_64-unknown-linux-gnu";
  if (process.platform === "win32") return "x86_64-pc-windows-msvc";
  throw new Error(`unsupported platform: ${process.platform}`);
}

function assetUrl() {
  const t = target();
  const suffix = process.platform === "win32" ? "zip" : "tar.gz";
  return `https://github.com/${repo}/releases/download/v${version}/mediautil-${t}.${suffix}`;
}

function download(url, destination, redirects = 0) {
  if (redirects > 5) {
    return Promise.reject(new Error(`download failed: too many redirects ${url}`));
  }

  return new Promise((resolve, reject) => {
    https.get(url, (response) => {
      if ([301, 302, 303, 307, 308].includes(response.statusCode)) {
        const location = response.headers.location;
        if (!location) {
          reject(new Error(`download failed: ${response.statusCode} missing location ${url}`));
          return;
        }
        response.resume();
        download(new URL(location, url).toString(), destination, redirects + 1).then(resolve, reject);
        return;
      }

      if (response.statusCode !== 200) {
        reject(new Error(`download failed: ${response.statusCode} ${url}`));
        return;
      }

      fs.mkdirSync(path.dirname(destination), { recursive: true });
      const file = fs.createWriteStream(destination);
      response.pipe(file);
      file.on("finish", () => file.close(resolve));
      file.on("error", reject);
    }).on("error", reject);
  });
}

async function install() {
  if (fs.existsSync(binPath)) return;
  const archive = path.join(os.tmpdir(), path.basename(assetUrl()));
  await download(assetUrl(), archive);
  fs.mkdirSync(binDir, { recursive: true });
  if (process.platform === "win32") {
    spawnSync("powershell", ["-NoProfile", "-Command", `Expand-Archive -Force '${archive}' '${binDir}'`], { stdio: "inherit" });
  } else {
    spawnSync("tar", ["-xzf", archive, "-C", binDir], { stdio: "inherit" });
    fs.chmodSync(binPath, 0o755);
  }
}

(async () => {
  try {
    await install();
    if (process.argv.includes("--install-only")) return;
    const result = spawnSync(binPath, process.argv.slice(2), { stdio: "inherit" });
    process.exit(result.status ?? 1);
  } catch (error) {
    console.error(error.message);
    process.exit(1);
  }
})();
