use abi_stable::{
    export_root_module,
    prefix_type::PrefixTypeTrait,
    sabi_extern_fn,
    sabi_trait::TD_Opaque,
    std_types::{RResult, RStr, RString}
};
use carrion_api::{
    Service, Plugin, PluginInfo, PluginRef, core_api::CorePlugin
};

use math_provider_api::*;

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
        priority: 54_851_253
    }
    .leak_into_prefix()
}

#[sabi_extern_fn]
fn init(core: &CorePlugin) -> RResult<(), RString> {
    let math: Math = MathApi_TO::from_value(MathImpl, TD_Opaque);
    core.add_service(Service::new(PLUGIN_MATH_ID, math));

    RResult::ROk(())
}