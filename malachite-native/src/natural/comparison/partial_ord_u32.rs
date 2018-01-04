use natural::Natural::{self, Large, Small};
use std::cmp::Ordering;

/// Compares a `Natural` to a `u32`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Examples
/// ```
/// use malachite_native::natural::Natural;
///
/// assert!(Natural::from(123u32) > 122);
/// assert!(Natural::from(123u32) >= 122);
/// assert!(Natural::from(123u32) < 124);
/// assert!(Natural::from(123u32) <= 124);
/// assert!(Natural::trillion() > 123);
/// assert!(Natural::trillion() >= 123);
/// ```
impl PartialOrd<u32> for Natural {
    fn partial_cmp(&self, other: &u32) -> Option<Ordering> {
        match *self {
            Small(ref small) => small.partial_cmp(other),
            Large(_) => Some(Ordering::Greater),
        }
    }
}

/// Compares a `u32` to `Natural`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Examples
/// ```
/// use malachite_native::natural::Natural;
///
/// assert!(122 < Natural::from(123u32));
/// assert!(122 <= Natural::from(123u32));
/// assert!(124 > Natural::from(123u32));
/// assert!(124 >= Natural::from(123u32));
/// assert!(123 < Natural::trillion());
/// assert!(123 <= Natural::trillion());
/// ```
impl PartialOrd<Natural> for u32 {
    fn partial_cmp(&self, other: &Natural) -> Option<Ordering> {
        match *other {
            Small(ref small) => self.partial_cmp(small),
            Large(_) => Some(Ordering::Less),
        }
    }
}
