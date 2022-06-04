use malachite_base::num::arithmetic::traits::DivRound;
use malachite_base::num::conversion::traits::{CheckedFrom, ConvertibleFrom, RoundingFrom};
use malachite_base::rounding_modes::RoundingMode;
use malachite_nz::integer::Integer;
use Rational;

impl CheckedFrom<Rational> for Integer {
    /// Converts a [`Rational`] to an [`Integer`](malachite_nz::integer::Integer), taking the
    /// [`Rational`] by value. If the [`Rational`] is not an integer, `None` is returned.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::conversion::traits::CheckedFrom;
    /// use malachite_nz::integer::Integer;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Integer::checked_from(Rational::from(123)).unwrap(), 123);
    /// assert_eq!(Integer::checked_from(Rational::from(-123)).unwrap(), -123);
    /// assert_eq!(Integer::checked_from(Rational::from_signeds(22, 7)), None);
    /// ```
    fn checked_from(x: Rational) -> Option<Integer> {
        if x.denominator == 1u32 {
            Some(Integer::from_sign_and_abs(x.sign, x.numerator))
        } else {
            None
        }
    }
}

impl<'a> CheckedFrom<&'a Rational> for Integer {
    /// Converts a [`Rational`] to an [`Integer`](malachite_nz::integer::Integer), taking the
    /// [`Rational`] by reference. If the [`Rational`] is not an integer, `None` is returned.
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
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::conversion::traits::CheckedFrom;
    /// use malachite_nz::integer::Integer;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Integer::checked_from(&Rational::from(123)).unwrap(), 123);
    /// assert_eq!(Integer::checked_from(&Rational::from(-123)).unwrap(), -123);
    /// assert_eq!(Integer::checked_from(&Rational::from_signeds(22, 7)), None);
    /// ```
    fn checked_from(x: &Rational) -> Option<Integer> {
        if x.denominator == 1u32 {
            Some(Integer::from_sign_and_abs_ref(x.sign, &x.numerator))
        } else {
            None
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
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
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
    /// [`Rational`] by value.
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
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::conversion::traits::RoundingFrom;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_nz::integer::Integer;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Integer::rounding_from(Rational::from(123), RoundingMode::Exact), 123);
    /// assert_eq!(Integer::rounding_from(Rational::from(-123), RoundingMode::Exact), -123);
    ///
    /// assert_eq!(Integer::rounding_from(Rational::from_signeds(22, 7), RoundingMode::Floor), 3);
    /// assert_eq!(Integer::rounding_from(Rational::from_signeds(22, 7), RoundingMode::Down), 3);
    /// assert_eq!(
    ///     Integer::rounding_from(Rational::from_signeds(22, 7), RoundingMode::Ceiling),
    ///     4
    /// );
    /// assert_eq!(Integer::rounding_from(Rational::from_signeds(22, 7), RoundingMode::Up), 4);
    /// assert_eq!(
    ///     Integer::rounding_from(Rational::from_signeds(22, 7), RoundingMode::Nearest),
    ///     3
    /// );
    ///
    /// assert_eq!(
    ///     Integer::rounding_from(Rational::from_signeds(-22, 7), RoundingMode::Floor),
    ///     -4
    /// );
    /// assert_eq!(Integer::rounding_from(Rational::from_signeds(-22, 7), RoundingMode::Down), -3);
    /// assert_eq!(
    ///     Integer::rounding_from(Rational::from_signeds(-22, 7), RoundingMode::Ceiling),
    ///     -3
    /// );
    /// assert_eq!(Integer::rounding_from(Rational::from_signeds(-22, 7), RoundingMode::Up), -4);
    /// assert_eq!(
    ///     Integer::rounding_from(Rational::from_signeds(-22, 7), RoundingMode::Nearest),
    ///     -3
    /// );
    /// ```
    fn rounding_from(x: Rational, rm: RoundingMode) -> Integer {
        Integer::from_sign_and_abs(
            x.sign,
            x.numerator
                .div_round(x.denominator, if x.sign { rm } else { -rm }),
        )
    }
}

impl<'a> RoundingFrom<&'a Rational> for Integer {
    /// Converts a [`Rational`] to an [`Integer`](malachite_nz::integer::Integer), using a
    /// specified [`RoundingMode`](malachite_base::rounding_modes::RoundingMode) and taking the
    /// [`Rational`] by reference.
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
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::conversion::traits::RoundingFrom;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_nz::integer::Integer;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Integer::rounding_from(&Rational::from(123), RoundingMode::Exact), 123);
    /// assert_eq!(Integer::rounding_from(&Rational::from(-123), RoundingMode::Exact), -123);
    ///
    /// assert_eq!(Integer::rounding_from(&Rational::from_signeds(22, 7), RoundingMode::Floor), 3);
    /// assert_eq!(Integer::rounding_from(&Rational::from_signeds(22, 7), RoundingMode::Down), 3);
    /// assert_eq!(
    ///     Integer::rounding_from(&Rational::from_signeds(22, 7), RoundingMode::Ceiling),
    ///     4
    /// );
    /// assert_eq!(Integer::rounding_from(&Rational::from_signeds(22, 7), RoundingMode::Up), 4);
    /// assert_eq!(
    ///     Integer::rounding_from(&Rational::from_signeds(22, 7), RoundingMode::Nearest),
    ///     3
    /// );
    ///
    /// assert_eq!(
    ///     Integer::rounding_from(&Rational::from_signeds(-22, 7), RoundingMode::Floor),
    ///     -4
    /// );
    /// assert_eq!(
    ///     Integer::rounding_from(&Rational::from_signeds(-22, 7), RoundingMode::Down),
    ///     -3
    /// );
    /// assert_eq!(
    ///     Integer::rounding_from(&Rational::from_signeds(-22, 7), RoundingMode::Ceiling),
    ///     -3
    /// );
    /// assert_eq!(Integer::rounding_from(&Rational::from_signeds(-22, 7), RoundingMode::Up), -4);
    /// assert_eq!(
    ///     Integer::rounding_from(&Rational::from_signeds(-22, 7), RoundingMode::Nearest),
    ///     -3
    /// );
    /// ```
    fn rounding_from(x: &Rational, rm: RoundingMode) -> Integer {
        Integer::from_sign_and_abs(
            x.sign,
            (&x.numerator).div_round(&x.denominator, if x.sign { rm } else { -rm }),
        )
    }
}
