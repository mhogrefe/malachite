use malachite_base::num::arithmetic::traits::NegAssign;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::logic::traits::NotAssign;
use std::ops::Neg;
use Rational;

impl Neg for Rational {
    type Output = Rational;

    /// Finds the negative of a `Rational`, taking the `Rational` by value.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_q;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!((-Rational::ZERO).to_string(), "0");
    /// assert_eq!((-Rational::from_signeds(22, 7)).to_string(), "-22/7");
    /// assert_eq!((-Rational::from_signeds(-22, 7)).to_string(), "22/7");
    /// ```
    fn neg(mut self) -> Rational {
        if self.numerator != 0 {
            self.sign.not_assign();
        }
        self
    }
}

impl<'a> Neg for &'a Rational {
    type Output = Rational;

    /// Returns the negative of a `Rational`, taking the `Rational` by reference.
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
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!((-&Rational::ZERO).to_string(), "0");
    /// assert_eq!((-&Rational::from_str("22/7").unwrap()).to_string(), "-22/7");
    /// assert_eq!((-&Rational::from_str("-22/7").unwrap()).to_string(), "22/7");
    /// ```
    fn neg(self) -> Rational {
        if self.numerator == 0 {
            Rational::ZERO
        } else {
            Rational {
                sign: !self.sign,
                numerator: self.numerator.clone(),
                denominator: self.denominator.clone(),
            }
        }
    }
}

impl NegAssign for Rational {
    /// Replaces a `Rational` with its negative.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_q;
    ///
    /// use malachite_base::num::arithmetic::traits::NegAssign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// let mut x = Rational::ZERO;
    /// x.neg_assign();
    /// assert_eq!(x.to_string(), "0");
    ///
    /// let mut x = Rational::from_str("22/7").unwrap();
    /// x.neg_assign();
    /// assert_eq!(x.to_string(), "-22/7");
    ///
    /// let mut x = Rational::from_str("-22/7").unwrap();
    /// x.neg_assign();
    /// assert_eq!(x.to_string(), "22/7");
    /// ```
    fn neg_assign(&mut self) {
        if self.numerator != 0 {
            self.sign.not_assign();
        }
    }
}
