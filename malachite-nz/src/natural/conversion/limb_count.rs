use malachite_base::conversion::WrappingFrom;

use natural::Natural::{self, Large, Small};

impl Natural {
    /// Returns the number of limbs of a `Natural`. Zero has 0
    /// limbs.
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
    /// use malachite_base::num::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(Natural::ZERO.limb_count(), 0);
    ///     assert_eq!(Natural::from(123u32).limb_count(), 1);
    ///     assert_eq!(Natural::trillion().limb_count(), 2);
    /// }
    /// ```
    pub fn limb_count(&self) -> u64 {
        match *self {
            Small(0) => 0,
            Small(_) => 1,
            Large(ref limbs) => u64::wrapping_from(limbs.len()),
        }
    }
}
