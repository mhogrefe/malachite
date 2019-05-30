use malachite_base::num::arithmetic::traits::EqModPowerOfTwo;

use integer::Integer;

impl<'a, 'b> EqModPowerOfTwo<&'b Integer> for &'a Integer {
    /// Returns whether two `Integer`s are equivalent mod two to the power of `pow`; that is,
    /// whether their `pow` least-significant bits are equal.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = min(`pow`, max(`self.significant_bits()`, `other.significant_bits()`))
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::EqModPowerOfTwo;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(Integer::ZERO.eq_mod_power_of_two(&Integer::from(-256), 8), true);
    ///     assert_eq!(Integer::from(-0b1101).eq_mod_power_of_two(&Integer::from(0b11011), 3),
    ///         true);
    ///     assert_eq!(Integer::from(-0b1101).eq_mod_power_of_two(&Integer::from(0b11011), 4),
    ///         false);
    /// }
    /// ```
    fn eq_mod_power_of_two(self, other: &'b Integer, pow: u64) -> bool {
        if self.sign == other.sign {
            self.abs.eq_mod_power_of_two(&other.abs, pow)
        } else {
            self.abs.eq_mod_power_of_two_neg_pos(&other.abs, pow)
        }
    }
}
