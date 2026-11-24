use abi_stable::std_types::{RArc, RResult, RStr, RString};
use carrion_api::core::feature::{Feature, FeatureApi};

use crate::FEATURES;

#[macro_export]
macro_rules! rresult_try {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(e) => return RResult::RErr(format!("{e:#}").into()),
        }
    };
}

pub struct FeatureApiImpl;

impl FeatureApi for FeatureApiImpl {
    fn add_feature(&self, plugin: RStr, feature_name: RStr, feature: RArc<Feature>) -> RResult<(), RString> {
        let plugin = plugin.as_str();
        let feature_name = feature_name.as_str();

        let mut features = FEATURES.lock();
        let plugin_features = features.entry(plugin.to_string()).or_default();

        if plugin_features.contains_key(feature_name) {
            return RResult::RErr(format!("Feature already registered: {plugin}/{feature_name}").into());
        }

        plugin_features.insert(feature_name.to_string(), feature);

        RResult::ROk(())
    }

    fn remove_feature(&self, plugin: RStr<'_>, feature: RStr<'_>) -> RResult<(), RString> {
        let feature = feature.as_str();

        let mut features = FEATURES.lock();
        let plugin_features = features.entry(plugin.to_string()).or_default();

        match plugin_features.remove(feature) {
            Some(_) => RResult::ROk(()),
            None => RResult::RErr(format!("Feature not found: {feature}").into()),
        }
    }

    fn _find_feature(&self, plugin_id: RStr<'_>, feature_id: RStr<'_>) -> RResult<RArc<Feature>, RString> {
        let features = FEATURES.lock();

        let Some(plugin) = features.get(plugin_id.as_str()) else {
            return RResult::RErr(format!("Plugin not found: {}", plugin_id.as_str()).into());
        };

        match plugin.get(feature_id.as_str()) {
            Some(f) => RResult::ROk(f.clone()),
            None => RResult::RErr(format!("Feature not found: {}", feature_id.as_str()).into()),
        }
    }
}