# Morphey Base

A Windows Rust DLL base with renderer overlay, configuration, input, drawing, and hook utility modules.

## Project Structure

```text
src/
|-- entry_point.rs      # DLL entry point
|-- config/             # JSON configuration system
|   |-- mod.rs          # Config load/save under %APPDATA%
|   `-- variables/      # Config variables
|-- game/               # Game-specific extension points
|   |-- sdk/            # SDK / offsets
|   |-- hooks/          # Hook registration
|   `-- hacks/          # Feature modules
|-- gfx/                # Graphics and rendering
|   |-- draw/           # Builder-pattern draw API
|   |-- menu/           # ImGui menu
|   |   `-- theme/      # ImGui theme
|   `-- render/         # Renderer hook setup via hudhook
`-- utilities/          # Shared utilities
    |-- input/          # Keybind system
    |-- math/           # Math helpers
    |-- hook/           # Hook helpers
    `-- logging/        # Logging helpers
```

## Features

- Multi-renderer overlay support through `hudhook`
- Builder-pattern drawing primitives
- ImGui menu and theme setup
- JSON configuration under `%APPDATA%/morphey/`
- Keybind modes: toggle, hold, force_on, force_off
- Debug logging through `tracing` in debug builds

## Requirements

- Windows
- Rust nightly, as configured in `rust-toolchain.toml`
- MSVC C++ build tools

Install the toolchain:

```sh
rustup toolchain install nightly
```

## Building

```sh
cargo build --release
```

The DLL is emitted at:

```text
target/release/morphey.dll
```

## Quality Checks

```sh
cargo check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

For dependency vulnerability checks, install `cargo-audit` and run it locally:

```sh
cargo install cargo-audit
cargo audit
```

## Switching Renderer

In `src/gfx/render/mod.rs`, change the hook type:

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
