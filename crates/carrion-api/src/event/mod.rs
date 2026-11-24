pub mod input;
pub mod system;
pub mod window;

pub use input::*;
pub use system::*;
pub use window::*;

#[repr(C)]
#[derive(abi_stable::StableAbi, Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PhysicalSize {
    pub w: u32,
    pub h: u32
}

#[repr(C)]
#[derive(abi_stable::StableAbi, Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PhysicalPosition {
    pub x: i32,
    pub y: i32
}

#[repr(u8)]
#[derive(abi_stable::StableAbi, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Event {
    Window(WindowEvent),
    Input(InputEvent),
    Visible(bool),
    System(SystemEvent)
}
