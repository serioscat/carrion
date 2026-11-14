use abi_stable::StableAbi;
use serde::{Deserialize, Serialize};


#[repr(C)]
#[derive(StableAbi, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Theme {
    pub bg: [u8; 4],
    pub bg_hover: [u8; 4],
    pub accent: [u8; 3],
    pub border: [u8; 3],
    pub text: [u8; 3],
    pub radius: f32,
    pub font_size: f32,
}