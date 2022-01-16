use malachite_nz::natural::Natural;
use Rational;

impl PartialEq<Natural> for Rational {
    /// Determines whether a `Rational` is equal to a `Natural`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where n = `min(self.significant_bits(), other.significant_bits())`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_nz;
    ///
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// assert!(Rational::from(123) == Natural::from(123u32));
    /// assert!(Rational::from_str("22/7").unwrap() != Natural::from(5u32));
    /// ```
    fn eq(&self, other: &Natural) -> bool {
        self.sign && self.denominator == 1 && self.numerator == *other
    }
}

impl PartialEq<Rational> for Natural {
    /// Determines whether a `Natural` is equal to a `Rational`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where n = `min(self.significant_bits(), other.significant_bits())`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_nz;
    ///
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// assert!(Natural::from(123u32) == Rational::from(123));
    /// assert!(Natural::from(5u32) != Rational::from_str("22/7").unwrap());
    /// ```
    fn eq(&self, other: &Rational) -> bool {
        other.sign && other.denominator == 1 && *self == other.numerator
    }
}
