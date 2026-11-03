use abi_stable::{
    export_root_module, prefix_type::PrefixTypeTrait, sabi_extern_fn, std_types::{RStr, RVec}
};
use carrion_api::{
    HostApi, Plugin, PluginInfo, PluginRef, Service, event::{
        Action, Event
    }
};

#[export_root_module]
fn instantiate() -> PluginRef {
    Plugin {
        info: PluginInfo {
            id: RStr::from_str(env!("CARGO_PKG_NAME")),
            description: RStr::from_str(env!("CARGO_PKG_DESCRIPTION")),
            author: RStr::from_str(env!("CARGO_PKG_AUTHORS")),
            version: RStr::from_str(env!("CARGO_PKG_VERSION")),
            api_version: 1
        },
        init,
        on_event,
        services,
        priority: 0
    }
    .leak_into_prefix()
}

#[sabi_extern_fn]
fn services() -> RVec<Service> {
    RVec::new()
}

#[sabi_extern_fn]
fn init(_host_api: HostApi) { }

#[sabi_extern_fn]
fn on_event(_event: &Event) -> Action {
    Action::Handled
}