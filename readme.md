# steps to get it working (Linux)

1. Install SDL2-devel
```sh
sudo dnf install SDL2-devel
```
2. Setup `Cargo.toml`
```toml
[package]
name = "test_sdl"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies.sdl2]
version = "0.35.0"
default-features = false
features = ["image", "ttf", "static-link", "use-vcpkg"]

[package.metadata.vcpkg]
dependencies = ["sdl2", "sdl2-image[libjpeg-turbo,tiff,libwebp]", "sdl2-ttf", "sdl2-gfx", "sdl2-mixer"]
git = "https://github.com/microsoft/vcpkg"
rev = "261c458af6e3eed5d099144aff95d2b5035f656b"

[package.metadata.vcpkg.target]
x86_64-pc-windows-msvc = { triplet = "x64-windows-static-md" }
```

3. Install `vcpkg`
```sh
cargo install cargo-vcpkg
cargo vcpkg build
```

4. Run program
```sh
cargo run
```

Refer to the original repo for further guidance [Rust-SDL2](https://github.com/Rust-SDL2/rust-sdl2)