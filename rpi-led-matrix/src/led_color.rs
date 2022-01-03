#[cfg(feature = "embeddedgraphics")]
use embedded_graphics_core::pixelcolor::{
    raw::RawU24, Bgr555, Bgr565, Bgr888, BinaryColor, Gray2, Gray4, Gray8, GrayColor, PixelColor,
    Rgb555, Rgb565, Rgb888, RgbColor,
};

/// The Rust handle representing a color you'd like to display.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LedColor {
    /// the red channel value
    pub red: u8,
    /// the green channel value
    pub green: u8,
    /// the blue channel value
    pub blue: u8,
}

#[cfg(feature = "embeddedgraphics")]
impl PixelColor for LedColor {
    type Raw = RawU24;
}

#[cfg(feature = "embeddedgraphics")]
impl From<Bgr555> for LedColor {
    fn from(p: Bgr555) -> Self {
        Self {
            red: p.r() << 3,
            green: p.g() << 3,
            blue: p.b() << 3,
        }
    }
}

#[cfg(feature = "embeddedgraphics")]
impl From<Bgr565> for LedColor {
    fn from(p: Bgr565) -> Self {
        Self {
            red: p.r() << 3,
            green: p.g() << 2,
            blue: p.b() << 3,
        }
    }
}

#[cfg(feature = "embeddedgraphics")]
impl From<Bgr888> for LedColor {
    fn from(p: Bgr888) -> Self {
        Self {
            red: p.r(),
            green: p.g(),
            blue: p.b(),
        }
    }
}

#[cfg(feature = "embeddedgraphics")]
impl From<Gray2> for LedColor {
    fn from(p: Gray2) -> Self {
        Self {
            red: p.luma() << 6,
            green: p.luma() << 6,
            blue: p.luma() << 6,
        }
    }
}

#[cfg(feature = "embeddedgraphics")]
impl From<Gray4> for LedColor {
    fn from(p: Gray4) -> Self {
        Self {
            red: p.luma() << 4,
            green: p.luma() << 4,
            blue: p.luma() << 4,
        }
    }
}

#[cfg(feature = "embeddedgraphics")]
impl From<Gray8> for LedColor {
    fn from(p: Gray8) -> Self {
        Self {
            red: p.luma(),
            green: p.luma(),
            blue: p.luma(),
        }
    }
}

#[cfg(feature = "embeddedgraphics")]
impl From<Rgb555> for LedColor {
    fn from(p: Rgb555) -> Self {
        Self {
            red: p.r() << 3,
            green: p.g() << 3,
            blue: p.b() << 3,
        }
    }
}

#[cfg(feature = "embeddedgraphics")]
impl From<Rgb565> for LedColor {
    fn from(p: Rgb565) -> Self {
        Self {
            red: p.r() << 3,
            green: p.g() << 2,
            blue: p.b() << 3,
        }
    }
}

#[cfg(feature = "embeddedgraphics")]
impl From<Rgb888> for LedColor {
    fn from(p: Rgb888) -> Self {
        Self {
            red: p.r(),
            green: p.g(),
            blue: p.b(),
        }
    }
}

#[cfg(feature = "embeddedgraphics")]
impl From<BinaryColor> for LedColor {
    fn from(p: BinaryColor) -> Self {
        let value = if p == BinaryColor::On {
            std::u8::MAX
        } else {
            0
        };
        Self {
            red: value,
            green: value,
            blue: value,
        }
    }
}
