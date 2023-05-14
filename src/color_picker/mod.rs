pub mod components;

use crate::utils::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorSpaceInfo {
    pub labels: (&'static str, &'static str, &'static str),
    pub units: (
        Option<&'static str>,
        Option<&'static str>,
        Option<&'static str>,
    ),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    pub const fn color_component_maxes(&self) -> (f64, f64, f64) {
        match self {
            ColorSpace::Rgb => Rgb::COMPONENT_MAXES,
            ColorSpace::Hsl => Hsl::COMPONENT_MAXES,
            ColorSpace::Hsv => Hsv::COMPONENT_MAXES,
        }
    }

    pub fn clamp_color_components(
        &self,
        components: (f64, f64, f64),
    ) -> (f64, f64, f64) {
        let clamp = match self {
            ColorSpace::Rgb => Rgb::clamp_components,
            ColorSpace::Hsl => Hsl::clamp_components,
            ColorSpace::Hsv => Hsv::clamp_components,
        };

        clamp(components)
    }

    fn components_to_floats(
        &self,
        components: (f64, f64, f64),
    ) -> (f64, f64, f64) {
        let convert = match self {
            ColorSpace::Rgb => Rgb::components_to_floats,
            ColorSpace::Hsl => Hsl::components_to_floats,
            ColorSpace::Hsv => Hsv::components_to_floats,
        };

        convert(components)
    }

    fn floats_to_components(&self, floats: (f64, f64, f64)) -> (f64, f64, f64) {
        let convert = match self {
            ColorSpace::Rgb => Rgb::floats_to_components,
            ColorSpace::Hsl => Hsl::floats_to_components,
            ColorSpace::Hsv => Hsv::floats_to_components,
        };

        convert(floats)
    }

    fn color_components_from_rgb(&self, rgb: Rgb) -> (f64, f64, f64) {
        match self {
            ColorSpace::Rgb => Rgb::from_rgb(rgb).as_components(),
            ColorSpace::Hsl => Hsl::from_rgb(rgb).as_components(),
            ColorSpace::Hsv => Hsv::from_rgb(rgb).as_components(),
        }
    }
    fn rgb_from_color_components(&self, components: (f64, f64, f64)) -> Rgb {
        match self {
            ColorSpace::Rgb => Rgb::from_components(components).as_rgb(),
            ColorSpace::Hsl => Hsl::from_components(components).as_rgb(),
            ColorSpace::Hsv => Hsv::from_components(components).as_rgb(),
        }
    }
}

impl UiDisplay for ColorSpace {
    type Environment = ();

    fn to_ui_string(&self, _env: Self::Environment) -> String {
        match self {
            ColorSpace::Rgb => "RGB",
            ColorSpace::Hsl => "HSL",
            ColorSpace::Hsv => "HSV",
        }
        .to_owned()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DynamicColor {
    components: (f64, f64, f64),
    color_space: ColorSpace,
}

impl DynamicColor {
    pub fn new(components: (f64, f64, f64), color_space: ColorSpace) -> Self {
        Self {
            components: color_space.clamp_color_components(components),
            color_space,
        }
    }

    pub fn from_floats(
        floats: (f64, f64, f64),
        color_space: ColorSpace,
    ) -> Self {
        Self {
            components: color_space.floats_to_components(floats),
            color_space,
        }
    }

    pub fn from_color<C: Color>(color: C) -> Self {
        Self::new(color.as_components(), C::COLOR_SPACE)
    }

    pub fn to_color<C: Color>(self) -> C {
        C::from_components(self.set_color_space(C::COLOR_SPACE).components)
    }

    pub fn components(&self) -> (f64, f64, f64) {
        self.components
    }

    pub fn set_components(mut self, components: (f64, f64, f64)) -> Self {
        self.components = self.color_space.clamp_color_components(components);
        self
    }

    pub fn set_component_0(mut self, component: f64) -> Self {
        self.components.0 = component;
        self
    }
    pub fn set_component_1(mut self, component: f64) -> Self {
        self.components.1 = component;
        self
    }
    pub fn set_component_2(mut self, component: f64) -> Self {
        self.components.2 = component;
        self
    }

    pub fn set_float_0(self, float: f64) -> Self {
        let mut floats = self.as_floats();
        floats.0 = float;
        self.set_floats(floats)
    }
    pub fn set_float_1(self, float: f64) -> Self {
        let mut floats = self.as_floats();
        floats.1 = float;
        self.set_floats(floats)
    }
    pub fn set_float_2(self, float: f64) -> Self {
        let mut floats = self.as_floats();
        floats.2 = float;
        self.set_floats(floats)
    }

    pub fn color_space(&self) -> ColorSpace {
        self.color_space
    }

    pub fn set_color_space(mut self, color_space: ColorSpace) -> Self {
        let rgb = self.color_space.rgb_from_color_components(self.components);

        self.components = color_space.color_components_from_rgb(rgb);

        self.color_space = color_space;

        self
    }

    pub fn as_floats(&self) -> (f64, f64, f64) {
        self.color_space.components_to_floats(self.components)
    }

    pub fn set_floats(mut self, floats: (f64, f64, f64)) -> Self {
        self.components = self.color_space.floats_to_components(floats);
        self
    }
}

pub trait Color {
    const COMPONENT_MAXES: (f64, f64, f64);
    const COLOR_SPACE: ColorSpace;

    fn as_components(&self) -> (f64, f64, f64);
    fn from_components(components: (f64, f64, f64)) -> Self
    where
        Self: Sized;

    fn as_floats(&self) -> (f64, f64, f64);
    fn from_floats(floats: (f64, f64, f64)) -> Self
    where
        Self: Sized;

    fn as_rgb(&self) -> Rgb;
    fn from_rgb(rgb: Rgb) -> Self
    where
        Self: Sized;

    fn as_color<C: Color>(&self) -> C
    where
        Self: Sized,
    {
        let rgb = self.as_rgb();

        C::from_rgb(rgb)
    }

    // fn as_color_space(&self, color_space: ColorSpace) -> Box<dyn Color_old>
    // where
    //     Self: Sized,
    // {
    //     let rgb = self.as_rgb();

    //     // let test: Box<dyn Color>;

    //     match color_space {
    //         ColorSpace::Rgb => Box::new(rgb),
    //         ColorSpace::Hsl => Box::new(rgb.as_color::<Hsl>()),
    //     }
    // }

    fn clamp_components(components: (f64, f64, f64)) -> (f64, f64, f64)
    where
        Self: Sized,
    {
        let maxes = Self::COMPONENT_MAXES;
        (
            components.0.clamp(0., maxes.0 + 1.),
            components.1.clamp(0., maxes.1 + 1.),
            components.2.clamp(0., maxes.2 + 1.),
        )
    }

    fn components_to_floats(components: (f64, f64, f64)) -> (f64, f64, f64)
    where
        Self: Sized,
    {
        let maxes = Self::COMPONENT_MAXES;

        (
            components.0 / maxes.0,
            components.1 / maxes.1,
            components.2 / maxes.2,
        )
    }

    fn floats_to_components(floats: (f64, f64, f64)) -> (f64, f64, f64)
    where
        Self: Sized,
    {
        let maxes = Self::COMPONENT_MAXES;

        (
            (floats.0 * maxes.0).clamp(0., maxes.0 + 1.),
            (floats.1 * maxes.1).clamp(0., maxes.1 + 1.),
            (floats.2 * maxes.2).clamp(0., maxes.2 + 1.),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rgb {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Rgb {
    /// Format rgb color as an hex code.
    ///
    /// The three hexadecimal components are returned without the usual hashtag
    /// prefix.
    /// E.g. `"f0f0f0"` instead of `"#f0f0f0"`.
    ///
    /// # Examples
    /// ```
    /// use lax_utils::color_picker::*;
    ///
    /// assert_eq!(Rgb::from_components((127., 127., 127.)).as_hex_code(), "7f7f7f".to_owned());
    ///
    /// assert_eq!(Rgb::from_components((0., 255., 0.)).as_hex_code(), "00ff00".to_owned());
    /// ```
    pub fn as_hex_code(self) -> String {
        let r = self.r as u8;
        let g = self.g as u8;
        let b = self.b as u8;

        format!("{:02x}{:02x}{:02x}", r, g, b)
    }
}

impl Color for Rgb {
    const COMPONENT_MAXES: (f64, f64, f64) = (255., 255., 255.);
    const COLOR_SPACE: ColorSpace = ColorSpace::Rgb;

    fn as_components(&self) -> (f64, f64, f64) {
        (self.r, self.g, self.b)
    }
    fn from_components(components: (f64, f64, f64)) -> Self {
        let components = ColorSpace::Rgb.clamp_color_components(components);

        Self {
            r: components.0,
            g: components.1,
            b: components.2,
        }
    }

    fn as_floats(&self) -> (f64, f64, f64) {
        (self.r / 255., self.g / 255., self.b / 255.)
    }
    fn from_floats(floats: (f64, f64, f64)) -> Self {
        Self {
            r: floats.0.clamp(0., 1.) * 255.,
            g: floats.1.clamp(0., 1.) * 255.,
            b: floats.2.clamp(0., 1.) * 255.,
        }
    }

    fn as_rgb(&self) -> Rgb {
        *self
    }
    fn from_rgb(rgb: Rgb) -> Self {
        rgb
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hsl {
    pub h: f64,
    pub s: f64,
    pub l: f64,
}

impl Color for Hsl {
    const COMPONENT_MAXES: (f64, f64, f64) = (360., 100., 100.);
    const COLOR_SPACE: ColorSpace = ColorSpace::Hsl;

    fn as_components(&self) -> (f64, f64, f64) {
        (self.h, self.s, self.l)
    }
    fn from_components(components: (f64, f64, f64)) -> Self {
        Self {
            h: components.0.clamp(0., 360.),
            s: components.1.clamp(0., 100.),
            l: components.2.clamp(0., 100.),
        }
    }

    fn as_floats(&self) -> (f64, f64, f64) {
        (self.h / 360., self.s / 100., self.l / 100.)
    }
    fn from_floats(floats: (f64, f64, f64)) -> Self {
        Self {
            h: floats.0.clamp(0., 1.) * 360.,
            s: floats.1.clamp(0., 1.) * 100.,
            l: floats.2.clamp(0., 1.) * 100.,
        }
    }

    /// Source: https://stackoverflow.com/a/9493060/15507414
    fn as_rgb(&self) -> Rgb {
        const ONE_THIRD: f64 = 1. / 3.;

        let (h, s, l) = self.as_floats();

        if s == 0. {
            let value = l * 255.;
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
            _ => {
                panic!("float comparison failed for finding maximum component")
            }
        } / 6.;

        Self::from_floats((h, s, l))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hsv {
    pub h: f64,
    pub s: f64,
    pub v: f64,
}

impl Color for Hsv {
    const COMPONENT_MAXES: (f64, f64, f64) = (360., 100., 100.);
    const COLOR_SPACE: ColorSpace = ColorSpace::Hsv;

    fn as_components(&self) -> (f64, f64, f64) {
        (self.h, self.s, self.v)
    }
    fn from_components(components: (f64, f64, f64)) -> Self {
        let components = Self::clamp_components(components);

        Self {
            h: components.0,
            s: components.1,
            v: components.2,
        }
    }

    fn as_floats(&self) -> (f64, f64, f64) {
        Self::components_to_floats(self.as_components())
    }
    fn from_floats(floats: (f64, f64, f64)) -> Self {
        let components = Self::floats_to_components(floats);

        Self::from_components(components)
    }

    /// Source: https://www.codespeedy.com/hsv-to-rgb-in-cpp/
    fn as_rgb(&self) -> Rgb {
        let (h, s, v) = self.as_components();

        if s == 0. {
            let adjusted_value = v / 100. * 255.;
            return Rgb::from_components((
                adjusted_value,
                adjusted_value,
                adjusted_value,
            ));
        }

        let (_, s_float, v_float) = self.as_floats();

        // I have no idea what any of the intermediary variables mean...
        let c = s_float * v_float;
        let x = c * (1. - ((h / 60. % 2.) - 1.).abs());
        let m = v_float - c;

        // These aren't really the final r, g, and b float components. See
        // below.
        let (r, g, b) = match () {
            _ if (0.0..=59.0).contains(&h) => (c, x, 0.),
            _ if (60.0..=119.0).contains(&h) => (x, c, 0.),
            _ if (120.0..=179.0).contains(&h) => (0., c, x),
            _ if (180.0..=239.0).contains(&h) => (0., x, c),
            _ if (240.0..=300.0).contains(&h) => (x, 0., c),
            _ => (c, 0., x),
        };

        Rgb::from_floats((r + m, g + m, b + m))
    }

    /// Source: https://www.rapidtables.com/convert/color/rgb-to-hsv.html
    /// Fixes from: https://mattlockyer.github.io/iat455/documents/rgb-hsv.pdf
    fn from_rgb(rgb: Rgb) -> Self
    where
        Self: Sized,
    {
        let (r, g, b) = rgb.as_floats();

        let max = r.max(g.max(b));
        let min = r.min(g.min(b));

        let delta = max - min;

        let h_degrees = if delta == 0. {
            0.
        } else if max == r {
            (g - b) / delta
        } else if max == g {
            (b - r) / delta + 2.
        } else {
            // blue is max
            (r - g) / delta + 4.
        }
        .rem_euclid(6.)
            * 60.;
        let h = h_degrees / 360.;

        let s = if max == 0. { 0. } else { delta / max };

        let v = max;

        Self::from_floats((h, s, v))
    }
}
