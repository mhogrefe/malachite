use natural::Natural;

impl Natural {
    /// Returns the number of limbs, or base-2^(32) digits, of `self`. Although GMP may use 32- or
    /// 64-bit limbs internally, this method always returns 32-bit limbs. Zero has 0 limbs.
    ///
    /// # Examples
    /// ```
    /// use malachite_gmp::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Natural::from(0).limb_count(), 0);
    /// assert_eq!(Natural::from(123).limb_count(), 1);
    /// assert_eq!(Natural::from_str("1000000000000").unwrap().limb_count(), 2);
    /// ```
    pub fn limb_count(&self) -> u64 {
        let bit_size = self.significant_bits();
        if bit_size & 0x1f == 0 {
            bit_size >> 5
        } else {
            (bit_size >> 5) + 1
        }
    }
}
