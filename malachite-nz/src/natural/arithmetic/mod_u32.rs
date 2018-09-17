use malachite_base::num::{
    JoinHalves, Mod, ModAssign, NegMod, NegModAssign, PrimitiveInteger, SplitInHalf, Zero,
};
use natural::Natural::{self, Large, Small};
use std::ops::{Rem, RemAssign};
use std::u32;

// These functions are adapted from udiv_qrnnd_preinv, mpn_div_qr_1n_pi1, and mpn_div_qr_1 in GMP
// 6.1.2.

pub(crate) fn mod_by_preinversion(
    n_high: u32,
    n_low: u32,
    divisor: u32,
    divisor_inverse: u32,
) -> u32 {
    let (quotient_high, quotient_low) = (u64::from(n_high) * u64::from(divisor_inverse))
        .wrapping_add(u64::join_halves(n_high.wrapping_add(1), n_low))
        .split_in_half();
    let mut remainder = n_low.wrapping_sub(quotient_high.wrapping_mul(divisor));
    if remainder > quotient_low {
        remainder = remainder.wrapping_add(divisor);
    }
    if remainder >= divisor {
        remainder -= divisor;
    }
    remainder
}

// high bit of divisor must be set
fn limbs_mod_limb_normalized(
    limbs: &[u32],
    high_limb: u32,
    divisor: u32,
    divisor_inverse: u32,
) -> u32 {
    let len = limbs.len();
    if len == 1 {
        return mod_by_preinversion(high_limb, limbs[0], divisor, divisor_inverse);
    }
    let power_of_two = divisor.wrapping_neg().wrapping_mul(divisor_inverse);
    let (sum, mut big_carry) = u64::join_halves(limbs[len - 1], limbs[len - 2])
        .overflowing_add(u64::from(power_of_two) * u64::from(high_limb));
    let (mut sum_high, mut sum_low) = sum.split_in_half();
    for &limb in limbs[..len - 2].iter().rev() {
        if big_carry {
            let (sum, carry) = sum_low.overflowing_add(power_of_two);
            sum_low = sum;
            if carry {
                sum_low = sum_low.wrapping_sub(divisor);
            }
        }
        let (sum, carry) = u64::join_halves(sum_low, limb)
            .overflowing_add(u64::from(sum_high) * u64::from(power_of_two));
        sum_high = sum.upper_half();
        sum_low = sum.lower_half();
        big_carry = carry;
    }
    if big_carry {
        sum_high = sum_high.wrapping_sub(divisor);
    }
    if sum_high >= divisor {
        sum_high = sum_high.wrapping_sub(divisor);
    }
    mod_by_preinversion(sum_high, sum_low, divisor, divisor_inverse)
}

// high bit of divisor must be set
fn limbs_mod_limb_normalized_shl(
    limbs: &[u32],
    high_limb: u32,
    divisor: u32,
    divisor_inverse: u32,
    bits: u32,
) -> u32 {
    let len = limbs.len();
    if len == 1 {
        return mod_by_preinversion(high_limb, limbs[0] << bits, divisor, divisor_inverse);
    }
    let power_of_two = divisor.wrapping_neg().wrapping_mul(divisor_inverse);
    let cobits = u32::WIDTH - bits;
    let second_highest_limb = limbs[len - 2];
    let highest_limb_after_shl = (limbs[len - 1] << bits) | (second_highest_limb >> cobits);
    let mut second_highest_limb_after_shl = second_highest_limb << bits;
    if len > 2 {
        second_highest_limb_after_shl |= limbs[len - 3] >> cobits;
    }
    let (sum, mut big_carry) =
        u64::join_halves(highest_limb_after_shl, second_highest_limb_after_shl)
            .overflowing_add(u64::from(power_of_two) * u64::from(high_limb));
    let (mut sum_high, mut sum_low) = sum.split_in_half();
    for j in (0..len - 2).rev() {
        if big_carry {
            let (sum, carry) = sum_low.overflowing_add(power_of_two);
            sum_low = sum;
            if carry {
                sum_low = sum_low.wrapping_sub(divisor);
            }
        }
        let mut limb = limbs[j] << bits;
        if j != 0 {
            limb |= limbs[j - 1] >> cobits;
        }
        let (sum, carry) = u64::join_halves(sum_low, limb)
            .overflowing_add(u64::from(sum_high) * u64::from(power_of_two));
        sum_high = sum.upper_half();
        sum_low = sum.lower_half();
        big_carry = carry;
    }
    if big_carry {
        sum_high = sum_high.wrapping_sub(divisor);
    }
    if sum_high >= divisor {
        sum_high = sum_high.wrapping_sub(divisor);
    }
    mod_by_preinversion(sum_high, sum_low, divisor, divisor_inverse)
}

/// Interpreting a slice of `u32`s as the limbs (in ascending order) of a `Natural`, returns the
/// remainder when the `Natural` is divided by a `u32`.
///
/// The divisor limb cannot be zero and the input limb slice must have at least two elements.
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
/// use malachite_nz::natural::arithmetic::mod_u32::limbs_mod_limb;
///
/// assert_eq!(limbs_mod_limb(&[123, 456], 789), 636);
/// assert_eq!(limbs_mod_limb(&[0xffff_ffff, 0xffff_ffff], 3), 0);
/// ```
pub fn limbs_mod_limb(limbs: &[u32], mut limb: u32) -> u32 {
    let len = limbs.len();
    assert!(len > 1);
    assert!(limb > 0);
    let len_minus_1 = len - 1;
    let mut highest_limb = limbs[len_minus_1];
    let bits = limb.leading_zeros();
    if bits == 0 {
        if highest_limb >= limb {
            highest_limb -= limb;
        }
        let limb_inverse = (u64::join_halves(!limb, u32::MAX) / u64::from(limb)).lower_half();
        limbs_mod_limb_normalized(&limbs[..len_minus_1], highest_limb, limb, limb_inverse)
    } else {
        limb <<= bits;
        let cobits = u32::WIDTH - bits;
        let limb_inverse = (u64::join_halves(!limb, u32::MAX) / u64::from(limb)).lower_half();
        let remainder = mod_by_preinversion(
            highest_limb >> cobits,
            (highest_limb << bits) | (limbs[len - 2] >> cobits),
            limb,
            limb_inverse,
        );
        limbs_mod_limb_normalized_shl(&limbs[..len_minus_1], remainder, limb, limb_inverse, bits)
            >> bits
    }
}

impl Rem<u32> for Natural {
    type Output = u32;

    /// Divides a `Natural` by a `u32`, taking the `Natural` by value and returning the remainder.
    /// In other words, returns r, where `self` = q * `other` + r and 0 <= r < `other`.
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
    ///     assert_eq!(Natural::from(456u32) % 123, 87);
    ///
    ///     // 8,130,081,300 * 123 + 100 = 10^12
    ///     assert_eq!(Natural::trillion() % 123, 100);
    /// }
    /// ```
    fn rem(self, other: u32) -> u32 {
        &self % other
    }
}

impl<'a> Rem<u32> for &'a Natural {
    type Output = u32;

    /// Divides a `Natural` by a `u32`, taking the `Natural` by reference and returning the
    /// remainder. In other words, returns r, where `self` = q * `other` + r and
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
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 3 * 123 + 87 = 456
    ///     assert_eq!(&Natural::from(456u32) % 123, 87);
    ///
    ///     // 8,130,081,300 * 123 + 100 = 10^12
    ///     assert_eq!(&Natural::trillion() % 123, 100);
    /// }
    /// ```
    fn rem(self, other: u32) -> u32 {
        if other == 0 {
            panic!("division by zero");
        } else {
            match *self {
                Small(small) => small % other,
                Large(ref limbs) => limbs_mod_limb(limbs, other),
            }
        }
    }
}

impl RemAssign<u32> for Natural {
    /// Divides a `Natural` by a `u32`, replacing the `Natural` by the remainder. In other words,
    /// replaces `self` with r, where `self` = q * `other` + r and 0 <= r < `other`.
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
    ///     x %= 123;
    ///     assert_eq!(x.to_string(), "87");
    ///
    ///     // 8,130,081,300 * 123 + 100 = 10^12
    ///     let mut x = Natural::trillion();
    ///     x %= 123;
    ///     assert_eq!(x.to_string(), "100");
    /// }
    /// ```
    fn rem_assign(&mut self, other: u32) {
        if other == 0 {
            panic!("division by zero");
        } else {
            let remainder = match *self {
                Small(ref mut small) => {
                    *small %= other;
                    return;
                }
                Large(ref mut limbs) => limbs_mod_limb(limbs, other),
            };
            *self = Small(remainder);
        }
    }
}

impl Rem<Natural> for u32 {
    type Output = u32;

    /// Divides a `u32` by a `Natural`, taking the `Natural` by value and returning the remainder.
    /// In other words, returns r, where `self` = q * `other` + r and 0 <= r < `other`.
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
    ///     assert_eq!(456 % Natural::from(123u32), 87);
    ///
    ///     // 0 * 10^12 + 123 = 123
    ///     assert_eq!(123 % Natural::trillion(), 123);
    /// }
    /// ```
    fn rem(self, other: Natural) -> u32 {
        self % &other
    }
}

impl<'a> Rem<&'a Natural> for u32 {
    type Output = u32;

    /// Divides a `u32` by a `Natural`, taking the `Natural` by reference and returning the
    /// remainder. In other words, returns r, where `self` = q * `other` + r and 0 <= r < `other`.
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
    ///     assert_eq!(456 % &Natural::from(123u32), 87);
    ///
    ///     // 0 * 10^12 + 123 = 123
    ///     assert_eq!(123 % &Natural::trillion(), 123);
    /// }
    /// ```
    fn rem(self, other: &'a Natural) -> u32 {
        if *other == 0 {
            panic!("division by zero");
        } else if *other == 1 {
            0
        } else {
            match *other {
                Small(small) => self % small,
                Large(_) => self,
            }
        }
    }
}

impl RemAssign<Natural> for u32 {
    /// Divides a `u32` by a `Natural` in place, taking the `Natural` by value and replacing the
    /// `u32` with the remainder. In other words, replaces `self` with r, where
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
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 3 * 123 + 87 = 456
    ///     let mut n = 456;
    ///     n %= Natural::from(123u32);
    ///     assert_eq!(n, 87);
    ///
    ///     // 0 * 10^12 + 123 = 123
    ///     let mut n = 123;
    ///     n %= Natural::trillion();
    ///     assert_eq!(n, 123);
    /// }
    /// ```
    fn rem_assign(&mut self, other: Natural) {
        *self %= &other;
    }
}

impl<'a> RemAssign<&'a Natural> for u32 {
    /// Divides a `u32` by a `Natural` in place taking the `Natural` by reference and replacing the
    /// `u32` with the remainder. In other words, replaces `self` with r, where
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
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 3 * 123 + 87 = 456
    ///     let mut n = 456;
    ///     n %= &Natural::from(123u32);
    ///     assert_eq!(n, 87);
    ///
    ///     // 0 * 10^12 + 123 = 123
    ///     let mut n = 123;
    ///     n %= &Natural::trillion();
    ///     assert_eq!(n, 123);
    /// }
    /// ```
    fn rem_assign(&mut self, other: &'a Natural) {
        *self = *self % other;
    }
}

impl Mod<u32> for Natural {
    type Output = u32;

    /// Divides a `Natural` by a `u32`, taking the `Natural` by value and returning the remainder.
    /// For `Natural`s, rem is equivalent to mod.
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
    /// use malachite_base::num::Mod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 3 * 123 + 87 = 456
    ///     assert_eq!(Natural::from(456u32).mod_op(123), 87);
    ///
    ///     // 8,130,081,300 * 123 + 100 = 10^12
    ///     assert_eq!(Natural::trillion().mod_op(123), 100);
    /// }
    /// ```
    fn mod_op(self, other: u32) -> u32 {
        self % other
    }
}

impl<'a> Mod<u32> for &'a Natural {
    type Output = u32;

    /// Divides a `Natural` by a `u32`, taking the `Natural` by reference and returning the
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
    /// use malachite_base::num::Mod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 3 * 123 + 87 = 456
    ///     assert_eq!((&Natural::from(456u32)).mod_op(123), 87);
    ///
    ///     // 8,130,081,300 * 123 + 100 = 10^12
    ///     assert_eq!((&Natural::trillion()).mod_op(123), 100);
    /// }
    /// ```
    fn mod_op(self, other: u32) -> u32 {
        self % other
    }
}

impl ModAssign<u32> for Natural {
    /// Divides a `Natural` by a `u32`, replacing the `Natural` by the remainder. For `Natural`s,
    /// rem is equivalent to mod.
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
    /// use malachite_base::num::ModAssign;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 3 * 123 + 87 = 456
    ///     let mut x = Natural::from(456u32);
    ///     x.mod_assign(123);
    ///     assert_eq!(x.to_string(), "87");
    ///
    ///     // 8,130,081,300 * 123 + 100 = 10^12
    ///     let mut x = Natural::trillion();
    ///     x.mod_assign(123);
    ///     assert_eq!(x.to_string(), "100");
    /// }
    /// ```
    fn mod_assign(&mut self, other: u32) {
        *self %= other;
    }
}

impl Mod<Natural> for u32 {
    type Output = u32;

    /// Divides a `u32` by a `Natural`, taking the `Natural` by value and returning the remainder.
    /// For `Natural`s, rem is equivalent to mod.
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
    /// use malachite_base::num::Mod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 3 * 123 + 87 = 456
    ///     assert_eq!(456.mod_op(Natural::from(123u32)), 87);
    ///
    ///     // 0 * 10^12 + 123 = 123
    ///     assert_eq!(123.mod_op(Natural::trillion()), 123);
    /// }
    /// ```
    fn mod_op(self, other: Natural) -> u32 {
        self % other
    }
}

impl<'a> Mod<&'a Natural> for u32 {
    type Output = u32;

    /// Divides a `u32` by a `Natural`, taking the `Natural` by reference and returning the
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
    /// use malachite_base::num::Mod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 3 * 123 + 87 = 456
    ///     assert_eq!(456.mod_op(&Natural::from(123u32)), 87);
    ///
    ///     // 0 * 10^12 + 123 = 123
    ///     assert_eq!(123.mod_op(&Natural::trillion()), 123);
    /// }
    /// ```
    fn mod_op(self, other: &'a Natural) -> u32 {
        self % other
    }
}

impl ModAssign<Natural> for u32 {
    /// Divides a `u32` by a `Natural` in place, taking the `Natural` by value and replacing the
    /// `u32` with the remainder. For `Natural`s, rem is equivalent to mod.
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
    /// use malachite_base::num::ModAssign;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 3 * 123 + 87 = 456
    ///     let mut n = 456;
    ///     n.mod_assign(Natural::from(123u32));
    ///     assert_eq!(n, 87);
    ///
    ///     // 0 * 10^12 + 123 = 123
    ///     let mut n = 123;
    ///     n.mod_assign(Natural::trillion());
    ///     assert_eq!(n, 123);
    /// }
    /// ```
    fn mod_assign(&mut self, other: Natural) {
        *self %= other;
    }
}

impl<'a> ModAssign<&'a Natural> for u32 {
    /// Divides a `u32` by a `Natural` in place taking the `Natural` by reference and replacing the
    /// `u32` with the remainder. For `Natural`s, rem is equivalent to mod.
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
    /// use malachite_base::num::ModAssign;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 3 * 123 + 87 = 456
    ///     let mut n = 456;
    ///     n.mod_assign(&Natural::from(123u32));
    ///     assert_eq!(n, 87);
    ///
    ///     // 0 * 10^12 + 123 = 123
    ///     let mut n = 123;
    ///     n.mod_assign(&Natural::trillion());
    ///     assert_eq!(n, 123);
    /// }
    /// ```
    fn mod_assign(&mut self, other: &'a Natural) {
        *self %= other;
    }
}

impl NegMod<u32> for Natural {
    type Output = u32;

    /// Divides the negative of a `Natural` by a `u32`, taking the `Natural` by value and returning
    /// the remainder. In other words, returns r, where `self` = q * `other` - r and
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
    /// use malachite_base::num::NegMod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 4 * 123 - 36 = 456
    ///     assert_eq!(Natural::from(456u32).neg_mod(123), 36);
    ///
    ///     // 8,130,081,301 * 123 - 23 = 10^12
    ///     assert_eq!(Natural::trillion().neg_mod(123), 23);
    /// }
    /// ```
    fn neg_mod(self, other: u32) -> u32 {
        (&self).neg_mod(other)
    }
}

impl<'a> NegMod<u32> for &'a Natural {
    type Output = u32;

    /// Divides the negative of a `Natural` by a `u32`, taking the `Natural` by reference and
    /// returning the remainder. In other words, returns r, where `self` = q * `other` - r and
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
    /// use malachite_base::num::NegMod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 3 * 124 - 36 = 456
    ///     assert_eq!((&Natural::from(456u32)).neg_mod(123), 36);
    ///
    ///     // 8,130,081,301 * 123 - 23 = 10^12
    ///     assert_eq!((&Natural::trillion()).neg_mod(123), 23);
    /// }
    /// ```
    fn neg_mod(self, other: u32) -> u32 {
        let rem = self % other;
        if rem == 0 {
            0
        } else {
            other - rem
        }
    }
}

impl NegModAssign<u32> for Natural {
    /// Divides the negative of a `Natural` by a `u32`, replacing the `Natural` by the remainder.
    /// In other words, replaces `self` with r, where `self` = q * `other` - r and 0 <= r < `other`.
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
    /// use malachite_base::num::NegModAssign;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 4 * 123 - 36 = 456
    ///     let mut x = Natural::from(456u32);
    ///     x.neg_mod_assign(123);
    ///     assert_eq!(x.to_string(), "36");
    ///
    ///     // 8,130,081,301 * 123 - 23 = 10^12
    ///     let mut x = Natural::trillion();
    ///     x.neg_mod_assign(123);
    ///     assert_eq!(x.to_string(), "23");
    /// }
    /// ```
    fn neg_mod_assign(&mut self, other: u32) {
        *self = Small((&*self).neg_mod(other));
    }
}

impl NegMod<Natural> for u32 {
    type Output = Natural;

    /// Divides the negative of a `u32` by a `Natural`, taking the `Natural` by value and returning
    /// the remainder. In other words, returns r, where `self` = q * `other` - r and
    /// 0 <= r < `other`.
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
    /// use malachite_base::num::NegMod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 4 * 123 - 36 = 456
    ///     assert_eq!(456.neg_mod(Natural::from(123u32)).to_string(), "36");
    ///
    ///     // 0 * 10^12 + 123 = 123
    ///     assert_eq!(123.neg_mod(Natural::trillion()).to_string(), "999999999877");
    /// }
    /// ```
    fn neg_mod(self, other: Natural) -> Natural {
        let rem = self % &other;
        if rem == 0 {
            Natural::ZERO
        } else {
            other - rem
        }
    }
}

impl<'a> NegMod<&'a Natural> for u32 {
    type Output = Natural;

    /// Divides the negative of a `u32` by a `Natural`, taking the `Natural` by reference and
    /// returning the remainder. In other words, returns r, where `self` = q * `other` - r and
    /// 0 <= r < `other`.
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
    /// use malachite_base::num::NegMod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 4 * 123 - 36 = 456
    ///     assert_eq!(456.neg_mod(&Natural::from(123u32)).to_string(), "36");
    ///
    ///     // 0 * 10^12 + 123 = 123
    ///     assert_eq!(123.neg_mod(&Natural::trillion()).to_string(), "999999999877");
    /// }
    /// ```
    fn neg_mod(self, other: &'a Natural) -> Natural {
        let rem = self % other;
        if rem == 0 {
            Natural::ZERO
        } else {
            other - rem
        }
    }
}

fn _limbs_rem_naive(limbs: &[u32], limb: u32) -> u32 {
    let limb = u64::from(limb);
    let mut remainder = 0;
    for &x in limbs.iter().rev() {
        remainder = (u64::join_halves(remainder, x) % limb).lower_half();
    }
    remainder
}

impl Natural {
    pub fn _mod_u32_naive(&self, other: u32) -> u32 {
        if other == 0 {
            panic!("division by zero")
        } else if other == 1 {
            0
        } else {
            match *self {
                Small(small) => small % other,
                Large(ref limbs) => _limbs_rem_naive(limbs, other),
            }
        }
    }
}
