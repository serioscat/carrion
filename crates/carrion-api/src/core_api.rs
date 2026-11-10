use abi_stable::{
    sabi_trait,
    sabi_trait::TD_Opaque,
    std_types::{
        RBox, RResult, RStr, RString
    }
};

use crate::{event::Event, Service, LogLevel};

#[sabi_trait]
pub trait EventHandler: Send + Sync {
    #[doc(hidden)]
    fn call(&self, c: &CorePlugin, e: Event);
}

pub type Handler = EventHandler_TO<'static, RBox<()>>;

struct FnHandler<F>(F);

impl<F> EventHandler for FnHandler<F>
where 
    F: Fn(&CorePlugin, Event) + Send + Sync + 'static
{
    fn call(&self, c: &CorePlugin, e: Event) {
        (self.0)(c, e)
    }
}

#[sabi_trait]
pub trait CorePluginApi: Send + Sync {
    fn get_config(&self, id: RStr) -> RResult<RString, RString>;

    fn add_service(&self, service: Service) -> RResult<(), RString>;

    fn remove_service(&self, service_id: RStr) -> RResult<(), RString>;

    #[doc(hidden)]
    fn _log(&self, level: LogLevel, target: RStr, message: RStr);

    #[doc(hidden)]
    fn _find_service(&self, id: RStr) -> RResult<*const Service, RString>;

    #[doc(hidden)]
    fn _add_event_handler(&self, h: Handler);
}

pub type CorePlugin = CorePluginApi_TO<'static, RBox<()>>;

impl CorePlugin {
    pub fn import<T>(&self, id: &str) -> Result<&T, String> {
        let service = match self._find_service(id.into()) {
            RResult::ROk(p) => unsafe { &*p },
            RResult::RErr(e) => return Err(e.to_string()),
        };

        unsafe { service.get::<T>(id) }.ok_or_else(|| format!("Export {id} type mismatch"))
    }


    pub fn on_event<F>(&self, f: F)
    where
        F: Fn(&CorePlugin, Event) + Send + Sync + 'static,
    {
        self._add_event_handler(EventHandler_TO::from_value(FnHandler(f), TD_Opaque));
    }
}


pub const CORE_ID: &str = "carrion::core";