use crate::led_color::LedColor;
use libc::c_int;
use std::ffi::CString;

// import all of the C FFI functions
pub(crate) use rpi_rgb_led_matrix_sys::*;

// make newtypes here so we can expand on them

#[repr(transparent)]
pub struct LedCanvas(pub(crate) rpi_rgb_led_matrix_sys::types::LedCanvas);

#[repr(transparent)]
pub struct LedFont(pub(crate) rpi_rgb_led_matrix_sys::types::LedFont);

#[repr(transparent)]
pub struct LedMatrix(pub(crate) rpi_rgb_led_matrix_sys::types::LedMatrix);

#[repr(transparent)]
pub struct LedMatrixOptions(pub(crate) rpi_rgb_led_matrix_sys::types::LedMatrixOptions);

#[repr(transparent)]
pub struct LedRuntimeOptions(pub(crate) rpi_rgb_led_matrix_sys::types::LedRuntimeOptions);

#[allow(dead_code)]
impl LedCanvas {
    pub fn canvas_size(&self) -> (i32, i32) {
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

    #[allow(clippy::too_many_arguments)]
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
