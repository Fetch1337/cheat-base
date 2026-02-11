use std::thread;
use windows::Win32::Foundation::{BOOL, HINSTANCE};
use windows::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};

use hudhook::{eject, Hudhook};
use log::{error, trace};

mod draw;
mod hooks;
mod menu;
mod sdk;
mod variables;

fn main_thread(h_module: HINSTANCE) {
    trace!("setup default config");
    {
        let config_path = format!(
            "{}\\settings.json",
            std::env::current_dir().unwrap().display()
        );

        variables::init_config(&config_path);
    }

    trace!("setup base render hooks");
    {
        if let Err(e) = Hudhook::builder()
            .with::<hudhook::hooks::dx11::ImguiDx11Hooks>(draw::Overlay)
            .with_hmodule(h_module)
            .build()
            .apply()
        {
            error!("couldn't apply hooks: {e:?}");
            eject();
        }
    }
}

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
unsafe extern "system" fn DllMain(
    h_module: HINSTANCE,
    ul_reason_for_call: u32,
    lp_reserved: *mut std::ffi::c_void,
) -> BOOL {
    if ul_reason_for_call == DLL_PROCESS_ATTACH {
        thread::spawn(move || {
            main_thread(h_module);
        });
    } else if ul_reason_for_call == DLL_PROCESS_DETACH {
        // unload
    }

    BOOL::from(true)
}
