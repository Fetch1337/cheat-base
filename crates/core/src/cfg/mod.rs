use serde::{de::DeserializeOwned, Deserialize, Serialize};

use std::fs;
use std::sync::{OnceLock, RwLock};

use crate::input;
use hudhook::imgui::Key;

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub menu_key: input::KeyBinds,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            menu_key: input::KeyBinds::new(Key::Insert as u32, input::KeyMode::Toggle),
        }
    }
}

pub fn get_config() -> &'static RwLock<Config> {
    static CONFIG: OnceLock<RwLock<Config>> = OnceLock::new();
    CONFIG.get_or_init(|| RwLock::new(Config::default()))
}

pub fn save<T: Serialize>(path: &str, config: &T) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::to_string_pretty(config)?;
    fs::write(path, data)?;
    Ok(())
}

pub fn load<T: DeserializeOwned>(path: &str) -> Result<T, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(path)?;
    let config = serde_json::from_str(&data)?;
    Ok(config)
}
