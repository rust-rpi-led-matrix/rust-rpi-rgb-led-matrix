# Control RGB LED displays from Rust

This repository contains rust bindings for the C++ library [rpi-rgb-led-matrix](https://github.com/hzeller/rpi-rgb-led-matrix),
which enables controlling Hub75 based displays.
In order to take advantage of the newer APIs, the minimum supported commit of the library is
[55fa32f](https://github.com/hzeller/rpi-rgb-led-matrix/commit/55fa32fc2e02afb254ac834aea93589d5b891a11)
(Sept. 7th, 2020), which exposes all parameters that affect how the matrix runs.

# Usage

The examples have more detailed usage, but here is basic usage to get things rendering on your display.
```rust
use rpi_led_matrix::{LedMatrix, LedColor};

let matrix = LedMatrix::new(None, None).unwrap();
let mut canvas = matrix.offscreen_canvas();
for red in 0..255 {
    for green in 0..255 {
        for blue in 0..255 {
            canvas.fill(&LedColor { red, green, blue });
            canvas = matrix.swap(canvas);
        }
    }
}
```
Note that if you have wirings other than the libraries "default", you will need to construct arguments to the library to specify the layout. See `LedMatrixOptions` for more information.
