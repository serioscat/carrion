use std::num::NonZeroU64;

use crate::types::{window::{LayerId, Sense}, math::{Pos2, Rect, Vec2}};

#[repr(C)]
#[derive(abi_stable::StableAbi, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Response {
    pub layer_id: LayerId,
    pub id: NonZeroU64,
    pub rect: Rect,
    pub interact_rect: Rect,
    pub sense: Sense,
    pub interact_pointer_pos_or_nan: Pos2,
    pub intrinsic_size_or_nan: Vec2,
    pub flags: Flags
}


#[repr(C)]
#[derive(abi_stable::StableAbi, Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Flags(u16);

bitflags::bitflags! {
    impl Flags: u16 {
        const ENABLED = 1 << 0;
        const CONTAINS_POINTER = 1 << 1;
        const HOVERED = 1 << 2;
        const HIGHLIGHTED = 1 << 3;
        const CLICKED = 1 << 4;
        const FAKE_PRIMARY_CLICKED = 1 << 5;
        const LONG_TOUCHED = 1 << 6;
        const DRAG_STARTED = 1 << 7;
        const DRAGGED = 1 << 8;
        const DRAG_STOPPED = 1 << 9;
        const IS_POINTER_BUTTON_DOWN_ON = 1 << 10;
        const CHANGED = 1 << 11;
        const CLOSE = 1 << 12;
        const VISIBLE = 1 << 13;
    }
}

impl Response {
    pub fn enabled(&self) -> bool { self.flags.contains(Flags::ENABLED) }
    pub fn contains_pointer(&self) -> bool { self.flags.contains(Flags::CONTAINS_POINTER) }
    pub fn hovered(&self) -> bool { self.flags.contains(Flags::HOVERED) }
    pub fn highlighted(&self) -> bool { self.flags.contains(Flags::HIGHLIGHTED) }
    pub fn clicked(&self) -> bool { self.flags.contains(Flags::CLICKED) }
    pub fn long_touched(&self) -> bool { self.flags.contains(Flags::LONG_TOUCHED) }
    pub fn drag_started(&self) -> bool { self.flags.contains(Flags::DRAG_STARTED) }
    pub fn dragged(&self) -> bool { self.flags.contains(Flags::DRAGGED) }
    pub fn drag_stopped(&self) -> bool { self.flags.contains(Flags::DRAG_STOPPED) }
    pub fn changed(&self) -> bool { self.flags.contains(Flags::CHANGED) }
    pub fn should_close(&self) -> bool { self.flags.contains(Flags::CLOSE) }
    pub fn is_visible(&self) -> bool { self.flags.contains(Flags::VISIBLE) }

    pub fn interact_pointer_pos(&self) -> Option<Pos2> {
        (!self.interact_pointer_pos_or_nan.x.is_nan()).then_some(self.interact_pointer_pos_or_nan)
    }

    pub fn intrinsic_size(&self) -> Option<Vec2> {
        (!self.intrinsic_size_or_nan.x.is_nan()).then_some(self.intrinsic_size_or_nan)
    }

    pub fn mark_changed(&mut self) { self.flags |= Flags::CHANGED; }
}