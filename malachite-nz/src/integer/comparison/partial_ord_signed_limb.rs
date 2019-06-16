use std::cmp::Ordering;

use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::conversion::traits::WrappingFrom;

use integer::Integer;
use platform::{Limb, SignedLimb};

/// Compares `self` to a `SignedLimb`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Examples
/// ```
/// use malachite_nz::integer::Integer;
///
/// assert!(Integer::from(-123) < -122);
/// assert!(Integer::from(-123) <= -122);
/// assert!(Integer::from(-123) > -124);
/// assert!(Integer::from(-123) >= -124);
/// assert!(Integer::trillion() > 123);
/// assert!(Integer::trillion() >= 123);
/// assert!(-Integer::trillion() < 123);
/// assert!(-Integer::trillion() <= 123);
/// ```
impl PartialOrd<SignedLimb> for Integer {
    fn partial_cmp(&self, other: &SignedLimb) -> Option<Ordering> {
        if self.sign {
            if *other >= 0 {
                self.abs.partial_cmp(&Limb::wrapping_from(*other))
            } else {
                Some(Ordering::Greater)
            }
        } else if *other >= 0 {
            Some(Ordering::Less)
        } else {
            other.unsigned_abs().partial_cmp(&self.abs)
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl PartialOrd<i32> for Integer {
    #[inline]
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.partial_cmp(&SignedLimb::from(*other))
    }
}

/// Compares a `SignedLimb` to `self`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Examples
/// ```
/// use malachite_nz::integer::Integer;
///
/// assert!(-122 > Integer::from(-123));
/// assert!(-122 >= Integer::from(-123));
/// assert!(-124 < Integer::from(-123));
/// assert!(-124 <= Integer::from(-123));
/// assert!(123 < Integer::trillion());
/// assert!(123 <= Integer::trillion());
/// assert!(123 > -Integer::trillion());
/// assert!(123 >= -Integer::trillion());
/// ```
impl PartialOrd<Integer> for SignedLimb {
    fn partial_cmp(&self, other: &Integer) -> Option<Ordering> {
        if other.sign {
            if *self >= 0 {
                (*self as Limb).partial_cmp(&other.abs)
            } else {
                Some(Ordering::Less)
            }
        } else if *self >= 0 {
            Some(Ordering::Greater)
        } else {
            other.abs.partial_cmp(&self.unsigned_abs())
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl PartialOrd<Integer> for i32 {
    #[inline]
    fn partial_cmp(&self, other: &Integer) -> Option<Ordering> {
        SignedLimb::from(*self).partial_cmp(other)
    }
}
