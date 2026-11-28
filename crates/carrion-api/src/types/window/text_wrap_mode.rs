#[repr(u8)]
#[derive(abi_stable::StableAbi, Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TextWrapMode {
    Extend,
    Wrap,
    Truncate,
}

#[cfg(feature = "egui")]
impl From<TextWrapMode> for egui::TextWrapMode {
    fn from(value: TextWrapMode) -> Self {
        match value {
            TextWrapMode::Extend => egui::TextWrapMode::Extend,
            TextWrapMode::Wrap => egui::TextWrapMode::Wrap,
            TextWrapMode::Truncate => egui::TextWrapMode::Truncate,
        }
    }
}
