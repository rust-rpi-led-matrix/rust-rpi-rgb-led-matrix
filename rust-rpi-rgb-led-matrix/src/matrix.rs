use crate::ffi;
use crate::{LedCanvas, LedColor, LedMatrixOptions, LedRuntimeOptions};
#[cfg(feature = "embeddedgraphics")]
use embedded_graphics::{drawable::Pixel, geometry::Size, pixelcolor::PixelColor, DrawTarget};

/// The Rust handle for the RGB matrix.
///
/// ```
/// use rpi_led_matrix::{LedMatrix, LedColor};
/// let matrix = LedMatrix::new(None, None).unwrap();
/// ```
pub struct LedMatrix {
    handle: *mut ffi::CLedMatrix,
    _options: LedMatrixOptions,
}

impl LedMatrix {
    /// Creates the rust handle for the RGB matrix, given the optional options.
    ///
    /// ```
    /// use rpi_led_matrix::{LedMatrix, LedColor, LedMatrixOptions};
    /// let mut options = LedMatrixOptions::new();
    /// options.set_hardware_mapping("adafruit-hat-pwm");
    /// let matrix = LedMatrix::new(Some(options), None).unwrap();
    /// ```
    pub fn new(
        options: Option<LedMatrixOptions>,
        rt_options: Option<LedRuntimeOptions>,
    ) -> Result<LedMatrix, &'static str> {
        let mut options = options.unwrap_or_default();
        let mut rt_options = rt_options.unwrap_or_default();

        let handle = unsafe {
            ffi::led_matrix_create_from_options_and_rt_options(
                &mut options.0 as *mut ffi::CLedMatrixOptions,
                &mut rt_options.0 as *mut ffi::CLedRuntimeOptions,
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

    /// Retrieves the on screen canvas.
    pub fn canvas(&self) -> LedCanvas {
        let handle = unsafe { ffi::led_matrix_get_canvas(self.handle) };

        LedCanvas { handle }
    }

    /// Retrieves the offscreen canvas. Used in conjunction with [swap](LedMatrix.swap).
    pub fn offscreen_canvas(&self) -> LedCanvas {
        let handle = unsafe { ffi::led_matrix_create_offscreen_canvas(self.handle) };

        LedCanvas { handle }
    }

    /// Cleanly swaps the canvas on v-sync, returning the off-screen canvas for updating.
    ///
    /// ```
    /// use rpi_led_matrix::{LedMatrix, LedColor};
    /// let matrix = LedMatrix::new(None, None).unwrap();
    /// let mut canvas = matrix.offscreen_canvas();
    /// let mut color = LedColor { red: 0, green: 0, blue: 0 };
    /// while(color.red < 255) {
    ///     canvas.fill(&color);
    ///     canvas = matrix.swap(canvas);
    ///     color.red += 1;
    /// }
    /// ```
    pub fn swap(&self, canvas: LedCanvas) -> LedCanvas {
        let handle = unsafe { ffi::led_matrix_swap_on_vsync(self.handle, canvas.handle) };

        LedCanvas { handle }
    }
}

impl Drop for LedMatrix {
    fn drop(&mut self) {
        unsafe {
            ffi::led_matrix_delete(self.handle);
        }
    }
}

#[cfg(feature = "embeddedgraphics")]
impl<C> DrawTarget<C> for LedCanvas
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
}
