//! Ultralight Code - Lightweight VS Code Alternative
//! 
//! Main entry point for the application

mod app;
mod window;
#[cfg(feature = "renderer")]
mod renderer;
mod plugin;
mod fs;
mod config;

use anyhow::Result;
use env_logger::Env;
use log::info;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    Env::default()
        .filter_or("ULTRALIGHT_CODE_LOG", "info")
        .write_style_or("ULTRALIGHT_CODE_LOG_STYLE", "always");
    
    env_logger::init();
    info!("Starting Ultralight Code...");

    // Load configuration
    let config = config::Config::load()?;
    info!("Configuration loaded");

    // Initialize application
    let mut app = app::Application::new(config)?;
    
    // Run main loop
    app.run().await?;

    info!("Ultralight Code shutdown complete");
    Ok(())
}
