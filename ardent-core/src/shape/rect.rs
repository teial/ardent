#[derive(Clone, Debug)]
pub struct Rect {
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}
