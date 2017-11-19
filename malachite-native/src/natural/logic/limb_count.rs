use natural::Natural::{self, Large, Small};

impl Natural {
    /// Returns the number of limbs, or base-2^(32) digits, of a `Natural`. Zero has 0 limbs.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_native;
    ///
    /// use malachite_base::traits::Zero;
    /// use malachite_native::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(Natural::zero().limb_count(), 0);
    ///     assert_eq!(Natural::from(123u32).limb_count(), 1);
    ///     assert_eq!(Natural::from_str("1000000000000").unwrap().limb_count(), 2);
    /// }
    /// ```
    pub fn limb_count(&self) -> u64 {
        match *self {
            Small(0) => 0,
            Small(_) => 1,
            Large(ref limbs) => limbs.len() as u64,
        }
    }
}
