use std::{
    ffi::OsString,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{Context, Result, bail};

#[derive(Debug, Clone)]
pub struct Tool {
    pub name: &'static str,
    pub purpose: &'static str,
}

pub const TOOLS: &[Tool] = &[
    Tool {
        name: "tesseract",
        purpose: "OCR images",
    },
    Tool {
        name: "pdftotext",
        purpose: "PDF text extraction",
    },
    Tool {
        name: "qpdf",
        purpose: "PDF split and merge",
    },
    Tool {
        name: "pdfcrop",
        purpose: "PDF page cropping",
    },
    Tool {
        name: "mutool",
        purpose: "PDF conversion",
    },
    Tool {
        name: "magick",
        purpose: "ImageMagick PDF/image conversion fallback",
    },
];

pub fn executable_exists(name: &str) -> bool {
    let Some(paths) = std::env::var_os("PATH") else {
        return false;
    };
    std::env::split_paths(&paths).any(|dir| {
        let candidate = dir.join(name);
        candidate.is_file() || executable_with_windows_suffix(&candidate)
    })
}

fn executable_with_windows_suffix(candidate: &Path) -> bool {
    cfg!(windows) && candidate.with_extension("exe").is_file()
}

pub fn run_capture(program: &str, args: Vec<OsString>) -> Result<String> {
    ensure_tool(program)?;
    let output = Command::new(program)
        .args(args)
        .output()
        .with_context(|| format!("failed to run {program}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("{program} failed: {}", stderr.trim());
    }

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

pub fn run_status(program: &str, args: Vec<OsString>) -> Result<()> {
    ensure_tool(program)?;
    let output = Command::new(program)
        .args(args)
        .output()
        .with_context(|| format!("failed to run {program}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("{program} failed: {}", stderr.trim());
    }

    Ok(())
}

pub fn ensure_tool(name: &str) -> Result<()> {
    if executable_exists(name) {
        Ok(())
    } else {
        bail!("required external tool `{name}` was not found on PATH")
    }
}

pub fn path_arg(path: &Path) -> OsString {
    PathBuf::from(path).into_os_string()
}
