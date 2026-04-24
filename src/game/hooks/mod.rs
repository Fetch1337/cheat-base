use crate::{
    utilities::hook,
    create_hook,
    get_original_fn,
 };

// unsafe extern "system" fn hk_func(
//     a1: *mut f32,
//     a2: u64,
// ) -> u64 {
//     get_original_fn!(hk_func, original_fn, (*mut f32, u64), u64);
//     original_fn(a1, a2)
// }

pub fn init() -> Result<(), hook::HookError> {
    hook::init()?;

    // example to hook any game funcs

    // let game_process = libmem::get_process()
    // .ok_or_else(|| {
    //     crate::log_error!("failed to get process");
    //     hook::HookError::ExternalError
    // })?;

    // let client_module = libmem::find_module_ex(&game_process, "module.dll")
    // .ok_or_else(|| {
    //     crate::log_error!("failed to find module.dll");
    //     hook::HookError::ExternalError
    // })?;

    // let func_target = libmem::sig_scan_ex(
    //     &game_process,
    //     "55 48 89 E5 66 B8 ?? ?? 48 8B 5D FC",
    //     client_module.base,
    //     client_module.size,
    // ).ok_or_else(|| {
    //     crate::log_error!("func signature scan failed");
    //     hook::HookError::ExternalError
    // })?;

    // create_hook!(func_target, hk_func);

    Ok(())
}

pub fn eject() {

}