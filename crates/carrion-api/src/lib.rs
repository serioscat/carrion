pub mod event;
pub mod types;

use std::ffi::c_void;

pub use crate::event::*;
pub use crate::types::*;

use abi_stable::sabi_extern_fn;
use abi_stable::std_types::RResult;
use abi_stable::std_types::RVec;
// im not full understand abi_stable
use abi_stable::{
    StableAbi,
    library::RootModule,
    package_version_strings,
    sabi_types::VersionStrings,
    std_types::{RString, RStr}
};

#[repr(C)]
#[derive(StableAbi, Debug)]
pub struct Service {
    pub id: RString,
    ptr: *mut c_void,
    drop_fn: extern "C" fn(*mut c_void)
}

unsafe impl Send for Service {}
unsafe impl Sync for Service {}

#[sabi_extern_fn]
fn drop_impl<T>(p: *mut c_void) {
    unsafe { drop(Box::from_raw(p as *mut T)); }
}

impl Service {
    pub fn new<T: Send + Sync + 'static>(id: &str, value: T) -> Self {
        Self {
            id: id.into(),
            ptr: Box::into_raw(Box::new(value)) as *mut c_void,
            drop_fn: drop_impl::<T>
        }
    }

    pub unsafe fn get<T>(&self, id: &str) -> Option<&T> {
        (self.id.as_str() == id).then(|| unsafe { &*(self.ptr as *const T) } )
    }
}

impl Drop for Service {
    fn drop(&mut self) {
        (self.drop_fn)(self.ptr)
    }
}

#[repr(C)]
#[derive(StableAbi, Clone, Copy, Debug)]
pub struct PluginInfo {
    pub id: RStr<'static>,
    pub description: RStr<'static>,
    pub author: RStr<'static>,
    pub version: RStr<'static>,
    pub api_version: u32
}

#[repr(C)]
#[derive(StableAbi, Clone, Copy, Debug)]
pub struct HostApi {
    pub find_service: extern "C" fn(plugin_id: RStr<'static>, service: RStr<'static>) -> RResult<*const Service, RString>
}


#[repr(C)]
#[derive(StableAbi, Clone, Copy, Debug)]
#[sabi(kind(Prefix(prefix_ref = PluginRef)))]
#[sabi(missing_field(panic))]
pub struct Plugin {
    #[sabi(last_prefix_field)]
    pub info: PluginInfo,
    #[sabi(last_prefix_field)]
    pub init: extern "C" fn(HostApi),
    #[sabi(last_prefix_field)]
    pub on_event: extern "C" fn(&Event) -> Action,
    #[sabi(last_prefix_field)]
    pub services: extern "C" fn() -> RVec<Service>,
    #[sabi(last_prefix_field)]
    pub priority: i32
}

impl RootModule for PluginRef {
    abi_stable::declare_root_module_statics! {PluginRef}
    const BASE_NAME: &'static str = "plugin";
    const NAME: &'static str = "plugin";
    const VERSION_STRINGS: VersionStrings = package_version_strings!();
}