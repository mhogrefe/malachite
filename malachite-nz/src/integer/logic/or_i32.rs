use integer::Integer;
use malachite_base::misc::WrappingFrom;
use malachite_base::num::UnsignedAbs;
use natural::Natural::{self, Large, Small};
use std::ops::{BitOr, BitOrAssign};

/// Interpreting a slice of `u32`s as the limbs (in ascending order) of an `Integer`, returns the
/// negative of the bitwise and of the `Integer` and a negative `i32`. The slice cannot be empty or
/// only contain zeros.
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
/// use malachite_nz::integer::logic::or_i32::limbs_pos_or_neg_i32;
///
/// assert_eq!(limbs_pos_or_neg_i32(&[6, 7], -3), 1);
/// assert_eq!(limbs_pos_or_neg_i32(&[100, 101, 102], -10), 10);
/// ```
pub fn limbs_pos_or_neg_i32(limbs: &[u32], limb: i32) -> u32 {
    (limbs[0] | u32::wrapping_from(limb)).wrapping_neg()
}

/// Interpreting a slice of `u32`s as the limbs (in ascending order) of the negative of an
/// `Integer`, returns the negative of the bitwise and of the `Integer` and a negative `i32`. The
/// slice cannot be empty or only contain zeros.
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
/// use malachite_nz::integer::logic::or_i32::limbs_neg_or_neg_i32;
///
/// assert_eq!(limbs_neg_or_neg_i32(&[6, 7], -3), 1);
/// assert_eq!(limbs_neg_or_neg_i32(&[100, 101, 102], -10), 2);
/// ```
pub fn limbs_neg_or_neg_i32(limbs: &[u32], limb: i32) -> u32 {
    (limbs[0].wrapping_neg() | u32::wrapping_from(limb)).wrapping_neg()
}

/// Takes the bitwise or of an `Integer` or an `i32`, taking the `Integer` by value.
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
/// use malachite_base::num::Zero;
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
impl BitOr<i32> for Integer {
    type Output = Integer;

    fn bitor(mut self, other: i32) -> Integer {
        self |= other;
        self
    }
}

/// Takes the bitwise or of an `Integer` or an `i32`, taking the `Integer` by reference.
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
/// use malachite_base::num::Zero;
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
impl<'a> BitOr<i32> for &'a Integer {
    type Output = Integer;

    fn bitor(self, other: i32) -> Integer {
        if other >= 0 {
            self | other.unsigned_abs()
        } else {
            Integer {
                sign: false,
                abs: if self.sign {
                    self.abs.or_pos_i32_neg(other)
                } else {
                    self.abs.or_neg_i32_neg(other)
                },
            }
        }
    }
}

/// Takes the bitwise or of an `i32` or an `Integer`, taking the `Integer` by value.
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
/// use malachite_base::num::Zero;
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
impl BitOr<Integer> for i32 {
    type Output = Integer;

    fn bitor(self, other: Integer) -> Integer {
        other | self
    }
}

/// Takes the bitwise or of an `i32` or an `Integer`, taking the `Integer` by reference.
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
/// use malachite_base::num::Zero;
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
impl<'a> BitOr<&'a Integer> for i32 {
    type Output = Integer;

    fn bitor(self, other: &'a Integer) -> Integer {
        other | self
    }
}

/// Bitwise-ors an `Integer` with an `i32` in place.
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
/// use malachite_base::num::Zero;
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
impl BitOrAssign<i32> for Integer {
    fn bitor_assign(&mut self, other: i32) {
        if other >= 0 {
            *self |= other.unsigned_abs();
        } else if self.sign {
            self.sign = false;
            self.abs.or_assign_pos_i32_neg(other);
        } else {
            self.abs.or_assign_neg_i32_neg(other);
        }
    }
}

impl Natural {
    fn or_assign_pos_i32_neg(&mut self, other: i32) {
        *self = self.or_pos_i32_neg(other);
    }

    fn or_pos_i32_neg(&self, other: i32) -> Natural {
        Small(match *self {
            Small(small) => (small | u32::wrapping_from(other)).wrapping_neg(),
            Large(ref limbs) => limbs_pos_or_neg_i32(limbs, other),
        })
    }

    fn or_assign_neg_i32_neg(&mut self, other: i32) {
        *self = self.or_neg_i32_neg(other);
    }

    fn or_neg_i32_neg(&self, other: i32) -> Natural {
        Small(match *self {
            Small(small) => (small.wrapping_neg() | u32::wrapping_from(other)).wrapping_neg(),
            Large(ref limbs) => limbs_neg_or_neg_i32(limbs, other),
        })
    }
}
