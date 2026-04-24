use std::{
    collections::VecDeque,
    sync::{Mutex, OnceLock},
    ptr::{from_mut, null_mut},
    ffi::c_void,
    fmt,
};

use crate::{log_error, log_info};

pub struct Hook {
    pub target: *mut c_void,
    pub detour: *mut c_void,
    pub original: *mut c_void,
}

unsafe impl Send for Hook {}

#[derive(Debug)]
pub enum HookError {
    LockPoisoned,
    MinHookInitFailed,
    CreateHookFailed,
    ExternalError,
}

impl fmt::Display for HookError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LockPoisoned => write!(f, "{}", obfstr::obfstr!("lock poisoned")),
            Self::MinHookInitFailed => write!(f, "{}", obfstr::obfstr!("min hook init failed")),
            Self::CreateHookFailed => write!(f, "{}", obfstr::obfstr!("create hook failed")),
            Self::ExternalError => write!(f, "{}", obfstr::obfstr!("external error"))
        }
    }
}

static TARGETS: OnceLock<Mutex<VecDeque<Hook>>> = OnceLock::new();

fn targets() -> &'static Mutex<VecDeque<Hook>> {
    TARGETS.get_or_init(|| Mutex::new(VecDeque::new()))
}

impl Hook {
    pub fn get_proto_original<F, R>(func: F) -> Option<R>
    where
        F: Fn() -> *mut c_void,
        R: From<*mut c_void>,
    {
        let Ok(guard) = targets().lock() else {
            log_error!("failed to lock targets");
            return None;
        };

        guard
            .iter()
            .find(|hook| hook.detour == func())
            .map(|hook| R::from(hook.original))
    }

    pub fn hook(target: *const c_void, detour: *const c_void) -> Result<(), HookError> {
        let mut targets = targets().lock().map_err(|_| {
            log_error!("failed to lock targets");
            HookError::LockPoisoned
        })?;

        let mut hk = Hook {
            target: target.cast_mut(),
            detour: detour.cast_mut(),
            original: null_mut(),
        };

        let result = unsafe {
            minhook_sys::MH_CreateHook(
                hk.target,
                hk.detour,
                from_mut(&mut hk.original),
            )
        };

        if result == 0 {
            unsafe {
                minhook_sys::MH_EnableHook(hk.target);
            }

            log_info!("hook installed successfully");
            targets.push_back(hk);
            Ok(())
        } else {
            log_error!("minhook create failed");
            Err(HookError::CreateHookFailed)
        }
    }
}

pub fn init() -> Result<(), HookError> {
    let res = unsafe { minhook_sys::MH_Initialize() };

    if res != 0 {
        log_error!("failed to initialize minhook");
        return Err(HookError::MinHookInitFailed);
    }

    log_info!("minhook initialized successfully");
    Ok(())
}

#[macro_export]
macro_rules! create_hook {
    ($target_function:ident, $detour_function:ident) => {{
        let target_function =
            $target_function as *const std::ffi::c_void;
        let detour_function_ptr =
            $detour_function as *const std::ffi::c_void;

        crate::log_info!("hooking target function: {:p}", target_function);
        crate::utilities::hook::Hook::hook(target_function, detour_function_ptr)?;
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
                crate::utilities::hook::Hook::get_proto_original(|| {
                    $hook_name as *mut std::ffi::c_void
                })
                .unwrap()
            )
        };
    };
}