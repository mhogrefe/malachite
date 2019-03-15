use integer::Integer;
use malachite_base::num::{CheckedHammingDistance, HammingDistance};
use natural::Natural;

impl<'a, 'b> CheckedHammingDistance<&'a Natural> for &'b Integer {
    /// Determines the Hamming distance between an `Integer` and a `Natural`. The `Integer` has
    /// infinitely many leading zeros or infinitely many leading ones, depending on its sign. If it
    /// is non-negative, the Hamming distance is finite. If it is negative, the Hamming distance is
    /// infinite, so `None` is returned.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = max(`self.significant_bits()`, `other.significant_bits()`)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::CheckedHammingDistance;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(Integer::from(123).checked_hamming_distance(&Natural::from(123u32)),
    ///         Some(0));
    ///     // 105 = 1101001b, 123 = 1111011
    ///     assert_eq!(Integer::from(105).checked_hamming_distance(&Natural::from(123u32)),
    ///         Some(2));
    ///     assert_eq!(Integer::from(-105).checked_hamming_distance(&Natural::from(123u32)), None);
    /// }
    /// ```
    fn checked_hamming_distance(self, other: &Natural) -> Option<u64> {
        if self.sign {
            Some(self.abs.hamming_distance(other))
        } else {
            None
        }
    }
}

impl<'a, 'b> CheckedHammingDistance<&'a Integer> for &'b Natural {
    /// Determines the Hamming distance between a `Natural` and an `Natural`. The `Integer` has
    /// infinitely many leading zeros or infinitely many leading ones, depending on its sign. If it
    /// is non-negative, the Hamming distance is finite. If it is negative, the Hamming distance is
    /// infinite, so `None` is returned.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = max(`self.significant_bits()`, `other.significant_bits()`)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::CheckedHammingDistance;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(Natural::from(123u32).checked_hamming_distance(&Integer::from(123)),
    ///         Some(0));
    ///     // 105 = 1101001b, 123 = 1111011
    ///     assert_eq!(Natural::from(105u32).checked_hamming_distance(&Integer::from(123)),
    ///         Some(2));
    ///     assert_eq!(Natural::from(123u32).checked_hamming_distance(&Integer::from(-105)), None);
    /// }
    /// ```
    #[inline]
    fn checked_hamming_distance(self, other: &Integer) -> Option<u64> {
        other.checked_hamming_distance(self)
    }
}
