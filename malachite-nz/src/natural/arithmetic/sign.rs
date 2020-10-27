use std::cmp::Ordering;

use malachite_base::num::arithmetic::traits::Sign;

use natural::Natural;

impl Sign for Natural {
    /// Returns the sign of an `Natural`. Interpret the result as the result of a comparison to
    /// zero, so that `Equal` means zero and `Greater` means positive.
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
    /// use malachite_base::num::arithmetic::traits::Sign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    /// use std::cmp::Ordering;
    ///
    /// assert_eq!(Natural::ZERO.sign(), Ordering::Equal);
    /// assert_eq!(Natural::from(123u32).sign(), Ordering::Greater);
    /// ```
    fn sign(&self) -> Ordering {
        if *self == 0 {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }
}
