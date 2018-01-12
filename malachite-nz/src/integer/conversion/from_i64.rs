use integer::Integer;
use natural::Natural;

/// Converts an `i64` to an `Integer`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Example
/// ```
/// use malachite_nz::integer::Integer;
///
/// assert_eq!(Integer::from(123i64).to_string(), "123");
/// assert_eq!(Integer::from(-123i64).to_string(), "-123");
/// ```
impl From<i64> for Integer {
    fn from(i: i64) -> Integer {
        Integer {
            sign: i >= 0,
            abs: Natural::from(i.abs() as u64),
        }
    }
}
