use abi_stable::StableAbi;
use serde::{Deserialize, Serialize};

use crate::{event::ElementState, types::Vec2};



#[repr(u8)]
#[derive(StableAbi, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum MouseEvent {
    ScrollDelta(MouseScrollDelta),
    Button(MouseButton),
    Motion(Vec2)
}

#[repr(C)]
#[derive(StableAbi, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct MouseButton {
    pub state: ElementState,
    pub button: u8
}

#[repr(u8)]
#[derive(StableAbi, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum MouseScrollDelta {
    LineDelta(f32, f32),
    PixelDelta(Vec2),
}