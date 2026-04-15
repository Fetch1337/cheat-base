use windows::Win32::Foundation::*;

use hudhook::imgui;
use hudhook::*;

use crate::gfx::menu;
use crate::utilities::input;

pub struct Overlay;
impl ImguiRenderLoop for Overlay {
    fn initialize(&mut self, ctx: &mut imgui::Context, _rc: &mut dyn RenderContext) {
        menu::theme::apply(ctx);
    }

    fn render(&mut self, ui: &mut imgui::Ui) {
        menu::on_render(ui);
    }

    fn message_filter(&self, _io: &imgui::Io) -> MessageFilter {
        if menu::menu_visible() {
            MessageFilter::InputMouse
        } else {
            MessageFilter::empty()
        }
    }

    fn after_wnd_proc(&self, _hwnd: HWND, umsg: u32, wparam: WPARAM, _lparam: LPARAM) {
        input::on_wnd_proc(umsg, wparam);
    }
}

pub fn init(hmodule: HINSTANCE) {
    let builder = hudhook::Hudhook::builder().with_hmodule(hmodule);

    if let Err(e) = builder
        .with::<hudhook::hooks::dx11::ImguiDx11Hooks>(Overlay)
        .build()
        .apply()
    {
        tracing::error!("dx11 hook failed: {:?}. ensure the game uses DirectX 11", e);
        eject();
    }
}
