use std::fs::{self, File};
use std::io::{Read, Write};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ThemeConfig {
    pub theme: String,            // e.g. "light", "dark", "amoled"
    pub font: String,             // e.g. "Pacifico", "Monospace"
    pub corner_radius: u8,        // 0 - 50
    pub transparency: u8,         // 0 - 100
}

impl Default for ThemeConfig {
    fn default() -> Self {
        ThemeConfig {
            theme: "dark".to_string(),
            font: "System Default".to_string(),
            corner_radius: 10,
            transparency: 30,
        }
    }
}

pub struct ThemeCustomizer;

impl ThemeCustomizer {
    const CONFIG_PATH: &'static str = "theme_config.json";

    pub fn load_config() -> ThemeConfig {
        if let Ok(mut file) = File::open(Self::CONFIG_PATH) {
            let mut contents = String::new();
            if file.read_to_string(&mut contents).is_ok() {
                if let Ok(config) = serde_json::from_str::<ThemeConfig>(&contents) {
                    return config;
                }
            }
        }
        ThemeConfig::default()
    }

    pub fn save_config(config: &ThemeConfig) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(config).unwrap();
        fs::write(Self::CONFIG_PATH, json)?;
        Ok(())
    }

    pub fn apply_theme(config: &ThemeConfig) {
        println!("🖌️ Applying Theme: {:?}", config);
        // Connect this to actual UI rendering in your OS
        // Example: update GTK/egui/TUI styles using the values
    }
}
