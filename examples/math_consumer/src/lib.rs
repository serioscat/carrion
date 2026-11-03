use abi_stable::{
    export_root_module, prefix_type::PrefixTypeTrait, sabi_extern_fn, std_types::{
        RResult::{RErr, ROk}, RStr, RVec
    }
};
use carrion_api::{
    HostApi, Plugin, PluginInfo, PluginRef, Service, event::{
        Action, Event
    }
};
use math_provider_api::*;

#[allow(dead_code)]
struct MathImpl;

impl MathApi for MathImpl {
    fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}

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
        // not a dependency
        priority: 0
    }
    .leak_into_prefix()
}

#[sabi_extern_fn]
fn services() -> RVec<Service> {
    RVec::new()
}

#[sabi_extern_fn]
fn init(host_api: HostApi) {
    let service = match (host_api.find_service)("math_provider".into(), "math_module".into()) {
        ROk(svr) => unsafe { &*svr },
        RErr(e) => {
            eprintln!("{e}");
            return;
        }
    };
    let math = match unsafe { service.get::<Math>("math_module".into()) } {
        Some(m) => m,
        _ => {
            eprintln!("Service is none");
            return;
        }
    };

    println!("55+44 = {}", math.add(55, 44));
}

#[sabi_extern_fn]
fn on_event(_event: &Event) -> Action {
    Action::Handled
}