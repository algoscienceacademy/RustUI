use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("RustUI Basic App")
        .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0))
        .build(&event_loop)
        .unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
