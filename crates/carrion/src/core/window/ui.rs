use std::num::NonZeroU64;

use carrion_api::{core::window::UiApi, types::{math::{Pos2, Rect}, window::{Flags, LayerId, Order, Response, Sense}}};
use egui::{Label, Ui};
use carrion_api::types::window::Label as CarrionLabel;


pub struct UiApiImpl<'a> {
    pub ui: &'a mut Ui
}

impl UiApi for UiApiImpl<'_> {
    fn label(&mut self, l: CarrionLabel) -> Response {
        let CarrionLabel {
            text,
            wrap_mode,
            sense,
            selectable,
            halign,
            show_tooltip_when_elided
        } = l;
        let label = Label::new(text.to_string());

        let resp = self.ui.add(label);

        // will be replaced
        Response {
            id: NonZeroU64::new(resp.id.value()).unwrap(),
            layer_id: LayerId { order: Order::Foreground, id: NonZeroU64::new(resp.layer_id.id.value()).unwrap() },
            rect: Rect { min: Pos2::new(resp.rect.min.x, resp.rect.min.y), max: Pos2::new(resp.rect.max.x, resp.rect.max.y) },
            interact_rect: Rect { min: Pos2::new(resp.interact_rect.min.x, resp.interact_rect.min.y), max: Pos2::new(resp.interact_rect.max.x, resp.interact_rect.max.y) },
            sense: Sense::empty(),
            interact_pointer_pos_or_nan: Pos2::new(f32::NAN, f32::NAN),
            intrinsic_size_or_nan: carrion_api::types::math::Vec2::new(f32::NAN, f32::NAN),
            flags: Flags::empty()

        }
    }
}