use abi_stable::{
    sabi_trait,
    sabi_trait::TD_Opaque,
    std_types::{RBox, RResult, RStr, RString, Tuple2},
};

use crate::{
    core::window::{
        render_fn::{RenderFn_TO, RenderFnObj},
        ui::UiHandle,
    },
    types::{fn_handler::FnHandler, math::Vec2, window::WindowTheme},
};

pub type WindowHandleObj = WindowHandle_TO<'static, RBox<()>>;

#[sabi_trait]
pub trait WindowHandle: Sync + Send {
    fn set_title(&mut self, value: RStr<'_>) -> WindowHandleObj;
    fn set_theme(&mut self, value: WindowTheme) -> WindowHandleObj;
    fn set_size(&mut self, value: Tuple2<f32, f32>) -> WindowHandleObj;
    fn set_position(&mut self, value: Vec2) -> WindowHandleObj;
    fn resizable(&mut self, value: bool) -> WindowHandleObj;
    fn open(&mut self, value: bool) -> WindowHandleObj;

    fn title(&self) -> RString;
    fn theme(&self) -> WindowTheme;
    fn size(&self) -> Tuple2<f32, f32>;
    fn position(&self) -> Vec2;
    fn is_resizable(&self) -> bool;
    fn is_open(&self) -> bool;

    #[doc(hidden)]
    fn _add_render_fn(&self, r: RenderFnObj);

    #[doc(hidden)]
    fn _destroy(&self, plugin: RStr<'_>, id: RStr<'_>) -> RResult<(), RString>;
}

impl WindowHandleObj {
    pub fn show<F: Fn(&mut UiHandle<'_>) + Send + Sync + 'static>(&self, f: F) {
        self._add_render_fn(RenderFn_TO::from_value(FnHandler(f), TD_Opaque));
    }

    pub fn destroy(&self, plugin: &str, id: &str) -> Result<(), String> {
        self._destroy(plugin.into(), id.into())
            .map_err(String::from)
            .into()
    }
}
