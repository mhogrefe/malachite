use integer::Integer;
use malachite_base::num::arithmetic::traits::DivisibleByPowerOf2;

impl<'a> DivisibleByPowerOf2 for &'a Integer {
    /// Returns whether `self` is divisible by 2<sup>`pow`</sup>. If `self` is 0, the result is
    /// always true; otherwise, it is equivalent to `self.trailing_zeros().unwrap() <= pow`, but
    /// more efficient.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `self.significant_bits`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivisibleByPowerOf2;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::ZERO.divisible_by_power_of_2(100), true);
    /// assert_eq!(Integer::from(-100).divisible_by_power_of_2(2), true);
    /// assert_eq!(Integer::from(100u32).divisible_by_power_of_2(3), false);
    /// assert_eq!((-Integer::trillion()).divisible_by_power_of_2(12), true);
    /// assert_eq!((-Integer::trillion()).divisible_by_power_of_2(13), false);
    /// ```
    fn divisible_by_power_of_2(self, pow: u64) -> bool {
        self.abs.divisible_by_power_of_2(pow)
    }
}
