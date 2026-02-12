use hudhook::imgui::{Condition, Ui};

use crate::variables::{self, *};
use core::config::ConfigManager;
use core::input;

pub fn draw_menu(ui: &Ui) {
    if !Variables::read(|cfg| input::is_bind_active(cfg.menu_key)) {
        return;
    }

    ui.window("rust internal cheat")
        .size([300.0, 200.0], Condition::FirstUseEver)
        .build(|| {
            ui.text("cheat settings");
            {
                let config_path = variables::get_config_path("settings.json");

                Variables::write(|cfg| {
                    ui.checkbox("enable", &mut cfg.test_bool);
                });

                if ui.button("save config") {
                    Variables::read(|cfg| {
                        let _ = cfg.save(&config_path);
                    });
                }

                ui.same_line();

                if ui.button("load config") {
                    Variables::reload(&config_path);
                }
            }
        });
}

pub fn on_render(ui: &Ui) {
    draw_menu(ui);
}
