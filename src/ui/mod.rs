mod camera;

use crate::position::IntChunkCoordinates;
use crate::ui::camera::Camera;
use crate::world::World;
use camera::{MoveDirection, PrimaryDirection};
use std::collections::HashSet;
use winit::event::{ElementState, KeyEvent, MouseScrollDelta};
use winit::keyboard::{KeyCode, PhysicalKey};

/// Represents the UI - anything that affects what the user sees, but which is not actually part of the simulation.
pub struct Ui {
    pub camera: Camera,
    held_inputs: Inputs,
}

impl Ui {
    #[must_use]
    pub fn new() -> Self {
        Self {
            camera: Camera::new(),
            held_inputs: Inputs {
                ..Default::default()
            },
        }
    }

    pub fn handle_key_event(&mut self, event: &KeyEvent) {
        // Ignore key repeats.
        if event.repeat {
            return;
        }

        if let PhysicalKey::Code(key_code) = event.physical_key {
            let key_state = if event.state == ElementState::Pressed {
                KeyState::Pressed
            } else {
                KeyState::Released
            };
            match key_code {
                KeyCode::KeyW | KeyCode::ArrowUp => {
                    self.held_inputs.up = key_state;
                }
                KeyCode::KeyS | KeyCode::ArrowDown => {
                    self.held_inputs.down = key_state;
                }
                KeyCode::KeyA | KeyCode::ArrowLeft => {
                    self.held_inputs.left = key_state;
                }
                KeyCode::KeyD | KeyCode::ArrowRight => {
                    self.held_inputs.right = key_state;
                }
                _ => {}
            }
        }
    }

    pub fn handle_scroll_event(&mut self, delta: &MouseScrollDelta) {
        if let MouseScrollDelta::LineDelta(_, lines) = delta {
            self.camera.zoom(*lines);
        }
    }

    pub fn tick(&mut self, world: &mut World) {
        self.move_camera();
        self.gen_chunks_around_camera(world);
    }

    fn move_camera(&mut self) {
        let mut move_directions = HashSet::new();

        if self.held_inputs.up == KeyState::Pressed {
            move_directions.insert(PrimaryDirection::North);
        }
        if self.held_inputs.down == KeyState::Pressed {
            move_directions.insert(PrimaryDirection::South);
        }
        if self.held_inputs.left == KeyState::Pressed {
            move_directions.insert(PrimaryDirection::West);
        }
        if self.held_inputs.right == KeyState::Pressed {
            move_directions.insert(PrimaryDirection::East);
        }

        self.camera
            .pan(&MoveDirection::from_primary_directions(move_directions));
    }

    /// Generate chunks in a radius around the camera.
    fn gen_chunks_around_camera(&self, world: &mut World) {
        const CHUNK_GENERATION_RADIUS: i32 = 10;

        let center_chunk_pos = self.camera.pos.into_int_chunk_coords();

        for x_chunk_pos in center_chunk_pos.x - CHUNK_GENERATION_RADIUS
            ..=center_chunk_pos.x + CHUNK_GENERATION_RADIUS
        {
            for y_chunk_pos in center_chunk_pos.y - CHUNK_GENERATION_RADIUS
                ..=center_chunk_pos.y + CHUNK_GENERATION_RADIUS
            {
                // TODO: Optimize - Only generate chunks when the camera moves, and skip chunks that
                //  would have been in the radius of the old camera position.
                world.generate_chunk(IntChunkCoordinates {
                    x: x_chunk_pos,
                    y: y_chunk_pos,
                });
            }
        }
    }
}

#[derive(Default)]
struct Inputs {
    up: KeyState,
    down: KeyState,
    left: KeyState,
    right: KeyState,
}

#[derive(Debug, PartialEq, Eq)]
enum KeyState {
    Pressed,
    Released,
}

impl Default for KeyState {
    fn default() -> Self {
        Self::Released
    }
}
