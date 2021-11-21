//! Rust bindings for the C++ library [rpi-rgb-led-matrix](https://github.com/hzeller/rpi-rgb-led-matrix).
//!
//! # Example Usage
//!
//! ```
//! use rpi_led_matrix::{LedMatrix, LedColor};
//! let matrix = LedMatrix::new(None, None).unwrap();
//! let mut canvas = matrix.offscreen_canvas();
//! for red in 0..255 {
//!     for green in 0..255 {
//!         for blue in 0..255 {
//!             canvas.fill(&LedColor { red, green, blue });
//!             canvas = matrix.swap(canvas);
//!         }
//!     }
//! }
//! ```
extern crate libc;

#[cfg(feature = "args")]
#[deny(missing_docs)]
pub mod args;
#[deny(missing_docs)]
mod canvas;
#[deny(missing_docs)]
mod font;
#[deny(missing_docs)]
mod led_color;
#[deny(missing_docs)]
mod matrix;
#[deny(missing_docs)]
mod options;

// import all of the C FFI functions
pub(crate) use rpi_led_matrix_sys as ffi;

// re-export objects to the root
#[doc(inline)]
pub use canvas::LedCanvas;
#[doc(inline)]
pub use font::LedFont;
#[doc(inline)]
pub use led_color::LedColor;
#[doc(inline)]
pub use matrix::LedMatrix;
#[doc(inline)]
pub use options::{LedMatrixOptions, LedRuntimeOptions};
