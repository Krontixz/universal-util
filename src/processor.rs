use oxipng::{optimize, InFile, OutFile, Options};
use image::{imageops::FilterType};
use std::path::{Path, PathBuf};
use std::fs;

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
    
    // Save to temp then rename to avoid corruption
    let temp_path = path.with_extension("tmp_upscale");
    upscaled.save(&temp_path)?;
    fs::rename(temp_path, path)?;
    Ok(())
}
