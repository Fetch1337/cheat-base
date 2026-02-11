use serde::{de::DeserializeOwned, Serialize};
use std::fs;
use std::path::Path;

pub trait ConfigManager: Serialize + DeserializeOwned + Default {
    fn load_or_default<P: AsRef<Path>>(path: P) -> Self {
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(config) = serde_json::from_str(&content) {
                return config;
            }
        }

        let default_config = Self::default();
        let _ = default_config.save(path);
        default_config
    }

    fn save<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let content = serde_json::to_string_pretty(self).unwrap();
        fs::write(path, content)
    }
}
