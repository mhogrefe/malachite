use integer::Integer;
use natural::Natural;

/// Determines whether `self` is equal to an `Integer`.
///
/// # Example
/// ```
/// use malachite_native::integer::Integer;
/// use malachite_native::natural::Natural;
///
/// assert!(Natural::from(123) == Integer::from(123));
/// assert!(Natural::from(123) != Integer::from(5));
/// ```
impl PartialEq<Integer> for Natural {
    fn eq(&self, i: &Integer) -> bool {
        i == self
    }
}
