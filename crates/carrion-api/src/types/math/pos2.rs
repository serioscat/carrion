use crate::types::math::Vec2;

#[repr(C)]
#[derive(abi_stable::StableAbi, Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pos2 {
    pub x: f32,
    pub y: f32,
}

impl Pos2 {
    #[inline(always)]
    pub fn to_vec2(self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.y,
        }
    }
}

#[inline(always)]
pub const fn pos2(x: f32, y: f32) -> Pos2 {
    Pos2 { x, y }
}

#[cfg(feature = "egui")]
impl From<egui::Pos2> for Pos2 {
    fn from(value: egui::Pos2) -> Self {
        pos2(value.x, value.y)
    }
}
