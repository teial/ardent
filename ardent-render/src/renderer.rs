use ardent_core::prelude::{Scene, Shape};
use lyon::path::Path;
use lyon::tessellation::{BuffersBuilder, FillOptions, FillTessellator, FillVertex, VertexBuffers};

/// A single 2D vertex to be sent to the GPU.
///
/// This is the lowest-level geometric primitive used in rendering.
/// Each vertex contains a 2D position (x, y) in local node coordinates.
/// Additional attributes like color or texture coordinates can be added later.
#[derive(Debug)]
pub struct Vertex {
    /// Position in logical (device-independent) pixels.
    pub position: [f32; 2],
}

impl Vertex {
    /// Converts a `lyon` tessellated vertex into an `ardent` vertex.
    pub fn from_fill_vertex(v: FillVertex) -> Self {
        let pos = v.position();
        Vertex {
            position: [pos.x, pos.y],
        }
    }
}

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
                    Shape::Rect { width, height } => {
                        let mut path_builder = Path::builder();
                        path_builder.begin(lyon::math::point(0.0, 0.0));
                        path_builder.line_to(lyon::math::point(*width, 0.0));
                        path_builder.line_to(lyon::math::point(*width, *height));
                        path_builder.line_to(lyon::math::point(0.0, *height));
                        path_builder.close();
                        let path = path_builder.build();
                        let _ = self.tessellator.tessellate_path(
                            &path,
                            &FillOptions::default(),
                            &mut BuffersBuilder::new(&mut geometry, |v: FillVertex| {
                                Vertex::from_fill_vertex(v)
                            }),
                        );
                    }
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
