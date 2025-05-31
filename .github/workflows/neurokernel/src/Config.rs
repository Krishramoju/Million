//! neurokernel/src/config.rs
//! User and system configuration loader and manager.

use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct Config {
    settings: HashMap<String, String>,
    config_path: String,
}

impl Config {
    pub fn new(config_path: &str) -> Self {
        Config {
            settings: HashMap::new(),
            config_path: config_path.to_string(),
        }
    }

    pub fn load(&mut self) -> Result<(), String> {
        if !Path::new(&self.config_path).exists() {
            return Err(format!("Config file '{}' not found.", self.config_path));
        }
        let content = fs::read_to_string(&self.config_path)
            .map_err(|e| format!("Failed to read config file: {}", e))?;

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some((key, value)) = line.split_once('=') {
                self.settings.insert(key.trim().to_string(), value.trim().to_string());
            }
        }
        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.settings.get(key)
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.settings.insert(key.to_string(), value.to_string());
    }

    pub fn save(&self) -> Result<(), String> {
        let mut content = String::new();
        for (key, value) in &self.settings {
            content.push_str(&format!("{}={}\n", key, value));
        }
        fs::write(&self.config_path, content)
            .map_err(|e| format!("Failed to write config file: {}", e))?;
        Ok(())
    }
}
