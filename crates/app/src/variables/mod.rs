use core::config::{impl_config, ConfigManager};
use core::input;

use hudhook::imgui::Key;
use serde::{Deserialize, Serialize};

use std::path::PathBuf;
use std::sync::OnceLock;

#[derive(Serialize, Deserialize, Clone)]
pub struct Variables {
    pub menu_key: input::KeyBinds,
    pub test_bool: bool,
}

impl Default for Variables {
    fn default() -> Self {
        Self {
            menu_key: input::KeyBinds::new(Key::Insert as u32, input::KeyMode::Toggle),
            test_bool: false,
        }
    }
}

static GLOBAL_CONFIG_DIR: OnceLock<PathBuf> = OnceLock::new();

pub fn init_config_directory() {
    GLOBAL_CONFIG_DIR.get_or_init(|| {
        let mut path = std::env::var_os("APPDATA")
            .map(PathBuf::from)
            .expect("failed to get APPDATA");

        path.push("cheat-base");
        path
    });
}

pub fn get_config_path(name: &str) -> PathBuf {
    let base = GLOBAL_CONFIG_DIR
        .get()
        .expect("config directory not initialized!");
    base.join(name)
}

impl_config!(Variables);
