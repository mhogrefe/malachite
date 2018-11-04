use malachite_base::num::{DivisibleByPowerOfTwo, EqMod, Parity, SplitInHalf, WrappingAddAssign};
use natural::arithmetic::div_exact_u32::limbs_invert_limb;
use natural::arithmetic::mod_u32::limbs_mod_limb;
use natural::Natural::{self, Large, Small};

// These functions are adapted from mpz_congruent_ui_p and mpn_modexact_1c_odd in GMP 6.1.2.

// must be >= 1
const BMOD_1_TO_MOD_1_THRESHOLD: usize = 10;

// Benchmarks show that this is never faster than just calling `limbs_eq_limb_mod_limb`.
//
// limbs.len() must be greater than 1; modulus must be nonzero.
pub fn _combined_limbs_eq_limb_mod_limb(limbs: &[u32], limb: u32, modulus: u32) -> bool {
    if limbs.len() < BMOD_1_TO_MOD_1_THRESHOLD {
        limbs_mod_limb(limbs, modulus) == limb % modulus
    } else {
        limbs_eq_limb_mod_limb(limbs, limb, modulus)
    }
}

// limbs.len() must be greater than 1; divisor must be odd.
pub(crate) fn limbs_mod_exact_odd_limb(limbs: &[u32], divisor: u32, mut carry: u32) -> u32 {
    let len = limbs.len();
    let inverse = limbs_invert_limb(divisor);
    let divisor_u64 = u64::from(divisor);
    let last_index = len - 1;
    for &limb in &limbs[..last_index] {
        let (mut difference, small_carry) = limb.overflowing_sub(carry);
        carry = (u64::from(difference.wrapping_mul(inverse)) * divisor_u64).upper_half();
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
        carry = (u64::from(difference.wrapping_mul(inverse)) * divisor_u64).upper_half();
        if small_carry {
            carry.wrapping_add_assign(1);
        }
        carry
    }
}

/// Interpreting a slice of `u32`s as the limbs of a `Natural` in ascending order, determines
/// whether that `Natural` is equal to a limb mod a given `u32` modulus.
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
/// Panics if the length of `limbs` is less than 2.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::eq_u32_mod_u32::limbs_eq_limb_mod_limb;
///
/// assert_eq!(limbs_eq_limb_mod_limb(&[6, 7], 3, 2), false);
/// assert_eq!(limbs_eq_limb_mod_limb(&[100, 101, 102], 1_238, 10), true);
/// ```
pub fn limbs_eq_limb_mod_limb(limbs: &[u32], limb: u32, modulus: u32) -> bool {
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

impl<'a> EqMod<u32, u32> for &'a Natural {
    /// Returns whether this `Natural` is equivalent to a `u32` mod a `u32` `modulus`; that is,
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
    /// use malachite_base::num::EqMod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(Natural::from(13u32).eq_mod(21, 8), true);
    ///     assert_eq!(Natural::from_str("12345678987654321").unwrap().eq_mod(321, 1_000), true);
    ///     assert_eq!(Natural::from_str("12345678987654321").unwrap().eq_mod(322, 1_000), false);
    /// }
    /// ```
    fn eq_mod(self, other: u32, modulus: u32) -> bool {
        match *self {
            Small(small) => small.eq_mod(other, modulus),
            Large(_) if modulus == 0 => false,
            Large(ref limbs) => limbs_eq_limb_mod_limb(limbs, other, modulus),
        }
    }
}

impl<'a> EqMod<&'a Natural, u32> for u32 {
    /// Returns whether this `u32` is equivalent to a `Natural` mod a `u32` `modulus`; that is,
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
    /// use malachite_base::num::EqMod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(21.eq_mod(&Natural::from(13u32), 8), true);
    ///     assert_eq!(321.eq_mod(&Natural::from_str("12345678987654321").unwrap(), 1_000), true);
    ///     assert_eq!(322.eq_mod(&Natural::from_str("12345678987654321").unwrap(), 1_000), false);
    /// }
    /// ```
    fn eq_mod(self, other: &'a Natural, modulus: u32) -> bool {
        other.eq_mod(self, modulus)
    }
}
