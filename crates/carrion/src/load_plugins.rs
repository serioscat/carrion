use abi_stable::{library::lib_header_from_path, std_types::RResult};
use anyhow::Context;
use carrion_api::{PluginRef, core_api::{CorePlugin}};
use tracing::{debug, error};

use std::{fs, path::PathBuf};

use crate::{CONFIG, PLUGINS};


pub fn load(core_plugin: &CorePlugin) -> anyhow::Result<()> {
    let config_dir = dirs::config_dir().context("Failed to get config dir.")?.join("carrion");
    let plugins_dir = config_dir.join("plugins");

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
        let header = lib_header_from_path(&plugin_path).context(format!("Plugin load failed: {}", plugin_path.to_string_lossy()))?;
        let plugin = header.init_root_module::<PluginRef>()?;

        let info = plugin.info();

        if PLUGINS.lock().unwrap().iter().any(|p| p.info().id == info.id) {
            anyhow::bail!("The {} plugin is already registered.", info.id);
        }

        plugins.push(plugin);
    }

    plugins.sort_by_key(|p| std::cmp::Reverse(p.priority()));

    let config = CONFIG.lock().unwrap(); 
    

    for plugin in plugins {
        let info = plugin.info();

        if let RResult::RErr(e) = plugin.init()(&core_plugin) {
            error!("Failed to init plugin {}: {e}", info.id);

            if config.exit_on_error {
                anyhow::bail!("Failed to init plugin {}: {e}", info.id);
            }
        }

        debug!(name = info.id.to_string(), version = info.version.to_string(), "Plugin loaded");
        PLUGINS.lock().unwrap().push(plugin);
    }

    Ok(())
}