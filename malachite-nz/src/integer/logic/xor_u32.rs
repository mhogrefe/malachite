use integer::Integer;
use malachite_base::num::WrappingNegAssign;
use natural::arithmetic::add_u32::{mpn_add_1, mpn_add_1_in_place, mpn_add_1_to_out};
use natural::arithmetic::sub_u32::{mpn_sub_1, mpn_sub_1_in_place, mpn_sub_1_to_out};
use natural::Natural::{self, Large, Small};
use std::ops::{BitXor, BitXorAssign};
use std::u32;

/// Interpreting a slice of `u32`s as the limbs (in ascending order) of the negative of an
/// `Integer`, returns the limbs of the bitwise xor of the `Integer` and a `u32`. `limbs` cannot be
/// empty or only contain zeros.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `limbs.len()`
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::xor_u32::limbs_neg_xor_limb;
///
/// assert_eq!(limbs_neg_xor_limb(&[123, 456], 789), &[880, 456]);
/// assert_eq!(limbs_neg_xor_limb(&[0xffff_fffe, 0xffff_ffff, 0xffff_ffff], 2), &[0, 0, 0, 1]);
/// ```
pub fn limbs_neg_xor_limb(limbs: &[u32], limb: u32) -> Vec<u32> {
    if limb == 0 {
        return limbs.to_vec();
    }
    let head = limbs[0];
    let tail = &limbs[1..];
    let mut result_limbs = Vec::with_capacity(limbs.len());
    if head != 0 {
        let head = head.wrapping_neg() ^ limb;
        if head == 0 {
            result_limbs.push(0);
            result_limbs.extend_from_slice(&mpn_add_1(tail, 1));
        } else {
            result_limbs.push(head.wrapping_neg());
            result_limbs.extend_from_slice(tail);
        }
    } else {
        result_limbs.push(limb.wrapping_neg());
        result_limbs.extend_from_slice(&mpn_sub_1(tail, 1).0);
    }
    result_limbs
}

/// Interpreting a slice of `u32`s as the limbs (in ascending order) of the negative of an
/// `Integer`, writes the limbs of the bitwise and of the `Integer`, writes the limbs of the bitwise
/// xor of the `Integer` and a `u32` to an output slice. The output slice must be at least as long
/// as the input slice. `limbs` cannot be empty or only contain zeros. Returns whether a carry
/// occurs.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `in_limbs.len()`
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::xor_u32::limbs_neg_xor_limb_to_out;
///
/// let mut limbs = vec![0, 0, 0, 0];
/// assert_eq!(limbs_neg_xor_limb_to_out(&mut limbs, &[123, 456], 789), false);
/// assert_eq!(limbs, &[880, 456, 0, 0]);
///
/// let mut limbs = vec![10, 10, 10, 10];
/// assert_eq!(limbs_neg_xor_limb_to_out(&mut limbs, &[0xffff_fffe, 0xffff_ffff, 0xffff_ffff], 2),
///     true);
/// assert_eq!(limbs, &[0, 0, 0, 10]);
/// ```
pub fn limbs_neg_xor_limb_to_out(out_limbs: &mut [u32], in_limbs: &[u32], limb: u32) -> bool {
    let len = in_limbs.len();
    assert!(out_limbs.len() >= len);
    if limb == 0 {
        out_limbs[0..len].copy_from_slice(in_limbs);
        return false;
    }
    let head = in_limbs[0];
    let tail = &in_limbs[1..];
    if head != 0 {
        let head = head.wrapping_neg() ^ limb;
        if head == 0 {
            out_limbs[0] = 0;
            mpn_add_1_to_out(&mut out_limbs[1..len], tail, 1)
        } else {
            out_limbs[0] = head.wrapping_neg();
            out_limbs[1..len].copy_from_slice(tail);
            false
        }
    } else {
        out_limbs[0] = limb.wrapping_neg();
        mpn_sub_1_to_out(&mut out_limbs[1..len], tail, 1);
        false
    }
}

/// Interpreting a slice of `u32`s as the limbs (in ascending order) of the negative of an
/// `Integer`, writes the limbs of the bitwise xor of the `Integer` and a `u32` to the input slice.
/// `limbs` cannot be empty or only contain zeros. Returns whether a carry occurs.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::xor_u32::limbs_slice_neg_xor_limb_in_place;
///
/// let mut limbs = vec![123, 456];
/// assert_eq!(limbs_slice_neg_xor_limb_in_place(&mut limbs, 789), false);
/// assert_eq!(limbs, &[880, 456]);
///
/// let mut limbs = vec![0xffff_fffe, 0xffff_ffff, 0xffff_ffff];
/// assert_eq!(limbs_slice_neg_xor_limb_in_place(&mut limbs, 2), true);
/// assert_eq!(limbs, &[0, 0, 0]);
/// ```
pub fn limbs_slice_neg_xor_limb_in_place(limbs: &mut [u32], limb: u32) -> bool {
    if limb == 0 {
        return false;
    }
    let (head, tail) = limbs.split_at_mut(1);
    let head = &mut head[0];
    if *head != 0 {
        *head = head.wrapping_neg() ^ limb;
        if *head == 0 {
            mpn_add_1_in_place(tail, 1)
        } else {
            head.wrapping_neg_assign();
            false
        }
    } else {
        *head = limb.wrapping_neg();
        mpn_sub_1_in_place(tail, 1);
        false
    }
}

/// Interpreting a `Vec` of `u32`s as the limbs (in ascending order) of the negative of an
/// `Integer`, writes the limbs of the bitwise xor of the `Integer` and a `u32` to the input slice.
/// `limbs` cannot be empty or only contain zeros. If a carry occurs, extends the `Vec`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::xor_u32::limbs_vec_neg_xor_limb_in_place;
///
/// let mut limbs = vec![123, 456];
/// limbs_vec_neg_xor_limb_in_place(&mut limbs, 789);
/// assert_eq!(limbs, &[880, 456]);
///
/// let mut limbs = vec![0xffff_fffe, 0xffff_ffff, 0xffff_ffff];
/// limbs_vec_neg_xor_limb_in_place(&mut limbs, 2);
/// assert_eq!(limbs, &[0, 0, 0, 1]);
/// ```
pub fn limbs_vec_neg_xor_limb_in_place(limbs: &mut Vec<u32>, limb: u32) {
    if limbs_slice_neg_xor_limb_in_place(limbs, limb) {
        limbs.push(1);
    }
}

/// Takes the bitwise xor of an `Integer` and a `u32`, taking the `Integer` by value.
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
///     assert_eq!((Integer::ZERO ^ 123u32).to_string(), "123");
///     assert_eq!((Integer::from(123) ^ 0u32).to_string(), "123");
///     assert_eq!((Integer::from(-123) ^ 456u32).to_string(), "-435");
/// }
/// ```
impl BitXor<u32> for Integer {
    type Output = Integer;

    fn bitxor(mut self, other: u32) -> Integer {
        self ^= other;
        self
    }
}

/// Takes the bitwise xor of an `Integer` and a `u32`, taking the `Integer` by reference.
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
///     assert_eq!((&Integer::ZERO ^ 123u32).to_string(), "123");
///     assert_eq!((&Integer::from(123) ^ 0u32).to_string(), "123");
///     assert_eq!((&Integer::from(-123) ^ 456u32).to_string(), "-435");
/// }
/// ```
impl<'a> BitXor<u32> for &'a Integer {
    type Output = Integer;

    fn bitxor(self, other: u32) -> Integer {
        Integer {
            sign: self.sign,
            abs: if self.sign {
                &self.abs ^ other
            } else {
                self.abs.xor_neg_u32_pos(other)
            },
        }
    }
}

/// Takes the bitwise xor of a `u32` and an `Integer`, taking the `Integer` by value.
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
///     assert_eq!((123u32 ^ Integer::ZERO).to_string(), "123");
///     assert_eq!((0u32 ^ Integer::from(123)).to_string(), "123");
///     assert_eq!((456u32 ^ Integer::from(-123)).to_string(), "-435");
/// }
/// ```
impl BitXor<Integer> for u32 {
    type Output = Integer;

    fn bitxor(self, other: Integer) -> Integer {
        other ^ self
    }
}

/// Takes the bitwise xor of a `u32` and an `Integer`, taking the `Integer` by reference.
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
///     assert_eq!((123u32 ^ &Integer::ZERO).to_string(), "123");
///     assert_eq!((0u32 ^ &Integer::from(123)).to_string(), "123");
///     assert_eq!((456u32 ^ &Integer::from(-123)).to_string(), "-435");
/// }
/// ```
impl<'a> BitXor<&'a Integer> for u32 {
    type Output = Integer;

    fn bitxor(self, other: &'a Integer) -> Integer {
        other ^ self
    }
}

/// Bitwise-xors an `Integer` with a `u32` in place.
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
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::from(0xffff_ffffu32);
///     x ^= 0x0000_000f;
///     x ^= 0x0000_0f00;
///     x ^= 0x000f_0000;
///     x ^= 0x0f00_0000;
///     assert_eq!(x, 0xf0f0_f0f0u32);
/// }
/// ```
impl BitXorAssign<u32> for Integer {
    fn bitxor_assign(&mut self, other: u32) {
        if self.sign {
            self.abs ^= other;
        } else {
            self.abs.xor_assign_neg_u32_pos(other);
        }
    }
}

impl Natural {
    pub(crate) fn xor_assign_neg_u32_pos(&mut self, other: u32) {
        if other == 0 {
            return;
        }
        mutate_with_possible_promotion!(
            self,
            small,
            limbs,
            {
                let result = small.wrapping_neg() ^ other;
                if result == 0 {
                    None
                } else {
                    Some(result.wrapping_neg())
                }
            },
            { limbs_vec_neg_xor_limb_in_place(limbs, other) }
        );
        self.trim();
    }

    pub(crate) fn xor_neg_u32_pos(&self, other: u32) -> Natural {
        match *self {
            Small(ref small) => {
                let result = small.wrapping_neg() ^ other;
                if result == 0 {
                    Large(vec![0, 1])
                } else {
                    Small(result.wrapping_neg())
                }
            }
            Large(ref limbs) => {
                let mut result = Large(limbs_neg_xor_limb(limbs, other));
                result.trim();
                result
            }
        }
    }
}
