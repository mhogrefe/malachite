use malachite_base::num::{BitAccess, JoinHalves, PrimitiveInteger, SplitInHalf};
use natural::arithmetic::add_u32::limbs_slice_add_limb_in_place;
use natural::arithmetic::div_mod_u32::div_mod_by_preinversion;
use natural::arithmetic::shl_u::{limbs_shl_to_out, limbs_slice_shl_in_place};
use natural::Natural::{self, Large, Small};
use std::ops::{Div, DivAssign};
use std::u32;

// These functions are adapted from udiv_qrnnd_preinv, mpn_div_qr_1n_pi1, and mpn_div_qr_1 in GMP
// 6.1.2.

fn div_by_preinversion(n_high: u32, n_low: u32, divisor: u32, divisor_inverse: u32) -> u32 {
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
    }
    quotient_high
}

// high bit of divisor must be set
fn limbs_div_normalized_in_place(
    limbs: &mut [u32],
    high_limb: u32,
    divisor: u32,
    divisor_inverse: u32,
) {
    let len = limbs.len();
    if len == 1 {
        limbs[0] = div_by_preinversion(high_limb, limbs[0], divisor, divisor_inverse);
        return;
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
    let temp = div_by_preinversion(sum_high, sum_low, divisor, divisor_inverse);
    let (quotient_high, quotient_low) = u64::join_halves(quotient_high, quotient_low)
        .wrapping_add(u64::from(temp))
        .split_in_half();
    assert!(!limbs_slice_add_limb_in_place(
        &mut limbs[1..],
        quotient_high
    ));
    limbs[0] = quotient_low;
}

// high bit of divisor must be set
fn limbs_div_normalized_to_out(
    out_limbs: &mut [u32],
    in_limbs: &[u32],
    high_limb: u32,
    divisor: u32,
    divisor_inverse: u32,
) {
    let len = in_limbs.len();
    if len == 1 {
        out_limbs[0] = div_by_preinversion(high_limb, in_limbs[0], divisor, divisor_inverse);
        return;
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
    let temp = div_by_preinversion(sum_high, sum_low, divisor, divisor_inverse);
    let (quotient_high, quotient_low) = u64::join_halves(quotient_high, quotient_low)
        .wrapping_add(u64::from(temp))
        .split_in_half();
    assert!(!limbs_slice_add_limb_in_place(
        &mut out_limbs[1..],
        quotient_high
    ));
    out_limbs[0] = quotient_low;
}

/// Interpreting a slice of `u32`s as the limbs (in ascending order) of a `Natural`, returns the
/// quotient limbs of the `Natural` divided by a `u32`. The divisor limb cannot be zero and the limb
/// slice must have at least two elements.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if the length of `limbs` is less than 2 or if `limb` is zero.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::div_u32::limbs_div_limb;
///
/// assert_eq!(limbs_div_limb(&[123, 456], 789), &[2_482_262_467, 0]);
/// assert_eq!(limbs_div_limb(&[0xffff_ffff, 0xffff_ffff], 3), &[0x5555_5555, 0x5555_5555]);
/// ```
pub fn limbs_div_limb(limbs: &[u32], limb: u32) -> Vec<u32> {
    let mut quotient_limbs = vec![0; limbs.len()];
    limbs_div_limb_to_out(&mut quotient_limbs, limbs, limb);
    quotient_limbs
}

/// Interpreting a slice of `u32`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the quotient of the `Natural` and a `u32` to an output slice. The output slice must be
/// at least as long as the input slice. The divisor limb cannot be zero and the input limb slice
/// must have at least two elements.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if `out_limbs` is shorter than `in_limbs`, the length of `in_limbs` is less than 2, or if
/// `limb` is zero.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::div_u32::limbs_div_limb_to_out;
///
/// let mut out_limbs = vec![10, 10, 10, 10];
/// limbs_div_limb_to_out(&mut out_limbs, &[123, 456], 789);
/// assert_eq!(out_limbs, &[2_482_262_467, 0, 10, 10]);
///
/// let mut out_limbs = vec![10, 10, 10, 10];
/// limbs_div_limb_to_out(&mut out_limbs, &[0xffff_ffff, 0xffff_ffff], 3);
/// assert_eq!(out_limbs, &[0x5555_5555, 0x5555_5555, 10, 10]);
/// ```
pub fn limbs_div_limb_to_out(out_limbs: &mut [u32], in_limbs: &[u32], mut limb: u32) {
    let len = in_limbs.len();
    assert!(out_limbs.len() >= len);
    assert!(len > 1);
    assert!(limb > 0);
    let len_minus_1 = len - 1;
    let mut highest_limb = in_limbs[len_minus_1];
    if limb.get_bit(u64::from(u32::WIDTH) - 1) {
        out_limbs[len_minus_1] = if highest_limb >= limb {
            highest_limb -= limb;
            1
        } else {
            0
        };
        let limb_inverse = (u64::join_halves(!limb, u32::MAX) / u64::from(limb)) as u32;
        limbs_div_normalized_to_out(
            out_limbs,
            &in_limbs[..len_minus_1],
            highest_limb,
            limb,
            limb_inverse,
        )
    } else {
        let bits = limb.leading_zeros();
        limb <<= bits;
        let highest_limb = limbs_shl_to_out(out_limbs, in_limbs, bits);
        let limb_inverse = (u64::join_halves(!limb, u32::MAX) / u64::from(limb)) as u32;
        let (quotient, remainder) =
            div_mod_by_preinversion(highest_limb, out_limbs[len_minus_1], limb, limb_inverse);
        out_limbs[len_minus_1] = quotient;
        limbs_div_normalized_in_place(&mut out_limbs[..len_minus_1], remainder, limb, limb_inverse)
    }
}

/// Interpreting a slice of `u32`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the quotient of the `Natural` and a `u32` to the input slice. The divisor limb cannot
/// be zero and the input limb slice must have at least two elements.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if the length of `limbs` is less than 2 or if `limb` is zero.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::div_u32::limbs_div_limb_in_place;
///
/// let mut limbs = vec![123, 456];
/// limbs_div_limb_in_place(&mut limbs, 789);
/// assert_eq!(limbs, &[2_482_262_467, 0]);
///
/// let mut limbs = vec![0xffff_ffff, 0xffff_ffff];
/// limbs_div_limb_in_place(&mut limbs, 3);
/// assert_eq!(limbs, &[0x5555_5555, 0x5555_5555]);
/// ```
pub fn limbs_div_limb_in_place(limbs: &mut [u32], mut limb: u32) {
    let len = limbs.len();
    assert!(len > 1);
    assert!(limb > 0);
    let len_minus_1 = len - 1;
    let mut highest_limb = limbs[len_minus_1];
    if limb.get_bit(u64::from(u32::WIDTH) - 1) {
        limbs[len_minus_1] = if highest_limb >= limb {
            highest_limb -= limb;
            1
        } else {
            0
        };
        let limb_inverse = (u64::join_halves(!limb, u32::MAX) / u64::from(limb)) as u32;
        limbs_div_normalized_in_place(&mut limbs[..len_minus_1], highest_limb, limb, limb_inverse)
    } else {
        let bits = limb.leading_zeros();
        limb <<= bits;
        let highest_limb = limbs_slice_shl_in_place(limbs, bits);
        let limb_inverse = (u64::join_halves(!limb, u32::MAX) / u64::from(limb)) as u32;
        let (quotient, remainder) =
            div_mod_by_preinversion(highest_limb, limbs[len_minus_1], limb, limb_inverse);
        limbs[len_minus_1] = quotient;
        limbs_div_normalized_in_place(&mut limbs[..len_minus_1], remainder, limb, limb_inverse)
    }
}

impl Div<u32> for Natural {
    type Output = Natural;

    /// Divides a `Natural` by a `u32`, taking the `Natural` by value. In other words, returns q,
    /// where `self` = q * `other` + r and 0 <= r < `other`.
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
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 3 * 123 + 87 = 456
    ///     assert_eq!((Natural::from(456u32) / 123).to_string(), "3");
    ///
    ///     // 8,130,081,300 * 123 + 100 = 10^12
    ///     assert_eq!((Natural::trillion() / 123).to_string(), "8130081300");
    /// }
    /// ```
    fn div(mut self, other: u32) -> Natural {
        self /= other;
        self
    }
}

impl<'a> Div<u32> for &'a Natural {
    type Output = Natural;

    /// Divides a `Natural` by a `u32`, taking the `Natural` by reference. In other words, returns
    /// q, where `self` = q * `other` + r and 0 <= r < `other`.
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
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 3 * 123 + 87 = 456
    ///     assert_eq!((&Natural::from(456u32) / 123).to_string(), "3");
    ///
    ///     // 8,130,081,300 * 123 + 100 = 10^12
    ///     assert_eq!((&Natural::trillion() / 123).to_string(), "8130081300");
    /// }
    /// ```
    fn div(self, other: u32) -> Natural {
        if other == 0 {
            panic!("division by zero");
        } else if other == 1 {
            self.clone()
        } else {
            match *self {
                Small(small) => Small(small / other),
                Large(ref limbs) => {
                    let mut quotient = Large(limbs_div_limb(limbs, other));
                    quotient.trim();
                    quotient
                }
            }
        }
    }
}

impl Div<Natural> for u32 {
    type Output = u32;

    /// Divides a `u32` by a `Natural`, taking the `Natural` by value. In other words, returns q,
    /// where `self` = q * `other` + r and 0 <= r < `other`.
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
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 3 * 123 + 87 = 456
    ///     assert_eq!(456 / Natural::from(123u32), 3);
    ///
    ///     // 0 * 10^12 + 123 = 123
    ///     assert_eq!(123 / Natural::trillion(), 0);
    /// }
    /// ```
    fn div(self, other: Natural) -> u32 {
        if other == 0 {
            panic!("division by zero");
        } else if other == 1 {
            self
        } else {
            match other {
                Small(small) => self / small,
                Large(_) => 0,
            }
        }
    }
}

impl<'a> Div<&'a Natural> for u32 {
    type Output = u32;

    /// Divides a `u32` by a `Natural`, taking the `Natural` by reference. In other words, returns
    /// q, where `self` = q * `other` + r and 0 <= r < `other`.
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
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 3 * 123 + 87 = 456
    ///     assert_eq!(456 / &Natural::from(123u32), 3);
    ///
    ///     // 0 * 10^12 + 123 = 123
    ///     assert_eq!(123 / &Natural::trillion(), 0);
    /// }
    /// ```
    fn div(self, other: &'a Natural) -> u32 {
        if *other == 0 {
            panic!("division by zero");
        } else if *other == 1 {
            self
        } else {
            match *other {
                Small(small) => self / small,
                Large(_) => 0,
            }
        }
    }
}

impl DivAssign<u32> for Natural {
    /// Divides a `Natural` by a `u32` in place. In other words, replaces `self` with q, where
    /// `self` = q * `other` + r and 0 <= r < `other`.
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
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 3 * 123 + 87 = 456
    ///     let mut x = Natural::from(456u32);
    ///     x /= 123;
    ///     assert_eq!(x.to_string(), "3");
    ///
    ///     // 8,130,081,300 * 123 + 100 = 10^12
    ///     let mut x = Natural::trillion();
    ///     x /= 123;
    ///     assert_eq!(x.to_string(), "8130081300");
    /// }
    /// ```
    fn div_assign(&mut self, other: u32) {
        if other == 0 {
            panic!("division by zero");
        } else if other != 1 {
            match *self {
                Small(ref mut small) => {
                    *small /= other;
                    return;
                }
                Large(ref mut limbs) => limbs_div_limb_in_place(limbs, other),
            }
            self.trim();
        }
    }
}

impl DivAssign<Natural> for u32 {
    /// Divides a `u32` by a `Natural` in place, taking the `Natural` by value. In other words,
    /// replaces `self` with q, where `self` = q * `other` + r and 0 <= r < `other`.
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
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 3 * 123 + 87 = 456
    ///     let mut n = 456;
    ///     n /= Natural::from(123u32);
    ///     assert_eq!(n, 3);
    ///
    ///     // 0 * 10^12 + 123 = 123
    ///     let mut n = 123;
    ///     n /= Natural::trillion();
    ///     assert_eq!(n, 0);
    /// }
    /// ```
    fn div_assign(&mut self, other: Natural) {
        *self /= &other;
    }
}

impl<'a> DivAssign<&'a Natural> for u32 {
    /// Divides a `u32` by a `Natural` in place, taking the `Natural` by reference. In other words,
    /// replaces `self` with q and returns r, where `self` = q * `other` + r and 0 <= r < `other`.
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
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 3 * 123 + 87 = 456
    ///     let mut n = 456;
    ///     n /= &Natural::from(123u32);
    ///     assert_eq!(n, 3);
    ///
    ///     // 0 * 10^12 + 123 = 123
    ///     let mut n = 123;
    ///     n /= &Natural::trillion();
    ///     assert_eq!(n, 0);
    /// }
    /// ```
    fn div_assign(&mut self, other: &'a Natural) {
        *self = *self / other;
    }
}
