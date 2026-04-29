//! Application module - core application logic and state management

use crate::config::Config;
use crate::window::WindowManager;
use crate::renderer::Renderer;
use crate::plugin::PluginHost;
use crate::fs::FileSystem;
use anyhow::Result;
use log::info;

/// Main application structure
pub struct Application {
    config: Config,
    window_manager: WindowManager,
    renderer: Renderer,
    plugin_host: PluginHost,
    file_system: FileSystem,
}

impl Application {
    /// Create a new application instance
    pub fn new(config: Config) -> Result<Self> {
        info!("Initializing application...");
        
        let window_manager = WindowManager::new()?;
        let renderer = Renderer::new()?;
        let plugin_host = PluginHost::new(&config)?;
        let file_system = FileSystem::new();

        Ok(Self {
            config,
            window_manager,
            renderer,
            plugin_host,
            file_system,
        })
    }

    /// Run the main application loop
    pub async fn run(&mut self) -> Result<()> {
        info!("Starting application loop...");
        
        // Load plugins
        self.plugin_host.load_all_plugins().await?;
        
        // Create main window
        self.window_manager.create_main_window(&self.renderer)?;
        
        // Start event loop
        self.event_loop().await?;
        
        Ok(())
    }

    /// Main event loop
    async fn event_loop(&mut self) -> Result<()> {
        info!("Entering event loop...");
        
        // Event handling will be implemented with winit
        // This is a placeholder for the actual event loop
        
        Ok(())
    }

    /// Get application configuration
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Get plugin host reference
    pub fn plugin_host(&self) -> &PluginHost {
        &self.plugin_host
    }

    /// Get file system reference
    pub fn file_system(&self) -> &FileSystem {
        &self.file_system
    }
}
