use abi_stable::StableAbi;
use serde::{Deserialize, Serialize};

#[repr(u8)]
#[derive(StableAbi, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyboardEvent {
    Grabbed,
    Ungrabbed
}

#[repr(u8)]
#[derive(StableAbi, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SystemEvent {
    PluginsLoaded,
    ServicesRegistered,
    Shutdown,
    Keyboard(KeyboardEvent),
}