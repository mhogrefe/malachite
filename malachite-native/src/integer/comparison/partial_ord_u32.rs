use integer::Integer;
use std::cmp::Ordering;

/// Compares an `Integer` to a `u32`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Examples
/// ```
/// use malachite_native::integer::Integer;
///
/// assert!(Integer::from(123) > 122);
/// assert!(Integer::from(123) >= 122);
/// assert!(Integer::from(123) < 124);
/// assert!(Integer::from(123) <= 124);
/// assert!(Integer::trillion() > 123);
/// assert!(Integer::trillion() >= 123);
/// assert!(-Integer::trillion() < 123);
/// assert!(-Integer::trillion() <= 123);
/// ```
impl PartialOrd<u32> for Integer {
    fn partial_cmp(&self, other: &u32) -> Option<Ordering> {
        if self.sign {
            self.abs.partial_cmp(other)
        } else {
            Some(Ordering::Less)
        }
    }
}

/// Compares a `u32` to an `Integer`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Examples
/// ```
/// use malachite_native::integer::Integer;
///
/// assert!(122 < Integer::from(123));
/// assert!(122 <= Integer::from(123));
/// assert!(124 > Integer::from(123));
/// assert!(124 >= Integer::from(123));
/// assert!(123 < Integer::trillion());
/// assert!(123 <= Integer::trillion());
/// assert!(123 > -Integer::trillion());
/// assert!(123 >= -Integer::trillion());
/// ```
impl PartialOrd<Integer> for u32 {
    fn partial_cmp(&self, other: &Integer) -> Option<Ordering> {
        if other.sign {
            self.partial_cmp(&other.abs)
        } else {
            Some(Ordering::Greater)
        }
    }
}
