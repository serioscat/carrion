use abi_stable::std_types::{ROption, RString};

use crate::types::window::{Align, Sense, TextWrapMode};

#[repr(C)]
#[derive(abi_stable::StableAbi, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Label {
    pub text: RString,
    pub wrap_mode: ROption<TextWrapMode>,
    pub sense: ROption<Sense>,
    pub selectable: ROption<bool>,
    pub halign: ROption<Align>,
    pub show_tooltip_when_elided: bool,
}

impl Label {
    pub fn new(text: impl Into<RString>) -> Self {
        Self {
            text: text.into(),
            wrap_mode: ROption::RNone,
            sense: ROption::RNone,
            selectable: ROption::RNone,
            halign: ROption::RNone,
            show_tooltip_when_elided: true,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    #[inline]
    pub fn wrap_mode(mut self, wrap_mode: TextWrapMode) -> Self {
        self.wrap_mode = ROption::RSome(wrap_mode);
        self
    }

    #[inline]
    pub fn wrap(mut self) -> Self {
        self.wrap_mode = ROption::RSome(TextWrapMode::Wrap);
        self
    }

    #[inline]
    pub fn truncate(mut self) -> Self {
        self.wrap_mode = ROption::RSome(TextWrapMode::Truncate);
        self
    }

    #[inline]
    pub fn extend(mut self) -> Self {
        self.wrap_mode = ROption::RSome(TextWrapMode::Extend);
        self
    }

    #[inline]
    pub fn halign(mut self, align: Align) -> Self {
        self.halign = ROption::RSome(align);
        self
    }

    #[inline]
    pub fn selectable(mut self, selectable: bool) -> Self {
        self.selectable = ROption::RSome(selectable);
        self
    }

    #[inline]
    pub fn sense(mut self, sense: Sense) -> Self {
        self.sense = ROption::RSome(sense);
        self
    }

    #[inline]
    pub fn show_tooltip_when_elided(mut self, show: bool) -> Self {
        self.show_tooltip_when_elided = show;
        self
    }
}
