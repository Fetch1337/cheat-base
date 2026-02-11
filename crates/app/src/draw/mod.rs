use hudhook::imgui;
use hudhook::*;

use crate::menu;
use core::input;

pub struct Overlay;

impl ImguiRenderLoop for Overlay {
    fn render(&mut self, ui: &mut imgui::Ui) {
        input::on_render(ui);
        menu::on_render(ui);
    }
}
