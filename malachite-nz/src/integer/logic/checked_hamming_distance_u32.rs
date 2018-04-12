use malachite_base::num::{CheckedHammingDistance, HammingDistance};
use integer::Integer;

impl<'a> CheckedHammingDistance<u32> for &'a Integer {
    /// Determines the Hamming distance between an `Integer` and a `u32`. The `u32` has infinitely
    /// many leading zeros. If the `Integer` is non-negative, it also has infinitely many leading
    /// zeros and the Hamming distance is finite; if it is negative, it has infinitely many leading
    /// ones and the Hamming distance is infinite, so `None` is returned.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::{CheckedHammingDistance, NegativeOne, One, Zero};
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(Integer::ZERO.checked_hamming_distance(0u32), Some(0));
    ///     // 105 = 1101001b, 123 = 1111011
    ///     assert_eq!(Integer::from(105).checked_hamming_distance(123u32), Some(2));
    ///     assert_eq!(Integer::from(-105).checked_hamming_distance(123u32), None);
    ///     assert_eq!((Integer::ONE << 100u32).checked_hamming_distance(0u32), Some(1));
    ///     assert_eq!((Integer::NEGATIVE_ONE << 100u32).checked_hamming_distance(0u32), None);
    /// }
    /// ```
    fn checked_hamming_distance(self, other: u32) -> Option<u64> {
        if self.sign {
            Some(self.abs.hamming_distance(other))
        } else {
            None
        }
    }
}
