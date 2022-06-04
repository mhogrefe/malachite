use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::traits::One;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use Rational;

impl From<Integer> for Rational {
    /// Converts an [`Integer`](malachite_nz::integer::Integer) to a [`Rational`], taking the
    /// [`Integer`](malachite_nz::integer::Integer) by value.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_nz;
    ///
    /// use malachite_nz::integer::Integer;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::from(Integer::from(123)), 123);
    /// assert_eq!(Rational::from(Integer::from(-123)), -123);
    /// ```
    fn from(value: Integer) -> Rational {
        Rational {
            sign: value >= 0,
            numerator: value.unsigned_abs(),
            denominator: Natural::ONE,
        }
    }
}

impl<'a> From<&'a Integer> for Rational {
    /// Converts an [`Integer`](malachite_nz::integer::Integer) to a [`Rational`], taking the
    /// [`Integer`](malachite_nz::integer::Integer) by reference.
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
    /// extern crate malachite_nz;
    ///
    /// use malachite_nz::integer::Integer;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::from(&Integer::from(123)), 123);
    /// assert_eq!(Rational::from(&Integer::from(-123)), -123);
    /// ```
    fn from(value: &'a Integer) -> Rational {
        Rational {
            sign: *value >= 0,
            numerator: value.unsigned_abs(),
            denominator: Natural::ONE,
        }
    }
}
