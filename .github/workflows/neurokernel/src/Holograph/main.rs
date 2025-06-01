mod holograph {
    pub mod hologram_engine;
    pub mod types;
    pub mod gesture_input;
}

use holograph::hologram_engine::*;
use holograph::types::*;

fn main() {
    let mut engine = HologramEngine::new();

    engine.create_scene("main");

    engine.add_object(
        "main",
        HoloObject {
            id: "globe_1".into(),
            shape: "sphere".into(),
            position: (0.0, 1.0, -2.0),
            rotation: (0.0, 45.0, 0.0),
            scale: (1.0, 1.0, 1.0),
            material: "holo-glass".into(),
        },
    );

    engine.render("main");
}
