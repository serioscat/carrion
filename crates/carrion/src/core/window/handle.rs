use std::sync::Arc;
use abi_stable::{sabi_trait::TD_Opaque, std_types::{RResult, RStr, RString, Tuple2}};
use carrion_api::{core::window::{RenderFnObj, WindowHandle, WindowHandleObj}, types::{math::{Vec2, vec2}, window::WindowTheme}};
use parking_lot::Mutex;

use crate::{RENDER_HANDLERS, WINDOWS};

pub type WindowRef = Arc<Mutex<WindowData>>;

pub struct WindowData {
    pub title: String,
    pub size: (f32, f32),
    pub position: Vec2,
    pub open: bool,
    pub resizable: bool,
    pub theme: WindowTheme,
    pub _render_fn: Option<RenderFnObj>
}

#[derive(Clone)]
pub struct WindowHandleImpl(pub WindowRef);

impl WindowData {
    pub fn new(title: String) -> Self {
        Self {
            title,
            size: (100., 100.),
            position: vec2(100., 100.),
            open: true,
            resizable: true,
            theme: WindowTheme::default(),
            _render_fn: None
        }
    }
}

impl WindowHandle for WindowHandleImpl {
    fn set_title(&mut self, title: RStr<'_>) -> WindowHandleObj {
        self.0.lock().title = title.to_string();
        WindowHandleObj::from_value(self.clone(), TD_Opaque)
    }

    fn set_theme(&mut self, theme: WindowTheme) -> WindowHandleObj {
        self.0.lock().theme = theme;
        WindowHandleObj::from_value(self.clone(), TD_Opaque)
    }

    fn set_size(&mut self, size: Tuple2<f32, f32>) -> WindowHandleObj {
        self.0.lock().size = size.into_tuple();
        WindowHandleObj::from_value(self.clone(), TD_Opaque)
    }

    fn set_position(&mut self, pos: Vec2) -> WindowHandleObj {
        self.0.lock().position = pos;
        WindowHandleObj::from_value(self.clone(), TD_Opaque)
    }

    fn resizable(&mut self, resizable: bool) -> WindowHandleObj {
        self.0.lock().resizable = resizable;
        WindowHandleObj::from_value(self.clone(), TD_Opaque)
    }

    fn open(&mut self, open: bool) -> WindowHandleObj {
        self.0.lock().open = open;
        WindowHandleObj::from_value(self.clone(), TD_Opaque)
    }

    fn theme(&self) -> WindowTheme {
        self.0.lock().theme
    }

    fn title(&self) -> RString {
        RString::from(self.0.lock().title.clone())
    }

    fn size(&self) -> Tuple2<f32, f32> {
        Tuple2::from(self.0.lock().size)
    }

    fn position(&self) -> Vec2 {
        self.0.lock().position
    }

    fn is_resizable(&self) -> bool {
        self.0.lock().resizable
    }

    fn is_open(&self) -> bool {
        self.0.lock().open
    }

    fn _add_render_fn(&self, func: RenderFnObj) {
        RENDER_HANDLERS.lock().push(func);
    }

    fn _destroy(&self, plugin: RStr<'_>, id: RStr<'_>) -> RResult<(), RString> {
        let mut windows = WINDOWS.lock();
        let plugin_windows = windows.entry(plugin.to_string()).or_default();

        match plugin_windows.remove(&id.to_string()) {
            Some(_) => RResult::ROk(()),
            None => RResult::RErr(format!("Window not found: {id}").into()),
        }
    }
}