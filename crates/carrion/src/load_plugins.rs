use abi_stable::{
    library::lib_header_from_path,
    sabi_trait::TD_Opaque,
    std_types::{RArc, RResult},
};
use anyhow::Context;
use carrion_api::{PluginRef, core::*};
use tracing::error;

use std::{fs, path::PathBuf};

use crate::{
    CONFIG, OVERLAY_DATA,
    core::{
        event::EventHandlerApiImpl, feature::FeatureApiImpl, logger::LoggerApiImpl,
        window::WindowApiImpl,
    },
};

pub fn load() -> anyhow::Result<()> {
    let config_dir = dirs::config_dir()
        .context("Failed to get config dir.")?
        .join("carrion");
    let plugins_dir = config_dir.join("plugins");

    let config = CONFIG.lock();

    if !plugins_dir.exists() {
        fs::create_dir_all(&plugins_dir)?;
    }

    let plugins_path: Vec<PathBuf> = fs::read_dir(&plugins_dir)?
        .filter_map(|r| r.ok())
        .map(|f| f.path())
        .filter(|p| p.is_file() && p.extension().is_some_and(|e| e == "so"))
        .collect();

    let mut plugins = Vec::new();

    for plugin_path in plugins_path {
        let header = lib_header_from_path(&plugin_path).context(format!(
            "Plugin load failed: {}",
            plugin_path.to_string_lossy()
        ))?;
        let plugin = match header.init_root_module::<PluginRef>() {
            Ok(p) => p,
            Err(e) => {
                error!(path = %plugin_path.display(), error = %e, "Failed to load plugin");
                continue;
            }
        };

        let info = plugin.info();

        if OVERLAY_DATA
            .lock()
            .plugins
            .iter()
            .any(|p| p.info().id == info.id)
        {
            tracing::warn!(plugin_id = %info.id, path = %plugin_path.display(), "Plugin is already registered");
            continue;
        }

        plugins.push(plugin);
    }

    plugins.sort_by_key(|p| std::cmp::Reverse(p.priority()));

    let f: feature::FeatureApiObj = feature::FeatureApi_TO::from_value(FeatureApiImpl, TD_Opaque);
    let l: logger::LoggerApiObj = logger::LoggerApi_TO::from_value(LoggerApiImpl, TD_Opaque);
    let e: event::EventHandlerApiObj =
        event::EventHandlerApi_TO::from_value(EventHandlerApiImpl, TD_Opaque);
    let w: window::WindowApiObj = window::WindowApi_TO::from_value(WindowApiImpl, TD_Opaque);

    f.add_feature(
        logger::PLUGIN_ID.into(),
        logger::FEATURE_ID.into(),
        RArc::new(feature::Feature::new(l)),
    );
    f.add_feature(
        event::PLUGIN_ID.into(),
        event::FEATURE_ID.into(),
        RArc::new(feature::Feature::new(e)),
    );
    f.add_feature(
        window::PLUGIN_ID.into(),
        window::FEATURE_ID.into(),
        RArc::new(feature::Feature::new(w)),
    );

    for plugin in plugins {
        let info = plugin.info();

        if let RResult::RErr(e) = plugin.init()(&f) {
            error!(plugin_id = %info.id, error = %e, "Failed to init plugin");

            if config.exit_on_error {
                anyhow::bail!("Failed to init plugin {}: {e}", info.id);
            }
        }

        tracing::debug!(id = %info.id, version = %info.version, "Loaded");
        OVERLAY_DATA.lock().plugins.push(plugin);
    }

    Ok(())
}
