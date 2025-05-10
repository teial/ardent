use std::sync::Arc;

use crate::frame::Frame;

use ardent_render::{GpuContext, Renderer};

use pollster::FutureExt;
use winit::{dpi::PhysicalSize, window::Window};

pub struct State<'a> {
    window: Arc<Window>,
    context: GpuContext<'a>,
    renderer: Renderer,
    frame: Frame,
}

impl State<'_> {
    pub fn new(window: Window) -> Self {
        let window = Arc::new(window);
        let context = GpuContext::new(window.clone()).block_on();
        let renderer = Renderer::new(&context);
        let size = window.inner_size();
        let frame = Frame::new(size.width, size.height);
        Self {
            window,
            context,
            renderer,
            frame,
        }
    }

    pub fn render(&mut self) {
        self.renderer.render(self.frame.scene(), &self.context);
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.context.resize(size.width, size.height);
    }

    pub fn window(&self) -> &Window {
        &self.window
    }
}
