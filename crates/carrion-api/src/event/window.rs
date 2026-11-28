use crate::event::{PhysicalPosition, PhysicalSize};
use abi_stable::std_types::RString;

#[repr(u8)]
#[derive(abi_stable::StableAbi, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum WindowEvent {
    CloseRequested(RString, RString),
    Destroyed(RString, RString),
    RedrawRequested(RString, RString),
    ScaleFactorChanged(f32),
    Resized {
        plugin: RString,
        window: RString,
        size: PhysicalSize,
    },
    Moved {
        plugin: RString,
        window: RString,
        size: PhysicalPosition,
    },
    Focused {
        plugin: RString,
        window: RString,
        value: bool,
    },
    Occluded {
        plugin: RString,
        window: RString,
        value: bool,
    },
}
