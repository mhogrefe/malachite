use malachite_base::num::arithmetic::traits::DivRound;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::{CheckedFrom, ConvertibleFrom, RoundingFrom};
use malachite_base::rounding_modes::RoundingMode;
use malachite_nz::natural::Natural;
use Rational;

impl CheckedFrom<Rational> for Natural {
    /// Converts a `Rational` to a `Natural`, taking the `Rational` by value. If the `Rational` is
    /// negative or not an integer, `None` is returned.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    /// extern crate malachite_q;
    ///
    /// use malachite_base::num::conversion::traits::CheckedFrom;
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Natural::checked_from(Rational::from(123)).unwrap(), 123);
    /// assert_eq!(Natural::checked_from(Rational::from(-123)), None);
    /// assert_eq!(Natural::checked_from(Rational::from_str("22/7").unwrap()), None);
    /// ```
    fn checked_from(x: Rational) -> Option<Natural> {
        if x.sign && x.denominator == 1u32 {
            Some(x.numerator)
        } else {
            None
        }
    }
}

impl<'a> CheckedFrom<&'a Rational> for Natural {
    /// Converts a `Rational` to a `Natural`, taking the `Rational` by reference. If the `Rational`
    /// is negative or not an integer, `None` is returned.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    /// extern crate malachite_q;
    ///
    /// use malachite_base::num::conversion::traits::CheckedFrom;
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Natural::checked_from(&Rational::from(123)).unwrap(), 123);
    /// assert_eq!(Natural::checked_from(&Rational::from(-123)), None);
    /// assert_eq!(Natural::checked_from(&Rational::from_str("22/7").unwrap()), None);
    /// ```
    fn checked_from(x: &Rational) -> Option<Natural> {
        if x.sign && x.denominator == 1u32 {
            Some(x.numerator.clone())
        } else {
            None
        }
    }
}

impl<'a> ConvertibleFrom<&'a Rational> for Natural {
    /// Determines whether a `Rational` can be converted to a `Natural` (when the `Rational` is
    /// non-negative and an integer). Takes the `Rational` by reference.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    /// extern crate malachite_q;
    ///
    /// use malachite_base::num::conversion::traits::ConvertibleFrom;
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Natural::convertible_from(&Rational::from(123)), true);
    /// assert_eq!(Natural::convertible_from(&Rational::from(-123)), false);
    /// assert_eq!(Natural::convertible_from(&Rational::from_str("22/7").unwrap()), false);
    /// ```
    #[inline]
    fn convertible_from(x: &Rational) -> bool {
        x.sign && x.denominator == 1u32
    }
}

impl RoundingFrom<Rational> for Natural {
    /// Converts a `Rational` to a `Natural`, using a specified `RoundingMode` and taking the
    /// `Rational` by value.
    ///
    /// If the `Rational` is negative, then it will be rounded to zero when the `RoundingMode` is
    /// `Ceiling`, `Down`, or `Nearest`. Otherwise, this function will panic.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Panics
    /// Panics if the `Rational` is not an integer and `RoundingMode` is `Exact`, or if the
    /// `Rational` is less than zero and `RoundingMode` is not `Down`, `Ceiling`, or `Nearest`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    /// extern crate malachite_q;
    ///
    /// use malachite_base::num::conversion::traits::RoundingFrom;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Natural::rounding_from(Rational::from(123), RoundingMode::Exact), 123);
    ///
    /// assert_eq!(
    ///     Natural::rounding_from(Rational::from_str("22/7").unwrap(), RoundingMode::Floor),
    ///     3
    /// );
    /// assert_eq!(
    ///     Natural::rounding_from(Rational::from_str("22/7").unwrap(), RoundingMode::Down),
    ///     3
    /// );
    /// assert_eq!(
    ///     Natural::rounding_from(Rational::from_str("22/7").unwrap(), RoundingMode::Ceiling),
    ///     4
    /// );
    /// assert_eq!(
    ///     Natural::rounding_from(Rational::from_str("22/7").unwrap(), RoundingMode::Up),
    ///     4
    /// );
    /// assert_eq!(
    ///     Natural::rounding_from(Rational::from_str("22/7").unwrap(), RoundingMode::Nearest),
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
    /// Converts a `Rational` to a `Natural`, using a specified `RoundingMode` and taking the
    /// `Rational` by reference.
    ///
    /// If the `Rational` is negative, then it will be rounded to zero when the `RoundingMode` is
    /// `Ceiling`, `Down`, or `Nearest`. Otherwise, this function will panic.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Panics
    /// Panics if the `Rational` is not an integer and `RoundingMode` is `Exact`, or if the
    /// `Rational` is less than zero and `RoundingMode` is not `Down`, `Ceiling`, or `Nearest`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    /// extern crate malachite_q;
    ///
    /// use malachite_base::num::conversion::traits::RoundingFrom;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Natural::rounding_from(&Rational::from(123), RoundingMode::Exact), 123);
    ///
    /// assert_eq!(
    ///     Natural::rounding_from(&Rational::from_str("22/7").unwrap(), RoundingMode::Floor),
    ///     3
    /// );
    /// assert_eq!(
    ///     Natural::rounding_from(&Rational::from_str("22/7").unwrap(), RoundingMode::Down),
    ///     3
    /// );
    /// assert_eq!(
    ///     Natural::rounding_from(&Rational::from_str("22/7").unwrap(), RoundingMode::Ceiling),
    ///     4
    /// );
    /// assert_eq!(
    ///     Natural::rounding_from(&Rational::from_str("22/7").unwrap(), RoundingMode::Up),
    ///     4
    /// );
    /// assert_eq!(
    ///     Natural::rounding_from(&Rational::from_str("22/7").unwrap(), RoundingMode::Nearest),
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
