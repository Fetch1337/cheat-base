use hudhook::*;
use log::{error, trace};
use windows::Win32::Foundation::HINSTANCE;

mod cfg;
mod input;
mod render;
mod ui;

pub fn initialize(hmodule: HINSTANCE) {
    trace!("setup base render hooks");

    if let Err(e) = Hudhook::builder()
        .with::<hooks::dx11::ImguiDx11Hooks>(render::Overlay {})
        .with_hmodule(hmodule)
        .build()
        .apply()
    {
        error!("couldn't apply hooks: {e:?}");
        eject();
    }
}
