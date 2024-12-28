/// Represents the UI - anything that affects what the user sees, but which is not actually part of the simulation.
pub struct Ui {}

impl Default for Ui {
    fn default() -> Self {
        Self::new()
    }
}

impl Ui {
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }

    // pub fn handle_input(&mut self, event: &WindowEvent) {}
}
