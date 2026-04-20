use std::sync::atomic::{AtomicBool, Ordering};

use windows::Win32::Foundation::*;
use windows::Win32::UI::Input::KeyboardAndMouse::*;
use windows::Win32::UI::WindowsAndMessaging::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BindType {
    #[serde(rename = "toggle")]
    Toggle,
    #[serde(rename = "hold")]
    Hold,
    #[serde(rename = "force_on")]
    ForceOn,
    #[serde(rename = "force_off")]
    ForceOff,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct KeyBinds {
    pub key: u32,
    pub mode: BindType,
}

impl KeyBinds {
    pub const fn new(key: u32, mode: BindType) -> Self {
        Self { key, mode }
    }
}

pub const VK_TABLE_SIZE: usize = 256;
pub const VK_NONE: usize = 0;

static TOGGLE_KEYS: [AtomicBool; VK_TABLE_SIZE] = [const { AtomicBool::new(false) }; VK_TABLE_SIZE];
static HOLD_KEYS: [AtomicBool; VK_TABLE_SIZE] = [const { AtomicBool::new(false) }; VK_TABLE_SIZE];

fn get_xbutton_wparam(wparam: usize) -> u16 {
    ((wparam >> 16) & 0xFFFF) as u16
}

fn update_key_state(vk_code: u32, is_down: bool) {
    let idx = vk_code as usize;
    if idx >= VK_TABLE_SIZE {
        return;
    }

    if is_down {
        if !HOLD_KEYS[idx].load(Ordering::Relaxed) {
            TOGGLE_KEYS[idx].fetch_xor(true, Ordering::Relaxed);
            HOLD_KEYS[idx].store(true, Ordering::Relaxed);
        }
    } else {
        HOLD_KEYS[idx].store(false, Ordering::Relaxed);
    }
}

pub fn on_wnd_proc(umsg: u32, wparam: WPARAM) {
    let idx = wparam.0 as u32;

    match umsg {
        WM_LBUTTONDOWN => update_key_state(VK_LBUTTON.0 as u32, true),
        WM_LBUTTONUP => update_key_state(VK_LBUTTON.0 as u32, false),

        WM_MBUTTONDOWN => update_key_state(VK_MBUTTON.0 as u32, true),
        WM_MBUTTONUP => update_key_state(VK_MBUTTON.0 as u32, false),

        WM_RBUTTONDOWN => update_key_state(VK_RBUTTON.0 as u32, true),
        WM_RBUTTONUP => update_key_state(VK_RBUTTON.0 as u32, false),

        WM_XBUTTONDOWN => {
            let xbtn = get_xbutton_wparam(wparam.0);
            if xbtn == XBUTTON1 {
                update_key_state(VK_XBUTTON1.0 as u32, true);
            }
            if xbtn == XBUTTON2 {
                update_key_state(VK_XBUTTON2.0 as u32, true);
            }
        }
        WM_XBUTTONUP => {
            let xbtn = get_xbutton_wparam(wparam.0);
            if xbtn == XBUTTON1 {
                update_key_state(VK_XBUTTON1.0 as u32, false);
            }
            if xbtn == XBUTTON2 {
                update_key_state(VK_XBUTTON2.0 as u32, false);
            }
        }

        WM_KEYDOWN | WM_SYSKEYDOWN => update_key_state(idx, true),
        WM_KEYUP | WM_SYSKEYUP => update_key_state(idx, false),

        _ => {}
    }
}

pub fn is_bind_active(key_bind: KeyBinds) -> bool {
    let idx = key_bind.key as usize;
    if idx >= VK_TABLE_SIZE {
        return false;
    }

    if idx == VK_NONE {
        return key_bind.mode == BindType::ForceOn;
    }

    match key_bind.mode {
        BindType::Toggle => TOGGLE_KEYS[idx].load(Ordering::Relaxed),
        BindType::Hold => HOLD_KEYS[idx].load(Ordering::Relaxed),
        BindType::ForceOff => false,
        BindType::ForceOn => true,
    }
}
