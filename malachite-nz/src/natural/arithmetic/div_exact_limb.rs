use malachite_base::misc::Max;
use malachite_base::num::{
    DivExact, DivExactAssign, ModPowerOfTwo, Parity, PrimitiveInteger, SplitInHalf,
    WrappingSubAssign,
};
use natural::Natural::{self, Large, Small};
use platform::{DoubleLimb, Limb};

// These functions are adapted from mpn_divexact_1, mpn_bdiv_dbm1c, and
// mpn_divexact_by3c in GMP 6.1.2.

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
/// use malachite_nz::natural::arithmetic::div_exact_limb::test_invert_limb_table;
///
/// test_invert_limb_table();
/// ```
pub fn test_invert_limb_table() {
    for (i, &inverse) in INVERT_LIMB_TABLE.iter().enumerate() {
        let value = ((i as u8) << 1) + 1;
        let product = value.wrapping_mul(inverse);
        assert_eq!(
            product, 1,
            "INVERT_LIMB_TABLE gives incorrect inverse, {}, for value {}",
            inverse, value
        );
    }
}

/// Finds the inverse of a `Limb` mod 2<sup>32</sup>; given x, returns y such that
/// x * y === 1 mod 2<sup>32</sup>. This inverse only exists for odd `Limb`s, so `limb` must be odd.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Panics
/// Panics if `limb` is even.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::div_exact_limb::limbs_invert_limb;
///
/// assert_eq!(limbs_invert_limb(3), 2_863_311_531);
/// assert_eq!(limbs_invert_limb(1_000_000_001), 2_211_001_857);
/// ```
///
/// This is binvert_limb from gmp-impl.h.
pub fn limbs_invert_limb(limb: Limb) -> Limb {
    assert!(limb.odd());
    let index = (limb >> 1).mod_power_of_two(INVERT_LIMB_TABLE_LOG_SIZE);
    let mut inverse: Limb = INVERT_LIMB_TABLE[index as usize].into();
    inverse = (inverse << 1).wrapping_sub((inverse * inverse).wrapping_mul(limb));
    inverse = (inverse << 1).wrapping_sub(inverse.wrapping_mul(inverse).wrapping_mul(limb));
    if cfg!(feature = "64_bit_limbs") {
        inverse = (inverse << 1).wrapping_sub(inverse.wrapping_mul(inverse).wrapping_mul(limb));
    }
    inverse
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
/// quotient limbs of the `Natural` divided by a `Limb`. The divisor limb cannot be zero and the
/// limb slice must be nonempty. The `Natural` must be exactly divisible by the `Limb`. If it isn't,
/// the behavior of this function is undefined.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if `limbs` is empty or if `divisor` is zero.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::div_exact_limb::limbs_div_exact_limb;
///
/// assert_eq!(limbs_div_exact_limb(&[6, 7], 2), &[2_147_483_651, 3]);
/// assert_eq!(limbs_div_exact_limb(&[0xffff_ffff, 0xffff_ffff], 3), &[0x5555_5555, 0x5555_5555]);
/// ```
pub fn limbs_div_exact_limb(limbs: &[Limb], divisor: Limb) -> Vec<Limb> {
    let mut quotient = vec![0; limbs.len()];
    limbs_div_exact_limb_to_out(&mut quotient, limbs, divisor);
    quotient
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the quotient of the `Natural` and a `Limb` to an output slice. The output slice must be
/// at least as long as the input slice. The divisor limb cannot be zero and the input limb slice
/// must be nonempty. The `Natural` must be exactly divisible by the `Limb`. If it isn't, the
/// behavior of this function is undefined.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if `out_limbs` is shorter than `in_limbs`, `in_limbs` is empty, or if `divisor` is zero.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::div_exact_limb::limbs_div_exact_limb_to_out;
///
/// let mut out_limbs = vec![10, 10, 10, 10];
/// limbs_div_exact_limb_to_out(&mut out_limbs, &[6, 7], 2);
/// assert_eq!(out_limbs, &[2_147_483_651, 3, 10, 10]);
///
/// let mut out_limbs = vec![10, 10, 10, 10];
/// limbs_div_exact_limb_to_out(&mut out_limbs, &[0xffff_ffff, 0xffff_ffff], 3);
/// assert_eq!(out_limbs, &[0x5555_5555, 0x5555_5555, 10, 10]);
/// ```
pub fn limbs_div_exact_limb_to_out(out_limbs: &mut [Limb], in_limbs: &[Limb], divisor: Limb) {
    assert_ne!(divisor, 0);
    let len = in_limbs.len();
    assert_ne!(len, 0);
    assert!(out_limbs.len() >= len);
    if divisor.even() {
        let shift = divisor.trailing_zeros();
        let shift_complement = Limb::WIDTH - shift;
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
            upper_half =
                (DoubleLimb::from(out_limb) * DoubleLimb::from(shifted_divisor)).upper_half();
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
            let mut upper_half =
                (DoubleLimb::from(out_limb) * DoubleLimb::from(divisor)).upper_half();
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

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the quotient of the `Natural` and a `Limb` to the input slice. The divisor limb cannot
/// be zero and the input limb slice must be nonempty. The `Natural` must be exactly divisible by
/// the `Limb`. If it isn't, the behavior of this function is undefined.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if `limbs` is empty or if `divisor` is zero.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::div_exact_limb::limbs_div_exact_limb_in_place;
///
/// let mut limbs = vec![6, 7];
/// limbs_div_exact_limb_in_place(&mut limbs, 2);
/// assert_eq!(limbs, &[2_147_483_651, 3]);
///
/// let mut limbs = vec![0xffff_ffff, 0xffff_ffff];
/// limbs_div_exact_limb_in_place(&mut limbs, 3);
/// assert_eq!(limbs, &[0x5555_5555, 0x5555_5555]);
/// ```
pub fn limbs_div_exact_limb_in_place(limbs: &mut [Limb], divisor: Limb) {
    assert_ne!(divisor, 0);
    let len = limbs.len();
    assert_ne!(len, 0);
    if divisor.even() {
        let shift = divisor.trailing_zeros();
        let shift_complement = Limb::WIDTH - shift;
        let shifted_divisor = divisor >> shift;
        let inverse = limbs_invert_limb(shifted_divisor);
        let shifted_divisor = DoubleLimb::from(shifted_divisor);
        let mut upper_half = 0;
        let mut previous_in_limb = limbs[0];
        for i in 1..len {
            let in_limb = limbs[i];
            let shifted_in_limb = (previous_in_limb >> shift) | (in_limb << shift_complement);
            previous_in_limb = in_limb;
            let (difference, carry) = shifted_in_limb.overflowing_sub(upper_half);
            let out_limb = difference.wrapping_mul(inverse);
            limbs[i - 1] = out_limb;
            upper_half = (DoubleLimb::from(out_limb) * shifted_divisor).upper_half();
            if carry {
                upper_half += 1;
            }
        }
        limbs[len - 1] = (previous_in_limb >> shift)
            .wrapping_sub(upper_half)
            .wrapping_mul(inverse);
    } else {
        let inverse = limbs_invert_limb(divisor);
        let divisor = DoubleLimb::from(divisor);
        let mut out_limb = limbs[0].wrapping_mul(inverse);
        limbs[0] = out_limb;
        let mut previous_carry = false;
        for limb in limbs[1..].iter_mut() {
            let mut upper_half = (DoubleLimb::from(out_limb) * divisor).upper_half();
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

const MAX_OVER_3: Limb = Limb::MAX / 3;
// This is MODLIMB_INVERSE_3 from gmp-impl.h.
const MODLIMB_INVERSE_3: Limb = (MAX_OVER_3 << 1) + 1;
const CEIL_MAX_OVER_3: Limb = MAX_OVER_3 + 1;
const CEIL_2_MAX_OVER_3: Limb = ((Limb::MAX >> 1) / 3 + 1) | (1 << (Limb::WIDTH - 1));

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
/// quotient limbs of the `Natural` divided by 3. The limb slice must be nonempty. The `Natural`
/// must be exactly divisible by 3. If it isn't, the behavior of this function is undefined.
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
/// use malachite_nz::natural::arithmetic::div_exact_limb::limbs_div_exact_3;
///
/// assert_eq!(limbs_div_exact_3(&[8, 7]), &[1_431_655_768, 2]);
/// assert_eq!(limbs_div_exact_3(&[0xffff_ffff, 0xffff_ffff]), &[0x5555_5555, 0x5555_5555]);
/// ```
pub fn limbs_div_exact_3(limbs: &[Limb]) -> Vec<Limb> {
    let mut quotient = vec![0; limbs.len()];
    limbs_div_exact_3_to_out(&mut quotient, limbs);
    quotient
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the quotient of the `Natural` and 3 to an output slice. The output slice must be at
/// least as long as the input slice. The input limb slice must be nonempty. The `Natural` must be
/// exactly divisible by 3. If it isn't, the behavior of this function is undefined.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if `out_limbs` is shorter than `in_limbs` or if `in_limbs` is empty.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::div_exact_limb::limbs_div_exact_3_to_out;
///
/// let mut out_limbs = vec![10, 10, 10, 10];
/// limbs_div_exact_3_to_out(&mut out_limbs, &[8, 7]);
/// assert_eq!(out_limbs, &[1_431_655_768, 2, 10, 10]);
///
/// let mut out_limbs = vec![10, 10, 10, 10];
/// limbs_div_exact_3_to_out(&mut out_limbs, &[0xffff_ffff, 0xffff_ffff]);
/// assert_eq!(out_limbs, &[0x5555_5555, 0x5555_5555, 10, 10]);
/// ```
pub fn limbs_div_exact_3_to_out(out_limbs: &mut [Limb], in_limbs: &[Limb]) {
    const MAX_OVER_3_U64: DoubleLimb = MAX_OVER_3 as DoubleLimb;
    let len = in_limbs.len();
    assert_ne!(len, 0);
    assert!(out_limbs.len() >= len);
    let last_index = len - 1;
    let mut out_limb = 0;
    for i in 0..last_index {
        let (upper, lower) = (DoubleLimb::from(in_limbs[i]) * MAX_OVER_3_U64).split_in_half();
        let carry = out_limb < lower;
        out_limb.wrapping_sub_assign(lower);
        out_limbs[i] = out_limb;
        out_limb.wrapping_sub_assign(upper);
        if carry {
            out_limb.wrapping_sub_assign(1);
        }
    }
    let lower = (DoubleLimb::from(in_limbs[last_index]) * MAX_OVER_3_U64).lower_half();
    out_limbs[last_index] = out_limb.wrapping_sub(lower);
}

// Benchmarks show that this algorithm is always worse than the default.
pub fn _limbs_div_exact_3_to_out_alt(out_limbs: &mut [Limb], in_limbs: &[Limb]) {
    let len = in_limbs.len();
    assert_ne!(len, 0);
    assert!(out_limbs.len() >= len);
    let last_index = len - 1;
    let mut big_carry = 0;
    for i in 0..last_index {
        let (difference, carry) = in_limbs[i].overflowing_sub(big_carry);
        big_carry = if carry { 1 } else { 0 };
        let out_limb = difference.wrapping_mul(MODLIMB_INVERSE_3);
        out_limbs[i] = out_limb;
        if out_limb >= CEIL_MAX_OVER_3 {
            big_carry += 1;
            if out_limb >= CEIL_2_MAX_OVER_3 {
                big_carry += 1;
            }
        }
    }
    out_limbs[last_index] = in_limbs[last_index]
        .wrapping_sub(big_carry)
        .wrapping_mul(MODLIMB_INVERSE_3);
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the quotient of the `Natural` and 3 to the input slice. The input limb slice must be
/// nonempty. The `Natural` must be exactly divisible by 3. If it isn't, the behavior of this
/// function is undefined.
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
/// use malachite_nz::natural::arithmetic::div_exact_limb::limbs_div_exact_3_in_place;
///
/// let mut limbs = vec![8, 7];
/// limbs_div_exact_3_in_place(&mut limbs);
/// assert_eq!(limbs, &[1_431_655_768, 2]);
///
/// let mut limbs = vec![0xffff_ffff, 0xffff_ffff];
/// limbs_div_exact_3_in_place(&mut limbs);
/// assert_eq!(limbs, &[0x5555_5555, 0x5555_5555]);
/// ```
pub fn limbs_div_exact_3_in_place(limbs: &mut [Limb]) {
    const MAX_OVER_3_U64: DoubleLimb = MAX_OVER_3 as DoubleLimb;
    let len = limbs.len();
    assert_ne!(len, 0);
    let last_index = len - 1;
    let mut out_limb = 0;
    for limb in limbs[..last_index].iter_mut() {
        let (upper, lower) = (DoubleLimb::from(*limb) * MAX_OVER_3_U64).split_in_half();
        let carry = out_limb < lower;
        out_limb.wrapping_sub_assign(lower);
        *limb = out_limb;
        out_limb.wrapping_sub_assign(upper);
        if carry {
            out_limb.wrapping_sub_assign(1);
        }
    }
    let lower = (DoubleLimb::from(limbs[last_index]) * MAX_OVER_3_U64).lower_half();
    limbs[last_index] = out_limb.wrapping_sub(lower);
}

// Benchmarks show that this algorithm is always worse than the default.
pub fn _limbs_div_exact_3_in_place_alt(limbs: &mut [Limb]) {
    let len = limbs.len();
    assert_ne!(len, 0);
    let last_index = len - 1;
    let mut big_carry = 0;
    for limb in limbs[..last_index].iter_mut() {
        let (difference, carry) = limb.overflowing_sub(big_carry);
        big_carry = if carry { 1 } else { 0 };
        let out_limb = difference.wrapping_mul(MODLIMB_INVERSE_3);
        *limb = out_limb;
        if out_limb >= CEIL_MAX_OVER_3 {
            big_carry += 1;
            if out_limb >= CEIL_2_MAX_OVER_3 {
                big_carry += 1;
            }
        }
    }
    limbs[last_index] = limbs[last_index]
        .wrapping_sub(big_carry)
        .wrapping_mul(MODLIMB_INVERSE_3);
}

impl DivExact<Limb> for Natural {
    type Output = Natural;

    /// Divides a `Natural` by a `Limb`, taking the `Natural` by value. The `Natural` must be
    /// exactly divisible by the `Limb`. If it isn't, the behavior of this function is undefined.
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
    fn div_exact(mut self, other: Limb) -> Natural {
        self.div_exact_assign(other);
        self
    }
}

impl<'a> DivExact<Limb> for &'a Natural {
    type Output = Natural;

    /// Divides a `Natural` by a `Limb`, taking the `Natural` by reference. The `Natural` must be
    /// exactly divisible by the `Limb`. If it isn't, the behavior of this function is undefined.
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
    fn div_exact(self, other: Limb) -> Natural {
        if other == 0 {
            panic!("division by zero");
        } else if other == 1 {
            self.clone()
        } else {
            match *self {
                Small(small) => Small(small / other),
                Large(ref limbs) => {
                    let mut quotient = Large(if other == 3 {
                        limbs_div_exact_3(limbs)
                    } else {
                        limbs_div_exact_limb(limbs, other)
                    });
                    quotient.trim();
                    quotient
                }
            }
        }
    }
}

impl DivExactAssign<Limb> for Natural {
    /// Divides a `Natural` by a `Limb` in place. The `Natural` must be exactly divisible by the
    /// `Limb`. If it isn't, the behavior of this function is undefined.
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
    fn div_exact_assign(&mut self, other: Limb) {
        if other == 0 {
            panic!("division by zero");
        } else if other != 1 {
            match *self {
                Small(ref mut small) => {
                    *small /= other;
                    return;
                }
                Large(ref mut limbs) => {
                    if other == 3 {
                        limbs_div_exact_3_in_place(limbs)
                    } else {
                        limbs_div_exact_limb_in_place(limbs, other)
                    }
                }
            }
            self.trim();
        }
    }
}

impl DivExact<Natural> for Limb {
    type Output = Limb;

    /// Divides a `Limb` by a `Natural`, taking the `Natural` by value. The `Limb` must be exactly
    /// divisible by the `Natural`. If it isn't, the behavior of this function is undefined.
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
    fn div_exact(self, other: Natural) -> Limb {
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

impl<'a> DivExact<&'a Natural> for Limb {
    type Output = Limb;

    /// Divides a `Limb` by a `Natural`, taking the `Natural` by reference. The `Limb` must be
    /// exactly divisible by the `Natural`. If it isn't, the behavior of this function is undefined.
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
    fn div_exact(self, other: &'a Natural) -> Limb {
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

impl DivExactAssign<Natural> for Limb {
    /// Divides a `Limb` by a `Natural` in place, taking the `Natural` by value. The `Limb` must be
    /// exactly divisible by the `Natural`. If it isn't, the behavior of this function is undefined.
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

impl<'a> DivExactAssign<&'a Natural> for Limb {
    /// Divides a `Limb` by a `Natural` in place, taking the `Natural` by reference. The `Limb` must
    /// be exactly divisible by the `Natural`. If it isn't, the behavior of this function is
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

impl Natural {
    pub fn _div_exact_no_special_case_3(&self, other: Limb) -> Natural {
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

    pub fn _div_exact_assign_no_special_case_3(&mut self, other: Limb) {
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
