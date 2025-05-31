//! neurokernel/src/fe.rs
//! Core rendering engine for visual UI elements in Neurokernel OS.

use std::collections::HashMap;

/// Represents a visual component on screen.
pub struct Widget {
    pub id: String,
    pub content: String,
    pub style: HashMap<String, String>,
}

impl Widget {
    pub fn new(id: &str, content: &str) -> Self {
        Self {
            id: id.to_string(),
            content: content.to_string(),
            style: HashMap::new(),
        }
    }

    pub fn set_style(&mut self, key: &str, value: &str) {
        self.style.insert(key.to_string(), value.to_string());
    }

    pub fn render(&self) {
        let mut visual = format!("📦 [{}]: {}\n", self.id, self.content);
        for (k, v) in &self.style {
            visual.push_str(&format!("    Style - {}: {}\n", k, v));
        }
        println!("{}", visual);
    }
}

/// Frontend engine managing widgets.
pub struct FEEngine {
    pub widgets: HashMap<String, Widget>,
}

impl FEEngine {
    pub fn new() -> Self {
        Self {
            widgets: HashMap::new(),
        }
    }

    pub fn add_widget(&mut self, widget: Widget) {
        self.widgets.insert(widget.id.clone(), widget);
    }

    pub fn render_all(&self) {
        println!("🎨 Rendering UI...");
        for widget in self.widgets.values() {
            widget.render();
        }
    }

    pub fn update_content(&mut self, id: &str, new_content: &str) {
        if let Some(widget) = self.widgets.get_mut(id) {
            widget.content = new_content.to_string();
        }
    }

    pub fn set_widget_style(&mut self, id: &str, key: &str, value: &str) {
        if let Some(widget) = self.widgets.get_mut(id) {
            widget.set_style(key, value);
        }
    }
}
