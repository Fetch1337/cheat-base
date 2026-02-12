use serde::{de::DeserializeOwned, Serialize};
use std::fs;
use std::io;
use std::path::Path;
use std::sync::RwLock;

pub trait ConfigManager: Serialize + DeserializeOwned + Default + Clone + 'static {
    fn instance() -> &'static RwLock<Self>;

    fn init<P: AsRef<Path>>(path: P) {
        let cfg = Self::load(path);
        let lock = Self::instance();
        let mut guard = lock.write().unwrap_or_else(|e| e.into_inner());
        *guard = cfg;
    }

    fn reload<P: AsRef<Path>>(path: P) {
        let new_cfg = Self::load(path);
        let mut guard = Self::instance().write().unwrap_or_else(|e| e.into_inner());
        *guard = new_cfg;
    }

    fn load<P: AsRef<Path>>(path: P) -> Self {
        let path = path.as_ref();
        if let Ok(content) = fs::read_to_string(path) {
            if let Ok(config) = serde_json::from_str(&content) {
                return config;
            }
        }
        let default_config = Self::default();
        let _ = default_config.save(path);
        default_config
    }

    fn save<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        let temp_path = path.with_extension("tmp");
        fs::write(&temp_path, content)?;
        fs::rename(&temp_path, path)
    }
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! _impl_config {
    ($t:ty) => {
        impl ConfigManager for $t {
            fn instance() -> &'static std::sync::RwLock<Self> {
                static INSTANCE: std::sync::OnceLock<std::sync::RwLock<$t>> =
                    std::sync::OnceLock::new();
                INSTANCE.get_or_init(|| std::sync::RwLock::new(<$t>::default()))
            }
        }

        impl $t {
            pub fn read<F, R>(f: F) -> R
            where
                F: FnOnce(&$t) -> R,
            {
                let lock = <$t as $crate::config::ConfigManager>::instance();
                let guard = lock.read().unwrap_or_else(|e| e.into_inner());
                f(&*guard)
            }

            pub fn write<F, R>(f: F) -> R
            where
                F: FnOnce(&mut $t) -> R,
            {
                let lock = <$t as $crate::config::ConfigManager>::instance();
                let mut guard = lock.write().unwrap_or_else(|e| e.into_inner());
                f(&mut *guard)
            }
        }
    };
}

pub use _impl_config as impl_config;
