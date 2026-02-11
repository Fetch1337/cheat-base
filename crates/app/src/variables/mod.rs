use core::config::ConfigManager;
use serde::{Deserialize, Serialize};
use std::sync::{OnceLock, RwLock};

use core::input;
use hudhook::imgui::Key;

#[derive(Serialize, Deserialize, Clone)]
pub struct Variables {
    pub menu_key: input::KeyBinds,
}

impl Default for Variables {
    fn default() -> Self {
        Self {
            menu_key: input::KeyBinds::new(Key::Insert as u32, input::KeyMode::Toggle),
        }
    }
}

impl ConfigManager for Variables {}

static CONFIG: OnceLock<RwLock<Variables>> = OnceLock::new();

pub fn init_config(path: &str) {
    let cfg = Variables::load_or_default(path);
    let _ = CONFIG.set(RwLock::new(cfg));
}

pub fn reload_config(path: &str) {
    let new_cfg = Variables::load_or_default(path);
    if let Some(lock) = CONFIG.get() {
        let mut cfg = lock.write().unwrap();
        *cfg = new_cfg;
    }
}

pub fn get_config() -> &'static RwLock<Variables> {
    CONFIG.get().expect("config must be initialized before use")
}
