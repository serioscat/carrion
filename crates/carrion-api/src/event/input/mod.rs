pub mod keyboard;
pub mod mouse;

pub use mouse::*;
pub use keyboard::*;

use abi_stable::StableAbi;
use serde::{Serialize, Deserialize};

#[repr(u8)]
#[derive(StableAbi, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ElementState {
    Pressed,
    Released,
}

#[repr(u8)]
#[derive(StableAbi, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum InputEvent {
    Keyboard(KeyboardInputEvent),
    Mouse(MouseEvent)
}