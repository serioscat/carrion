use std::sync::Arc;

use carrion_api::core::event::{EventFnObj, EventHandlerApi};

use crate::OVERLAY_DATA;

pub struct EventHandlerApiImpl;

impl EventHandlerApi for EventHandlerApiImpl {
    fn _add_event_handler(&self, func: EventFnObj) {
        OVERLAY_DATA.lock().event_handlers.push(Arc::new(func));
    }
}
