use carrion_api::core::event::{EventFnObj, EventHandlerApi};

use crate::EVENT_HANDLERS;

pub struct EventHandlerApiImpl;

impl EventHandlerApi for EventHandlerApiImpl {
    fn _add_event_handler(&self, func: EventFnObj) {
        EVENT_HANDLERS.lock().push(func);
    }
}