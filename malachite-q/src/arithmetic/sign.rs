use malachite_base::num::arithmetic::traits::Sign;
use std::cmp::Ordering;
use Rational;

impl Sign for Rational {
    /// Compares a [`Rational`] to zero.
    ///
    /// Returns `Greater`, `Equal`, or `Less`, depending on whether the [`Rational`] is positive,
    /// zero, or negative, respectively.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::arithmetic::traits::Sign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering;
    ///
    /// assert_eq!(Rational::ZERO.sign(), Ordering::Equal);
    /// assert_eq!(Rational::from_signeds(22, 7).sign(), Ordering::Greater);
    /// assert_eq!(Rational::from_signeds(-22, 7).sign(), Ordering::Less);
    /// ```
    fn sign(&self) -> Ordering {
        if self.sign {
            if self.numerator == 0 {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        } else {
            Ordering::Less
        }
    }
}
