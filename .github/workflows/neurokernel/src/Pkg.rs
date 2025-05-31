//! neurokernel/src/pkg.rs
//! Lightweight package manager for installing, updating, and removing modules/apps in Neurokernel OS.

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub description: String,
    pub installed: bool,
}

pub struct PackageManager {
    registry: HashMap<String, Package>,
    installed: Vec<String>,
}

impl PackageManager {
    pub fn new() -> Self {
        let mut manager = Self {
            registry: HashMap::new(),
            installed: Vec::new(),
        };

        // Preload with core packages
        manager.register(Package {
            name: "cortex-ui".to_string(),
            version: "1.0.0".to_string(),
            description: "Core UI system for Neurokernel".to_string(),
            installed: false,
        });

        manager.register(Package {
            name: "neuroscript-lang".to_string(),
            version: "1.0.0".to_string(),
            description: "Natural language-style command line interface".to_string(),
            installed: false,
        });

        manager.register(Package {
            name: "brainteaser-module".to_string(),
            version: "1.0.0".to_string(),
            description: "Cognitive enhancement brain training module".to_string(),
            installed: false,
        });

        manager
    }

    pub fn register(&mut self, pkg: Package) {
        self.registry.insert(pkg.name.clone(), pkg);
    }

    pub fn install(&mut self, name: &str) -> Result<(), String> {
        if let Some(pkg) = self.registry.get_mut(name) {
            if pkg.installed {
                return Err(format!("Package '{}' is already installed.", name));
            }

            pkg.installed = true;
            self.installed.push(name.to_string());
            println!("📦 Installed package: {} v{}", pkg.name, pkg.version);
            Ok(())
        } else {
            Err(format!("Package '{}' not found.", name))
        }
    }

    pub fn uninstall(&mut self, name: &str) -> Result<(), String> {
        if let Some(pkg) = self.registry.get_mut(name) {
            if !pkg.installed {
                return Err(format!("Package '{}' is not installed.", name));
            }

            pkg.installed = false;
            self.installed.retain(|n| n != name);
            println!("🗑️ Uninstalled package: {}", name);
            Ok(())
        } else {
            Err(format!("Package '{}' not found.", name))
        }
    }

    pub fn list_installed(&self) {
        println!("📦 Installed Packages:");
        for name in &self.installed {
            if let Some(pkg) = self.registry.get(name) {
                println!(" - {} v{}: {}", pkg.name, pkg.version, pkg.description);
            }
        }
    }

    pub fn search(&self, keyword: &str) {
        println!("🔍 Searching for '{}':", keyword);
        for pkg in self.registry.values() {
            if pkg.name.contains(keyword) || pkg.description.contains(keyword) {
                println!(
                    " - {} v{} ({}): {}",
                    pkg.name, pkg.version, if pkg.installed { "Installed" } else { "Available" }, pkg.description
                );
            }
        }
    }
}
