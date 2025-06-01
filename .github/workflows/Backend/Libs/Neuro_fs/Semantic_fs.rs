// neurokernel/src/fs/semantic_fs.rs

use std::collections::{HashMap, HashSet};

/// A semantic "file" — a memory unit tagged with topics and intent
#[derive(Debug, Clone)]
pub struct SemanticFile {
    pub id: String,
    pub content: String,
    pub tags: Vec<String>,
    pub last_accessed: Option<String>,
}

/// The Semantic File System — stores data based on meaning, not directories
pub struct SemanticFS {
    files: HashMap<String, SemanticFile>,
    tag_index: HashMap<String, HashSet<String>>, // tag -> file_ids
}

impl SemanticFS {
    /// Creates a new empty semantic file system
    pub fn new() -> Self {
        SemanticFS {
            files: HashMap::new(),
            tag_index: HashMap::new(),
        }
    }

    /// Stores a new file with semantic tags
    pub fn store_file(&mut self, id: &str, content: &str, tags: Vec<&str>) {
        let tag_list = tags.iter().map(|s| s.to_string()).collect::<Vec<String>>();
        let file = SemanticFile {
            id: id.to_string(),
            content: content.to_string(),
            tags: tag_list.clone(),
            last_accessed: None,
        };

        self.files.insert(id.to_string(), file);

        for tag in tag_list {
            self.tag_index
                .entry(tag)
                .or_insert_with(HashSet::new)
                .insert(id.to_string());
        }
    }

    /// Retrieves all files related to a given semantic tag
    pub fn search_by_tag(&self, tag: &str) -> Vec<&SemanticFile> {
        if let Some(ids) = self.tag_index.get(tag) {
            ids.iter()
                .filter_map(|id| self.files.get(id))
                .collect::<Vec<_>>()
        } else {
            vec![]
        }
    }

    /// Deletes a semantic file and cleans up the tag index
    pub fn delete_file(&mut self, id: &str) {
        if let Some(file) = self.files.remove(id) {
            for tag in file.tags {
                if let Some(set) = self.tag_index.get_mut(&tag) {
                    set.remove(id);
                    if set.is_empty() {
                        self.tag_index.remove(&tag);
                    }
                }
            }
        }
    }

    /// Lists all files with their semantic context
    pub fn list_files(&self) -> Vec<&SemanticFile> {
        self.files.values().collect()
    }

    /// Returns all unique tags present in the file system
    pub fn list_tags(&self) -> Vec<String> {
        self.tag_index.keys().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semantic_fs_store_and_search() {
        let mut fs = SemanticFS::new();
        fs.store_file("1", "Learn Rust", vec!["programming", "rust", "education"]);
        fs.store_file("2", "Poetry Collection", vec!["art", "writing"]);

        let results = fs.search_by_tag("rust");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "1");

        let tags = fs.list_tags();
        assert!(tags.contains(&"programming".to_string()));
        assert!(tags.contains(&"writing".to_string()));
    }
}
