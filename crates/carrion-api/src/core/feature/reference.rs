use abi_stable::std_types::RArc;
use std::{marker::PhantomData, ops::Deref};

use crate::core::feature::Feature;

pub struct FeatureRef<T> {
    feature: RArc<Feature>,
    _t: PhantomData<fn() -> T>,
}

impl<T> FeatureRef<T> {
    pub(crate) unsafe fn new(feature: RArc<Feature>) -> Self {
        Self {
            feature,
            _t: PhantomData,
        }
    }
}

impl<T> Deref for FeatureRef<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*(self.feature.ptr as *const T) }
    }
}
