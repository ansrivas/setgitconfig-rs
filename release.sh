cargo clean
# Include rust-openssl in dependencies
# [dependencies]
# openssl = { version = "0.10", features = ["vendored"] }

# CC=/usr/local/bin/musl-gcc RUSTFLAGS='-C link-args=-s' cargo build --release

# alias rust-musl-builder='docker run --rm -it -v "$(pwd)":/home/rust/src ekidd/rust-musl-builder'
# rust-musl-builder cargo build --release

#sudo dnf install -y mingw64-gcc mingw64-freetype mingw64-cairo mingw64-harfbuzz mingw64-pango mingw64-poppler mingw64-gtk3 mingw64-winpthreads-static mingw64-glib2-static gcc boost zip && dnf clean all -y
# dnf install -y mingw64-openssl-static openssl-devel openssl-static
# rustup target list
#rustup target add x86_64-pc-windows-gnu
#rustup target add x86_64-unknown-linux-musl
#PKG_CONFIG_ALLOW_CROSS=1
#PKG_CONFIG_PATH=/usr/x86_64-w64-mingw32/sys-root/mingw/lib/pkgconfig/
#RUSTFLAGS='-C link-args=-s' cargo build --release --target=x86_64-pc-windows-gnu


#USAGE:
#Put this in your bashrc. Everytime a command is executed, this command will run.
#PROMPT_COMMAND="/usr/local/bin/setgitconfig; $PROMPT_COMMAND"
cargo clean
rm -rf build
CC=/usr/bin/musl-gcc RUSTFLAGS='-C link-args=-s' cargo build --release --target=x86_64-unknown-linux-musl
sha256sum target/x86_64-unknown-linux-musl/release/setgitconfig | awk '{print $1}' > setgitconfig.sha256sum

mkdir -p build
upx --lzma --best target/x86_64-unknown-linux-musl/release/setgitconfig

mv target/x86_64-unknown-linux-musl/release/setgitconfig build/setgitconfig-linux-musl
mv setgitconfig.sha256sum build
