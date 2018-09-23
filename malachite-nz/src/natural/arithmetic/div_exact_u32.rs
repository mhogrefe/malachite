use malachite_base::num::{
    DivExact, DivExactAssign, ModPowerOfTwo, Parity, PrimitiveInteger, SplitInHalf,
};
use natural::Natural::{self, Large, Small};

// These functions are adapted from binvert_limb and mpn_divexact_1 in GMP 6.1.2.

const INVERT_LIMB_TABLE_LOG_SIZE: u64 = 7;

const INVERT_LIMB_TABLE_SIZE: usize = 1 << INVERT_LIMB_TABLE_LOG_SIZE;

// The entry at index `i` is the multiplicative inverse of 2 * `i` + 1 mod 2<sup>8</sup>.
const INVERT_LIMB_TABLE: [u8; INVERT_LIMB_TABLE_SIZE] = [
    0x01, 0xAB, 0xCD, 0xB7, 0x39, 0xA3, 0xC5, 0xEF, 0xF1, 0x1B, 0x3D, 0xA7, 0x29, 0x13, 0x35, 0xDF,
    0xE1, 0x8B, 0xAD, 0x97, 0x19, 0x83, 0xA5, 0xCF, 0xD1, 0xFB, 0x1D, 0x87, 0x09, 0xF3, 0x15, 0xBF,
    0xC1, 0x6B, 0x8D, 0x77, 0xF9, 0x63, 0x85, 0xAF, 0xB1, 0xDB, 0xFD, 0x67, 0xE9, 0xD3, 0xF5, 0x9F,
    0xA1, 0x4B, 0x6D, 0x57, 0xD9, 0x43, 0x65, 0x8F, 0x91, 0xBB, 0xDD, 0x47, 0xC9, 0xB3, 0xD5, 0x7F,
    0x81, 0x2B, 0x4D, 0x37, 0xB9, 0x23, 0x45, 0x6F, 0x71, 0x9B, 0xBD, 0x27, 0xA9, 0x93, 0xB5, 0x5F,
    0x61, 0x0B, 0x2D, 0x17, 0x99, 0x03, 0x25, 0x4F, 0x51, 0x7B, 0x9D, 0x07, 0x89, 0x73, 0x95, 0x3F,
    0x41, 0xEB, 0x0D, 0xF7, 0x79, 0xE3, 0x05, 0x2F, 0x31, 0x5B, 0x7D, 0xE7, 0x69, 0x53, 0x75, 0x1F,
    0x21, 0xCB, 0xED, 0xD7, 0x59, 0xC3, 0xE5, 0x0F, 0x11, 0x3B, 0x5D, 0xC7, 0x49, 0x33, 0x55, 0xFF,
];

/// Tests that INVERT_LIMB_TABLE is correct.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::div_exact_u32::test_invert_limb_table;
///
/// test_invert_limb_table();
/// ```
pub fn test_invert_limb_table() {
    for i in 0..INVERT_LIMB_TABLE_SIZE {
        let value = ((i as u8) << 1) + 1;
        let inverse = INVERT_LIMB_TABLE[i];
        let product = value.wrapping_mul(inverse);
        assert_eq!(
            product, 1,
            "INVERT_LIMB_TABLE gives incorrect inverse, {}, for value {}",
            inverse, value
        );
    }
}

pub fn limbs_invert_limb(limb: u32) -> u32 {
    assert!(limb.is_odd());
    let index = (limb >> 1).mod_power_of_two(INVERT_LIMB_TABLE_LOG_SIZE);
    let mut inverse: u32 = INVERT_LIMB_TABLE[index as usize].into();
    inverse = (inverse << 1).wrapping_sub((inverse * inverse).wrapping_mul(limb));
    inverse = (inverse << 1).wrapping_sub(inverse.wrapping_mul(inverse).wrapping_mul(limb));
    inverse
}

pub fn limbs_div_exact_limb(limbs: &[u32], divisor: u32) -> Vec<u32> {
    let mut quotient = vec![0; limbs.len()];
    limbs_div_exact_limb_to_out(&mut quotient, limbs, divisor);
    quotient
}

pub fn limbs_div_exact_limb_in_place(limbs: &mut [u32], divisor: u32) {
    assert!(divisor > 0);
    let len = limbs.len();
    assert!(len > 0);
    if divisor.is_even() {
        let shift = divisor.trailing_zeros();
        let shift_complement = u32::WIDTH - shift;
        let shifted_divisor = divisor >> shift;
        let inverse = limbs_invert_limb(shifted_divisor);
        let shifted_divisor = u64::from(shifted_divisor);
        let mut upper_half = 0;
        let mut previous_in_limb = limbs[0];
        for i in 1..len {
            let in_limb = limbs[i];
            let shifted_in_limb = (previous_in_limb >> shift) | (in_limb << shift_complement);
            previous_in_limb = in_limb;
            let (difference, carry) = shifted_in_limb.overflowing_sub(upper_half);
            let out_limb = difference.wrapping_mul(inverse);
            limbs[i - 1] = out_limb;
            upper_half = (u64::from(out_limb) * shifted_divisor).upper_half();
            if carry {
                upper_half += 1;
            }
        }
        limbs[len - 1] = (previous_in_limb >> shift)
            .wrapping_sub(upper_half)
            .wrapping_mul(inverse);
    } else {
        let inverse = limbs_invert_limb(divisor);
        let divisor = u64::from(divisor);
        let mut out_limb = limbs[0].wrapping_mul(inverse);
        limbs[0] = out_limb;
        let mut previous_carry = false;
        for limb in limbs[1..].iter_mut() {
            let mut upper_half = (u64::from(out_limb) * divisor).upper_half();
            if previous_carry {
                upper_half += 1;
            }
            let (difference, carry) = limb.overflowing_sub(upper_half);
            previous_carry = carry;
            out_limb = difference.wrapping_mul(inverse);
            *limb = out_limb;
        }
    }
}

pub fn limbs_div_exact_limb_to_out(out_limbs: &mut [u32], in_limbs: &[u32], divisor: u32) {
    assert!(divisor > 0);
    let len = in_limbs.len();
    assert!(len > 0);
    assert!(out_limbs.len() >= len);
    if divisor.is_even() {
        let shift = divisor.trailing_zeros();
        let shift_complement = u32::WIDTH - shift;
        let shifted_divisor = divisor >> shift;
        let inverse = limbs_invert_limb(shifted_divisor);
        let mut upper_half = 0;
        let mut previous_in_limb = in_limbs[0];
        for i in 1..len {
            let in_limb = in_limbs[i];
            let shifted_in_limb = (previous_in_limb >> shift) | (in_limb << shift_complement);
            previous_in_limb = in_limb;
            let (difference, carry) = shifted_in_limb.overflowing_sub(upper_half);
            let out_limb = difference.wrapping_mul(inverse);
            out_limbs[i - 1] = out_limb;
            upper_half = (u64::from(out_limb) * u64::from(shifted_divisor)).upper_half();
            if carry {
                upper_half += 1;
            }
        }
        out_limbs[len - 1] = (previous_in_limb >> shift)
            .wrapping_sub(upper_half)
            .wrapping_mul(inverse);
    } else {
        let inverse = limbs_invert_limb(divisor);
        let mut out_limb = in_limbs[0].wrapping_mul(inverse);
        out_limbs[0] = out_limb;
        let mut previous_carry = false;
        for i in 1..len {
            let mut upper_half = (u64::from(out_limb) * u64::from(divisor)).upper_half();
            if previous_carry {
                upper_half += 1;
            }
            let (difference, carry) = in_limbs[i].overflowing_sub(upper_half);
            previous_carry = carry;
            out_limb = difference.wrapping_mul(inverse);
            out_limbs[i] = out_limb;
        }
    }
}

impl DivExact<u32> for Natural {
    type Output = Natural;

    /// Divides a `Natural` by a `u32`, taking the `Natural` by value. The `Natural` must be exactly
    /// divisible by the `u32`. If it isn't, the behavior of this function is undefined.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `other.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::DivExact;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 3 * 123 = 369
    ///     assert_eq!(Natural::from(369u32).div_exact(123).to_string(), "3");
    ///
    ///     // 8,130,081,300 * 123 = 999,999,999,900
    ///     assert_eq!(Natural::from_str("999999999900").unwrap().div_exact(123).to_string(),
    ///         "8130081300");
    /// }
    /// ```
    fn div_exact(mut self, other: u32) -> Natural {
        self.div_exact_assign(other);
        self
    }
}

impl<'a> DivExact<u32> for &'a Natural {
    type Output = Natural;

    /// Divides a `Natural` by a `u32`, taking the `Natural` by reference. The `Natural` must be
    /// exactly divisible by the `u32`. If it isn't, the behavior of this function is undefined.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `other.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::DivExact;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 3 * 123 = 369
    ///     assert_eq!((&Natural::from(369u32)).div_exact(123).to_string(), "3");
    ///
    ///     // 8,130,081,300 * 123 = 999,999,999,900
    ///     assert_eq!((&Natural::from_str("999999999900").unwrap()).div_exact(123).to_string(),
    ///         "8130081300");
    /// }
    /// ```
    fn div_exact(self, other: u32) -> Natural {
        if other == 0 {
            panic!("division by zero");
        } else if other == 1 {
            self.clone()
        } else {
            match *self {
                Small(small) => Small(small / other),
                Large(ref limbs) => {
                    let mut quotient = Large(limbs_div_exact_limb(limbs, other));
                    quotient.trim();
                    quotient
                }
            }
        }
    }
}

impl DivExactAssign<u32> for Natural {
    /// Divides a `Natural` by a `u32` in place. The `Natural` must be exactly divisible by the
    /// `u32`. If it isn't, the behavior of this function is undefined.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `other.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::DivExactAssign;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 3 * 123 = 369
    ///     let mut x = Natural::from(369u32);
    ///     x.div_exact_assign(123);
    ///     assert_eq!(x.to_string(), "3");
    ///
    ///     // 8,130,081,300 * 123 = 999,999,999,900
    ///     let mut x = Natural::from_str("999999999900").unwrap();
    ///     x.div_exact_assign(123);
    ///     assert_eq!(x.to_string(), "8130081300");
    /// }
    /// ```
    fn div_exact_assign(&mut self, other: u32) {
        if other == 0 {
            panic!("division by zero");
        } else if other != 1 {
            match *self {
                Small(ref mut small) => {
                    *small /= other;
                    return;
                }
                Large(ref mut limbs) => limbs_div_exact_limb_in_place(limbs, other),
            }
            self.trim();
        }
    }
}

impl DivExact<Natural> for u32 {
    type Output = u32;

    /// Divides a `u32` by a `Natural`, taking the `Natural` by value. The `Natural` must be exactly
    /// divisible by the `u32`. If it isn't, the behavior of this function is undefined.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::DivExact;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     // 3 * 123 = 369
    ///     assert_eq!(369.div_exact(Natural::from(123u32)), 3);
    /// }
    /// ```
    fn div_exact(self, other: Natural) -> u32 {
        if other == 0 {
            panic!("division by zero");
        } else {
            match other {
                Small(small) => self / small,
                _ => unreachable!(),
            }
        }
    }
}

impl<'a> DivExact<&'a Natural> for u32 {
    type Output = u32;

    /// Divides a `u32` by a `Natural`, taking the `Natural` by reference. The `Natural` must be
    /// exactly divisible by the `u32`. If it isn't, the behavior of this function is undefined.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::DivExact;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     // 3 * 123 = 369
    ///     assert_eq!(369.div_exact(&Natural::from(123u32)), 3);
    /// }
    /// ```
    fn div_exact(self, other: &'a Natural) -> u32 {
        if *other == 0 {
            panic!("division by zero");
        } else {
            match *other {
                Small(small) => self / small,
                _ => unreachable!(),
            }
        }
    }
}

impl DivExactAssign<Natural> for u32 {
    /// Divides a `u32` by a `Natural` in place, taking the `Natural` by value. The `Natural` must
    /// be exactly divisible by the `u32`. If it isn't, the behavior of this function is undefined.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::DivExactAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     // 3 * 123 = 369
    ///     let mut n = 369;
    ///     n.div_exact_assign(Natural::from(123u32));
    ///     assert_eq!(n, 3);
    /// }
    /// ```
    fn div_exact_assign(&mut self, other: Natural) {
        self.div_exact_assign(&other);
    }
}

impl<'a> DivExactAssign<&'a Natural> for u32 {
    /// Divides a `u32` by a `Natural` in place, taking the `Natural` by reference. The `Natural`
    /// must be exactly divisible by the `u32`. If it isn't, the behavior of this function is
    /// undefined.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::DivExactAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     // 3 * 123 = 369
    ///     let mut n = 369;
    ///     n.div_exact_assign(&Natural::from(123u32));
    ///     assert_eq!(n, 3);
    /// }
    /// ```
    fn div_exact_assign(&mut self, other: &'a Natural) {
        *self = self.div_exact(other);
    }
}
