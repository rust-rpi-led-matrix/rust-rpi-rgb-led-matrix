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
