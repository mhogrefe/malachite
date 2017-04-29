use integer::Integer;
use traits::Assign;

/// Converts a `u32` to an `Integer`.
///
/// # Example
/// ```
/// use malachite_gmp::integer::Integer;
///
/// assert_eq!(Integer::from(123).to_string(), "123");
/// ```
impl From<u32> for Integer {
    fn from(u: u32) -> Integer {
        let mut out = Integer::new();
        out.assign(u);
        out
    }
}
