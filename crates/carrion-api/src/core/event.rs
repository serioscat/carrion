use abi_stable::{sabi_trait, sabi_trait::TD_Opaque, std_types::RBox};

use crate::{core::feature::FeatureKey, event::Event, types::fn_handler::FnHandler};

pub const PLUGIN_ID: &str = "core";
pub const FEATURE_ID: &str = "event";

pub type EventHandlerApiObj = EventHandlerApi_TO<'static, RBox<()>>;
pub type EventFnObj = EventFn_TO<'static, RBox<()>>;
pub const FEATURE_KEY: FeatureKey<EventHandlerApiObj> = FeatureKey::new(PLUGIN_ID, FEATURE_ID);

#[sabi_trait]
pub trait EventHandlerApi: Send + Sync {
    #[doc(hidden)]
    fn _add_event_handler(&self, h: EventFnObj);
}

#[sabi_trait]
pub trait EventFn: Send + Sync {
    #[doc(hidden)]
    fn _call(&self, e: Event);
}

impl EventHandlerApiObj {
    pub fn on_event<F>(&self, f: F)
    where
        F: Fn(Event) + Send + Sync + 'static,
    {
        self._add_event_handler(EventFn_TO::from_value(FnHandler(f), TD_Opaque));
    }
}
