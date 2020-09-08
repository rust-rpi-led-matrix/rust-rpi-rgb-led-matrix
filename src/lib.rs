extern crate libc;

#[cfg(feature = "args")]
pub mod args;
mod c;
mod led_color;
mod options;

#[cfg(feature = "embeddedgraphics")]
use embedded_graphics::{drawable::Pixel, geometry::Size, pixelcolor::PixelColor, DrawTarget};
use libc::c_int;
use std::ffi::CString;
use std::path::Path;
use std::marker::PhantomData;

pub use c::{LedMatrixOptions, LedRuntimeOptions};
pub use led_color::LedColor;

pub struct LedCanvas<C> {
    handle: *mut c::LedCanvas,
    phantom: PhantomData<C>,
}

pub struct LedMatrix {
    handle: *mut c::LedMatrix,
    _options: LedMatrixOptions,
}

pub struct LedFont {
    handle: *mut c::LedFont,
}

impl LedMatrix
{
    pub fn new(
        options: Option<LedMatrixOptions>,
        rt_options: Option<LedRuntimeOptions>,
    ) -> Result<LedMatrix, &'static str> {
        let mut options = options.unwrap_or_default();
        let mut rt_options = rt_options.unwrap_or_default();

        let handle = unsafe {
            c::led_matrix_create_from_options_and_rt_options(
                &mut options as *mut LedMatrixOptions,
                &mut rt_options as *mut LedRuntimeOptions,
            )
        };

        if handle.is_null() {
            Err("Couldn't create LedMatrix")
        } else {
            Ok(LedMatrix {
                handle,
                _options: options,
            })
        }
    }

    pub fn canvas<C>(&self) -> LedCanvas<C> {
        let handle = unsafe { c::led_matrix_get_canvas(self.handle) };

        LedCanvas { 
            handle: handle, 
            phantom: PhantomData 
        }
    }

    pub fn offscreen_canvas<C>(&self) -> LedCanvas<C> {
        let handle = unsafe { c::led_matrix_create_offscreen_canvas(self.handle) };

        LedCanvas { 
            handle: handle, 
            phantom: PhantomData 
        }
    }

    pub fn swap<C>(&self, canvas: LedCanvas<C>) -> LedCanvas<C> {
        let handle = unsafe { c::led_matrix_swap_on_vsync(self.handle, canvas.handle) };

        LedCanvas { 
            handle: handle, 
            phantom: PhantomData 
        }
    }
}

impl Drop for LedMatrix {
    fn drop(&mut self) {
        unsafe {
            c::led_matrix_delete(self.handle);
        }
    }
}

impl LedFont {
    pub fn new(bdf_file: &Path) -> Result<LedFont, &'static str> {
        let string = match bdf_file.to_str() {
            Some(s) => s,
            None => return Err("Couldn't convert path to str"),
        };
        let cstring = CString::new(string).unwrap();

        let handle = unsafe { c::load_font(cstring.as_ptr()) };

        if handle.is_null() {
            Err("Couldn't load font")
        } else {
            Ok(LedFont { handle })
        }
    }
}

impl Drop for LedFont {
    fn drop(&mut self) {
        unsafe { c::delete_font(self.handle) }
    }
}

impl<C> LedCanvas<C> {
    pub fn canvas_size(&self) -> (i32, i32) {
        let (mut width, mut height): (c_int, c_int) = (0, 0);
        unsafe {
            c::led_canvas_get_size(
                self.handle,
                &mut width as *mut c_int,
                &mut height as *mut c_int,
            );
        }
        (width as i32, height as i32)
    }

    pub fn set(&mut self, x: i32, y: i32, color: &LedColor) {
        unsafe {
            c::led_canvas_set_pixel(
                self.handle,
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
            c::led_canvas_clear(self.handle);
        }
    }

    pub fn fill(&mut self, color: &LedColor) {
        unsafe {
            c::led_canvas_fill(self.handle, color.red, color.green, color.blue);
        }
    }

    pub fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: &LedColor) {
        unsafe {
            c::draw_line(
                self.handle,
                x0,
                y0,
                x1,
                y1,
                color.red,
                color.green,
                color.blue,
            );
        }
    }

    pub fn draw_circle(&mut self, x: i32, y: i32, radius: u32, color: &LedColor) {
        unsafe {
            c::draw_circle(
                self.handle,
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
                c::vertical_draw_text(
                    self.handle,
                    font.handle,
                    x as c_int,
                    y as c_int,
                    color.red,
                    color.green,
                    color.blue,
                    ctext.as_ptr(),
                    kerning_offset as c_int,
                ) as i32
            } else {
                c::draw_text(
                    self.handle,
                    font.handle,
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

#[cfg(feature = "embeddedgraphics")]
impl<C> DrawTarget<C> for LedCanvas<C>
where
    C: Into<LedColor> + PixelColor,
{
    type Error = core::convert::Infallible;

    fn draw_pixel(&mut self, item: Pixel<C>) -> Result<(), Self::Error> {
        let Pixel(point, color) = item;
        self.set(point.x, point.y, &color.into());
        Ok(())
    }

    fn size(&self) -> Size {
        let size = self.canvas_size();
        Size::new(size.0 as u32, size.1 as u32)
    }

    fn clear(&mut self, color: C) -> Result<(), Self::Error> {
        self.fill(&color.into());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;
    use std::{thread, time};

    fn led_matrix() -> LedMatrix {
        let mut options = LedMatrixOptions::new();
        let mut rt_options = LedRuntimeOptions::new();
        options.set_hardware_mapping("adafruit-hat-pwm");
        options.set_chain_length(2);
        options.set_hardware_pulsing(false);
        options.set_refresh_rate(true);
        options.set_brightness(10).unwrap();
        rt_options.set_gpio_slowdown(2);
        LedMatrix::new(Some(options), Some(rt_options)).unwrap()
    }

    #[test]
    fn matrix_create() {
        let _matrix = led_matrix();
    }

    #[test]
    fn size() {
        let matrix = led_matrix();
        let canvas = matrix.canvas();
        assert_eq!(canvas.canvas_size(), (64, 32));
    }

    #[test]
    fn draw_line() {
        let matrix = led_matrix();
        let mut canvas = matrix.canvas();
        let (width, height) = canvas.canvas_size();
        let mut color = LedColor {
            red: 127,
            green: 0,
            blue: 0,
        };

        canvas.clear();
        for x in 0..width {
            color.blue = 255 - 3 * x as u8;
            canvas.draw_line(x, 0, width - 1 - x, height - 1, &color);
            thread::sleep(time::Duration::new(0, 10000000));
        }
    }

    #[test]
    fn draw_circle() {
        let matrix = led_matrix();
        let mut canvas = matrix.canvas();
        let (width, height) = canvas.canvas_size();
        let mut color = LedColor {
            red: 127,
            green: 0,
            blue: 0,
        };
        let (x, y) = (width / 2, height / 2);

        canvas.clear();
        for r in 0..(width / 2) {
            color.green = color.red;
            color.red = color.blue;
            color.blue = (r * r) as u8;
            canvas.draw_circle(x, y, r as u32, &color);
            thread::sleep(time::Duration::new(0, 100000000));
        }
    }

    #[test]
    fn draw_text() {
        let matrix = led_matrix();
        let mut canvas = matrix.canvas();
        let font = LedFont::new(Path::new("/usr/share/fonts/misc/10x20.bdf")).unwrap();
        let color = LedColor {
            red: 0,
            green: 127,
            blue: 0,
        };
        let (width, height) = canvas.canvas_size();
        let text_width = 10 * 9;
        let baseline = height / 2;

        canvas = matrix.offscreen_canvas();
        for x in 0..(2 * width) {
            let x = x % (10 * 9);
            canvas.clear();
            canvas.draw_text(&font, "Mah boy! ", x, baseline, &color, 0, false);
            canvas.draw_text(
                &font,
                "Mah boy! ",
                x - text_width,
                baseline,
                &color,
                0,
                false,
            );
            canvas.draw_text(
                &font,
                "Mah boy! ",
                x + text_width,
                baseline,
                &color,
                0,
                false,
            );
            canvas = matrix.swap(canvas);
            thread::sleep(time::Duration::new(0, 50000000));
        }
    }

    #[test]
    fn gradient() {
        let matrix = led_matrix();
        let mut canvas = matrix.canvas();
        let mut color = LedColor {
            red: 0,
            green: 0,
            blue: 0,
        };
        let period = 400;
        let duration = time::Duration::new(3, 0);
        let sleep_duration = duration / period;

        for t in 0..period {
            let t = t as f64;
            color.red = ((PI * t / period as f64).sin() * 255.) as u8;
            color.green = ((2. * PI * t / period as f64).cos() * 255.) as u8;
            color.blue = ((3. * PI * t / period as f64 + 0.3).cos() * 255.) as u8;
            canvas.fill(&color);
            thread::sleep(sleep_duration);
        }
    }

    #[test]
    fn canvas_swap() {
        let matrix = led_matrix();
        let mut canvas = matrix.canvas();
        let mut color = LedColor {
            red: 127,
            green: 127,
            blue: 0,
        };

        canvas.fill(&color);
        canvas = matrix.offscreen_canvas();
        color.blue = 127;
        canvas.fill(&color);
        thread::sleep(time::Duration::new(0, 500000000));
        canvas = matrix.swap(canvas);
        color.red = 0;
        canvas.fill(&color);
        thread::sleep(time::Duration::new(0, 500000000));
        matrix.swap(canvas);
        thread::sleep(time::Duration::new(0, 500000000));
    }
}
