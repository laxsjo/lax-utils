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
                units: (Some("°"), Some("%"), Some("%")),
            },
            ColorSpace::Hsv => ColorSpaceInfo {
                labels: ("H", "S", "V"),
                units: (Some("°"), Some("%"), Some("%")),
            },
        }
    }
}

pub trait Color: Copy {
    fn as_components(self) -> (u16, u16, u16);
    fn from_components(components: (u16, u16, u16)) -> Self;

    fn as_floats(self) -> (f64, f64, f64);
    fn from_floats(float_color: (f64, f64, f64)) -> Self;

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
    fn as_components(self) -> (u16, u16, u16) {
        (self.r as u16, self.g as u16, self.b as u16)
    }
    fn from_components(components: (u16, u16, u16)) -> Self {
        Self {
            r: components.0 as u8,
            g: components.1 as u8,
            b: components.2 as u8,
        }
    }

    fn as_floats(self) -> (f64, f64, f64) {
        (
            self.r as f64 / 255.,
            self.g as f64 / 255.,
            self.b as f64 / 255.,
        )
    }
    fn from_floats(float_color: (f64, f64, f64)) -> Self {
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

#[derive(Debug, Clone, Copy)]
pub struct Hsl {
    pub h: u16,
    pub s: u16,
    pub l: u16,
}

impl Color for Hsl {
    fn as_components(self) -> (u16, u16, u16) {
        (self.h, self.s, self.l)
    }
    fn from_components(components: (u16, u16, u16)) -> Self {
        Self {
            h: components.0.clamp(0, 101),
            s: components.1.clamp(0, 101),
            l: components.2.clamp(0, 101),
        }
    }

    fn as_floats(self) -> (f64, f64, f64) {
        (
            self.h as f64 / 360.,
            self.s as f64 / 100.,
            self.l as f64 / 100.,
        )
    }
    fn from_floats(float_color: (f64, f64, f64)) -> Self {
        Self {
            h: (float_color.0.clamp(0., 1.) * 360.) as u16,
            s: (float_color.0.clamp(0., 1.) * 100.) as u16,
            l: (float_color.0.clamp(0., 1.) * 100.) as u16,
        }
    }

    /// Source: https://stackoverflow.com/a/9493060/15507414
    fn as_rgb(self) -> Rgb {
        const ONE_THIRD: f64 = 1. / 3.;

        let (h, s, l) = self.as_floats();

        if s == 0. {
            let value = (l * 255.) as u16;
            return Rgb::from_components((value, value, value));
        }

        /// What are `p`, `q`, and `t`? I have no idea :D
        fn hue_to_rgb(p: f64, q: f64, mut t: f64) -> f64 {
            const ONE_SIXTH: f64 = 1. / 6.;
            const ONE_HALF: f64 = 0.5;
            const TWO_THIRDS: f64 = 2. / 3.;

            if t < 0. {
                t += 1.
            };
            if t > 1. {
                t -= 1.
            };
            match t {
                _ if t < ONE_SIXTH => p + (q - p) * 6. * t,
                _ if t < ONE_HALF => q,
                _ if t < TWO_THIRDS => p + (q - p) * (TWO_THIRDS - t) * 6.,
                _ => p,
            }
        }

        let q = if l < 0.5 { l * (1. + s) } else { l + s - l * s };
        let p = 2. * l - q;

        // r, g, b are all floats in the range 0 to 1
        let r = hue_to_rgb(p, q, h + ONE_THIRD);
        let g = hue_to_rgb(p, q, h);
        let b = hue_to_rgb(p, q, h - ONE_THIRD);

        Rgb::from_floats((r, g, b))
    }

    /// Source: https://stackoverflow.com/a/9493060/15507414
    fn from_rgb(rgb: Rgb) -> Self {
        let (r, g, b) = rgb.as_floats();

        let max = r.max(g.max(b));
        let min = r.min(g.min(b));

        let l = (max + min) / 2.;

        if max == min {
            // achromatic
            return Self::from_floats((0., 0., l));
        }

        let delta = max - min;

        let s = if l > 0.5 {
            delta / (2. - min - max)
        } else {
            delta / (max + min)
        };

        // matching on a float feels iffy...
        let h = match max {
            _ if max == r => (g - b) / delta + (if g < b { 6. } else { 0. }),
            _ if max == g => (b - r) / delta + 2.,
            _ if max == b => (r - g) / delta + 4.,
            _ => panic!("float comparison failed for finding maximum component"),
        } / 6.;

        Self::from_floats((h, s, l))
    }
}
