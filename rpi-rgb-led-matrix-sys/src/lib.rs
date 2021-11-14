use libc::{c_char, c_int};

pub enum LedMatrix {}
pub enum LedCanvas {}
pub enum LedFont {}

/// The Rust representation of LedMatrixOptions, which contains parameters to specify your hardware setup.
#[derive(Debug)]
#[repr(C)]
pub struct LedMatrixOptions {
    pub hardware_mapping: *mut c_char,
    pub rows: c_int,
    pub cols: c_int,
    pub chain_length: c_int,
    pub parallel: c_int,
    pub pwm_bits: c_int,
    pub pwm_lsb_nanoseconds: c_int,
    pub pwm_dither_bits: c_int,
    pub brightness: c_int,
    pub scan_mode: c_int,
    pub row_address_type: c_int,
    pub multiplexing: c_int,
    pub led_rgb_sequence: *mut c_char,
    pub pixel_mapper_config: *mut c_char,
    pub panel_type: *mut c_char,
    pub disable_hardware_pulsing: c_char,
    pub show_refresh_rate: c_char,
    pub inverse_colors: c_char,
    pub limit_refresh_rate_hz: c_int,
}

/// The Rust representation of LedRuntimeOptions, which contains parameters to specify how the library behaves at runtime.
#[derive(Debug)]
#[repr(C)]
pub struct LedRuntimeOptions {
    pub gpio_slowdown: c_int,
    pub daemon: c_int,
    pub drop_privileges: c_int,
    pub do_gpio_init: bool,
}

#[link(name = "rgbmatrix")]
extern "C" {
    // unused C functions omitted
    pub fn led_matrix_create_from_options_and_rt_options(
        opts: *mut LedMatrixOptions,
        rt_opts: *mut LedRuntimeOptions,
    ) -> *mut LedMatrix;
    pub fn led_matrix_delete(matrix: *mut LedMatrix);
    pub fn led_matrix_get_canvas(matrix: *mut LedMatrix) -> *mut LedCanvas;
    pub fn led_canvas_get_size(canvas: *const LedCanvas, width: *mut c_int, height: *mut c_int);
    pub fn led_canvas_set_pixel(canvas: *mut LedCanvas, x: c_int, y: c_int, r: u8, g: u8, b: u8);
    pub fn led_canvas_clear(canvas: *mut LedCanvas);
    pub fn led_canvas_fill(canvas: *mut LedCanvas, r: u8, g: u8, b: u8);
    pub fn led_matrix_create_offscreen_canvas(matrix: *mut LedMatrix) -> *mut LedCanvas;
    pub fn led_matrix_swap_on_vsync(
        matrix: *mut LedMatrix,
        canvas: *mut LedCanvas,
    ) -> *mut LedCanvas;
    pub fn load_font(bdf_font_file: *const c_char) -> *mut LedFont;
    pub fn delete_font(font: *mut LedFont);
    pub fn draw_text(
        canvas: *mut LedCanvas,
        font: *const LedFont,
        x: c_int,
        y: c_int,
        r: u8,
        g: u8,
        b: u8,
        utf8_text: *const c_char,
        kerning_offset: c_int,
    ) -> c_int;
    pub fn vertical_draw_text(
        canvas: *mut LedCanvas,
        font: *const LedFont,
        x: c_int,
        y: c_int,
        r: u8,
        g: u8,
        b: u8,
        utf8_text: *const c_char,
        kerning_offset: c_int,
    ) -> c_int;
    pub fn draw_circle(
        canvas: *mut LedCanvas,
        x: c_int,
        y: c_int,
        radius: c_int,
        r: u8,
        g: u8,
        b: u8,
    );
    pub fn draw_line(
        canvas: *mut LedCanvas,
        x0: c_int,
        y0: c_int,
        x1: c_int,
        y1: c_int,
        r: u8,
        g: u8,
        b: u8,
    );
}
