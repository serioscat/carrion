use abi_stable::StableAbi;

#[repr(u8)]
#[derive(StableAbi, Clone, Copy, Debug, Eq, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Align {
    #[default]
    Min,
    Center,
    Max,
}

#[cfg(feature = "egui")]
impl From<Align> for egui::Align {
    fn from(value: Align) -> Self {
        match value {
            Align::Center => egui::Align::Center,
            Align::Max => egui::Align::Max,
            Align::Min => egui::Align::Min,
        }
    }
}
