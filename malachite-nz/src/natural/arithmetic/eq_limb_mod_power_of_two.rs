use malachite_base::conversion::CheckedFrom;
use malachite_base::num::integers::PrimitiveInteger;
use malachite_base::num::traits::EqModPowerOfTwo;
use natural::arithmetic::divisible_by_power_of_two::limbs_divisible_by_power_of_two;
use natural::Natural::{self, Large, Small};
use platform::Limb;

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns
/// whether the `Natural` is equivalent to a limb mod two to the power of `pow`; that is, whether
/// the `pow` least-significant bits of the `Natural` and the limb are equal.
///
/// This function assumes that `limbs` has length at least 2 and the last (most significant) limb is
/// nonzero.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::eq_limb_mod_power_of_two::limbs_eq_limb_mod_power_of_two;
///
/// assert_eq!(limbs_eq_limb_mod_power_of_two(&[0b1111011, 0b111001000], 0b101011, 4), true);
/// assert_eq!(limbs_eq_limb_mod_power_of_two(&[0b1111011, 0b111001000], 0b101011, 5), false);
/// assert_eq!(limbs_eq_limb_mod_power_of_two(&[0b1111011, 0b111001000], 0b1111011, 35), true);
/// assert_eq!(limbs_eq_limb_mod_power_of_two(&[0b1111011, 0b111001000], 0b1111011, 36), false);
/// assert_eq!(limbs_eq_limb_mod_power_of_two(&[0b1111011, 0b111001000], 0b1111011, 100), false);
/// ```
pub fn limbs_eq_limb_mod_power_of_two(limbs: &[Limb], limb: Limb, pow: u64) -> bool {
    let i = usize::checked_from(pow >> Limb::LOG_WIDTH).unwrap();
    if i >= limbs.len() {
        false
    } else if i == 0 {
        limbs[0].eq_mod_power_of_two(limb, pow)
    } else {
        limbs[0] == limb
            && limbs_divisible_by_power_of_two(&limbs[1..], pow - u64::from(Limb::WIDTH))
    }
}

impl<'a> EqModPowerOfTwo<Limb> for &'a Natural {
    /// Returns whether this `Natural` is equivalent to a `Limb` mod two to the power of `pow`; that
    /// is, whether the `pow` least-significant bits of the `Natural` and the `Limb` are equal.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = min(`pow`, `self.significant_bits()`)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::traits::{EqModPowerOfTwo, Zero};
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(Natural::ZERO.eq_mod_power_of_two(256u32, 8), true);
    ///     assert_eq!(Natural::from(0b1101u32).eq_mod_power_of_two(0b10101, 3), true);
    ///     assert_eq!(Natural::from(0b1101u32).eq_mod_power_of_two(0b10101, 4), false);
    /// }
    /// ```
    fn eq_mod_power_of_two(self, other: Limb, pow: u64) -> bool {
        match *self {
            Small(small) => small.eq_mod_power_of_two(other, pow),
            Large(ref limbs) => limbs_eq_limb_mod_power_of_two(limbs, other, pow),
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> EqModPowerOfTwo<u32> for &'a Natural {
    #[inline]
    fn eq_mod_power_of_two(self, other: u32, pow: u64) -> bool {
        self.eq_mod_power_of_two(Limb::from(other), pow)
    }
}

impl<'a> EqModPowerOfTwo<&'a Natural> for Limb {
    /// Returns whether this `Limb` is equivalent to a `Natural` mod two to the power of `pow`; that
    /// is, whether the `pow` least-significant bits of the `Limb` and the `Natural` are equal.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = min(`pow`, `self.significant_bits()`)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::traits::{EqModPowerOfTwo, Zero};
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(256u32.eq_mod_power_of_two(&Natural::ZERO, 8), true);
    ///     assert_eq!(0b10101.eq_mod_power_of_two(&Natural::from(0b1101u32), 3), true);
    ///     assert_eq!(0b10101.eq_mod_power_of_two(&Natural::from(0b1101u32), 4), false);
    /// }
    /// ```
    fn eq_mod_power_of_two(self, other: &'a Natural, pow: u64) -> bool {
        match *other {
            Small(small) => self.eq_mod_power_of_two(small, pow),
            Large(ref limbs) => limbs_eq_limb_mod_power_of_two(limbs, self, pow),
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> EqModPowerOfTwo<&'a Natural> for u32 {
    #[inline]
    fn eq_mod_power_of_two(self, other: &'a Natural, pow: u64) -> bool {
        Limb::from(self).eq_mod_power_of_two(other, pow)
    }
}
