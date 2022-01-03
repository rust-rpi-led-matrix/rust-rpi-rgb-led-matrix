//! rust implemented stubs of the C++ library for testing on non rpi hosts
#![allow(clippy::wildcard_imports)]
#![allow(clippy::missing_const_for_fn)]
use crate::*;
use libc::{c_char, c_int};

#[no_mangle]
extern "C" fn led_matrix_create_from_options_and_rt_options(
    _opts: *mut CLedMatrixOptions,
    _rt_opts: *mut CLedRuntimeOptions,
) -> *mut CLedMatrix {
    std::ptr::null_mut()
}

#[no_mangle]
extern "C" fn led_matrix_delete(_matrix: *mut CLedMatrix) {}

#[no_mangle]
extern "C" fn led_matrix_get_canvas(_matrix: *mut CLedMatrix) -> *mut CLedCanvas {
    std::ptr::null_mut()
}

#[no_mangle]
extern "C" fn led_canvas_get_size(
    _canvas: *const CLedCanvas,
    _width: *mut c_int,
    _height: *mut c_int,
) {
}

#[no_mangle]
extern "C" fn led_canvas_set_pixel(
    _canvas: *mut CLedCanvas,
    _x: c_int,
    _y: c_int,
    _r: u8,
    _g: u8,
    _b: u8,
) {
}

#[no_mangle]
extern "C" fn led_canvas_clear(_canvas: *mut CLedCanvas) {}

#[no_mangle]
extern "C" fn led_canvas_fill(_canvas: *mut CLedCanvas, _r: u8, _g: u8, _b: u8) {}

#[no_mangle]
extern "C" fn led_matrix_create_offscreen_canvas(_matrix: *mut CLedMatrix) -> *mut CLedCanvas {
    std::ptr::null_mut()
}

#[no_mangle]
extern "C" fn led_matrix_swap_on_vsync(
    _matrix: *mut CLedMatrix,
    _canvas: *mut CLedCanvas,
) -> *mut CLedCanvas {
    std::ptr::null_mut()
}

#[no_mangle]
extern "C" fn load_font(_bdf_font_file: *const c_char) -> *mut CLedFont {
    std::ptr::null_mut()
}

#[no_mangle]
extern "C" fn delete_font(_font: *mut CLedFont) {}

#[no_mangle]
extern "C" fn draw_text(
    _canvas: *mut CLedCanvas,
    _font: *const CLedFont,
    _x: c_int,
    _y: c_int,
    _r: u8,
    _g: u8,
    _b: u8,
    _utf8_text: *const c_char,
    _kerning_offset: c_int,
) -> c_int {
    0
}

#[no_mangle]
extern "C" fn vertical_draw_text(
    _canvas: *mut CLedCanvas,
    _font: *const CLedFont,
    _x: c_int,
    _y: c_int,
    _r: u8,
    _g: u8,
    _b: u8,
    _utf8_text: *const c_char,
    _kerning_offset: c_int,
) -> c_int {
    0
}

#[no_mangle]
extern "C" fn draw_circle(
    _canvas: *mut CLedCanvas,
    _x: c_int,
    _y: c_int,
    _radius: c_int,
    _r: u8,
    _g: u8,
    _b: u8,
) {
}

#[no_mangle]
extern "C" fn draw_line(
    _canvas: *mut CLedCanvas,
    _x0: c_int,
    _y0: c_int,
    _x1: c_int,
    _y1: c_int,
    _r: u8,
    _g: u8,
    _b: u8,
) {
}
