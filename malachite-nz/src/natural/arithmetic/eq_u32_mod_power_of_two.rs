use malachite_base::num::{EqModPowerOfTwo, PrimitiveInteger};
use natural::arithmetic::divisible_by_power_of_two::limbs_divisible_by_power_of_two;
use natural::Natural::{self, Large, Small};

/// Interpreting a slice of `u32`s as the limbs (in ascending order) of a `Natural`, returns whether
/// the `Natural` is equivalent to a limb mod two to the power of `pow`; that is, whether the `pow`
/// least-significant bits of the `Natural` and the limb are equal.
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
/// use malachite_nz::natural::arithmetic::eq_u32_mod_power_of_two::limbs_eq_limb_mod_power_of_two;
///
/// assert_eq!(limbs_eq_limb_mod_power_of_two(&[0b1111011, 0b111001000], 0b101011, 4), true);
/// assert_eq!(limbs_eq_limb_mod_power_of_two(&[0b1111011, 0b111001000], 0b101011, 5), false);
/// assert_eq!(limbs_eq_limb_mod_power_of_two(&[0b1111011, 0b111001000], 0b1111011, 35), true);
/// assert_eq!(limbs_eq_limb_mod_power_of_two(&[0b1111011, 0b111001000], 0b1111011, 36), false);
/// assert_eq!(limbs_eq_limb_mod_power_of_two(&[0b1111011, 0b111001000], 0b1111011, 100), false);
/// ```
pub fn limbs_eq_limb_mod_power_of_two(limbs: &[u32], limb: u32, pow: u64) -> bool {
    let i = (pow >> u32::LOG_WIDTH) as usize;
    if i >= limbs.len() {
        false
    } else if i == 0 {
        limbs[0].eq_mod_power_of_two(limb, pow)
    } else {
        limbs[0] == limb
            && limbs_divisible_by_power_of_two(&limbs[1..], pow - u64::from(u32::WIDTH))
    }
}

impl<'a> EqModPowerOfTwo<u32> for &'a Natural {
    /// Returns whether this `Natural` is equivalent to a `u32` mod two to the power of `pow`; that
    /// is, whether the `pow` least-significant bits of the `Natural` and the `u32` are equal.
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
    /// use malachite_base::num::{EqModPowerOfTwo, Zero};
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(Natural::ZERO.eq_mod_power_of_two(256u32, 8), true);
    ///     assert_eq!(Natural::from(0b1101u32).eq_mod_power_of_two(0b10101, 3), true);
    ///     assert_eq!(Natural::from(0b1101u32).eq_mod_power_of_two(0b10101, 4), false);
    /// }
    /// ```
    fn eq_mod_power_of_two(self, other: u32, pow: u64) -> bool {
        match *self {
            Small(small) => small.eq_mod_power_of_two(other, pow),
            Large(ref limbs) => limbs_eq_limb_mod_power_of_two(limbs, other, pow),
        }
    }
}

impl<'a> EqModPowerOfTwo<&'a Natural> for u32 {
    /// Returns whether this `u32` is equivalent to a `Natural` mod two to the power of `pow`; that
    /// is, whether the `pow` least-significant bits of the `u32` and the `Natural` are equal.
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
    /// use malachite_base::num::{EqModPowerOfTwo, Zero};
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
