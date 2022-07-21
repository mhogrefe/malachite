use crate::integer::Integer;
use malachite_base::num::conversion::traits::IsInteger;

impl<'a> IsInteger for &'a Integer {
    /// Determines whether an [`Integer`] is an integer. It always returns `true`.
    ///
    /// $f(x) = \textrm{true}$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
    /// use malachite_base::num::conversion::traits::IsInteger;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::ZERO.is_integer(), true);
    /// assert_eq!(Integer::ONE.is_integer(), true);
    /// assert_eq!(Integer::from(100).is_integer(), true);
    /// assert_eq!(Integer::NEGATIVE_ONE.is_integer(), true);
    /// assert_eq!(Integer::from(-100).is_integer(), true);
    /// ```
    #[inline]
    fn is_integer(self) -> bool {
        true
    }
}
