use std::marker::PhantomData;

pub struct FeatureKey<T> {
    pub plugin: &'static str,
    pub name: &'static str,
    _t: PhantomData<fn() -> T>,
}

impl<T> FeatureKey<T> {
    pub const fn new(plugin: &'static str, name: &'static str) -> Self {
        Self {
            plugin,
            name,
            _t: PhantomData,
        }
    }
}
