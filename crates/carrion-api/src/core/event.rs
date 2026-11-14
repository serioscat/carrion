use abi_stable::{
    sabi_trait, sabi_trait::TD_Opaque, std_types::RBox
};

use crate::{core::feature::FeatureKey, event::Event};

pub const PLUGIN_ID: &str = "core";
pub const FEATURE_ID: &str = "event";

pub type EventHandlerApiObj = EventHandlerApi_TO<'static, RBox<()>>;
pub const FEATURE_KEY: FeatureKey<EventHandlerApiObj> = FeatureKey::new(PLUGIN_ID, FEATURE_ID);


#[sabi_trait]
pub trait EventHandlerApi: Send + Sync {
    #[doc(hidden)]
    fn _call(&self, e: Event);

    #[doc(hidden)]
    fn _add_event_handler(&self, h: EventHandlerApiObj);
}

struct FnHandler<F>(F);

impl<F> EventHandlerApi for FnHandler<F>
where 
    F: Fn(Event) + Send + Sync + 'static
{
    fn _call(&self, e: Event) {
        (self.0)(e)
    }

    fn _add_event_handler(&self, _h: EventHandlerApiObj) {}
}

impl EventHandlerApiObj {
    pub fn on_event<F>(&self, f: F)
    where
        F: Fn(Event) + Send + Sync + 'static,
    {
        self._add_event_handler(EventHandlerApi_TO::from_value(FnHandler(f), TD_Opaque));
    }
}