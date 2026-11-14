use std::path::PathBuf;

use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub toggle_bind: String,
    pub exit_on_error: bool,
    pub font_path: PathBuf
}

impl Config {
    pub fn get() -> anyhow::Result<Config> {
        let cfg_dir = dirs::config_dir().context("Failed get config dir")?.join("carrion");
        let cfg_path = cfg_dir.join("carrion.toml");

        if !cfg_path.exists() {
            tracing::debug!("Config not found, creating...");

            std::fs::create_dir_all(cfg_dir)?;
            std::fs::write(cfg_path, toml::to_string(&Config::default())?)?;

            return Ok(Config::default());
        }

        Ok(toml::from_str::<Self>(
            &std::fs::read_to_string(&cfg_path)
            .context("Failed to read config")?
        )?)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            toggle_bind: String::from("Super+G"),
            exit_on_error: true,
            font_path: PathBuf::from("/usr/share/fonts/liberation/LiberationSans-Regular.ttf")
        }
    }
}
