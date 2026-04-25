pub mod variables;

use std::{
    fs::{self, OpenOptions},
    io,
    path::{Path, PathBuf},
    sync::OnceLock,
};

use parking_lot::Mutex;

use serde::{Serialize, de::DeserializeOwned};

use anyhow::{Context, Result, anyhow};

use variables::Variables;

static CONFIG: OnceLock<Mutex<Variables>> = OnceLock::new();
static CONFIG_DIR: OnceLock<PathBuf> = OnceLock::new();

fn get_config_dir(app_name: &str) -> Result<&'static PathBuf> {
    if let Some(dir) = CONFIG_DIR.get() {
        return Ok(dir);
    }

    let mut path = std::env::var_os(obfstr::obfstr!("APPDATA"))
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            std::env::current_dir().unwrap_or_else(|_| PathBuf::from(obfstr::obfstr!(".")))
        });

    path.push(app_name);
    fs::create_dir_all(&path).with_context(|| {
        format!(
            "{} {}",
            obfstr::obfstr!("failed to create config directory at"),
            path.display()
        )
    })?;

    let _ = CONFIG_DIR.set(path);
    CONFIG_DIR
        .get()
        .ok_or_else(|| anyhow!("{}", obfstr::obfstr!("config directory not initialized")))
}

pub fn get_path(name: &str) -> Result<PathBuf> {
    CONFIG_DIR
        .get()
        .cloned()
        .map(|dir| dir.join(name))
        .ok_or_else(|| anyhow!("{}", obfstr::obfstr!("config directory not initialized")))
}

#[allow(unused_variables)]
pub fn init(name: &str) -> Result<()> {
    let path = get_config_dir(name)?.join(obfstr::obfstr!("config.json"));

    let cfg = match load::<Variables>(&path) {
        Ok(cfg) => cfg,
        Err(err)
            if err
                .downcast_ref::<io::Error>()
                .is_some_and(|e| e.kind() == io::ErrorKind::NotFound) =>
        {
            let cfg = Variables::default();
            save(&cfg, &path)?;
            cfg
        }
        Err(err) => {
            return Err(err).with_context(|| {
                format!(
                    "{} {}",
                    obfstr::obfstr!("failed to load config from"),
                    path.display()
                )
            });
        }
    };

    CONFIG
        .set(Mutex::new(cfg))
        .map_err(|_| anyhow!("{}", obfstr::obfstr!("config already initialized")))?;

    Ok(())
}

pub fn get() -> Option<parking_lot::MutexGuard<'static, Variables>> {
    CONFIG.get().map(|cfg| cfg.lock())
}

pub fn snapshot() -> Variables {
    get().map(|cfg| *cfg).unwrap_or_default()
}

pub fn with<R>(f: impl FnOnce(&Variables) -> R) -> R {
    let cfg = snapshot();
    f(&cfg)
}

pub fn with_mut<R>(f: impl FnOnce(&mut Variables) -> R) -> Option<R> {
    get().map(|mut cfg| f(&mut cfg))
}

pub fn save<T: Serialize>(cfg: &T, path: &Path) -> Result<()> {
    let tmp_path = path.with_extension(obfstr::obfstr!("json.tmp"));
    let file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&tmp_path)
        .with_context(|| {
            format!(
                "{} {}",
                obfstr::obfstr!("failed to create config file"),
                tmp_path.display()
            )
        })?;

    serde_json::to_writer_pretty(&file, cfg).with_context(|| {
        format!(
            "{} {}",
            obfstr::obfstr!("failed to serialize config to"),
            tmp_path.display()
        )
    })?;

    file.sync_all().with_context(|| {
        format!(
            "{} {}",
            obfstr::obfstr!("failed to flush config file"),
            tmp_path.display()
        )
    })?;

    fs::rename(&tmp_path, path).with_context(|| {
        format!(
            "{} {} {}",
            obfstr::obfstr!("failed to replace config file"),
            tmp_path.display(),
            path.display()
        )
    })?;

    Ok(())
}

pub fn load<T: DeserializeOwned>(path: &Path) -> Result<T> {
    let file = fs::File::open(path).with_context(|| {
        format!(
            "{} {}",
            obfstr::obfstr!("failed to open config file"),
            path.display()
        )
    })?;

    serde_json::from_reader(file).with_context(|| {
        format!(
            "{} {}",
            obfstr::obfstr!("failed to deserialize config from"),
            path.display()
        )
    })
}