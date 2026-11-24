#[repr(C)]
#[derive(abi_stable::StableAbi, Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WindowTheme {
    pub bg: [u8; 4],
    pub bg_hover: [u8; 4],
    pub accent: [u8; 3],
    pub border: [u8; 3],
    pub text: [u8; 3],
    pub radius: f32,
    pub font_size: f32,
}

impl Default for WindowTheme {
    fn default() -> Self {
        Self {
            bg: [10, 10, 10, 255],
            bg_hover: [15, 15, 15, 255],
            accent: [20, 20, 20],
            border: [0, 0, 0],
            text: [255, 255, 255],
            radius: 5.,
            font_size: 14.
        }
    }
}