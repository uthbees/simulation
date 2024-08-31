use crate::{Ui, World};
use winit::window::Window;

/// An object that handles rendering the application state onto the screen.
pub struct Renderer {
    window: Window,
}

impl Renderer {
    pub fn new(window: Window) -> Renderer {
        // TODO: Set up the instance, surface, adapter, etc here.
        Renderer { window }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn render(&self, ui: &Ui, world: &World) {}
}
