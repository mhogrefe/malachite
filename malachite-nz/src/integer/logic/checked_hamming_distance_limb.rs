use malachite_base::num::logic::traits::{CheckedHammingDistance, HammingDistance};

use integer::Integer;
use platform::Limb;

impl<'a> CheckedHammingDistance<Limb> for &'a Integer {
    /// Determines the Hamming distance between an `Integer` and a `Limb`. The `Limb` has infinitely
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
    /// use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
    /// use malachite_base::num::logic::traits::CheckedHammingDistance;
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
    fn checked_hamming_distance(self, other: Limb) -> Option<u64> {
        if self.sign {
            Some(self.abs.hamming_distance(other))
        } else {
            None
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> CheckedHammingDistance<u32> for &'a Integer {
    #[inline]
    fn checked_hamming_distance(self, other: u32) -> Option<u64> {
        self.checked_hamming_distance(Limb::from(other))
    }
}

impl<'a> CheckedHammingDistance<&'a Integer> for Limb {
    /// Determines the Hamming distance between a `Limb` and an `Integer`. The `Limb` has infinitely
    /// many leading zeros. If the `Integer` is non-negative, it also has infinitely many leading
    /// zeros and the Hamming distance is finite; if it is negative, it has infinitely many leading
    /// ones and the Hamming distance is infinite, so `None` is returned.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `other.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
    /// use malachite_base::num::logic::traits::CheckedHammingDistance;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(0u32.checked_hamming_distance(&Integer::ZERO), Some(0));
    ///     // 105 = 1101001b, 123 = 1111011
    ///     assert_eq!(123u32.checked_hamming_distance(&Integer::from(105)), Some(2));
    ///     assert_eq!(123u32.checked_hamming_distance(&Integer::from(-105)), None);
    ///     assert_eq!(0u32.checked_hamming_distance(&(Integer::ONE << 100u32)), Some(1));
    ///     assert_eq!(0u32.checked_hamming_distance(&(Integer::NEGATIVE_ONE << 100u32)), None);
    /// }
    /// ```
    #[inline]
    fn checked_hamming_distance(self, other: &'a Integer) -> Option<u64> {
        other.checked_hamming_distance(self)
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> CheckedHammingDistance<&'a Integer> for u32 {
    #[inline]
    fn checked_hamming_distance(self, other: &'a Integer) -> Option<u64> {
        Limb::from(self).checked_hamming_distance(other)
    }
}
