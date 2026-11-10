use abi_stable::StableAbi;
use serde::{Deserialize, Serialize};


#[repr(C)]
#[derive(StableAbi, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}