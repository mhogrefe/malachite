use integer::Integer;
use malachite_base::limbs::limbs_leading_zero_limbs;
use natural::Natural::{self, Large, Small};
use std::ops::{BitOr, BitOrAssign};
use std::u32;

/// Interpreting a slice of `u32`s as the limbs (in ascending order) of the negative of an
/// `Integer`, returns the limbs of the bitwise or of the `Integer` and a `u32`. `limbs` cannot be
/// empty or only contain zeros.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `limbs.len()`
///
/// # Panics
/// May panic if `limbs` is empty or only contains zeros.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::or_u32::limbs_neg_or_limb;
///
/// assert_eq!(limbs_neg_or_limb(&[123, 456], 789), &[107, 456]);
/// assert_eq!(limbs_neg_or_limb(&[0, 0, 456], 789), &[0xffff_fceb, 0xffff_ffff, 455]);
/// ```
pub fn limbs_neg_or_limb(limbs: &[u32], limb: u32) -> Vec<u32> {
    if limb == 0 {
        return limbs.to_vec();
    }
    let mut result_limbs = vec![0; limbs.len()];
    let i = limbs_leading_zero_limbs(limbs);
    if i == 0 {
        result_limbs[0] = (limbs[0].wrapping_neg() | limb).wrapping_neg();
        result_limbs[1..].copy_from_slice(&limbs[1..]);
    } else {
        result_limbs[0] = limb.wrapping_neg();
        for x in result_limbs[1..i].iter_mut() {
            *x = u32::MAX;
        }
        result_limbs[i] = limbs[i] - 1;
        result_limbs[i + 1..].copy_from_slice(&limbs[i + 1..]);
    }
    result_limbs
}

/// Interpreting a slice of `u32`s as the limbs (in ascending order) of the negative of an
/// `Integer`, writes the limbs of the bitwise or of the `Integer` and a `u32` to an output slice.
/// The output slice must be at least as long as the input slice. `limbs` cannot be empty or only
/// contain zeros.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `in_limbs.len()`
///
/// # Panics
/// May panic if `in_limbs` is empty or only contains zeros, or if `out_limbs` is shorter than
/// `in_limbs`.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::or_u32::limbs_neg_or_limb_to_out;
///
/// let mut limbs = vec![0, 0, 0, 0];
/// limbs_neg_or_limb_to_out(&mut limbs, &[123, 456], 789);
/// assert_eq!(limbs, &[107, 456, 0, 0]);
///
/// let mut limbs = vec![0, 0, 0, 0];
/// limbs_neg_or_limb_to_out(&mut limbs, &[0, 0, 456], 789);
/// assert_eq!(limbs, &[0xffff_fceb, 0xffff_ffff, 455, 0]);
/// ```
pub fn limbs_neg_or_limb_to_out(out_limbs: &mut [u32], in_limbs: &[u32], limb: u32) {
    let len = in_limbs.len();
    assert!(out_limbs.len() >= len);
    if limb == 0 {
        out_limbs[..len].copy_from_slice(in_limbs);
        return;
    }
    let i = limbs_leading_zero_limbs(in_limbs);
    if i == 0 {
        out_limbs[0] = (in_limbs[0].wrapping_neg() | limb).wrapping_neg();
        out_limbs[1..len].copy_from_slice(&in_limbs[1..]);
    } else {
        out_limbs[0] = limb.wrapping_neg();
        for x in out_limbs[1..i].iter_mut() {
            *x = u32::MAX;
        }
        out_limbs[i] = in_limbs[i] - 1;
        out_limbs[i + 1..len].copy_from_slice(&in_limbs[i + 1..]);
    }
}

/// Interpreting a slice of `u32`s as the limbs (in ascending order) of the negative of an
/// `Integer`, writes the limbs of the bitwise or of the `Integer`, writes the limbs of the bitwise
/// or of the `Integer` and a `u32` to the input slice. `limbs` cannot be empty or only contain
/// zeros.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Panics
/// May panic if `limbs` is empty or only contains zeros.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::or_u32::limbs_neg_or_limb_in_place;
///
/// let mut limbs = vec![123, 456];
/// limbs_neg_or_limb_in_place(&mut limbs, 789);
/// assert_eq!(limbs, &[107, 456]);
///
/// let mut limbs = vec![0, 0, 456];
/// limbs_neg_or_limb_in_place(&mut limbs, 789);
/// assert_eq!(limbs, &[0xffff_fceb, 0xffff_ffff, 455]);
/// ```
pub fn limbs_neg_or_limb_in_place(limbs: &mut [u32], limb: u32) {
    if limb == 0 {
        return;
    }
    let i = limbs_leading_zero_limbs(limbs);
    if i == 0 {
        limbs[0] = (limbs[0].wrapping_neg() | limb).wrapping_neg();
    } else {
        limbs[0] = limb.wrapping_neg();
        for x in limbs[1..i].iter_mut() {
            *x = u32::MAX;
        }
        limbs[i] -= 1;
    }
}

/// Takes the bitwise or of an `Integer` and a `u32`, taking the `Integer` by value.
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
///     assert_eq!((Integer::ZERO | 123u32).to_string(), "123");
///     assert_eq!((Integer::from(123) | 0u32).to_string(), "123");
///     assert_eq!((Integer::from(-123) | 456u32).to_string(), "-51");
/// }
/// ```
impl BitOr<u32> for Integer {
    type Output = Integer;

    fn bitor(mut self, other: u32) -> Integer {
        self |= other;
        self
    }
}

/// Takes the bitwise or of an `Integer` and a `u32`, taking the `Integer` by reference.
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
///     assert_eq!((&Integer::ZERO | 123u32).to_string(), "123");
///     assert_eq!((&Integer::from(123) | 0u32).to_string(), "123");
///     assert_eq!((&Integer::from(-123) | 456u32).to_string(), "-51");
/// }
/// ```
impl<'a> BitOr<u32> for &'a Integer {
    type Output = Integer;

    fn bitor(self, other: u32) -> Integer {
        Integer {
            sign: self.sign,
            abs: if self.sign {
                &self.abs | other
            } else {
                self.abs.or_neg_u32_pos(other)
            },
        }
    }
}

/// Takes the bitwise or of a `u32` and an `Integer`, taking the `Integer` by value.
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
///     assert_eq!((123u32 | Integer::ZERO).to_string(), "123");
///     assert_eq!((0u32 | Integer::from(123)).to_string(), "123");
///     assert_eq!((456u32 | Integer::from(-123)).to_string(), "-51");
/// }
/// ```
impl BitOr<Integer> for u32 {
    type Output = Integer;

    fn bitor(self, other: Integer) -> Integer {
        other | self
    }
}

/// Takes the bitwise or of a `u32` and an `Integer`, taking the `Integer` by reference.
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
///     assert_eq!((123u32 | &Integer::ZERO).to_string(), "123");
///     assert_eq!((0u32 | &Integer::from(123)).to_string(), "123");
///     assert_eq!((456u32 | &Integer::from(-123)).to_string(), "-51");
/// }
/// ```
impl<'a> BitOr<&'a Integer> for u32 {
    type Output = Integer;

    fn bitor(self, other: &'a Integer) -> Integer {
        other | self
    }
}

/// Bitwise-ors an `Integer` with a `u32` in place.
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
///     x |= 0x0000_000f;
///     x |= 0x0000_0f00;
///     x |= 0x000f_0000;
///     x |= 0x0f00_0000;
///     assert_eq!(x, 0x0f0f_0f0f);
/// }
/// ```
impl BitOrAssign<u32> for Integer {
    fn bitor_assign(&mut self, other: u32) {
        if self.sign {
            self.abs |= other;
        } else {
            self.abs.or_assign_neg_u32_pos(other);
        }
    }
}

impl Natural {
    pub(crate) fn or_assign_neg_u32_pos(&mut self, other: u32) {
        match *self {
            Small(ref mut small) => {
                *small = (small.wrapping_neg() | other).wrapping_neg();
                return;
            }
            Large(ref mut limbs) => limbs_neg_or_limb_in_place(limbs, other),
        }
        self.trim();
    }

    pub(crate) fn or_neg_u32_pos(&self, other: u32) -> Natural {
        match *self {
            Small(ref small) => Small((small.wrapping_neg() | other).wrapping_neg()),
            Large(ref limbs) => {
                let mut result = Large(limbs_neg_or_limb(limbs, other));
                result.trim();
                result
            }
        }
    }
}
