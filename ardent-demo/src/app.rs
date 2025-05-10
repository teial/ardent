use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

use crate::state::State;

#[derive(Default)]
pub struct Application<'a> {
    state: Option<State<'a>>,
}

impl ApplicationHandler for Application<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let atrributes = Window::default_attributes()
            .with_title("Ardent Demo")
            .with_inner_size(winit::dpi::LogicalSize::new(800, 600));
        let window = event_loop
            .create_window(atrributes)
            .expect("Failed to create a window");
        self.state = Some(State::new(window));
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        let window = self.state.as_ref().unwrap().window();
        if window.id() == id {
            match event {
                WindowEvent::CloseRequested => event_loop.exit(),
                WindowEvent::Resized(size) => self
                    .state
                    .as_mut()
                    .expect("State should exist in window events")
                    .resize(size),
                WindowEvent::RedrawRequested => self
                    .state
                    .as_mut()
                    .expect("State should exist in window events")
                    .render(),
                _ => (),
            }
        }
    }
}
