use crate::{event::ElementState, types::math::Vec2};

#[repr(u8)]
#[derive(abi_stable::StableAbi, Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MouseEvent {
    ScrollDelta(MouseScrollDelta),
    Button(MouseButton),
    Motion(Vec2),
}

#[repr(C)]
#[derive(abi_stable::StableAbi, Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MouseButton {
    pub state: ElementState,
    pub button: u8,
}

#[repr(u8)]
#[derive(abi_stable::StableAbi, Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MouseScrollDelta {
    LineDelta(f32, f32),
    PixelDelta(Vec2),
}
