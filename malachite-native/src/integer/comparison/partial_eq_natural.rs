use integer::Integer;
use natural::Natural;

/// Determines whether `self` is equal to a `Natural`.
///
/// # Example
/// ```
/// use malachite_native::integer::Integer;
/// use malachite_native::natural::Natural;
///
/// assert!(Integer::from(123) == Natural::from(123u32));
/// assert!(Integer::from(123) != Natural::from(5u32));
/// ```
impl PartialEq<Natural> for Integer {
    fn eq(&self, other: &Natural) -> bool {
        match *self {
            Integer { sign: true, ref abs } if abs == other => true,
            _ => false,
        }
    }
}
