use natural::Natural;

impl Natural {
    /// Returns the number of limbs, or base-2^(32) digits, of a `Natural`. Zero has 0 limbs.
    /// Although GMP may use 32- or 64-bit limbs internally, this method always returns 32-bit
    /// limbs.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_gmp;
    ///
    /// use malachite_base::traits::Zero;
    /// use malachite_gmp::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(Natural::ZERO.limb_count(), 0);
    ///     assert_eq!(Natural::from(123u32).limb_count(), 1);
    ///     assert_eq!(Natural::trillion().limb_count(), 2);
    /// }
    /// ```
    pub fn limb_count(&self) -> u64 {
        let bit_size = self.significant_bits();
        if bit_size.trailing_zeros() >= 5 {
            bit_size >> 5
        } else {
            (bit_size >> 5) + 1
        }
    }
}
