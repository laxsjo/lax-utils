use std::str::FromStr;

pub trait StringUtils {
    fn parse_input<T: FromStr>(&self) -> Option<T>;
}

impl StringUtils for str {
    /// Trim and parse a value from an input element into the desired type.
    fn parse_input<T: FromStr>(&self) -> Option<T> {
        let result = self.to_owned().trim().parse::<T>();

        result.ok()
    }
}
