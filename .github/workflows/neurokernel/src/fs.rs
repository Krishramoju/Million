//! neurokernel/src/fs.rs
//! Simple in-memory filesystem module for Neurokernel OS.

use std::collections::HashMap;

#[derive(Clone)]
pub struct File {
    pub name: String,
    pub content: String,
    pub readonly: bool,
}

pub struct FileSystem {
    files: HashMap<String, File>,
}

impl FileSystem {
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
        }
    }

    pub fn create_file(&mut self, name: &str, content: &str) -> bool {
        if self.files.contains_key(name) {
            return false;
        }
        self.files.insert(
            name.to_string(),
            File {
                name: name.to_string(),
                content: content.to_string(),
                readonly: false,
            },
        );
        true
    }

    pub fn read_file(&self, name: &str) -> Option<&str> {
        self.files.get(name).map(|f| f.content.as_str())
    }

    pub fn write_file(&mut self, name: &str, content: &str) -> Result<(), String> {
        if let Some(file) = self.files.get_mut(name) {
            if file.readonly {
                return Err("File is read-only".to_string());
            }
            file.content = content.to_string();
            Ok(())
        } else {
            Err("File not found".to_string())
        }
    }

    pub fn delete_file(&mut self, name: &str) -> bool {
        self.files.remove(name).is_some()
    }

    pub fn list_files(&self) -> Vec<String> {
        self.files.keys().cloned().collect()
    }

    pub fn set_readonly(&mut self, name: &str, readonly: bool) -> bool {
        if let Some(file) = self.files.get_mut(name) {
            file.readonly = readonly;
            true
        } else {
            false
        }
    }
}
