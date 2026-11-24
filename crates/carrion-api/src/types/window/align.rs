use abi_stable::StableAbi;

#[repr(u8)]
#[derive(StableAbi, Clone, Copy, Debug, Eq, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Align {
    #[default]
    Min,
    Center,
    Max
}