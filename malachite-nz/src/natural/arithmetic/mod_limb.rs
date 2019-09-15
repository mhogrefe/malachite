use std::ops::{Rem, RemAssign};

use malachite_base::num::arithmetic::traits::{
    Mod, ModAssign, NegMod, NegModAssign, WrappingAddAssign, WrappingSubAssign,
};
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::traits::Zero;
#[cfg(not(feature = "32_bit_limbs"))]
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::conversion::traits::{JoinHalves, SplitInHalf};

use natural::arithmetic::div_mod_limb::limbs_invert_limb;
use natural::Natural::{self, Large, Small};
use platform::{DoubleLimb, Limb};

/// Time: O(1)
///
/// Additional memory: O(1)
///
/// This is udiv_qrnnd_preinv from gmp-impl.h, but not computing the quotient.
pub(crate) fn mod_by_preinversion(
    n_high: Limb,
    n_low: Limb,
    divisor: Limb,
    divisor_inverse: Limb,
) -> Limb {
    let (quotient_high, quotient_low) = (DoubleLimb::from(n_high)
        * DoubleLimb::from(divisor_inverse))
    .wrapping_add(DoubleLimb::join_halves(n_high.wrapping_add(1), n_low))
    .split_in_half();
    let mut remainder = n_low.wrapping_sub(quotient_high.wrapping_mul(divisor));
    if remainder > quotient_low {
        remainder.wrapping_add_assign(divisor);
    }
    if remainder >= divisor {
        remainder -= divisor;
    }
    remainder
}

/// The high bit of `divisor` must be set.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// This is mpn_div_qr_1n_pi1 from mpn/generic/div_qr_1n_pi1.c with DIV_QR_1N_METHOD == 2, but not
/// computing the quotient.
fn limbs_mod_limb_normalized(
    limbs: &[Limb],
    high_limb: Limb,
    divisor: Limb,
    divisor_inverse: Limb,
) -> Limb {
    let len = limbs.len();
    if len == 1 {
        return mod_by_preinversion(high_limb, limbs[0], divisor, divisor_inverse);
    }
    let power_of_two = divisor.wrapping_neg().wrapping_mul(divisor_inverse);
    let (sum, mut big_carry) = DoubleLimb::join_halves(limbs[len - 1], limbs[len - 2])
        .overflowing_add(DoubleLimb::from(power_of_two) * DoubleLimb::from(high_limb));
    let (mut sum_high, mut sum_low) = sum.split_in_half();
    for &limb in limbs[..len - 2].iter().rev() {
        if big_carry {
            let (sum, carry) = sum_low.overflowing_add(power_of_two);
            sum_low = sum;
            if carry {
                sum_low.wrapping_sub_assign(divisor);
            }
        }
        let (sum, carry) = DoubleLimb::join_halves(sum_low, limb)
            .overflowing_add(DoubleLimb::from(sum_high) * DoubleLimb::from(power_of_two));
        sum_high = sum.upper_half();
        sum_low = sum.lower_half();
        big_carry = carry;
    }
    if big_carry {
        sum_high.wrapping_sub_assign(divisor);
    }
    if sum_high >= divisor {
        sum_high.wrapping_sub_assign(divisor);
    }
    mod_by_preinversion(sum_high, sum_low, divisor, divisor_inverse)
}

/// The high bit of `divisor` must be set.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// This is mpn_div_qr_1n_pi1 from mpn/generic/div_qr_1n_pi1.c with DIV_QR_1N_METHOD == 2, but not
/// computing the quotient, and where the input is left-shifted by `bits`.
fn limbs_mod_limb_normalized_shl(
    limbs: &[Limb],
    high_limb: Limb,
    divisor: Limb,
    divisor_inverse: Limb,
    bits: u32,
) -> Limb {
    let len = limbs.len();
    if len == 1 {
        return mod_by_preinversion(high_limb, limbs[0] << bits, divisor, divisor_inverse);
    }
    let power_of_two = divisor.wrapping_neg().wrapping_mul(divisor_inverse);
    let cobits = Limb::WIDTH - bits;
    let second_highest_limb = limbs[len - 2];
    let highest_limb_after_shl = (limbs[len - 1] << bits) | (second_highest_limb >> cobits);
    let mut second_highest_limb_after_shl = second_highest_limb << bits;
    if len > 2 {
        second_highest_limb_after_shl |= limbs[len - 3] >> cobits;
    }
    let (sum, mut big_carry) =
        DoubleLimb::join_halves(highest_limb_after_shl, second_highest_limb_after_shl)
            .overflowing_add(DoubleLimb::from(power_of_two) * DoubleLimb::from(high_limb));
    let (mut sum_high, mut sum_low) = sum.split_in_half();
    for j in (0..len - 2).rev() {
        if big_carry {
            let (sum, carry) = sum_low.overflowing_add(power_of_two);
            sum_low = sum;
            if carry {
                sum_low.wrapping_sub_assign(divisor);
            }
        }
        let mut limb = limbs[j] << bits;
        if j != 0 {
            limb |= limbs[j - 1] >> cobits;
        }
        let (sum, carry) = DoubleLimb::join_halves(sum_low, limb)
            .overflowing_add(DoubleLimb::from(sum_high) * DoubleLimb::from(power_of_two));
        sum_high = sum.upper_half();
        sum_low = sum.lower_half();
        big_carry = carry;
    }
    if big_carry {
        sum_high.wrapping_sub_assign(divisor);
    }
    if sum_high >= divisor {
        sum_high.wrapping_sub_assign(divisor);
    }
    mod_by_preinversion(sum_high, sum_low, divisor, divisor_inverse)
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
/// remainder when the `Natural` is divided by a `Limb`.
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
/// Panics if the length of `limbs` is less than 2 or if `divisor` is zero.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::mod_limb::limbs_mod_limb;
///
/// assert_eq!(limbs_mod_limb(&[123, 456], 789), 636);
/// assert_eq!(limbs_mod_limb(&[0xffff_ffff, 0xffff_ffff], 3), 0);
/// ```
///
/// This is mpn_div_qr_1 from mpn/generic/div_qr_1.c where the quotient is not computed and the
/// remainder is returned.
pub fn limbs_mod_limb(limbs: &[Limb], divisor: Limb) -> Limb {
    assert_ne!(divisor, 0);
    let len = limbs.len();
    assert!(len > 1);
    let len_minus_1 = len - 1;
    let mut highest_limb = limbs[len_minus_1];
    let bits = divisor.leading_zeros();
    if bits == 0 {
        if highest_limb >= divisor {
            highest_limb -= divisor;
        }
        let limb_inverse = limbs_invert_limb(divisor);
        limbs_mod_limb_normalized(&limbs[..len_minus_1], highest_limb, divisor, limb_inverse)
    } else {
        let divisor = divisor << bits;
        let cobits = Limb::WIDTH - bits;
        let limb_inverse = limbs_invert_limb(divisor);
        let remainder = mod_by_preinversion(
            highest_limb >> cobits,
            (highest_limb << bits) | (limbs[len - 2] >> cobits),
            divisor,
            limb_inverse,
        );
        limbs_mod_limb_normalized_shl(
            &limbs[..len_minus_1],
            remainder,
            divisor,
            limb_inverse,
            bits,
        ) >> bits
    }
}

impl Mod<Limb> for Natural {
    type Output = Limb;

    /// Divides a `Natural` by a `Limb`, taking the `Natural` by value and returning the remainder.
    /// The quotient and remainder satisfy `self` = q * `other` + r and 0 <= r < `other`.
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
    /// use malachite_base::num::arithmetic::traits::Mod;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!(Natural::from(23u32).mod_op(10), 3);
    /// }
    /// ```
    #[inline]
    fn mod_op(self, other: Limb) -> Limb {
        self % other
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl Mod<u32> for Natural {
    type Output = u32;

    #[inline]
    fn mod_op(self, other: u32) -> u32 {
        u32::wrapping_from(self.mod_op(Limb::from(other)))
    }
}

impl<'a> Mod<Limb> for &'a Natural {
    type Output = Limb;

    /// Divides a `Natural` by a `Limb`, taking the `Natural` by reference and returning the
    /// remainder. The quotient and remainder satisfy `self` = q * `other` + r and 0 <= r < `other`.
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
    /// use malachite_base::num::arithmetic::traits::Mod;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!((&Natural::from(23u32)).mod_op(10), 3);
    /// }
    /// ```
    #[inline]
    fn mod_op(self, other: Limb) -> Limb {
        self % other
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> Mod<u32> for &'a Natural {
    type Output = u32;

    #[inline]
    fn mod_op(self, other: u32) -> u32 {
        u32::wrapping_from(self.mod_op(Limb::from(other)))
    }
}

impl ModAssign<Limb> for Natural {
    /// Divides a `Natural` by a `Limb`, replacing the `Natural` by the remainder. The quotient and
    /// remainder satisfy `self` = q * `other` + r and 0 <= r < `other`.
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
    /// use malachite_base::num::arithmetic::traits::ModAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     let mut x = Natural::from(23u32);
    ///     x.mod_assign(10);
    ///     assert_eq!(x.to_string(), "3");
    /// }
    /// ```
    #[inline]
    fn mod_assign(&mut self, other: Limb) {
        *self %= other;
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl ModAssign<u32> for Natural {
    #[inline]
    fn mod_assign(&mut self, other: u32) {
        self.mod_assign(Limb::from(other));
    }
}

impl Mod<Natural> for Limb {
    type Output = Limb;

    /// Divides a `Limb` by a `Natural`, taking the `Natural` by value and returning the remainder.
    /// The quotient and remainder satisfy `self` = q * `other` + r and 0 <= r < `other`.
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
    /// use malachite_base::num::arithmetic::traits::Mod;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!(23.mod_op(Natural::from(10u32)), 3);
    /// }
    /// ```
    #[inline]
    fn mod_op(self, other: Natural) -> Limb {
        self % other
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl Mod<Natural> for u32 {
    type Output = u32;

    #[inline]
    fn mod_op(self, other: Natural) -> u32 {
        u32::wrapping_from(Limb::from(self).mod_op(other))
    }
}

impl<'a> Mod<&'a Natural> for Limb {
    type Output = Limb;

    /// Divides a `Limb` by a `Natural`, taking the `Natural` by reference and returning the
    /// remainder. The quotient and remainder satisfy `self` = q * `other` + r and 0 <= r < `other`.
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
    /// use malachite_base::num::arithmetic::traits::Mod;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!(23.mod_op(&Natural::from(10u32)), 3);
    /// }
    /// ```
    #[inline]
    fn mod_op(self, other: &'a Natural) -> Limb {
        self % other
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> Mod<&'a Natural> for u32 {
    type Output = u32;

    #[inline]
    fn mod_op(self, other: &'a Natural) -> u32 {
        u32::wrapping_from(Limb::from(self).mod_op(other))
    }
}

impl ModAssign<Natural> for Limb {
    /// Divides a `Limb` by a `Natural` in place, taking the `Natural` by value and replacing the
    /// `Limb` with the remainder. The quotient and remainder satisfy `self` = q * `other` + r and
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
    /// use malachite_base::num::arithmetic::traits::ModAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     let mut n = 23;
    ///     n.mod_assign(Natural::from(10u32));
    ///     assert_eq!(n, 3);
    /// }
    /// ```
    #[inline]
    fn mod_assign(&mut self, other: Natural) {
        *self %= other;
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl ModAssign<Natural> for u32 {
    #[inline]
    fn mod_assign(&mut self, other: Natural) {
        *self = self.mod_op(other);
    }
}

impl<'a> ModAssign<&'a Natural> for Limb {
    /// Divides a `Limb` by a `Natural` in place taking the `Natural` by reference and replacing the
    /// `Limb` with the remainder. The quotient and remainder satisfy `self` = q * `other` + r and
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
    /// use malachite_base::num::arithmetic::traits::ModAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     let mut n = 23;
    ///     n.mod_assign(&Natural::from(10u32));
    ///     assert_eq!(n, 3);
    /// }
    /// ```
    #[inline]
    fn mod_assign(&mut self, other: &'a Natural) {
        *self %= other;
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> ModAssign<&'a Natural> for u32 {
    #[inline]
    fn mod_assign(&mut self, other: &'a Natural) {
        *self = self.mod_op(other);
    }
}

impl Rem<Limb> for Natural {
    type Output = Limb;

    /// Divides a `Natural` by a `Limb`, taking the `Natural` by value and returning the remainder.
    /// The quotient and remainder satisfy `self` = q * `other` + r and 0 <= r < `other`. For
    /// `Natural`s, rem is equivalent to mod.
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
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!(Natural::from(23u32) % 10, 3);
    /// }
    /// ```
    #[inline]
    fn rem(self, other: Limb) -> Limb {
        &self % other
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl Rem<u32> for Natural {
    type Output = u32;

    #[inline]
    fn rem(self, other: u32) -> u32 {
        u32::wrapping_from(self % Limb::from(other))
    }
}

impl<'a> Rem<Limb> for &'a Natural {
    type Output = Limb;

    /// Divides a `Natural` by a `Limb`, taking the `Natural` by reference and returning the
    /// remainder. The quotient and remainder satisfy `self` = q * `other` + r and 0 <= r < `other`.
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
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!(&Natural::from(23u32) % 10, 3);
    /// }
    /// ```
    fn rem(self, other: Limb) -> Limb {
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

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> Rem<u32> for &'a Natural {
    type Output = u32;

    #[inline]
    fn rem(self, other: u32) -> u32 {
        u32::wrapping_from(self % Limb::from(other))
    }
}

impl RemAssign<Limb> for Natural {
    /// Divides a `Natural` by a `Limb`, replacing the `Natural` by the remainder. The quotient and
    /// remainder satisfy `self` = q * `other` + r and 0 <= r < `other`. For `Natural`s, rem is
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
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     let mut x = Natural::from(23u32);
    ///     x %= 10;
    ///     assert_eq!(x.to_string(), "3");
    /// }
    /// ```
    fn rem_assign(&mut self, other: Limb) {
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

#[cfg(not(feature = "32_bit_limbs"))]
impl RemAssign<u32> for Natural {
    #[inline]
    fn rem_assign(&mut self, other: u32) {
        *self %= Limb::from(other);
    }
}

impl Rem<Natural> for Limb {
    type Output = Limb;

    /// Divides a `Limb` by a `Natural`, taking the `Natural` by value and returning the remainder.
    /// The quotient and remainder satisfy `self` = q * `other` + r and 0 <= r < `other`. For
    /// `Natural`s, rem is equivalent to mod.
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
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!(23 % Natural::from(10u32), 3);
    /// }
    /// ```
    #[inline]
    fn rem(self, other: Natural) -> Limb {
        self % &other
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl Rem<Natural> for u32 {
    type Output = u32;

    #[inline]
    fn rem(self, other: Natural) -> u32 {
        u32::wrapping_from(Limb::from(self) % other)
    }
}

impl<'a> Rem<&'a Natural> for Limb {
    type Output = Limb;

    /// Divides a `Limb` by a `Natural`, taking the `Natural` by reference and returning the
    /// remainder. The quotient and remainder satisfy `self` = q * `other` + r and 0 <= r < `other`.
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
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!(23 % &Natural::from(10u32), 3);
    /// }
    /// ```
    fn rem(self, other: &'a Natural) -> Limb {
        if *other == 0 as Limb {
            panic!("division by zero");
        } else if *other == 1 as Limb {
            0
        } else {
            match *other {
                Small(small) => self % small,
                Large(_) => self,
            }
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> Rem<&'a Natural> for u32 {
    type Output = u32;

    #[inline]
    fn rem(self, other: &'a Natural) -> u32 {
        u32::wrapping_from(Limb::from(self) % other)
    }
}

impl RemAssign<Natural> for Limb {
    /// Divides a `Limb` by a `Natural` in place, taking the `Natural` by value and replacing the
    /// `Limb` with the remainder. The quotient and remainder satisfy `self` = q * `other` + r and
    /// 0 <= r < `other`. For `Natural`s, rem is equivalent to mod.
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
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     let mut n = 23;
    ///     n %= Natural::from(10u32);
    ///     assert_eq!(n, 3);
    /// }
    /// ```
    #[inline]
    fn rem_assign(&mut self, other: Natural) {
        *self %= &other;
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl RemAssign<Natural> for u32 {
    #[inline]
    fn rem_assign(&mut self, other: Natural) {
        *self = *self % other
    }
}

impl<'a> RemAssign<&'a Natural> for Limb {
    /// Divides a `Limb` by a `Natural` in place taking the `Natural` by reference and replacing the
    /// `Limb` with the remainder. The quotient and remainder satisfy `self` = q * `other` + r and
    /// 0 <= r < `other`. For `Natural`s, rem is equivalent to mod.
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
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     let mut n = 23;
    ///     n %= &Natural::from(10u32);
    ///     assert_eq!(n, 3);
    /// }
    /// ```
    #[inline]
    fn rem_assign(&mut self, other: &'a Natural) {
        *self = *self % other;
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> RemAssign<&'a Natural> for u32 {
    #[inline]
    fn rem_assign(&mut self, other: &'a Natural) {
        *self = *self % other;
    }
}

impl NegMod<Limb> for Natural {
    type Output = Limb;

    /// Divides the negative of a `Natural` by a `Limb`, taking the `Natural` by value and returning
    /// the remainder. The quotient and remainder satisfy `self` = q * `other` - r and
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
    /// use malachite_base::num::arithmetic::traits::NegMod;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     // 4 * 10 - 7 = 23
    ///     assert_eq!(Natural::from(23u32).neg_mod(10), 7);
    /// }
    /// ```
    #[inline]
    fn neg_mod(self, other: Limb) -> Limb {
        (&self).neg_mod(other)
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl NegMod<u32> for Natural {
    type Output = u32;

    #[inline]
    fn neg_mod(self, other: u32) -> u32 {
        u32::wrapping_from(self.neg_mod(Limb::from(other)))
    }
}

impl<'a> NegMod<Limb> for &'a Natural {
    type Output = Limb;

    /// Divides the negative of a `Natural` by a `Limb`, taking the `Natural` by reference and
    /// returning the remainder. The quotient and remainder satisfy `self` = q * `other` - r and
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
    /// use malachite_base::num::arithmetic::traits::NegMod;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     // 3 * 124 - 7 = 23
    ///     assert_eq!((&Natural::from(23u32)).neg_mod(10), 7);
    /// }
    /// ```
    fn neg_mod(self, other: Limb) -> Limb {
        let rem = self % other;
        if rem == 0 {
            0
        } else {
            other - rem
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> NegMod<u32> for &'a Natural {
    type Output = u32;

    #[inline]
    fn neg_mod(self, other: u32) -> u32 {
        u32::wrapping_from(self.neg_mod(Limb::from(other)))
    }
}

impl NegModAssign<Limb> for Natural {
    /// Divides the negative of a `Natural` by a `Limb`, replacing the `Natural` by the remainder.
    /// The quotient and remainder satisfy `self` = q * `other` - r and 0 <= r < `other`.
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
    /// use malachite_base::num::arithmetic::traits::NegModAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     // 4 * 10 - 7 = 23
    ///     let mut x = Natural::from(23u32);
    ///     x.neg_mod_assign(10);
    ///     assert_eq!(x.to_string(), "7");
    /// }
    /// ```
    #[inline]
    fn neg_mod_assign(&mut self, other: Limb) {
        *self = Small((&*self).neg_mod(other));
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl NegModAssign<u32> for Natural {
    #[inline]
    fn neg_mod_assign(&mut self, other: u32) {
        self.neg_mod_assign(Limb::from(other))
    }
}

impl NegMod<Natural> for Limb {
    type Output = Natural;

    /// Divides the negative of a `Limb` by a `Natural`, taking the `Natural` by value and returning
    /// the remainder. The quotient and remainder satisfy `self` = q * `other` - r and
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
    /// use malachite_base::num::arithmetic::traits::NegMod;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     // 4 * 10 - 7 = 23
    ///     assert_eq!(23.neg_mod(Natural::from(10u32)).to_string(), "7");
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

#[cfg(not(feature = "32_bit_limbs"))]
impl NegMod<Natural> for u32 {
    type Output = Natural;

    #[inline]
    fn neg_mod(self, other: Natural) -> Natural {
        Limb::from(self).neg_mod(other)
    }
}

impl<'a> NegMod<&'a Natural> for Limb {
    type Output = Natural;

    /// Divides the negative of a `Limb` by a `Natural`, taking the `Natural` by reference and
    /// returning the remainder. The quotient and remainder satisfy `self` = q * `other` - r and
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
    /// use malachite_base::num::arithmetic::traits::NegMod;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     // 4 * 10 - 7 = 23
    ///     assert_eq!(23.neg_mod(&Natural::from(10u32)).to_string(), "7");
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

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> NegMod<&'a Natural> for u32 {
    type Output = Natural;

    #[inline]
    fn neg_mod(self, other: &'a Natural) -> Natural {
        Limb::from(self).neg_mod(other)
    }
}

fn _limbs_rem_naive(limbs: &[Limb], limb: Limb) -> Limb {
    let limb = DoubleLimb::from(limb);
    let mut remainder = 0;
    for &x in limbs.iter().rev() {
        remainder = (DoubleLimb::join_halves(remainder, x) % limb).lower_half();
    }
    remainder
}

impl Natural {
    pub fn _mod_limb_naive(&self, other: Limb) -> Limb {
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
