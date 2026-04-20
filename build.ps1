$env:RUSTFLAGS = "-Zlocation-detail=none"
cargo build --release
Remove-Item Env:RUSTFLAGS