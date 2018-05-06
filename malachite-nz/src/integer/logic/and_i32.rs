use integer::Integer;
use malachite_base::misc::WrappingFrom;
use malachite_base::num::{UnsignedAbs, Zero};
use natural::arithmetic::add_u32::{mpn_add_1, mpn_add_1_in_place};
use natural::Natural::{self, Large, Small};
use std::ops::{BitAnd, BitAndAssign};

/// Interpreting a slice of `u32`s as the twos-complement limbs of the negative of an `Integer`, in
/// ascending order, writes the limbs of the bitwise and of the `Integer` and a negative number
/// whose lowest limb is given by `limb` and whose other limbs are full of `true` bits to an output
/// slice. `in_limbs` may not be empty or only contain zeros. Returns whether a carry occurs.
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
/// use malachite_nz::integer::logic::and_i32::limbs_slice_neg_and_limb_neg;
///
/// let mut result = vec![0, 0];
/// assert_eq!(limbs_slice_neg_and_limb_neg(&mut result, &[0, 2], 3), false);
/// assert_eq!(result, vec![0, 2]);
///
/// let mut result = vec![0, 0];
/// assert_eq!(limbs_slice_neg_and_limb_neg(&mut result, &[1, 1], 3), false);
/// assert_eq!(result, vec![4294967293, 1]);
///
/// let mut result = vec![0, 0];
/// assert_eq!(limbs_slice_neg_and_limb_neg(&mut result, &[0xffff_fffe, 1], 1), false);
/// assert_eq!(result, vec![0, 2]);
///
/// let mut result = vec![0, 0];
/// assert_eq!(limbs_slice_neg_and_limb_neg(&mut result, &[0xffff_fffe, 0xffff_ffff], 1), true);
/// assert_eq!(result, vec![0, 0]);
/// ```
pub fn limbs_slice_neg_and_limb_neg(out_limbs: &mut [u32], in_limbs: &[u32], limb: u32) -> bool {
    assert!(out_limbs.len() >= in_limbs.len());
    if in_limbs[0] == 0 {
        out_limbs[0..in_limbs.len()].copy_from_slice(in_limbs);
    } else {
        let result_head = in_limbs[0].wrapping_neg() & limb;
        if result_head == 0 {
            out_limbs[0] = 0;
            if mpn_add_1(&mut out_limbs[1..], &in_limbs[1..], 1) {
                return true;
            }
        } else {
            out_limbs[0] = result_head.wrapping_neg();
            out_limbs[1..in_limbs.len()].copy_from_slice(&in_limbs[1..]);
        }
    }
    false
}

/// Interpreting a slice of `u32`s as the twos-complement limbs of the negative of an `Integer`, in
/// ascending order, returns the limbs of the bitwise and of the `Integer` and a negative number
/// whose lowest limb is given by `limb` and whose other limbs are full of `true` bits. `limbs` may
/// not be empty or only contain zeros.
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
/// use malachite_nz::integer::logic::and_i32::limbs_vec_neg_and_limb_neg;
///
/// assert_eq!(limbs_vec_neg_and_limb_neg(&[0, 2], 3), vec![0, 2]);
/// assert_eq!(limbs_vec_neg_and_limb_neg(&[1, 1], 3), vec![4294967293, 1]);
/// assert_eq!(limbs_vec_neg_and_limb_neg(&[0xffff_fffe, 1], 1), vec![0, 2]);
/// assert_eq!(limbs_vec_neg_and_limb_neg(&[0xffff_fffe, 0xffff_ffff], 1), vec![0, 0, 1]);
/// ```
pub fn limbs_vec_neg_and_limb_neg(limbs: &[u32], limb: u32) -> Vec<u32> {
    let mut result_limbs = vec![0; limbs.len()];
    if limbs_slice_neg_and_limb_neg(&mut result_limbs, limbs, limb) {
        result_limbs.push(1);
    }
    result_limbs
}

/// Interpreting a `Vec` of `u32`s as the twos-complement limbs of the negative of an `Integer`, in
/// ascending order, takes the bitwise and of the `Integer` and a negative number whose lowest limb
/// is given by `limb` and whose other limbs are full of `true` bits, in place. `limbs` may not be
///// empty or only contain zeros. Returns whether there is a carry.
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
/// assert_eq!(limbs, vec![0, 2]);
///
/// let mut limbs = vec![1, 1];
/// assert_eq!(limbs_slice_neg_and_limb_neg_in_place(&mut limbs, 3), false);
/// assert_eq!(limbs, vec![4294967293, 1]);
///
/// let mut limbs = vec![0xffff_fffe, 1];
/// assert_eq!(limbs_slice_neg_and_limb_neg_in_place(&mut limbs, 1), false);
/// assert_eq!(limbs, vec![0, 2]);
///
/// let mut limbs = vec![0xffff_fffe, 0xffff_ffff];
/// assert_eq!(limbs_slice_neg_and_limb_neg_in_place(&mut limbs, 1), true);
/// assert_eq!(limbs, vec![0, 0]);
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
            *head = head.wrapping_neg();
            false
        }
    }
}

/// Interpreting a slices of `u32`s as the twos-complement limbs of the negative of an `Integer`, in
/// ascending order, takes the bitwise and of the `Integer` and a negative number whose lowest limb
/// is given by `limb` and whose other limbs are full of `true` bits, in place. `limbs` may not be
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
/// assert_eq!(limbs, vec![0, 2]);
///
/// let mut limbs = vec![1, 1];
/// limbs_vec_neg_and_limb_neg_in_place(&mut limbs, 3);
/// assert_eq!(limbs, vec![4294967293, 1]);
///
/// let mut limbs = vec![0xffff_fffe, 1];
/// limbs_vec_neg_and_limb_neg_in_place(&mut limbs, 1);
/// assert_eq!(limbs, vec![0, 2]);
///
/// let mut limbs = vec![0xffff_fffe, 0xffff_ffff];
/// limbs_vec_neg_and_limb_neg_in_place(&mut limbs, 1);
/// assert_eq!(limbs, vec![0, 0, 1]);
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
        if other >= 0 {
            Integer {
                sign: true,
                abs: Small(self & other.unsigned_abs()),
            }
        } else {
            Integer {
                sign: self.sign,
                abs: if self.sign {
                    self.abs.and_pos_u32_neg(u32::wrapping_from(other))
                } else {
                    self.abs.and_neg_u32_neg(u32::wrapping_from(other))
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

/// Bitwise-ands an `i32` with an `Integer` in place.
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
            Large(ref mut limbs) => limbs[0] &= other,
        }
    }

    pub(crate) fn and_pos_u32_neg(&self, other: u32) -> Natural {
        match *self {
            Small(small) => Small(small & other),
            Large(ref limbs) => {
                let mut result_limbs = limbs.clone();
                result_limbs[0] &= other;
                Large(result_limbs)
            }
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
                if *small == 0 {
                    Some(0)
                } else {
                    let result = (small.wrapping_neg() & other).wrapping_neg();
                    if result == 0 {
                        None
                    } else {
                        Some(result)
                    }
                }
            },
            { limbs_vec_neg_and_limb_neg_in_place(limbs, other) }
        );
    }

    pub(crate) fn and_neg_u32_neg(&self, other: u32) -> Natural {
        match *self {
            Small(0) => Natural::ZERO,
            Small(small) => {
                let result = (small.wrapping_neg() & other).wrapping_neg();
                if result == 0 {
                    Large(vec![0, 1])
                } else {
                    Small(result)
                }
            }
            Large(ref limbs) => Large(limbs_vec_neg_and_limb_neg(limbs, other)),
        }
    }
}
