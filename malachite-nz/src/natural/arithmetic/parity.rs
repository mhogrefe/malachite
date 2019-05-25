use malachite_base::num::traits::Parity;

use natural::Natural::{self, Large, Small};

impl<'a> Parity for &'a Natural {
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
    /// use malachite_base::num::traits::{Parity, Zero};
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(Natural::ZERO.even(), true);
    ///     assert_eq!(Natural::from(123u32).even(), false);
    ///     assert_eq!(Natural::from(0x80u32).even(), true);
    ///     assert_eq!(Natural::trillion().even(), true);
    ///     assert_eq!((Natural::trillion() + 1).even(), false);
    /// }
    /// ```
    fn even(self) -> bool {
        match self {
            Small(small) => small.even(),
            Large(ref limbs) => limbs[0].even(),
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
    /// use malachite_base::num::traits::{Parity, Zero};
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(Natural::ZERO.odd(), false);
    ///     assert_eq!(Natural::from(123u32).odd(), true);
    ///     assert_eq!(Natural::from(0x80u32).odd(), false);
    ///     assert_eq!(Natural::trillion().odd(), false);
    ///     assert_eq!((Natural::trillion() + 1).odd(), true);
    /// }
    /// ```
    fn odd(self) -> bool {
        match *self {
            Small(small) => small.odd(),
            Large(ref limbs) => limbs[0].odd(),
        }
    }
}
