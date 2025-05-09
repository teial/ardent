/// A color in RGBA format, with each component in the range [0.0, 1.0].
///
/// Used across the system for fills, strokes, and effects.
#[derive(Clone, Copy, Debug)]
pub struct Color(pub f32, pub f32, pub f32, pub f32);

impl Color {
    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self(r, g, b, 1.0)
    }

    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self(r, g, b, a)
    }

    pub fn white() -> Self {
        Self(1.0, 1.0, 1.0, 1.0)
    }

    pub fn black() -> Self {
        Self(0.0, 0.0, 0.0, 1.0)
    }

    pub fn transparent() -> Self {
        Self(0.0, 0.0, 0.0, 0.0)
    }
}
