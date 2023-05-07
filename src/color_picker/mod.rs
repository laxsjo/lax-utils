#[derive(Debug, Clone, Copy)]
pub struct ColorSpaceInfo {
    pub labels: (&'static str, &'static str, &'static str),
    pub units: (
        Option<&'static str>,
        Option<&'static str>,
        Option<&'static str>,
    ),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorSpace {
    Rgb,
    Hsl,
    Hsv,
    // TODO: Figure out what color spaces to add
}

impl ColorSpace {
    pub fn info(&self) -> ColorSpaceInfo {
        match self {
            ColorSpace::Rgb => ColorSpaceInfo {
                labels: ("R", "G", "B"),
                units: (None, None, None),
            },
            ColorSpace::Hsl => ColorSpaceInfo {
                labels: ("H", "S", "L"),
                units: (None, Some("%"), Some("%")),
            },
            ColorSpace::Hsv => ColorSpaceInfo {
                labels: ("H", "S", "V"),
                units: (None, Some("%"), Some("%")),
            },
        }
    }
}

pub trait Color: Copy {
    fn as_components(self) -> (u8, u8, u8);
    fn from_components(components: (u8, u8, u8)) -> Self;

    fn as_float(self) -> (f64, f64, f64);
    fn from_float(float_color: (f64, f64, f64)) -> Self;

    fn as_rgb(self) -> Rgb;
    fn from_rgb(rgb: Rgb) -> Self;

    fn as_color<C: Color>(self) -> C {
        let rgb = self.as_rgb();

        C::from_rgb(rgb)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color for Rgb {
    fn as_components(self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }
    fn from_components(components: (u8, u8, u8)) -> Self {
        Self {
            r: components.0,
            g: components.1,
            b: components.2,
        }
    }

    fn as_float(self) -> (f64, f64, f64) {
        (
            self.r as f64 / 255.,
            self.g as f64 / 255.,
            self.b as f64 / 255.,
        )
    }
    fn from_float(float_color: (f64, f64, f64)) -> Self {
        Self {
            r: (float_color.0 * 255.) as u8,
            g: (float_color.1 * 255.) as u8,
            b: (float_color.2 * 255.) as u8,
        }
    }

    fn as_rgb(self) -> Rgb {
        self
    }
    fn from_rgb(rgb: Rgb) -> Self {
        rgb
    }
}
