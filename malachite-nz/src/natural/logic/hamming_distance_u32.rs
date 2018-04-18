use malachite_base::num::HammingDistance;
use natural::logic::count_ones::limbs_count_ones;
use natural::Natural::{self, Large, Small};

/// Interpreting a slice of `u32`s as the limbs of a `Natural` in ascending order, returns the
/// Hamming distance between that `Natural` and a `u32`. Both have infinitely many implicit leading
/// zeros. `limbs` cannot be empty.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if `limbs` is empty.
///
/// # Example
/// ```
/// use malachite_nz::natural::logic::hamming_distance_u32::limbs_hamming_distance_limb;
///
/// assert_eq!(limbs_hamming_distance_limb(&[2], 3), 1);
/// assert_eq!(limbs_hamming_distance_limb(&[1, 1, 1], 1), 2);
/// ```
pub fn limbs_hamming_distance_limb(limbs: &[u32], other_limb: u32) -> u64 {
    limbs[0].hamming_distance(other_limb) + limbs_count_ones(&limbs[1..])
}

impl<'a> HammingDistance<u32> for &'a Natural {
    /// Determines the Hamming distance between a `Natural` and a `u32`. Both have infinitely many
    /// implicit leading zeros.
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
    /// use malachite_base::num::{HammingDistance, One, Zero};
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(Natural::ZERO.hamming_distance(0), 0);
    ///     // 105 = 1101001b, 123 = 1111011
    ///     assert_eq!(Natural::from(105u32).hamming_distance(123), 2);
    ///     assert_eq!((Natural::ONE << 100u32).hamming_distance(0), 1);
    /// }
    /// ```
    fn hamming_distance(self, other: u32) -> u64 {
        match *self {
            Small(small) => small.hamming_distance(other),
            Large(ref limbs) => limbs_hamming_distance_limb(limbs, other),
        }
    }
}

impl<'a> HammingDistance<&'a Natural> for u32 {
    /// Determines the Hamming distance between a `u32` and a `Natural`. Both have infinitely many
    /// implicit leading zeros.
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
    /// use malachite_base::num::{HammingDistance, One, Zero};
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(0.hamming_distance(&Natural::ZERO), 0);
    ///     // 105 = 1101001b, 123 = 1111011
    ///     assert_eq!(123.hamming_distance(&Natural::from(105u32)), 2);
    ///     assert_eq!(0.hamming_distance(&(Natural::ONE << 100u32)), 1);
    /// }
    /// ```
    fn hamming_distance(self, other: &'a Natural) -> u64 {
        other.hamming_distance(self)
    }
}
