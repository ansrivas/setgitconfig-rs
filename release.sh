cargo clean
RUSTFLAGS='-C link-args=-s' cargo build --release
