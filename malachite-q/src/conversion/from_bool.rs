use crate::Rational;
use malachite_base::num::basic::traits::{One, Zero};

impl From<bool> for Rational {
    /// Converts a [`bool`] to 0 or 1.
    ///
    /// This function is known as the
    /// [Iverson bracket](https://en.wikipedia.org/wiki/Iverson_bracket).
    ///
    /// $$
    /// f(P) = \[P\] = \\begin{cases}
    ///     1 & \text{if} \\quad P, \\\\
    ///     0 & \\text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::from(false), 0);
    /// assert_eq!(Rational::from(true), 1);
    /// ```
    #[inline]
    fn from(b: bool) -> Rational {
        if b {
            Rational::ONE
        } else {
            Rational::ZERO
        }
    }
}
