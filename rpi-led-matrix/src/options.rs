use libc::c_int;
use std::ffi::CString;

use crate::ffi;

/// Options related to the LED matrix, like rows/cols/HW mapping
#[derive(Debug)]
pub struct LedMatrixOptions(pub(crate) ffi::CLedMatrixOptions);

/// Options related to how the runtime operates, like GPIO slowdown or daemon/sudo privileges
#[derive(Debug)]
pub struct LedRuntimeOptions(pub(crate) ffi::CLedRuntimeOptions);

type LedMatrixOptionsResult = Result<(), &'static str>;

impl LedMatrixOptions {
    /// Creates a new `LedMatrixOptions` struct with the default parameters.
    ///
    /// ```
    /// use rpi_led_matrix::{LedMatrix,LedMatrixOptions};
    /// let mut options = LedMatrixOptions::new();
    /// options.set_hardware_mapping("adafruit-hat-pwm");
    /// let matrix = LedMatrix::new(Some(options), None).unwrap();
    /// ```
    ///
    /// # Panics
    /// If for some reason the conversion from constant literal strings to `CString` fails.
    /// It should never fail but we do `.unwrap()` it.
    #[must_use]
    pub fn new() -> Self {
        Self(ffi::CLedMatrixOptions {
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
        })
    }

    /// Sets the type of GPIO mapping used (e.g., "adafruit-hat-pwm").
    ///
    /// # Panics
    /// If the given `mapping` string fails to convert to a `CString`. This can
    /// occur when there is a null character mid way in the string.
    pub fn set_hardware_mapping(&mut self, mapping: &str) {
        unsafe {
            let _ = CString::from_raw(self.0.hardware_mapping);
            self.0.hardware_mapping = CString::new(mapping)
                .expect("given string failed to convert into a CString")
                .into_raw();
        }
    }

    /// Sets the number of rows on the panels being used. Typically 8, 16, 32 or 64.
    pub fn set_rows(&mut self, rows: u32) {
        self.0.rows = rows as c_int;
    }

    /// Sets the number of columns on the panels being used. Typically 32 or 64.
    pub fn set_cols(&mut self, cols: u32) {
        self.0.cols = cols as c_int;
    }

    /// Sets the number of panels daisy-chained together.
    pub fn set_chain_length(&mut self, chain_length: u32) {
        self.0.chain_length = chain_length as c_int;
    }

    /// Sets the number of parallel chains. Valid range: \[1,3\].
    pub fn set_parallel(&mut self, parallel: u32) {
        self.0.parallel = parallel as c_int;
    }

    /// Sets the number of PWM bits to use. Valid range: \[0,11\].
    ///
    /// # Errors
    /// If the given `pwm_bits` is outside the valid range
    pub fn set_pwm_bits(&mut self, pwm_bits: u8) -> LedMatrixOptionsResult {
        if pwm_bits > 11 {
            Err("Pwm bits can only have value between 0 and 11 inclusive")
        } else {
            self.0.pwm_bits = c_int::from(pwm_bits);
            Ok(())
        }
    }

    /// Sets the number of nanoseconds of delay for our LSB
    pub fn set_pwm_lsb_nanoseconds(&mut self, pwm_lsb_nanoseconds: u32) {
        self.0.pwm_lsb_nanoseconds = pwm_lsb_nanoseconds as c_int;
    }

    /// Sets the panel brightness in percent.
    ///
    /// # Errors
    /// If the given `brightness` is not in the range \[1,100\].
    pub fn set_brightness(&mut self, brightness: u8) -> LedMatrixOptionsResult {
        if (1..=100).contains(&brightness) {
            self.0.brightness = c_int::from(brightness);
            Ok(())
        } else {
            Err("Brightness can only have value between 1 and 100 inclusive")
        }
    }

    /// Sets the scan mode. 0: progressive, 1: interlaced.
    pub fn set_scan_mode(&mut self, scan_mode: u32) {
        self.0.scan_mode = scan_mode as c_int;
    }

    /// Sets the ordering of the LEDs on your panel.
    ///
    /// # Panics
    /// If the given `sequence` string fails to convert to a `CString`. This can
    /// occur when there is a null character mid way in the string.
    pub fn set_led_rgb_sequence(&mut self, sequence: &str) {
        unsafe {
            let _ = CString::from_raw(self.0.led_rgb_sequence);
            self.0.led_rgb_sequence = CString::new(sequence)
                .expect("given string failed to convert into a CString")
                .into_raw();
        }
    }

    /// Semicolon-separated list of pixel-mappers to arrange pixels (e.g. "U-mapper;Rotate:90").
    ///
    /// Valid mapping options
    ///
    /// * `Mirror`
    /// * `Rotate:<Angle>`
    /// * `U-mapper`
    /// * `V-mapper`
    ///
    /// # Panics
    /// If the given `mapper` string fails to convert to a `CString`. This can
    /// occur when there is a null character mid way in the string.
    pub fn set_pixel_mapper_config(&mut self, mapper: &str) {
        unsafe {
            let _ = CString::from_raw(self.0.pixel_mapper_config);
            self.0.pixel_mapper_config = CString::new(mapper)
                .expect("given string failed to convert into a CString")
                .into_raw();
        }
    }

    /// Sets if hardware pin-pulse generation should be used.
    pub fn set_hardware_pulsing(&mut self, enable: bool) {
        if enable {
            self.0.disable_hardware_pulsing = 0;
        } else {
            self.0.disable_hardware_pulsing = 1;
        }
    }

    /// Configures if the current refresh rate should be printed by the C++ library.
    pub fn set_refresh_rate(&mut self, enable: bool) {
        if enable {
            self.0.show_refresh_rate = 1;
        } else {
            self.0.show_refresh_rate = 0;
        }
    }

    /// If set, invert the color displayed.
    pub fn set_inverse_colors(&mut self, enable: bool) {
        if enable {
            self.0.inverse_colors = 1;
        } else {
            self.0.inverse_colors = 0;
        }
    }

    /// Sets the type of multiplexing used.
    ///
    /// 0.  `direct`
    /// 1.  `Stripe`
    /// 2.  `Checkered`
    /// 3.  `Spiral`
    /// 4.  `ZStripe`
    /// 5.  `ZnMirrorZStripe`
    /// 6.  `coreman`
    /// 7.  `Kaler2Scan`
    /// 8.  `ZStripeUneven`
    /// 9.  `P10-128x4-Z`
    /// 10. `QiangLiQ8`
    /// 11. `InversedZStripe`
    /// 12. `P10Outdoor1R1G1-1`
    /// 13. `P10Outdoor1R1G1-2`
    /// 14. `P10Outdoor1R1G1-3`
    /// 15. `P10CoremanMapper`
    /// 16. `P8Outdoor1R1G1`
    pub fn set_multiplexing(&mut self, multiplexing: u32) {
        self.0.multiplexing = multiplexing as c_int;
    }

    /// Sets the type of row addressing to be used.
    ///
    /// 0. default
    /// 1. AB-addressed panels
    /// 2. direct row select
    /// 3. ABC-addressed panels
    /// 4. ABC Shift + DE direct
    pub fn set_row_addr_type(&mut self, row_addr_type: u32) {
        self.0.row_address_type = row_addr_type as c_int;
    }

    /// Limit refresh rate to this frequency in Hz. (0 = no limit)
    ///
    /// Useful to keep a constant refresh rate on loaded system.
    pub fn set_limit_refresh(&mut self, limit_refresh: u32) {
        self.0.limit_refresh_rate_hz = limit_refresh as c_int;
    }

    /// Configures how many bits to use for time-based dithering.
    pub fn set_pwm_dither_bits(&mut self, pwm_dither_bits: u32) {
        self.0.pwm_dither_bits = pwm_dither_bits as c_int;
    }

    /// Needed to initialize special panels. Supported: 'FM6126A', 'FM6127'
    ///
    /// # Panics
    /// If the given `panel_type` string fails to convert to a `CString`. This can
    /// occur when there is a null character mid way in the string.
    pub fn set_panel_type(&mut self, panel_type: &str) {
        unsafe {
            let _ = CString::from_raw(self.0.panel_type);
            self.0.panel_type = CString::new(panel_type)
                .expect("given string failed to convert into a CString")
                .into_raw();
        }
    }
}

impl Default for LedMatrixOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for LedMatrixOptions {
    fn drop(&mut self) {
        unsafe {
            let _ = CString::from_raw(self.0.hardware_mapping);
            let _ = CString::from_raw(self.0.led_rgb_sequence);
            let _ = CString::from_raw(self.0.panel_type);
        }
    }
}

impl LedRuntimeOptions {
    /// Creates a new `LedRuntimeOptions` struct with the default parameters.
    ///
    /// ```
    /// use rpi_led_matrix::{LedMatrix, LedRuntimeOptions};
    /// let mut rt_options = LedRuntimeOptions::new();
    /// rt_options.set_gpio_slowdown(3);
    /// let matrix = LedMatrix::new(None, Some(rt_options)).unwrap();
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self(ffi::CLedRuntimeOptions {
            gpio_slowdown: 1,
            daemon: 0,
            drop_privileges: 1,
            do_gpio_init: true,
        })
    }

    /// Sets the GPIO slowdown, in . Needed for faster Pis/slower panels
    pub fn set_gpio_slowdown(&mut self, gpio_slowdown: u32) {
        self.0.gpio_slowdown = gpio_slowdown as i32;
    }

    /// If True, make the process run in the background as daemon.
    pub fn set_daemon(&mut self, daemon: bool) {
        self.0.daemon = if daemon { 1 } else { 0 };
    }

    /// If True, drop privileges from 'root' after initializing the hardware.
    pub fn set_drop_privileges(&mut self, drop_privileges: bool) {
        self.0.drop_privileges = if drop_privileges { 1 } else { 0 };
    }

    /// You almost definitely want this to be left as True. Use this if you know what you're doing.
    pub fn set_do_gpio_init(&mut self, do_gpio_init: bool) {
        self.0.do_gpio_init = do_gpio_init;
    }
}

impl Default for LedRuntimeOptions {
    fn default() -> Self {
        Self::new()
    }
}
