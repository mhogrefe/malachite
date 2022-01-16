use malachite_base::num::arithmetic::traits::Sign;
use std::cmp::Ordering;
use Rational;

impl Sign for Rational {
    /// Returns the sign of an `Rational`. Interpret the result as the result of a comparison to
    /// zero, so that `Equal` means zero, `Greater` means positive, and `Less` means negative.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_q;
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
