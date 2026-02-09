use hudhook::imgui::{sys, Key, Ui};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Serialize, Deserialize, PartialEq, Copy, Clone)]
pub enum KeyMode {
    Toggle,
    Hold,
    ForceDisable,
    ForceEnable,
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct KeyBinds {
    pub key_selected: u32,
    pub mode_selected: KeyMode,
}

const MAX_KEY_INDEX: usize = sys::ImGuiKey_KeysData_SIZE as usize;

impl KeyBinds {
    pub const KEY_NONE: usize = MAX_KEY_INDEX + 1;

    pub const fn new(key: u32, mode: KeyMode) -> Self {
        Self {
            key_selected: key,
            mode_selected: mode,
        }
    }
}

static TOGGLE_KEYS: [AtomicBool; MAX_KEY_INDEX] = [const { AtomicBool::new(false) }; MAX_KEY_INDEX];
static HOLD_KEYS: [AtomicBool; MAX_KEY_INDEX] = [const { AtomicBool::new(false) }; MAX_KEY_INDEX];

pub fn on_render(ui: &Ui) {
    for key_id in Key::VARIANTS {
        let idx = key_id as usize;

        if ui.is_key_pressed(key_id) {
            let current = TOGGLE_KEYS[idx].load(Ordering::Relaxed);
            TOGGLE_KEYS[idx].store(!current, Ordering::Relaxed);
        }

        HOLD_KEYS[idx].store(ui.is_key_down(key_id), Ordering::Relaxed);
    }
}

pub fn is_bind_active(key_bind: KeyBinds) -> bool {
    let idx = key_bind.key_selected as usize;

    if idx == KeyBinds::KEY_NONE {
        return key_bind.mode_selected == KeyMode::ForceEnable;
    }

    match key_bind.mode_selected {
        KeyMode::Toggle => TOGGLE_KEYS[idx].load(Ordering::Relaxed),
        KeyMode::Hold => HOLD_KEYS[idx].load(Ordering::Relaxed),
        KeyMode::ForceDisable => false,
        KeyMode::ForceEnable => true,
    }
}
