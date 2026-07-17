use std::fs;

use assert_cmd::Command;
use image::{DynamicImage, GenericImageView};
use predicates::prelude::*;
use tempfile::tempdir;

#[test]
fn base64_round_trips_a_file() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let input = dir.path().join("input.bin");
    let encoded = dir.path().join("encoded.txt");
    let decoded = dir.path().join("decoded.bin");
    fs::write(&input, b"mediautil")?;

    Command::cargo_bin("mediautil")?
        .args(["base64", "encode"])
        .arg(&input)
        .args(["--output"])
        .arg(&encoded)
        .assert()
        .success();

    Command::cargo_bin("mediautil")?
        .args(["base64", "decode"])
        .arg(&encoded)
        .args(["--output"])
        .arg(&decoded)
        .assert()
        .success();

    assert_eq!(fs::read(decoded)?, b"mediautil");
    Ok(())
}

#[test]
fn qr_generation_and_reading_round_trip() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let qr = dir.path().join("qr.png");

    Command::cargo_bin("mediautil")?
        .args(["qr", "gen", "local-first media tools"])
        .arg(&qr)
        .assert()
        .success();

    Command::cargo_bin("mediautil")?
        .args(["qr", "read"])
        .arg(&qr)
        .assert()
        .success()
        .stdout(predicate::str::contains("local-first media tools"));

    Ok(())
}

#[test]
fn doctor_lists_external_tool_statuses() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("mediautil")?
        .arg("doctor")
        .assert()
        .success()
        .stdout(predicate::str::contains("tesseract"))
        .stdout(predicate::str::contains("pdftotext"));

    Ok(())
}

#[test]
fn image_resize_crop_convert_icon_and_data_uri_work() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let input = dir.path().join("input.png");
    let resized = dir.path().join("resized.png");
    let cropped = dir.path().join("cropped.png");
    let converted = dir.path().join("converted.jpg");
    let icon = dir.path().join("favicon.ico");
    DynamicImage::new_rgb8(80, 40).save(&input)?;

    Command::cargo_bin("mediautil")?
        .args(["image", "resize"])
        .arg(&input)
        .arg(&resized)
        .args(["--width", "20", "--height", "20", "--mode", "exact"])
        .assert()
        .success();
    assert_eq!(image::open(&resized)?.dimensions(), (20, 20));

    Command::cargo_bin("mediautil")?
        .args(["image", "crop"])
        .arg(&input)
        .arg(&cropped)
        .args(["--x", "10", "--y", "5", "--width", "30", "--height", "20"])
        .assert()
        .success();
    assert_eq!(image::open(&cropped)?.dimensions(), (30, 20));

    Command::cargo_bin("mediautil")?
        .args(["image", "convert"])
        .arg(&input)
        .arg(&converted)
        .assert()
        .success();
    assert_eq!(image::open(&converted)?.dimensions(), (80, 40));

    Command::cargo_bin("mediautil")?
        .args(["image", "icon"])
        .arg(&input)
        .arg(&icon)
        .args(["--size", "32"])
        .assert()
        .success();
    assert!(fs::metadata(&icon)?.len() > 0);

    Command::cargo_bin("mediautil")?
        .args(["image", "data-uri"])
        .arg(&input)
        .assert()
        .success()
        .stdout(predicate::str::starts_with("data:image/png;base64,"));

    Ok(())
}

#[test]
fn image_crop_reports_bounds_errors() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let input = dir.path().join("input.png");
    let output = dir.path().join("bad.png");
    DynamicImage::new_rgb8(16, 16).save(&input)?;

    Command::cargo_bin("mediautil")?
        .args(["image", "crop"])
        .arg(&input)
        .arg(&output)
        .args(["--x", "15", "--y", "15", "--width", "10", "--height", "10"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("exceeds image bounds"));

    Ok(())
}

#[test]
fn base64_decode_rejects_invalid_input() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let input = dir.path().join("invalid.txt");
    fs::write(&input, "not valid base64 ***")?;

    Command::cargo_bin("mediautil")?
        .args(["base64", "decode"])
        .arg(&input)
        .assert()
        .failure()
        .stderr(predicate::str::contains("failed to decode base64 input"));

    Ok(())
}

#[test]
fn qr_read_reports_missing_qr_code() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let blank = dir.path().join("blank.png");
    DynamicImage::new_luma8(128, 128).save(&blank)?;

    Command::cargo_bin("mediautil")?
        .args(["qr", "read"])
        .arg(&blank)
        .assert()
        .failure()
        .stderr(predicate::str::contains("no QR code detected"));

    Ok(())
}

#[test]
fn ocr_reports_missing_tesseract_when_path_has_no_tools() -> Result<(), Box<dyn std::error::Error>>
{
    let dir = tempdir()?;
    let input = dir.path().join("input.png");
    DynamicImage::new_luma8(32, 32).save(&input)?;

    Command::cargo_bin("mediautil")?
        .env("PATH", "")
        .args(["ocr"])
        .arg(&input)
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "required external tool `tesseract`",
        ));

    Ok(())
}

#[cfg(unix)]
mod external_tool_tests {
    use std::{
        fs,
        os::unix::fs::PermissionsExt,
        path::{Path, PathBuf},
    };

    use assert_cmd::Command;
    use predicates::prelude::*;
    use tempfile::{TempDir, tempdir};

    fn fake_bin(name: &str, body: &str) -> Result<(TempDir, PathBuf), Box<dyn std::error::Error>> {
        let dir = tempdir()?;
        let path = dir.path().join(name);
        fs::write(&path, format!("#!/bin/sh\n{body}\n"))?;
        let mut permissions = fs::metadata(&path)?.permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(&path, permissions)?;
        Ok((dir, path))
    }

    fn path_with_fake_tool(dir: &Path) -> String {
        let existing = std::env::var("PATH").unwrap_or_default();
        format!("{}:{existing}", dir.display())
    }

    #[test]
    fn ocr_uses_tesseract_stdout() -> Result<(), Box<dyn std::error::Error>> {
        let (dir, _) = fake_bin(
            "tesseract",
            r#"printf '%s\n' "$@" > "$MEDIAUTIL_ARGS"; printf 'detected text\n'"#,
        )?;
        let args_file = dir.path().join("args.txt");
        let input = dir.path().join("scan.png");
        fs::write(&input, b"not really an image")?;

        Command::cargo_bin("mediautil")?
            .env("PATH", path_with_fake_tool(dir.path()))
            .env("MEDIAUTIL_ARGS", &args_file)
            .args(["ocr"])
            .arg(&input)
            .args(["--lang", "hin"])
            .assert()
            .success()
            .stdout(predicate::str::contains("detected text"));

        let args = fs::read_to_string(args_file)?;
        assert!(args.contains("stdout"));
        assert!(args.contains("-l"));
        assert!(args.contains("hin"));
        Ok(())
    }

    #[test]
    fn pdf_text_uses_pdftotext_layout_stdout() -> Result<(), Box<dyn std::error::Error>> {
        let (dir, _) = fake_bin(
            "pdftotext",
            r#"printf '%s\n' "$@" > "$MEDIAUTIL_ARGS"; printf 'pdf text\n'"#,
        )?;
        let args_file = dir.path().join("args.txt");
        let input = dir.path().join("doc.pdf");
        fs::write(&input, b"%PDF")?;

        Command::cargo_bin("mediautil")?
            .env("PATH", path_with_fake_tool(dir.path()))
            .env("MEDIAUTIL_ARGS", &args_file)
            .args(["pdf", "text"])
            .arg(&input)
            .assert()
            .success()
            .stdout(predicate::str::contains("pdf text"));

        let args = fs::read_to_string(args_file)?;
        assert!(args.contains("-layout"));
        assert!(args.contains("-"));
        Ok(())
    }

    #[test]
    fn pdf_split_and_merge_use_qpdf() -> Result<(), Box<dyn std::error::Error>> {
        let (dir, _) = fake_bin(
            "qpdf",
            r#"printf '%s\n' "$@" > "$MEDIAUTIL_ARGS"; for arg do last="$arg"; done; touch "$last""#,
        )?;
        let args_file = dir.path().join("args.txt");
        let input_a = dir.path().join("a.pdf");
        let input_b = dir.path().join("b.pdf");
        let output = dir.path().join("out.pdf");
        fs::write(&input_a, b"%PDF")?;
        fs::write(&input_b, b"%PDF")?;

        Command::cargo_bin("mediautil")?
            .env("PATH", path_with_fake_tool(dir.path()))
            .env("MEDIAUTIL_ARGS", &args_file)
            .args(["pdf", "split"])
            .arg(&input_a)
            .arg(&output)
            .args(["--pages", "1-2"])
            .assert()
            .success();

        let split_args = fs::read_to_string(&args_file)?;
        assert!(split_args.contains("--pages"));
        assert!(split_args.contains("1-2"));

        Command::cargo_bin("mediautil")?
            .env("PATH", path_with_fake_tool(dir.path()))
            .env("MEDIAUTIL_ARGS", &args_file)
            .args(["pdf", "merge"])
            .arg(&input_a)
            .arg(&input_b)
            .args(["--output"])
            .arg(&output)
            .assert()
            .success();

        let merge_args = fs::read_to_string(args_file)?;
        assert!(merge_args.contains("--empty"));
        assert!(merge_args.contains("1-z"));
        Ok(())
    }
}
