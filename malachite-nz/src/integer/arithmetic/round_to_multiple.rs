// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use core::cmp::Ordering;
use malachite_base::num::arithmetic::traits::{RoundToMultiple, RoundToMultipleAssign};
use malachite_base::rounding_modes::RoundingMode;

impl RoundToMultiple<Integer> for Integer {
    type Output = Integer;

    /// Rounds an [`Integer`] to a multiple of another [`Integer`], according to a specified
    /// rounding mode. Both [`Integer`]s are taken by value. An [`Ordering`] is also returned,
    /// indicating whether the returned value is less than, equal to, or greater than the original
    /// value.
    ///
    /// Let $q = \frac{x}{|y|}$:
    ///
    /// $f(x, y, \mathrm{Down}) =  \operatorname{sgn}(q) |y| \lfloor |q| \rfloor.$
    ///
    /// $f(x, y, \mathrm{Up}) = \operatorname{sgn}(q) |y| \lceil |q| \rceil.$
    ///
    /// $f(x, y, \mathrm{Floor}) = |y| \lfloor q \rfloor.$
    ///
    /// $f(x, y, \mathrm{Ceiling}) = |y| \lceil q \rceil.$
    ///
    /// $$
    /// f(x, y, \mathrm{Nearest}) = \begin{cases}
    ///     y \lfloor q \rfloor & \text{if} \\quad
    ///         q - \lfloor q \rfloor < \frac{1}{2} \\\\
    ///     y \lceil q \rceil & \text{if} \\quad q - \lfloor q \rfloor > \frac{1}{2} \\\\
    ///     y \lfloor q \rfloor &
    ///     \text{if} \\quad q - \lfloor q \rfloor =
    ///         \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor
    ///     \\ \text{is even} \\\\
    ///     y \lceil q \rceil &
    ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2}
    ///         \\ \text{and} \\ \lfloor q \rfloor \\ \text{is odd.}
    /// \end{cases}
    /// $$
    ///
    /// $f(x, y, \mathrm{Exact}) = q$, but panics if $q \notin \Z$.
    ///
    /// The following two expressions are equivalent:
    /// - `x.round_to_multiple(other, Exact)`
    /// - `{ assert!(x.divisible_by(other)); x }`
    ///
    /// but the latter should be used as it is clearer and more efficient.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// - If `rm` is `Exact`, but `self` is not a multiple of `other`.
    /// - If `self` is nonzero, `other` is zero, and `rm` is trying to round away from zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::RoundToMultiple;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     Integer::from(-5)
    ///         .round_to_multiple(Integer::ZERO, Down)
    ///         .to_debug_string(),
    ///     "(0, Greater)"
    /// );
    ///
    /// assert_eq!(
    ///     Integer::from(-10)
    ///         .round_to_multiple(Integer::from(4), Down)
    ///         .to_debug_string(),
    ///     "(-8, Greater)"
    /// );
    /// assert_eq!(
    ///     Integer::from(-10)
    ///         .round_to_multiple(Integer::from(4), Up)
    ///         .to_debug_string(),
    ///     "(-12, Less)"
    /// );
    /// assert_eq!(
    ///     Integer::from(-10)
    ///         .round_to_multiple(Integer::from(5), Exact)
    ///         .to_debug_string(),
    ///     "(-10, Equal)"
    /// );
    /// assert_eq!(
    ///     Integer::from(-10)
    ///         .round_to_multiple(Integer::from(3), Nearest)
    ///         .to_debug_string(),
    ///     "(-9, Greater)"
    /// );
    /// assert_eq!(
    ///     Integer::from(-20)
    ///         .round_to_multiple(Integer::from(3), Nearest)
    ///         .to_debug_string(),
    ///     "(-21, Less)"
    /// );
    /// assert_eq!(
    ///     Integer::from(-10)
    ///         .round_to_multiple(Integer::from(4), Nearest)
    ///         .to_debug_string(),
    ///     "(-8, Greater)"
    /// );
    /// assert_eq!(
    ///     Integer::from(-14)
    ///         .round_to_multiple(Integer::from(4), Nearest)
    ///         .to_debug_string(),
    ///     "(-16, Less)"
    /// );
    ///
    /// assert_eq!(
    ///     Integer::from(-10)
    ///         .round_to_multiple(Integer::from(-4), Down)
    ///         .to_debug_string(),
    ///     "(-8, Greater)"
    /// );
    /// assert_eq!(
    ///     Integer::from(-10)
    ///         .round_to_multiple(Integer::from(-4), Up)
    ///         .to_debug_string(),
    ///     "(-12, Less)"
    /// );
    /// assert_eq!(
    ///     Integer::from(-10)
    ///         .round_to_multiple(Integer::from(-5), Exact)
    ///         .to_debug_string(),
    ///     "(-10, Equal)"
    /// );
    /// assert_eq!(
    ///     Integer::from(-10)
    ///         .round_to_multiple(Integer::from(-3), Nearest)
    ///         .to_debug_string(),
    ///     "(-9, Greater)"
    /// );
    /// assert_eq!(
    ///     Integer::from(-20)
    ///         .round_to_multiple(Integer::from(-3), Nearest)
    ///         .to_debug_string(),
    ///     "(-21, Less)"
    /// );
    /// assert_eq!(
    ///     Integer::from(-10)
    ///         .round_to_multiple(Integer::from(-4), Nearest)
    ///         .to_debug_string(),
    ///     "(-8, Greater)"
    /// );
    /// assert_eq!(
    ///     Integer::from(-14)
    ///         .round_to_multiple(Integer::from(-4), Nearest)
    ///         .to_debug_string(),
    ///     "(-16, Less)"
    /// );
    /// ```
    #[inline]
    fn round_to_multiple(mut self, other: Integer, rm: RoundingMode) -> (Integer, Ordering) {
        let o = self.round_to_multiple_assign(other, rm);
        (self, o)
    }
}

impl RoundToMultiple<&Integer> for Integer {
    type Output = Integer;

    /// Rounds an [`Integer`] to a multiple of another [`Integer`], according to a specified
    /// rounding mode. The first [`Integer`] is taken by value and the second by reference. An
    /// [`Ordering`] is also returned, indicating whether the returned value is less than, equal to,
    /// or greater than the original value.
    ///
    /// Let $q = \frac{x}{|y|}$:
    ///
    /// $f(x, y, \mathrm{Down}) =  \operatorname{sgn}(q) |y| \lfloor |q| \rfloor.$
    ///
    /// $f(x, y, \mathrm{Up}) = \operatorname{sgn}(q) |y| \lceil |q| \rceil.$
    ///
    /// $f(x, y, \mathrm{Floor}) = |y| \lfloor q \rfloor.$
    ///
    /// $f(x, y, \mathrm{Ceiling}) = |y| \lceil q \rceil.$
    ///
    /// $$
    /// f(x, y, \mathrm{Nearest}) = \begin{cases}
    ///     y \lfloor q \rfloor & \text{if} \\quad
    ///         q - \lfloor q \rfloor < \frac{1}{2} \\\\
    ///     y \lceil q \rceil & \text{if} \\quad q - \lfloor q \rfloor > \frac{1}{2} \\\\
    ///     y \lfloor q \rfloor &
    ///     \text{if} \\quad q - \lfloor q \rfloor =
    ///         \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor
    ///     \\ \text{is even} \\\\
    ///     y \lceil q \rceil &
    ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2}
    ///         \\ \text{and} \\ \lfloor q \rfloor \\ \text{is odd.}
    /// \end{cases}
    /// $$
    ///
    /// $f(x, y, \mathrm{Exact}) = q$, but panics if $q \notin \Z$.
    ///
    /// The following two expressions are equivalent:
    /// - `x.round_to_multiple(other, Exact)`
    /// - `{ assert!(x.divisible_by(other)); x }`
    ///
    /// but the latter should be used as it is clearer and more efficient.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// - If `rm` is `Exact`, but `self` is not a multiple of `other`.
    /// - If `self` is nonzero, `other` is zero, and `rm` is trying to round away from zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::RoundToMultiple;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     Integer::from(-5)
    ///         .round_to_multiple(&Integer::ZERO, Down)
    ///         .to_debug_string(),
    ///     "(0, Greater)"
    /// );
    ///
    /// assert_eq!(
    ///     Integer::from(-10)
    ///         .round_to_multiple(&Integer::from(4), Down)
    ///         .to_debug_string(),
    ///     "(-8, Greater)"
    /// );
    /// assert_eq!(
    ///     Integer::from(-10)
    ///         .round_to_multiple(&Integer::from(4), Up)
    ///         .to_debug_string(),
    ///     "(-12, Less)"
    /// );
    /// assert_eq!(
    ///     Integer::from(-10)
    ///         .round_to_multiple(&Integer::from(5), Exact)
    ///         .to_debug_string(),
    ///     "(-10, Equal)"
    /// );
    /// assert_eq!(
    ///     Integer::from(-10)
    ///         .round_to_multiple(&Integer::from(3), Nearest)
    ///         .to_debug_string(),
    ///     "(-9, Greater)"
    /// );
    /// assert_eq!(
    ///     Integer::from(-20)
    ///         .round_to_multiple(&Integer::from(3), Nearest)
    ///         .to_debug_string(),
    ///     "(-21, Less)"
    /// );
    /// assert_eq!(
    ///     Integer::from(-10)
    ///         .round_to_multiple(&Integer::from(4), Nearest)
    ///         .to_debug_string(),
    ///     "(-8, Greater)"
    /// );
    /// assert_eq!(
    ///     Integer::from(-14)
    ///         .round_to_multiple(&Integer::from(4), Nearest)
    ///         .to_debug_string(),
    ///     "(-16, Less)"
    /// );
    ///
    /// assert_eq!(
    ///     Integer::from(-10)
    ///         .round_to_multiple(&Integer::from(-4), Down)
    ///         .to_debug_string(),
    ///     "(-8, Greater)"
    /// );
    /// assert_eq!(
    ///     Integer::from(-10)
    ///         .round_to_multiple(&Integer::from(-4), Up)
    ///         .to_debug_string(),
    ///     "(-12, Less)"
    /// );
    /// assert_eq!(
    ///     Integer::from(-10)
    ///         .round_to_multiple(&Integer::from(-5), Exact)
    ///         .to_debug_string(),
    ///     "(-10, Equal)"
    /// );
    /// assert_eq!(
    ///     Integer::from(-10)
    ///         .round_to_multiple(&Integer::from(-3), Nearest)
    ///         .to_debug_string(),
    ///     "(-9, Greater)"
    /// );
    /// assert_eq!(
    ///     Integer::from(-20)
    ///         .round_to_multiple(&Integer::from(-3), Nearest)
    ///         .to_debug_string(),
    ///     "(-21, Less)"
    /// );
    /// assert_eq!(
    ///     Integer::from(-10)
    ///         .round_to_multiple(&Integer::from(-4), Nearest)
    ///         .to_debug_string(),
    ///     "(-8, Greater)"
    /// );
    /// assert_eq!(
    ///     Integer::from(-14)
    ///         .round_to_multiple(&Integer::from(-4), Nearest)
    ///         .to_debug_string(),
    ///     "(-16, Less)"
    /// );
    /// ```
    #[inline]
    fn round_to_multiple(mut self, other: &Integer, rm: RoundingMode) -> (Integer, Ordering) {
        let o = self.round_to_multiple_assign(other, rm);
        (self, o)
    }
}

impl RoundToMultiple<Integer> for &Integer {
    type Output = Integer;

    /// Rounds an [`Integer`] to a multiple of another [`Integer`], according to a specified
    /// rounding mode. The first [`Integer`] is taken by reference and the second by value. An
    /// [`Ordering`] is also returned, indicating whether the returned value is less than, equal to,
    /// or greater than the original value.
    ///
    /// Let $q = \frac{x}{|y|}$:
    ///
    /// $f(x, y, \mathrm{Down}) =  \operatorname{sgn}(q) |y| \lfloor |q| \rfloor.$
    ///
    /// $f(x, y, \mathrm{Up}) = \operatorname{sgn}(q) |y| \lceil |q| \rceil.$
    ///
    /// $f(x, y, \mathrm{Floor}) = |y| \lfloor q \rfloor.$
    ///
    /// $f(x, y, \mathrm{Ceiling}) = |y| \lceil q \rceil.$
    ///
    /// $$
    /// f(x, y, \mathrm{Nearest}) = \begin{cases}
    ///     y \lfloor q \rfloor & \text{if} \\quad
    ///         q - \lfloor q \rfloor < \frac{1}{2} \\\\
    ///     y \lceil q \rceil & \text{if} \\quad q - \lfloor q \rfloor > \frac{1}{2} \\\\
    ///     y \lfloor q \rfloor &
    ///     \text{if} \\quad q - \lfloor q \rfloor =
    ///         \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor
    ///     \\ \text{is even} \\\\
    ///     y \lceil q \rceil &
    ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2}
    ///         \\ \text{and} \\ \lfloor q \rfloor \\ \text{is odd.}
    /// \end{cases}
    /// $$
    ///
    /// $f(x, y, \mathrm{Exact}) = q$, but panics if $q \notin \Z$.
    ///
    /// The following two expressions are equivalent:
    /// - `x.round_to_multiple(other, Exact)`
    /// - `{ assert!(x.divisible_by(other)); x }`
    ///
    /// but the latter should be used as it is clearer and more efficient.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// - If `rm` is `Exact`, but `self` is not a multiple of `other`.
    /// - If `self` is nonzero, `other` is zero, and `rm` is trying to round away from zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::RoundToMultiple;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     (&Integer::from(-5))
    ///         .round_to_multiple(Integer::ZERO, Down)
    ///         .to_debug_string(),
    ///     "(0, Greater)"
    /// );
    ///
    /// assert_eq!(
    ///     (&Integer::from(-10))
    ///         .round_to_multiple(Integer::from(4), Down)
    ///         .to_debug_string(),
    ///     "(-8, Greater)"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10))
    ///         .round_to_multiple(Integer::from(4), Up)
    ///         .to_debug_string(),
    ///     "(-12, Less)"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10))
    ///         .round_to_multiple(Integer::from(5), Exact)
    ///         .to_debug_string(),
    ///     "(-10, Equal)"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10))
    ///         .round_to_multiple(Integer::from(3), Nearest)
    ///         .to_debug_string(),
    ///     "(-9, Greater)"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-20))
    ///         .round_to_multiple(Integer::from(3), Nearest)
    ///         .to_debug_string(),
    ///     "(-21, Less)"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10))
    ///         .round_to_multiple(Integer::from(4), Nearest)
    ///         .to_debug_string(),
    ///     "(-8, Greater)"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-14))
    ///         .round_to_multiple(Integer::from(4), Nearest)
    ///         .to_debug_string(),
    ///     "(-16, Less)"
    /// );
    ///
    /// assert_eq!(
    ///     (&Integer::from(-10))
    ///         .round_to_multiple(Integer::from(-4), Down)
    ///         .to_debug_string(),
    ///     "(-8, Greater)"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10))
    ///         .round_to_multiple(Integer::from(-4), Up)
    ///         .to_debug_string(),
    ///     "(-12, Less)"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10))
    ///         .round_to_multiple(Integer::from(-5), Exact)
    ///         .to_debug_string(),
    ///     "(-10, Equal)"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10))
    ///         .round_to_multiple(Integer::from(-3), Nearest)
    ///         .to_debug_string(),
    ///     "(-9, Greater)"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-20))
    ///         .round_to_multiple(Integer::from(-3), Nearest)
    ///         .to_debug_string(),
    ///     "(-21, Less)"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10))
    ///         .round_to_multiple(Integer::from(-4), Nearest)
    ///         .to_debug_string(),
    ///     "(-8, Greater)"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-14))
    ///         .round_to_multiple(Integer::from(-4), Nearest)
    ///         .to_debug_string(),
    ///     "(-16, Less)"
    /// );
    /// ```
    fn round_to_multiple(self, other: Integer, rm: RoundingMode) -> (Integer, Ordering) {
        let (n, o) = (&self.abs).round_to_multiple(other.abs, if self.sign { rm } else { -rm });
        (
            Integer::from_sign_and_abs(self.sign, n),
            if self.sign { o } else { o.reverse() },
        )
    }
}

impl RoundToMultiple<&Integer> for &Integer {
    type Output = Integer;

    /// Rounds an [`Integer`] to a multiple of another [`Integer`], according to a specified
    /// rounding mode. Both [`Integer`]s are taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the returned value is less than, equal to, or greater than the original
    /// value.
    ///
    /// Let $q = \frac{x}{|y|}$:
    ///
    /// $f(x, y, \mathrm{Down}) =  \operatorname{sgn}(q) |y| \lfloor |q| \rfloor.$
    ///
    /// $f(x, y, \mathrm{Up}) = \operatorname{sgn}(q) |y| \lceil |q| \rceil.$
    ///
    /// $f(x, y, \mathrm{Floor}) = |y| \lfloor q \rfloor.$
    ///
    /// $f(x, y, \mathrm{Ceiling}) = |y| \lceil q \rceil.$
    ///
    /// $$
    /// f(x, y, \mathrm{Nearest}) = \begin{cases}
    ///     y \lfloor q \rfloor & \text{if} \\quad
    ///         q - \lfloor q \rfloor < \frac{1}{2} \\\\
    ///     y \lceil q \rceil & \text{if} \\quad q - \lfloor q \rfloor > \frac{1}{2} \\\\
    ///     y \lfloor q \rfloor &
    ///     \text{if} \\quad q - \lfloor q \rfloor =
    ///         \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor
    ///     \\ \text{is even} \\\\
    ///     y \lceil q \rceil &
    ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2}
    ///         \\ \text{and} \\ \lfloor q \rfloor \\ \text{is odd.}
    /// \end{cases}
    /// $$
    ///
    /// $f(x, y, \mathrm{Exact}) = q$, but panics if $q \notin \Z$.
    ///
    /// The following two expressions are equivalent:
    /// - `x.round_to_multiple(other, Exact)`
    /// - `{ assert!(x.divisible_by(other)); x }`
    ///
    /// but the latter should be used as it is clearer and more efficient.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// - If `rm` is `Exact`, but `self` is not a multiple of `other`.
    /// - If `self` is nonzero, `other` is zero, and `rm` is trying to round away from zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::RoundToMultiple;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     (&Integer::from(-5))
    ///         .round_to_multiple(&Integer::ZERO, Down)
    ///         .to_debug_string(),
    ///     "(0, Greater)"
    /// );
    ///
    /// assert_eq!(
    ///     (&Integer::from(-10))
    ///         .round_to_multiple(&Integer::from(4), Down)
    ///         .to_debug_string(),
    ///     "(-8, Greater)"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10))
    ///         .round_to_multiple(&Integer::from(4), Up)
    ///         .to_debug_string(),
    ///     "(-12, Less)"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10))
    ///         .round_to_multiple(&Integer::from(5), Exact)
    ///         .to_debug_string(),
    ///     "(-10, Equal)"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10))
    ///         .round_to_multiple(&Integer::from(3), Nearest)
    ///         .to_debug_string(),
    ///     "(-9, Greater)"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-20))
    ///         .round_to_multiple(&Integer::from(3), Nearest)
    ///         .to_debug_string(),
    ///     "(-21, Less)"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10))
    ///         .round_to_multiple(&Integer::from(4), Nearest)
    ///         .to_debug_string(),
    ///     "(-8, Greater)"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-14))
    ///         .round_to_multiple(&Integer::from(4), Nearest)
    ///         .to_debug_string(),
    ///     "(-16, Less)"
    /// );
    ///
    /// assert_eq!(
    ///     (&Integer::from(-10))
    ///         .round_to_multiple(&Integer::from(-4), Down)
    ///         .to_debug_string(),
    ///     "(-8, Greater)"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10))
    ///         .round_to_multiple(&Integer::from(-4), Up)
    ///         .to_debug_string(),
    ///     "(-12, Less)"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10))
    ///         .round_to_multiple(&Integer::from(-5), Exact)
    ///         .to_debug_string(),
    ///     "(-10, Equal)"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10))
    ///         .round_to_multiple(&Integer::from(-3), Nearest)
    ///         .to_debug_string(),
    ///     "(-9, Greater)"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-20))
    ///         .round_to_multiple(&Integer::from(-3), Nearest)
    ///         .to_debug_string(),
    ///     "(-21, Less)"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10))
    ///         .round_to_multiple(&Integer::from(-4), Nearest)
    ///         .to_debug_string(),
    ///     "(-8, Greater)"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-14))
    ///         .round_to_multiple(&Integer::from(-4), Nearest)
    ///         .to_debug_string(),
    ///     "(-16, Less)"
    /// );
    /// ```
    fn round_to_multiple(self, other: &Integer, rm: RoundingMode) -> (Integer, Ordering) {
        let (n, o) = (&self.abs).round_to_multiple(&other.abs, if self.sign { rm } else { -rm });
        (
            Integer::from_sign_and_abs(self.sign, n),
            if self.sign { o } else { o.reverse() },
        )
    }
}

impl RoundToMultipleAssign<Integer> for Integer {
    /// Rounds an [`Integer`] to a multiple of another [`Integer`] in place, according to a
    /// specified rounding mode. The [`Integer`] on the right-hand side is taken by value. An
    /// [`Ordering`] is returned, indicating whether the returned value is less than, equal to, or
    /// greater than the original value.
    ///
    /// See the [`RoundToMultiple`] documentation for details.
    ///
    /// The following two expressions are equivalent:
    /// - `x.round_to_multiple_assign(other, Exact);`
    /// - `assert!(x.divisible_by(other));`
    ///
    /// but the latter should be used as it is clearer and more efficient.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// - If `rm` is `Exact`, but `self` is not a multiple of `other`.
    /// - If `self` is nonzero, `other` is zero, and `rm` is trying to round away from zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::num::arithmetic::traits::RoundToMultipleAssign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(-5);
    /// assert_eq!(x.round_to_multiple_assign(Integer::ZERO, Down), Greater);
    /// assert_eq!(x, 0);
    ///
    /// let mut x = Integer::from(-10);
    /// assert_eq!(x.round_to_multiple_assign(Integer::from(4), Down), Greater);
    /// assert_eq!(x, -8);
    ///
    /// let mut x = Integer::from(-10);
    /// assert_eq!(x.round_to_multiple_assign(Integer::from(4), Up), Less);
    /// assert_eq!(x, -12);
    ///
    /// let mut x = Integer::from(-10);
    /// assert_eq!(x.round_to_multiple_assign(Integer::from(5), Exact), Equal);
    /// assert_eq!(x, -10);
    ///
    /// let mut x = Integer::from(-10);
    /// assert_eq!(
    ///     x.round_to_multiple_assign(Integer::from(3), Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x, -9);
    ///
    /// let mut x = Integer::from(-20);
    /// assert_eq!(x.round_to_multiple_assign(Integer::from(3), Nearest), Less);
    /// assert_eq!(x, -21);
    ///
    /// let mut x = Integer::from(-10);
    /// assert_eq!(
    ///     x.round_to_multiple_assign(Integer::from(4), Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x, -8);
    ///
    /// let mut x = Integer::from(-14);
    /// assert_eq!(x.round_to_multiple_assign(Integer::from(4), Nearest), Less);
    /// assert_eq!(x, -16);
    ///
    /// let mut x = Integer::from(-10);
    /// assert_eq!(x.round_to_multiple_assign(Integer::from(-4), Down), Greater);
    /// assert_eq!(x, -8);
    ///
    /// let mut x = Integer::from(-10);
    /// assert_eq!(x.round_to_multiple_assign(Integer::from(-4), Up), Less);
    /// assert_eq!(x, -12);
    ///
    /// let mut x = Integer::from(-10);
    /// assert_eq!(x.round_to_multiple_assign(Integer::from(-5), Exact), Equal);
    /// assert_eq!(x, -10);
    ///
    /// let mut x = Integer::from(-10);
    /// assert_eq!(
    ///     x.round_to_multiple_assign(Integer::from(-3), Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x, -9);
    ///
    /// let mut x = Integer::from(-20);
    /// assert_eq!(x.round_to_multiple_assign(Integer::from(-3), Nearest), Less);
    /// assert_eq!(x, -21);
    ///
    /// let mut x = Integer::from(-10);
    /// assert_eq!(
    ///     x.round_to_multiple_assign(Integer::from(-4), Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x, -8);
    ///
    /// let mut x = Integer::from(-14);
    /// assert_eq!(x.round_to_multiple_assign(Integer::from(-4), Nearest), Less);
    /// assert_eq!(x, -16);
    /// ```
    fn round_to_multiple_assign(&mut self, other: Integer, rm: RoundingMode) -> Ordering {
        if self.sign {
            self.abs.round_to_multiple_assign(other.abs, rm)
        } else {
            let o = self.abs.round_to_multiple_assign(other.abs, -rm);
            self.sign = self.abs == 0;
            o.reverse()
        }
    }
}

impl RoundToMultipleAssign<&Integer> for Integer {
    /// Rounds an [`Integer`] to a multiple of another [`Integer`] in place, according to a
    /// specified rounding mode. The [`Integer`] on the right-hand side is taken by reference. An
    /// [`Ordering`] is returned, indicating whether the returned value is less than, equal to, or
    /// greater than the original value.
    ///
    /// See the [`RoundToMultiple`] documentation for details.
    ///
    /// The following two expressions are equivalent:
    /// - `x.round_to_multiple_assign(other, Exact);`
    /// - `assert!(x.divisible_by(other));`
    ///
    /// but the latter should be used as it is clearer and more efficient.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// - If `rm` is `Exact`, but `self` is not a multiple of `other`.
    /// - If `self` is nonzero, `other` is zero, and `rm` is trying to round away from zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::num::arithmetic::traits::RoundToMultipleAssign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(-5);
    /// assert_eq!(x.round_to_multiple_assign(&Integer::ZERO, Down), Greater);
    /// assert_eq!(x, 0);
    ///
    /// let mut x = Integer::from(-10);
    /// assert_eq!(x.round_to_multiple_assign(&Integer::from(4), Down), Greater);
    /// assert_eq!(x, -8);
    ///
    /// let mut x = Integer::from(-10);
    /// assert_eq!(x.round_to_multiple_assign(&Integer::from(4), Up), Less);
    /// assert_eq!(x, -12);
    ///
    /// let mut x = Integer::from(-10);
    /// assert_eq!(x.round_to_multiple_assign(&Integer::from(5), Exact), Equal);
    /// assert_eq!(x, -10);
    ///
    /// let mut x = Integer::from(-10);
    /// assert_eq!(
    ///     x.round_to_multiple_assign(&Integer::from(3), Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x, -9);
    ///
    /// let mut x = Integer::from(-20);
    /// assert_eq!(x.round_to_multiple_assign(&Integer::from(3), Nearest), Less);
    /// assert_eq!(x, -21);
    ///
    /// let mut x = Integer::from(-10);
    /// assert_eq!(
    ///     x.round_to_multiple_assign(&Integer::from(4), Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x, -8);
    ///
    /// let mut x = Integer::from(-14);
    /// assert_eq!(x.round_to_multiple_assign(&Integer::from(4), Nearest), Less);
    /// assert_eq!(x, -16);
    ///
    /// let mut x = Integer::from(-10);
    /// assert_eq!(
    ///     x.round_to_multiple_assign(&Integer::from(-4), Down),
    ///     Greater
    /// );
    /// assert_eq!(x, -8);
    ///
    /// let mut x = Integer::from(-10);
    /// assert_eq!(x.round_to_multiple_assign(&Integer::from(-4), Up), Less);
    /// assert_eq!(x, -12);
    ///
    /// let mut x = Integer::from(-10);
    /// assert_eq!(x.round_to_multiple_assign(&Integer::from(-5), Exact), Equal);
    /// assert_eq!(x, -10);
    ///
    /// let mut x = Integer::from(-10);
    /// assert_eq!(
    ///     x.round_to_multiple_assign(&Integer::from(-3), Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x, -9);
    ///
    /// let mut x = Integer::from(-20);
    /// assert_eq!(
    ///     x.round_to_multiple_assign(&Integer::from(-3), Nearest),
    ///     Less
    /// );
    /// assert_eq!(x, -21);
    ///
    /// let mut x = Integer::from(-10);
    /// assert_eq!(
    ///     x.round_to_multiple_assign(&Integer::from(-4), Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x, -8);
    ///
    /// let mut x = Integer::from(-14);
    /// assert_eq!(
    ///     x.round_to_multiple_assign(&Integer::from(-4), Nearest),
    ///     Less
    /// );
    /// assert_eq!(x, -16);
    /// ```
    fn round_to_multiple_assign(&mut self, other: &Integer, rm: RoundingMode) -> Ordering {
        if self.sign {
            self.abs.round_to_multiple_assign(&other.abs, rm)
        } else {
            let o = self.abs.round_to_multiple_assign(&other.abs, -rm);
            self.sign = self.abs == 0;
            o.reverse()
        }
    }
}
