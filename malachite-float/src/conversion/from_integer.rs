// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::conversion::from_natural::{
    from_natural_prec_round_zero_exponent, from_natural_zero_exponent,
};
use crate::Float;
use core::cmp::Ordering;
use malachite_base::num::arithmetic::traits::{FloorLogBase2, UnsignedAbs};
use malachite_base::num::conversion::traits::{ConvertibleFrom, SaturatingFrom};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_q::conversion::primitive_float_from_rational::FloatConversionError;

pub(crate) fn from_integer_zero_exponent(n: Integer) -> Float {
    let sign = n >= 0;
    let f = from_natural_zero_exponent(n.unsigned_abs());
    if sign {
        f
    } else {
        -f
    }
}

pub(crate) fn from_integer_prec_round_zero_exponent(
    x: Integer,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    let sign = x >= 0;
    let (f, o) =
        from_natural_prec_round_zero_exponent(x.unsigned_abs(), prec, if sign { rm } else { -rm });
    if sign {
        (f, o)
    } else {
        (-f, o.reverse())
    }
}

impl Float {
    /// Converts an [`Integer`] to a [`Float`], taking the [`Integer`] by value. If the [`Float`] is
    /// nonzero, it has the specified precision. If rounding is needed, the specified rounding mode
    /// is used. An [`Ordering`] is also returned, indicating whether the returned value is less
    /// than, equal to, or greater than the original value.
    ///
    /// If you're only using [`Nearest`], try using [`Float::from_integer_prec`] instead.
    ///
    /// - If the [`Integer`] rounds to a value greater than or equal to $2^{2^{30}-1}$), this
    ///   function overflows to $\infty$ if `rm` is `Ceiling`, `Up`, or `Nearest`, and rounds down
    ///   to $(1-(1/2)^p)2^{2^{30}-1}$ otherwise, where $p$ is `prec`.
    /// - If the [`Integer`] rounds to a value less than or equal to $-2^{2^{30}-1}$), this function
    ///   overflows to $-\infty$ if `rm` is `Ceiling`, `Up`, or `Nearest`, and rounds up to
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ otherwise, where $p$ is `prec`.
    ///
    /// # Worst-case complexity
    /// $T(m,n) = O(\max(m,n))$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $m$ is `n.significant_bits()`, and $n$ is
    /// `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is exact and the `Integer` cannot be exactly
    /// represented with the specified precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = Float::from_integer_prec_round(Integer::ZERO, 10, Exact);
    /// assert_eq!(x.to_string(), "0.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (x, o) = Float::from_integer_prec_round(Integer::from(123), 20, Exact);
    /// assert_eq!(x.to_string(), "123.0");
    /// assert_eq!(x.get_prec(), Some(20));
    /// assert_eq!(o, Equal);
    ///
    /// let (x, o) = Float::from_integer_prec_round(Integer::from(123), 4, Floor);
    /// assert_eq!(x.to_string(), "1.2e2");
    /// assert_eq!(x.get_prec(), Some(4));
    /// assert_eq!(o, Less);
    ///
    /// let (x, o) = Float::from_integer_prec_round(Integer::from(123), 4, Ceiling);
    /// assert_eq!(x.to_string(), "1.3e2");
    /// assert_eq!(x.get_prec(), Some(4));
    /// assert_eq!(o, Greater);
    ///
    /// let (x, o) = Float::from_integer_prec_round(Integer::from(-123), 20, Exact);
    /// assert_eq!(x.to_string(), "-123.0");
    /// assert_eq!(x.get_prec(), Some(20));
    /// assert_eq!(o, Equal);
    ///
    /// let (x, o) = Float::from_integer_prec_round(Integer::from(-123), 4, Floor);
    /// assert_eq!(x.to_string(), "-1.3e2");
    /// assert_eq!(x.get_prec(), Some(4));
    /// assert_eq!(o, Less);
    ///
    /// let (x, o) = Float::from_integer_prec_round(Integer::from(-123), 4, Ceiling);
    /// assert_eq!(x.to_string(), "-1.2e2");
    /// assert_eq!(x.get_prec(), Some(4));
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn from_integer_prec_round(x: Integer, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
        let sign = x >= 0;
        let (f, o) =
            Float::from_natural_prec_round(x.unsigned_abs(), prec, if sign { rm } else { -rm });
        if sign {
            (f, o)
        } else {
            (-f, o.reverse())
        }
    }

    /// Converts an [`Integer`] to a [`Float`], taking the [`Integer`] by reference. If the
    /// [`Float`] is nonzero, it has the specified precision. If rounding is needed, the specified
    /// rounding mode is used. An [`Ordering`] is also returned, indicating whether the returned
    /// value is less than, equal to, or greater than the original value.
    ///
    /// If you're only using [`Nearest`], try using [`Float::from_integer_prec_ref`] instead.
    ///
    /// - If the [`Integer`] rounds to a value greater than or equal to $2^{2^{30}-1}$), this
    ///   function overflows to $\infty$ if `rm` is `Ceiling`, `Up`, or `Nearest`, and rounds down
    ///   to $(1-(1/2)^p)2^{2^{30}-1}$ otherwise, where $p$ is `prec`.
    /// - If the [`Integer`] rounds to a value less than or equal to $-2^{2^{30}-1}$), this function
    ///   overflows to $-\infty$ if `rm` is `Ceiling`, `Up`, or `Nearest`, and rounds up to
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ otherwise, where $p$ is `prec`.
    ///
    /// # Worst-case complexity
    /// $T(m,n) = O(\max(m,n))$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $m$ is `n.significant_bits()`, and $n$ is
    /// `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is exact and the `Integer` cannot be exactly
    /// represented with the specified precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = Float::from_integer_prec_round_ref(&Integer::ZERO, 10, Exact);
    /// assert_eq!(x.to_string(), "0.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (x, o) = Float::from_integer_prec_round_ref(&Integer::from(123), 20, Exact);
    /// assert_eq!(x.to_string(), "123.0");
    /// assert_eq!(x.get_prec(), Some(20));
    /// assert_eq!(o, Equal);
    ///
    /// let (x, o) = Float::from_integer_prec_round_ref(&Integer::from(123), 4, Floor);
    /// assert_eq!(x.to_string(), "1.2e2");
    /// assert_eq!(x.get_prec(), Some(4));
    /// assert_eq!(o, Less);
    ///
    /// let (x, o) = Float::from_integer_prec_round_ref(&Integer::from(123), 4, Ceiling);
    /// assert_eq!(x.to_string(), "1.3e2");
    /// assert_eq!(x.get_prec(), Some(4));
    /// assert_eq!(o, Greater);
    ///
    /// let (x, o) = Float::from_integer_prec_round_ref(&Integer::from(-123), 20, Exact);
    /// assert_eq!(x.to_string(), "-123.0");
    /// assert_eq!(x.get_prec(), Some(20));
    /// assert_eq!(o, Equal);
    ///
    /// let (x, o) = Float::from_integer_prec_round_ref(&Integer::from(-123), 4, Floor);
    /// assert_eq!(x.to_string(), "-1.3e2");
    /// assert_eq!(x.get_prec(), Some(4));
    /// assert_eq!(o, Less);
    ///
    /// let (x, o) = Float::from_integer_prec_round_ref(&Integer::from(-123), 4, Ceiling);
    /// assert_eq!(x.to_string(), "-1.2e2");
    /// assert_eq!(x.get_prec(), Some(4));
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn from_integer_prec_round_ref(
        x: &Integer,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        let sign = *x >= 0;
        let (f, o) = Float::from_natural_prec_round_ref(
            x.unsigned_abs_ref(),
            prec,
            if sign { rm } else { -rm },
        );
        if sign {
            (f, o)
        } else {
            (-f, o.reverse())
        }
    }

    /// Converts an [`Integer`] to a [`Float`], taking the [`Integer`] by value. If the [`Float`] is
    /// nonzero, it has the specified precision. An [`Ordering`] is also returned, indicating
    /// whether the returned value is less than, equal to, or greater than the original value.
    ///
    /// If you want the [`Float`]'s precision to be equal to the [`Integer`]'s number of significant
    /// bits, try just using `Float::from` instead.
    ///
    /// Rounding may occur, in which case [`Nearest`] is used by default. To specify a rounding mode
    /// as well as a precision, try [`Float::from_integer_prec_round`].
    ///
    /// - If the [`Integer`] rounds to a value greater than or equal to $2^{2^{30}-1}$), this
    ///   function overflows to $\infty$.
    /// - If the [`Integer`] rounds to a value less than or equal to -$2^{2^{30}-1}$), this function
    ///   overflows to $\infty$.
    ///
    /// # Worst-case complexity
    /// $T(m,n) = O(\max(m,n))$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $m$ is `n.significant_bits()`, and $n$ is
    /// `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_float::Float;
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = Float::from_integer_prec(Integer::ZERO, 10);
    /// assert_eq!(x.to_string(), "0.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (x, o) = Float::from_integer_prec(Integer::from(123), 20);
    /// assert_eq!(x.to_string(), "123.0");
    /// assert_eq!(x.get_prec(), Some(20));
    /// assert_eq!(o, Equal);
    ///
    /// let (x, o) = Float::from_integer_prec(Integer::from(123), 4);
    /// assert_eq!(x.to_string(), "1.2e2");
    /// assert_eq!(x.get_prec(), Some(4));
    /// assert_eq!(o, Less);
    ///
    /// let (x, o) = Float::from_integer_prec(Integer::from(-123), 20);
    /// assert_eq!(x.to_string(), "-123.0");
    /// assert_eq!(x.get_prec(), Some(20));
    /// assert_eq!(o, Equal);
    ///
    /// let (x, o) = Float::from_integer_prec(Integer::from(-123), 4);
    /// assert_eq!(x.to_string(), "-1.2e2");
    /// assert_eq!(x.get_prec(), Some(4));
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn from_integer_prec(x: Integer, prec: u64) -> (Float, Ordering) {
        Float::from_integer_prec_round(x, prec, Nearest)
    }

    /// Converts an [`Integer`] to a [`Float`], taking the [`Integer`] by reference. If the
    /// [`Float`] is nonzero, it has the specified precision. An [`Ordering`] is also returned,
    /// indicating whether the returned value is less than, equal to, or greater than the original
    /// value.
    ///
    /// If you want the [`Float`]'s precision to be equal to the [`Integer`]'s number of significant
    /// bits, try just using `Float::from` instead.
    ///
    /// Rounding may occur, in which case [`Nearest`] is used by default. To specify a rounding mode
    /// as well as a precision, try [`Float::from_integer_prec_round_ref`].
    ///
    /// - If the [`Integer`] rounds to a value greater than or equal to $2^{2^{30}-1}$), this
    ///   function overflows to $\infty$.
    /// - If the [`Integer`] rounds to a value less than or equal to -$2^{2^{30}-1}$), this function
    ///   overflows to $\infty$.
    ///
    /// # Worst-case complexity
    /// $T(m,n) = O(\max(m,n))$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $m$ is `n.significant_bits()`, and $n$ is
    /// `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_float::Float;
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = Float::from_integer_prec_ref(&Integer::ZERO, 10);
    /// assert_eq!(x.to_string(), "0.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (x, o) = Float::from_integer_prec_ref(&Integer::from(123), 20);
    /// assert_eq!(x.to_string(), "123.0");
    /// assert_eq!(x.get_prec(), Some(20));
    /// assert_eq!(o, Equal);
    ///
    /// let (x, o) = Float::from_integer_prec_ref(&Integer::from(123), 4);
    /// assert_eq!(x.to_string(), "1.2e2");
    /// assert_eq!(x.get_prec(), Some(4));
    /// assert_eq!(o, Less);
    ///
    /// let (x, o) = Float::from_integer_prec_ref(&Integer::from(-123), 20);
    /// assert_eq!(x.to_string(), "-123.0");
    /// assert_eq!(x.get_prec(), Some(20));
    /// assert_eq!(o, Equal);
    ///
    /// let (x, o) = Float::from_integer_prec_ref(&Integer::from(-123), 4);
    /// assert_eq!(x.to_string(), "-1.2e2");
    /// assert_eq!(x.get_prec(), Some(4));
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn from_integer_prec_ref(x: &Integer, prec: u64) -> (Float, Ordering) {
        let sign = *x >= 0;
        let (f, o) = Float::from_natural_prec_ref(x.unsigned_abs_ref(), prec);
        if sign {
            (f, o)
        } else {
            (-f, o.reverse())
        }
    }
}

impl TryFrom<Integer> for Float {
    type Error = FloatConversionError;

    /// Converts an [`Integer`] to a [`Float`], taking the [`Integer`] by value.
    ///
    /// If the [`Integer`] is nonzero, the precision of the [`Float`] is the minimum possible
    /// precision to represent the [`Integer`] exactly. If you want to specify some other precision,
    /// try [`Float::from_integer_prec`]. This may require rounding, which uses [`Nearest`] by
    /// default. To specify a rounding mode as well as a precision, try
    /// [`Float::from_integer_prec_round`].
    ///
    /// If the absolue value of the [`Integer`] is greater than or equal to $2^{2^{30}-1}$, this
    /// function returns an overflow error.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `n.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_float::Float;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Float::try_from(Integer::ZERO).unwrap().to_string(), "0.0");
    /// assert_eq!(
    ///     Float::try_from(Integer::from(123)).unwrap().to_string(),
    ///     "123.0"
    /// );
    /// assert_eq!(
    ///     Float::try_from(Integer::from(123)).unwrap().get_prec(),
    ///     Some(7)
    /// );
    /// assert_eq!(
    ///     Float::try_from(Integer::from(10)).unwrap().to_string(),
    ///     "10.0"
    /// );
    /// assert_eq!(
    ///     Float::try_from(Integer::from(10)).unwrap().get_prec(),
    ///     Some(3)
    /// );
    /// assert_eq!(
    ///     Float::try_from(Integer::from(-123)).unwrap().to_string(),
    ///     "-123.0"
    /// );
    /// assert_eq!(
    ///     Float::try_from(Integer::from(-123)).unwrap().get_prec(),
    ///     Some(7)
    /// );
    /// assert_eq!(
    ///     Float::try_from(Integer::from(-10)).unwrap().to_string(),
    ///     "-10.0"
    /// );
    /// assert_eq!(
    ///     Float::try_from(Integer::from(-10)).unwrap().get_prec(),
    ///     Some(3)
    /// );
    /// ```
    #[inline]
    fn try_from(n: Integer) -> Result<Float, Self::Error> {
        let sign = n >= 0;
        let abs = Float::try_from(n.unsigned_abs())?;
        Ok(if sign { abs } else { -abs })
    }
}

impl TryFrom<&Integer> for Float {
    type Error = FloatConversionError;

    /// Converts an [`Integer`] to a [`Float`], taking the [`Integer`] by reference.
    ///
    /// If the [`Integer`] is nonzero, the precision of the [`Float`] is the minimum possible
    /// precision to represent the [`Integer`] exactly. If you want to specify some other precision,
    /// try [`Float::from_integer_prec`]. This may require rounding, which uses [`Nearest`] by
    /// default. To specify a rounding mode as well as a precision, try
    /// [`Float::from_integer_prec_round`].
    ///
    /// If the absolue value of the [`Integer`] is greater than or equal to $2^{2^{30}-1}$, this
    /// function returns an overflow error.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `n.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_float::Float;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Float::try_from(&Integer::ZERO).unwrap().to_string(), "0.0");
    /// assert_eq!(
    ///     Float::try_from(&Integer::from(123)).unwrap().to_string(),
    ///     "123.0"
    /// );
    /// assert_eq!(
    ///     Float::try_from(&Integer::from(123)).unwrap().get_prec(),
    ///     Some(7)
    /// );
    /// assert_eq!(
    ///     Float::try_from(&Integer::from(10)).unwrap().to_string(),
    ///     "10.0"
    /// );
    /// assert_eq!(
    ///     Float::try_from(&Integer::from(10)).unwrap().get_prec(),
    ///     Some(3)
    /// );
    /// assert_eq!(
    ///     Float::try_from(&Integer::from(-123)).unwrap().to_string(),
    ///     "-123.0"
    /// );
    /// assert_eq!(
    ///     Float::try_from(&Integer::from(-123)).unwrap().get_prec(),
    ///     Some(7)
    /// );
    /// assert_eq!(
    ///     Float::try_from(&Integer::from(-10)).unwrap().to_string(),
    ///     "-10.0"
    /// );
    /// assert_eq!(
    ///     Float::try_from(&Integer::from(-10)).unwrap().get_prec(),
    ///     Some(3)
    /// );
    /// ```
    #[inline]
    fn try_from(n: &Integer) -> Result<Float, Self::Error> {
        let sign = *n >= 0;
        let abs = Float::try_from(n.unsigned_abs())?;
        Ok(if sign { abs } else { -abs })
    }
}

impl ConvertibleFrom<&Integer> for Float {
    /// Determines whether an [`Integer`] can be converted to an [`Float`], taking the [`Integer`]
    /// by reference.
    ///
    /// The [`Integer`]s that are convertible to [`Float`]s are those whose that would not overflow:
    /// that is, those whose absolute values are less than $2^{2^{30}-1}$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::num::conversion::traits::ConvertibleFrom;
    /// use malachite_float::Float;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Float::convertible_from(&Integer::ZERO), true);
    /// assert_eq!(Float::convertible_from(&Integer::from(3u8)), true);
    /// ```
    #[inline]
    fn convertible_from(x: &Integer) -> bool {
        *x == 0
            || (Float::MIN_EXPONENT..=Float::MAX_EXPONENT).contains(
                &i32::saturating_from(x.unsigned_abs_ref().floor_log_base_2()).saturating_add(1),
            )
    }
}
