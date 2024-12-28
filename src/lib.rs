mod display;
mod ui;
mod world;

use crate::display::Display;
use crate::ui::Ui;
use crate::world::World;
use cfg_if::cfg_if;
use std::ops::Add;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use web_time::{Duration, Instant};
use winit::event::{Event, StartCause, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget};

/// Runs the application. Called by WASM directly.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    init_logging();

    let event_loop = EventLoop::new().expect("Failed to initialize main event loop");
    let window = display::create_window(&event_loop);

    let mut app_state = App {
        display: Display::new(&window).await,
        next_frame_start_time: Instant::now(),
        #[expect(clippy::needless_update)]
        ui: Ui {
            ..Default::default()
        },
        world: World {
            ..Default::default()
        },
    };

    // TODO: Use EventLoopExtWebSys::spawn() instead of run() on web to avoid the JS exception trick.
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop
        .run(move |event, control_flow| handle_winit_event(&event, control_flow, &mut app_state))
        .expect("Main event loop failed");
}

struct App<'a> {
    display: Display<'a>,
    next_frame_start_time: Instant,
    #[expect(dead_code)]
    ui: Ui,
    world: World,
}

fn handle_winit_event(
    event: &Event<()>,
    control_flow: &EventLoopWindowTarget<()>,
    app_state: &mut App,
) {
    let frame_length = Duration::from_secs_f32(FRAME_LENGTH);
    let App {
        ref mut display,
        ref mut next_frame_start_time,
        ref mut world,
        ..
    } = app_state;

    match *event {
        Event::WindowEvent {
            event: WindowEvent::RedrawRequested,
            ..
        } => {
            match display.render(world) {
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
        } => {
            control_flow.exit();
            // It might take up to a few seconds to clean up, so we'll hide the window now. This makes
            // it appear as though everything quits instantly, even though the process might hang around
            // for a few more seconds.
            display.window().set_visible(false);
        }
        Event::NewEvents(cause)
            if cause == StartCause::Poll && Instant::now() >= *next_frame_start_time =>
        {
            world.tick();

            display.window().request_redraw();

            // Increment the frame counter.
            *next_frame_start_time += frame_length;

            // Drop any missed frames.
            let mut proposed_next_frame_time = next_frame_start_time.add(frame_length);
            while proposed_next_frame_time < Instant::now() {
                eprintln!(
                    "Dropped a frame. (Missed it by {:?}.)",
                    proposed_next_frame_time.elapsed()
                );
                *next_frame_start_time = proposed_next_frame_time;
                proposed_next_frame_time = next_frame_start_time.add(frame_length);
            }
        }
        _ => {}
    }
}

const FRAME_LENGTH: f32 = 1.0 / 60.0;

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
