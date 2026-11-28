use std::num::NonZeroU64;

use crate::types::{
    math::{Pos2, Rect, Vec2},
    window::{LayerId, Sense},
};

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
    pub flags: Flags,
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

macro_rules! flag_getters {
    ($($(#[$m:meta])* $name:ident => $flag:ident),*) => {
        $(
            $(#[$m])*
            pub fn $name(&self) -> bool {
                self.flags.contains(Flags::$flag)
            }
        )*
    };
}

impl Response {
    flag_getters! {
        enabled => ENABLED,
        contains_pointer => CONTAINS_POINTER,
        hovered => HOVERED,
        highlighted => HIGHLIGHTED,
        clicked => CLICKED,
        long_touched => LONG_TOUCHED,
        drag_started => DRAG_STARTED,
        dragged => DRAGGED,
        drag_stopped => DRAG_STOPPED,
        changed => CHANGED,
        should_close => CLOSE,
        is_visible => VISIBLE
    }

    pub fn interact_pointer_pos(&self) -> Option<Pos2> {
        (!self.interact_pointer_pos_or_nan.x.is_nan()).then_some(self.interact_pointer_pos_or_nan)
    }

    pub fn intrinsic_size(&self) -> Option<Vec2> {
        (!self.intrinsic_size_or_nan.x.is_nan()).then_some(self.intrinsic_size_or_nan)
    }

    pub fn mark_changed(&mut self) {
        self.flags |= Flags::CHANGED;
    }
}

#[cfg(feature = "egui")]
impl From<egui::response::Flags> for Flags {
    fn from(value: egui::response::Flags) -> Self {
        convert_bitflags!(value, egui::response::Flags => Flags {
            ENABLED, CONTAINS_POINTER, HOVERED,
            HIGHLIGHTED, CLICKED, FAKE_PRIMARY_CLICKED,
            LONG_TOUCHED, DRAG_STARTED, DRAGGED,
            DRAG_STOPPED, IS_POINTER_BUTTON_DOWN_ON,
            CHANGED, CLOSE, VISIBLE
        })
    }
}

#[cfg(feature = "egui")]
impl From<egui::Response> for Response {
    fn from(value: egui::Response) -> Self {
        Response {
            id: NonZeroU64::new(value.id.value()).unwrap(),
            layer_id: LayerId::from(value.layer_id),
            rect: Rect::from(value.rect),
            interact_rect: Rect::from(value.interact_rect),
            sense: Sense::from(value.sense),
            interact_pointer_pos_or_nan: Pos2::from(value.interact_pointer_pos_or_nan),
            intrinsic_size_or_nan: crate::types::math::Vec2::from(value.intrinsic_size_or_nan),
            flags: Flags::from(value.flags),
        }
    }
}
