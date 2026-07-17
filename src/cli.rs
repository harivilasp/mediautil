use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(name = "mediautil", version, about = "Local media utility shell")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Image crop, resize, conversion, icon, and data URI utilities.
    Image {
        #[command(subcommand)]
        command: ImageCommand,
    },
    /// PDF helpers powered by common local tools.
    Pdf {
        #[command(subcommand)]
        command: PdfCommand,
    },
    /// OCR text from images or PDFs using local OCR tools.
    Ocr(OcrArgs),
    /// QR code generation and reading.
    Qr {
        #[command(subcommand)]
        command: QrCommand,
    },
    /// Base64 encode/decode any file, including PDFs.
    Base64 {
        #[command(subcommand)]
        command: Base64Command,
    },
    /// Print available external integrations and their install status.
    Doctor,
}

#[derive(Debug, Subcommand)]
pub enum ImageCommand {
    /// Resize an image.
    Resize(ResizeArgs),
    /// Crop an image.
    Crop(CropArgs),
    /// Convert an image to another format inferred from the output extension.
    Convert(ConvertArgs),
    /// Generate a .ico file from an image.
    Icon(IconArgs),
    /// Convert an image into a data URI.
    DataUri(DataUriArgs),
}

#[derive(Debug, Args)]
pub struct ResizeArgs {
    pub input: PathBuf,
    pub output: PathBuf,
    #[arg(long)]
    pub width: u32,
    #[arg(long)]
    pub height: u32,
    #[arg(long, value_enum, default_value_t = ResizeMode::Fit)]
    pub mode: ResizeMode,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ResizeMode {
    Fit,
    Exact,
}

#[derive(Debug, Args)]
pub struct CropArgs {
    pub input: PathBuf,
    pub output: PathBuf,
    #[arg(long)]
    pub x: u32,
    #[arg(long)]
    pub y: u32,
    #[arg(long)]
    pub width: u32,
    #[arg(long)]
    pub height: u32,
}

#[derive(Debug, Args)]
pub struct ConvertArgs {
    pub input: PathBuf,
    pub output: PathBuf,
}

#[derive(Debug, Args)]
pub struct IconArgs {
    pub input: PathBuf,
    pub output: PathBuf,
    #[arg(long, default_value_t = 256)]
    pub size: u32,
}

#[derive(Debug, Args)]
pub struct DataUriArgs {
    pub input: PathBuf,
}

#[derive(Debug, Subcommand)]
pub enum PdfCommand {
    /// Extract text using pdftotext.
    Text(PdfTextArgs),
    /// Split pages using qpdf page ranges, for example 1-3 or 2,4,6.
    Split(PdfSplitArgs),
    /// Merge PDFs using qpdf.
    Merge(PdfMergeArgs),
    /// Crop pages using pdfcrop.
    Crop(PdfCropArgs),
    /// Convert PDF pages using mutool or ImageMagick.
    Convert(PdfConvertArgs),
}

#[derive(Debug, Args)]
pub struct PdfTextArgs {
    pub input: PathBuf,
}

#[derive(Debug, Args)]
pub struct PdfSplitArgs {
    pub input: PathBuf,
    pub output: PathBuf,
    #[arg(long)]
    pub pages: String,
}

#[derive(Debug, Args)]
pub struct PdfMergeArgs {
    #[arg(required = true)]
    pub inputs: Vec<PathBuf>,
    #[arg(short, long)]
    pub output: PathBuf,
}

#[derive(Debug, Args)]
pub struct PdfCropArgs {
    pub input: PathBuf,
    pub output: PathBuf,
    #[arg(long, default_value_t = 0)]
    pub left: i32,
    #[arg(long, default_value_t = 0)]
    pub top: i32,
    #[arg(long, default_value_t = 0)]
    pub right: i32,
    #[arg(long, default_value_t = 0)]
    pub bottom: i32,
}

#[derive(Debug, Args)]
pub struct PdfConvertArgs {
    pub input: PathBuf,
    pub output: PathBuf,
}

#[derive(Debug, Args)]
pub struct OcrArgs {
    pub input: PathBuf,
    #[arg(short, long)]
    pub output: Option<PathBuf>,
    #[arg(short, long, default_value = "eng")]
    pub lang: String,
}

#[derive(Debug, Subcommand)]
pub enum QrCommand {
    /// Generate a QR code PNG.
    Gen(QrGenArgs),
    /// Read a QR code from an image.
    Read(QrReadArgs),
}

#[derive(Debug, Args)]
pub struct QrGenArgs {
    pub text: String,
    pub output: PathBuf,
}

#[derive(Debug, Args)]
pub struct QrReadArgs {
    pub input: PathBuf,
}

#[derive(Debug, Subcommand)]
pub enum Base64Command {
    Encode(FileIoArgs),
    Decode(FileIoArgs),
}

#[derive(Debug, Args)]
pub struct FileIoArgs {
    pub input: PathBuf,
    #[arg(short, long)]
    pub output: Option<PathBuf>,
}
