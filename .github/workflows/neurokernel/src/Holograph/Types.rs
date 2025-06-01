// neurokernel/src/holograph/types.rs

#[derive(Debug, Clone)]
pub struct HoloObject {
    pub id: String,
    pub shape: String,         // e.g. "cube", "sphere", "model"
    pub position: (f32, f32, f32),
    pub rotation: (f32, f32, f32),
    pub scale: (f32, f32, f32),
    pub material: String,      // e.g. "glass", "neon", "holo-mesh"
}

#[derive(Debug)]
pub struct HoloScene {
    pub name: String,
    pub objects: Vec<HoloObject>,
}

impl HoloScene {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            objects: vec![],
        }
    }
}
