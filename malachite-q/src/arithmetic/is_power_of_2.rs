use crate::Rational;
use malachite_base::num::arithmetic::traits::IsPowerOf2;

impl IsPowerOf2 for Rational {
    /// Determines whether a [`Rational`] is an integer power of 2.
    ///
    /// $f(x) = (\exists n \in \Z : 2^n = x)$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::arithmetic::traits::IsPowerOf2;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::from(0x80).is_power_of_2(), true);
    /// assert_eq!(Rational::from_signeds(1, 8).is_power_of_2(), true);
    /// assert_eq!(Rational::from_signeds(-1, 8).is_power_of_2(), false);
    /// assert_eq!(Rational::from_signeds(22, 7).is_power_of_2(), false);
    /// ```
    fn is_power_of_2(&self) -> bool {
        self.sign
            && (self.denominator == 1u32 && self.numerator.is_power_of_2()
                || self.numerator == 1u32 && self.denominator.is_power_of_2())
    }
}
