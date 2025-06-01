// neurokernel/src/holograph/hologram_engine.rs

use crate::holograph::types::{HoloObject, HoloScene};
use std::collections::HashMap;

pub struct HologramEngine {
    pub scenes: HashMap<String, HoloScene>,
}

impl HologramEngine {
    pub fn new() -> Self {
        Self {
            scenes: HashMap::new(),
        }
    }

    pub fn create_scene(&mut self, name: &str) {
        self.scenes.insert(name.to_string(), HoloScene::new(name));
    }

    pub fn add_object(&mut self, scene: &str, obj: HoloObject) {
        if let Some(scene) = self.scenes.get_mut(scene) {
            scene.objects.push(obj);
        }
    }

    pub fn render(&self, scene: &str) {
        if let Some(s) = self.scenes.get(scene) {
            println!("🔭 Rendering holographic scene '{}'", scene);
            for obj in &s.objects {
                println!("Rendering object: {:?}", obj);
            }
        }
    }
}
