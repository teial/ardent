mod color;
mod fill;
mod gradient;
mod stroke;

pub use color::Color;
pub use fill::Fill;
pub use gradient::Gradient;
pub use stroke::Stroke;

/// Defines the overall appearance of a shape.
///
/// A style combines fill and stroke settings to describe how a shape
/// is rendered. If either is `None`, that visual aspect is omitted.
#[derive(Clone, Debug)]
pub struct Style {
    /// Optional fill for the shape interior.
    pub fill: Option<Fill>,

    /// Optional stroke for the shape border.
    pub stroke: Option<Stroke>,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            fill: None,
            stroke: None,
        }
    }
}
