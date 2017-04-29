use integer::Integer;
use natural::Natural;

/// Converts an `i32` to an `Integer`.
///
/// # Example
/// ```
/// use malachite_native::integer::Integer;
///
/// assert_eq!(Integer::from(-123).to_string(), "-123");
/// ```
impl From<i32> for Integer {
    fn from(i: i32) -> Integer {
        Integer {
            sign: i >= 0,
            abs: Natural::from(i.abs() as u32),
        }
    }
}
