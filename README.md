# Coomer - Modified Woomer (Boomer but for wayland)

## Original: https://github.com/coffeeispower/woomer
## Changes:
1. Pressing any key except ```LEFT_SHIFT || LEFT_CTRL || KEY_R``` will close the program
2. Sane default spotlight size
3. Little bit faster zoom (1.3x, doesnt really change much but perfect for me)

## Installation:
#### Dependencies:
- rust
- clang (for compiling raylib)
- cmake
- libclang (for bindgen)
- raylib & raylib-devel
- libglfw3
- libwayland-client
- pkg-config
- mesa-libgbm (for this implementation only, also idk abt other gpu)

then just do ```cargo build --release```, your compiled binary will be in target/release  
or download pre-compiled binary from release (only tested on Fedora linux)
