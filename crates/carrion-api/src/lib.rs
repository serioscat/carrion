pub mod event;
use crate::event::*;

// im not full understand abi_stable
use abi_stable::{
    StableAbi,
    library::RootModule,
    package_version_strings,
    sabi_types::VersionStrings,
    std_types::{
        RSlice, RStr
    }
};
use serde::{Serialize, Deserialize};

#[repr(C)]
#[derive(StableAbi, Clone, Copy, Debug)]
pub struct PluginInfo {
    pub id: RStr<'static>,
    pub description: RStr<'static>,
    pub author: RStr<'static>,
    pub version: RStr<'static>,
    pub api_version: u32,
    pub priority: i32
}

#[repr(u8)]
#[derive(StableAbi, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Action {
    /// Handle the action at the end (don't call it)
    Handled,
    /// Inputs or outputs have been overridden with new values
    Continue,
    /// Continue with the original action
    Changed,
    /// Immediately stop the hook chain and handle the original
    Stop,
}


#[repr(C)]
#[derive(StableAbi, Clone, Copy, Debug)]
#[sabi(kind(Prefix(prefix_ref = PluginRef)))]
#[sabi(missing_field(panic))]
pub struct Plugin {
    #[sabi(last_prefix_field)]
    pub info: PluginInfo,
    #[sabi(last_prefix_field)]
    pub on_event: extern "C" fn(&Event) -> Action,
    #[sabi(missing_field(default))]
    pub dependencies: RSlice<'static, RStr<'static>>
}

impl RootModule for PluginRef {
    abi_stable::declare_root_module_statics! {PluginRef}
    const BASE_NAME: &'static str = "plugin";
    const NAME: &'static str = "plugin";
    const VERSION_STRINGS: VersionStrings = package_version_strings!();
}