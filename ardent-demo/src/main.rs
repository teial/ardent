use app::Application;
use winit::event_loop::{ControlFlow, EventLoop};

mod app;
mod frame;
mod state;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = Application::default();
    if let Err(e) = event_loop.run_app(&mut app) {
        eprintln!("{e}");
        std::process::exit(-1);
    }
}
