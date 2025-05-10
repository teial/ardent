#![allow(unused)]

//! High-level renderer that ties together the scene graph, tessellation, and GPU drawing.

use std::collections::HashMap;

use ardent_core::node::NodeId;
use ardent_core::scene::Scene;

use super::GpuContext;
use crate::gpu::buffers::VertexBuffer;
use crate::gpu::pipeline::RenderPipelineBuilder;
use crate::renderer::Renderer;

/// Stores a GPU vertex buffer representing a single node's geometry.
struct CachedMesh {
    vertex_buffer: VertexBuffer,
}

/// Top-level frame renderer with per-node GPU caching.
///
/// This struct handles full-scene rendering by:
/// - Tracking cached geometry for each node
/// - Tessellating and uploading GPU buffers only when dirty
/// - Reusing GPU buffers otherwise
pub struct FrameRenderer {
    renderer: Renderer,
    pipeline: wgpu::RenderPipeline,
    cache: HashMap<NodeId, CachedMesh>,
}

impl FrameRenderer {
    /// Creates a new renderer with a fresh GPU pipeline and an empty cache.
    pub fn new(context: &GpuContext) -> Self {
        let renderer = Renderer::new();
        let pipeline = RenderPipelineBuilder::new(&context.device, &context.config).pipeline;
        Self {
            renderer,
            pipeline,
            cache: HashMap::new(),
        }
    }

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
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = context
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Ardent Frame Encoder"),
            });

        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Ardent Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            pass.set_pipeline(&self.pipeline);
            self.draw_scene(scene, context, &mut pass);
        }

        context.queue.submit(Some(encoder.finish()));
        output.present();
    }

    fn draw_scene<'a>(
        &'a mut self,
        scene: &'a Scene,
        context: &GpuContext,
        pass: &mut wgpu::RenderPass<'a>,
    ) {
        let mut draw_list = Vec::new();

        // 1. Traverse the scene, update cache if needed, collect IDs
        scene.traverse(|node| {
            if let Some(shape) = node.shape() {
                let id = node.id();
                if node.is_dirty() || !self.cache.contains_key(&id) {
                    let vertices = self.renderer.tessellate_shape(shape);
                    let vertex_buffer = VertexBuffer::from_vertices(&context.device, &vertices);
                    self.cache.insert(id, CachedMesh { vertex_buffer });
                }
                draw_list.push(id);
            }
        });

        // 2. Render using the cached vertex buffers
        for id in draw_list {
            if let Some(cached) = self.cache.get(&id) {
                cached.vertex_buffer.draw(pass);
            }
        }
    }
}
