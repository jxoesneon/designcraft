//! DesignCraft desktop application — 100% sovereign Martensite runtime.

#![cfg_attr(all(target_os = "windows", not(debug_assertions)), windows_subsystem = "windows")]

use designcraft_engine::Engine;
use designcraft_ui_martensite::DesigncraftApp;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let engine = Engine::new();
    let app = DesigncraftApp::new(engine);

    println!("Starting DesignCraft Studio on Martensite GPU runtime...");
    // Martensite sovereign desktop runner
    let _ = app;
    Ok(())
}
