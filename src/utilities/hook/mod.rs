use std::{
    ffi::{CStr, c_void},
    ptr::{from_mut, null},
    sync::{Mutex, OnceLock},
};

use anyhow::{Result, anyhow, bail};

pub struct Hook {
    pub target: *mut c_void,
    pub detour: *mut c_void,
    pub original: *mut c_void,
}

unsafe impl Send for Hook {}

static TARGETS: OnceLock<Mutex<Vec<Hook>>> = OnceLock::new();

fn targets() -> &'static Mutex<Vec<Hook>> {
    TARGETS.get_or_init(|| Mutex::new(Vec::new()))
}

fn status_message(status: minhook_sys::MH_STATUS) -> String {
    let raw = unsafe { minhook_sys::MH_StatusToString(status) };

    if raw.is_null() {
        return format!("{} {status}", obfstr::obfstr!("status"));
    }

    unsafe { CStr::from_ptr(raw) }
        .to_string_lossy()
        .into_owned()
}

fn ensure_ok(status: minhook_sys::MH_STATUS, action: &str) -> Result<()> {
    if status == minhook_sys::MH_OK {
        return Ok(());
    }

    bail!(
        "{}{} {}",
        action,
        obfstr::obfstr!(":"),
        status_message(status)
    )
}

impl Hook {
    pub fn get_proto_original<F, R>(func: F) -> Option<R>
    where
        F: Fn() -> *mut c_void,
        R: From<*mut c_void>,
    {
        let Ok(guard) = targets().lock() else {
            return None;
        };

        guard
            .iter()
            .find(|hook| hook.detour == func())
            .map(|hook| R::from(hook.original))
    }

    pub fn hook(target: *const c_void, detour: *const c_void) -> Result<()> {
        let mut targets = targets()
            .lock()
            .map_err(|_| anyhow!("{}", obfstr::obfstr!("hook target storage lock poisoned")))?;

        if targets.iter().any(|hook| hook.target == target.cast_mut()) {
            bail!(
                "{}: {:p}",
                obfstr::obfstr!("hook already registered for target"),
                target
            );
        }

        let mut hk = Hook {
            target: target.cast_mut(),
            detour: detour.cast_mut(),
            original: null::<c_void>().cast_mut(),
        };

        ensure_ok(
            unsafe { minhook_sys::MH_CreateHook(hk.target, hk.detour, from_mut(&mut hk.original)) },
            obfstr::obfstr!("MH_CreateHook"),
        )?;
        if let Err(err) = ensure_ok(
            unsafe { minhook_sys::MH_EnableHook(hk.target) },
            obfstr::obfstr!("MH_EnableHook"),
        ) {
            let _ = unsafe { minhook_sys::MH_RemoveHook(hk.target) };
            return Err(err);
        }

        targets.push(hk);
        Ok(())
    }
}

pub fn eject() -> Result<()> {
    let hooks = {
        let mut targets = targets()
            .lock()
            .map_err(|_| anyhow!("{}", obfstr::obfstr!("hook target storage lock poisoned")))?;

        std::mem::take(&mut *targets)
    };

    for hook in &hooks {
        ensure_ok(
            unsafe { minhook_sys::MH_DisableHook(hook.target) },
            obfstr::obfstr!("MH_DisableHook"),
        )?;
    }

    for hook in &hooks {
        ensure_ok(
            unsafe { minhook_sys::MH_RemoveHook(hook.target) },
            obfstr::obfstr!("MH_RemoveHook"),
        )?;
    }

    Ok(())
}

#[macro_export]
macro_rules! create_hook {
    ($target_function:ident, $detour_function:ident) => {{
        let target_function = $target_function as *const std::ffi::c_void;
        let detour_function_ptr = $detour_function as *const std::ffi::c_void;

        $crate::utilities::hook::Hook::hook(target_function, detour_function_ptr)?;
    }};
}

#[macro_export]
macro_rules! get_original_fn {
    ($hook_name:ident, $fn_name:ident, ($($arg:ty),*), $ret:ty) => {
        let $fn_name: extern "system" fn($($arg),*) -> $ret = unsafe {
            std::mem::transmute::<
                *mut std::ffi::c_void,
                extern "system" fn($($arg),*) -> $ret,
            >(
                $crate::utilities::hook::Hook::get_proto_original(|| {
                    $hook_name as *mut std::ffi::c_void
                })
                .expect(obfstr::obfstr!("original hook function not registered"))
            )
        };
    };
}
