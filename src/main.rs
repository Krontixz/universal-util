mod processor; // Import our processor module

use slint::ComponentHandle;
use rfd::FileDialog;
use rayon::prelude::*; // For super-fast bulk processing
use std::path::PathBuf;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    // --- Tab Switching Logic ---
    let ui_handle = ui.as_weak();
    ui.on_switch_tab(move |index| {
        let ui = ui_handle.unwrap();
        ui.set_active_tab(index);
        let status = match index {
            0 => "Ready to convert formats...",
            1 => "Clean Pixel Upscaler active (Select images).",
            2 => "Lossless Compression mode active.",
            3 => "File Viewer ready.",
            _ => "Packager ready.",
        };
        ui.set_status_text(status.into());
    });

    // --- Main Action Logic ---
    let ui_handle_action = ui.as_weak();
    ui.on_process_action(move |action_type| {
        let ui = ui_handle_action.unwrap();
        let tab = ui.get_active_tab();

        // 1. Open Native File Dialog
        let files = FileDialog::new()
            .set_title("Select Files to Process")
            .pick_files()
            .unwrap_or_default();

        if files.is_empty() {
            ui.set_status_text("No files selected.".into());
            return;
        }

        ui.set_status_text(format!("Processing {} files...", files.len()).into());

        // 2. Process based on which Tab is open
        // We use spawn to keep the UI responsive while working
        std::thread::spawn(move || {
            let results: Vec<_> = files.into_par_iter().map(|path| {
                match tab {
                    1 => processor::upscale_image(&path, 1024, 1024), // Bulk Upscale
                    2 => processor::compress_png_lossless(&path),    // Bulk Compress
                    _ => Ok(()),
                }
            }).collect();

            // 3. Update UI when done
            let success_count = results.iter().filter(|r| r.is_ok()).count();
            let _ = slint::invoke_from_event_loop(move || {
                ui.set_status_text(format!("Finished! Processed {}/{} files successfully.", success_count, results.len()).into());
            });
        });
    });

    ui.run()
}
