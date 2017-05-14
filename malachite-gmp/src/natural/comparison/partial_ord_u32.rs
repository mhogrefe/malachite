use natural::Natural::{self, Large, Small};
use std::cmp::Ordering;

/// Compares `self` to a `u32`.
///
/// # Examples
/// ```
/// use malachite_gmp::natural::Natural;
/// use std::str::FromStr;
///
/// assert!(Natural::from(123) > 122);
/// assert!(Natural::from(123) >= 122);
/// assert!(Natural::from(123) < 124);
/// assert!(Natural::from(123) <= 124);
/// assert!(Natural::from_str("1000000000000").unwrap() > 123);
/// assert!(Natural::from_str("1000000000000").unwrap() >= 123);
/// ```
impl PartialOrd<u32> for Natural {
    fn partial_cmp(&self, other: &u32) -> Option<Ordering> {
        match *self {
            Small(ref small) => small.partial_cmp(other),
            Large(_) => Some(Ordering::Greater),
        }
    }
}

/// Compares a `u32` to `self`.
///
/// # Examples
/// ```
/// use malachite_gmp::natural::Natural;
/// use std::str::FromStr;
///
/// assert!(122 < Natural::from(123));
/// assert!(122 <= Natural::from(123));
/// assert!(124 > Natural::from(123));
/// assert!(124 >= Natural::from(123));
/// assert!(123 < Natural::from_str("1000000000000").unwrap());
/// assert!(123 <= Natural::from_str("1000000000000").unwrap());
/// ```
impl PartialOrd<Natural> for u32 {
    fn partial_cmp(&self, other: &Natural) -> Option<Ordering> {
        other.partial_cmp(self).map(|o| o.reverse())
    }
}
