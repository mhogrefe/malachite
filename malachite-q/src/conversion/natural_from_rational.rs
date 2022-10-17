use crate::Rational;
use malachite_base::num::arithmetic::traits::DivRound;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::{ConvertibleFrom, RoundingFrom};
use malachite_base::rounding_modes::RoundingMode;
use malachite_nz::natural::Natural;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NaturalFromRationalError;

impl TryFrom<Rational> for Natural {
    type Error = NaturalFromRationalError;

    /// Converts a [`Rational`] to a [`Natural`](malachite_nz::natural::Natural), taking the
    /// [`Rational`] by value. If the [`Rational`] is negative or not an integer, an error is
    /// returned.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::conversion::natural_from_rational::NaturalFromRationalError;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Natural::try_from(Rational::from(123)).unwrap(), 123);
    /// assert_eq!(Natural::try_from(Rational::from(-123)), Err(NaturalFromRationalError));
    /// assert_eq!(
    ///     Natural::try_from(Rational::from_signeds(22, 7)),
    ///     Err(NaturalFromRationalError)
    /// );
    /// ```
    fn try_from(x: Rational) -> Result<Natural, Self::Error> {
        if x.sign && x.denominator == 1u32 {
            Ok(x.numerator)
        } else {
            Err(NaturalFromRationalError)
        }
    }
}

impl<'a> TryFrom<&'a Rational> for Natural {
    type Error = NaturalFromRationalError;

    /// Converts a [`Rational`] to a [`Natural`](malachite_nz::natural::Natural), taking the
    /// [`Rational`] by reference. If the [`Rational`] is negative or not an integer, an error is
    /// returned.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::conversion::natural_from_rational::NaturalFromRationalError;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Natural::try_from(&Rational::from(123)).unwrap(), 123);
    /// assert_eq!(Natural::try_from(&Rational::from(-123)), Err(NaturalFromRationalError));
    /// assert_eq!(
    ///     Natural::try_from(&Rational::from_signeds(22, 7)),
    ///     Err(NaturalFromRationalError)
    /// );
    /// ```
    fn try_from(x: &Rational) -> Result<Natural, Self::Error> {
        if x.sign && x.denominator == 1u32 {
            Ok(x.numerator.clone())
        } else {
            Err(NaturalFromRationalError)
        }
    }
}

impl<'a> ConvertibleFrom<&'a Rational> for Natural {
    /// Determines whether a [`Rational`] can be converted to a
    /// [`Natural`](malachite_nz::natural::Natural) (when the [`Rational`] is non-negative and an
    /// integer), taking the [`Rational`] by reference.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::conversion::traits::ConvertibleFrom;
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Natural::convertible_from(&Rational::from(123)), true);
    /// assert_eq!(Natural::convertible_from(&Rational::from(-123)), false);
    /// assert_eq!(Natural::convertible_from(&Rational::from_signeds(22, 7)), false);
    /// ```
    #[inline]
    fn convertible_from(x: &Rational) -> bool {
        x.sign && x.denominator == 1u32
    }
}

impl RoundingFrom<Rational> for Natural {
    /// Converts a [`Rational`] to a [`Natural`](malachite_nz::natural::Natural), using a
    /// specified [`RoundingMode`](malachite_base::rounding_modes::RoundingMode) and taking the
    /// [`Rational`] by value.
    ///
    /// If the [`Rational`] is negative, then it will be rounded to zero when the
    /// [`RoundingMode`](malachite_base::rounding_modes::RoundingMode) is `Ceiling`, `Down`, or
    /// `Nearest`. Otherwise, this function will panic.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if the [`Rational`] is not an integer and `rm` is `Exact`, or if the [`Rational`] is
    /// less than zero and `rm` is not `Down`, `Ceiling`, or `Nearest`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::conversion::traits::RoundingFrom;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Natural::rounding_from(Rational::from(123), RoundingMode::Exact), 123);
    ///
    /// assert_eq!(Natural::rounding_from(Rational::from_signeds(22, 7), RoundingMode::Floor), 3);
    /// assert_eq!(Natural::rounding_from(Rational::from_signeds(22, 7), RoundingMode::Down), 3);
    /// assert_eq!(
    ///     Natural::rounding_from(Rational::from_signeds(22, 7), RoundingMode::Ceiling),
    ///     4
    /// );
    /// assert_eq!(Natural::rounding_from(Rational::from_signeds(22, 7), RoundingMode::Up), 4);
    /// assert_eq!(
    ///     Natural::rounding_from(Rational::from_signeds(22, 7), RoundingMode::Nearest),
    ///     3
    /// );
    ///
    /// assert_eq!(Natural::rounding_from(Rational::from(-123), RoundingMode::Down), 0);
    /// assert_eq!(Natural::rounding_from(Rational::from(-123), RoundingMode::Ceiling), 0);
    /// assert_eq!(Natural::rounding_from(Rational::from(-123), RoundingMode::Nearest), 0);
    /// ```
    fn rounding_from(x: Rational, rm: RoundingMode) -> Natural {
        if x.sign {
            x.numerator.div_round(x.denominator, rm)
        } else if rm == RoundingMode::Down
            || rm == RoundingMode::Ceiling
            || rm == RoundingMode::Nearest
        {
            Natural::ZERO
        } else {
            panic!(
                "Cannot round negative Rational to Natural using RoundingMode {}",
                rm
            );
        }
    }
}

impl<'a> RoundingFrom<&'a Rational> for Natural {
    /// Converts a [`Rational`] to a [`Natural`](malachite_nz::natural::Natural), using a
    /// specified [`RoundingMode`](malachite_base::rounding_modes::RoundingMode) and taking the
    /// [`Rational`] by reference.
    ///
    /// If the [`Rational`] is negative, then it will be rounded to zero when the
    /// [`RoundingMode`](malachite_base::rounding_modes::RoundingMode) is `Ceiling`, `Down`, or
    /// `Nearest`. Otherwise, this function will panic.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if the [`Rational`] is not an integer and `rm` is `Exact`, or if the [`Rational`] is
    /// less than zero and `rm` is not `Down`, `Ceiling`, or `Nearest`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::conversion::traits::RoundingFrom;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Natural::rounding_from(&Rational::from(123), RoundingMode::Exact), 123);
    ///
    /// assert_eq!(Natural::rounding_from(&Rational::from_signeds(22, 7), RoundingMode::Floor), 3);
    /// assert_eq!(Natural::rounding_from(&Rational::from_signeds(22, 7), RoundingMode::Down), 3);
    /// assert_eq!(
    ///     Natural::rounding_from(&Rational::from_signeds(22, 7), RoundingMode::Ceiling),
    ///     4
    /// );
    /// assert_eq!(Natural::rounding_from(&Rational::from_signeds(22, 7), RoundingMode::Up), 4);
    /// assert_eq!(
    ///     Natural::rounding_from(&Rational::from_signeds(22, 7), RoundingMode::Nearest),
    ///     3
    /// );
    ///
    /// assert_eq!(Natural::rounding_from(&Rational::from(-123), RoundingMode::Down), 0);
    /// assert_eq!(Natural::rounding_from(&Rational::from(-123), RoundingMode::Ceiling), 0);
    /// assert_eq!(Natural::rounding_from(&Rational::from(-123), RoundingMode::Nearest), 0);
    /// ```
    fn rounding_from(x: &Rational, rm: RoundingMode) -> Natural {
        if x.sign {
            (&x.numerator).div_round(&x.denominator, rm)
        } else if rm == RoundingMode::Down
            || rm == RoundingMode::Ceiling
            || rm == RoundingMode::Nearest
        {
            Natural::ZERO
        } else {
            panic!(
                "Cannot round negative Rational to Natural using RoundingMode {}",
                rm
            );
        }
    }
}
