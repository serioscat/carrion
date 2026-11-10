pub mod input;
pub mod system;
pub mod window;

pub use input::*;
pub use system::*;
pub use window::*;

use abi_stable::StableAbi;
use serde::{Serialize, Deserialize};

pub type WindowId = usize;

#[repr(C)]
#[derive(StableAbi, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct PhysicalSize {
    pub w: u32,
    pub h: u32
}

#[repr(C)]
#[derive(StableAbi, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct PhysicalPosition {
    pub x: i32,
    pub y: i32
}

#[repr(u8)]
#[derive(StableAbi, Clone, Debug, Serialize, Deserialize)]
pub enum Event {
    Window(WindowEvent),
    Input(InputEvent),
    Visible(bool),
    System(SystemEvent)
}
