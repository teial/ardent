use std::collections::HashMap;

use ardent_core::node::NodeId;
use ardent_core::scene::Scene;
use ardent_core::shape::Shape;
use lyon::tessellation::VertexBuffers;

use crate::geometry::Vertex;
use crate::gpu::GpuContext;
use crate::gpu::RenderPipelineBuilder;
use crate::gpu::VertexBuffer;
use crate::tesselate::Tesselate;

use lyon::tessellation::FillTessellator;

/// Stores a GPU vertex buffer representing a single node's geometry.
struct CachedMesh {
    vertex_buffer: VertexBuffer,
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
    pipeline: wgpu::RenderPipeline,
    cache: HashMap<NodeId, CachedMesh>,
}

impl Renderer {
    /// Initializes the renderer and internal GPU pipeline.
    pub fn new(context: &GpuContext) -> Self {
        let tessellator = FillTessellator::new();
        let pipeline = RenderPipelineBuilder::new(&context.device, &context.config).pipeline;

        Self {
            tessellator,
            pipeline,
            cache: HashMap::new(),
        }
    }

    /// Renders the given scene graph into the provided surface.
    ///
    /// Performs dirty checking, GPU upload, and draw call submission.
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

    /// Internal helper: draws all renderable nodes in the scene.
    fn draw_scene<'a>(
        &'a mut self,
        scene: &'a Scene,
        context: &GpuContext,
        pass: &mut wgpu::RenderPass<'a>,
    ) {
        let mut draw_list = Vec::new();

        // Traverse scene graph and prepare dirty meshes
        scene.traverse(|node| {
            if let Some(shape) = node.shape() {
                let id = node.id();

                if node.is_dirty() || !self.cache.contains_key(&id) {
                    let vertices = self.tessellate_shape(shape);
                    let vertex_buffer = VertexBuffer::from_vertices(&context.device, &vertices);
                    self.cache.insert(id, CachedMesh { vertex_buffer });
                }

                draw_list.push(id);
            }
        });

        // Perform draw calls from prepared list
        for id in draw_list {
            if let Some(cached) = self.cache.get(&id) {
                cached.vertex_buffer.draw(pass);
            }
        }
    }

    /// Tessellates a single shape using the internal lyon tessellator.
    fn tessellate_shape(&mut self, shape: &Shape) -> Vec<Vertex> {
        let mut geometry: VertexBuffers<Vertex, u16> = VertexBuffers::new();
        match shape {
            Shape::Rect(rect) => {
                rect.tesselate(&mut geometry, &mut self.tessellator);
            } // Future: other shape variants
        }
        geometry.vertices
    }
}
