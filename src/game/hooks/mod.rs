use anyhow::Result;

use crate::utilities::hook;

// unsafe extern "system" fn hk_func(
//     a1: *mut f32,
//     a2: u64,
// ) -> u64 {
//     get_original_fn!(hk_func, original_fn, (*mut f32, u64), u64);
//     original_fn(a1, a2)
// }

pub fn init() -> Result<()> {
    // example to hook any game funcs

    // let game_process = libmem::get_process()
    // .ok_or_else(|| {
    //     crate::log_error!("failed to get process");
    //     anyhow::anyhow!(obfstr::obfstr!("failed to get process"))
    // })?;

    // let client_module = libmem::find_module_ex(&game_process, "module.dll")
    // .ok_or_else(|| {
    //     crate::log_error!("failed to find module.dll");
    //     anyhow::anyhow!(obfstr::obfstr!("failed to find module.dll"))
    // })?;

    // let func_target = libmem::sig_scan_ex(
    //     &game_process,
    //     "55 48 89 E5 66 B8 ?? ?? 48 8B 5D FC",
    //     client_module.base,
    //     client_module.size,
    // ).ok_or_else(|| {
    //     crate::log_error!("func signature scan failed");
    //     anyhow::anyhow!(obfstr::obfstr!("func signature scan failed"))
    // })?;

    // create_hook!(func_target, hk_func);

    Ok(())
}

pub fn eject() -> Result<()> {
    hook::eject()
}
