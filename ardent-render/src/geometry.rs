use lyon::tessellation::{FillVertex, VertexBuffers};

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

pub type Geometry = VertexBuffers<Vertex, u16>;
