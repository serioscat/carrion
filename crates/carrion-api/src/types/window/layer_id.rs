use std::num::NonZeroU64;

#[repr(u8)]
#[derive(abi_stable::StableAbi, Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Order {
    Background,
    Middle,
    Foreground,
    Tooltip,
    Debug
}

#[repr(C)]
#[derive(abi_stable::StableAbi, Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LayerId {
    pub order: Order,
    pub id: NonZeroU64
}