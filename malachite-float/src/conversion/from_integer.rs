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
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;

impl Float {
    /// Converts an [`Integer`] to a [`Float`], taking the [`Integer`] by value. If the [`Float`] is
    /// nonzero, it has the specified precision. If rounding is needed, the specified rounding mode
    /// is used. An [`Ordering`] is also returned, indicating whether the returned value is less
    /// than, equal to, or greater than the original value.
    ///
    /// If you're only using [`Nearest`], try using [`Float::from_integer_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(m,n) = O(\max(m,n))$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $m$ is `n.significant_bits()`, and $n$ is
    /// `prec`.
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
    /// # Worst-case complexity
    /// $T(m,n) = O(\max(m,n))$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $m$ is `n.significant_bits()`, and $n$ is
    /// `prec`.
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
    /// [`Float`] is nonzero, it has the specified precision. If rounding is needed, the specified
    /// rounding mode is used. An [`Ordering`] is also returned, indicating whether the returned
    /// value is less than, equal to, or greater than the original value.
    ///
    /// If you're only using [`Nearest`], try using [`Float::from_integer_prec_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(m,n) = O(\max(m,n))$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $m$ is `n.significant_bits()`, and $n$ is
    /// `prec`.
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
    /// # Worst-case complexity
    /// $T(m,n) = O(\max(m,n))$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $m$ is `n.significant_bits()`, and $n$ is
    /// `prec`.
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

    /// Converts an [`Integer`] to a [`Float`], taking the [`Integer`] by value.
    ///
    /// If the [`Integer`] is nonzero, the precision of the [`Float`] is the minimum possible
    /// precision to represent the [`Integer`] exactly. If you instead want to use the precision
    /// equal to the [`Integer`]'s number of significant bits, try `from`. If you want to specify
    /// some other precision, try [`Float::from_integer_prec`]. This may require rounding, which
    /// uses [`Nearest`] by default. To specify a rounding mode as well as a precision, try
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
    /// assert_eq!(
    ///     Float::from_integer_min_prec(Integer::ZERO).to_string(),
    ///     "0.0"
    /// );
    /// assert_eq!(
    ///     Float::from_integer_min_prec(Integer::from(100)).to_string(),
    ///     "100.0"
    /// );
    /// assert_eq!(
    ///     Float::from_integer_min_prec(Integer::from(100)).get_prec(),
    ///     Some(5)
    /// );
    /// assert_eq!(
    ///     Float::from_integer_min_prec(Integer::from(-100)).to_string(),
    ///     "-100.0"
    /// );
    /// assert_eq!(
    ///     Float::from_integer_min_prec(Integer::from(-100)).get_prec(),
    ///     Some(5)
    /// );
    /// ```
    pub fn from_integer_min_prec(x: Integer) -> Float {
        let sign = x >= 0;
        let abs = Float::from_natural_min_prec(x.unsigned_abs());
        if sign {
            abs
        } else {
            -abs
        }
    }

    /// Converts an [`Integer`] to a [`Float`], taking the [`Integer`] by reference.
    ///
    /// If the [`Integer`] is nonzero, the precision of the [`Float`] is the minimum possible
    /// precision to represent the [`Integer`] exactly. If you instead want to use the precision
    /// equal to the [`Integer`]'s number of significant bits, try `from`. If you want to specify
    /// some other precision, try [`Float::from_integer_prec_ref`]. This may require rounding, which
    /// uses [`Nearest`] by default. To specify a rounding mode as well as a precision, try
    /// [`Float::from_integer_prec_round_ref`].
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
    /// assert_eq!(
    ///     Float::from_integer_min_prec_ref(&Integer::ZERO).to_string(),
    ///     "0.0"
    /// );
    /// assert_eq!(
    ///     Float::from_integer_min_prec_ref(&Integer::from(100)).to_string(),
    ///     "100.0"
    /// );
    /// assert_eq!(
    ///     Float::from_integer_min_prec_ref(&Integer::from(100)).get_prec(),
    ///     Some(5)
    /// );
    /// assert_eq!(
    ///     Float::from_integer_min_prec_ref(&Integer::from(-100)).to_string(),
    ///     "-100.0"
    /// );
    /// assert_eq!(
    ///     Float::from_integer_min_prec_ref(&Integer::from(-100)).get_prec(),
    ///     Some(5)
    /// );
    /// ```
    pub fn from_integer_min_prec_ref(x: &Integer) -> Float {
        let abs = Float::from_natural_min_prec_ref(x.unsigned_abs_ref());
        if *x >= 0 {
            abs
        } else {
            -abs
        }
    }
}

impl From<Integer> for Float {
    /// Converts an [`Integer`] to a [`Float`], taking the [`Integer`] by value.
    ///
    /// If the [`Integer`] is nonzero, the precision of the [`Float`] is equal to the [`Integer`]'s
    /// number of significant bits. If you instead want to use the minimum possible precision to
    /// represent the [`Integer`] exactly, try [`Float::from_integer_min_prec`]. If you want to
    /// specify some other precision, try [`Float::from_integer_prec`]. This may require rounding,
    /// which uses [`Nearest`] by default. To specify a rounding mode as well as a precision, try
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
        let sign = n >= 0;
        let f = Float::from(n.unsigned_abs());
        if sign {
            f
        } else {
            -f
        }
    }
}

impl<'a> From<&'a Integer> for Float {
    /// Converts an [`Integer`] to a [`Float`], taking the [`Integer`] by reference.
    ///
    /// If the [`Integer`] is nonzero, the precision of the [`Float`] is equal to the [`Integer`]'s
    /// number of significant bits. If you instead want to use the minimum possible precision to
    /// represent the [`Integer`] exactly, try [`Float::from_integer_min_prec_ref`]. If you want to
    /// specify some other precision, try [`Float::from_integer_prec_ref`]. This may require
    /// rounding, which uses [`Nearest`] by default. To specify a rounding mode as well as a
    /// precision, try [`Float::from_integer_prec_round_ref`].
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
        let f = Float::from(n.unsigned_abs_ref());
        if *n >= 0 {
            f
        } else {
            -f
        }
    }
}
