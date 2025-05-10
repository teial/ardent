use crate::geometry::{Geometry, Vertex};

use lyon::path::Path;
use lyon::tessellation::{BuffersBuilder, FillOptions, FillTessellator, FillVertex};

mod rect;

pub trait Tesselate {
    fn path(&self) -> Path;

    fn tesselate(&self, geometry: &mut Geometry, tessellator: &mut FillTessellator) {
        let _ = tessellator.tessellate_path(
            &self.path(),
            &FillOptions::default(),
            &mut BuffersBuilder::new(geometry, |v: FillVertex| Vertex::from_fill_vertex(v)),
        );
    }
}
