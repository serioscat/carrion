use abi_stable::{sabi_trait, std_types::{RBox, RResult, RStr, RString}};

use crate::core::{feature::FeatureKey, window::WindowHandleObj};


pub const PLUGIN_ID: &str = "core";
pub const FEATURE_ID: &str = "window";

pub type WindowApiObj = WindowApi_TO<'static, RBox<()>>;
pub const FEATURE_KEY: FeatureKey<WindowApiObj> = FeatureKey::new(PLUGIN_ID, FEATURE_ID);

#[sabi_trait]
pub trait WindowApi: Send + Sync {
    fn create_window(&self, plugin: RStr<'_>, id: RStr<'_>, title: RStr<'_>) -> RResult<WindowHandleObj, RString>;
}
