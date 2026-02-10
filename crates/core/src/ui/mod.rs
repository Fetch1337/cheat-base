use crate::cfg;
use crate::input;
use hudhook::imgui::{Condition, Ui};
use std::path::Path;

pub fn load_config<P: AsRef<Path>>(path: P) {
    match cfg::load::<cfg::Config>(path.as_ref().to_str().unwrap_or("config.json")) {
        Ok(new_cfg) => {
            if let Ok(mut cfg) = cfg::get_config().write() {
                *cfg = new_cfg;
                println!("Config loaded successfully");
            }
        }
        Err(e) => eprintln!("Failed to load config: {}", e),
    }
}

pub fn save_config<P: AsRef<Path>>(path: P) {
    if let Ok(cfg) = cfg::get_config().read() {
        if let Err(e) = cfg::save(path.as_ref().to_str().unwrap_or("config.json"), &*cfg) {
            eprintln!("Failed to save config: {}", e);
        } else {
            println!("Config saved successfully");
        }
    }
}

pub fn draw_menu(ui: &Ui) {
    let config = cfg::get_config().read().unwrap();
    if !input::is_bind_active(config.menu_key) {
        return;
    }

    ui.window("Rust Internal Hook")
        .size([300.0, 200.0], Condition::FirstUseEver)
        .build(|| {
            ui.text("Nigga Settings");

            if ui.button("Save Config") {
                save_config("settings.json");
            }

            ui.same_line();

            if ui.button("Load Config") {
                load_config("settings.json");
            }
        });
}

pub fn on_render(ui: &Ui) {
    draw_menu(ui);
}
