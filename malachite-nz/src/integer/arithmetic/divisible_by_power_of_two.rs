use malachite_base::num::traits::DivisibleByPowerOfTwo;

use integer::Integer;

impl<'a> DivisibleByPowerOfTwo for &'a Integer {
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
    /// use malachite_base::num::traits::{DivisibleByPowerOfTwo, Zero};
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(Integer::ZERO.divisible_by_power_of_two(100), true);
    ///     assert_eq!(Integer::from(-100).divisible_by_power_of_two(2), true);
    ///     assert_eq!(Integer::from(100u32).divisible_by_power_of_two(3), false);
    ///     assert_eq!((-Integer::trillion()).divisible_by_power_of_two(12), true);
    ///     assert_eq!((-Integer::trillion()).divisible_by_power_of_two(13), false);
    /// }
    /// ```
    fn divisible_by_power_of_two(self, pow: u64) -> bool {
        self.abs.divisible_by_power_of_two(pow)
    }
}
