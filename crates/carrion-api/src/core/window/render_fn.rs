use abi_stable::{sabi_trait, std_types::RBox};

use crate::core::window::ui::UiHandle;

pub type RenderFnObj = RenderFn_TO<'static, RBox<()>>;

#[sabi_trait]
pub trait RenderFn: Send + Sync {
    fn _render(&self, ui: &mut UiHandle<'_>);
}