mod rect;

pub use rect::Rect;

/// Represents a geometric shape that can be rendered on screen.
///
/// A `Shape` defines the visible geometry of a node in the scene graph.
/// Unlike raster images, these shapes are resolution-independent and
/// scalable without loss of quality. Shapes are eventually tessellated
/// into GPU-friendly triangles for rendering.
#[derive(Clone, Debug)]
pub enum Shape {
    /// A simple rectangle defined by its width and height.
    ///
    /// Rectangles are axis-aligned by default. Transformations such as
    /// rotation or scaling can be applied separately via the node's `Transform`.
    Rect(rect::Rect),
}
