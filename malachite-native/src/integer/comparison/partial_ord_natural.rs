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
/// assert!(Integer::from(123) > Natural::from(122));
/// assert!(Integer::from(123) >= Natural::from(122));
/// assert!(Integer::from(123) < Natural::from(124));
/// assert!(Integer::from(123) <= Natural::from(124));
/// assert!(Integer::from(-123) < Natural::from(123));
/// assert!(Integer::from(-123) <= Natural::from(123));
/// ```
impl PartialOrd<Natural> for Integer {
    fn partial_cmp(&self, other: &Natural) -> Option<Ordering> {
        if self.sign {
            self.abs.partial_cmp(other)
        } else {
            Some(Ordering::Less)
        }
    }
}
