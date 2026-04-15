pub mod variables;

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use parking_lot::Mutex;
use serde::{Serialize, de::DeserializeOwned};

use variables::Variables;

static CONFIG: OnceLock<Mutex<Variables>> = OnceLock::new();
static CONFIG_DIR: OnceLock<PathBuf> = OnceLock::new();

fn get_config_dir(app_name: &str) -> &'static PathBuf {
    CONFIG_DIR.get_or_init(|| {
        let path = std::env::var_os(obfstr::obfstr!("APPDATA"))
            .map(PathBuf::from)
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());

        let mut path = path;
        path.push(app_name);
        fs::create_dir_all(&path).ok();

        path
    })
}

pub fn get_path(name: &str) -> PathBuf {
    CONFIG_DIR
        .get()
        .expect("config directory not initialized")
        .join(name)
}

pub fn init(name: &str) {
    let dir = get_config_dir(name);
    let path = dir.join(obfstr::obfstr!("config.json"));

    let (cfg, is_default) = load::<Variables>(&path);
    if is_default {
        save(&cfg, &path);
    }

    CONFIG.set(Mutex::new(cfg)).ok();
}

pub fn get() -> parking_lot::MutexGuard<'static, Variables> {
    CONFIG.get().expect("config not initialized").lock()
}

pub fn save<T: Serialize>(cfg: &T, path: &Path) {
    match serde_json::to_string_pretty(cfg) {
        Ok(json) => {
            if let Err(e) = fs::write(path, json) {
                tracing::debug!("failed to save config: {}", e);
            }
        }
        Err(e) => {
            tracing::debug!("failed to serialize config: {}", e);
        }
    }
}

pub fn load<T: Default + DeserializeOwned>(path: &Path) -> (T, bool) {
    match fs::read_to_string(path) {
        Ok(data) => match serde_json::from_str(&data) {
            Ok(cfg) => (cfg, false),
            Err(e) => {
                tracing::debug!("failed to parse config: {}, using defaults", e);
                (T::default(), true)
            }
        },
        Err(_) => (T::default(), true),
    }
}
