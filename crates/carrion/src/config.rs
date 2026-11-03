use std::fs;

use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub general: GeneralConfig,
    // pub plugin: HashMap<String, PluginConfig>
}

impl Config {
    fn load() -> anyhow::Result<Self> {
        let config_dir = dirs::config_dir().context("Failed get config dir.")?.join("carrion");
        let config_path = config_dir.join("carrion.toml");

        if !config_path.exists() {
            fs::create_dir(config_dir)?;
            let config_content = toml::to_string_pretty(&Self::default())?;
            fs::write(config_path, config_content)?;

            return Ok(Self::default());
        }

        let bytes = fs::read(config_path)?;

        Ok(toml::from_slice::<Self>(&bytes)?)
    }
}


impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            // plugin: HashMap::new()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GeneralConfig {
    pub log_level: LogLevel,
    pub toggle_overlay: String,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            log_level: LogLevel::Info,
            toggle_overlay: "Super+G".into()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum LogLevel {
    Info,
    Debug,
    Trace,
}
