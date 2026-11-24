#[repr(u8)]
#[derive(abi_stable::StableAbi, Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum KeyboardEvent {
    Grabbed,
    Ungrabbed
}

#[repr(u8)]
#[derive(abi_stable::StableAbi, Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SystemEvent {
    PluginsLoaded,
    ServicesRegistered,
    Shutdown,
    Keyboard(KeyboardEvent),
}