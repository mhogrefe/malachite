use malachite_base::num::conversion::traits::IsInteger;
use Rational;

impl<'a> IsInteger for &'a Rational {
    /// Determines whether a `Rational` is an integer.
    ///
    /// $f(x) = x \in \Z$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_q;
    ///
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_base::num::conversion::traits::IsInteger;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Rational::ZERO.is_integer(), true);
    /// assert_eq!(Rational::ONE.is_integer(), true);
    /// assert_eq!(Rational::from(100).is_integer(), true);
    /// assert_eq!(Rational::from_str("22/7").unwrap().is_integer(), false);
    /// ```
    #[inline]
    fn is_integer(self) -> bool {
        self.denominator == 1u32
    }
}
