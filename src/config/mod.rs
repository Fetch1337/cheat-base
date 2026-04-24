pub mod variables;

use std::{
    fs,
    io,
    path::{Path, PathBuf},
    sync::OnceLock,
    error::Error,
    fmt,
};

use parking_lot::Mutex;
use serde::{de::DeserializeOwned, Serialize};

use variables::Variables;

static CONFIG: OnceLock<Mutex<Variables>> = OnceLock::new();
static CONFIG_DIR: OnceLock<PathBuf> = OnceLock::new();

#[derive(Debug)]
pub enum ConfigError {
    Io(io::Error),
    Json(serde_json::Error),
    NotInitialized,
    AlreadyInitialized,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "{}: {e}", obfstr::obfstr!("io error")),
            Self::Json(e) => write!(f, "{}: {e}", obfstr::obfstr!("json error")),
            Self::NotInitialized => write!(f, "{}", obfstr::obfstr!("config not initialized")),
            Self::AlreadyInitialized => {
                write!(f, "{}", obfstr::obfstr!("config already initialized"))
            }
        }
    }
}

impl Error for ConfigError {}

impl From<io::Error> for ConfigError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<serde_json::Error> for ConfigError {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}

fn get_config_dir(app_name: &str) -> Result<&'static PathBuf, ConfigError> {
    if let Some(dir) = CONFIG_DIR.get() {
        return Ok(dir);
    }

    let mut path = std::env::var_os(obfstr::obfstr!("APPDATA"))
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

    path.push(app_name);
    fs::create_dir_all(&path)?;

    let _ = CONFIG_DIR.set(path);
    CONFIG_DIR.get().ok_or(ConfigError::NotInitialized)
}

pub fn get_path(name: &str) -> Result<PathBuf, ConfigError> {
    CONFIG_DIR
        .get()
        .cloned()
        .map(|dir| dir.join(name))
        .ok_or(ConfigError::NotInitialized)
}

#[allow(unused_variables)]
pub fn init(name: &str) -> Result<(), ConfigError> {
    let dir = get_config_dir(name);
    let path = dir?.join(obfstr::obfstr!("config.json"));

    let cfg = match load::<Variables>(&path) {
        Ok(cfg) => cfg,
        Err(ConfigError::Io(e)) if e.kind() == io::ErrorKind::NotFound => {
            let cfg = Variables::default();
            save(&cfg, &path)?;
            cfg
        }
        Err(e) => {
            crate::log_error!("failed to load config: {e}");

            let cfg = Variables::default();
            if let Err(save_err) = save(&cfg, &path) {
                crate::log_error!("failed to rewrite default config: {save_err}");
            }
            cfg
        }
    };

    CONFIG
        .set(Mutex::new(cfg))
        .map_err(|_| ConfigError::AlreadyInitialized)?;

    Ok(())
}

pub fn get() -> Option<parking_lot::MutexGuard<'static, Variables>> {
    CONFIG.get().map(|cfg| cfg.lock())
}

pub fn save<T: Serialize>(cfg: &T, path: &Path) -> Result<(), ConfigError> {
    let file = fs::File::create(path)?;
    serde_json::to_writer_pretty(file, cfg)?;
    Ok(())
}

pub fn load<T: DeserializeOwned>(path: &Path) -> Result<T, ConfigError> {
    let file = fs::File::open(path)?;
    Ok(serde_json::from_reader(file)?)
}
