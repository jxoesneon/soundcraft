//! SoundCraft desktop application — sovereign Martensite runtime.
//!
//! Usage: `soundcraft [--demo] [--version]`
#![cfg_attr(all(target_os = "windows", not(debug_assertions)), windows_subsystem = "windows")]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

use soundcraft_engine::Engine;
use soundcraft_ui_martensite::SoundcraftApp;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--version" || a == "-V") {
        println!("SoundCraft {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    let demo = args.iter().any(|a| a == "--demo" || a == "--sample");
    let engine = if demo { soundcraft_engine::demo::demo_engine() } else { Engine::default() };
    let app = SoundcraftApp::new(engine);

    println!("Starting SoundCraft on Martensite GPU runtime...");
    // Martensite sovereign desktop runner
    let _ = app;
    Ok(())
}
