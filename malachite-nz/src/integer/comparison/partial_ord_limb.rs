use integer::Integer;
use platform::Limb;
use std::cmp::Ordering;

/// Compares an `Integer` to a `Limb`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Examples
/// ```
/// use malachite_nz::integer::Integer;
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
impl PartialOrd<Limb> for Integer {
    fn partial_cmp(&self, other: &Limb) -> Option<Ordering> {
        if self.sign {
            self.abs.partial_cmp(other)
        } else {
            Some(Ordering::Less)
        }
    }
}

/// Compares a `Limb` to an `Integer`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Examples
/// ```
/// use malachite_nz::integer::Integer;
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
impl PartialOrd<Integer> for Limb {
    fn partial_cmp(&self, other: &Integer) -> Option<Ordering> {
        if other.sign {
            self.partial_cmp(&other.abs)
        } else {
            Some(Ordering::Greater)
        }
    }
}
