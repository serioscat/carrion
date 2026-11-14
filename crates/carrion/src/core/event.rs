use carrion_api::core::event::{EventHandlerApiObj, EventHandlerApi};

use crate::HANDLERS;

pub struct EventHandlerApiImpl;

impl EventHandlerApi for EventHandlerApiImpl {
    fn _call(&self, _event: carrion_api::event::Event) {}

    fn _add_event_handler(&self, handler: EventHandlerApiObj) {
        if let Ok(mut h) = HANDLERS.lock() {
            h.push(handler);
        }
    }
}