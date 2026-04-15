pub mod config;
pub mod game;
pub mod gfx;
pub mod utilities;

use windows::Win32::Foundation::{HINSTANCE, HMODULE};
use windows::Win32::System::LibraryLoader::DisableThreadLibraryCalls;
use windows::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};

fn load(hmodule: HINSTANCE) {
    #[cfg(feature = "debug-logging")]
    {
        let _ = hudhook::alloc_console();
        hudhook::enable_console_colors();

        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .init();
    }

    tracing::info!("{}", obfstr::obfstr!("initializing config"));
    config::init(obfstr::obfstr!("morphey"));

    tracing::info!("{}", obfstr::obfstr!("initializing render"));
    gfx::render::init(hmodule);
}

fn unload(_hmodule: HINSTANCE) {
    tracing::info!("{}", obfstr::obfstr!("unloading"));
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
