mod processor;

use slint::ComponentHandle;
use rfd::FileDialog;
use rayon::prelude::*;
use std::path::PathBuf;
use futures::executor::block_on; // To run the async converter in a thread

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_switch_tab(move |index| {
        let ui = ui_handle.unwrap();
        ui.set_active_tab(index);
        let status = match index {
            0 => "Universal Converter: Select any media file.",
            1 => "Pixel-Perfect Upscaler: Sharp 1024x1024 scaling.",
            2 => "Lossless Compressor: Reducing size, keeping quality.",
            3 => "File Viewer: Deep inspection of magic bytes.",
            _ => "Packager: Bundle files into executables.",
        };
        ui.set_status_text(status.into());
    });

    let ui_handle_action = ui.as_weak();
    ui.on_process_action(move |action_type| {
        let ui = ui_handle_action.unwrap();
        let tab = ui.get_active_tab();

        let files = FileDialog::new()
            .set_title("Select Files")
            .pick_files()
            .unwrap_or_default();

        if files.is_empty() { return; }

        ui.set_status_text("Processing...".into());

        std::thread::spawn(move || {
            files.into_par_iter().for_each(|path| {
                match tab {
                    0 => { let _ = block_on(processor::convert_media(&path, "mp4")); }
                    1 => { let _ = processor::upscale_image(&path, 1024, 1024); }
                    2 => { let _ = processor::compress_png_lossless(&path); }
                    3 => { 
                        let info = processor::identify_file(&path);
                        let ui_info = ui.as_weak();
                        let _ = slint::invoke_from_event_loop(move || {
                            ui_info.unwrap().set_status_text(info.into());
                        });
                    }
                    _ => {} // Packager logic goes here
                }
            });

            let ui_final = ui.as_weak();
            let _ = slint::invoke_from_event_loop(move || {
                ui_final.unwrap().set_status_text("Operation Complete!".into());
            });
        });
    });

    ui.run()
}
