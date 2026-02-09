use std::thread;
use windows::Win32::Foundation::{BOOL, HINSTANCE};
use windows::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};

mod instance;

fn main_thread(h_module: HINSTANCE) {
    core::initialize(h_module);
    instance::initialize();
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
