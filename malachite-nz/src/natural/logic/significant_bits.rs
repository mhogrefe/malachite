use malachite_base::num::SignificantBits;
use natural::LOG_LIMB_BITS;
use natural::Natural::{self, Large, Small};

/// Interpreting a slice of `u32`s as the limbs of a `Natural` in ascending order, returns the
/// smallest number of bits necessary to represent that `Natural`. 0 has zero significant bits. When
/// the `Natural` is nonzero, this is equal to 1 + floor(log<sub>2<\sub>(`self`)).
///
/// This function assumes that `limbs` is nonempty and the last (most significant) limb is nonzero.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Panics
/// Panics if `limbs` is empty.
///
/// # Example
/// ```
/// use malachite_nz::natural::logic::significant_bits::limbs_significant_bits;
///
/// assert_eq!(limbs_significant_bits(&[0b11]), 2);
/// assert_eq!(limbs_significant_bits(&[0, 0b1101]), 36);
/// ```
pub fn limbs_significant_bits(limbs: &[u32]) -> u64 {
    ((limbs.len() as u64 - 1) << u64::from(LOG_LIMB_BITS))
        + limbs.last().unwrap().significant_bits()
}

impl<'a> SignificantBits for &'a Natural {
    /// Returns the smallest number of bits necessary to represent a `Natural`. 0 has zero
    /// significant bits. When `self` is nonzero, this is equal to
    /// 1 + floor(log<sub>2<\sub>(`self`)).
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::SignificantBits;
    /// use malachite_base::num::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(Natural::ZERO.significant_bits(), 0);
    ///     assert_eq!(Natural::from(100u32).significant_bits(), 7);
    /// }
    /// ```
    fn significant_bits(self) -> u64 {
        match *self {
            Small(small) => small.significant_bits(),
            Large(ref limbs) => limbs_significant_bits(limbs),
        }
    }
}
