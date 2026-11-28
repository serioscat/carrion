pub mod keyboard;
pub mod mouse;

pub use keyboard::*;
pub use mouse::*;

#[repr(u8)]
#[derive(abi_stable::StableAbi, Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ElementState {
    Pressed,
    Released,
}

#[repr(u8)]
#[derive(abi_stable::StableAbi, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InputEvent {
    Keyboard(KeyboardInputEvent),
    Mouse(MouseEvent),
}
