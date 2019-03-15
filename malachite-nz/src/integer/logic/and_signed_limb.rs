use integer::Integer;
use malachite_base::misc::WrappingFrom;
use malachite_base::num::{UnsignedAbs, WrappingNegAssign};
use natural::arithmetic::add_limb::{limbs_add_limb_to_out, limbs_slice_add_limb_in_place};
use natural::Natural::{self, Large, Small};
use platform::{Limb, SignedLimb};
use std::ops::{BitAnd, BitAndAssign};

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of an `Integer`, returns the
/// limbs of the bitwise and of the `Integer` and a negative number whose lowest limb is given by
/// `limb` and whose other limbs are full of `true` bits. `limbs` may not be empty.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if `limbs` is empty.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::and_signed_limb::limbs_pos_and_limb_neg;
///
/// assert_eq!(limbs_pos_and_limb_neg(&[0, 2], 3), &[0, 2]);
/// assert_eq!(limbs_pos_and_limb_neg(&[123, 456], 789), &[17, 456]);
/// ```
pub fn limbs_pos_and_limb_neg(limbs: &[Limb], limb: Limb) -> Vec<Limb> {
    let mut result_limbs = limbs.to_vec();
    result_limbs[0] &= limb;
    result_limbs
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of an `Integer`, writes the
/// limbs of the bitwise and of the `Integer` and a negative number whose lowest limb is given by
/// `limb` and whose other limbs are full of `true` bits, to an output slice. `in_limbs` may not be
/// empty. The output slice must be at least as long as the input slice.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `in_limbs.len()`
///
/// # Panics
/// Panics if `in_limbs` is empty or if `out` is shorter than `in_limbs`.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::and_signed_limb::limbs_pos_and_limb_neg_to_out;
///
/// let mut result = vec![10, 10];
/// limbs_pos_and_limb_neg_to_out(&mut result, &[0, 2], 3);
/// assert_eq!(result, &[0, 2]);
///
/// let mut result = vec![10, 10, 10, 10];
/// limbs_pos_and_limb_neg_to_out(&mut result, &[123, 456], 789);
/// assert_eq!(result, &[17, 456, 10, 10]);
/// ```
pub fn limbs_pos_and_limb_neg_to_out(out: &mut [Limb], in_limbs: &[Limb], limb: Limb) {
    let len = in_limbs.len();
    assert!(out.len() >= len);
    out[0] = in_limbs[0] & limb;
    out[1..len].copy_from_slice(&in_limbs[1..]);
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of an `Integer`, writes the
/// limbs of the bitwise and of the `Integer` and a negative number whose lowest limb is given by
/// `limb` and whose other limbs are full of `true` bits, to the input slice. `limbs` may not be
/// empty.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Panics
/// Panics if `in_limbs` is empty.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::and_signed_limb::limbs_pos_and_limb_neg_in_place;
///
/// let mut limbs = vec![0, 2];
/// limbs_pos_and_limb_neg_in_place(&mut limbs, 3);
/// assert_eq!(limbs, &[0, 2]);
///
/// let mut limbs = vec![123, 456];
/// limbs_pos_and_limb_neg_in_place(&mut limbs, 789);
/// assert_eq!(limbs, &[17, 456]);
/// ```
pub fn limbs_pos_and_limb_neg_in_place(limbs: &mut [Limb], limb: Limb) {
    limbs[0] &= limb;
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of the negative of an
/// `Integer`, returns the limbs of the bitwise and of the `Integer` and a negative number whose
/// lowest limb is given by `limb` and whose other limbs are full of `true` bits. `limbs` may not be
/// empty or only contain zeros.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if `limbs` is empty.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::and_signed_limb::limbs_neg_and_limb_neg;
///
/// assert_eq!(limbs_neg_and_limb_neg(&[0, 2], 3), &[0, 2]);
/// assert_eq!(limbs_neg_and_limb_neg(&[1, 1], 3), &[4294967293, 1]);
/// assert_eq!(limbs_neg_and_limb_neg(&[0xffff_fffe, 1], 1), &[0, 2]);
/// assert_eq!(limbs_neg_and_limb_neg(&[0xffff_fffe, 0xffff_ffff], 1), &[0, 0, 1]);
/// ```
pub fn limbs_neg_and_limb_neg(limbs: &[Limb], limb: Limb) -> Vec<Limb> {
    let mut result_limbs = limbs.to_vec();
    limbs_vec_neg_and_limb_neg_in_place(&mut result_limbs, limb);
    result_limbs
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of the negative of an
/// `Integer`, writes the limbs of the bitwise and of the `Integer` and a negative number whose
/// lowest limb is given by `limb` and whose other limbs are full of `true` bits to an output slice.
/// `in_limbs` may not be empty or only contain zeros. Returns whether a carry occurs. The output
/// slice must be at least as long as the input slice.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `in_limbs.len()`
///
/// # Panics
/// Panics if `in_limbs` is empty or if `out` is shorter than `in_limbs`.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::and_signed_limb::limbs_neg_and_limb_neg_to_out;
///
/// let mut result = vec![0, 0];
/// assert_eq!(limbs_neg_and_limb_neg_to_out(&mut result, &[0, 2], 3), false);
/// assert_eq!(result, &[0, 2]);
///
/// let mut result = vec![0, 0];
/// assert_eq!(limbs_neg_and_limb_neg_to_out(&mut result, &[1, 1], 3), false);
/// assert_eq!(result, &[4294967293, 1]);
///
/// let mut result = vec![0, 0];
/// assert_eq!(limbs_neg_and_limb_neg_to_out(&mut result, &[0xffff_fffe, 1], 1), false);
/// assert_eq!(result, &[0, 2]);
///
/// let mut result = vec![0, 0];
/// assert_eq!(limbs_neg_and_limb_neg_to_out(&mut result, &[0xffff_fffe, 0xffff_ffff], 1),
///         true);
/// assert_eq!(result, &[0, 0]);
/// ```
pub fn limbs_neg_and_limb_neg_to_out(out: &mut [Limb], in_limbs: &[Limb], limb: Limb) -> bool {
    assert!(out.len() >= in_limbs.len());
    if in_limbs[0] == 0 {
        out[..in_limbs.len()].copy_from_slice(in_limbs);
        false
    } else {
        let result_head = in_limbs[0].wrapping_neg() & limb;
        if result_head == 0 {
            out[0] = 0;
            limbs_add_limb_to_out(&mut out[1..], &in_limbs[1..], 1)
        } else {
            out[0] = result_head.wrapping_neg();
            out[1..in_limbs.len()].copy_from_slice(&in_limbs[1..]);
            false
        }
    }
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of the negative of an
/// `Integer`, takes the bitwise and of the `Integer` and a negative number whose lowest limb is
/// given by `limb` and whose other limbs are full of `true` bits, in place. `limbs` may not be
/// empty or only contain zeros. Returns whether there is a carry.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if `limbs` is empty.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::and_signed_limb::limbs_slice_neg_and_limb_neg_in_place;
///
/// let mut limbs = vec![0, 2];
/// assert_eq!(limbs_slice_neg_and_limb_neg_in_place(&mut limbs, 3), false);
/// assert_eq!(limbs, &[0, 2]);
///
/// let mut limbs = vec![1, 1];
/// assert_eq!(limbs_slice_neg_and_limb_neg_in_place(&mut limbs, 3), false);
/// assert_eq!(limbs, &[4294967293, 1]);
///
/// let mut limbs = vec![0xffff_fffe, 1];
/// assert_eq!(limbs_slice_neg_and_limb_neg_in_place(&mut limbs, 1), false);
/// assert_eq!(limbs, &[0, 2]);
///
/// let mut limbs = vec![0xffff_fffe, 0xffff_ffff];
/// assert_eq!(limbs_slice_neg_and_limb_neg_in_place(&mut limbs, 1), true);
/// assert_eq!(limbs, &[0, 0]);
/// ```
pub fn limbs_slice_neg_and_limb_neg_in_place(limbs: &mut [Limb], limb: Limb) -> bool {
    let (head, tail) = limbs.split_at_mut(1);
    let head = &mut head[0];
    if *head == 0 {
        false
    } else {
        *head = head.wrapping_neg() & limb;
        if *head == 0 {
            limbs_slice_add_limb_in_place(tail, 1)
        } else {
            head.wrapping_neg_assign();
            false
        }
    }
}

/// Interpreting a `Vec` of `Limb`s as the limbs (in ascending order) of the negative of an
/// `Integer`, takes the bitwise and of the `Integer` and a negative number whose lowest limb is
/// given by `limb` and whose other limbs are full of `true` bits, in place. `limbs` may not be
/// empty or only contain zeros. If there is a carry, increases the length of the `Vec` by 1.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if `limbs` is empty.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::and_signed_limb::limbs_vec_neg_and_limb_neg_in_place;
///
/// let mut limbs = vec![0, 2];
/// limbs_vec_neg_and_limb_neg_in_place(&mut limbs, 3);
/// assert_eq!(limbs, &[0, 2]);
///
/// let mut limbs = vec![1, 1];
/// limbs_vec_neg_and_limb_neg_in_place(&mut limbs, 3);
/// assert_eq!(limbs, &[4294967293, 1]);
///
/// let mut limbs = vec![0xffff_fffe, 1];
/// limbs_vec_neg_and_limb_neg_in_place(&mut limbs, 1);
/// assert_eq!(limbs, &[0, 2]);
///
/// let mut limbs = vec![0xffff_fffe, 0xffff_ffff];
/// limbs_vec_neg_and_limb_neg_in_place(&mut limbs, 1);
/// assert_eq!(limbs, &[0, 0, 1]);
/// ```
pub fn limbs_vec_neg_and_limb_neg_in_place(limbs: &mut Vec<Limb>, limb: Limb) {
    if limbs_slice_neg_and_limb_neg_in_place(limbs, limb) {
        limbs.push(1)
    }
}

/// Takes the bitwise and of an `Integer` and a `SignedLimb`, taking the `Integer` by value.
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
///     assert_eq!((Integer::ZERO & 123i32).to_string(), "0");
///     assert_eq!((Integer::from(123) & 0i32).to_string(), "0");
///     assert_eq!((Integer::from(-123) & -456i32).to_string(), "-512");
///     assert_eq!((Integer::from(123) & -456i32).to_string(), "56");
///     assert_eq!((-Integer::from(0xffff_ffffu32) & 2i32).to_string(), "0");
///     assert_eq!((Integer::from_str("-12345678987654321").unwrap() & -456i32).to_string(),
///         "-12345678987654648");
/// }
/// ```
impl BitAnd<SignedLimb> for Integer {
    type Output = Integer;

    #[inline]
    fn bitand(mut self, other: SignedLimb) -> Integer {
        self &= other;
        self
    }
}

#[cfg(feature = "64_bit_limbs")]
impl BitAnd<i32> for Integer {
    type Output = Integer;

    #[inline]
    fn bitand(self, other: i32) -> Integer {
        self & SignedLimb::from(other)
    }
}

/// Takes the bitwise and of an `Integer` and a `SignedLimb`, taking the `Integer` by reference.
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
///     assert_eq!((&Integer::ZERO & 123i32).to_string(), "0");
///     assert_eq!((&Integer::from(123) & 0i32).to_string(), "0");
///     assert_eq!((&Integer::from(-123) & -456i32).to_string(), "-512");
///     assert_eq!((&Integer::from(123) & -456i32).to_string(), "56");
///     assert_eq!((&-Integer::from(0xffff_ffffu32) & 2i32).to_string(), "0");
///     assert_eq!((&Integer::from_str("-12345678987654321").unwrap() & -456i32).to_string(),
///         "-12345678987654648");
/// }
/// ```
impl<'a> BitAnd<SignedLimb> for &'a Integer {
    type Output = Integer;

    fn bitand(self, other: SignedLimb) -> Integer {
        let u_other = Limb::wrapping_from(other);
        if other >= 0 {
            Integer {
                sign: true,
                abs: Small(self & u_other),
            }
        } else {
            Integer {
                sign: self.sign,
                abs: if self.sign {
                    self.abs.and_pos_limb_neg(u_other)
                } else {
                    self.abs.and_neg_limb_neg(u_other)
                },
            }
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> BitAnd<i32> for &'a Integer {
    type Output = Integer;

    #[inline]
    fn bitand(self, other: i32) -> Integer {
        self & SignedLimb::from(other)
    }
}

/// Takes the bitwise and of a `SignedLimb` and an `Integer`, taking the `Integer` by value.
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
///     assert_eq!((123i32 & Integer::ZERO).to_string(), "0");
///     assert_eq!((0i32 & Integer::from(123)).to_string(), "0");
///     assert_eq!((-456i32 & Integer::from(-123)).to_string(), "-512");
///     assert_eq!((-456i32 & Integer::from(123)).to_string(), "56");
///     assert_eq!((2i32 & -Integer::from(0xffff_ffffu32)).to_string(), "0");
///     assert_eq!((-456i32 & Integer::from_str("-12345678987654321").unwrap()).to_string(),
///         "-12345678987654648");
/// }
/// ```
impl BitAnd<Integer> for SignedLimb {
    type Output = Integer;

    #[inline]
    fn bitand(self, other: Integer) -> Integer {
        other & self
    }
}

#[cfg(feature = "64_bit_limbs")]
impl BitAnd<Integer> for i32 {
    type Output = Integer;

    #[inline]
    fn bitand(self, other: Integer) -> Integer {
        SignedLimb::from(self) & other
    }
}

/// Takes the bitwise and of a `SignedLimb` and an `Integer`, taking the `Integer` by reference.
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
///     assert_eq!((123i32 & &Integer::ZERO).to_string(), "0");
///     assert_eq!((0i32 & &Integer::from(123)).to_string(), "0");
///     assert_eq!((-456i32 & &Integer::from(-123)).to_string(), "-512");
///     assert_eq!((-456i32 & &Integer::from(123)).to_string(), "56");
///     assert_eq!((2i32 & &-Integer::from(0xffff_ffffu32)).to_string(), "0");
///     assert_eq!((-456i32 & &Integer::from_str("-12345678987654321").unwrap()).to_string(),
///         "-12345678987654648");
/// }
/// ```
impl<'a> BitAnd<&'a Integer> for SignedLimb {
    type Output = Integer;

    #[inline]
    fn bitand(self, other: &'a Integer) -> Integer {
        other & self
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> BitAnd<&'a Integer> for i32 {
    type Output = Integer;

    #[inline]
    fn bitand(self, other: &'a Integer) -> Integer {
        SignedLimb::from(self) & other
    }
}

/// Bitwise-ands an `Integer` with a `SignedLimb` in place.
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
/// use malachite_base::num::NegativeOne;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::NEGATIVE_ONE;
///     x &= 0x70ff_ffff;
///     x &= 0x7ff0_ffff;
///     x &= 0x7fff_f0ff;
///     x &= 0x7fff_fff0;
///     assert_eq!(x, 0x70f0f0f0);
/// }
/// ```
impl BitAndAssign<SignedLimb> for Integer {
    fn bitand_assign(&mut self, other: SignedLimb) {
        if other >= 0 {
            *self &= other.unsigned_abs();
        } else if self.sign {
            self.abs.and_assign_pos_limb_neg(Limb::wrapping_from(other));
        } else {
            self.abs.and_assign_neg_limb_neg(Limb::wrapping_from(other));
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl BitAndAssign<i32> for Integer {
    #[inline]
    fn bitand_assign(&mut self, other: i32) {
        *self &= SignedLimb::from(other);
    }
}

impl Natural {
    pub(crate) fn and_assign_pos_limb_neg(&mut self, other: Limb) {
        match *self {
            Small(ref mut small) => *small &= other,
            Large(ref mut limbs) => limbs_pos_and_limb_neg_in_place(limbs, other),
        }
    }

    pub(crate) fn and_pos_limb_neg(&self, other: Limb) -> Natural {
        match *self {
            Small(small) => Small(small & other),
            Large(ref limbs) => Large(limbs_pos_and_limb_neg(limbs, other)),
        }
    }

    pub(crate) fn and_assign_neg_limb_neg(&mut self, other: Limb) {
        if *self == 0 {
            return;
        }
        mutate_with_possible_promotion!(
            self,
            small,
            limbs,
            {
                let result = small.wrapping_neg() & other;
                if result == 0 {
                    None
                } else {
                    Some(result.wrapping_neg())
                }
            },
            { limbs_vec_neg_and_limb_neg_in_place(limbs, other) }
        );
    }

    pub(crate) fn and_neg_limb_neg(&self, other: Limb) -> Natural {
        match *self {
            Small(small) => {
                let result = small.wrapping_neg() & other;
                if result == 0 {
                    Large(vec![0, 1])
                } else {
                    Small(result.wrapping_neg())
                }
            }
            Large(ref limbs) => Large(limbs_neg_and_limb_neg(limbs, other)),
        }
    }
}
