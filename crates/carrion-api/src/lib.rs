#[allow(non_local_definitions)]
pub mod core;
pub mod event;
pub mod types;

use abi_stable::{
    StableAbi,
    library::RootModule,
    package_version_strings,
    sabi_types::VersionStrings,
    std_types::{RResult, RStr, RString},
};

use crate::core::feature::{Feature, FeatureApiObj};

#[repr(C)]
#[derive(StableAbi, Clone, Copy, Debug)]
pub struct PluginInfo {
    pub id: RStr<'static>,
    pub description: RStr<'static>,
    pub author: RStr<'static>,
    pub version: RStr<'static>,
}

#[repr(C)]
#[derive(StableAbi, Clone, Copy, Debug)]
pub struct HostApi {
    pub find_feature: extern "C" fn(
        plugin_id: RStr<'static>,
        feature: RStr<'static>,
    ) -> RResult<Feature, RString>,
}

#[repr(C)]
#[derive(StableAbi, Clone, Copy, Debug)]
#[sabi(kind(Prefix(prefix_ref = PluginRef)))]
#[sabi(missing_field(panic))]
pub struct Plugin {
    pub info: PluginInfo,
    pub init: extern "C" fn(&FeatureApiObj) -> RResult<(), RString>,
    #[sabi(last_prefix_field)]
    pub priority: i32,
}

impl RootModule for PluginRef {
    abi_stable::declare_root_module_statics! {PluginRef}
    const BASE_NAME: &'static str = "plugin";
    const NAME: &'static str = "plugin";
    const VERSION_STRINGS: VersionStrings = package_version_strings!();
}
