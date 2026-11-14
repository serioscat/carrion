use abi_stable::{
    export_root_module, prefix_type::PrefixTypeTrait, sabi_extern_fn, std_types::{RResult, RStr, RString}
};
use carrion_api::{
    Plugin, PluginInfo, PluginRef, core::feature::FeatureApiObj
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
fn init(f: &FeatureApiObj) -> RResult<(), RString> {
    let logger = match f.import(carrion_api::core::logger::FEATURE_KEY) {
        Ok(p) => p,
        Err(e) => return RResult::RErr(RString::from(e))
    };

    carrion_api::core::logger::init_logger(logger);
    carrion_api::info!("Hello, World!");

    RResult::ROk(())
}