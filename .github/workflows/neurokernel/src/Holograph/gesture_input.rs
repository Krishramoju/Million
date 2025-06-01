// neurokernel/src/holograph/gesture_input.rs

#[derive(Debug)]
pub enum Gesture {
    PinchZoomIn,
    PinchZoomOut,
    SwipeLeft,
    SwipeRight,
    RotateCW,
    RotateCCW,
}

pub fn interpret_gesture(data: Vec<f32>) -> Option<Gesture> {
    if data.len() < 2 {
        return None;
    }

    let dx = data[0];
    let dy = data[1];

    if dx > 0.7 {
        Some(Gesture::SwipeRight)
    } else if dx < -0.7 {
        Some(Gesture::SwipeLeft)
    } else if dy > 0.7 {
        Some(Gesture::PinchZoomIn)
    } else if dy < -0.7 {
        Some(Gesture::PinchZoomOut)
    } else {
        None
    }
}
