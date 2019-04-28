use integer::Integer;
use malachite_base::num::traits::{EqModPowerOfTwo, UnsignedAbs};
use platform::SignedLimb;

impl<'a> EqModPowerOfTwo<SignedLimb> for &'a Integer {
    /// Returns whether this `Integer` is equivalent to a `SignedLimb` mod two to the power of
    /// `pow`; that is, whether the `pow` least-significant twos-complement bits of the `Integer`
    /// and the `SignedLimb` are equal.
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
    /// use malachite_base::num::traits::{EqModPowerOfTwo, Zero};
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!((&Integer::ZERO).eq_mod_power_of_two(256i32, 8), true);
    ///     assert_eq!((&Integer::from(-0b1101)).eq_mod_power_of_two(0b1011i32, 3), true);
    ///     assert_eq!((&Integer::from(-0b1101)).eq_mod_power_of_two(0b1011i32, 4), false);
    ///     assert_eq!((&Integer::from(0b1101)).eq_mod_power_of_two(-0b1011, 3), true);
    ///     assert_eq!((&Integer::from(0b1101)).eq_mod_power_of_two(-0b1011, 4), false);
    /// }
    /// ```
    fn eq_mod_power_of_two(self, other: SignedLimb, pow: u64) -> bool {
        let other_abs = other.unsigned_abs();
        if other >= 0 {
            self.eq_mod_power_of_two(other_abs, pow)
        } else if self.sign {
            self.abs.eq_mod_power_of_two_neg_limb(other_abs, pow)
        } else {
            self.abs.eq_mod_power_of_two(other_abs, pow)
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> EqModPowerOfTwo<i32> for &'a Integer {
    #[inline]
    fn eq_mod_power_of_two(self, other: i32, pow: u64) -> bool {
        self.eq_mod_power_of_two(SignedLimb::from(other), pow)
    }
}

impl<'a> EqModPowerOfTwo<&'a Integer> for SignedLimb {
    /// Returns whether this `SignedLimb` is equivalent to a `Integer` mod two to the power of
    /// `pow`; that is, whether the `pow` least-significant twos-complement bits of the `SignedLimb`
    /// and the `Integer` are equal.
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
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(256i32.eq_mod_power_of_two(&Integer::ZERO, 8), true);
    ///     assert_eq!(0b10101i32.eq_mod_power_of_two(&Integer::from(-0b11011), 3), true);
    ///     assert_eq!(0b10101i32.eq_mod_power_of_two(&Integer::from(-0b10011), 4), false);
    ///     assert_eq!((-0b11011).eq_mod_power_of_two(&Integer::from(0b10101), 3), true);
    ///     assert_eq!((-0b10011).eq_mod_power_of_two(&Integer::from(0b10101), 4), false);
    /// }
    /// ```
    #[inline]
    fn eq_mod_power_of_two(self, other: &'a Integer, pow: u64) -> bool {
        other.eq_mod_power_of_two(self, pow)
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> EqModPowerOfTwo<&'a Integer> for i32 {
    #[inline]
    fn eq_mod_power_of_two(self, other: &'a Integer, pow: u64) -> bool {
        SignedLimb::from(self).eq_mod_power_of_two(other, pow)
    }
}
