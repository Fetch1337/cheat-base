# Cheat Base

A universal base for creating game cheats in Rust with support for multiple rendering APIs and convenient abstractions.

## Project Structure

The project is divided into several crates:

*   **`crates/core`**: Core logic, rendering abstractions, and utilities. Contains ImGui wrappers and helper modules.
*   **`crates/app`**: Entry point (DLL), hooks, menu, and initialization. This is where game-specific setup and renderer selection happen.

## Features

### 🎨 Multi-renderer Support
The project uses [hudhook](https://github.com/veeenu/hudhook) to support a wide range of graphic APIs:
*   DirectX 9, 10, 11, 12
*   OpenGL 3
*   Vulkan

Choose your renderer in `crates/app/src/entry_point.rs`.

### 🛠 Utilities
*   **Rendering**: Easy-to-use `DrawContext` API in `core::render` for drawing primitives (lines, rectangles, text, circles) via ImGui.
*   **Memory**: Integration with [libmem](https://github.com/rdbo/libmem-rs) for memory reading/writing, signature scanning, and module handling.
*   **Configuration**: JSON-based settings system using `serde`.
*   **Logging**: Pre-configured logger for easy debugging.
*   **Security**: String obfuscation using `obfuse`.

## Building

To build the project in Release mode (DLL):

```bash
cargo build --release
```

The `app.dll` file will be located in `target/release/`.

## Usage

1.  Build the DLL.
2.  Use any injector that supports LoadLibrary or Manual Map to inject `app.dll` into the target game process.
3.  By default, the menu opens with the **Insert** key (configurable in `lib.rs` / `hooks`).

## Customization

1.  Open `crates/app/src/entry_point.rs`.
2.  In the `main_thread` function, set up hooks for your desired graphic API (DX11 by default):
    ```rust
    Hudhook::builder()
        .with::<hudhook::hooks::dx11::ImguiDx11Hooks>(draw::Overlay) // Replace with your hook
        .with_hmodule(h_module)
        .build()
        .apply()
    ```
3.  Implement your menu logic in `crates/app/src/menu`.

## Credits

*   [hudhook](https://github.com/veeenu/hudhook) - Graphic API hooks.
*   [libmem](https://github.com/rdbo/libmem-rs) - Memory manipulation.
*   [imgui-rs](https://github.com/imgui-rs/imgui-rs) - Rust bindings for Dear ImGui.
