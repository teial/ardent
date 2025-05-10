use ardent_core::prelude::*;

pub struct Frame {
    scene: Scene,
}

impl Frame {
    pub fn new(width: u32, height: u32) -> Self {
        // Create scene.
        let mut scene = Scene::new();
        let root = scene.root();

        // Define a rectangle node.
        let mut rect_node = Node::new();
        rect_node.set_shape(Shape::Rect(Rect {
            width: 200.0,
            height: 100.0,
        }));

        // Set transform.
        rect_node.transform_mut().translate =
            ((width as f32 - 200.0) / 2.0, (height as f32 - 100.0) / 2.0);

        // Set style.
        rect_node.style_mut().fill = Some(Fill {
            color: Color::rgb(0.2, 0.5, 0.8),
            gradient: None,
        });

        // Add rect node to scene.
        scene.add_node(root, rect_node);
        Self { scene }
    }

    pub fn scene(&self) -> &Scene {
        &self.scene
    }
}
