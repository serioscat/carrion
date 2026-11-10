use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub debug: bool,
    pub toggle_bind: String,
    pub exit_on_error: bool,
    pub font_path: PathBuf
}

impl Default for Config {
    fn default() -> Self {
        Self {
            debug: false,
            toggle_bind: String::from("Super+G"),
            exit_on_error: true,
            font_path: PathBuf::from("/usr/share/fonts/liberation/LiberationSans-Regular.ttf")
        }
    }
}
