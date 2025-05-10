#![allow(unused)]

//! Initializes the GPU backend using `wgpu` and prepares a surface for rendering.
//!
//! This module sets up the WGPU instance, device, queue, and swapchain surface.
//! It forms the foundation for all GPU rendering in `ardent`.

use std::sync::Arc;

use wgpu::{
    Backends, Device, DeviceDescriptor, Instance, InstanceDescriptor, Queue, Surface,
    SurfaceConfiguration,
};
use winit::window::Window;

/// Holds the essential GPU components needed for rendering.
pub struct GpuContext<'a> {
    /// The GPU device, used to create buffers, shaders, and pipelines.
    pub device: Device,

    /// The queue used to submit rendering commands to the GPU.
    pub queue: Queue,

    /// The surface (usually a window) that we render into.
    pub surface: Surface<'a>,

    /// The surface configuration (format, usage, present mode, etc.)
    pub config: SurfaceConfiguration,

    /// The size of the surface (width, height in pixels).
    pub size: (u32, u32),
}

impl GpuContext<'_> {
    /// Creates a new GPU context bound to the given window.
    ///
    /// This initializes the GPU instance, chooses an adapter and device,
    /// creates a swapchain surface, and configures it for rendering.
    pub async fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();

        // 1. Create instace.
        let instance = Instance::new(&InstanceDescriptor {
            backends: Backends::all(),
            ..Default::default()
        });

        // 2. Create surface for the instance.
        let surface = instance
            .create_surface(window)
            .expect("Failed to create surface");

        // 3. Request the adapter.
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("Failed to find GPU adapter");

        // 4. Request device and queue.
        let (device, queue) = adapter
            .request_device(&DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                ..Default::default()
            })
            .await
            .expect("Failed to create device");

        // 5. Configure the surface.
        let config = surface
            .get_default_config(&adapter, size.width, size.height)
            .expect("Failed to configure surface");
        surface.configure(&device, &config);

        Self {
            device,
            queue,
            surface,
            config,
            size: (size.width, size.height),
        }
    }

    /// Resizes the surface when the window size changes.
    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.size = (width, height);
            self.surface.configure(&self.device, &self.config);
        }
    }
}
