use malachite_nz::natural::Natural;
use crate::Rational;

impl PartialEq<Natural> for Rational {
    /// Determines whether a [`Rational`] is equal to a
    /// [`Natural`](malachite_nz::natural::Natural).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `min(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_nz;
    ///
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    ///
    /// assert!(Rational::from(123) == Natural::from(123u32));
    /// assert!(Rational::from_signeds(22, 7) != Natural::from(5u32));
    /// ```
    fn eq(&self, other: &Natural) -> bool {
        self.sign && self.denominator == 1 && self.numerator == *other
    }
}

impl PartialEq<Rational> for Natural {
    /// Determines whether a [`Natural`](malachite_nz::natural::Natural) is equal to a
    /// [`Rational`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `min(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_nz;
    ///
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    ///
    /// assert!(Natural::from(123u32) == Rational::from(123));
    /// assert!(Natural::from(5u32) != Rational::from_signeds(22, 7));
    /// ```
    fn eq(&self, other: &Rational) -> bool {
        other.sign && other.denominator == 1 && *self == other.numerator
    }
}
