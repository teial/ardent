pub mod node;
pub mod scene;
pub mod shape;
pub mod style;
pub mod transform;

pub mod prelude {
    pub use crate::node::Node;
    pub use crate::scene::Scene;
    pub use crate::shape::Shape;
    pub use crate::style::*;
    pub use crate::transform::Transform;
}

pub struct EventHandler;
pub struct Transform;
