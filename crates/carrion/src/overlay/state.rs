use winit::window::Window;

use crate::overlay::keyboard_grab::KeyboardGrab;


pub struct OverlayState {
    pub visible: bool,
    pub pending_grab: bool,
    pub kb_grab: Option<KeyboardGrab>
}

impl OverlayState {
    pub fn set_visible(&mut self, window: &Window, visible: bool) {
        self.visible = visible;
        window.set_visible(visible);
    }
}

impl Default for OverlayState {
    fn default() -> Self {
        Self {
            visible: false,
            pending_grab: false,
            kb_grab: None
        }
    }
}