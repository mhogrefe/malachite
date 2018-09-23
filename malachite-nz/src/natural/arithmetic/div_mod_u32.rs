use malachite_base::num::{
    CeilingDivAssignNegMod, CeilingDivNegMod, DivAssignMod, DivAssignRem, DivMod, DivRem,
    JoinHalves, SplitInHalf,
};
use natural::arithmetic::add_u32::limbs_slice_add_limb_in_place;
use natural::arithmetic::shl_u::{limbs_shl_to_out, limbs_slice_shl_in_place};
use natural::Natural::{self, Large, Small};
use std::u32;

// These functions are adapted from udiv_qrnnd_preinv, mpn_div_qr_1n_pi1, and mpn_div_qr_1 in GMP
// 6.1.2.

pub(crate) fn div_mod_by_preinversion(
    n_high: u32,
    n_low: u32,
    divisor: u32,
    divisor_inverse: u32,
) -> (u32, u32) {
    let (mut quotient_high, quotient_low) = (u64::from(n_high) * u64::from(divisor_inverse))
        .wrapping_add(u64::join_halves(n_high.wrapping_add(1), n_low))
        .split_in_half();
    let mut remainder = n_low.wrapping_sub(quotient_high.wrapping_mul(divisor));
    if remainder > quotient_low {
        quotient_high = quotient_high.wrapping_sub(1);
        remainder = remainder.wrapping_add(divisor);
    }
    if remainder >= divisor {
        quotient_high = quotient_high.wrapping_add(1);
        remainder -= divisor;
    }
    (quotient_high, remainder)
}

// high bit of divisor must be set
fn limbs_div_limb_normalized_in_place_mod(
    limbs: &mut [u32],
    high_limb: u32,
    divisor: u32,
    divisor_inverse: u32,
) -> u32 {
    let len = limbs.len();
    if len == 1 {
        let (quotient, remainder) =
            div_mod_by_preinversion(high_limb, limbs[0], divisor, divisor_inverse);
        limbs[0] = quotient;
        return remainder;
    }
    let power_of_two = divisor.wrapping_neg().wrapping_mul(divisor_inverse);
    let (mut quotient_high, mut quotient_low) =
        (u64::from(divisor_inverse) * u64::from(high_limb)).split_in_half();
    quotient_high = quotient_high.wrapping_add(high_limb);
    let second_highest_limb = limbs[len - 1];
    limbs[len - 1] = quotient_high;
    let (sum, mut big_carry) = u64::join_halves(second_highest_limb, limbs[len - 2])
        .overflowing_add(u64::from(power_of_two) * u64::from(high_limb));
    let (mut sum_high, mut sum_low) = sum.split_in_half();
    for j in (0..len - 2).rev() {
        let (temp, remainder) = (u64::from(sum_high) * u64::from(divisor_inverse)).split_in_half();
        let mut quotient = u64::from(sum_high) + u64::from(temp) + u64::from(quotient_low);
        quotient_low = remainder;
        if big_carry {
            quotient = quotient.wrapping_add(u64::join_halves(1, divisor_inverse));
            let (sum, carry) = sum_low.overflowing_add(power_of_two);
            sum_low = sum;
            if carry {
                sum_low = sum_low.wrapping_sub(divisor);
                quotient = quotient.wrapping_add(1);
            }
        }
        let (quotient_higher, quotient_high) = quotient.split_in_half();
        limbs[j + 1] = quotient_high;
        assert!(!limbs_slice_add_limb_in_place(
            &mut limbs[j + 2..],
            quotient_higher
        ));
        let (sum, carry) = u64::join_halves(sum_low, limbs[j])
            .overflowing_add(u64::from(sum_high) * u64::from(power_of_two));
        sum_high = sum.upper_half();
        sum_low = sum.lower_half();
        big_carry = carry;
    }
    let mut quotient_high = 0;
    if big_carry {
        quotient_high += 1;
        sum_high = sum_high.wrapping_sub(divisor);
    }
    if sum_high >= divisor {
        quotient_high += 1;
        sum_high = sum_high.wrapping_sub(divisor);
    }
    let (temp, remainder) = div_mod_by_preinversion(sum_high, sum_low, divisor, divisor_inverse);
    let (quotient_high, quotient_low) = u64::join_halves(quotient_high, quotient_low)
        .wrapping_add(u64::from(temp))
        .split_in_half();
    assert!(!limbs_slice_add_limb_in_place(
        &mut limbs[1..],
        quotient_high
    ));
    limbs[0] = quotient_low;
    return remainder;
}

// high bit of divisor must be set
fn limbs_div_limb_normalized_to_out_mod(
    out_limbs: &mut [u32],
    in_limbs: &[u32],
    high_limb: u32,
    divisor: u32,
    divisor_inverse: u32,
) -> u32 {
    let len = in_limbs.len();
    if len == 1 {
        let (quotient, remainder) =
            div_mod_by_preinversion(high_limb, in_limbs[0], divisor, divisor_inverse);
        out_limbs[0] = quotient;
        return remainder;
    }
    let power_of_two = divisor.wrapping_neg().wrapping_mul(divisor_inverse);
    let (mut quotient_high, mut quotient_low) =
        (u64::from(divisor_inverse) * u64::from(high_limb)).split_in_half();
    quotient_high = quotient_high.wrapping_add(high_limb);
    out_limbs[len - 1] = quotient_high;
    let (sum, mut big_carry) = u64::join_halves(in_limbs[len - 1], in_limbs[len - 2])
        .overflowing_add(u64::from(power_of_two) * u64::from(high_limb));
    let (mut sum_high, mut sum_low) = sum.split_in_half();
    for j in (0..len - 2).rev() {
        let (temp, remainder) = (u64::from(sum_high) * u64::from(divisor_inverse)).split_in_half();
        let mut quotient = u64::from(sum_high) + u64::from(temp) + u64::from(quotient_low);
        quotient_low = remainder;
        if big_carry {
            quotient = quotient.wrapping_add(u64::join_halves(1, divisor_inverse));
            let (sum, carry) = sum_low.overflowing_add(power_of_two);
            sum_low = sum;
            if carry {
                sum_low = sum_low.wrapping_sub(divisor);
                quotient = quotient.wrapping_add(1);
            }
        }
        let (quotient_higher, quotient_high) = quotient.split_in_half();
        out_limbs[j + 1] = quotient_high;
        assert!(!limbs_slice_add_limb_in_place(
            &mut out_limbs[j + 2..],
            quotient_higher
        ));
        let (sum, carry) = u64::join_halves(sum_low, in_limbs[j])
            .overflowing_add(u64::from(sum_high) * u64::from(power_of_two));
        sum_high = sum.upper_half();
        sum_low = sum.lower_half();
        big_carry = carry;
    }
    let mut quotient_high = 0;
    if big_carry {
        quotient_high += 1;
        sum_high = sum_high.wrapping_sub(divisor);
    }
    if sum_high >= divisor {
        quotient_high += 1;
        sum_high = sum_high.wrapping_sub(divisor);
    }
    let (temp, remainder) = div_mod_by_preinversion(sum_high, sum_low, divisor, divisor_inverse);
    let (quotient_high, quotient_low) = u64::join_halves(quotient_high, quotient_low)
        .wrapping_add(u64::from(temp))
        .split_in_half();
    assert!(!limbs_slice_add_limb_in_place(
        &mut out_limbs[1..],
        quotient_high
    ));
    out_limbs[0] = quotient_low;
    return remainder;
}

/// Interpreting a slice of `u32`s as the limbs (in ascending order) of a `Natural`, returns the
/// quotient limbs and remainder of the `Natural` divided by a `u32`. The divisor limb cannot be
/// zero and the limb slice must have at least two elements.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if the length of `limbs` is less than 2 or if `divisor` is zero.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::div_mod_u32::limbs_div_limb_mod;
///
/// assert_eq!(limbs_div_limb_mod(&[123, 456], 789), (vec![2_482_262_467, 0], 636));
/// assert_eq!(limbs_div_limb_mod(&[0xffff_ffff, 0xffff_ffff], 3),
///     (vec![0x5555_5555, 0x5555_5555], 0));
/// ```
pub fn limbs_div_limb_mod(limbs: &[u32], divisor: u32) -> (Vec<u32>, u32) {
    let mut quotient_limbs = vec![0; limbs.len()];
    let remainder = limbs_div_limb_to_out_mod(&mut quotient_limbs, limbs, divisor);
    (quotient_limbs, remainder)
}

/// Interpreting a slice of `u32`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the quotient of the `Natural` and a `u32` to an output slice, and returns the
/// remainder. The output slice must be at least as long as the input slice. The divisor limb cannot
/// be zero and the input limb slice must have at least two elements.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if `out_limbs` is shorter than `in_limbs`, the length of `in_limbs` is less than 2, or if
/// `divisor` is zero.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::div_mod_u32::limbs_div_limb_to_out_mod;
///
/// let mut out_limbs = vec![10, 10, 10, 10];
/// assert_eq!(limbs_div_limb_to_out_mod(&mut out_limbs, &[123, 456], 789), 636);
/// assert_eq!(out_limbs, &[2_482_262_467, 0, 10, 10]);
///
/// let mut out_limbs = vec![10, 10, 10, 10];
/// assert_eq!(limbs_div_limb_to_out_mod(&mut out_limbs, &[0xffff_ffff, 0xffff_ffff], 3), 0);
/// assert_eq!(out_limbs, &[0x5555_5555, 0x5555_5555, 10, 10]);
/// ```
pub fn limbs_div_limb_to_out_mod(out_limbs: &mut [u32], in_limbs: &[u32], mut divisor: u32) -> u32 {
    assert!(divisor > 0);
    let len = in_limbs.len();
    assert!(len > 1);
    assert!(out_limbs.len() >= len);
    let len_minus_1 = len - 1;
    let mut highest_limb = in_limbs[len_minus_1];
    let bits = divisor.leading_zeros();
    if bits == 0 {
        out_limbs[len_minus_1] = if highest_limb >= divisor {
            highest_limb -= divisor;
            1
        } else {
            0
        };
        let limb_inverse = (u64::join_halves(!divisor, u32::MAX) / u64::from(divisor)).lower_half();
        limbs_div_limb_normalized_to_out_mod(
            out_limbs,
            &in_limbs[..len_minus_1],
            highest_limb,
            divisor,
            limb_inverse,
        )
    } else {
        divisor <<= bits;
        let highest_limb = limbs_shl_to_out(out_limbs, in_limbs, bits);
        let limb_inverse = (u64::join_halves(!divisor, u32::MAX) / u64::from(divisor)).lower_half();
        let (quotient, remainder) =
            div_mod_by_preinversion(highest_limb, out_limbs[len_minus_1], divisor, limb_inverse);
        out_limbs[len_minus_1] = quotient;
        limbs_div_limb_normalized_in_place_mod(
            &mut out_limbs[..len_minus_1],
            remainder,
            divisor,
            limb_inverse,
        ) >> bits
    }
}

/// Interpreting a slice of `u32`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the quotient of the `Natural` and a `u32` to the input slice and returns the remainder.
/// The divisor limb cannot be zero and the input limb slice must have at least two elements.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if the length of `limbs` is less than 2 or if `divisor` is zero.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::div_mod_u32::limbs_div_limb_in_place_mod;
///
/// let mut limbs = vec![123, 456];
/// assert_eq!(limbs_div_limb_in_place_mod(&mut limbs, 789), 636);
/// assert_eq!(limbs, &[2_482_262_467, 0]);
///
/// let mut limbs = vec![0xffff_ffff, 0xffff_ffff];
/// assert_eq!(limbs_div_limb_in_place_mod(&mut limbs, 3), 0);
/// assert_eq!(limbs, &[0x5555_5555, 0x5555_5555]);
/// ```
pub fn limbs_div_limb_in_place_mod(limbs: &mut [u32], mut divisor: u32) -> u32 {
    assert!(divisor > 0);
    let len = limbs.len();
    assert!(len > 1);
    let len_minus_1 = len - 1;
    let mut highest_limb = limbs[len_minus_1];
    let bits = divisor.leading_zeros();
    if bits == 0 {
        limbs[len_minus_1] = if highest_limb >= divisor {
            highest_limb -= divisor;
            1
        } else {
            0
        };
        let limb_inverse = (u64::join_halves(!divisor, u32::MAX) / u64::from(divisor)).lower_half();
        limbs_div_limb_normalized_in_place_mod(
            &mut limbs[..len_minus_1],
            highest_limb,
            divisor,
            limb_inverse,
        )
    } else {
        divisor <<= bits;
        let highest_limb = limbs_slice_shl_in_place(limbs, bits);
        let limb_inverse = (u64::join_halves(!divisor, u32::MAX) / u64::from(divisor)).lower_half();
        let (quotient, remainder) =
            div_mod_by_preinversion(highest_limb, limbs[len_minus_1], divisor, limb_inverse);
        limbs[len_minus_1] = quotient;
        limbs_div_limb_normalized_in_place_mod(
            &mut limbs[..len_minus_1],
            remainder,
            divisor,
            limb_inverse,
        ) >> bits
    }
}

impl DivMod<u32> for Natural {
    type DivOutput = Natural;
    type ModOutput = u32;

    /// Divides a `Natural` by a `u32`, taking the `Natural` by value and returning the quotient and
    /// remainder. In other words, returns (q, r), where `self` = q * `other` + r and
    /// 0 <= r < `other`.
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
    /// use malachite_base::num::DivMod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 3 * 123 + 87 = 456
    ///     assert_eq!(format!("{:?}", Natural::from(456u32).div_mod(123)), "(3, 87)");
    ///
    ///     // 8,130,081,300 * 123 + 100 = 10^12
    ///     assert_eq!(format!("{:?}", Natural::trillion().div_mod(123)), "(8130081300, 100)");
    /// }
    /// ```
    fn div_mod(mut self, other: u32) -> (Natural, u32) {
        let remainder = self.div_assign_rem(other);
        (self, remainder)
    }
}

impl<'a> DivMod<u32> for &'a Natural {
    type DivOutput = Natural;
    type ModOutput = u32;

    /// Divides a `Natural` by a `u32`, taking the `Natural` by reference and returning the quotient
    /// and remainder. In other words, returns (q, r), where `self` = q * `other` + r and
    /// 0 <= r < `other`.
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
    /// use malachite_base::num::DivMod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 3 * 123 + 87 = 456
    ///     assert_eq!(format!("{:?}", (&Natural::from(456u32)).div_mod(123)), "(3, 87)");
    ///
    ///     // 8,130,081,300 * 123 + 100 = 10^12
    ///     assert_eq!(format!("{:?}", (&Natural::trillion()).div_mod(123)), "(8130081300, 100)");
    /// }
    /// ```
    fn div_mod(self, other: u32) -> (Natural, u32) {
        if other == 0 {
            panic!("division by zero");
        } else if other == 1 {
            (self.clone(), 0)
        } else {
            match *self {
                Small(small) => {
                    let (quotient, remainder) = small.div_rem(other);
                    (Small(quotient), remainder)
                }
                Large(ref limbs) => {
                    let (quotient_limbs, remainder) = limbs_div_limb_mod(limbs, other);
                    let mut quotient = Large(quotient_limbs);
                    quotient.trim();
                    (quotient, remainder)
                }
            }
        }
    }
}

impl DivAssignMod<u32> for Natural {
    type ModOutput = u32;

    /// Divides a `Natural` by a `u32` in place, returning the remainder. In other words, replaces
    /// `self` with q and returns r, where `self` = q * `other` + r and 0 <= r < `other`.
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
    /// use malachite_base::num::DivAssignMod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 3 * 123 + 87 = 456
    ///     let mut x = Natural::from(456u32);
    ///     assert_eq!(x.div_assign_mod(123), 87);
    ///     assert_eq!(x.to_string(), "3");
    ///
    ///     // 8,130,081,300 * 123 + 100 = 10^12
    ///     let mut x = Natural::trillion();
    ///     assert_eq!(x.div_assign_mod(123), 100);
    ///     assert_eq!(x.to_string(), "8130081300");
    /// }
    /// ```
    fn div_assign_mod(&mut self, other: u32) -> u32 {
        if other == 0 {
            panic!("division by zero");
        } else if other == 1 {
            0
        } else {
            let remainder = match *self {
                Small(ref mut small) => {
                    return small.div_assign_rem(other);
                }
                Large(ref mut limbs) => limbs_div_limb_in_place_mod(limbs, other),
            };
            self.trim();
            remainder
        }
    }
}

impl DivMod<Natural> for u32 {
    type DivOutput = u32;
    type ModOutput = u32;

    /// Divides a `u32` by a `Natural`, taking the `Natural` by value and returning the quotient and
    /// remainder. In other words, returns (q, r), where `self` = q * `other` + r and
    /// 0 <= r < `other`.
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
    /// use malachite_base::num::DivMod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 3 * 123 + 87 = 456
    ///     assert_eq!(456.div_mod(Natural::from(123u32)), (3, 87));
    ///
    ///     // 0 * 10^12 + 123 = 123
    ///     assert_eq!(123.div_mod(Natural::trillion()), (0, 123));
    /// }
    /// ```
    fn div_mod(self, other: Natural) -> (u32, u32) {
        self.div_mod(&other)
    }
}

impl<'a> DivMod<&'a Natural> for u32 {
    type DivOutput = u32;
    type ModOutput = u32;

    /// Divides a `u32` by a `Natural`, taking the `Natural` by reference and returning the quotient
    /// and remainder. In other words, returns (q, r), where `self` = q * `other` + r and
    /// 0 <= r < `other`.
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
    /// use malachite_base::num::DivMod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 3 * 123 + 87 = 456
    ///     assert_eq!(456.div_mod(&Natural::from(123u32)), (3, 87));
    ///
    ///     // 0 * 10^12 + 123 = 123
    ///     assert_eq!(123.div_mod(&Natural::trillion()), (0, 123));
    /// }
    /// ```
    fn div_mod(self, other: &'a Natural) -> (u32, u32) {
        if *other == 0 {
            panic!("division by zero");
        } else if *other == 1 {
            (self, 0)
        } else {
            match *other {
                Small(small) => self.div_rem(small),
                Large(_) => (0, self),
            }
        }
    }
}

impl DivAssignMod<Natural> for u32 {
    type ModOutput = u32;

    /// Divides a `u32` by a `Natural` in place, taking the `Natural` by value and returning the
    /// remainder. In other words, replaces `self` with q and returns r, where
    /// `self` = q * `other` + r and 0 <= r < `other`.
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
    /// use malachite_base::num::DivAssignMod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 3 * 123 + 87 = 456
    ///     let mut n = 456;
    ///     assert_eq!(n.div_assign_mod(Natural::from(123u32)), 87);
    ///     assert_eq!(n, 3);
    ///
    ///     // 0 * 10^12 + 123 = 123
    ///     let mut n = 123;
    ///     assert_eq!(n.div_assign_mod(Natural::trillion()), 123);
    ///     assert_eq!(n, 0);
    /// }
    /// ```
    fn div_assign_mod(&mut self, other: Natural) -> u32 {
        self.div_assign_mod(&other)
    }
}

impl<'a> DivAssignMod<&'a Natural> for u32 {
    type ModOutput = u32;

    /// Divides a `u32` by a `Natural` in place, taking the `Natural` by reference and returning the
    /// remainder. In other words, replaces `self` with q and returns r, where
    /// `self` = q * `other` + r and 0 <= r < `other`.
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
    /// use malachite_base::num::DivAssignMod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 3 * 123 + 87 = 456
    ///     let mut n = 456;
    ///     assert_eq!(n.div_assign_mod(&Natural::from(123u32)), 87);
    ///     assert_eq!(n, 3);
    ///
    ///     // 0 * 10^12 + 123 = 123
    ///     let mut n = 123;
    ///     assert_eq!(n.div_assign_mod(&Natural::trillion()), 123);
    ///     assert_eq!(n, 0);
    /// }
    /// ```
    fn div_assign_mod(&mut self, other: &'a Natural) -> u32 {
        let (q, r) = self.div_mod(other);
        *self = q;
        r
    }
}

impl DivRem<u32> for Natural {
    type DivOutput = Natural;
    type RemOutput = u32;

    /// Divides a `Natural` by a `u32`, taking the `Natural` by value and returning the quotient and
    /// remainder. For `Natural`s, rem is equivalent to mod.
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
    /// use malachite_base::num::DivRem;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 3 * 123 + 87 = 456
    ///     assert_eq!(format!("{:?}", Natural::from(456u32).div_rem(123)), "(3, 87)");
    ///
    ///     // 8,130,081,300 * 123 + 100 = 10^12
    ///     assert_eq!(format!("{:?}", Natural::trillion().div_rem(123)), "(8130081300, 100)");
    /// }
    /// ```
    fn div_rem(self, other: u32) -> (Natural, u32) {
        self.div_mod(other)
    }
}

impl<'a> DivRem<u32> for &'a Natural {
    type DivOutput = Natural;
    type RemOutput = u32;

    /// Divides a `Natural` by a `u32`, taking the `Natural` by reference and returning the quotient
    /// and remainder. For `Natural`s, rem is equivalent to mod.
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
    /// use malachite_base::num::DivRem;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 3 * 123 + 87 = 456
    ///     assert_eq!(format!("{:?}", (&Natural::from(456u32)).div_rem(123)), "(3, 87)");
    ///
    ///     // 8,130,081,300 * 123 + 100 = 10^12
    ///     assert_eq!(format!("{:?}", (&Natural::trillion()).div_rem(123)), "(8130081300, 100)");
    /// }
    /// ```
    fn div_rem(self, other: u32) -> (Natural, u32) {
        self.div_mod(other)
    }
}

impl DivAssignRem<u32> for Natural {
    type RemOutput = u32;

    /// Divides a `Natural` by a `u32` in place, returning the remainder. For `Natural`s, rem is
    /// equivalent to mod.
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
    /// use malachite_base::num::DivAssignRem;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 3 * 123 + 87 = 456
    ///     let mut x = Natural::from(456u32);
    ///     assert_eq!(x.div_assign_rem(123), 87);
    ///     assert_eq!(x.to_string(), "3");
    ///
    ///     // 8,130,081,300 * 123 + 100 = 10^12
    ///     let mut x = Natural::trillion();
    ///     assert_eq!(x.div_assign_rem(123), 100);
    ///     assert_eq!(x.to_string(), "8130081300");
    /// }
    /// ```
    fn div_assign_rem(&mut self, other: u32) -> u32 {
        self.div_assign_mod(other)
    }
}

impl DivRem<Natural> for u32 {
    type DivOutput = u32;
    type RemOutput = u32;

    /// Divides a `u32` by a `Natural`, taking the `Natural` by value and returning the quotient and
    /// remainder. For `Natural`s, rem is equivalent to mod.
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
    /// use malachite_base::num::DivRem;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 3 * 123 + 87 = 456
    ///     assert_eq!(456.div_rem(Natural::from(123u32)), (3, 87));
    ///
    ///     // 0 * 10^12 + 123 = 123
    ///     assert_eq!(123.div_rem(Natural::trillion()), (0, 123));
    /// }
    /// ```
    fn div_rem(self, other: Natural) -> (u32, u32) {
        self.div_mod(other)
    }
}

impl<'a> DivRem<&'a Natural> for u32 {
    type DivOutput = u32;
    type RemOutput = u32;

    /// Divides a `u32` by a `Natural`, taking the `Natural` by reference and returning the quotient
    /// and remainder. For `Natural`s, rem is equivalent to mod.
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
    /// use malachite_base::num::DivRem;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 3 * 123 + 87 = 456
    ///     assert_eq!(456.div_rem(&Natural::from(123u32)), (3, 87));
    ///
    ///     // 0 * 10^12 + 123 = 123
    ///     assert_eq!(123.div_rem(&Natural::trillion()), (0, 123));
    /// }
    /// ```
    fn div_rem(self, other: &'a Natural) -> (u32, u32) {
        self.div_mod(other)
    }
}

impl DivAssignRem<Natural> for u32 {
    type RemOutput = u32;

    /// Divides a `u32` by a `Natural` in place, taking the `Natural` by value and returning the
    /// remainder. For `Natural`s, rem is equivalent to mod.
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
    /// use malachite_base::num::DivAssignRem;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 3 * 123 + 87 = 456
    ///     let mut n = 456;
    ///     assert_eq!(n.div_assign_rem(Natural::from(123u32)), 87);
    ///     assert_eq!(n, 3);
    ///
    ///     // 0 * 10^12 + 123 = 123
    ///     let mut n = 123;
    ///     assert_eq!(n.div_assign_rem(Natural::trillion()), 123);
    ///     assert_eq!(n, 0);
    /// }
    /// ```
    fn div_assign_rem(&mut self, other: Natural) -> u32 {
        self.div_assign_mod(other)
    }
}

impl<'a> DivAssignRem<&'a Natural> for u32 {
    type RemOutput = u32;

    /// Divides a `u32` by a `Natural` in place, taking the `Natural` by reference and returning the
    /// remainder. For `Natural`s, rem is equivalent to mod.
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
    /// use malachite_base::num::DivAssignRem;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 3 * 123 + 87 = 456
    ///     let mut n = 456;
    ///     assert_eq!(n.div_assign_rem(&Natural::from(123u32)), 87);
    ///     assert_eq!(n, 3);
    ///
    ///     // 0 * 10^12 + 123 = 123
    ///     let mut n = 123;
    ///     assert_eq!(n.div_assign_rem(&Natural::trillion()), 123);
    ///     assert_eq!(n, 0);
    /// }
    /// ```
    fn div_assign_rem(&mut self, other: &'a Natural) -> u32 {
        self.div_assign_mod(other)
    }
}

impl CeilingDivNegMod<u32> for Natural {
    type DivOutput = Natural;
    type ModOutput = u32;

    /// Divides a `Natural` by a `u32`, taking the `Natural` by value and returning the ceiling of
    /// the quotient and the remainder of the negative of the `Natural` divided by the `u32`. In
    /// other words, returns (q, r), where `self` = q * `other` - r and 0 <= r < `other`.
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
    /// use malachite_base::num::CeilingDivNegMod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 4 * 123 - 36 = 456
    ///     assert_eq!(format!("{:?}", Natural::from(456u32).ceiling_div_neg_mod(123)), "(4, 36)");
    ///
    ///     // 8,130,081,301 * 123 - 23 = 10^12
    ///     assert_eq!(format!("{:?}", Natural::trillion().ceiling_div_neg_mod(123)),
    ///         "(8130081301, 23)");
    /// }
    /// ```
    fn ceiling_div_neg_mod(mut self, other: u32) -> (Natural, u32) {
        let remainder = self.ceiling_div_assign_neg_mod(other);
        (self, remainder)
    }
}

impl<'a> CeilingDivNegMod<u32> for &'a Natural {
    type DivOutput = Natural;
    type ModOutput = u32;

    /// Divides a `Natural` by a `u32`, taking the `Natural` by reference and returning the ceiling
    /// of the quotient and the remainder of the negative of the `Natural` divided by the `u32`. In
    /// other words, returns (q, r), where `self` = q * `other` - r and 0 <= r < `other`.
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
    /// use malachite_base::num::CeilingDivNegMod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 4 * 123 - 36 = 456
    ///     assert_eq!(format!("{:?}", (&Natural::from(456u32)).ceiling_div_neg_mod(123)),
    ///         "(4, 36)");
    ///
    ///     // 8,130,081,301 * 123 - 23 = 10^12
    ///     assert_eq!(format!("{:?}", (&Natural::trillion()).ceiling_div_neg_mod(123)),
    ///         "(8130081301, 23)");
    /// }
    /// ```
    fn ceiling_div_neg_mod(self, other: u32) -> (Natural, u32) {
        let (quotient, remainder) = self.div_mod(other);
        if remainder == 0 {
            (quotient, 0)
        } else {
            (quotient + 1, other - remainder)
        }
    }
}

impl CeilingDivAssignNegMod<u32> for Natural {
    type ModOutput = u32;

    /// Divides a `Natural` by a `u32` in place, taking the ceiling of the quotient and returning
    /// the remainder of the negative of the `Natural` divided by the `u32`. In other words,
    /// replaces `self` with q and returns r, where `self` = q * `other` - r and 0 <= r < `other`.
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
    /// use malachite_base::num::CeilingDivAssignNegMod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 4 * 123 - 36 = 456
    ///     let mut x = Natural::from(456u32);
    ///     assert_eq!(x.ceiling_div_assign_neg_mod(123), 36);
    ///     assert_eq!(x.to_string(), "4");
    ///
    ///     // 8,130,081,301 * 123 - 23 = 10^12
    ///     let mut x = Natural::trillion();
    ///     assert_eq!(x.ceiling_div_assign_neg_mod(123), 23);
    ///     assert_eq!(x.to_string(), "8130081301");
    /// }
    /// ```
    fn ceiling_div_assign_neg_mod(&mut self, other: u32) -> u32 {
        let remainder = self.div_assign_mod(other);
        if remainder == 0 {
            0
        } else {
            *self += 1;
            other - remainder
        }
    }
}

fn _limbs_div_in_place_mod_naive(limbs: &mut [u32], limb: u32) -> u32 {
    let limb = u64::from(limb);
    let mut upper = 0;
    for x in limbs.iter_mut().rev() {
        let lower = *x;
        let (q, r) = u64::join_halves(upper, lower).div_rem(limb);
        *x = q.lower_half();
        upper = r.lower_half();
    }
    upper
}

impl Natural {
    pub fn _div_mod_u32_naive(mut self, other: u32) -> (Natural, u32) {
        let remainder = self._div_assign_mod_u32_naive(other);
        (self, remainder)
    }

    pub fn _div_assign_mod_u32_naive(&mut self, other: u32) -> u32 {
        if other == 0 {
            panic!("division by zero");
        } else if other == 1 {
            0
        } else {
            let remainder = match *self {
                Small(ref mut small) => {
                    return small.div_assign_rem(other);
                }
                Large(ref mut limbs) => _limbs_div_in_place_mod_naive(limbs, other),
            };
            self.trim();
            remainder
        }
    }
}
