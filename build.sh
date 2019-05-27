#!/usr/bin/env bash
# cargo rustc --release --target=i686-pc-windows-gnu -- -C linker=i686-w64-mingw32-gcc
cargo rustc --release --target=x86_64-pc-windows-gnu -- -C linker=x86_64-w64-mingw32-gcc
# cargo rustc --release --target=i686-unknown-linux-gnu
cargo rustc --release --target=x86_64-unknown-linux-gnu
# cargo rustc --release --target=arm-unknown-linux-gnueabihf -- -C linker=arm-linux-androideabi-gcc
# cargo rustc --release --target=armv7-linux-androideabi -- -C linker=arm-linux-androideabi-gcc
# cargo rustc --release --target=arm-linux-androideabi -- -C linker=arm-linux-androideabi-gcc
# cargo rustc --release --target=thumbv7neon-linux-androideabi -- -C linker=arm-linux-androideabi-gcc