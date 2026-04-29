//! File system module - file operations and watching

use anyhow::Result;
use log::info;
use std::path::{Path, PathBuf};

/// File system manager for file operations
pub struct FileSystem {
    // File watcher will be here
}

impl FileSystem {
    /// Create a new file system manager
    pub fn new() -> Self {
        info!("Initializing file system manager...");
        Self {}
    }

    /// Read a file's contents
    pub fn read_file(&self, path: &Path) -> Result<String> {
        Ok(std::fs::read_to_string(path)?)
    }

    /// Write contents to a file
    pub fn write_file(&self, path: &Path, content: &str) -> Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Delete a file
    pub fn delete_file(&self, path: &Path) -> Result<()> {
        std::fs::remove_file(path)?;
        Ok(())
    }

    /// List directory contents
    pub fn list_dir(&self, path: &Path) -> Result<Vec<PathBuf>> {
        let mut entries = Vec::new();
        
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            entries.push(entry.path());
        }
        
        Ok(entries)
    }

    /// Check if a path exists
    pub fn exists(&self, path: &Path) -> bool {
        path.exists()
    }

    /// Check if a path is a directory
    pub fn is_dir(&self, path: &Path) -> bool {
        path.is_dir()
    }

    /// Get the parent directory of a path
    pub fn parent(&self, path: &Path) -> Option<PathBuf> {
        path.parent().map(|p| p.to_path_buf())
    }

    /// Join path components
    pub fn join(&self, base: &Path, components: &[&str]) -> PathBuf {
        let mut result = base.to_path_buf();
        for component in components {
            result.push(component);
        }
        result
    }
}

impl Default for FileSystem {
    fn default() -> Self {
        Self::new()
    }
}
