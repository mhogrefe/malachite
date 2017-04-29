use integer::{Integer, Small};

/// Converts an `i32` to an `Integer`.
///
/// # Example
/// ```
/// use malachite_gmp::integer::Integer;
///
/// assert_eq!(Integer::from(-123).to_string(), "-123");
/// ```
impl From<i32> for Integer {
    fn from(i: i32) -> Integer {
        Small(i)
    }
}
