slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_switch_tab(move |index| {
        let ui = ui_handle.unwrap();
        ui.set_active_tab(index);
        
        let status = match index {
            0 => "Ready to convert formats...",
            1 => "Waiting for images to upscale...",
            2 => "Lossless optimization mode active.",
            3 => "Open any file to inspect contents.",
            _ => "Package your app for distribution.",
        };
        ui.set_status_text(status.into());
    });

    let ui_handle_action = ui.as_weak();
    ui.on_process_action(move |action_type| {
        let ui = ui_handle_action.unwrap();
        if action_type == "select" {
            ui.set_status_text("Opening File Dialog...".into());
            // Here we will integrate 'rfd' (Rust File Dialog) in the next step
        }
    });

    ui.run()
}
