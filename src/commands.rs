use std::{ffi::OsString, fs, io::Write, path::Path};

use anyhow::{Context, Result, bail};
use base64::{Engine, engine::general_purpose::STANDARD};
use image::{DynamicImage, GenericImageView, ImageFormat, imageops::FilterType};
use qrcode::QrCode;

use crate::{
    cli::{Base64Command, Command, FileIoArgs, ImageCommand, PdfCommand, QrCommand, ResizeMode},
    external,
};

pub fn run(command: Command) -> Result<()> {
    match command {
        Command::Image { command } => run_image(command),
        Command::Pdf { command } => run_pdf(command),
        Command::Ocr(args) => run_ocr(&args.input, args.output.as_deref(), &args.lang),
        Command::Qr { command } => run_qr(command),
        Command::Base64 { command } => run_base64(command),
        Command::Doctor => run_doctor(),
    }
}

fn run_image(command: ImageCommand) -> Result<()> {
    match command {
        ImageCommand::Resize(args) => resize_image(
            &args.input,
            &args.output,
            args.width,
            args.height,
            args.mode,
        ),
        ImageCommand::Crop(args) => crop_image(
            &args.input,
            &args.output,
            args.x,
            args.y,
            args.width,
            args.height,
        ),
        ImageCommand::Convert(args) => convert_image(&args.input, &args.output),
        ImageCommand::Icon(args) => icon_from_image(&args.input, &args.output, args.size),
        ImageCommand::DataUri(args) => {
            println!("{}", data_uri(&args.input)?);
            Ok(())
        }
    }
}

pub fn resize_image(
    input: &Path,
    output: &Path,
    width: u32,
    height: u32,
    mode: ResizeMode,
) -> Result<()> {
    let image = open_image(input)?;
    let resized = match mode {
        ResizeMode::Fit => image.resize(width, height, FilterType::Lanczos3),
        ResizeMode::Exact => image.resize_exact(width, height, FilterType::Lanczos3),
    };
    save_image(&resized, output)
}

pub fn crop_image(
    input: &Path,
    output: &Path,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) -> Result<()> {
    let image = open_image(input)?;
    let (image_width, image_height) = image.dimensions();
    if x.saturating_add(width) > image_width || y.saturating_add(height) > image_height {
        bail!(
            "crop rectangle {}x{} at {},{} exceeds image bounds {}x{}",
            width,
            height,
            x,
            y,
            image_width,
            image_height
        );
    }
    save_image(&image.crop_imm(x, y, width, height), output)
}

pub fn convert_image(input: &Path, output: &Path) -> Result<()> {
    let image = open_image(input)?;
    save_image(&image, output)
}

pub fn icon_from_image(input: &Path, output: &Path, size: u32) -> Result<()> {
    let image = open_image(input)?.resize_exact(size, size, FilterType::Lanczos3);
    image
        .save_with_format(output, ImageFormat::Ico)
        .with_context(|| format!("failed to save icon {}", output.display()))
}

pub fn data_uri(input: &Path) -> Result<String> {
    let bytes = fs::read(input).with_context(|| format!("failed to read {}", input.display()))?;
    let mime = mime_guess::from_path(input).first_or_octet_stream();
    Ok(format!("data:{mime};base64,{}", STANDARD.encode(bytes)))
}

fn open_image(input: &Path) -> Result<DynamicImage> {
    image::open(input).with_context(|| format!("failed to open image {}", input.display()))
}

fn save_image(image: &DynamicImage, output: &Path) -> Result<()> {
    image
        .save(output)
        .with_context(|| format!("failed to save image {}", output.display()))
}

fn run_pdf(command: PdfCommand) -> Result<()> {
    match command {
        PdfCommand::Text(args) => {
            let text = external::run_capture(
                "pdftotext",
                vec![
                    OsString::from("-layout"),
                    external::path_arg(&args.input),
                    OsString::from("-"),
                ],
            )?;
            print!("{text}");
            Ok(())
        }
        PdfCommand::Split(args) => external::run_status(
            "qpdf",
            vec![
                external::path_arg(&args.input),
                OsString::from("--pages"),
                external::path_arg(&args.input),
                OsString::from(args.pages),
                OsString::from("--"),
                external::path_arg(&args.output),
            ],
        ),
        PdfCommand::Merge(args) => {
            let mut command_args = vec![OsString::from("--empty"), OsString::from("--pages")];
            for input in args.inputs {
                command_args.push(external::path_arg(&input));
                command_args.push(OsString::from("1-z"));
            }
            command_args.push(OsString::from("--"));
            command_args.push(external::path_arg(&args.output));
            external::run_status("qpdf", command_args)
        }
        PdfCommand::Crop(args) => external::run_status(
            "pdfcrop",
            vec![
                OsString::from("--margins"),
                OsString::from(format!(
                    "{} {} {} {}",
                    args.left, args.top, args.right, args.bottom
                )),
                external::path_arg(&args.input),
                external::path_arg(&args.output),
            ],
        ),
        PdfCommand::Convert(args) => {
            let mut result = external::run_status(
                "mutool",
                vec![
                    OsString::from("convert"),
                    OsString::from("-o"),
                    external::path_arg(&args.output),
                    external::path_arg(&args.input),
                ],
            );
            if result.is_err() && external::executable_exists("magick") {
                result = external::run_status(
                    "magick",
                    vec![
                        external::path_arg(&args.input),
                        external::path_arg(&args.output),
                    ],
                );
            }
            result
        }
    }
}

fn run_ocr(input: &Path, output: Option<&Path>, lang: &str) -> Result<()> {
    let text = external::run_capture(
        "tesseract",
        vec![
            external::path_arg(input),
            OsString::from("stdout"),
            OsString::from("-l"),
            OsString::from(lang),
        ],
    )?;
    write_or_print(output, text.as_bytes())
}

fn run_qr(command: QrCommand) -> Result<()> {
    match command {
        QrCommand::Gen(args) => {
            let code = QrCode::new(args.text.as_bytes()).context("failed to encode QR text")?;
            let image = code.render::<image::Luma<u8>>().build();
            image
                .save(&args.output)
                .with_context(|| format!("failed to save QR image {}", args.output.display()))
        }
        QrCommand::Read(args) => {
            let image = image::open(&args.input)
                .with_context(|| format!("failed to open QR image {}", args.input.display()))?
                .to_luma8();
            let mut prepared = rqrr::PreparedImage::prepare(image);
            let grids = prepared.detect_grids();
            let first = grids
                .into_iter()
                .next()
                .context("no QR code detected in image")?;
            let (_, content) = first.decode().context("failed to decode QR code")?;
            println!("{content}");
            Ok(())
        }
    }
}

fn run_base64(command: Base64Command) -> Result<()> {
    match command {
        Base64Command::Encode(args) => {
            let bytes = fs::read(&args.input)
                .with_context(|| format!("failed to read {}", args.input.display()))?;
            write_or_print(args.output.as_deref(), STANDARD.encode(bytes).as_bytes())
        }
        Base64Command::Decode(args) => {
            let encoded = fs::read_to_string(&args.input)
                .with_context(|| format!("failed to read {}", args.input.display()))?;
            let decoded = STANDARD
                .decode(encoded.trim())
                .context("failed to decode base64 input")?;
            write_or_print(args.output.as_deref(), &decoded)
        }
    }
}

fn write_or_print(output: Option<&Path>, bytes: &[u8]) -> Result<()> {
    match output {
        Some(path) => {
            fs::write(path, bytes).with_context(|| format!("failed to write {}", path.display()))
        }
        None => {
            std::io::stdout()
                .write_all(bytes)
                .context("failed to write stdout")?;
            Ok(())
        }
    }
}

fn run_doctor() -> Result<()> {
    for tool in external::TOOLS {
        let status = if external::executable_exists(tool.name) {
            "ok"
        } else {
            "missing"
        };
        println!("{:<10} {:<8} {}", tool.name, status, tool.purpose);
    }
    Ok(())
}

#[allow(dead_code)]
fn _keep_file_io_args_public(_: FileIoArgs) {}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn data_uri_includes_mime_type_and_base64_payload() -> Result<()> {
        let dir = tempdir()?;
        let path = dir.path().join("sample.txt");
        fs::write(&path, "hello")?;

        assert_eq!(data_uri(&path)?, "data:text/plain;base64,aGVsbG8=");
        Ok(())
    }

    #[test]
    fn resize_image_writes_expected_dimensions() -> Result<()> {
        let dir = tempdir()?;
        let input = dir.path().join("input.png");
        let output = dir.path().join("output.png");
        DynamicImage::new_rgb8(40, 20).save(&input)?;

        resize_image(&input, &output, 10, 10, ResizeMode::Fit)?;

        assert_eq!(image::open(output)?.dimensions(), (10, 5));
        Ok(())
    }

    #[test]
    fn crop_rejects_out_of_bounds_rectangle() -> Result<()> {
        let dir = tempdir()?;
        let input = dir.path().join("input.png");
        let output = dir.path().join("output.png");
        DynamicImage::new_rgb8(10, 10).save(&input)?;

        let error = crop_image(&input, &output, 8, 8, 4, 4).unwrap_err();

        assert!(error.to_string().contains("exceeds image bounds"));
        Ok(())
    }
}
