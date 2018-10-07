use integer::Integer;
use malachite_base::num::{EqModPowerOfTwo, UnsignedAbs};

impl<'a> EqModPowerOfTwo<i32> for &'a Integer {
    /// Returns whether this `Integer` is equivalent to an `i32` mod two to the power of `pow`; that
    /// is, whether the `pow` least-significant twos-complement bits of the `Integer` and the `i32`
    /// are equal.
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
    /// use malachite_base::num::{EqModPowerOfTwo, Zero};
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!((&Integer::ZERO).eq_mod_power_of_two(256, 8), true);
    ///     assert_eq!((&Integer::from(-0b1101)).eq_mod_power_of_two(0b1011, 3), true);
    ///     assert_eq!((&Integer::from(-0b1101)).eq_mod_power_of_two(0b1011, 4), false);
    ///     assert_eq!((&Integer::from(0b1101)).eq_mod_power_of_two(-0b1011, 3), true);
    ///     assert_eq!((&Integer::from(0b1101)).eq_mod_power_of_two(-0b1011, 4), false);
    /// }
    /// ```
    fn eq_mod_power_of_two(self, other: i32, pow: u64) -> bool {
        let other_abs = other.unsigned_abs();
        if other >= 0 {
            self.eq_mod_power_of_two(other_abs, pow)
        } else if self.sign {
            self.abs.eq_mod_power_of_two_neg_u32(other_abs, pow)
        } else {
            self.abs.eq_mod_power_of_two(other_abs, pow)
        }
    }
}
