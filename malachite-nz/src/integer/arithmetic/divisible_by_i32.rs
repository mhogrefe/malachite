use integer::Integer;
use malachite_base::num::{DivisibleBy, UnsignedAbs};

impl<'a> DivisibleBy<i32> for &'a Integer {
    /// Returns whether an `Integer` is divisible by an `i32`; in other words, whether the `Integer`
    /// is a multiple of the `i32`. This means that zero is divisible by any number, including zero;
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
    ///     assert_eq!(Integer::ZERO.divisible_by(0i32), true);
    ///     assert_eq!(Integer::from(100).divisible_by(-3i32), false);
    ///     assert_eq!(Integer::from(-102).divisible_by(3i32), true);
    /// }
    /// ```
    fn divisible_by(self, other: i32) -> bool {
        self.abs.divisible_by(other.unsigned_abs())
    }
}

impl<'a> DivisibleBy<&'a Integer> for i32 {
    /// Returns whether an `i32` is divisible by an `Integer`; in other words, whether the `i32` is
    /// a multiple of the `Integer`. This means that zero is divisible by any number, including
    /// zero; but a nonzero number is never divisible by zero.
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
    ///     assert_eq!(0i32.divisible_by(&Integer::ZERO), true);
    ///     assert_eq!((-100i32).divisible_by(&Integer::from(3)), false);
    ///     assert_eq!(102i32.divisible_by(&Integer::from(-3)), true);
    /// }
    /// ```
    fn divisible_by(self, other: &'a Integer) -> bool {
        self.unsigned_abs().divisible_by(&other.abs)
    }
}
