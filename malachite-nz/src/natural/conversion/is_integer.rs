use crate::natural::Natural;
use malachite_base::num::conversion::traits::IsInteger;

impl<'a> IsInteger for &'a Natural {
    /// Determines whether a [`Natural`] is an integer. It always returns `true`.
    ///
    /// $f(x) = \textrm{true}$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_base::num::conversion::traits::IsInteger;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO.is_integer(), true);
    /// assert_eq!(Natural::ONE.is_integer(), true);
    /// assert_eq!(Natural::from(100u32).is_integer(), true);
    /// ```
    #[inline]
    fn is_integer(self) -> bool {
        true
    }
}
