use crate::Rational;
use malachite_base::num::conversion::traits::IsInteger;

impl<'a> IsInteger for &'a Rational {
    /// Determines whether a [`Rational`] is an integer.
    ///
    /// $f(x) = x \in \Z$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_base::num::conversion::traits::IsInteger;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::ZERO.is_integer(), true);
    /// assert_eq!(Rational::ONE.is_integer(), true);
    /// assert_eq!(Rational::from(100).is_integer(), true);
    /// assert_eq!(Rational::from_signeds(22, 7).is_integer(), false);
    /// ```
    #[inline]
    fn is_integer(self) -> bool {
        self.denominator == 1u32
    }
}
