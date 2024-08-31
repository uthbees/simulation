use winit::event::WindowEvent;

/// Represents the UI - anything that affects what the user sees, but which is not actually part of the simulation.
pub struct Ui {}

impl Ui {
    pub fn new() -> Ui {
        Ui {}
    }

    pub fn handle_input(&mut self, event: &WindowEvent) {}
}
