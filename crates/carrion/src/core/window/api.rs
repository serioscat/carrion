use abi_stable::{
    sabi_trait::TD_Opaque,
    std_types::{RResult, RStr, RString},
};
use carrion_api::core::window::{WindowApi, WindowHandleObj};
use parking_lot::Mutex;
use std::sync::Arc;

use crate::{
    OVERLAY_DATA,
    core::window::handle::{WindowData, WindowHandleImpl, WindowRef},
};

pub struct WindowApiImpl;

impl WindowApi for WindowApiImpl {
    fn create_window(
        &self,
        plugin: RStr<'_>,
        id: RStr<'_>,
        title: RStr<'_>,
    ) -> RResult<WindowHandleObj, RString> {
        let win: WindowRef = Arc::new(Mutex::new(WindowData::new(title.to_string())));

        let windows = &mut OVERLAY_DATA.lock().windows;
        let plugin_windows = windows.entry(plugin.to_string()).or_default();

        if plugin_windows.contains_key(&id.to_string()) {
            return RResult::RErr(format!("Window already registered: {plugin}/{id}").into());
        }

        plugin_windows.insert(id.to_string(), win.clone());

        RResult::ROk(WindowHandleObj::from_value(
            WindowHandleImpl(win),
            TD_Opaque,
        ))
    }
}
