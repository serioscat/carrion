use abi_stable::{
    export_root_module,
    prefix_type::PrefixTypeTrait,
    sabi_extern_fn,
    sabi_trait::TD_Opaque,
    std_types::{RArc, RResult, RStr, RString},
};
use carrion_api::{
    Plugin, PluginInfo, PluginRef,
    core::feature::{Feature, FeatureApiObj},
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
        },
        init,
        priority: 54_851_253,
    }
    .leak_into_prefix()
}

#[sabi_extern_fn]
fn init(f: &FeatureApiObj) -> RResult<(), RString> {
    let math: Math = MathApi_TO::from_value(MathImpl, TD_Opaque);
    f.add_feature(
        PLUGIN_ID.into(),
        FEATURE_ID.into(),
        RArc::new(Feature::new(math)),
    )
}
