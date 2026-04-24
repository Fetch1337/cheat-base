pub mod config;
pub mod game;
pub mod gfx;
pub mod utilities;

use windows::Win32::{
    Foundation::{HINSTANCE, HMODULE},
    System::{LibraryLoader::DisableThreadLibraryCalls, SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH}}
};

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
        log_error!("config init failed: {e}");
    }

    log_info!("initializing render");
    if let Err(e) = gfx::render::init(hmodule) {
        log_error!("render init failed: {e}");
    }

    log_info!("initializing hooks");
    if let Err(e) = game::hooks::init() {
        log_error!("hooks init failed: {e}");
    }
}

fn unload(_hmodule: HINSTANCE) {
    log_info!("unloading");
    game::hooks::eject();
    hudhook::eject();
}

/// # Safety
/// Standard DLL entry point. Must match the OS calling convention.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn DllMain(hmodule: HINSTANCE, reason: u32, _: *mut std::ffi::c_void) {
    if reason == DLL_PROCESS_ATTACH {
        let _ = unsafe { DisableThreadLibraryCalls(HMODULE(hmodule.0)) };

        std::thread::spawn({
            let h = hmodule.0 as usize;
            move || load(HINSTANCE(h as _))
        });
    } else if reason == DLL_PROCESS_DETACH {
        unload(hmodule);
    }
}
