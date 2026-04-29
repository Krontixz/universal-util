use oxipng::{optimize, InFile, OutFile, Options};
use image::{imageops::FilterType};
use std::path::{Path, PathBuf};
use std::fs;
use rust_ffmpeg::prelude::*; // Make sure to add this to Cargo.toml

/// Detects what a file actually is, regardless of extension
pub fn identify_file(path: &Path) -> String {
    match infer::get_from_path(path) {
        Ok(Some(kind)) => format!("Type: {} | Mime: {}", kind.extension(), kind.mime_type()),
        _ => "Unknown File Type".to_string(),
    }
}

/// Lossless PNG compression
pub fn compress_png_lossless(path: &Path) -> anyhow::Result<()> {
    let input = InFile::Path(path.to_path_buf());
    let output = OutFile::Path(Some(path.to_path_buf()));
    let options = Options::default(); 
    optimize(&input, &output, &options)?;
    Ok(())
}

/// Clean Pixel Upscaling (Nearest Neighbor)
pub fn upscale_image(path: &Path, width: u32, height: u32) -> anyhow::Result<()> {
    let img = image::open(path)?;
    let upscaled = img.resize(width, height, FilterType::Nearest);
    let temp_path = path.with_extension("tmp_upscale");
    upscaled.save(&temp_path)?;
    fs::rename(temp_path, path)?;
    Ok(())
}

/// Universal Media Converter (FFmpeg)
pub async fn convert_media(path: &Path, target_ext: &str) -> anyhow::Result<()> {
    let output_path = path.with_extension(target_ext);
    // This executes a local FFmpeg command
    FFmpegBuilder::new()
        .stderr_to_log()
        .input(path.to_str().unwrap())
        .output(output_path.to_str().unwrap())
        .run()
        .await?;
    Ok(())
}
