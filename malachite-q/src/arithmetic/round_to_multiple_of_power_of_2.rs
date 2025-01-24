// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use core::cmp::Ordering;
use malachite_base::num::arithmetic::traits::{
    RoundToMultipleOfPowerOf2, RoundToMultipleOfPowerOf2Assign,
};
use malachite_base::num::conversion::traits::RoundingFrom;
use malachite_base::rounding_modes::RoundingMode;
use malachite_nz::integer::Integer;

impl RoundToMultipleOfPowerOf2<i64> for Rational {
    type Output = Rational;

    /// Rounds a [`Rational`] to an integer multiple of $2^k$ according to a specified rounding
    /// mode. The [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the returned value is less than, equal to, or greater than the original value.
    ///
    /// Let $q = \frac{x}{2^k}$:
    ///
    /// $f(x, k, \mathrm{Down}) = 2^k \operatorname{sgn}(q) \lfloor |q| \rfloor.$
    ///
    /// $f(x, k, \mathrm{Up}) = 2^k \operatorname{sgn}(q) \lceil |q| \rceil.$
    ///
    /// $f(x, k, \mathrm{Floor}) = 2^k \lfloor q \rfloor.$
    ///
    /// $f(x, k, \mathrm{Ceiling}) = 2^k \lceil q \rceil.$
    ///
    /// $$
    /// f(x, k, \mathrm{Nearest}) = \begin{cases}
    ///     2^k \lfloor q \rfloor & \text{if} \\quad
    ///     q - \lfloor q \rfloor < \frac{1}{2} \\\\
    ///     2^k \lceil q \rceil & \text{if} \\quad q - \lfloor q \rfloor > \frac{1}{2} \\\\
    ///     2^k \lfloor q \rfloor &
    ///     \text{if} \\quad q - \lfloor q \rfloor =
    ///         \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor
    ///     \\ \text{is even} \\\\
    ///     2^k \lceil q \rceil &
    ///     \text{if} \\quad q - \lfloor q \rfloor =
    ///         \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor \\ \text{is odd.}
    /// \end{cases}
    /// $$
    ///
    /// $f(x, k, \mathrm{Exact}) = 2^k q$, but panics if $q \notin \Z$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(), pow /
    /// Limb::WIDTH)`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, but `self` is not a multiple of the power of 2.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::RoundToMultipleOfPowerOf2;
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_q::Rational;
    ///
    /// let q = Rational::exact_from(std::f64::consts::PI);
    /// assert_eq!(
    ///     q.clone()
    ///         .round_to_multiple_of_power_of_2(-3, Floor)
    ///         .to_debug_string(),
    ///     "(25/8, Less)"
    /// );
    /// assert_eq!(
    ///     q.clone()
    ///         .round_to_multiple_of_power_of_2(-3, Down)
    ///         .to_debug_string(),
    ///     "(25/8, Less)"
    /// );
    /// assert_eq!(
    ///     q.clone()
    ///         .round_to_multiple_of_power_of_2(-3, Ceiling)
    ///         .to_debug_string(),
    ///     "(13/4, Greater)"
    /// );
    /// assert_eq!(
    ///     q.clone()
    ///         .round_to_multiple_of_power_of_2(-3, Up)
    ///         .to_debug_string(),
    ///     "(13/4, Greater)"
    /// );
    /// assert_eq!(
    ///     q.clone()
    ///         .round_to_multiple_of_power_of_2(-3, Nearest)
    ///         .to_debug_string(),
    ///     "(25/8, Less)"
    /// );
    /// ```
    #[inline]
    fn round_to_multiple_of_power_of_2(
        mut self,
        pow: i64,
        rm: RoundingMode,
    ) -> (Rational, Ordering) {
        let o = self.round_to_multiple_of_power_of_2_assign(pow, rm);
        (self, o)
    }
}

impl RoundToMultipleOfPowerOf2<i64> for &Rational {
    type Output = Rational;

    /// Rounds a [`Rational`] to an integer multiple of $2^k$ according to a specified rounding
    /// mode. The [`Rational`] is taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the returned value is less than, equal to, or greater than the original value.
    ///
    /// Let $q = \frac{x}{2^k}$:
    ///
    /// $f(x, k, \mathrm{Down}) = 2^k \operatorname{sgn}(q) \lfloor |q| \rfloor.$
    ///
    /// $f(x, k, \mathrm{Up}) = 2^k \operatorname{sgn}(q) \lceil |q| \rceil.$
    ///
    /// $f(x, k, \mathrm{Floor}) = 2^k \lfloor q \rfloor.$
    ///
    /// $f(x, k, \mathrm{Ceiling}) = 2^k \lceil q \rceil.$
    ///
    /// $$
    /// f(x, k, \mathrm{Nearest}) = \begin{cases}
    ///     2^k \lfloor q \rfloor & \text{if} \\quad
    ///     q - \lfloor q \rfloor < \frac{1}{2} \\\\
    ///     2^k \lceil q \rceil & \text{if} \\quad q - \lfloor q \rfloor > \frac{1}{2} \\\\
    ///     2^k \lfloor q \rfloor &
    ///     \text{if} \\quad q - \lfloor q \rfloor =
    ///         \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor
    ///     \\ \text{is even} \\\\
    ///     2^k \lceil q \rceil &
    ///     \text{if} \\quad q - \lfloor q \rfloor =
    ///         \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor \\ \text{is odd.}
    /// \end{cases}
    /// $$
    ///
    /// $f(x, k, \mathrm{Exact}) = 2^k q$, but panics if $q \notin \Z$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(), pow /
    /// Limb::WIDTH)`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, but `self` is not a multiple of the power of 2.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::RoundToMultipleOfPowerOf2;
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_q::Rational;
    ///
    /// let q = Rational::exact_from(std::f64::consts::PI);
    /// assert_eq!(
    ///     (&q).round_to_multiple_of_power_of_2(-3, Floor)
    ///         .to_debug_string(),
    ///     "(25/8, Less)"
    /// );
    /// assert_eq!(
    ///     (&q).round_to_multiple_of_power_of_2(-3, Down)
    ///         .to_debug_string(),
    ///     "(25/8, Less)"
    /// );
    /// assert_eq!(
    ///     (&q).round_to_multiple_of_power_of_2(-3, Ceiling)
    ///         .to_debug_string(),
    ///     "(13/4, Greater)"
    /// );
    /// assert_eq!(
    ///     (&q).round_to_multiple_of_power_of_2(-3, Up)
    ///         .to_debug_string(),
    ///     "(13/4, Greater)"
    /// );
    /// assert_eq!(
    ///     (&q).round_to_multiple_of_power_of_2(-3, Nearest)
    ///         .to_debug_string(),
    ///     "(25/8, Less)"
    /// );
    /// ```
    fn round_to_multiple_of_power_of_2(self, pow: i64, rm: RoundingMode) -> (Rational, Ordering) {
        let (s, o) = Integer::rounding_from(self >> pow, rm);
        (Rational::from(s) << pow, o)
    }
}

impl RoundToMultipleOfPowerOf2Assign<i64> for Rational {
    /// Rounds a [`Rational`] to a multiple of $2^k$ in place, according to a specified rounding
    /// mode. An [`Ordering`] is returned, indicating whether the returned value is less than, equal
    /// to, or greater than the original value.
    ///
    /// See the [`RoundToMultipleOfPowerOf2`] documentation for details.
    ///
    /// but the latter should be used as it is clearer and more efficient.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(), pow /
    /// Limb::WIDTH)`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, but `self` is not a multiple of the power of 2.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::RoundToMultipleOfPowerOf2Assign;
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let q = Rational::exact_from(std::f64::consts::PI);
    ///
    /// let mut x = q.clone();
    /// assert_eq!(x.round_to_multiple_of_power_of_2_assign(-3, Floor), Less);
    /// assert_eq!(x.to_string(), "25/8");
    ///
    /// let mut x = q.clone();
    /// assert_eq!(x.round_to_multiple_of_power_of_2_assign(-3, Down), Less);
    /// assert_eq!(x.to_string(), "25/8");
    ///
    /// let mut x = q.clone();
    /// assert_eq!(
    ///     x.round_to_multiple_of_power_of_2_assign(-3, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "13/4");
    ///
    /// let mut x = q.clone();
    /// assert_eq!(x.round_to_multiple_of_power_of_2_assign(-3, Up), Greater);
    /// assert_eq!(x.to_string(), "13/4");
    ///
    /// let mut x = q.clone();
    /// assert_eq!(x.round_to_multiple_of_power_of_2_assign(-3, Nearest), Less);
    /// assert_eq!(x.to_string(), "25/8");
    /// ```
    fn round_to_multiple_of_power_of_2_assign(&mut self, pow: i64, rm: RoundingMode) -> Ordering {
        *self >>= pow;
        let (s, o) = Integer::rounding_from(&*self, rm);
        *self = Rational::from(s) << pow;
        o
    }
}
