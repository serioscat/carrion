use abi_stable::std_types::ROption;
use carrion_api::types::window::Label as CarrionLabel;
use carrion_api::{core::window::UiApi, types::window::Response};
use egui::{Label, Ui};

pub struct UiApiImpl<'a> {
    pub ui: &'a mut Ui,
}

impl UiApi for UiApiImpl<'_> {
    fn label(&mut self, l: CarrionLabel) -> Response {
        let CarrionLabel {
            text,
            wrap_mode,
            sense,
            selectable,
            halign,
            show_tooltip_when_elided,
        } = l;

        let mut label =
            Label::new(text.to_string()).show_tooltip_when_elided(show_tooltip_when_elided);
        if let ROption::RSome(v) = sense {
            label = label.sense(egui::Sense::from(v));
        }
        if let ROption::RSome(v) = selectable {
            label = label.selectable(v);
        }
        if let ROption::RSome(v) = halign {
            label = label.halign(egui::Align::from(v));
        }
        if let ROption::RSome(v) = wrap_mode {
            label = label.wrap_mode(egui::TextWrapMode::from(v))
        }

        let resp = self.ui.add(label);

        Response::from(resp)
    }
}
