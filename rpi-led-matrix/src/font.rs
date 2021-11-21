use std::ffi::CString;
use std::path::Path;

use crate::ffi;

/// The Rust handle for LedFont.
pub struct LedFont {
    pub(crate) handle: *mut ffi::CLedFont,
}

impl LedFont {
    /// Creates a new `LedFont` instance with the given bdf filepath, if it exists.
    pub fn new(bdf_file: &Path) -> Result<LedFont, &'static str> {
        let string = match bdf_file.to_str() {
            Some(s) => s,
            None => return Err("Couldn't convert path to str"),
        };
        let cstring = CString::new(string).unwrap();

        let handle = unsafe { ffi::load_font(cstring.as_ptr()) };

        if handle.is_null() {
            Err("Couldn't load font")
        } else {
            Ok(LedFont { handle })
        }
    }
}

impl Drop for LedFont {
    fn drop(&mut self) {
        unsafe { ffi::delete_font(self.handle) }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{LedColor, LedMatrix};
    use std::{thread, time};

    #[test]
    fn draw_text() {
        let matrix = LedMatrix::new(None, None).unwrap();
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
}
