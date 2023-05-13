use crate::string_utils::*;
use approx::*;
use leptos::html::*;
use num_traits::{AsPrimitive, Float};

pub trait FloatUtils: Float {
    /// Round float to a specified amount of decimal digits.
    /// Positive values for `digits` specify the amount of digits before the decimal
    /// point and negative values specify
    /// the amount after.
    ///
    /// **Note:** This doesn't seem to be safe for very large floats. Proceed
    /// with caution.
    ///
    /// # Examples
    /// ```
    /// use lax_utils::utils::FloatUtils;
    ///
    /// assert_eq!(10.11.round_digits(-1), 10.1);
    /// assert_eq!(123.456.round_digits(-2), 123.46);
    /// assert_eq!(111.1.round_digits(0), 111.);
    /// assert_eq!(111.1.round_digits(1), 110.);
    /// assert_eq!(111.1.round_digits(2), 100.);
    /// // assert_eq!(
    /// //     11111111111111111111.0.round_digits(5),
    /// //     11111111111111100000.0
    /// // );
    /// ```
    fn round_digits(self, digits: i32) -> Self
    where
        Self: 'static,
        f64: AsPrimitive<Self>,
    {
        let ten: Self = 10.0_f64.as_();

        let factor = ten.powi(-digits);

        (self * factor).round() / factor
    }

    /// Get the nth digit of a float.
    /// The digit `0` represents the first digit to the left of the decimal
    /// point, while a value of `-1` represents the first to the right.
    ///
    /// # Examples
    /// ```
    /// use lax_utils::utils::FloatUtils;
    ///
    /// assert_eq!(123.0.nth_digit(0), 3_u8);
    /// assert_eq!(123.456.nth_digit(-2), 5_u8);
    /// assert_eq!(123.000089.nth_digit(-5), 8_u8);
    /// assert_eq!(100000000000000000000.0.nth_digit(20), 1_u8);
    /// assert_eq!(99144444444444444444444.0.nth_digit(20), 1_u8);
    /// ```
    fn nth_digit(self, digit: isize) -> u8
    where
        Self: 'static + std::fmt::Display,
    {
        let string = self.to_string();
        let str = string.trim_start_matches('-');

        let len = str.chars().count();

        let point_index = str.chars().position(|char| char == '.').unwrap_or(len) as isize;

        let index = if digit >= 0 {
            point_index - 1 - digit
        } else {
            point_index - digit
        };

        if index < 0 || index >= len as isize {
            return 0;
        }
        let index = index as usize;

        let char = str
            .chars()
            .nth(index)
            .expect("index is smaller than char length");

        if !char.is_ascii_digit() {
            panic!(
                "float string '{}' contains non decimal characters '.0-9'",
                str
            );
        }

        // ? How can I remove this extra expect, while keeping the formatting
        // from `panic!`;
        char.to_digit(10)
            .expect("float string contains only decimal characters '.0-9'") as u8
    }

    // Prone to floating point issues:
    // fn nth_digit(self, digit: i32) -> u8
    // where
    //     Self: 'static + AsPrimitive<u8>,
    //     f64: AsPrimitive<Self>,
    // {
    //     let ten: Self = 10.0_f64.as_();

    //     let factor = ten.powi(-digit - 1);

    //     let fractional = (self * factor).fract();

    //     (fractional * ten).floor().as_()
    // }

    fn decimal_places(self) -> usize
    where
        Self: std::fmt::Display,
    {
        let string = self.to_string();

        let len = string.chars().count();

        let point_index = string.chars().position(|char| char == '.').unwrap_or(len);

        let Some(non_zero_index_back) = string.chars().rev().position(|char| char != '0') else {
            return 0;
        };
        let non_zero_index = len - 1 - non_zero_index_back;

        if non_zero_index <= point_index {
            return 0;
        }

        non_zero_index - point_index
    }

    /// Check if two floats are equal to a specified decimal digits.
    /// Positive values for `digits` specify the amount of digits before the decimal
    /// point, and negative values specify the amount after.
    ///
    /// **Note:** This does not naturally round the floats, but instead floors them to
    /// cut of the unwanted digits.
    /// TODO: Maybe this should change?
    ///
    /// # Examples
    /// ```
    /// use lax_utils::utils::FloatUtils;
    ///
    /// assert!(1.0_f64.float_compare_digits(1.002, -2));
    /// assert!(!1.0_f64.float_compare_digits(1.002, -3));
    /// assert!(0.000011_f64.float_compare_digits(0.0000112, -6));
    /// assert!(!0.000011_f64.float_compare_digits(0.0000112, -7));
    /// assert!(100150.0_f64.float_compare_digits(100000., 4));
    /// assert!(1.0_f64.float_compare_digits(1.1, 0));
    /// assert!(0.0_f64.float_compare_digits(0.01, -1));
    /// ```
    fn float_compare_digits(self, other: Self, digits: i32) -> bool
    where
        Self: 'static + UlpsEq<Self, Epsilon = Self>,
        i32: AsPrimitive<Self>,
        f64: AsPrimitive<Self>,
    {
        // let digits_f: F = digits.as_();

        let ten: Self = 10.0_f64.as_();

        let factor = ten.powi(-digits);

        // you should read
        // https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/
        // (I was too tired to read it when writing this)
        ulps_eq!((self * factor).floor(), (other * factor).floor())
    }
}

impl<F> FloatUtils for F where F: Float {}

/// Format a float to a string, with a minimum amount of displayed decimal
/// places, but also no more than the specified maximum.
///
/// The amount of displayed decimals are less than the maximum if and only if
/// the last decimals are zeros.
///
/// # Examples
/// ```
/// use lax_utils::utils::naturally_format_float;
/// assert_eq!(naturally_format_float(1.23456, 1, 3), "1.235".to_owned());
/// assert_eq!(naturally_format_float(1.23000, 1, 3), "1.23".to_owned());
/// assert_eq!(naturally_format_float(1.00000, 1, 3), "1.0".to_owned());
/// ```
pub fn naturally_format_float(float: f64, min_decimals: usize, max_decimals: usize) -> String {
    // float.fract()
    let float = float.round_digits(-(max_decimals as i32));

    println!("{}", float);

    let decimal_places = float.decimal_places().clamp(min_decimals, max_decimals);

    format!("{:.*}", decimal_places, float)
}

#[test]
fn test() {}

/// Update a input value with a new modified version of the same value, keeping
/// it unchanged if they are sufficiently similar (to get around floating point errors).
#[allow(unused)]
pub fn sync_input_value_float(
    input_element: &HtmlElement<Input>,
    value: f64,
    decimals: usize,
    format_fn: impl FnOnce(f64) -> String,
) {
    let current_value = input_element.value().parse_input::<f64>().unwrap_or(0.);

    if !value.float_compare_digits(current_value, -(decimals as i32)) {
        input_element.set_value(&format_fn(value));
    }
}
