use malachite_base::num::logic::traits::HammingDistance;

use natural::logic::count_ones::limbs_count_ones;
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;

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
/// use malachite_nz::natural::logic::hamming_distance_limb::limbs_hamming_distance_limb;
///
/// assert_eq!(limbs_hamming_distance_limb(&[2], 3), 1);
/// assert_eq!(limbs_hamming_distance_limb(&[1, 1, 1], 1), 2);
/// ```
pub fn limbs_hamming_distance_limb(limbs: &[Limb], other_limb: Limb) -> u64 {
    limbs[0].hamming_distance(other_limb) + limbs_count_ones(&limbs[1..])
}

impl<'a> HammingDistance<Limb> for &'a Natural {
    /// Determines the Hamming distance between a `Natural` and a `Limb`. Both have infinitely many
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
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_base::num::logic::traits::HammingDistance;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(Natural::ZERO.hamming_distance(0), 0);
    ///     // 105 = 1101001b, 123 = 1111011
    ///     assert_eq!(Natural::from(105u32).hamming_distance(123), 2);
    ///     assert_eq!((Natural::ONE << 100u32).hamming_distance(0), 1);
    /// }
    /// ```
    fn hamming_distance(self, other: Limb) -> u64 {
        match *self {
            Natural(Small(small)) => small.hamming_distance(other),
            Natural(Large(ref limbs)) => limbs_hamming_distance_limb(limbs, other),
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> HammingDistance<u32> for &'a Natural {
    #[inline]
    fn hamming_distance(self, other: u32) -> u64 {
        self.hamming_distance(Limb::from(other))
    }
}

impl<'a> HammingDistance<&'a Natural> for Limb {
    /// Determines the Hamming distance between a `Limb` and a `Natural`. Both have infinitely many
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
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_base::num::logic::traits::HammingDistance;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(0.hamming_distance(&Natural::ZERO), 0);
    ///     // 105 = 1101001b, 123 = 1111011
    ///     assert_eq!(123.hamming_distance(&Natural::from(105u32)), 2);
    ///     assert_eq!(0.hamming_distance(&(Natural::ONE << 100u32)), 1);
    /// }
    /// ```
    #[inline]
    fn hamming_distance(self, other: &'a Natural) -> u64 {
        other.hamming_distance(self)
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> HammingDistance<&'a Natural> for u32 {
    #[inline]
    fn hamming_distance(self, other: &'a Natural) -> u64 {
        Limb::from(self).hamming_distance(other)
    }
}
