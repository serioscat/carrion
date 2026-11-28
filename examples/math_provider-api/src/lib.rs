#![allow(non_local_definitions)]

use abi_stable::{sabi_trait, std_types::RBox};
use carrion_api::core::feature::FeatureKey;

#[sabi_trait]
pub trait MathApi: Send + Sync {
    fn add(&self, a: i32, b: i32) -> i32;
}

pub const PLUGIN_ID: &str = "math";
pub const FEATURE_ID: &str = "math";

pub type Math = MathApi_TO<'static, RBox<()>>;
pub const FEATURE_KEY: FeatureKey<Math> = FeatureKey::new(PLUGIN_ID, FEATURE_ID);
