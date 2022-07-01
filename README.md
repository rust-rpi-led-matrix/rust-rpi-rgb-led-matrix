# Control RGB LED displays from Rust

This repository contains rust bindings for the C++ library
[rpi-rgb-led-matrix](https://github.com/hzeller/rpi-rgb-led-matrix),
which enables controlling Hub75 based displays. It includes both raw bindings
to the library in [`rpi-led-matrix-sys`] as well as higher level, safe rust
bindings in [`rpi-led-matrix`].

[`rpi-led-matrix` README](./rpi-led-matrix/README.md)

[`rpi-led-matrix-sys` README](./rpi-led-matrix-sys/README.md)

[`rpi-led-matrix-sys`]: https://docs.rs/rpi-led-matrix-sys
[`rpi-led-matrix`]: https://docs.rs/rpi-led-matrix

There is also a pure Rust rewrite in the crate [`rpi-led-panel`](https://crates.io/crates/rpi_led_panel)!
