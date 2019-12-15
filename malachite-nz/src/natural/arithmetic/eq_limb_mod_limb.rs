use malachite_base::num::arithmetic::traits::{
    DivisibleByPowerOfTwo, EqMod, Parity, WrappingAddAssign,
};
use malachite_base::num::conversion::traits::SplitInHalf;

use natural::arithmetic::div_exact_limb::limbs_modular_invert_limb;
use natural::arithmetic::mod_op::limbs_mod_limb;
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::{DoubleLimb, Limb, BMOD_1_TO_MOD_1_THRESHOLD};

/// divisor must be odd. //TODO test
///
/// This is mpn_modexact_1c_odd from mpn/generic/mode1o.c.
pub(crate) fn limbs_mod_exact_odd_limb(limbs: &[Limb], divisor: Limb, mut carry: Limb) -> Limb {
    let len = limbs.len();
    if len == 1 {
        let limb = limbs[0];
        return if limb > carry {
            let result = (limb - carry) % divisor;
            if result == 0 {
                0
            } else {
                divisor - result
            }
        } else {
            (carry - limb) % divisor
        };
    }
    let inverse = limbs_modular_invert_limb(divisor);
    let divisor_double = DoubleLimb::from(divisor);
    let last_index = len - 1;
    for &limb in &limbs[..last_index] {
        let (difference, small_carry) = limb.overflowing_sub(carry);
        carry = (DoubleLimb::from(difference.wrapping_mul(inverse)) * divisor_double).upper_half();
        if small_carry {
            carry.wrapping_add_assign(1);
        }
    }
    let last = limbs[last_index];
    if last <= divisor {
        if carry >= last {
            carry - last
        } else {
            carry.wrapping_add(divisor - last)
        }
    } else {
        let (difference, small_carry) = last.overflowing_sub(carry);
        carry = (DoubleLimb::from(difference.wrapping_mul(inverse)) * divisor_double).upper_half();
        if small_carry {
            carry.wrapping_add_assign(1);
        }
        carry
    }
}

/// Benchmarks show that this is never faster than just calling `limbs_eq_limb_mod_limb`.
///
/// limbs.len() must be greater than 1; modulus must be nonzero.
///
/// This is mpz_congruent_ui_p from mpz/cong_ui.c where a is non-negative.
pub fn _combined_limbs_eq_limb_mod_limb(limbs: &[Limb], limb: Limb, modulus: Limb) -> bool {
    if limbs.len() < BMOD_1_TO_MOD_1_THRESHOLD {
        limbs_mod_limb(limbs, modulus) == limb % modulus
    } else {
        limbs_eq_limb_mod_limb(limbs, limb, modulus)
    }
}

/// Interpreting a slice of `Limb`s as the limbs of a `Natural` in ascending order, determines
/// whether that `Natural` is equal to a limb mod a given `Limb` modulus.
///
/// This function assumes that `modulus` is nonzero, `limbs` has at least two elements, and the last
/// element of `limbs` is nonzero.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if the length of `limbs` is less than 2 or `modulus` is zero.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::eq_limb_mod_limb::limbs_eq_limb_mod_limb;
///
/// assert_eq!(limbs_eq_limb_mod_limb(&[6, 7], 3, 2), false);
/// assert_eq!(limbs_eq_limb_mod_limb(&[100, 101, 102], 1_238, 10), true);
/// ```
///
/// This is mpz_congruent_ui_p from mpz/cong_ui.c where a is positive and the ABOVE_THRESHOLD branch
/// is excluded.
pub fn limbs_eq_limb_mod_limb(limbs: &[Limb], limb: Limb, modulus: Limb) -> bool {
    assert_ne!(modulus, 0);
    assert!(limbs.len() > 1);
    let remainder = if modulus.even() {
        let twos = modulus.trailing_zeros();
        if !limbs[0]
            .wrapping_sub(limb)
            .divisible_by_power_of_two(twos.into())
        {
            return false;
        }
        limbs_mod_exact_odd_limb(limbs, modulus >> twos, limb)
    } else {
        limbs_mod_exact_odd_limb(limbs, modulus, limb)
    };
    remainder == 0 || remainder == modulus
}

impl<'a> EqMod<Limb, Limb> for &'a Natural {
    /// Returns whether this `Natural` is equivalent to a `Limb` mod a `Limb` `modulus`; that is,
    /// whether `self` - other is a multiple of `modulus`. Two numbers are equal to each other mod 0
    /// iff they are equal.
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
    /// use malachite_base::num::arithmetic::traits::EqMod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!((&Natural::from(13u32)).eq_mod(21, 8), true);
    ///     assert_eq!(
    ///         (&Natural::from_str("12345678987654321").unwrap()).eq_mod(321, 1_000),
    ///         true
    ///     );
    ///     assert_eq!(
    ///         (&Natural::from_str("12345678987654321").unwrap()).eq_mod(322, 1_000),
    ///         false
    ///     );
    /// }
    /// ```
    fn eq_mod(self, other: Limb, modulus: Limb) -> bool {
        match *self {
            Natural(Small(small)) => small.eq_mod(other, modulus),
            Natural(Large(_)) if modulus == 0 => false,
            Natural(Large(ref limbs)) => limbs_eq_limb_mod_limb(limbs, other, modulus),
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> EqMod<u32, u32> for &'a Natural {
    #[inline]
    fn eq_mod(self, other: u32, modulus: u32) -> bool {
        self.eq_mod(Limb::from(other), Limb::from(modulus))
    }
}

impl<'a> EqMod<&'a Natural, Limb> for Limb {
    /// Returns whether this `Limb` is equivalent to a `Natural` mod a `Limb` `modulus`; that is,
    /// whether other - `self` is a multiple of `modulus`. Two numbers are equal to each other mod 0
    /// iff they are equal.
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
    /// use malachite_base::num::arithmetic::traits::EqMod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(21.eq_mod(&Natural::from(13u32), 8), true);
    ///     assert_eq!(321.eq_mod(&Natural::from_str("12345678987654321").unwrap(), 1_000), true);
    ///     assert_eq!(322.eq_mod(&Natural::from_str("12345678987654321").unwrap(), 1_000), false);
    /// }
    /// ```
    #[inline]
    fn eq_mod(self, other: &'a Natural, modulus: Limb) -> bool {
        other.eq_mod(self, modulus)
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> EqMod<&'a Natural, u32> for u32 {
    #[inline]
    fn eq_mod(self, other: &'a Natural, modulus: u32) -> bool {
        Limb::from(self).eq_mod(other, Limb::from(modulus))
    }
}

impl<'a> EqMod<Limb, &'a Natural> for Limb {
    /// Returns whether this `Limb` is equivalent to a `Limb` mod a `Natural` `modulus`; that is,
    /// whether other - `self` is a multiple of `modulus`. Two numbers are equal to each other mod 0
    /// iff they are equal.
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
    /// use malachite_base::num::arithmetic::traits::EqMod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(21.eq_mod(8, &Natural::from(13u32)), true);
    ///     assert_eq!(21.eq_mod(21, &Natural::from_str("12345678987654321").unwrap()), true);
    ///     assert_eq!(21.eq_mod(22, &Natural::from_str("12345678987654321").unwrap()), false);
    /// }
    /// ```
    fn eq_mod(self, other: Limb, modulus: &'a Natural) -> bool {
        match *modulus {
            Natural(Small(small)) => self.eq_mod(other, small),
            Natural(Large(_)) => self == other,
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> EqMod<u32, &'a Natural> for u32 {
    #[inline]
    fn eq_mod(self, other: u32, modulus: &'a Natural) -> bool {
        Limb::from(self).eq_mod(Limb::from(other), modulus)
    }
}
