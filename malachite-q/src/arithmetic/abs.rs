use malachite_base::num::arithmetic::traits::{Abs, AbsAssign};
use Rational;

impl Abs for Rational {
    type Output = Rational;

    /// Finds the absolute value of a `Rational`, taking the `Rational` by value.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_q;
    ///
    /// use malachite_base::num::arithmetic::traits::Abs;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::ZERO.abs().to_string(), "0");
    /// assert_eq!(Rational::from_signeds(22, 7).abs().to_string(), "22/7");
    /// assert_eq!(Rational::from_signeds(-22, 7).abs().to_string(), "22/7");
    /// ```
    fn abs(mut self) -> Rational {
        self.sign = true;
        self
    }
}

impl<'a> Abs for &'a Rational {
    type Output = Rational;

    /// Finds the absolute value of a `Rational`, taking the `Rational` by reference.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_q;
    ///
    /// use malachite_base::num::arithmetic::traits::Abs;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!((&Rational::ZERO).abs().to_string(), "0");
    /// assert_eq!((&Rational::from_str("22/7").unwrap()).abs().to_string(), "22/7");
    /// assert_eq!((&Rational::from_str("-22/7").unwrap()).abs().to_string(), "22/7");
    /// ```
    fn abs(self) -> Rational {
        Rational {
            sign: true,
            numerator: self.numerator.clone(),
            denominator: self.denominator.clone(),
        }
    }
}

impl AbsAssign for Rational {
    /// Replaces a `Rational` with its absolute value.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_q;
    ///
    /// use malachite_base::num::arithmetic::traits::AbsAssign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// let mut x = Rational::ZERO;
    /// x.abs_assign();
    /// assert_eq!(x.to_string(), "0");
    ///
    /// let mut x = Rational::from_str("22/7").unwrap();
    /// x.abs_assign();
    /// assert_eq!(x.to_string(), "22/7");
    ///
    /// let mut x = Rational::from_str("-22/7").unwrap();
    /// x.abs_assign();
    /// assert_eq!(x.to_string(), "22/7");
    /// ```
    fn abs_assign(&mut self) {
        self.sign = true;
    }
}
