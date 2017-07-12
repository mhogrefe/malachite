use integer::Integer;

/// Converts an `i64` to an `Integer`.
///
/// # Example
/// ```
/// use malachite_gmp::integer::Integer;
///
/// assert_eq!(Integer::from(123i64).to_string(), "123");
/// assert_eq!(Integer::from(-123i64).to_string(), "-123");
/// ```
impl From<i64> for Integer {
    fn from(i: i64) -> Integer {
        if i >= 0 {
            Integer::from(i as u64)
        } else {
            -Integer::from(i.abs() as u64)
        }
    }
}
