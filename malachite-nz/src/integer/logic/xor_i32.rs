use integer::Integer;
use malachite_base::misc::WrappingFrom;
use natural::arithmetic::add_u32::{
    limbs_add_limb, limbs_add_limb_to_out, limbs_slice_add_limb_in_place,
};
use natural::arithmetic::sub_u32::{mpn_sub_1, mpn_sub_1_in_place, mpn_sub_1_to_out};
use natural::Natural::{self, Large, Small};
use std::ops::{BitXor, BitXorAssign};
use std::u32;

/// Interpreting a slice of `u32`s as the limbs (in ascending order) of an `Integer`, returns the
/// limbs of the bitwise xor of the `Integer` and a negative number whose lowest limb is given by
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
/// use malachite_nz::integer::logic::xor_i32::limbs_pos_xor_limb_neg;
///
/// assert_eq!(limbs_pos_xor_limb_neg(&[0, 2], 3), &[4294967293, 2]);
/// assert_eq!(limbs_pos_xor_limb_neg(&[1, 2, 3], 4), &[4294967291, 2, 3]);
/// assert_eq!(limbs_pos_xor_limb_neg(&[2, 0xffff_ffff], 2), &[0, 0, 1]);
/// ```
pub fn limbs_pos_xor_limb_neg(limbs: &[u32], limb: u32) -> Vec<u32> {
    let lowest_limb = limbs[0] ^ limb;
    let mut result_limbs;
    if lowest_limb == 0 {
        result_limbs = limbs_add_limb(&limbs[1..], 1);
        result_limbs.insert(0, 0);
    } else {
        result_limbs = limbs.to_vec();
        result_limbs[0] = lowest_limb.wrapping_neg();
    }
    result_limbs
}

/// Interpreting a slice of `u32`s as the limbs (in ascending order) of an `Integer`, writes the
/// limbs of the bitwise xor of the `Integer` and a negative number whose lowest limb is given by
/// `limb` and whose other limbs are full of `true` bits to an output slice. `in_limbs` may not be
/// empty or only contain zeros. The output slice must be at least as long as the input slice.
/// Returns whether there is a carry.
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
/// use malachite_nz::integer::logic::xor_i32::limbs_pos_xor_limb_neg_to_out;
///
/// let mut result = vec![10, 10];
/// assert_eq!(limbs_pos_xor_limb_neg_to_out(&mut result, &[0, 2], 3), false);
/// assert_eq!(result, &[4294967293, 2]);
///
/// let mut result = vec![10, 10, 10, 10];
/// assert_eq!(limbs_pos_xor_limb_neg_to_out(&mut result, &[1, 2, 3], 4), false);
/// assert_eq!(result, &[4294967291, 2, 3, 10]);
///
/// let mut result = vec![10, 10, 10, 10];
/// assert_eq!(limbs_pos_xor_limb_neg_to_out(&mut result, &[2, 0xffff_ffff], 2), true);
/// assert_eq!(result, &[0, 0, 10, 10]);
/// ```
pub fn limbs_pos_xor_limb_neg_to_out(out_limbs: &mut [u32], in_limbs: &[u32], limb: u32) -> bool {
    let len = in_limbs.len();
    assert!(out_limbs.len() >= len);
    let lowest_limb = in_limbs[0] ^ limb;
    if lowest_limb == 0 {
        out_limbs[0] = 0;
        limbs_add_limb_to_out(&mut out_limbs[1..len], &in_limbs[1..], 1)
    } else {
        out_limbs[0] = lowest_limb.wrapping_neg();
        out_limbs[1..len].copy_from_slice(&in_limbs[1..]);
        false
    }
}

/// Interpreting a slice of `u32`s as the limbs (in ascending order) of an `Integer`, takes the
/// bitwise xor of the `Integer` and a negative number whose lowest limb is given by `limb` and
/// whose other limbs are full of `true` bits, in place. `limbs` may not be empty. Returns whether
/// there is a carry.
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
/// use malachite_nz::integer::logic::xor_i32::limbs_slice_pos_xor_limb_neg_in_place;
///
/// let mut limbs = vec![0, 2];
/// assert_eq!(limbs_slice_pos_xor_limb_neg_in_place(&mut limbs, 3), false);
/// assert_eq!(limbs, &[4294967293, 2]);
///
/// let mut limbs = vec![1, 2, 3];
/// assert_eq!(limbs_slice_pos_xor_limb_neg_in_place(&mut limbs, 4), false);
/// assert_eq!(limbs, &[4294967291, 2, 3]);
///
/// let mut limbs = vec![2, 0xffff_ffff];
/// assert_eq!(limbs_slice_pos_xor_limb_neg_in_place(&mut limbs, 2), true);
/// assert_eq!(limbs, &[0, 0]);
/// ```
pub fn limbs_slice_pos_xor_limb_neg_in_place(limbs: &mut [u32], limb: u32) -> bool {
    let (head, tail) = limbs.split_at_mut(1);
    let head = &mut head[0];
    *head ^= limb;
    if *head == 0 {
        limbs_slice_add_limb_in_place(tail, 1)
    } else {
        *head = head.wrapping_neg();
        false
    }
}

/// Interpreting a `Vec` of `u32`s as the limbs (in ascending order) of an `Integer`, takes the
/// bitwise xor of the `Integer` and a negative number whose lowest limb is given by `limb` and
/// whose other limbs are full of `true` bits, in place. `limbs` may not be empty.
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
/// use malachite_nz::integer::logic::xor_i32::limbs_vec_pos_xor_limb_neg_in_place;
///
/// let mut limbs = vec![0, 2];
/// limbs_vec_pos_xor_limb_neg_in_place(&mut limbs, 3);
/// assert_eq!(limbs, &[4294967293, 2]);
///
/// let mut limbs = vec![1, 2, 3];
/// limbs_vec_pos_xor_limb_neg_in_place(&mut limbs, 4);
/// assert_eq!(limbs, &[4294967291, 2, 3]);
///
/// let mut limbs = vec![2, 0xffff_ffff];
/// limbs_vec_pos_xor_limb_neg_in_place(&mut limbs, 2);
/// assert_eq!(limbs, &[0, 0, 1]);
pub fn limbs_vec_pos_xor_limb_neg_in_place(limbs: &mut Vec<u32>, limb: u32) {
    if limbs_slice_pos_xor_limb_neg_in_place(limbs, limb) {
        limbs.push(1);
    }
}

/// Interpreting a slice of `u32`s as the limbs (in ascending order) of the negative of an
/// `Integer`, returns the limbs of the bitwise xor of the `Integer` and a negative number whose
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
/// Panics if `limbs` is empty or only contains zeros.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::xor_i32::limbs_neg_xor_limb_neg;
///
/// assert_eq!(limbs_neg_xor_limb_neg(&[0, 2], 3), &[3, 1]);
/// assert_eq!(limbs_neg_xor_limb_neg(&[1, 2, 3], 4), &[4294967291, 2, 3]);
/// ```
pub fn limbs_neg_xor_limb_neg(limbs: &[u32], limb: u32) -> Vec<u32> {
    let mut result_limbs;
    if limbs[0] == 0 {
        let (result, carry) = mpn_sub_1(limbs, 1);
        result_limbs = result;
        assert!(!carry);
        result_limbs[0] = limb;
    } else {
        result_limbs = limbs.to_vec();
        result_limbs[0] = limbs[0].wrapping_neg() ^ limb;
    }
    result_limbs
}

/// Interpreting a slice of `u32`s as the limbs (in ascending order) of the negative of an
/// `Integer`, writes the limbs of the bitwise xor of the `Integer` and a negative number whose
/// lowest limb is given by `limb` and whose other limbs are full of `true` bits to an output slice.
/// `in_limbs` may not be empty or only contain zeros. The output slice must be at least as long as
/// the input slice.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `in_limbs.len()`
///
/// # Panics
/// Panics if `in_limbs` is empty or only contains zeros, or if `out_limbs` is shorter than
/// `in_limbs`.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::xor_i32::limbs_neg_xor_limb_neg_to_out;
///
/// let mut result = vec![10, 10];
/// limbs_neg_xor_limb_neg_to_out(&mut result, &[0, 2], 3);
/// assert_eq!(result, &[3, 1]);
///
/// let mut result = vec![10, 10, 10, 10];
/// limbs_neg_xor_limb_neg_to_out(&mut result, &[1, 2, 3], 4);
/// assert_eq!(result, &[4294967291, 2, 3, 10]);
/// ```
pub fn limbs_neg_xor_limb_neg_to_out(out_limbs: &mut [u32], in_limbs: &[u32], limb: u32) {
    let len = in_limbs.len();
    assert!(out_limbs.len() >= len);
    if in_limbs[0] == 0 {
        out_limbs[0] = limb;
        assert!(!mpn_sub_1_to_out(&mut out_limbs[1..len], &in_limbs[1..], 1));
    } else {
        out_limbs[0] = in_limbs[0].wrapping_neg() ^ limb;
        out_limbs[1..len].copy_from_slice(&in_limbs[1..]);
    }
}

/// Interpreting a `Vec` of `u32`s as the limbs (in ascending order) of the negative of an
/// `Integer`, takes the bitwise xor of the `Integer` and a negative number whose lowest limb is
/// given by `limb` and whose other limbs are full of `true` bits, in place. `limbs` may not be
/// empty or only contain zeros.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if `limbs` is empty or only contains zeros.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::xor_i32::limbs_neg_xor_limb_neg_in_place;
///
/// let mut limbs = vec![0, 2];
/// limbs_neg_xor_limb_neg_in_place(&mut limbs, 3);
/// assert_eq!(limbs, &[3, 1]);
///
/// let mut limbs = vec![1, 2, 3];
/// limbs_neg_xor_limb_neg_in_place(&mut limbs, 4);
/// assert_eq!(limbs, &[4294967291, 2, 3]);
/// ```
pub fn limbs_neg_xor_limb_neg_in_place(limbs: &mut [u32], limb: u32) {
    if limbs[0] == 0 {
        assert!(!mpn_sub_1_in_place(&mut limbs[1..], 1));
        limbs[0] = limb;
    } else {
        limbs[0] = limbs[0].wrapping_neg() ^ limb;
    }
}

/// Takes the bitwise xor of an `Integer` and an `i32`, taking the `Integer` by value.
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
///     assert_eq!((Integer::ZERO ^ 123i32).to_string(), "123");
///     assert_eq!((Integer::from(123) ^ 0i32).to_string(), "123");
///     assert_eq!((Integer::from(-123) ^ -456i32).to_string(), "445");
///     assert_eq!((Integer::from(123) ^ -456i32).to_string(), "-445");
///     assert_eq!((-Integer::from(0xffff_fffeu32) ^ 2i32).to_string(), "-4294967296");
///     assert_eq!((Integer::from_str("-12345678987654321").unwrap() ^ -456i32).to_string(),
///         "12345678987654519");
/// }
/// ```
impl BitXor<i32> for Integer {
    type Output = Integer;

    fn bitxor(mut self, other: i32) -> Integer {
        self ^= other;
        self
    }
}

/// Takes the bitwise xor of an `Integer` and an `i32`, taking the `Integer` by reference.
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
///     assert_eq!((&Integer::ZERO ^ 123i32).to_string(), "123");
///     assert_eq!((&Integer::from(123) ^ 0i32).to_string(), "123");
///     assert_eq!((&Integer::from(-123) ^ -456i32).to_string(), "445");
///     assert_eq!((&Integer::from(123) ^ -456i32).to_string(), "-445");
///     assert_eq!((&-Integer::from(0xffff_fffeu32) ^ 2i32).to_string(), "-4294967296");
///     assert_eq!((&Integer::from_str("-12345678987654321").unwrap() ^ -456i32).to_string(),
///         "12345678987654519");
/// }
/// ```
impl<'a> BitXor<i32> for &'a Integer {
    type Output = Integer;

    fn bitxor(self, other: i32) -> Integer {
        let u_other = u32::wrapping_from(other);
        if other >= 0 {
            self ^ u_other
        } else {
            Integer {
                sign: !self.sign,
                abs: if self.sign {
                    self.abs.xor_pos_u32_neg(u_other)
                } else {
                    self.abs.xor_neg_u32_neg(u_other)
                },
            }
        }
    }
}

/// Takes the bitwise xor of an `i32` and an `Integer`, taking the `Integer` by value.
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
///     assert_eq!((123i32 ^ Integer::ZERO).to_string(), "123");
///     assert_eq!((0i32 ^ Integer::from(123)).to_string(), "123");
///     assert_eq!((-456i32 ^ Integer::from(-123)).to_string(), "445");
///     assert_eq!((-456i32 ^ Integer::from(123)).to_string(), "-445");
///     assert_eq!((2i32 ^ -Integer::from(0xffff_fffeu32)).to_string(), "-4294967296");
///     assert_eq!((-456i32 ^ Integer::from_str("-12345678987654321").unwrap()).to_string(),
///         "12345678987654519");
/// }
/// ```
impl BitXor<Integer> for i32 {
    type Output = Integer;

    fn bitxor(self, other: Integer) -> Integer {
        other ^ self
    }
}

/// Takes the bitwise xor of an `i32` and an `Integer`, taking the `Integer` by reference.
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
///     assert_eq!((123i32 ^ &Integer::ZERO).to_string(), "123");
///     assert_eq!((0i32 ^ &Integer::from(123)).to_string(), "123");
///     assert_eq!((-456i32 ^ &Integer::from(-123)).to_string(), "445");
///     assert_eq!((-456i32 ^ &Integer::from(123)).to_string(), "-445");
///     assert_eq!((2i32 ^ &-Integer::from(0xffff_fffeu32)).to_string(), "-4294967296");
///     assert_eq!((-456i32 ^ &Integer::from_str("-12345678987654321").unwrap()).to_string(),
///         "12345678987654519");
/// }
/// ```
impl<'a> BitXor<&'a Integer> for i32 {
    type Output = Integer;

    fn bitxor(self, other: &'a Integer) -> Integer {
        other ^ self
    }
}

/// Bitwise-xors an `Integer` with an `i32` in place.
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
///     x ^= 0x0000_000fi32;
///     x ^= 0x0000_0f00i32;
///     x ^= 0x000f_0000i32;
///     x ^= 0x0f00_0000i32;
///     assert_eq!(x, 0xf0f0_f0f0u32);
/// }
/// ```
impl BitXorAssign<i32> for Integer {
    fn bitxor_assign(&mut self, other: i32) {
        let u_other = u32::wrapping_from(other);
        if other >= 0 {
            *self ^= u_other;
        } else if self.sign {
            self.sign = false;
            self.abs.xor_assign_pos_u32_neg(u_other);
        } else {
            self.sign = true;
            self.abs.xor_assign_neg_u32_neg(u_other);
        }
    }
}

impl Natural {
    pub(crate) fn xor_assign_pos_u32_neg(&mut self, other: u32) {
        mutate_with_possible_promotion!(
            self,
            small,
            limbs,
            {
                let result = *small ^ other;
                if result == 0 {
                    None
                } else {
                    Some(result.wrapping_neg())
                }
            },
            { limbs_vec_pos_xor_limb_neg_in_place(limbs, other) }
        );
    }

    pub(crate) fn xor_pos_u32_neg(&self, other: u32) -> Natural {
        match *self {
            Small(small) => {
                let result = small ^ other;
                if result == 0 {
                    Large(vec![0, 1])
                } else {
                    Small(result.wrapping_neg())
                }
            }
            Large(ref limbs) => Large(limbs_pos_xor_limb_neg(limbs, other)),
        }
    }

    pub(crate) fn xor_assign_neg_u32_neg(&mut self, other: u32) {
        match *self {
            Small(ref mut small) => *small = small.wrapping_neg() ^ other,
            Large(ref mut limbs) => limbs_neg_xor_limb_neg_in_place(limbs, other),
        }
        self.trim();
    }

    pub(crate) fn xor_neg_u32_neg(&self, other: u32) -> Natural {
        match *self {
            Small(small) => Small(small.wrapping_neg() ^ other),
            Large(ref limbs) => {
                let mut result = Large(limbs_neg_xor_limb_neg(limbs, other));
                result.trim();
                result
            }
        }
    }
}
