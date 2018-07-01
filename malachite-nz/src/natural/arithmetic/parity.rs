use malachite_base::num::Parity;
use natural::Natural::{self, Large, Small};

impl Parity for Natural {
    /// Determines whether a `Natural` is even.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::{Parity, Zero};
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(Natural::ZERO.is_even(), true);
    ///     assert_eq!(Natural::from(123u32).is_even(), false);
    ///     assert_eq!(Natural::from(0x80u32).is_even(), true);
    ///     assert_eq!(Natural::trillion().is_even(), true);
    ///     assert_eq!((Natural::trillion() + 1).is_even(), false);
    /// }
    /// ```
    fn is_even(&self) -> bool {
        match *self {
            Small(small) => small.is_even(),
            Large(ref limbs) => limbs[0].is_even(),
        }
    }

    /// Determines whether a `Natural` is odd.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::{Parity, Zero};
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(Natural::ZERO.is_odd(), false);
    ///     assert_eq!(Natural::from(123u32).is_odd(), true);
    ///     assert_eq!(Natural::from(0x80u32).is_odd(), false);
    ///     assert_eq!(Natural::trillion().is_odd(), false);
    ///     assert_eq!((Natural::trillion() + 1).is_odd(), true);
    /// }
    /// ```
    fn is_odd(&self) -> bool {
        match *self {
            Small(small) => small.is_odd(),
            Large(ref limbs) => limbs[0].is_odd(),
        }
    }
}
