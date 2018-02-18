use malachite_base::num::SignificantBits;
use natural::LOG_LIMB_BITS;
use natural::Natural::{self, Large, Small};

impl<'a> SignificantBits for &'a Natural {
    /// Returns the smallest number of bits necessary to represent a `Natural`. 0 has zero
    /// significant bits.
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
            Large(ref limbs) => {
                ((limbs.len() as u64) << u64::from(LOG_LIMB_BITS))
                    - u64::from(limbs.last().unwrap().leading_zeros())
            }
        }
    }
}
