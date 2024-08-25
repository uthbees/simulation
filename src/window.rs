use winit::{
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};

pub fn run() {
    // When wgpu hits any error, it panics with a generic message, while logging the real error via
    // the log crate. Initializing the logger here allows us to see all the relevant information if
    // something goes wrong.
    env_logger::init();

    let event_loop = EventLoop::new().expect("Failed to initialize main event loop.");
    let window = WindowBuilder::new()
        .build(&event_loop)
        .expect("Failed to create window.");

    // TODO: Fix this properly once I learn more about Rust (match, at least).
    #[allow(clippy::collapsible_match)]
    event_loop
        // TODO: Review this stuff once I learn more about Rust (callbacks, at least).
        .run(move |event, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            state: ElementState::Pressed,
                            physical_key: PhysicalKey::Code(KeyCode::Escape),
                            ..
                        },
                    ..
                } => control_flow.exit(),
                _ => {}
            },
            _ => {}
        })
        .expect("Main event loop failed.");
}
