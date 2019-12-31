use malachite_base::num::arithmetic::traits::Parity;

use natural::InnerNatural::{Large, Small};
use natural::Natural;

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
    /// use malachite_base::num::arithmetic::traits::Parity;
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO.even(), true);
    /// assert_eq!(Natural::from(123u32).even(), false);
    /// assert_eq!(Natural::from(0x80u32).even(), true);
    /// assert_eq!(Natural::trillion().even(), true);
    /// assert_eq!((Natural::trillion() + Natural::ONE).even(), false);
    /// ```
    fn even(self) -> bool {
        match self {
            Natural(Small(small)) => small.even(),
            Natural(Large(ref limbs)) => limbs[0].even(),
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
    /// use malachite_base::num::arithmetic::traits::Parity;
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO.odd(), false);
    /// assert_eq!(Natural::from(123u32).odd(), true);
    /// assert_eq!(Natural::from(0x80u32).odd(), false);
    /// assert_eq!(Natural::trillion().odd(), false);
    /// assert_eq!((Natural::trillion() + Natural::ONE).odd(), true);
    /// ```
    fn odd(self) -> bool {
        match *self {
            Natural(Small(small)) => small.odd(),
            Natural(Large(ref limbs)) => limbs[0].odd(),
        }
    }
}
