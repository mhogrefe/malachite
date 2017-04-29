use integer::Integer;
use natural::Natural;

/// Converts a `u32` to an `Integer`.
///
/// # Example
/// ```
/// use malachite_native::integer::Integer;
///
/// assert_eq!(Integer::from(123).to_string(), "123");
/// ```
impl From<u32> for Integer {
    fn from(u: u32) -> Integer {
        Integer {
            sign: true,
            abs: Natural::from(u),
        }
    }
}
