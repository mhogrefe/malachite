use integer::Integer;
use malachite_base::num::DivisibleBy;

impl<'a> DivisibleBy<u32> for &'a Integer {
    /// Returns whether an `Integer` is divisible by a `u32`; in other words, whether the `Integer`
    /// is a multiple of the `u32`. This means that zero is divisible by any number, including zero;
    /// but a nonzero number is never divisible by zero.
    ///
    /// This method is more efficient than finding a remainder and checking whether it's zero.
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
    /// use malachite_base::num::{DivisibleBy, Zero};
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(Integer::ZERO.divisible_by(0), true);
    ///     assert_eq!(Integer::from(100).divisible_by(3), false);
    ///     assert_eq!(Integer::from(-102).divisible_by(3), true);
    /// }
    /// ```
    fn divisible_by(self, other: u32) -> bool {
        self.abs.divisible_by(other)
    }
}

impl<'a> DivisibleBy<&'a Integer> for u32 {
    /// Returns whether a `u32` is divisible by an `Integer`; in other words, whether the `u32` is a
    /// multiple of the `Integer`. This means that zero is divisible by any number, including zero;
    /// but a nonzero number is never divisible by zero.
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
    /// use malachite_base::num::{DivisibleBy, Zero};
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(0.divisible_by(&Integer::ZERO), true);
    ///     assert_eq!(100.divisible_by(&Integer::from(3)), false);
    ///     assert_eq!(102.divisible_by(&Integer::from(-3)), true);
    /// }
    /// ```
    fn divisible_by(self, other: &'a Integer) -> bool {
        self.divisible_by(&other.abs)
    }
}
