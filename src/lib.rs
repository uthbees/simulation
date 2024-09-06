#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::unwrap_used)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_errors_doc)]

use std::ops::{Add, AddAssign};
use std::time::{Duration, Instant};

use cfg_if::cfg_if;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use winit::event::{Event, StartCause, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

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

    let mut next_frame_start_time = Instant::now();
    let frame_length_as_d = Duration::from_secs_f32(FRAME_LENGTH);

    // TODO: Use EventLoopExtWebSys::spawn() instead of run() on web to avoid the JS exception trick.
    // TODO: Fix the panic on web (low priority since the current functionality still works).
    event_loop.set_control_flow(ControlFlow::Poll);
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
            Event::NewEvents(cause)
                if cause == StartCause::Poll && Instant::now() >= next_frame_start_time =>
            {
                temp_update();

                display.window().request_redraw();

                // Increment the frame counter.
                next_frame_start_time += frame_length_as_d;

                // Drop any missed frames.
                let mut proposed_next_frame_time = next_frame_start_time.add(frame_length_as_d);
                while proposed_next_frame_time < Instant::now() {
                    eprintln!("Dropped a frame.");
                    next_frame_start_time = proposed_next_frame_time;
                    proposed_next_frame_time = next_frame_start_time.add(frame_length_as_d);
                }
            }
            _ => {}
        })
        .expect("Main event loop failed");
}

const FRAME_LENGTH: f32 = 1.0 / 60.0;

/// Pretend we're doing work to process the next tick. A placeholder for when we actually do need to do that work later.
fn temp_update() {
    println!("Updating...");
    let start_time = Instant::now();
    let target_time = start_time.add(Duration::new(0, 10 * 1_000_000));
    while Instant::now() < target_time {}
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
