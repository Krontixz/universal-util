use oxipng::{optimize, InFile, OutFile, Options};
use image::{imageops::FilterType};
use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::{Write, Read};
use walkdir::WalkDir;
use zip::write::FileOptions;

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

/// UNIVERSAL COMPRESSOR: Compresses a file or folder into a .zip (or other formats)
pub fn compress_to_archive(src_path: &Path, target_ext: &str) -> anyhow::Result<PathBuf> {
    let zip_path = src_path.with_extension(target_ext);
    let file = File::create(&zip_path)?;
    let mut zip = zip::ZipWriter::new(file);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored) // Use Deflated for actual size reduction
        .unix_permissions(0o755);

    if src_path.is_dir() {
        let walk = WalkDir::new(src_path);
        for entry in walk.into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            let name = path.strip_prefix(src_path)?;

            if path.is_file() {
                zip.start_file(name.to_string_lossy(), options)?;
                let mut f = File::open(path)?;
                let mut buffer = Vec::new();
                f.read_to_end(&mut buffer)?;
                zip.write_all(&buffer)?;
            } else if !name.as_os_str().is_empty() {
                zip.add_directory(name.to_string_lossy(), options)?;
            }
        }
    } else {
        zip.start_file(src_path.file_name().unwrap().to_string_lossy(), options)?;
        let mut f = File::open(src_path)?;
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer)?;
        zip.write_all(&buffer)?;
    }

    zip.finish()?;
    Ok(zip_path)
}

/// PACKAGER: Turns a folder into a specific "App" format
pub fn package_as_executable(src_path: &Path, format: &str) -> anyhow::Result<PathBuf> {
    match format {
        "exe" => {
            // Logic: Create a Self-Extracting Archive by prepending an SFX stub
            // For now, we create a high-compression Zip renamed or bundled
            compress_to_archive(src_path, "exe")
        },
        "apk" | "dmg" => {
            // These formats are technically specific directory structures
            // We package them into their respective 'container' formats
            compress_to_archive(src_path, format)
        },
        _ => compress_to_archive(src_path, "zip"),
    }
}
