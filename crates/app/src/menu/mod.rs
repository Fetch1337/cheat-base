use hudhook::imgui::{Condition, Ui};

use crate::variables;
use core::config::ConfigManager;
use core::input;

pub fn draw_menu(ui: &Ui) {
    let show_menu = {
        let cfg = variables::get_config().read().unwrap();
        input::is_bind_active(cfg.menu_key)
    };

    if !show_menu {
        return;
    }

    ui.window("rust internal cheat")
        .size([300.0, 200.0], Condition::FirstUseEver)
        .build(|| {
            ui.text("cheat settings");
            {
                let config_path = format!(
                    "{}\\settings.json",
                    std::env::current_dir().unwrap().display()
                );

                if ui.button("save config") {
                    let cfg = variables::get_config().read().unwrap();
                    cfg.save(&config_path).expect("failed to save");
                }

                ui.same_line();

                if ui.button("load config") {
                    variables::reload_config(&config_path);
                }
            }
        });
}

pub fn on_render(ui: &Ui) {
    draw_menu(ui);
}
