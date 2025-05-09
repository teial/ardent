/// Describes how a node is positioned, scaled, and rotated in 2D space.
///
/// Transforms allow nodes to be moved, resized, or rotated relative to
/// their parent. These transformations are combined hierarchically in
/// the scene graph, enabling complex compositions from simple building blocks.
#[derive(Clone, Debug)]
pub struct Transform {
    /// The (x, y) offset from the parent's origin.
    pub translate: (f32, f32),

    /// The (x, y) scaling factor. (1.0, 1.0) means no scaling.
    pub scale: (f32, f32),

    /// Rotation in radians, clockwise around the origin.
    pub rotate: f32,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            translate: (0.0, 0.0),
            scale: (1.0, 1.0),
            rotate: 0.0,
        }
    }
}
