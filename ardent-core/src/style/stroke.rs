use super::Color;

/// Describes how a shape is outlined or stroked.
///
/// The stroke defines the contour or border drawn around a shape.
#[derive(Clone, Debug)]
pub struct Stroke {
    /// The stroke color.
    pub color: Color,

    /// Width of the stroke in logical pixels.
    pub width: f32,

    /// Placeholder for stroke alignment (inside/outside/center).
    pub align: StrokeAlign,
}

/// Stroke alignment relative to the shape boundary.
#[derive(Clone, Debug)]
pub enum StrokeAlign {
    Center,
    Inside,
    Outside,
}
