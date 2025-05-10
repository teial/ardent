#![allow(unused)]

//! High-level renderer that ties together the scene graph, tessellation, and GPU drawing.

use ardent_core::scene::Scene;

use super::GpuContext;
use crate::gpu::buffers::VertexBuffer;
use crate::gpu::pipeline::RenderPipelineBuilder;
use crate::renderer::Renderer;
use wgpu::{
    CommandEncoderDescriptor, LoadOp, Operations, RenderPassColorAttachment, RenderPassDescriptor,
    StoreOp, TextureViewDescriptor,
};

/// A reusable renderer that renders a scene into a WGPU surface frame.
///
/// This struct encapsulates the GPU drawing loop: it tessellates the scene,
/// uploads geometry, and issues draw calls into a window.
pub struct FrameRenderer {
    renderer: Renderer,
    pipeline: wgpu::RenderPipeline,
}

impl FrameRenderer {
    /// Initializes the rendering pipeline and internal tessellator.
    pub fn new(context: &GpuContext) -> Self {
        let renderer = Renderer::new();
        let pipeline = RenderPipelineBuilder::new(&context.device, &context.config).pipeline;
        Self { renderer, pipeline }
    }

    /// Renders the current scene graph to the surface frame.
    ///
    /// - Traverses the scene graph to collect shapes.
    /// - Tessellates them into triangles.
    /// - Uploads them to a vertex buffer.
    /// - Draws them using the GPU render pipeline.
    pub fn render(&mut self, scene: &Scene, context: &GpuContext) {
        let output = match context.surface.get_current_texture() {
            Ok(frame) => frame,
            Err(e) => {
                eprintln!("Failed to acquire surface frame: {:?}", e);
                return;
            }
        };

        let view = output
            .texture
            .create_view(&TextureViewDescriptor::default());

        // Tessellate the scene to CPU-side geometry
        let vertices = self.renderer.tessellate_scene(scene);
        let vertex_buffer = VertexBuffer::from_vertices(&context.device, &vertices);

        // Begin encoding commands
        let mut encoder = context
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Ardent Frame Encoder"),
            });

        {
            let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Ardent Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(wgpu::Color {
                            r: 1.0,
                            g: 1.0,
                            b: 1.0,
                            a: 1.0,
                        }),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            pass.set_pipeline(&self.pipeline);
            vertex_buffer.draw(&mut pass);
        }

        // Submit and present
        context.queue.submit(Some(encoder.finish()));
        output.present();
    }
}
