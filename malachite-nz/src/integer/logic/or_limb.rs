use std::ops::{BitOr, BitOrAssign};

use malachite_base::comparison::Max;
use malachite_base::limbs::limbs_leading_zero_limbs;

use integer::Integer;
use natural::Natural::{self, Large, Small};
use platform::Limb;

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of the negative of an
/// `Integer`, returns the limbs of the bitwise or of the `Integer` and a `Limb`. `limbs` cannot be
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
/// use malachite_nz::integer::logic::or_limb::limbs_neg_or_limb;
///
/// assert_eq!(limbs_neg_or_limb(&[123, 456], 789), &[107, 456]);
/// assert_eq!(limbs_neg_or_limb(&[0, 0, 456], 789), &[0xffff_fceb, 0xffff_ffff, 455]);
/// ```
pub fn limbs_neg_or_limb(limbs: &[Limb], limb: Limb) -> Vec<Limb> {
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
            *x = Limb::MAX;
        }
        result_limbs[i] = limbs[i] - 1;
        result_limbs[i + 1..].copy_from_slice(&limbs[i + 1..]);
    }
    result_limbs
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of the negative of an
/// `Integer`, writes the limbs of the bitwise or of the `Integer` and a `Limb` to an output slice.
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
/// May panic if `in_limbs` is empty or only contains zeros, or if `out` is shorter than
/// `in_limbs`.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::or_limb::limbs_neg_or_limb_to_out;
///
/// let mut limbs = vec![0, 0, 0, 0];
/// limbs_neg_or_limb_to_out(&mut limbs, &[123, 456], 789);
/// assert_eq!(limbs, &[107, 456, 0, 0]);
///
/// let mut limbs = vec![0, 0, 0, 0];
/// limbs_neg_or_limb_to_out(&mut limbs, &[0, 0, 456], 789);
/// assert_eq!(limbs, &[0xffff_fceb, 0xffff_ffff, 455, 0]);
/// ```
pub fn limbs_neg_or_limb_to_out(out: &mut [Limb], in_limbs: &[Limb], limb: Limb) {
    let len = in_limbs.len();
    assert!(out.len() >= len);
    if limb == 0 {
        out[..len].copy_from_slice(in_limbs);
        return;
    }
    let i = limbs_leading_zero_limbs(in_limbs);
    if i == 0 {
        out[0] = (in_limbs[0].wrapping_neg() | limb).wrapping_neg();
        out[1..len].copy_from_slice(&in_limbs[1..]);
    } else {
        out[0] = limb.wrapping_neg();
        for x in out[1..i].iter_mut() {
            *x = Limb::MAX;
        }
        out[i] = in_limbs[i] - 1;
        out[i + 1..len].copy_from_slice(&in_limbs[i + 1..]);
    }
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of the negative of an
/// `Integer`, writes the limbs of the bitwise or of the `Integer`, writes the limbs of the bitwise
/// or of the `Integer` and a `Limb` to the input slice. `limbs` cannot be empty or only contain
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
/// use malachite_nz::integer::logic::or_limb::limbs_neg_or_limb_in_place;
///
/// let mut limbs = vec![123, 456];
/// limbs_neg_or_limb_in_place(&mut limbs, 789);
/// assert_eq!(limbs, &[107, 456]);
///
/// let mut limbs = vec![0, 0, 456];
/// limbs_neg_or_limb_in_place(&mut limbs, 789);
/// assert_eq!(limbs, &[0xffff_fceb, 0xffff_ffff, 455]);
/// ```
pub fn limbs_neg_or_limb_in_place(limbs: &mut [Limb], limb: Limb) {
    if limb == 0 {
        return;
    }
    let i = limbs_leading_zero_limbs(limbs);
    if i == 0 {
        limbs[0] = (limbs[0].wrapping_neg() | limb).wrapping_neg();
    } else {
        limbs[0] = limb.wrapping_neg();
        for x in limbs[1..i].iter_mut() {
            *x = Limb::MAX;
        }
        limbs[i] -= 1;
    }
}

/// Takes the bitwise or of an `Integer` and a `Limb`, taking the `Integer` by value.
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
/// use malachite_base::num::traits::Zero;
/// use malachite_nz::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((Integer::ZERO | 123u32).to_string(), "123");
///     assert_eq!((Integer::from(123) | 0u32).to_string(), "123");
///     assert_eq!((Integer::from(-123) | 456u32).to_string(), "-51");
/// }
/// ```
impl BitOr<Limb> for Integer {
    type Output = Integer;

    #[inline]
    fn bitor(mut self, other: Limb) -> Integer {
        self |= other;
        self
    }
}

#[cfg(feature = "64_bit_limbs")]
impl BitOr<u32> for Integer {
    type Output = Integer;

    #[inline]
    fn bitor(self, other: u32) -> Integer {
        self | Limb::from(other)
    }
}

/// Takes the bitwise or of an `Integer` and a `Limb`, taking the `Integer` by reference.
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
/// use malachite_base::num::traits::Zero;
/// use malachite_nz::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((&Integer::ZERO | 123u32).to_string(), "123");
///     assert_eq!((&Integer::from(123) | 0u32).to_string(), "123");
///     assert_eq!((&Integer::from(-123) | 456u32).to_string(), "-51");
/// }
/// ```
impl<'a> BitOr<Limb> for &'a Integer {
    type Output = Integer;

    fn bitor(self, other: Limb) -> Integer {
        Integer {
            sign: self.sign,
            abs: if self.sign {
                &self.abs | other
            } else {
                self.abs.or_neg_limb_pos(other)
            },
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> BitOr<u32> for &'a Integer {
    type Output = Integer;

    #[inline]
    fn bitor(self, other: u32) -> Integer {
        self | Limb::from(other)
    }
}

/// Takes the bitwise or of a `Limb` and an `Integer`, taking the `Integer` by value.
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
/// use malachite_base::num::traits::Zero;
/// use malachite_nz::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((123u32 | Integer::ZERO).to_string(), "123");
///     assert_eq!((0u32 | Integer::from(123)).to_string(), "123");
///     assert_eq!((456u32 | Integer::from(-123)).to_string(), "-51");
/// }
/// ```
impl BitOr<Integer> for Limb {
    type Output = Integer;

    #[inline]
    fn bitor(self, other: Integer) -> Integer {
        other | self
    }
}

#[cfg(feature = "64_bit_limbs")]
impl BitOr<Integer> for u32 {
    type Output = Integer;

    #[inline]
    fn bitor(self, other: Integer) -> Integer {
        Limb::from(self) | other
    }
}

/// Takes the bitwise or of a `Limb` and an `Integer`, taking the `Integer` by reference.
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
/// use malachite_base::num::traits::Zero;
/// use malachite_nz::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((123u32 | &Integer::ZERO).to_string(), "123");
///     assert_eq!((0u32 | &Integer::from(123)).to_string(), "123");
///     assert_eq!((456u32 | &Integer::from(-123)).to_string(), "-51");
/// }
/// ```
impl<'a> BitOr<&'a Integer> for Limb {
    type Output = Integer;

    fn bitor(self, other: &'a Integer) -> Integer {
        other | self
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> BitOr<&'a Integer> for u32 {
    type Output = Integer;

    #[inline]
    fn bitor(self, other: &'a Integer) -> Integer {
        Limb::from(self) | other
    }
}

/// Bitwise-ors an `Integer` with a `Limb` in place.
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
/// use malachite_base::num::traits::Zero;
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
impl BitOrAssign<Limb> for Integer {
    fn bitor_assign(&mut self, other: Limb) {
        if self.sign {
            self.abs |= other;
        } else {
            self.abs.or_assign_neg_limb_pos(other);
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl BitOrAssign<u32> for Integer {
    #[inline]
    fn bitor_assign(&mut self, other: u32) {
        *self |= Limb::from(other);
    }
}

impl Natural {
    pub(crate) fn or_assign_neg_limb_pos(&mut self, other: Limb) {
        match *self {
            Small(ref mut small) => {
                *small = (small.wrapping_neg() | other).wrapping_neg();
                return;
            }
            Large(ref mut limbs) => limbs_neg_or_limb_in_place(limbs, other),
        }
        self.trim();
    }

    pub(crate) fn or_neg_limb_pos(&self, other: Limb) -> Natural {
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
