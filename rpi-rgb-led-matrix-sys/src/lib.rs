pub mod types;

use libc::{c_char, c_int};

#[link(name = "rgbmatrix")]
extern "C" {
    // unused C functions omitted
    pub fn led_matrix_create_from_options_and_rt_options(
        opts: *mut types::LedMatrixOptions,
        rt_opts: *mut types::LedRuntimeOptions,
    ) -> *mut types::LedMatrix;
    pub fn led_matrix_delete(matrix: *mut types::LedMatrix);
    pub fn led_matrix_get_canvas(matrix: *mut types::LedMatrix) -> *mut types::LedCanvas;
    pub fn led_canvas_get_size(
        canvas: *const types::LedCanvas,
        width: *mut c_int,
        height: *mut c_int,
    );
    pub fn led_canvas_set_pixel(
        canvas: *mut types::LedCanvas,
        x: c_int,
        y: c_int,
        r: u8,
        g: u8,
        b: u8,
    );
    pub fn led_canvas_clear(canvas: *mut types::LedCanvas);
    pub fn led_canvas_fill(canvas: *mut types::LedCanvas, r: u8, g: u8, b: u8);
    pub fn led_matrix_create_offscreen_canvas(
        matrix: *mut types::LedMatrix,
    ) -> *mut types::LedCanvas;
    pub fn led_matrix_swap_on_vsync(
        matrix: *mut types::LedMatrix,
        canvas: *mut types::LedCanvas,
    ) -> *mut types::LedCanvas;
    pub fn load_font(bdf_font_file: *const c_char) -> *mut types::LedFont;
    pub fn delete_font(font: *mut types::LedFont);
    pub fn draw_text(
        canvas: *mut types::LedCanvas,
        font: *const types::LedFont,
        x: c_int,
        y: c_int,
        r: u8,
        g: u8,
        b: u8,
        utf8_text: *const c_char,
        kerning_offset: c_int,
    ) -> c_int;
    pub fn vertical_draw_text(
        canvas: *mut types::LedCanvas,
        font: *const types::LedFont,
        x: c_int,
        y: c_int,
        r: u8,
        g: u8,
        b: u8,
        utf8_text: *const c_char,
        kerning_offset: c_int,
    ) -> c_int;
    pub fn draw_circle(
        canvas: *mut types::LedCanvas,
        x: c_int,
        y: c_int,
        radius: c_int,
        r: u8,
        g: u8,
        b: u8,
    );
    pub fn draw_line(
        canvas: *mut types::LedCanvas,
        x0: c_int,
        y0: c_int,
        x1: c_int,
        y1: c_int,
        r: u8,
        g: u8,
        b: u8,
    );
}
