use malachite_base::misc::WrappingInto;
use malachite_base::num::{CheckedHammingDistance, HammingDistance, UnsignedAbs};
use integer::logic::checked_count_zeros::limbs_count_zeros_neg;
use integer::Integer;
use natural::Natural::{self, Large, Small};

/// Interpreting a slice of `u32`s as the limbs of a `Natural` in ascending order, returns the
/// Hamming distance between the negative of that `Natural` (two's complement) and a negative `i32`.
/// Both have infinitely many implicit leading ones. `limbs` cannot be empty or only contain zeros.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if `limbs` is empty or `other_limb` is non-negative.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::checked_hamming_distance_i32::limbs_hamming_distance_limb_neg;
///
/// assert_eq!(limbs_hamming_distance_limb_neg(&[2], -2), 0);
/// assert_eq!(limbs_hamming_distance_limb_neg(&[1, 1, 1], -1), 2);
/// ```
pub fn limbs_hamming_distance_limb_neg(limbs: &[u32], other_limb: i32) -> u64 {
    assert!(other_limb < 0);
    let least_significant_limb = limbs[0].wrapping_neg();
    limbs_count_zeros_neg(limbs) - u64::from(least_significant_limb.count_zeros())
        + least_significant_limb.hamming_distance(other_limb.wrapping_into())
}

impl<'a> CheckedHammingDistance<i32> for &'a Integer {
    /// Determines the Hamming distance between an `Integer` and an `i32`. The Integer and `u32`
    /// have infinitely many leading zeros or infinitely many leading ones, depending on their
    /// signs. If they are both non-negative or both negative, the Hamming distance is finite. If
    /// one is non-negative and the other is negative, the Hamming distance is infinite, so `None`
    /// is returned.
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
    ///     assert_eq!(Integer::ZERO.checked_hamming_distance(0), Some(0));
    ///     // 105 = 1101001b, 123 = 1111011
    ///     assert_eq!(Integer::from(105).checked_hamming_distance(123), Some(2));
    ///     assert_eq!(Integer::from(105).checked_hamming_distance(-123), None);
    ///     assert_eq!(Integer::from(-105).checked_hamming_distance(123), None);
    ///     // -105 = 10010111, -123 = 10000101 in two's complement
    ///     assert_eq!(Integer::from(-105).checked_hamming_distance(-123), Some(2));
    ///     assert_eq!((Integer::ONE << 100u32).checked_hamming_distance(0), Some(1));
    ///     assert_eq!((Integer::NEGATIVE_ONE << 100u32).checked_hamming_distance(0), None);
    ///     assert_eq!((Integer::NEGATIVE_ONE << 100u32).checked_hamming_distance(-1), Some(100));
    /// }
    /// ```
    fn checked_hamming_distance(self, other: i32) -> Option<u64> {
        if self.sign != (other >= 0) {
            None
        } else if self.sign {
            Some(self.abs.hamming_distance(other.unsigned_abs()))
        } else {
            Some(self.abs.hamming_distance_neg(other))
        }
    }
}

impl Natural {
    fn hamming_distance_neg(&self, other: i32) -> u64 {
        match *self {
            Small(small) => small.wrapping_neg().hamming_distance(other.wrapping_into()),
            Large(ref limbs) => limbs_hamming_distance_limb_neg(limbs, other),
        }
    }
}
