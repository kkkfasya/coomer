# Coomer - Modified Woomer (Boomer but for wayland)

## Original: https://github.com/coffeeispower/woomer
## Changes:
1. Uses flameshot instead of libwayshot so this will work on gnome's wayland compositor (mutter)
2. Pressing any key except ```LEFT_SHIFT || LEFT_CTRL || KEY_R``` will close the program
3. Sane default spotlight size
4. Little bit faster zoom (1.3x, doesnt really change much but perfect for me)

## Installation:
#### Dependencies:
- wayland-client
- cmake
- rust
- pkg-config
- raylib & raylib-devel
- clang (for compiling raylib)
- libclang (for bindgen)

then just do ```cargo build --release```, your compiled binary will be in target/release
