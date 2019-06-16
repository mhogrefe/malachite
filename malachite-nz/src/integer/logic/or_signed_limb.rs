use std::ops::{BitOr, BitOrAssign};

use malachite_base::num::conversion::traits::WrappingFrom;

use integer::Integer;
use natural::Natural::{self, Large, Small};
use platform::{Limb, SignedLimb};

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of an `Integer`, returns the
/// negative of the bitwise or of the `Integer` and a negative number whose lowest limb is given by
/// `limb` and whose other limbs are full of `true` bits. The slice cannot be empty or only contain
/// zeros.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Panics
/// Panics if `limbs` is empty.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::or_signed_limb::limbs_pos_or_neg_limb;
///
/// assert_eq!(limbs_pos_or_neg_limb(&[6, 7], 3), 4294967289);
/// assert_eq!(limbs_pos_or_neg_limb(&[100, 101, 102], 10), 4294967186);
/// ```
pub fn limbs_pos_or_neg_limb(limbs: &[Limb], limb: Limb) -> Limb {
    (limbs[0] | limb).wrapping_neg()
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of the negative of an
/// `Integer`, returns the negative of the bitwise or of the `Integer` and a negative number whose
/// lowest limb is given by `limb` and whose other limbs are full of `true` bits. The slice cannot
/// be empty or only contain zeros.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Panics
/// Panics if `limbs` is empty.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::or_signed_limb::limbs_neg_or_neg_limb;
///
/// assert_eq!(limbs_neg_or_neg_limb(&[6, 7], 3), 5);
/// assert_eq!(limbs_neg_or_neg_limb(&[100, 101, 102], 10), 98);
/// ```
pub fn limbs_neg_or_neg_limb(limbs: &[Limb], limb: Limb) -> Limb {
    (limbs[0].wrapping_neg() | limb).wrapping_neg()
}

/// Takes the bitwise or of an `Integer` or a `SignedLimb`, taking the `Integer` by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `self.significant_bits()`
///
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_nz::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((Integer::ZERO | 123i32).to_string(), "123");
///     assert_eq!((Integer::from(123) | 0i32).to_string(), "123");
///     assert_eq!((Integer::from(-123) | -456i32).to_string(), "-67");
///     assert_eq!((Integer::from(123) | -456i32).to_string(), "-389");
///     assert_eq!((Integer::from_str("12345678987654321").unwrap() | -456i32).to_string(), "-327");
///     assert_eq!((Integer::from_str("-12345678987654321").unwrap() | -456i32).to_string(),
///         "-129");
/// }
/// ```
impl BitOr<SignedLimb> for Integer {
    type Output = Integer;

    #[inline]
    fn bitor(mut self, other: SignedLimb) -> Integer {
        self |= other;
        self
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl BitOr<i32> for Integer {
    type Output = Integer;

    #[inline]
    fn bitor(self, other: i32) -> Integer {
        self | SignedLimb::from(other)
    }
}

/// Takes the bitwise or of an `Integer` or a `SignedLimb`, taking the `Integer` by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `self.significant_bits()`
///
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_nz::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((&Integer::ZERO | 123i32).to_string(), "123");
///     assert_eq!((&Integer::from(123) | 0i32).to_string(), "123");
///     assert_eq!((&Integer::from(-123) | -456i32).to_string(), "-67");
///     assert_eq!((&Integer::from(123) | -456i32).to_string(), "-389");
///     assert_eq!((&Integer::from_str("12345678987654321").unwrap() | -456i32).to_string(),
///         "-327");
///     assert_eq!((&Integer::from_str("-12345678987654321").unwrap() | -456i32).to_string(),
///         "-129");
/// }
/// ```
impl<'a> BitOr<SignedLimb> for &'a Integer {
    type Output = Integer;

    fn bitor(self, other: SignedLimb) -> Integer {
        let other_non_neg = other >= 0;
        let other = Limb::wrapping_from(other);
        if other_non_neg {
            self | other
        } else {
            Integer {
                sign: false,
                abs: if self.sign {
                    self.abs.or_pos_limb_neg(other)
                } else {
                    self.abs.or_neg_limb_neg(other)
                },
            }
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> BitOr<i32> for &'a Integer {
    type Output = Integer;

    #[inline]
    fn bitor(self, other: i32) -> Integer {
        self | SignedLimb::from(other)
    }
}

/// Takes the bitwise or of a `SignedLimb` or an `Integer`, taking the `Integer` by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `other.significant_bits()`
///
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_nz::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((123i32 | Integer::ZERO).to_string(), "123");
///     assert_eq!((0i32 | Integer::from(123)).to_string(), "123");
///     assert_eq!((-456i32 | Integer::from(-123)).to_string(), "-67");
///     assert_eq!((-456i32 | Integer::from(123)).to_string(), "-389");
///     assert_eq!((-456i32 | Integer::from_str("12345678987654321").unwrap()).to_string(), "-327");
///     assert_eq!((-456i32 | Integer::from_str("-12345678987654321").unwrap()).to_string(),
///         "-129");
/// }
/// ```
impl BitOr<Integer> for SignedLimb {
    type Output = Integer;

    #[inline]
    fn bitor(self, other: Integer) -> Integer {
        other | self
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl BitOr<Integer> for i32 {
    type Output = Integer;

    #[inline]
    fn bitor(self, other: Integer) -> Integer {
        SignedLimb::from(self) | other
    }
}

/// Takes the bitwise or of a `SignedLimb` or an `Integer`, taking the `Integer` by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `other.significant_bits()`
///
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_nz::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((123i32 | &Integer::ZERO).to_string(), "123");
///     assert_eq!((0i32 | &Integer::from(123)).to_string(), "123");
///     assert_eq!((-456i32 | &Integer::from(-123)).to_string(), "-67");
///     assert_eq!((-456i32 | &Integer::from(123)).to_string(), "-389");
///     assert_eq!((-456i32 | &Integer::from_str("12345678987654321").unwrap()).to_string(),
///         "-327");
///     assert_eq!((-456i32 | &Integer::from_str("-12345678987654321").unwrap()).to_string(),
///         "-129");
/// }
/// ```
impl<'a> BitOr<&'a Integer> for SignedLimb {
    type Output = Integer;

    #[inline]
    fn bitor(self, other: &'a Integer) -> Integer {
        other | self
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> BitOr<&'a Integer> for i32 {
    type Output = Integer;

    #[inline]
    fn bitor(self, other: &'a Integer) -> Integer {
        SignedLimb::from(self) | other
    }
}

/// Bitwise-ors an `Integer` with a `SignedLimb` in place.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `self.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::ZERO;
///     x |= 0x0000_000fi32;
///     x |= 0x0000_0f00i32;
///     x |= 0x000f_0000i32;
///     x |= 0x0f00_0000i32;
///     assert_eq!(x, 0x0f0f_0f0f);
/// }
/// ```
impl BitOrAssign<SignedLimb> for Integer {
    fn bitor_assign(&mut self, other: SignedLimb) {
        let other_non_neg = other >= 0;
        let other = Limb::wrapping_from(other);
        if other_non_neg {
            *self |= other;
        } else if self.sign {
            self.sign = false;
            self.abs.or_assign_pos_limb_neg(other);
        } else {
            self.abs.or_assign_neg_limb_neg(other);
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl BitOrAssign<i32> for Integer {
    #[inline]
    fn bitor_assign(&mut self, other: i32) {
        *self |= SignedLimb::from(other);
    }
}

impl Natural {
    pub(crate) fn or_assign_pos_limb_neg(&mut self, other: Limb) {
        *self = self.or_pos_limb_neg(other);
    }

    pub(crate) fn or_pos_limb_neg(&self, other: Limb) -> Natural {
        Small(match *self {
            Small(small) => (small | other).wrapping_neg(),
            Large(ref limbs) => limbs_pos_or_neg_limb(limbs, other),
        })
    }

    pub(crate) fn or_assign_neg_limb_neg(&mut self, other: Limb) {
        *self = self.or_neg_limb_neg(other);
    }

    pub(crate) fn or_neg_limb_neg(&self, other: Limb) -> Natural {
        Small(match *self {
            Small(small) => (small.wrapping_neg() | other).wrapping_neg(),
            Large(ref limbs) => limbs_neg_or_neg_limb(limbs, other),
        })
    }
}
