use std::ops::{Rem, RemAssign};

use malachite_base::comparison::Max;
use malachite_base::num::arithmetic::traits::{
    Mod, ModAssign, ModPowerOfTwo, NegMod, NegModAssign, Parity, WrappingAddAssign,
    WrappingMulAssign, WrappingSubAssign,
};
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::traits::Zero;
#[cfg(not(feature = "32_bit_limbs"))]
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::conversion::traits::{JoinHalves, SplitInHalf};

use natural::arithmetic::div_mod_limb::limbs_invert_limb;
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::{
    DoubleLimb, Limb, MOD_1N_TO_MOD_1_1_THRESHOLD, MOD_1U_TO_MOD_1_1_THRESHOLD, MOD_1_1P_METHOD,
    MOD_1_1_TO_MOD_1_2_THRESHOLD, MOD_1_2_TO_MOD_1_4_THRESHOLD, MOD_1_NORM_THRESHOLD,
    MOD_1_UNNORM_THRESHOLD,
};

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

/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// This is mpn_divrem_1 from mpn/generic/divrem_1.c, where qxn is 0 and un > 1, but not computing
/// the quotient.
pub fn _limbs_mod_limb_alt_3(limbs: &[Limb], divisor: Limb) -> Limb {
    assert_ne!(divisor, 0);
    let len = limbs.len();
    assert!(len > 1);
    let bits = divisor.leading_zeros();
    let (limbs_last, limbs_init) = limbs.split_last().unwrap();
    if bits == 0 {
        // High quotient limb is 0 or 1, skip a divide step.
        let mut remainder = *limbs_last;
        if remainder >= divisor {
            remainder -= divisor;
        }
        // Multiply-by-inverse, divisor already normalized.
        let inverse = limbs_invert_limb(divisor);
        for limb in limbs_init.iter().rev() {
            remainder = mod_by_preinversion(remainder, *limb, divisor, inverse);
        }
        remainder
    } else {
        // Skip a division if high < divisor (high quotient 0). Testing here before normalizing will
        // still skip as often as possible.
        let (limbs, mut remainder) = if *limbs_last < divisor {
            (limbs_init, *limbs_last)
        } else {
            (limbs, 0)
        };
        let divisor = divisor << bits;
        remainder <<= bits;
        let inverse = limbs_invert_limb(divisor);
        let (limbs_last, limbs_init) = limbs.split_last().unwrap();
        let mut previous_limb = *limbs_last;
        let cobits = Limb::WIDTH - bits;
        remainder |= previous_limb >> cobits;
        for &limb in limbs_init.iter().rev() {
            let shifted_limb = (previous_limb << bits) | (limb >> cobits);
            remainder = mod_by_preinversion(remainder, shifted_limb, divisor, inverse);
            previous_limb = limb;
        }
        mod_by_preinversion(remainder, previous_limb << bits, divisor, inverse) >> bits
    }
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
#[cfg(feature = "32_bit_limbs")]
#[inline]
pub fn limbs_mod_limb(limbs: &[Limb], divisor: Limb) -> Limb {
    _limbs_mod_limb_alt_2(limbs, divisor)
}
#[cfg(not(feature = "32_bit_limbs"))]
#[inline]
pub fn limbs_mod_limb(limbs: &[Limb], divisor: Limb) -> Limb {
    _limbs_mod_limb_alt_1(limbs, divisor)
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
    /// # Panics
    /// Panics if `other` is zero.
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
    /// # Panics
    /// Panics if `other` is zero.
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
    /// # Panics
    /// Panics if `other` is zero.
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
    /// # Panics
    /// Panics if `other` is zero.
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
    /// # Panics
    /// Panics if `other` is zero.
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
    /// # Panics
    /// Panics if `other` is zero.
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
    /// # Panics
    /// Panics if `other` is zero.
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
    /// # Panics
    /// Panics if `other` is zero.
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
                Natural(Small(small)) => small % other,
                Natural(Large(ref limbs)) => limbs_mod_limb(limbs, other),
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
    /// # Panics
    /// Panics if `other` is zero.
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
                Natural(Small(ref mut small)) => {
                    *small %= other;
                    return;
                }
                Natural(Large(ref mut limbs)) => limbs_mod_limb(limbs, other),
            };
            *self = Natural(Small(remainder));
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
    /// # Panics
    /// Panics if `other` is zero.
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
    /// # Panics
    /// Panics if `other` is zero.
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
                Natural(Small(small)) => self % small,
                Natural(Large(_)) => self,
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
    /// # Panics
    /// Panics if `other` is zero.
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
    /// # Panics
    /// Panics if `other` is zero.
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
    /// # Panics
    /// Panics if `other` is zero.
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
    /// # Panics
    /// Panics if `other` is zero.
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
    /// # Panics
    /// Panics if `other` is zero.
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
        *self = Natural(Small((&*self).neg_mod(other)));
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
    /// # Panics
    /// Panics if `other` is zero.
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
            other - Natural::from(rem)
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
    /// # Panics
    /// Panics if `other` is zero.
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
            other - Natural::from(rem)
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

fn _limbs_rem_naive(limbs: &[Limb], divisor: Limb) -> Limb {
    let limb = DoubleLimb::from(divisor);
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
                Natural(Small(small)) => small % other,
                Natural(Large(ref limbs)) => _limbs_rem_naive(limbs, other),
            }
        }
    }
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

/// This is mpn_div_qr_1 from mpn/generic/div_qr_1.c where the quotient is not computed and the
/// remainder is returned. Experiments show that this is always slower than `_limbs_mod_limb`.
pub fn _limbs_mod_limb_alt_1(limbs: &[Limb], divisor: Limb) -> Limb {
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

/// Dividing (`n_high`, `n_low`) by `divisor`, returning the remainder only. Unlike
/// `mod_by_preinversion`, works also for the case `n_high` == `divisor`, where the quotient doesn't
/// quite fit in a single limb.
///
/// Time: O(1)
///
/// Additional memory: O(1)
///
/// This is udiv_rnnd_preinv from gmp-impl.h.
fn mod_by_preinversion_special(
    n_high: Limb,
    n_low: Limb,
    divisor: Limb,
    divisor_inverse: Limb,
) -> Limb {
    let (quotient_high, quotient_low) = ((DoubleLimb::from(n_high)
        * DoubleLimb::from(divisor_inverse))
    .wrapping_add(DoubleLimb::join_halves(n_high.wrapping_add(1), n_low)))
    .split_in_half();
    let mut remainder = n_low.wrapping_sub(quotient_high.wrapping_mul(divisor));
    // both > and >= are OK
    if remainder > quotient_low {
        remainder.wrapping_add_assign(divisor);
    }
    if remainder >= divisor {
        remainder.wrapping_sub_assign(divisor);
    }
    remainder
}

pub fn _limbs_mod_limb_small_small(limbs: &[Limb], divisor: Limb, mut remainder: Limb) -> Limb {
    let divisor = DoubleLimb::from(divisor);
    for &limb in limbs.iter().rev() {
        remainder = (DoubleLimb::join_halves(remainder, limb) % divisor).lower_half();
    }
    remainder
}

pub fn _limbs_mod_limb_small_normalized_large(
    limbs: &[Limb],
    divisor: Limb,
    mut remainder: Limb,
) -> Limb {
    let inverse = limbs_invert_limb(divisor);
    for &limb in limbs.iter().rev() {
        remainder = mod_by_preinversion_special(remainder, limb, divisor, inverse);
    }
    remainder
}

/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// This is mpn_mod_1_norm from mpn/generic/mod_1.c.
#[allow(clippy::absurd_extreme_comparisons)]
pub fn _limbs_mod_limb_small_normalized(limbs: &[Limb], divisor: Limb) -> Limb {
    let mut len = limbs.len();
    assert_ne!(len, 0);
    assert!(divisor.get_highest_bit());
    // High limb is initial remainder, possibly with one subtraction of d to get r < d.
    let mut remainder = limbs[len - 1];
    if remainder >= divisor {
        remainder -= divisor;
    }
    len -= 1;
    if len == 0 {
        return remainder;
    }
    let limbs = &limbs[..len];
    if len < MOD_1_NORM_THRESHOLD {
        _limbs_mod_limb_small_small(limbs, divisor, remainder)
    } else {
        _limbs_mod_limb_small_normalized_large(limbs, divisor, remainder)
    }
}

pub fn _limbs_mod_limb_small_unnormalized_large(
    limbs: &[Limb],
    mut divisor: Limb,
    mut remainder: Limb,
) -> Limb {
    let shift = divisor.leading_zeros();
    divisor <<= shift;
    let (limbs_last, limbs_init) = limbs.split_last().unwrap();
    let mut previous_limb = *limbs_last;
    let co_shift = Limb::WIDTH - shift;
    remainder = (remainder << shift) | (previous_limb >> co_shift);
    let divisor_inverse = limbs_invert_limb(divisor);
    for &limb in limbs_init.iter().rev() {
        let shifted_limb = (previous_limb << shift) | (limb >> co_shift);
        remainder = mod_by_preinversion_special(remainder, shifted_limb, divisor, divisor_inverse);
        previous_limb = limb;
    }
    mod_by_preinversion_special(remainder, previous_limb << shift, divisor, divisor_inverse)
        >> shift
}

/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// This is mpn_mod_1_unnorm from mpn/generic/mod_1.c, where UDIV_NEEDS_NORMALIZATION is false.
#[allow(clippy::absurd_extreme_comparisons)]
pub fn _limbs_mod_limb_small_unnormalized(limbs: &[Limb], divisor: Limb) -> Limb {
    let mut len = limbs.len();
    assert_ne!(len, 0);
    assert_ne!(divisor, 0);
    assert!(!divisor.get_highest_bit());
    // Skip a division if high < divisor. Having the test here before normalizing will still skip as
    // often as possible.
    let mut remainder = limbs[len - 1];
    if remainder < divisor {
        len -= 1;
        if len == 0 {
            return remainder;
        }
    } else {
        remainder = 0;
    }
    let limbs = &limbs[..len];
    if len < MOD_1_UNNORM_THRESHOLD {
        _limbs_mod_limb_small_small(limbs, divisor, remainder)
    } else {
        _limbs_mod_limb_small_unnormalized_large(limbs, divisor, remainder)
    }
}

pub fn _limbs_mod_limb_any_leading_zeros(limbs: &[Limb], divisor: Limb) -> Limb {
    if MOD_1_1P_METHOD {
        _limbs_mod_limb_any_leading_zeros_1(limbs, divisor)
    } else {
        _limbs_mod_limb_any_leading_zeros_2(limbs, divisor)
    }
}

/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// This is mpn_mod_1_1p_cps_1 combined with mpn_mod_1_1p_1 from mpn/generic/mod_1.c.
pub fn _limbs_mod_limb_any_leading_zeros_1(limbs: &[Limb], divisor: Limb) -> Limb {
    let len = limbs.len();
    assert!(len >= 2);
    let shift = divisor.leading_zeros();
    let divisor = divisor << shift;
    let divisor_inverse = limbs_invert_limb(divisor);
    let mut base_mod_divisor = divisor.wrapping_neg();
    if shift != 0 {
        base_mod_divisor
            .wrapping_mul_assign((divisor_inverse >> (Limb::WIDTH - shift)) | (1 << shift));
    }
    assert!(base_mod_divisor <= divisor); // not fully reduced mod divisor
    let base_pow_2_mod_divisor = DoubleLimb::from(
        mod_by_preinversion_special(base_mod_divisor, 0, divisor, divisor_inverse) >> shift,
    );
    let base_mod_divisor = DoubleLimb::from(base_mod_divisor >> shift);
    let (mut r_hi, mut r_lo) = (DoubleLimb::from(limbs[len - 1]) * base_mod_divisor)
        .wrapping_add(DoubleLimb::from(limbs[len - 2]))
        .split_in_half();
    for &limb in limbs[..len - 2].iter().rev() {
        let (new_r_hi, new_r_lo) = (DoubleLimb::from(r_hi) * base_pow_2_mod_divisor)
            .wrapping_add(DoubleLimb::from(r_lo) * base_mod_divisor)
            .wrapping_add(DoubleLimb::from(limb))
            .split_in_half();
        r_hi = new_r_hi;
        r_lo = new_r_lo;
    }
    if shift != 0 {
        r_hi = (r_hi << shift) | (r_lo >> (Limb::WIDTH - shift));
    }
    if r_hi >= divisor {
        r_hi.wrapping_sub_assign(divisor);
    }
    mod_by_preinversion_special(r_hi, r_lo << shift, divisor, divisor_inverse) >> shift
}

/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// This is mpn_mod_1_1p_cps_2 combined with mpn_mod_1_1p_2 from mpn/generic/mod_1.c.
pub fn _limbs_mod_limb_any_leading_zeros_2(limbs: &[Limb], divisor: Limb) -> Limb {
    let len = limbs.len();
    assert!(len >= 2);
    let shift = divisor.leading_zeros();
    let divisor = divisor << shift;
    let divisor_inverse = limbs_invert_limb(divisor);
    let base_mod_divisor = if shift == 0 {
        0
    } else {
        let base_mod_divisor = divisor
            .wrapping_neg()
            .wrapping_mul((divisor_inverse >> (Limb::WIDTH - shift)) | (1 << shift));
        assert!(base_mod_divisor <= divisor); // not fully reduced mod divisor
        DoubleLimb::from(base_mod_divisor >> shift)
    };
    let small_base_pow_2_mod_divisor = divisor.wrapping_neg().wrapping_mul(divisor_inverse);
    // equality iff divisor = 2 ^ (Limb::WIDTH - 1)
    assert!(small_base_pow_2_mod_divisor <= divisor);
    let base_pow_2_mod_divisor = DoubleLimb::from(small_base_pow_2_mod_divisor);
    let mut r_lo = limbs[len - 2];
    let mut r_hi = limbs[len - 1];
    if len > 2 {
        let (r, mut carry) = DoubleLimb::join_halves(r_lo, limbs[len - 3])
            .overflowing_add(DoubleLimb::from(r_hi) * base_pow_2_mod_divisor);
        let (new_r_hi, new_r_lo) = r.split_in_half();
        r_hi = new_r_hi;
        r_lo = new_r_lo;
        for &limb in limbs[..len - 3].iter().rev() {
            if carry {
                let (new_r_lo, carry) = r_lo.overflowing_add(small_base_pow_2_mod_divisor);
                r_lo = new_r_lo;
                if carry {
                    r_lo.wrapping_sub_assign(divisor);
                }
            }
            let (r, new_carry) = DoubleLimb::join_halves(r_lo, limb)
                .overflowing_add(DoubleLimb::from(r_hi) * base_pow_2_mod_divisor);
            carry = new_carry;
            let (new_r_hi, new_r_lo) = r.split_in_half();
            r_hi = new_r_hi;
            r_lo = new_r_lo;
        }
        if carry {
            r_hi.wrapping_sub_assign(divisor);
        }
    }
    if shift != 0 {
        let (new_r_hi, temp) = (DoubleLimb::from(r_hi) * base_mod_divisor).split_in_half();
        let (new_r_hi, new_r_lo) =
            (DoubleLimb::join_halves(new_r_hi, r_lo).wrapping_add(DoubleLimb::from(temp)) << shift)
                .split_in_half();
        r_hi = new_r_hi;
        r_lo = new_r_lo;
    } else if r_hi >= divisor {
        // might get r_hi == divisor here, but `mod_by_preinversion_special` allows that.
        r_hi.wrapping_sub_assign(divisor);
    }
    mod_by_preinversion_special(r_hi, r_lo, divisor, divisor_inverse) >> shift
}

/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// This is mpn_mod_1s_2p_cps combined with mpn_mod_1s_2p from mpn/generic/mod_1_2.c.
pub fn _limbs_mod_limb_at_least_1_leading_zero(limbs: &[Limb], divisor: Limb) -> Limb {
    let mut len = limbs.len();
    assert_ne!(len, 0);
    let shift = divisor.leading_zeros();
    assert_ne!(shift, 0);
    let co_shift = Limb::WIDTH - shift;
    let divisor = divisor << shift;
    let divisor_inverse = limbs_invert_limb(divisor);
    let base_mod_divisor = divisor
        .wrapping_neg()
        .wrapping_mul((divisor_inverse >> co_shift) | (1 << shift));
    assert!(base_mod_divisor <= divisor); // not fully reduced mod divisor
    let base_pow_2_mod_divisor =
        mod_by_preinversion_special(base_mod_divisor, 0, divisor, divisor_inverse);
    let base_mod_divisor = DoubleLimb::from(base_mod_divisor >> shift);
    let base_pow_3_mod_divisor = DoubleLimb::from(
        mod_by_preinversion_special(base_pow_2_mod_divisor, 0, divisor, divisor_inverse) >> shift,
    );
    let base_pow_2_mod_divisor = DoubleLimb::from(base_pow_2_mod_divisor >> shift);
    let (mut r_hi, mut r_lo) = if len.odd() {
        len -= 1;
        if len == 0 {
            let rl = limbs[len];
            return mod_by_preinversion_special(
                rl >> co_shift,
                rl << shift,
                divisor,
                divisor_inverse,
            ) >> shift;
        }
        (DoubleLimb::from(limbs[len]) * base_pow_2_mod_divisor)
            .wrapping_add(DoubleLimb::from(limbs[len - 1]) * base_mod_divisor)
            .wrapping_add(DoubleLimb::from(limbs[len - 2]))
            .split_in_half()
    } else {
        (limbs[len - 1], limbs[len - 2])
    };
    for chunk in limbs[..len - 2].rchunks_exact(2) {
        let (new_r_hi, new_r_lo) = (DoubleLimb::from(r_hi) * base_pow_3_mod_divisor)
            .wrapping_add(DoubleLimb::from(r_lo) * base_pow_2_mod_divisor)
            .wrapping_add(DoubleLimb::from(chunk[1]) * base_mod_divisor)
            .wrapping_add(DoubleLimb::from(chunk[0]))
            .split_in_half();
        r_hi = new_r_hi;
        r_lo = new_r_lo;
    }
    let (r_hi, r_lo) = (DoubleLimb::from(r_hi) * base_mod_divisor)
        .wrapping_add(DoubleLimb::from(r_lo))
        .split_in_half();
    mod_by_preinversion_special(
        (r_hi << shift) | (r_lo >> co_shift),
        r_lo << shift,
        divisor,
        divisor_inverse,
    ) >> shift
}

/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// This is mpn_mod_1s_4p_cps combined with mpn_mod_1s_4p from mpn/generic/mod_1_4.c.
pub fn _limbs_mod_limb_at_least_2_leading_zeros(limbs: &[Limb], divisor: Limb) -> Limb {
    let mut len = limbs.len();
    assert_ne!(len, 0);
    let shift = divisor.leading_zeros();
    assert!(shift >= 2);
    let co_shift = Limb::WIDTH - shift;
    let divisor = divisor << shift;
    let divisor_inverse = limbs_invert_limb(divisor);
    let base_mod_divisor = divisor
        .wrapping_neg()
        .wrapping_mul((divisor_inverse >> co_shift) | (1 << shift));
    assert!(base_mod_divisor <= divisor); // not fully reduced mod divisor
    let base_pow_2_mod_divisor =
        mod_by_preinversion_special(base_mod_divisor, 0, divisor, divisor_inverse);
    let base_mod_divisor = DoubleLimb::from(base_mod_divisor >> shift);
    let base_pow_3_mod_divisor =
        mod_by_preinversion_special(base_pow_2_mod_divisor, 0, divisor, divisor_inverse);
    let base_pow_2_mod_divisor = DoubleLimb::from(base_pow_2_mod_divisor >> shift);
    let base_pow_4_mod_divisor =
        mod_by_preinversion_special(base_pow_3_mod_divisor, 0, divisor, divisor_inverse);
    let base_pow_3_mod_divisor = DoubleLimb::from(base_pow_3_mod_divisor >> shift);
    let base_pow_5_mod_divisor = DoubleLimb::from(
        mod_by_preinversion_special(base_pow_4_mod_divisor, 0, divisor, divisor_inverse) >> shift,
    );
    let base_pow_4_mod_divisor = DoubleLimb::from(base_pow_4_mod_divisor >> shift);
    let (mut r_hi, mut r_lo) = match len.mod_power_of_two(2) {
        0 => {
            len -= 4;
            (DoubleLimb::from(limbs[len + 3]) * base_pow_3_mod_divisor)
                .wrapping_add(DoubleLimb::from(limbs[len + 2]) * base_pow_2_mod_divisor)
                .wrapping_add(DoubleLimb::from(limbs[len + 1]) * base_mod_divisor)
                .wrapping_add(DoubleLimb::from(limbs[len]))
                .split_in_half()
        }
        1 => {
            len -= 1;
            (0, limbs[len])
        }
        2 => {
            len -= 2;
            (limbs[len + 1], limbs[len])
        }
        3 => {
            len -= 3;
            (DoubleLimb::from(limbs[len + 2]) * base_pow_2_mod_divisor)
                .wrapping_add(DoubleLimb::from(limbs[len + 1]) * base_mod_divisor)
                .wrapping_add(DoubleLimb::from(limbs[len]))
                .split_in_half()
        }
        _ => unreachable!(),
    };
    for chunk in limbs[..len].rchunks_exact(4) {
        let (new_r_hi, new_r_lo) = (DoubleLimb::from(r_hi) * base_pow_5_mod_divisor)
            .wrapping_add(DoubleLimb::from(r_lo) * base_pow_4_mod_divisor)
            .wrapping_add(DoubleLimb::from(chunk[3]) * base_pow_3_mod_divisor)
            .wrapping_add(DoubleLimb::from(chunk[2]) * base_pow_2_mod_divisor)
            .wrapping_add(DoubleLimb::from(chunk[1]) * base_mod_divisor)
            .wrapping_add(DoubleLimb::from(chunk[0]))
            .split_in_half();
        r_hi = new_r_hi;
        r_lo = new_r_lo;
    }
    let (r_hi, r_lo) = (DoubleLimb::from(r_hi) * base_mod_divisor)
        .wrapping_add(DoubleLimb::from(r_lo))
        .split_in_half();
    mod_by_preinversion_special(
        (r_hi << shift) | (r_lo >> co_shift),
        r_lo << shift,
        divisor,
        divisor_inverse,
    ) >> shift
}

const HIGHEST_TWO_BITS_MASK: Limb = !(Limb::MAX >> 2);

/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// This is mpn_mod_1 from mpn/generic/mod_1.c, where n > 1.
#[allow(clippy::absurd_extreme_comparisons)]
pub fn _limbs_mod_limb_alt_2(limbs: &[Limb], divisor: Limb) -> Limb {
    let len = limbs.len();
    assert!(len > 1);
    assert_ne!(divisor, 0);
    if divisor.get_highest_bit() {
        if len < MOD_1N_TO_MOD_1_1_THRESHOLD {
            _limbs_mod_limb_small_normalized(limbs, divisor)
        } else {
            _limbs_mod_limb_any_leading_zeros(limbs, divisor)
        }
    } else if len < MOD_1U_TO_MOD_1_1_THRESHOLD {
        _limbs_mod_limb_small_unnormalized(limbs, divisor)
    } else if len < MOD_1_1_TO_MOD_1_2_THRESHOLD {
        _limbs_mod_limb_any_leading_zeros(limbs, divisor)
    } else if len < MOD_1_2_TO_MOD_1_4_THRESHOLD || divisor & HIGHEST_TWO_BITS_MASK != 0 {
        _limbs_mod_limb_at_least_1_leading_zero(limbs, divisor)
    } else {
        _limbs_mod_limb_at_least_2_leading_zeros(limbs, divisor)
    }
}
