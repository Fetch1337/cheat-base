pub mod config;
pub mod game;
pub mod gfx;
pub mod utilities;

use windows::Win32::{
    Foundation::{HINSTANCE, HMODULE},
    System::{LibraryLoader::DisableThreadLibraryCalls, SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH}}
};

use anyhow::Error;

pub fn format_error(err: &Error) -> String {
    let mut parts = vec![err.to_string()];

    for cause in err.chain().skip(1) {
        parts.push(cause.to_string());
    }

    parts.join(obfstr::obfstr!(": "))
}

#[allow(unused_variables)]
fn load(hmodule: HINSTANCE) {
    #[cfg(debug_assertions)]
    {
        let _ = hudhook::alloc_console();
        hudhook::enable_console_colors();

        let _ = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .try_init();
    }

    log_info!("initializing config");
    if let Err(e) = config::init(obfstr::obfstr!("morphey")) {
        log_error!("config init failed: {}", format_error(&e));
    }

    log_info!("initializing render");
    if let Err(e) = gfx::render::init(hmodule) {
        log_error!("render init failed: {}", format_error(&e));
    }

    log_info!("initializing hooks");
    if let Err(e) = game::hooks::init() {
        log_error!("hooks init failed: {}", format_error(&e));
    }
}

fn unload(_hmodule: HINSTANCE) {
    log_info!("unloading");
    if let Err(e) = game::hooks::eject() {
        log_error!("hooks eject failed: {}", format_error(&e));
    }
    hudhook::eject();
}

/// # Safety
/// Standard DLL entry point. Must match the OS calling convention.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn DllMain(hmodule: HINSTANCE, reason: u32, _: *mut std::ffi::c_void) {
    if reason == DLL_PROCESS_ATTACH {
        let _ = unsafe {
            DisableThreadLibraryCalls(HMODULE(hmodule.0)) 
        };

        std::thread::spawn({
            let h = hmodule.0 as usize;
            move || load(HINSTANCE(h as _))
        });
    } else if reason == DLL_PROCESS_DETACH {
        unload(hmodule);
    }
}
