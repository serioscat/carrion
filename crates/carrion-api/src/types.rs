use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PluginConfig {
    pub general: toml::Table,
    pub element: HashMap<String, toml::Table>
}