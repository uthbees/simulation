#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::unwrap_used)]
#![allow(clippy::missing_panics_doc)]

mod render;
mod ui;
mod world;

use crate::render::Renderer;
pub use crate::ui::Ui;
pub use crate::world::World;
use cfg_if::cfg_if;
#[cfg(target_arch = "wasm32")]
// TODO: Upgrade winit to the latest version.
use wasm_bindgen::prelude::*;
use winit::window::Window;
use winit::{
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    init_logging();

    let event_loop = EventLoop::new().expect("Failed to initialize main event loop");
    let window = init_window(&event_loop);

    let renderer = Renderer::new(&window).await;
    let mut ui = Ui::new();
    let mut world = World::new();

    // TODO: Update the event loop to handle inputs and window resizing and to call renderer.render().
    // TODO: Use EventLoopExtWebSys::spawn() instead of run on web to avoid the JS exception trick.
    event_loop
        .run(|event, control_flow| match event {
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                match renderer.render(&ui, &world) {
                    Ok(_) => {}
                    // Reconfigure the surface if lost.
                    Err(wgpu::SurfaceError::Lost) => {} // TODO: Call surface.configure somehow.
                    // If the system is out of memory, we should probably quit.
                    Err(wgpu::SurfaceError::OutOfMemory) => control_flow.exit(),
                    // The other errors (Outdated, Timeout) should be resolved by the next frame.
                    Err(error) => eprintln!("{:?}", error),
                }
            }
            Event::WindowEvent {
                event:
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        physical_key: PhysicalKey::Code(KeyCode::Escape),
                        ..
                    },
                    ..
                },
                ..
            } => control_flow.exit(),
            Event::AboutToWait => {
                // Temporary until we get the main loop running.
                window.request_redraw();
            }
            _ => {}
        })
        .expect("Main event loop failed");

    // TODO: Implement the simulation loop (calling world.tick and window.request_redraw).
    //  Ideally, this should be done in such a way that we can maintain 60 UPS even if we dip below 60 FPS.
    //  Make sure to remove the request_redraw call from the event loop.
}

fn init_logging() {
    cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            // Set up logging for the web. We have to do this specially since env_logger doesn't support wasm.
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            // Even if we're running normally, we still have to do special setup for logging. When
            // wgpu hits any error, it panics with a generic message, while logging the real error
            // via the log crate. We initialize env_logger here to make sure we always know what's
            // going on.
            env_logger::init();
        }
    }
}

fn init_window(event_loop: &EventLoop<()>) -> Window {
    let window = WindowBuilder::new()
        .build(event_loop)
        .expect("Failed to create window");

    #[cfg(target_arch = "wasm32")]
    {
        use winit::dpi::LogicalSize;
        use winit::platform::web::WindowExtWebSys;

        web_sys::window()
            .and_then(|js_window| js_window.document())
            .and_then(|document| {
                let destination = document.get_element_by_id("canvas-holder")?;
                let canvas = web_sys::Element::from(window.canvas()?);
                destination.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document");

        // winit doesn't allow sizing with CSS, so we have to set the size manually when on web.
        // Note that this sets the size of the canvas on web, not the window itself.
        let _ = window.request_inner_size(LogicalSize::new(1000, 500));
    }

    window
}
