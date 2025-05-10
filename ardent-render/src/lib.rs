//! The `ardent_render` crate is responsible for rendering UI geometry
//! to the GPU using tessellation and `wgpu`.
//!
//! It processes the scene graph (defined in `ardent_core`) by converting
//! shapes into triangles using the `lyon` tessellation engine. These
//! triangles are then passed to the GPU for drawing.
//!
//! At its core, `ardent_render` acts as the visual backend of the system.

pub mod geometry;
pub mod gpu;
pub mod renderer;
pub mod tesselate;
