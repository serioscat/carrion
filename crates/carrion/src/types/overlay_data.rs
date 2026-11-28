use std::{collections::HashMap, sync::Arc};

use carrion_api::{
    PluginRef,
    core::{event::EventFnObj, feature::Feature, window::RenderFnObj},
};
use winit::window::Window;

use crate::{core::window::WindowRef, types::KeyboardGrab};

pub struct OverlayData {
    pub visible: bool,
    pub pending_grab: bool,
    pub kb_grab: Option<Arc<KeyboardGrab>>,
    pub features: HashMap<String, HashMap<String, Arc<Feature>>>,
    pub event_handlers: Vec<Arc<EventFnObj>>,
    pub render_handlers: Vec<Arc<RenderFnObj>>,
    pub windows: HashMap<String, HashMap<String, WindowRef>>,
    pub plugins: Vec<PluginRef>,
}

impl OverlayData {
    pub fn set_visible(&mut self, window: &Window, visible: bool) {
        self.visible = visible;
        window.set_visible(visible);
    }
}

impl Default for OverlayData {
    fn default() -> Self {
        Self {
            visible: false,
            pending_grab: false,
            kb_grab: None,
            features: HashMap::new(),
            event_handlers: Vec::new(),
            render_handlers: Vec::new(),
            windows: HashMap::new(),
            plugins: Vec::new(),
        }
    }
}
