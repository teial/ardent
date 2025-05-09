use super::{Color, Gradient};

/// Describes how a shape is filled.
///
/// A fill defines the interior appearance of a shape — typically using a solid
/// color, but later extensible to include gradients or image patterns.
#[derive(Clone, Debug)]
pub struct Fill {
    /// The fill color of the shape.
    pub color: Color,

    /// Placeholder for future gradient support.
    pub gradient: Option<Gradient>, // Currently unused
}
