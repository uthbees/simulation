use crate::Position;
use winit::event::{ElementState, KeyEvent};
use winit::keyboard::{KeyCode, PhysicalKey};

/// Represents the UI - anything that affects what the user sees, but which is not actually part of the simulation.
pub struct Ui {
    pub camera_pos: Position,
    current_inputs: Inputs,
}

impl Ui {
    #[must_use]
    pub fn new() -> Self {
        Self {
            camera_pos: Position { x: 0.0, y: 0.0 },
            current_inputs: Inputs {
                ..Default::default()
            },
        }
    }

    pub fn handle_input(&mut self, event: &KeyEvent) {
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
                    self.current_inputs.up = key_state;
                }
                KeyCode::KeyS | KeyCode::ArrowDown => {
                    self.current_inputs.down = key_state;
                }
                KeyCode::KeyA | KeyCode::ArrowLeft => {
                    self.current_inputs.left = key_state;
                }
                KeyCode::KeyD | KeyCode::ArrowRight => {
                    self.current_inputs.right = key_state;
                }
                _ => {}
            }
        }
    }

    // Note: Will accept World in the future (if any interaction with the simulation is added).
    pub fn tick(&mut self) {
        if self.current_inputs.up == KeyState::Pressed {
            self.camera_pos.y += CAMERA_SPEED_PX;
        }
        if self.current_inputs.down == KeyState::Pressed {
            self.camera_pos.y -= CAMERA_SPEED_PX;
        }
        if self.current_inputs.left == KeyState::Pressed {
            self.camera_pos.x -= CAMERA_SPEED_PX;
        }
        if self.current_inputs.right == KeyState::Pressed {
            self.camera_pos.x += CAMERA_SPEED_PX;
        }
    }
}

/// The single-axis speed of the camera when moving, in pixels per tick.
const CAMERA_SPEED_PX: f64 = 10.0;

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
