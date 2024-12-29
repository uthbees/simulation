use crate::Position;
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
            camera: Camera {
                ..Default::default()
            },
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
        match delta {
            MouseScrollDelta::LineDelta(_, lines) => self.camera.zoom(*lines),
            _ => {}
        }
    }

    // Note: Will accept World in the future (if any interaction with the simulation is added).
    pub fn tick(&mut self) {
        let mut camera_move_direction: Option<Direction> = None;

        if self.held_inputs.up == KeyState::Pressed {
            camera_move_direction = Some(Direction::North);
        } else if self.held_inputs.down == KeyState::Pressed {
            camera_move_direction = Some(Direction::South);
        } else if self.held_inputs.left == KeyState::Pressed {
            camera_move_direction = Some(Direction::West);
        } else if self.held_inputs.right == KeyState::Pressed {
            camera_move_direction = Some(Direction::East);
        }

        if let Some(direction) = camera_move_direction {
            self.camera.pan(&direction);
        }
    }
}

#[derive(Default)]
pub struct Camera {
    pub pos: Position,
    /// Note that the zoom level can be a decimal value, but only the value before the decimal place
    /// is used when determining zoom multiplier.
    zoom_level: f32,
}

impl Camera {
    fn pan(&mut self, direction: &Direction) {
        let speed = CAMERA_SPEED_PX / f64::from(self.zoom_multiplier());

        match direction {
            Direction::North => {
                self.pos.y += speed;
            }
            Direction::East => {
                self.pos.x += speed;
            }
            Direction::South => {
                self.pos.y -= speed;
            }
            Direction::West => {
                self.pos.x -= speed;
            }
        }
    }

    /// The amount that the camera is currently zoomed by. Positive values denote zooming in and vice versa.
    ///
    /// For example, a value of 2 would indicate that everything looks twice as big as normal.
    pub fn zoom_multiplier(&self) -> f32 {
        // Round the zoom level so that zooming happens in increments instead of continuously.
        let clipped_zoom_level = self.zoom_level as i32;
        #[expect(clippy::cast_precision_loss)]
        CAMERA_ZOOM_LEVEL_MULTIPLIER.powf(clipped_zoom_level as f32)
    }

    fn zoom(&mut self, lines: f32) {
        self.zoom_level += lines;
        self.zoom_level = self
            .zoom_level
            .clamp(CAMERA_MIN_ZOOM_LEVEL, CAMERA_MAX_ZOOM_LEVEL);
    }
}

enum Direction {
    North,
    East,
    South,
    West,
}

/// The single-axis speed of the camera when moving, in pixels of the window per tick.
const CAMERA_SPEED_PX: f64 = 10.0;
/// The amount each zoom increment zooms in/out.
const CAMERA_ZOOM_LEVEL_MULTIPLIER: f32 = 1.15;
const CAMERA_MAX_ZOOM_LEVEL: f32 = 25.0;
const CAMERA_MIN_ZOOM_LEVEL: f32 = -25.0;

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
