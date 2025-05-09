use super::Tesselate;

use ardent_core::shape::Rect;

use lyon::path::Path;

impl Tesselate for Rect {
    fn path(&self) -> Path {
        let mut path_builder = Path::builder();
        path_builder.begin(lyon::math::point(0.0, 0.0));
        path_builder.line_to(lyon::math::point(self.width, 0.0));
        path_builder.line_to(lyon::math::point(self.width, self.height));
        path_builder.line_to(lyon::math::point(0.0, self.height));
        path_builder.close();
        path_builder.build()
    }
}
