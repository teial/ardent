use ardent_core::prelude::Shape;
use lyon::tessellation::{FillTessellator, VertexBuffers};

use crate::geometry::Vertex;
use crate::tesselate::Tesselate;

/// The rendering engine that tessellates and prepares UI geometry for GPU rendering.
///
/// This struct owns the `lyon` tessellator and manages the process of walking
/// the scene graph, extracting shape data, and turning it into a list of
/// triangles that can be uploaded to the GPU.
///
/// For now, only filled rectangles are supported. Future versions will handle
/// strokes, paths, and text as vector geometry.
pub struct Renderer {
    tessellator: FillTessellator,
}

impl Renderer {
    /// Creates a new, stateless renderer.
    pub fn new() -> Self {
        Self {
            tessellator: FillTessellator::new(),
        }
    }

    /// Tessellates a single shape into vertices.
    ///
    /// This uses the shape’s `Tesselate` implementation to generate
    /// a vector path and convert it into triangles.
    pub fn tessellate_shape(&mut self, shape: &Shape) -> Vec<Vertex> {
        let mut geometry: VertexBuffers<Vertex, u16> = VertexBuffers::new();
        match shape {
            Shape::Rect(rect) => rect.tesselate(&mut geometry, &mut self.tessellator),
            // Add more shape variants here (e.g. Circle, Path, Text)
        }
        geometry.vertices
    }
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}
