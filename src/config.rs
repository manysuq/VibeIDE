//! Configuration module - application settings and preferences

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use anyhow::Result;

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Window settings
    pub window: WindowConfig,
    
    /// Editor settings
    pub editor: EditorConfig,
    
    /// Plugin settings
    pub plugins: PluginConfig,
    
    /// Theme settings
    pub theme: ThemeConfig,
}

/// Window configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowConfig {
    pub width: u32,
    pub height: u32,
    pub maximized: bool,
    pub title: String,
}

/// Editor configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorConfig {
    pub font_family: String,
    pub font_size: u16,
    pub line_height: f32,
    pub tab_size: u8,
    pub word_wrap: bool,
    pub minimap_enabled: bool,
    pub line_numbers: bool,
}

/// Plugin configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfig {
    pub enabled: bool,
    pub auto_update: bool,
    pub marketplace_url: String,
}

/// Theme configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub name: String,
    pub is_dark: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            window: WindowConfig {
                width: 1200,
                height: 800,
                maximized: false,
                title: "Ultralight Code".to_string(),
            },
            editor: EditorConfig {
                font_family: "Consolas, 'Courier New', monospace".to_string(),
                font_size: 14,
                line_height: 1.5,
                tab_size: 4,
                word_wrap: true,
                minimap_enabled: true,
                line_numbers: true,
            },
            plugins: PluginConfig {
                enabled: true,
                auto_update: false,
                marketplace_url: "https://marketplace.visualstudio.com".to_string(),
            },
            theme: ThemeConfig {
                name: "Default Dark".to_string(),
                is_dark: true,
            },
        }
    }
}

impl Config {
    /// Load configuration from file or create default
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;
        
        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let config: Config = serde_json::from_str(&content)?;
            Ok(config)
        } else {
            // Create default config
            let config = Config::default();
            config.save()?;
            Ok(config)
        }
    }

    /// Save configuration to file
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;
        
        // Ensure directory exists
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(&config_path, content)?;
        
        Ok(())
    }

    /// Get configuration file path
    fn config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?;
        
        Ok(config_dir.join("ultralight_code").join("config.json"))
    }
}
