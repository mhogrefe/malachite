use malachite_base::num::arithmetic::traits::DivisibleBy;

use integer::Integer;
use platform::Limb;

impl<'a> DivisibleBy<Limb> for &'a Integer {
    /// Returns whether an `Integer` is divisible by a `Limb`; in other words, whether the `Integer`
    /// is a multiple of the `Limb`. This means that zero is divisible by any number, including
    /// zero; but a nonzero number is never divisible by zero.
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
    /// use malachite_base::num::arithmetic::traits::DivisibleBy;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(Integer::ZERO.divisible_by(0u32), true);
    ///     assert_eq!(Integer::from(100).divisible_by(3u32), false);
    ///     assert_eq!(Integer::from(-102).divisible_by(3u32), true);
    /// }
    /// ```
    fn divisible_by(self, other: Limb) -> bool {
        self.abs.divisible_by(other)
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> DivisibleBy<u32> for &'a Integer {
    #[inline]
    fn divisible_by(self, other: u32) -> bool {
        self.divisible_by(Limb::from(other))
    }
}

impl<'a> DivisibleBy<&'a Integer> for Limb {
    /// Returns whether a `Limb` is divisible by an `Integer`; in other words, whether the `Limb` is
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
    /// use malachite_base::num::arithmetic::traits::DivisibleBy;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(0u32.divisible_by(&Integer::ZERO), true);
    ///     assert_eq!(100u32.divisible_by(&Integer::from(3)), false);
    ///     assert_eq!(102u32.divisible_by(&Integer::from(-3)), true);
    /// }
    /// ```
    fn divisible_by(self, other: &'a Integer) -> bool {
        self.divisible_by(&other.abs)
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> DivisibleBy<&'a Integer> for u32 {
    #[inline]
    fn divisible_by(self, other: &'a Integer) -> bool {
        Limb::from(self).divisible_by(other)
    }
}
