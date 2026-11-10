use abi_stable::StableAbi;
use serde::{Serialize, Deserialize};
use crate::event::{PhysicalPosition, PhysicalSize, WindowId};


#[repr(u8)]
#[derive(StableAbi, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum WindowEvent {
    CloseRequested(WindowId),
    Destroyed(WindowId),
    RedrawRequested(WindowId),
    ScaleFactorChanged(f32),
    Resized {
        window_id: WindowId,
        size: PhysicalSize
    },
    Moved {
        window_id: WindowId,
        size: PhysicalPosition
    },
    Focused {
        window_id: WindowId,
        value: bool
    },
    Occluded {
        window_id: WindowId,
        value: bool
    },
}
