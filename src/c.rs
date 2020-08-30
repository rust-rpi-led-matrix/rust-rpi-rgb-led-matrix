use libc::{c_char, c_int};
use std::ffi::CString;

pub enum LedMatrix {}
pub enum LedCanvas {}
pub enum LedFont {}

#[derive(Clone, Copy)]
pub struct LedColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

type LedMatrixOptionsResult = Result<(), &'static str>;

#[repr(C, packed)]
pub struct LedMatrixOptions {
    hardware_mapping: *mut c_char,
    rows: c_int,
    cols: c_int,
    chain_length: c_int,
    parallel: c_int,
    pwm_bits: c_int,
    pwm_lsb_nanoseconds: c_int,
    pwm_dither_bits: c_int,
    brightness: c_int,
    scan_mode: c_int,
    row_address_type: c_int,
    multiplexing: c_int,
    led_rgb_sequence: *mut c_char,
    pixel_mapper_config: *mut c_char,
    panel_type: *mut c_char,
    disable_hardware_pulsing: c_char,
    show_refresh_rate: c_char,
    inverse_colors: c_char,
    limit_refresh_rate_hz: c_int,
}

impl LedMatrixOptions {
    pub fn new() -> LedMatrixOptions {
        LedMatrixOptions {
            hardware_mapping: CString::new("regular").unwrap().into_raw(),
            rows: 32,
            cols: 32,
            chain_length: 1,
            parallel: 1,
            pwm_bits: 11,
            pwm_lsb_nanoseconds: 1000,
            pwm_dither_bits: 1,
            brightness: 100,
            scan_mode: 0,
            row_address_type: 0,
            multiplexing: 0,
            led_rgb_sequence: CString::new("RGB").unwrap().into_raw(),
            pixel_mapper_config: CString::new("").unwrap().into_raw(),
            panel_type: CString::new("").unwrap().into_raw(),
            disable_hardware_pulsing: 1,
            show_refresh_rate: 1,
            inverse_colors: 0,
            limit_refresh_rate_hz: 0,
        }
    }

    pub fn set_hardware_mapping(&mut self, mapping: &str) {
        unsafe {
            let _ = CString::from_raw(self.hardware_mapping);
            self.hardware_mapping = CString::new(mapping).unwrap().into_raw();
        }
    }

    pub fn set_rows(&mut self, rows: u32) {
        self.rows = rows as c_int;
    }

    pub fn set_cols(&mut self, cols: u32) {
        self.cols = cols as c_int;
    }

    pub fn set_chain_length(&mut self, chain_length: u32) {
        self.chain_length = chain_length as c_int;
    }

    pub fn set_parallel(&mut self, parallel: bool) {
        if parallel {
            self.parallel = 1;
        } else {
            self.parallel = 0;
        }
    }

    pub fn set_pwm_bits(&mut self, pwm_bits: u8) -> LedMatrixOptionsResult {
        if pwm_bits > 11 {
            Err("Pwm bits can only have value between 0 and 11 inclusive")
        } else {
            self.pwm_bits = pwm_bits as c_int;
            Ok(())
        }
    }

    pub fn set_pwm_lsb_nanoseconds(&mut self, pwm_lsb_nanoseconds: u32) {
        self.pwm_lsb_nanoseconds = pwm_lsb_nanoseconds as c_int;
    }

    pub fn set_brightness(&mut self, brightness: u8) -> LedMatrixOptionsResult {
        if brightness > 100 || brightness < 1 {
            Err("Brigthness can only have value between 1 and 100 inclusive")
        } else {
            self.brightness = brightness as c_int;
            Ok(())
        }
    }

    pub fn set_scan_mode(&mut self, scan_mode: bool) {
        if scan_mode {
            self.scan_mode = 1 as c_int;
        } else {
            self.scan_mode = 0 as c_int;
        }
    }

    pub fn set_led_rgb_sequence(&mut self, sequence: &str) {
        unsafe {
            let _ = CString::from_raw(self.led_rgb_sequence);
            self.led_rgb_sequence = CString::new(sequence).unwrap().into_raw();
        }
    }

    pub fn set_pixel_mapper_config(&mut self, mapper: &str) {
        unsafe {
            let _ = CString::from_raw(self.pixel_mapper_config);
            self.pixel_mapper_config = CString::new(mapper).unwrap().into_raw();
        }
    }

    pub fn set_panel_type(&mut self, panel_type: &str) {
        unsafe {
            let _ = CString::from_raw(self.panel_type);
            self.panel_type = CString::new(panel_type).unwrap().into_raw();
        }
    }

    pub fn set_hardware_pulsing(&mut self, enable: bool) {
        if enable {
            self.disable_hardware_pulsing = 0;
        } else {
            self.disable_hardware_pulsing = 1;
        }
    }

    pub fn set_refresh_rate(&mut self, enable: bool) {
        if enable {
            self.show_refresh_rate = 1;
        } else {
            self.show_refresh_rate = 0;
        }
    }

    pub fn set_inverse_colors(&mut self, enable: bool) {
        if enable {
            self.inverse_colors = 1;
        } else {
            self.inverse_colors = 0;
        }
    }
}

impl Drop for LedMatrixOptions {
    fn drop(&mut self) {
        unsafe {
            let _ = CString::from_raw(self.hardware_mapping);
            let _ = CString::from_raw(self.led_rgb_sequence);
        }
    }
}

#[allow(dead_code)]
impl LedCanvas {
    pub fn size(&self) -> (i32, i32) {
        let (mut width, mut height): (c_int, c_int) = (0, 0);
        unsafe {
            led_canvas_get_size(self, &mut width as *mut c_int, &mut height as *mut c_int);
        }
        (width as i32, height as i32)
    }

    pub fn set(&mut self, x: i32, y: i32, color: &LedColor) {
        unsafe {
            led_canvas_set_pixel(
                self,
                x as c_int,
                y as c_int,
                color.red,
                color.green,
                color.blue,
            )
        }
    }

    pub fn clear(&mut self) {
        unsafe {
            led_canvas_clear(self);
        }
    }

    pub fn fill(&mut self, color: &LedColor) {
        unsafe {
            led_canvas_fill(self, color.red as u8, color.green as u8, color.blue as u8);
        }
    }

    pub fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: &LedColor) {
        unsafe {
            draw_line(
                self,
                x0 as c_int,
                y0 as c_int,
                x1 as c_int,
                y1 as c_int,
                color.red as u8,
                color.green as u8,
                color.blue as u8,
            );
        }
    }

    pub fn draw_circle(&mut self, x: i32, y: i32, radius: u32, color: &LedColor) {
        unsafe {
            draw_circle(
                self,
                x as c_int,
                y as c_int,
                radius as c_int,
                color.red,
                color.green,
                color.blue,
            );
        }
    }

    pub fn draw_text(
        &mut self,
        font: &LedFont,
        text: &str,
        x: i32,
        y: i32,
        color: &LedColor,
        kerning_offset: i32,
        vertical: bool,
    ) -> i32 {
        let ctext = CString::new(text).unwrap();
        unsafe {
            if vertical {
                vertical_draw_text(
                    self,
                    font,
                    x as c_int,
                    y as c_int,
                    color.red,
                    color.green,
                    color.blue,
                    ctext.as_ptr(),
                    kerning_offset as c_int,
                ) as i32
            } else {
                draw_text(
                    self,
                    font,
                    x as c_int,
                    y as c_int,
                    color.red,
                    color.green,
                    color.blue,
                    ctext.as_ptr(),
                    kerning_offset as c_int,
                ) as i32
            }
        }
    }
}

#[link(name = "rgbmatrix")]
extern "C" {
    pub fn led_matrix_create_from_options(
        options: *const LedMatrixOptions,
        argc: *mut c_int,
        argv: *mut *mut *mut c_char,
    ) -> *mut LedMatrix;
    //    pub fn led_matrix_create(
    //        rows: c_int, chained: c_int, parallel: c_int) -> *mut LedMatrix;
    pub fn led_matrix_delete(matrix: *mut LedMatrix);
    //    pub fn led_matrix_print_flags(out: *mut FILE);
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
