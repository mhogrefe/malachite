use malachite_base::num::arithmetic::traits::ModPowerOfTwoIsReduced;
use malachite_base::num::logic::traits::SignificantBits;
use natural::Natural;

impl ModPowerOfTwoIsReduced for Natural {
    /// Returns whether `self` is reduced mod 2<sup>`pow`</sup>; in other words, whether it has no
    /// more than `pow` significant bits.
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
    /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoIsReduced;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO.mod_power_of_two_is_reduced(5), true);
    /// assert_eq!(Natural::trillion().mod_power_of_two_is_reduced(39), false);
    /// assert_eq!(Natural::trillion().mod_power_of_two_is_reduced(40), true);
    /// ```
    #[inline]
    fn mod_power_of_two_is_reduced(&self, pow: u64) -> bool {
        self.significant_bits() <= pow
    }
}
