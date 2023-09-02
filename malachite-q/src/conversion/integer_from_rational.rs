use crate::Rational;
use malachite_base::num::arithmetic::traits::DivRound;
use malachite_base::num::conversion::traits::{ConvertibleFrom, RoundingFrom};
use malachite_base::rounding_modes::RoundingMode;
use malachite_nz::integer::Integer;
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IntegerFromRationalError;

impl TryFrom<Rational> for Integer {
    type Error = IntegerFromRationalError;

    /// Converts a [`Rational`] to an [`Integer`](malachite_nz::integer::Integer), taking the
    /// [`Rational`] by value. If the [`Rational`] is not an integer, an error is returned.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    /// use malachite_q::conversion::integer_from_rational::IntegerFromRationalError;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Integer::try_from(Rational::from(123)).unwrap(), 123);
    /// assert_eq!(Integer::try_from(Rational::from(-123)).unwrap(), -123);
    /// assert_eq!(
    ///     Integer::try_from(Rational::from_signeds(22, 7)),
    ///     Err(IntegerFromRationalError)
    /// );
    /// ```
    fn try_from(x: Rational) -> Result<Integer, Self::Error> {
        if x.denominator == 1u32 {
            Ok(Integer::from_sign_and_abs(x.sign, x.numerator))
        } else {
            Err(IntegerFromRationalError)
        }
    }
}

impl<'a> TryFrom<&'a Rational> for Integer {
    type Error = IntegerFromRationalError;

    /// Converts a [`Rational`] to an [`Integer`](malachite_nz::integer::Integer), taking the
    /// [`Rational`] by reference. If the [`Rational`] is not an integer, an error is returned.
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
    /// use malachite_nz::integer::Integer;
    /// use malachite_q::conversion::integer_from_rational::IntegerFromRationalError;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Integer::try_from(&Rational::from(123)).unwrap(), 123);
    /// assert_eq!(Integer::try_from(&Rational::from(-123)).unwrap(), -123);
    /// assert_eq!(
    ///     Integer::try_from(&Rational::from_signeds(22, 7)),
    ///     Err(IntegerFromRationalError)
    /// );
    /// ```
    fn try_from(x: &Rational) -> Result<Integer, Self::Error> {
        if x.denominator == 1u32 {
            Ok(Integer::from_sign_and_abs_ref(x.sign, &x.numerator))
        } else {
            Err(IntegerFromRationalError)
        }
    }
}

impl<'a> ConvertibleFrom<&'a Rational> for Integer {
    /// Determines whether a [`Rational`] can be converted to an
    /// [`Integer`](malachite_nz::integer::Integer), taking the [`Rational`] by reference.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::traits::ConvertibleFrom;
    /// use malachite_nz::integer::Integer;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Integer::convertible_from(&Rational::from(123)), true);
    /// assert_eq!(Integer::convertible_from(&Rational::from(-123)), true);
    /// assert_eq!(Integer::convertible_from(&Rational::from_signeds(22, 7)), false);
    /// ```
    #[inline]
    fn convertible_from(x: &Rational) -> bool {
        x.denominator == 1u32
    }
}

impl RoundingFrom<Rational> for Integer {
    /// Converts a [`Rational`] to an [`Integer`](malachite_nz::integer::Integer), using a
    /// specified [`RoundingMode`](malachite_base::rounding_modes::RoundingMode) and taking the
    /// [`Rational`] by value. An [`Ordering`] is also returned, indicating whether the returned
    /// value is less than, equal to, or greater than the original value.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if the [`Rational`] is not an integer and `rm` is `Exact`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::traits::RoundingFrom;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::integer::Integer;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Integer::rounding_from(Rational::from(123), RoundingMode::Exact).to_debug_string(),
    ///     "(123, Equal)"
    /// );
    /// assert_eq!(
    ///     Integer::rounding_from(Rational::from(-123), RoundingMode::Exact).to_debug_string(),
    ///     "(-123, Equal)"
    /// );
    ///
    /// assert_eq!(
    ///     Integer::rounding_from(Rational::from_signeds(22, 7), RoundingMode::Floor)
    ///         .to_debug_string(),
    ///     "(3, Less)"
    /// );
    /// assert_eq!(
    ///     Integer::rounding_from(Rational::from_signeds(22, 7), RoundingMode::Down)
    ///         .to_debug_string(),
    ///     "(3, Less)"
    /// );
    /// assert_eq!(
    ///     Integer::rounding_from(Rational::from_signeds(22, 7), RoundingMode::Ceiling)
    ///         .to_debug_string(),
    ///     "(4, Greater)"
    /// );
    /// assert_eq!(
    ///     Integer::rounding_from(Rational::from_signeds(22, 7), RoundingMode::Up)
    ///         .to_debug_string(),
    ///     "(4, Greater)"
    /// );
    /// assert_eq!(
    ///     Integer::rounding_from(Rational::from_signeds(22, 7), RoundingMode::Nearest)
    ///         .to_debug_string(),
    ///     "(3, Less)"
    /// );
    ///
    /// assert_eq!(
    ///     Integer::rounding_from(Rational::from_signeds(-22, 7), RoundingMode::Floor)
    ///         .to_debug_string(),
    ///     "(-4, Less)"
    /// );
    /// assert_eq!(
    ///     Integer::rounding_from(Rational::from_signeds(-22, 7), RoundingMode::Down)
    ///         .to_debug_string(),
    ///     "(-3, Greater)"
    /// );
    /// assert_eq!(
    ///     Integer::rounding_from(Rational::from_signeds(-22, 7), RoundingMode::Ceiling)
    ///         .to_debug_string(),
    ///     "(-3, Greater)"
    /// );
    /// assert_eq!(
    ///     Integer::rounding_from(Rational::from_signeds(-22, 7), RoundingMode::Up)
    ///         .to_debug_string(),
    ///     "(-4, Less)"
    /// );
    /// assert_eq!(
    ///     Integer::rounding_from(Rational::from_signeds(-22, 7), RoundingMode::Nearest)
    ///         .to_debug_string(),
    ///     "(-3, Greater)"
    /// );
    /// ```
    fn rounding_from(x: Rational, rm: RoundingMode) -> (Integer, Ordering) {
        let s = x.sign;
        let (n, o) = x
            .numerator
            .div_round(x.denominator, if s { rm } else { -rm });
        (
            Integer::from_sign_and_abs(x.sign, n),
            if s { o } else { o.reverse() },
        )
    }
}

impl<'a> RoundingFrom<&'a Rational> for Integer {
    /// Converts a [`Rational`] to an [`Integer`](malachite_nz::integer::Integer), using a
    /// specified [`RoundingMode`](malachite_base::rounding_modes::RoundingMode) and taking the
    /// [`Rational`] by reference. An [`Ordering`] is also returned, indicating whether the
    /// returned value is less than, equal to, or greater than the original value.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if the [`Rational`] is not an integer and `rm` is `Exact`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::traits::RoundingFrom;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::integer::Integer;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Integer::rounding_from(&Rational::from(123), RoundingMode::Exact).to_debug_string(),
    ///     "(123, Equal)"
    /// );
    /// assert_eq!(
    ///     Integer::rounding_from(&Rational::from(-123), RoundingMode::Exact).to_debug_string(),
    ///     "(-123, Equal)"
    /// );
    ///
    /// assert_eq!(
    ///     Integer::rounding_from(&Rational::from_signeds(22, 7), RoundingMode::Floor)
    ///         .to_debug_string(),
    ///     "(3, Less)"
    /// );
    /// assert_eq!(
    ///     Integer::rounding_from(&Rational::from_signeds(22, 7), RoundingMode::Down)
    ///         .to_debug_string(),
    ///     "(3, Less)"
    /// );
    /// assert_eq!(
    ///     Integer::rounding_from(&Rational::from_signeds(22, 7), RoundingMode::Ceiling)
    ///         .to_debug_string(),
    ///     "(4, Greater)"
    /// );
    /// assert_eq!(
    ///     Integer::rounding_from(&Rational::from_signeds(22, 7), RoundingMode::Up)
    ///         .to_debug_string(),
    ///     "(4, Greater)"
    /// );
    /// assert_eq!(
    ///     Integer::rounding_from(&Rational::from_signeds(22, 7), RoundingMode::Nearest)
    ///         .to_debug_string(),
    ///     "(3, Less)"
    /// );
    ///
    /// assert_eq!(
    ///     Integer::rounding_from(Rational::from_signeds(-22, 7), RoundingMode::Floor)
    ///         .to_debug_string(),
    ///     "(-4, Less)"
    /// );
    /// assert_eq!(
    ///     Integer::rounding_from(Rational::from_signeds(-22, 7), RoundingMode::Down)
    ///         .to_debug_string(),
    ///     "(-3, Greater)"
    /// );
    /// assert_eq!(
    ///     Integer::rounding_from(Rational::from_signeds(-22, 7), RoundingMode::Ceiling)
    ///         .to_debug_string(),
    ///     "(-3, Greater)"
    /// );
    /// assert_eq!(Integer::rounding_from(Rational::from_signeds(-22, 7), RoundingMode::Up)
    ///         .to_debug_string(),
    ///     "(-4, Less)"
    /// );
    /// assert_eq!(
    ///     Integer::rounding_from(Rational::from_signeds(-22, 7), RoundingMode::Nearest)
    ///         .to_debug_string(),
    ///     "(-3, Greater)"
    /// );
    /// ```
    fn rounding_from(x: &Rational, rm: RoundingMode) -> (Integer, Ordering) {
        let (n, o) = (&x.numerator).div_round(&x.denominator, if x.sign { rm } else { -rm });
        (
            Integer::from_sign_and_abs(x.sign, n),
            if x.sign { o } else { o.reverse() },
        )
    }
}
