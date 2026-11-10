use abi_stable::{
    export_root_module, prefix_type::PrefixTypeTrait, sabi_extern_fn, std_types::{RResult, RStr, RString}
};
use carrion_api::{
    Plugin, PluginInfo, PluginRef, core_api::CorePlugin
};

#[export_root_module]
fn instantiate() -> PluginRef {
    Plugin {
        info: PluginInfo {
            id: RStr::from_str(env!("CARGO_PKG_NAME")),
            description: RStr::from_str(env!("CARGO_PKG_DESCRIPTION")),
            author: RStr::from_str(env!("CARGO_PKG_AUTHORS")),
            version: RStr::from_str(env!("CARGO_PKG_VERSION"))
        },
        init,
        priority: -100_000_000
    }
    .leak_into_prefix()
}

#[sabi_extern_fn]
fn init(core: &CorePlugin) -> RResult<(), RString> {
    carrion_api::info!(core, "Hello, World!");

    RResult::ROk(())
}