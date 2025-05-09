use ardent_core::prelude::{Scene, Shape};
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

    /// Traverses the scene graph and tessellates all visible shapes into vertex data.
    ///
    /// This method is CPU-side only. It does not upload anything to the GPU —
    /// it simply converts all renderable shapes into a flat list of triangles
    /// that can be sent to a GPU buffer.
    ///
    /// Only `Shape::Rect` is currently supported.
    pub fn tessellate_scene(&mut self, scene: &Scene) -> Vec<Vertex> {
        let mut geometry: VertexBuffers<Vertex, u16> = VertexBuffers::new();
        scene.traverse(|node| {
            if let Some(shape) = node.shape() {
                match shape {
                    Shape::Rect(rect) => rect.tesselate(&mut geometry, &mut self.tessellator),
                }
            }
        });
        geometry.vertices
    }
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}
