use crate::Rational;
use malachite_base::num::basic::traits::One;
use malachite_nz::natural::Natural;

impl From<Natural> for Rational {
    /// Converts a [`Natural`](malachite_nz::natural::Natural) to a [`Rational`], taking the
    /// [`Natural`](malachite_nz::natural::Natural) by value.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::from(Natural::from(123u32)), 123);
    /// ```
    fn from(value: Natural) -> Rational {
        Rational {
            sign: true,
            numerator: value,
            denominator: Natural::ONE,
        }
    }
}

impl<'a> From<&'a Natural> for Rational {
    /// Converts a [`Natural`](malachite_nz::natural::Natural) to a [`Rational`], taking the
    /// [`Natural`](malachite_nz::natural::Natural) by reference.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `value.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::from(&Natural::from(123u32)), 123);
    /// ```
    fn from(value: &'a Natural) -> Rational {
        Rational {
            sign: true,
            numerator: value.clone(),
            denominator: Natural::ONE,
        }
    }
}
