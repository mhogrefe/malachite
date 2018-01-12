use integer::Integer;
use natural::Natural;

/// Converts a `u32` to an `Integer`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Example
/// ```
/// use malachite_nz::integer::Integer;
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
