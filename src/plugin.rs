//! Plugin system module - VS Code extension compatibility layer

use crate::config::Config;
use anyhow::Result;
use log::{info, warn};
use std::collections::HashMap;
use std::path::PathBuf;

/// Plugin host for managing and executing VS Code extensions
pub struct PluginHost {
    config: Config,
    plugins: HashMap<String, Plugin>,
    runtime: rquickjs::Runtime,
}

/// Represents a loaded plugin
#[derive(Debug, Clone)]
pub struct Plugin {
    pub id: String,
    pub name: String,
    pub version: String,
    pub path: PathBuf,
    pub enabled: bool,
    pub contributes: PluginContributions,
}

/// Plugin contributions (commands, menus, etc.)
#[derive(Debug, Clone, Default)]
pub struct PluginContributions {
    pub commands: Vec<CommandContribution>,
    pub menus: Vec<MenuContribution>,
    pub languages: Vec<LanguageContribution>,
}

/// Command contribution from a plugin
#[derive(Debug, Clone)]
pub struct CommandContribution {
    pub command: String,
    pub title: String,
}

/// Menu contribution from a plugin
#[derive(Debug, Clone)]
pub struct MenuContribution {
    pub menu: String,
    pub command: String,
}

/// Language contribution from a plugin
#[derive(Debug, Clone)]
pub struct LanguageContribution {
    pub id: String,
    pub aliases: Vec<String>,
    pub extensions: Vec<String>,
}

impl PluginHost {
    /// Create a new plugin host
    pub fn new(config: &Config) -> Result<Self> {
        info!("Initializing plugin host...");
        
        let runtime = rquickjs::Runtime::new()?;
        
        Ok(Self {
            config: config.clone(),
            plugins: HashMap::new(),
            runtime,
        })
    }

    /// Load all plugins from the plugins directory
    pub async fn load_all_plugins(&mut self) -> Result<()> {
        if !self.config.plugins.enabled {
            info!("Plugins are disabled in configuration");
            return Ok(());
        }

        let plugins_dir = Self::plugins_directory()?;
        
        if !plugins_dir.exists() {
            std::fs::create_dir_all(&plugins_dir)?;
            info!("Created plugins directory at {:?}", plugins_dir);
            return Ok(());
        }

        info!("Loading plugins from {:?}", plugins_dir);
        
        // Scan for .vsix files and plugin directories
        for entry in std::fs::read_dir(&plugins_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().map_or(false, |ext| ext == "vsix") {
                // Load VS Code extension package
                self.load_vsix_plugin(&path).await?;
            } else if path.is_dir() {
                // Load unpacked plugin
                self.load_unpacked_plugin(&path).await?;
            }
        }

        info!("Loaded {} plugins", self.plugins.len());
        Ok(())
    }

    /// Load a VSIX plugin package
    async fn load_vsix_plugin(&mut self, path: &PathBuf) -> Result<()> {
        info!("Loading VSIX plugin: {:?}", path);
        
        // VSIX files are ZIP archives containing:
        // - extension.vsixmanifest (metadata)
        // - extension/ (actual extension code)
        // - package.json (extension manifest)
        
        // Extract and parse package.json
        // Initialize the extension in QuickJS runtime
        
        Ok(())
    }

    /// Load an unpacked plugin directory
    async fn load_unpacked_plugin(&mut self, path: &PathBuf) -> Result<()> {
        info!("Loading unpacked plugin: {:?}", path);
        
        let package_json_path = path.join("package.json");
        
        if !package_json_path.exists() {
            warn!("No package.json found in {:?}", path);
            return Ok(());
        }

        // Parse package.json to get plugin metadata
        let content = std::fs::read_to_string(&package_json_path)?;
        let package_json: serde_json::Value = serde_json::from_str(&content)?;
        
        let plugin_id = package_json["name"]
            .as_str()
            .unwrap_or("unknown")
            .to_string();
        
        let plugin = Plugin {
            id: plugin_id.clone(),
            name: package_json["displayName"]
                .as_str()
                .unwrap_or(&plugin_id)
                .to_string(),
            version: package_json["version"]
                .as_str()
                .unwrap_or("0.0.0")
                .to_string(),
            path: path.clone(),
            enabled: true,
            contributes: self.parse_contributions(&package_json)?,
        };

        self.plugins.insert(plugin_id, plugin);
        
        Ok(())
    }

    /// Parse plugin contributions from package.json
    fn parse_contributions(&self, package_json: &serde_json::Value) -> Result<PluginContributions> {
        let mut contributions = PluginContributions::default();
        
        if let Some(contributes) = package_json.get("contributes") {
            // Parse commands
            if let Some(commands) = contributes.get("commands").and_then(|v| v.as_array()) {
                for cmd in commands {
                    contributions.commands.push(CommandContribution {
                        command: cmd["command"].as_str().unwrap_or("").to_string(),
                        title: cmd["title"].as_str().unwrap_or("").to_string(),
                    });
                }
            }
            
            // Parse menus
            if let Some(menus) = contributes.get("menus").and_then(|v| v.as_object()) {
                for (menu_name, menu_items) in menus {
                    if let Some(items) = menu_items.as_array() {
                        for item in items {
                            contributions.menus.push(MenuContribution {
                                menu: menu_name.clone(),
                                command: item["command"].as_str().unwrap_or("").to_string(),
                            });
                        }
                    }
                }
            }
            
            // Parse languages
            if let Some(languages) = contributes.get("languages").and_then(|v| v.as_array()) {
                for lang in languages {
                    contributions.languages.push(LanguageContribution {
                        id: lang["id"].as_str().unwrap_or("").to_string(),
                        aliases: lang.get("aliases")
                            .and_then(|v| v.as_array())
                            .map(|arr| arr.iter()
                                .filter_map(|v| v.as_str())
                                .map(String::from)
                                .collect())
                            .unwrap_or_default(),
                        extensions: lang.get("extensions")
                            .and_then(|v| v.as_array())
                            .map(|arr| arr.iter()
                                .filter_map(|v| v.as_str())
                                .map(String::from)
                                .collect())
                            .unwrap_or_default(),
                    });
                }
            }
        }
        
        Ok(contributions)
    }

    /// Enable a plugin
    pub fn enable_plugin(&mut self, plugin_id: &str) -> Result<()> {
        if let Some(plugin) = self.plugins.get_mut(plugin_id) {
            plugin.enabled = true;
            info!("Enabled plugin: {}", plugin_id);
        }
        Ok(())
    }

    /// Disable a plugin
    pub fn disable_plugin(&mut self, plugin_id: &str) -> Result<()> {
        if let Some(plugin) = self.plugins.get_mut(plugin_id) {
            plugin.enabled = false;
            info!("Disabled plugin: {}", plugin_id);
        }
        Ok(())
    }

    /// Get all loaded plugins
    pub fn list_plugins(&self) -> Vec<&Plugin> {
        self.plugins.values().collect()
    }

    /// Get plugins directory path
    fn plugins_directory() -> Result<PathBuf> {
        let data_dir = dirs::data_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine data directory"))?;
        
        Ok(data_dir.join("ultralight_code").join("plugins"))
    }
}
