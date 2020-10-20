mod interop;
mod window_target;

use interop::{create_dispatcher_queue_controller_for_current_thread, ro_initialize, RoInitType};
use window_target::CompositionDesktopWindowTargetSource;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use bindings::windows::{foundation::numerics::Vector2, ui::composition::Compositor};

fn run() -> winrt::Result<()> {

    ro_initialize(RoInitType::MultiThreaded)?;
    let _controller = create_dispatcher_queue_controller_for_current_thread()?;

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    window.set_title("Rust/WinRT Sample Window");

    let compositor = Compositor::new()?;
    let target = window.create_window_target(&compositor, false)?;

    let root = compositor.create_container_visual()?;
    root.set_relative_size_adjustment(Vector2 { x: 1.0, y: 1.0 })?;
    target.set_root(&root)?;

    let window_size = window.inner_size();
    let _window_size = Vector2 {
        x: window_size.width as f32,
        y: window_size.height as f32,
    };


    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            
            _ => (),
        }
    });
}

fn main() {
    let result = run();
    if let Err(error) = result {
        error.code().unwrap();
    }
}
