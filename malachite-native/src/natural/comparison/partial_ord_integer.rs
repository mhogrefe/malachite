use integer::Integer;
use natural::Natural;
use std::cmp::Ordering;

/// Compares `self` to an `Integer`.
///
/// # Examples
/// ```
/// use malachite_native::integer::Integer;
/// use malachite_native::natural::Natural;
///
/// assert!(Natural::from(123) > Integer::from(122));
/// assert!(Natural::from(123) >= Integer::from(122));
/// assert!(Natural::from(123) < Integer::from(124));
/// assert!(Natural::from(123) <= Integer::from(124));
/// assert!(Natural::from(123) > Integer::from(-123));
/// assert!(Natural::from(123) >= Integer::from(-123));
/// ```
impl PartialOrd<Integer> for Natural {
    fn partial_cmp(&self, other: &Integer) -> Option<Ordering> {
        other.partial_cmp(self).map(|o| o.reverse())
    }
}
