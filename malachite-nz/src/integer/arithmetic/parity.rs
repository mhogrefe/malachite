use integer::Integer;
use malachite_base::num::arithmetic::traits::Parity;

impl<'a> Parity for &'a Integer {
    /// Determines whether `self` is even.
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
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::ZERO.even(), true);
    /// assert_eq!(Integer::from(123).even(), false);
    /// assert_eq!(Integer::from(-0x80).even(), true);
    /// assert_eq!(Integer::trillion().even(), true);
    /// assert_eq!((-Integer::trillion() - Integer::ONE).even(), false);
    /// ```
    fn even(self) -> bool {
        self.abs.even()
    }

    /// Determines whether `self` is odd.
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
    /// use malachite_base::num::arithmetic::traits::Parity;;
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::ZERO.odd(), false);
    /// assert_eq!(Integer::from(123).odd(), true);
    /// assert_eq!(Integer::from(-0x80).odd(), false);
    /// assert_eq!(Integer::trillion().odd(), false);
    /// assert_eq!((-Integer::trillion() - Integer::ONE).odd(), true);
    /// ```
    fn odd(self) -> bool {
        self.abs.odd()
    }
}
