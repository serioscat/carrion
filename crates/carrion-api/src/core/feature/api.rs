use crate::core::feature::{Feature, FeatureRef};

use super::FeatureKey;

use abi_stable::{
    sabi_trait,
    std_types::{RArc, RBox, RResult, RStr, RString},
};

pub type FeatureApiObj = FeatureApi_TO<'static, RBox<()>>;

pub const PLUGIN_ID: &str = "core";
pub const FEATURE_ID: &str = "feature";

pub const FEATURE_KEY: FeatureKey<FeatureApiObj> = FeatureKey::new(PLUGIN_ID, FEATURE_ID);

#[sabi_trait]
pub trait FeatureApi: Send + Sync {
    fn add_feature(
        &self,
        plugin: RStr,
        feature_name: RStr,
        plugin_id: RArc<Feature>,
    ) -> RResult<(), RString>;

    fn remove_feature(&self, plugin: RStr, feature_id: RStr) -> RResult<(), RString>;

    #[doc(hidden)]
    fn _find_feature(&self, plugin_id: RStr, feature_id: RStr) -> RResult<RArc<Feature>, RString>;
}

impl FeatureApiObj {
    pub fn import<T>(&self, feature: FeatureKey<T>) -> Result<FeatureRef<T>, String> {
        match self._find_feature(feature.plugin.into(), feature.name.into()) {
            RResult::ROk(f) => Ok(unsafe { FeatureRef::new(f) }),
            RResult::RErr(e) => Err(e.to_string()),
        }
    }
}
