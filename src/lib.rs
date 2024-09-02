#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::unwrap_used)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_errors_doc)]

use cfg_if::cfg_if;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;

use crate::display::{create_window, Display};
pub use crate::ui::Ui;
pub use crate::world::World;

pub mod display;
pub mod ui;
pub mod world;

/// Runs the application. Called by WASM directly.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    init_logging();

    let event_loop = EventLoop::new().expect("Failed to initialize main event loop");
    let window = create_window(&event_loop);

    let mut display = Display::new(&window).await;

    /*
    Desired event loop:
    loop {
    update
    render
    wait for next frame (use set_control_flow(WaitUntil) for now, but make sure to log any dropped frames)
    }
    */

    // TODO: Use EventLoopExtWebSys::spawn() instead of run() on web to avoid the JS exception trick.
    // TODO: Fix the panic on web (low priority since the current functionality still works).
    event_loop
        .run(move |event, control_flow| match event {
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                match display.render() {
                    Ok(()) => {}
                    // Reconfigure the surface if lost.
                    Err(wgpu::SurfaceError::Lost) => display.configure_surface(),
                    // If the system is out of memory, we should probably quit.
                    Err(wgpu::SurfaceError::OutOfMemory) => control_flow.exit(),
                    // The other errors (Outdated, Timeout) should be resolved by the next frame.
                    Err(error) => eprintln!("{error:?}"),
                }
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(physical_size),
                ..
            } => display.resize(physical_size),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => control_flow.exit(),
            Event::AboutToWait => {
                // Temporary until we get the main loop running.
                display.window().request_redraw();
            }
            _ => {}
        })
        .expect("Main event loop failed");

    // TODO: Implement the simulation loop (calling world.tick and window.request_redraw).
    //  Ideally, this should be done in such a way that we can maintain 60 UPS even if we dip below 60 FPS.
    //  Make sure to remove the request_redraw call from the event loop.
}

/// Sets up logging for whatever platform we're running on.
pub fn init_logging() {
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
