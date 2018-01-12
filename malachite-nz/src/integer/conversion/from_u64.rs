use integer::Integer;
use natural::Natural;

/// Converts a `u64` to an `Integer`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Example
/// ```
/// use malachite_nz::integer::Integer;
///
/// assert_eq!(Integer::from(123u64).to_string(), "123");
/// ```
impl From<u64> for Integer {
    fn from(u: u64) -> Integer {
        Integer {
            sign: true,
            abs: Natural::from(u),
        }
    }
}
