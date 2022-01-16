use malachite_nz::integer::Integer;
use Rational;

impl PartialEq<Integer> for Rational {
    /// Determines whether a `Rational` is equal to an `Integer`.
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
    /// use malachite_nz::integer::Integer;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// assert!(Rational::from(-123) == Integer::from(-123));
    /// assert!(Rational::from_str("22/7").unwrap() != Integer::from(5));
    /// ```
    fn eq(&self, other: &Integer) -> bool {
        self.sign == (*other >= 0)
            && self.denominator == 1
            && self.numerator == *other.unsigned_abs_ref()
    }
}

impl PartialEq<Rational> for Integer {
    /// Determines whether an `Integer` is equal to a `Rational`.
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
    /// use malachite_nz::integer::Integer;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// assert!(Integer::from(-123) == Rational::from(-123));
    /// assert!(Integer::from(5) != Rational::from_str("22/7").unwrap());
    /// ```
    fn eq(&self, other: &Rational) -> bool {
        other.sign == (*self >= 0)
            && other.denominator == 1
            && *self.unsigned_abs_ref() == other.numerator
    }
}
