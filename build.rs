use std::env;
use std::path::PathBuf;

fn main() {
    // 1. Compile the Slint UI to Rust code
    slint_build::compile("src/ui/main.slint").unwrap();

    // 2. Add Windows Version Information (Reduces "Suspicious App" flags)
    // Only run this when targeting Windows
    if env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winres::WindowsResource::new();
        res.set_icon("icon.ico"); // Make sure to place an 'icon.ico' in your root folder
        res.set("ProductName", "OmniTools Universal");
        res.set("CompanyName", "YourName/YourGithub");
        res.set("FileDescription", "High-Performance Local File Utility");
        res.set("LegalCopyright", "Copyright © 2026");
        res.compile().unwrap();
    }

    // 3. Optional: Link C++ files if you add custom processor.cpp logic later
    /*
    cc::Build::new()
        .cpp(true)
        .file("src/cpp/bridge.cpp")
        .compile("bridge");
    */
}
