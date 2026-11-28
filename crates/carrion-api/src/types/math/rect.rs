use crate::types::math::Pos2;

#[repr(C)]
#[derive(abi_stable::StableAbi, Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rect {
    pub min: Pos2,
    pub max: Pos2,
}

#[cfg(feature = "egui")]
impl From<egui::Rect> for Rect {
    fn from(value: egui::Rect) -> Self {
        Rect {
            min: Pos2::from(value.min),
            max: Pos2::from(value.max),
        }
    }
}
