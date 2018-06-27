use integer::Integer;
use malachite_base::misc::WrappingFrom;
use malachite_base::num::{UnsignedAbs, WrappingNegAssign};
use natural::arithmetic::add_u32::{mpn_add_1_in_place, mpn_add_1_to_out};
use natural::Natural::{self, Large, Small};
use std::ops::{BitAnd, BitAndAssign};

/// Interpreting a slice of `u32`s as the limbs (in ascending order) of an `Integer`, returns the
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
/// use malachite_nz::integer::logic::and_i32::limbs_pos_and_limb_neg;
///
/// assert_eq!(limbs_pos_and_limb_neg(&[0, 2], 3), &[0, 2]);
/// assert_eq!(limbs_pos_and_limb_neg(&[123, 456], 789), &[17, 456]);
/// ```
pub fn limbs_pos_and_limb_neg(limbs: &[u32], limb: u32) -> Vec<u32> {
    let mut result_limbs = limbs.to_vec();
    result_limbs[0] &= limb;
    result_limbs
}

/// Interpreting a slice of `u32`s as the limbs (in ascending order) of an `Integer`, writes the
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
/// Panics if `in_limbs` is empty or if `out_limbs` is shorter than `in_limbs`.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::and_i32::limbs_pos_and_limb_neg_to_out;
///
/// let mut result = vec![10, 10];
/// limbs_pos_and_limb_neg_to_out(&mut result, &[0, 2], 3);
/// assert_eq!(result, &[0, 2]);
///
/// let mut result = vec![10, 10, 10, 10];
/// limbs_pos_and_limb_neg_to_out(&mut result, &[123, 456], 789);
/// assert_eq!(result, &[17, 456, 10, 10]);
/// ```
pub fn limbs_pos_and_limb_neg_to_out(out_limbs: &mut [u32], in_limbs: &[u32], limb: u32) {
    let len = in_limbs.len();
    assert!(out_limbs.len() >= len);
    out_limbs[0] = in_limbs[0] & limb;
    out_limbs[1..len].copy_from_slice(&in_limbs[1..]);
}

/// Interpreting a slice of `u32`s as the limbs (in ascending order) of an `Integer`, writes the
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
/// use malachite_nz::integer::logic::and_i32::limbs_pos_and_limb_neg_in_place;
///
/// let mut limbs = vec![0, 2];
/// limbs_pos_and_limb_neg_in_place(&mut limbs, 3);
/// assert_eq!(limbs, &[0, 2]);
///
/// let mut limbs = vec![123, 456];
/// limbs_pos_and_limb_neg_in_place(&mut limbs, 789);
/// assert_eq!(limbs, &[17, 456]);
/// ```
pub fn limbs_pos_and_limb_neg_in_place(limbs: &mut [u32], limb: u32) {
    limbs[0] &= limb;
}

/// Interpreting a slice of `u32`s as the limbs (in ascending order) of the negative of an
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
/// use malachite_nz::integer::logic::and_i32::limbs_neg_and_limb_neg;
///
/// assert_eq!(limbs_neg_and_limb_neg(&[0, 2], 3), &[0, 2]);
/// assert_eq!(limbs_neg_and_limb_neg(&[1, 1], 3), &[4294967293, 1]);
/// assert_eq!(limbs_neg_and_limb_neg(&[0xffff_fffe, 1], 1), &[0, 2]);
/// assert_eq!(limbs_neg_and_limb_neg(&[0xffff_fffe, 0xffff_ffff], 1), &[0, 0, 1]);
/// ```
pub fn limbs_neg_and_limb_neg(limbs: &[u32], limb: u32) -> Vec<u32> {
    let mut result_limbs = limbs.to_vec();
    limbs_vec_neg_and_limb_neg_in_place(&mut result_limbs, limb);
    result_limbs
}

/// Interpreting a slice of `u32`s as the limbs (in ascending order) of the negative of an
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
/// Panics if `in_limbs` is empty or if `out_limbs` is shorter than `in_limbs`.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::and_i32::limbs_neg_and_limb_neg_to_out;
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
pub fn limbs_neg_and_limb_neg_to_out(out_limbs: &mut [u32], in_limbs: &[u32], limb: u32) -> bool {
    assert!(out_limbs.len() >= in_limbs.len());
    if in_limbs[0] == 0 {
        out_limbs[0..in_limbs.len()].copy_from_slice(in_limbs);
        false
    } else {
        let result_head = in_limbs[0].wrapping_neg() & limb;
        if result_head == 0 {
            out_limbs[0] = 0;
            mpn_add_1_to_out(&mut out_limbs[1..], &in_limbs[1..], 1)
        } else {
            out_limbs[0] = result_head.wrapping_neg();
            out_limbs[1..in_limbs.len()].copy_from_slice(&in_limbs[1..]);
            false
        }
    }
}

/// Interpreting a slice of `u32`s as the limbs (in ascending order) of the negative of an
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
/// use malachite_nz::integer::logic::and_i32::limbs_slice_neg_and_limb_neg_in_place;
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
pub fn limbs_slice_neg_and_limb_neg_in_place(limbs: &mut [u32], limb: u32) -> bool {
    let (head, tail) = limbs.split_at_mut(1);
    let head = &mut head[0];
    if *head == 0 {
        false
    } else {
        *head = head.wrapping_neg() & limb;
        if *head == 0 {
            mpn_add_1_in_place(tail, 1)
        } else {
            head.wrapping_neg_assign();
            false
        }
    }
}

/// Interpreting a `Vec` of `u32`s as the limbs (in ascending order) of the negative of an
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
/// use malachite_nz::integer::logic::and_i32::limbs_vec_neg_and_limb_neg_in_place;
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
pub fn limbs_vec_neg_and_limb_neg_in_place(limbs: &mut Vec<u32>, limb: u32) {
    if limbs_slice_neg_and_limb_neg_in_place(limbs, limb) {
        limbs.push(1)
    }
}

/// Takes the bitwise and of an `Integer` and an `i32`, taking the `Integer` by value.
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
impl BitAnd<i32> for Integer {
    type Output = Integer;

    fn bitand(mut self, other: i32) -> Integer {
        self &= other;
        self
    }
}

/// Takes the bitwise and of an `Integer` and an `i32`, taking the `Integer` by reference.
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
impl<'a> BitAnd<i32> for &'a Integer {
    type Output = Integer;

    fn bitand(self, other: i32) -> Integer {
        let u_other = u32::wrapping_from(other);
        if other >= 0 {
            Integer {
                sign: true,
                abs: Small(self & u_other),
            }
        } else {
            Integer {
                sign: self.sign,
                abs: if self.sign {
                    self.abs.and_pos_u32_neg(u_other)
                } else {
                    self.abs.and_neg_u32_neg(u_other)
                },
            }
        }
    }
}

/// Takes the bitwise and of an `i32` and an `Integer`, taking the `Integer` by value.
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
impl BitAnd<Integer> for i32 {
    type Output = Integer;

    fn bitand(self, other: Integer) -> Integer {
        other & self
    }
}

/// Takes the bitwise and of an `i32` and an `Integer`, taking the `Integer` by reference.
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
impl<'a> BitAnd<&'a Integer> for i32 {
    type Output = Integer;

    fn bitand(self, other: &'a Integer) -> Integer {
        other & self
    }
}

/// Bitwise-ands an `Integer` with an `i32` in place.
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
impl BitAndAssign<i32> for Integer {
    fn bitand_assign(&mut self, other: i32) {
        if other >= 0 {
            *self &= other.unsigned_abs();
        } else if self.sign {
            self.abs.and_assign_pos_u32_neg(u32::wrapping_from(other));
        } else {
            self.abs.and_assign_neg_u32_neg(u32::wrapping_from(other));
        }
    }
}

impl Natural {
    pub(crate) fn and_assign_pos_u32_neg(&mut self, other: u32) {
        match *self {
            Small(ref mut small) => *small &= other,
            Large(ref mut limbs) => limbs_pos_and_limb_neg_in_place(limbs, other),
        }
    }

    pub(crate) fn and_pos_u32_neg(&self, other: u32) -> Natural {
        match *self {
            Small(small) => Small(small & other),
            Large(ref limbs) => Large(limbs_pos_and_limb_neg(limbs, other)),
        }
    }

    pub(crate) fn and_assign_neg_u32_neg(&mut self, other: u32) {
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

    pub(crate) fn and_neg_u32_neg(&self, other: u32) -> Natural {
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
