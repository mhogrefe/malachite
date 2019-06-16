use std::cmp::Ordering;

use integer::Integer;
use platform::Limb;

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

#[cfg(not(feature = "32_bit_limbs"))]
impl PartialOrd<u32> for Integer {
    #[inline]
    fn partial_cmp(&self, other: &u32) -> Option<Ordering> {
        self.partial_cmp(&Limb::from(*other))
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

#[cfg(not(feature = "32_bit_limbs"))]
impl PartialOrd<Integer> for u32 {
    #[inline]
    fn partial_cmp(&self, other: &Integer) -> Option<Ordering> {
        Limb::from(*self).partial_cmp(other)
    }
}
