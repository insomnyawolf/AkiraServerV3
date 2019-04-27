## Add rust Targets
`rustup target add (target string)`

## Compile commans
### Windows
[GNU vs MSVC](https://www.reddit.com/r/rust/comments/a63dlt/difference_between_the_gnu_and_msvc_toolchains/)
#### GNU
`cargo rustc --release --target=i686-pc-windows-gnu -- -C linker=i686-w64-mingw32-gcc`
`cargo rustc --release --target=x86_64-pc-windows-gnu -- -C linker=x86_64-w64-mingw32-gcc`
#### MSVC
`cargo rustc --release --target=i686-pc-windows-msvc`
`cargo rustc --release --target=x86_64-pc-windows-msvc`
### Linux
`cargo rustc --release --target=i686-unknown-linux-gnu`
`cargo rustc --release --target=x86_64-unknown-linux-gnu `
### Android
`cargo rustc --release --target=armv7-linux-androideabi`

## Targets

[Rust Platform Support](https://forge.rust-lang.org/platform-support.html)  

All Targets:
```
aarch64-apple-ios
aarch64-fuchsia
aarch64-linux-android
aarch64-pc-windows-msvc
aarch64-unknown-cloudabi
aarch64-unknown-linux-gnu
aarch64-unknown-linux-musl
arm-linux-androideabi
arm-unknown-linux-gnueabi
arm-unknown-linux-gnueabihf
arm-unknown-linux-musleabi
arm-unknown-linux-musleabihf
armebv7r-none-eabi
armebv7r-none-eabihf
armv5te-unknown-linux-gnueabi
armv5te-unknown-linux-musleabi
armv7-apple-ios
armv7-linux-androideabi
armv7-unknown-linux-gnueabihf
armv7-unknown-linux-musleabihf
armv7r-none-eabi
armv7r-none-eabihf
armv7s-apple-ios
asmjs-unknown-emscripten
i386-apple-ios
i586-pc-windows-msvc
i586-unknown-linux-gnu
i586-unknown-linux-musl
i686-apple-darwin
i686-linux-android
i686-pc-windows-gnu
i686-pc-windows-msvc
i686-unknown-freebsd
i686-unknown-linux-gnu
i686-unknown-linux-musl
mips-unknown-linux-gnu
mips-unknown-linux-musl
mips64-unknown-linux-gnuabi64
mips64el-unknown-linux-gnuabi64
mipsel-unknown-linux-gnu
mipsel-unknown-linux-musl
nvptx64-nvidia-cuda
powerpc-unknown-linux-gnu
powerpc64-unknown-linux-gnu
powerpc64le-unknown-linux-gnu
riscv32imac-unknown-none-elf
riscv32imc-unknown-none-elf
riscv64gc-unknown-none-elf
riscv64imac-unknown-none-elf
s390x-unknown-linux-gnu
sparc64-unknown-linux-gnu
sparcv9-sun-solaris
thumbv6m-none-eabi
thumbv7em-none-eabi
thumbv7em-none-eabihf
thumbv7m-none-eabi
thumbv7neon-linux-androideabi
thumbv7neon-unknown-linux-gnueabihf
thumbv8m.main-none-eabi
wasm32-unknown-emscripten
wasm32-unknown-unknown
x86_64-apple-darwin
x86_64-apple-ios
x86_64-fortanix-unknown-sgx
x86_64-fuchsia
x86_64-linux-android
x86_64-pc-windows-gnu
x86_64-pc-windows-msvc (default)
x86_64-rumprun-netbsd
x86_64-sun-solaris
x86_64-unknown-cloudabi
x86_64-unknown-freebsd
x86_64-unknown-linux-gnu
x86_64-unknown-linux-gnux32
x86_64-unknown-linux-musl
x86_64-unknown-netbsd
x86_64-unknown-redox
```