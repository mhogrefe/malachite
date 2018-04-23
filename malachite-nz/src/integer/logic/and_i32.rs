use integer::Integer;
use malachite_base::misc::WrappingFrom;
use malachite_base::num::UnsignedAbs;
use natural::arithmetic::add_u32::mpn_add_1_in_place;
use natural::Natural::{self, Large, Small};
use std::ops::{BitAnd, BitAndAssign};

fn limbs_slice_neg_and_limb_neg_special(limbs: &mut [u32], limb: u32) -> bool {
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

fn limbs_vec_neg_and_limb_neg_special(limbs: &mut Vec<u32>, limb: u32) {
    if limbs_slice_neg_and_limb_neg_special(limbs, limb) {
        limbs.push(1)
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
///     assert_eq!((&(-Integer::from(0xffff_ffffu32)) & 2i32).to_string(), "0");
///     assert_eq!((&Integer::from_str("-12345678987654321").unwrap() & -456i32).to_string(),
///         "-12345678987654648");
/// }
/// ```
impl<'a> BitAnd<i32> for &'a Integer {
    type Output = Integer;

    fn bitand(self, other: i32) -> Integer {
        let mut n = self.clone();
        n &= other;
        n
    }
}

/// Takes the bitwise and of an `i32` and an `Integer`, taking the `Integer` by reference. The
/// output is an `i32`.
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
///     assert_eq!((2i32 & &(-Integer::from(0xffff_ffffu32))).to_string(), "0");
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
            self.abs
                .and_assign_u32_neg_special(u32::wrapping_from(other));
        } else {
            self.abs
                .and_neg_assign_u32_neg_special(u32::wrapping_from(other));
        }
    }
}

impl Natural {
    fn and_assign_u32_neg_special(&mut self, other: u32) {
        match *self {
            Small(ref mut small) => *small &= other,
            Large(ref mut limbs) => limbs[0] &= other,
        }
    }

    fn and_neg_assign_u32_neg_special(&mut self, other: u32) {
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
            { limbs_vec_neg_and_limb_neg_special(limbs, other) }
        );
    }
}
