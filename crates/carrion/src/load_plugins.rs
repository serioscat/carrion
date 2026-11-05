use abi_stable::{library::lib_header_from_path, std_types::{RResult, RStr}};
use anyhow::Context;
use carrion_api::{PluginRef, core_api::{CORE_ID, CorePlugin}};

use std::{fs, path::PathBuf};

use crate::PLUGINS;


pub fn load() -> anyhow::Result<()> {
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


    let _core = crate::core_plugin::instantiate();

    let core_service = unsafe {
        &*crate::core_plugin::find_service(CORE_ID)
        .unwrap_or_else(|e| panic!("Core service not found: {e}"))
    };
    let core_plugin = match unsafe { core_service.get::<CorePlugin>(CORE_ID) } {
        Some(c) => c,
        None => panic!("Core service type mismatch: id “{}” found, but type is not CorePlugin", CORE_ID),
    };

    let config = toml::from_str::<toml::Table>(
        Result::from(
            core_plugin
                .get_config(
                    RStr::from("carrion")
                )
        )
        .map_err(String::from)
        .unwrap()
        .as_str()
    )?;

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

    for plugin in plugins {
        let info = plugin.info();

        if let RResult::RErr(e) = plugin.init()(core_plugin) {
            eprintln!("Failed to init plugin {}: {e}", info.id);

            match config["exit_on_error"] {
                toml::Value::Boolean(v) if v => anyhow::bail!("Failed to init plugin {}: {e}", info.id),
                _ => continue
            }
        }

        println!("Loaded - {} | {}", info.id, info.version);
        PLUGINS.lock().unwrap().push(plugin);
    }

    Ok(())
}