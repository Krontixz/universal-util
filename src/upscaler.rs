use image::{DynamicImage, imageops::FilterType};
use std::fs;
use std::path::Path;

pub fn upscale_image(path: &Path, target_width: u32, target_height: u32) -> anyhow::Result<()> {
    // 1. Load the original file
    let img = image::open(path)?;

    // 2. Perform the "Clean Pixel" Resize
    // FilterType::Nearest ensures no blurring or warping occurs.
    let upscaled = img.resize(target_width, target_height, FilterType::Nearest);

    // 3. Save to a temporary location first for safety
    let temp_path = path.with_extension("tmp_upscale");
    upscaled.save(&temp_path)?;

    // 4. Overwrite the original (as requested: "replaces it with same file names")
    // std::fs::rename is atomic on Windows, ensuring no data loss.
    fs::rename(temp_path, path)?;

    Ok(())
}
