mod api;
mod key;
mod reference;

pub use api::*;
pub use key::*;
pub use reference::*;

use std::ffi::c_void;

use abi_stable::{StableAbi, sabi_extern_fn};


#[repr(C)]
#[derive(StableAbi, Debug)]
pub struct Feature {
    ptr: *mut c_void,
    drop_fn: extern "C" fn(*mut c_void)
}

unsafe impl Send for Feature {}
unsafe impl Sync for Feature {}

impl Feature {
    pub fn new<T: Send + Sync + 'static>(value: T) -> Self {
        Self {
            ptr: Box::into_raw(Box::new(value)) as *mut c_void,
            drop_fn: drop_impl::<T>
        }
    }
}

#[sabi_extern_fn]
fn drop_impl<T>(p: *mut c_void) {
    unsafe { drop(Box::from_raw(p as *mut T)); }
}

impl Drop for Feature {
    fn drop(&mut self) {
        (self.drop_fn)(self.ptr)
    }
}