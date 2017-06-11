use integer::Integer;
use natural::Natural;
use std::cmp::Ordering;

/// Compares `self` to an `Integer`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `self.significant_bits() + other.significant_bits()`
///
/// # Examples
/// ```
/// use malachite_native::integer::Integer;
/// use malachite_native::natural::Natural;
///
/// assert!(Natural::from(123u32) > Integer::from(122));
/// assert!(Natural::from(123u32) >= Integer::from(122));
/// assert!(Natural::from(123u32) < Integer::from(124));
/// assert!(Natural::from(123u32) <= Integer::from(124));
/// assert!(Natural::from(123u32) > Integer::from(-123));
/// assert!(Natural::from(123u32) >= Integer::from(-123));
/// ```
impl PartialOrd<Integer> for Natural {
    fn partial_cmp(&self, other: &Integer) -> Option<Ordering> {
        if other.sign {
            self.partial_cmp(&other.abs)
        } else {
            Some(Ordering::Greater)
        }
    }
}
