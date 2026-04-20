# Morphey Cheat Base

A universal base for creating game cheats in Rust with multi-renderer support and convenient abstractions.

## Project Structure

```
src/
├── entry_point.rs      # DLL entry point
├── config/             # Configuration system
│   ├── mod.rs          # JSON config load/save (%APPDATA%)
│   └── variables/      # Config variables (menu key, etc.)
├── game/               # Game-specific code
│   ├── sdk/            # Game SDK / offsets
│   ├── hooks/          # Game hooks
│   └── hacks/          # Cheat features
├── gfx/                # Graphics & rendering
│   ├── draw/           # Builder-pattern draw API (rect, circle, line, text, polygon + outlines)
│   ├── menu/           # ImGui menu
│   │   └── theme/      # Custom dark theme with rounded corners
│   └── render/         # Renderer hook setup via hudhook
└── utilities/          # Shared utilities
    ├── input/          # Keybind system (toggle / hold / force_on / force_off)
    ├── math/           # Math helpers
    ├── hook/           # Hook helpers
    └── logging/        # Logging helpers
```

## Features

- **Multi-renderer Support** — DirectX 9, 11, 12, OpenGL 3 via [hudhook](https://github.com/veeenu/hudhook). Switching renderer requires changing **one line** (see below).
- **Draw API** — builder-pattern primitives with outline support (rect, circle, line, text, polygon)
- **ImGui Menu** — custom dark theme with rounded corners
- **Config System** — JSON-based, auto-saved to `%APPDATA%/morphey/`
- **Keybind System** — toggle, hold, force_on, force_off modes
- **String Obfuscation** — compile-time via [obfstr](https://github.com/CasualX/obfstr)
- **Debug Logging** — debug console + `tracing` (only in debug build)

## Switching Renderer

In `src/gfx/render/mod.rs`, change the hook type on a single line:

```rust
// DirectX 9
.with::<hudhook::hooks::dx9::ImguiDx9Hooks>(Overlay)

// DirectX 11 (default)
.with::<hudhook::hooks::dx11::ImguiDx11Hooks>(Overlay)

// DirectX 12
.with::<hudhook::hooks::dx12::ImguiDx12Hooks>(Overlay)

// OpenGL 3
.with::<hudhook::hooks::opengl3::ImguiOpenGl3Hooks>(Overlay)
```

## Building

Step 1:
```sh
cargo rustup install nightly
```

Step 2:
```sh
cargo install cargo-alias-exec
```

Step 3:
```sh
cargo release
```

The DLL will be at `target/release/morphey.dll`.

## Usage

1. Build the DLL
2. Inject into the target game process
3. Press **Insert** to toggle the menu (configurable in config)

## Dependencies

- [hudhook](https://github.com/veeenu/hudhook) — renderer hooks & ImGui rendering
- [obfstr](https://github.com/CasualX/obfstr) — compile-time string obfuscation
- [serde](https://serde.rs) / [serde_json](https://docs.rs/serde_json) — config serialization
- [windows-rs](https://github.com/microsoft/windows-rs) — Win32 API bindings
- [parking_lot](https://github.com/Amanieu/parking_lot) — efficient synchronization primitives

