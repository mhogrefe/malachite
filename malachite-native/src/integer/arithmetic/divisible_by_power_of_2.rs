use integer::Integer;

impl Integer {
    /// Returns whether `self` is divisible by 2^(`pow`). If `self` is 0, the result is always true;
    /// otherwise, it is equivalent to `self.trailing_zeros().unwrap() <= pow`, but more efficient.
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
    /// extern crate malachite_native;
    ///
    /// use malachite_base::traits::Zero;
    /// use malachite_native::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(Integer::zero().divisible_by_power_of_2(100), true);
    ///     assert_eq!(Integer::from(-100).divisible_by_power_of_2(2), true);
    ///     assert_eq!(Integer::from(100u32).divisible_by_power_of_2(3), false);
    ///     assert_eq!(Integer::from_str("-1000000000000").unwrap().divisible_by_power_of_2(12),
    ///         true);
    ///     assert_eq!(Integer::from_str("1000000000000").unwrap().divisible_by_power_of_2(13),
    ///         false);
    /// }
    /// ```
    pub fn divisible_by_power_of_2(&self, pow: u64) -> bool {
        self.abs.divisible_by_power_of_2(pow)
    }
}
