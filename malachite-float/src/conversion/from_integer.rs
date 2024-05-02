// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use core::cmp::Ordering;
use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::rounding_modes::RoundingMode;
use malachite_nz::integer::Integer;

impl Float {
    #[doc(hidden)]
    pub fn from_integer_times_power_of_2(x: Integer, pow: i64) -> Float {
        let sign = x >= 0;
        let f = Float::from_natural_times_power_of_2(x.unsigned_abs(), pow);
        if sign {
            f
        } else {
            -f
        }
    }

    #[doc(hidden)]
    pub fn from_integer_times_power_of_2_prec_round(
        x: Integer,
        pow: i64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        let sign = x >= 0;
        let (f, o) = Float::from_natural_times_power_of_2_prec_round(
            x.unsigned_abs(),
            pow,
            prec,
            if sign { rm } else { -rm },
        );
        if sign {
            (f, o)
        } else {
            (-f, o.reverse())
        }
    }

    #[doc(hidden)]
    #[inline]
    pub fn from_integer_times_power_of_2_prec(
        x: Integer,
        pow: i64,
        prec: u64,
    ) -> (Float, Ordering) {
        Float::from_integer_times_power_of_2_prec_round(x, pow, prec, RoundingMode::Nearest)
    }

    /// Converts an [`Integer`] to a [`Float`], taking the [`Integer`] by value. If the [`Float`] is
    /// nonzero, it has the specified precision. If rounding is needed, the specified rounding mode
    /// is used. An [`Ordering`] is also returned, indicating whether the returned value is less
    /// than, equal to, or greater than the original value.
    ///
    /// If you're only using [`RoundingMode::Nearest`], try using [`Float::from_integer_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(n.significant_bits(), prec)`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_float::Float;
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering;
    ///
    /// let (x, o) = Float::from_integer_prec_round(Integer::ZERO, 10, RoundingMode::Exact);
    /// assert_eq!(x.to_string(), "0.0");
    /// assert_eq!(o, Ordering::Equal);
    ///
    /// let (x, o) = Float::from_integer_prec_round(
    ///     Integer::from(123),
    ///     20,
    ///     RoundingMode::Exact
    /// );
    /// assert_eq!(x.to_string(), "123.0");
    /// assert_eq!(x.get_prec(), Some(20));
    /// assert_eq!(o, Ordering::Equal);
    ///
    /// let (x, o) = Float::from_integer_prec_round(Integer::from(123), 4, RoundingMode::Floor);
    /// assert_eq!(x.to_string(), "1.2e2");
    /// assert_eq!(x.get_prec(), Some(4));
    /// assert_eq!(o, Ordering::Less);
    ///
    /// let (x, o) = Float::from_integer_prec_round(
    ///     Integer::from(123),
    ///     4,
    ///     RoundingMode::Ceiling
    /// );
    /// assert_eq!(x.to_string(), "1.3e2");
    /// assert_eq!(x.get_prec(), Some(4));
    /// assert_eq!(o, Ordering::Greater);
    ///
    /// let (x, o) = Float::from_integer_prec_round(
    ///     Integer::from(-123),
    ///     20,
    ///     RoundingMode::Exact
    /// );
    /// assert_eq!(x.to_string(), "-123.0");
    /// assert_eq!(x.get_prec(), Some(20));
    /// assert_eq!(o, Ordering::Equal);
    ///
    /// let (x, o) = Float::from_integer_prec_round(Integer::from(-123), 4, RoundingMode::Floor);
    /// assert_eq!(x.to_string(), "-1.3e2");
    /// assert_eq!(x.get_prec(), Some(4));
    /// assert_eq!(o, Ordering::Less);
    ///
    /// let (x, o) = Float::from_integer_prec_round(
    ///     Integer::from(-123),
    ///     4,
    ///     RoundingMode::Ceiling
    /// );
    /// assert_eq!(x.to_string(), "-1.2e2");
    /// assert_eq!(x.get_prec(), Some(4));
    /// assert_eq!(o, Ordering::Greater);
    /// ```
    #[inline]
    pub fn from_integer_prec_round(x: Integer, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
        Float::from_integer_times_power_of_2_prec_round(x, 0, prec, rm)
    }

    /// Converts an [`Integer`] to a [`Float`], taking the [`Integer`] by value. If the [`Float`] is
    /// nonzero, it has the specified precision. An [`Ordering`] is also returned, indicating
    /// whether the returned value is less than, equal to, or greater than the original value.
    ///
    /// If you want the [`Float`]'s precision to be equal to the [`Integer`]'s number of significant
    /// bits, try just using `Float::from` instead.
    ///
    /// Rounding may occur, in which case [`RoundingMode::Nearest`] is used by default. To specify a
    /// rounding mode as well as a precision, try [`Float::from_integer_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(n.significant_bits(), prec)`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_float::Float;
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering;
    ///
    /// let (x, o) = Float::from_integer_prec(Integer::ZERO, 10);
    /// assert_eq!(x.to_string(), "0.0");
    /// assert_eq!(o, Ordering::Equal);
    ///
    /// let (x, o) = Float::from_integer_prec(Integer::from(123), 20);
    /// assert_eq!(x.to_string(), "123.0");
    /// assert_eq!(x.get_prec(), Some(20));
    /// assert_eq!(o, Ordering::Equal);
    ///
    /// let (x, o) = Float::from_integer_prec(Integer::from(123), 4);
    /// assert_eq!(x.to_string(), "1.2e2");
    /// assert_eq!(x.get_prec(), Some(4));
    /// assert_eq!(o, Ordering::Less);
    ///
    /// let (x, o) = Float::from_integer_prec(Integer::from(-123), 20);
    /// assert_eq!(x.to_string(), "-123.0");
    /// assert_eq!(x.get_prec(), Some(20));
    /// assert_eq!(o, Ordering::Equal);
    ///
    /// let (x, o) = Float::from_integer_prec(Integer::from(-123), 4);
    /// assert_eq!(x.to_string(), "-1.2e2");
    /// assert_eq!(x.get_prec(), Some(4));
    /// assert_eq!(o, Ordering::Greater);
    /// ```
    #[inline]
    pub fn from_integer_prec(x: Integer, prec: u64) -> (Float, Ordering) {
        Float::from_integer_times_power_of_2_prec_round(x, 0, prec, RoundingMode::Nearest)
    }

    #[doc(hidden)]
    pub fn from_integer_times_power_of_2_ref(x: &Integer, pow: i64) -> Float {
        let f = Float::from_natural_times_power_of_2_ref(x.unsigned_abs_ref(), pow);
        if *x >= 0 {
            f
        } else {
            -f
        }
    }

    #[doc(hidden)]
    pub fn from_integer_times_power_of_2_prec_round_ref(
        x: &Integer,
        pow: i64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        let sign = *x >= 0;
        let (f, o) = Float::from_natural_times_power_of_2_prec_round_ref(
            x.unsigned_abs_ref(),
            pow,
            prec,
            if sign { rm } else { -rm },
        );
        if sign {
            (f, o)
        } else {
            (-f, o.reverse())
        }
    }

    #[doc(hidden)]
    #[inline]
    pub fn from_integer_times_power_of_2_prec_ref(
        x: &Integer,
        pow: i64,
        prec: u64,
    ) -> (Float, Ordering) {
        Float::from_integer_times_power_of_2_prec_round_ref(x, pow, prec, RoundingMode::Nearest)
    }

    /// Converts an [`Integer`] to a [`Float`], taking the [`Integer`] by reference. If the
    /// [`Float`] is nonzero, it has the specified precision. If rounding is needed, the specified
    /// rounding mode is used. An [`Ordering`] is also returned, indicating whether the returned
    /// value is less than, equal to, or greater than the original value.
    ///
    /// If you're only using [`RoundingMode::Nearest`], try using [`Float::from_integer_prec_ref`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(n.significant_bits(), prec)`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_float::Float;
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering;
    ///
    /// let (x, o) = Float::from_integer_prec_round_ref(&Integer::ZERO, 10, RoundingMode::Exact);
    /// assert_eq!(x.to_string(), "0.0");
    /// assert_eq!(o, Ordering::Equal);
    ///
    /// let (x, o) = Float::from_integer_prec_round_ref(
    ///     &Integer::from(123),
    ///     20,
    ///     RoundingMode::Exact
    /// );
    /// assert_eq!(x.to_string(), "123.0");
    /// assert_eq!(x.get_prec(), Some(20));
    /// assert_eq!(o, Ordering::Equal);
    ///
    /// let (x, o) = Float::from_integer_prec_round_ref(
    ///     &Integer::from(123),
    ///     4,
    ///     RoundingMode::Floor
    /// );
    /// assert_eq!(x.to_string(), "1.2e2");
    /// assert_eq!(x.get_prec(), Some(4));
    /// assert_eq!(o, Ordering::Less);
    ///
    /// let (x, o) = Float::from_integer_prec_round_ref(
    ///     &Integer::from(123),
    ///     4,
    ///     RoundingMode::Ceiling
    /// );
    /// assert_eq!(x.to_string(), "1.3e2");
    /// assert_eq!(x.get_prec(), Some(4));
    /// assert_eq!(o, Ordering::Greater);
    ///
    /// let (x, o) = Float::from_integer_prec_round_ref(
    ///     &Integer::from(-123),
    ///     20,
    ///     RoundingMode::Exact
    /// );
    /// assert_eq!(x.to_string(), "-123.0");
    /// assert_eq!(x.get_prec(), Some(20));
    /// assert_eq!(o, Ordering::Equal);
    ///
    /// let (x, o) = Float::from_integer_prec_round_ref(
    ///     &Integer::from(-123),
    ///     4,
    ///     RoundingMode::Floor
    /// );
    /// assert_eq!(x.to_string(), "-1.3e2");
    /// assert_eq!(x.get_prec(), Some(4));
    /// assert_eq!(o, Ordering::Less);
    ///
    /// let (x, o) = Float::from_integer_prec_round_ref(
    ///     &Integer::from(-123),
    ///     4,
    ///     RoundingMode::Ceiling
    /// );
    /// assert_eq!(x.to_string(), "-1.2e2");
    /// assert_eq!(x.get_prec(), Some(4));
    /// assert_eq!(o, Ordering::Greater);
    /// ```
    #[inline]
    pub fn from_integer_prec_round_ref(
        x: &Integer,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        Float::from_integer_times_power_of_2_prec_round_ref(x, 0, prec, rm)
    }

    /// Converts an [`Integer`] to a [`Float`], taking the [`Integer`] by reference. If the
    /// [`Float`] is nonzero, it has the specified precision. An [`Ordering`] is also returned,
    /// indicating whether the returned value is less than, equal to, or greater than the original
    /// value.
    ///
    /// If you want the [`Float`]'s precision to be equal to the [`Integer`]'s number of significant
    /// bits, try just using `Float::from` instead.
    ///
    /// Rounding may occur, in which case [`RoundingMode::Nearest`] is used by default. To specify a
    /// rounding mode as well as a precision, try [`Float::from_integer_prec_round_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(n.significant_bits(), prec)`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_float::Float;
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering;
    ///
    /// let (x, o) = Float::from_integer_prec_ref(&Integer::ZERO, 10);
    /// assert_eq!(x.to_string(), "0.0");
    /// assert_eq!(o, Ordering::Equal);
    ///
    /// let (x, o) = Float::from_integer_prec_ref(&Integer::from(123), 20);
    /// assert_eq!(x.to_string(), "123.0");
    /// assert_eq!(x.get_prec(), Some(20));
    /// assert_eq!(o, Ordering::Equal);
    ///
    /// let (x, o) = Float::from_integer_prec_ref(&Integer::from(123), 4);
    /// assert_eq!(x.to_string(), "1.2e2");
    /// assert_eq!(x.get_prec(), Some(4));
    /// assert_eq!(o, Ordering::Less);
    ///
    /// let (x, o) = Float::from_integer_prec_ref(&Integer::from(-123), 20);
    /// assert_eq!(x.to_string(), "-123.0");
    /// assert_eq!(x.get_prec(), Some(20));
    /// assert_eq!(o, Ordering::Equal);
    ///
    /// let (x, o) = Float::from_integer_prec_ref(&Integer::from(-123), 4);
    /// assert_eq!(x.to_string(), "-1.2e2");
    /// assert_eq!(x.get_prec(), Some(4));
    /// assert_eq!(o, Ordering::Greater);
    /// ```
    #[inline]
    pub fn from_integer_prec_ref(x: &Integer, prec: u64) -> (Float, Ordering) {
        Float::from_integer_times_power_of_2_prec_round_ref(x, 0, prec, RoundingMode::Nearest)
    }
}

impl From<Integer> for Float {
    /// Converts an [`Integer`] to a [`Float`], taking the [`Integer`] by value.
    ///
    /// If the [`Integer`] is nonzero, the precision of the [`Float`] is equal to the [`Integer`]'s
    /// number of significant bits. If you want to specify a different precision, try
    /// [`Float::from_integer_prec`]. This may require rounding, which uses
    /// [`RoundingMode::Nearest`] by default. To specify a rounding mode as well as a precision, try
    /// [`Float::from_integer_prec_round`].
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
    /// assert_eq!(Float::from(Integer::ZERO).to_string(), "0.0");
    /// assert_eq!(Float::from(Integer::from(123)).to_string(), "123.0");
    /// assert_eq!(Float::from(Integer::from(123)).get_prec(), Some(7));
    /// assert_eq!(Float::from(Integer::from(-123)).to_string(), "-123.0");
    /// ```
    #[inline]
    fn from(n: Integer) -> Float {
        Float::from_integer_times_power_of_2(n, 0)
    }
}

impl<'a> From<&'a Integer> for Float {
    /// Converts an [`Integer`] to a [`Float`], taking the [`Integer`] by reference.
    ///
    /// If the [`Integer`] is nonzero, the precision of the [`Float`] is equal to the [`Integer`]'s
    /// number of significant bits. If you want to specify a different precision, try
    /// [`Float::from_integer_prec_ref`]. This may require rounding, which uses
    /// [`RoundingMode::Nearest`] by default. To specify a rounding mode as well as a precision, try
    /// [`Float::from_integer_prec_round_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `n.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_float::Float;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Float::from(&Integer::ZERO).to_string(), "0.0");
    /// assert_eq!(Float::from(&Integer::from(123)).to_string(), "123.0");
    /// assert_eq!(Float::from(&Integer::from(123)).get_prec(), Some(7));
    /// assert_eq!(Float::from(&Integer::from(-123)).to_string(), "-123.0");
    /// ```
    #[inline]
    fn from(n: &'a Integer) -> Float {
        Float::from_integer_times_power_of_2_ref(n, 0)
    }
}
