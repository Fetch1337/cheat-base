pub mod theme;

use hudhook::imgui::{Condition, Ui};

use std::sync::atomic::{AtomicBool, Ordering};

use crate::config;
use crate::utilities::input;

pub static MENU_VISIBLE: AtomicBool = AtomicBool::new(false);

fn draw_menu(ui: &Ui) {
    if !menu_visible() {
        return;
    }

    ui.window(obfstr::obfstr!("morphey"))
        .size([300.0, 200.0], Condition::FirstUseEver)
        .build(|| {});
}

pub fn menu_visible() -> bool {
    MENU_VISIBLE.load(Ordering::Relaxed)
}

pub fn on_render(ui: &Ui) {
    let menu_visible = config::with(|cfg| input::is_bind_active(cfg.menu_key));
    MENU_VISIBLE.store(menu_visible, Ordering::Relaxed);

    draw_menu(ui);
}
