// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{NegAssign, RoundToMultiple, RoundToMultipleAssign};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::RoundingFrom;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;

impl RoundToMultiple<Rational> for Rational {
    type Output = Rational;

    /// Rounds a [`Rational`] to an integer multiple of another [`Rational`], according to a
    /// specified rounding mode. Both [`Rational`]s are taken by value. An [`Ordering`] is also
    /// returned, indicating whether the returned value is less than, equal to, or greater than the
    /// original value.
    ///
    /// Let $q = \frac{x}{y}$:
    ///
    /// $f(x, y, \mathrm{Down}) = f(x, y, \mathrm{Floor}) = y \lfloor q \rfloor.$
    ///
    /// $f(x, y, \mathrm{Up}) = f(x, y, \mathrm{Ceiling}) = y \lceil q \rceil.$
    ///
    /// $$
    /// f(x, y, \mathrm{Nearest}) = \begin{cases}
    ///     y \lfloor q \rfloor & \text{if} \\quad
    ///         q - \lfloor q \rfloor < \frac{1}{2} \\\\
    ///     y \lceil q \rceil & \text{if} \\quad q - \lfloor q \rfloor > \frac{1}{2} \\\\
    ///     y \lfloor q \rfloor &
    ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor
    ///     \\ \text{is even} \\\\
    ///     y \lceil q \rceil & \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2}
    ///         \\ \text{and} \\ \lfloor q \rfloor \\ \text{is odd.}
    /// \end{cases}
    /// $$
    ///
    /// $f(x, y, \mathrm{Exact}) = x$, but panics if $q \notin \Z$.
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
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::from(-5)
    ///         .round_to_multiple(Rational::ZERO, Down)
    ///         .to_debug_string(),
    ///     "(0, Greater)"
    /// );
    ///
    /// let q = Rational::exact_from(std::f64::consts::PI);
    /// let hundredth = Rational::from_signeds(1, 100);
    /// assert_eq!(
    ///     q.clone()
    ///         .round_to_multiple(hundredth.clone(), Down)
    ///         .to_debug_string(),
    ///     "(157/50, Less)"
    /// );
    /// assert_eq!(
    ///     q.clone()
    ///         .round_to_multiple(hundredth.clone(), Floor)
    ///         .to_debug_string(),
    ///     "(157/50, Less)"
    /// );
    /// assert_eq!(
    ///     q.clone()
    ///         .round_to_multiple(hundredth.clone(), Up)
    ///         .to_debug_string(),
    ///     "(63/20, Greater)"
    /// );
    /// assert_eq!(
    ///     q.clone()
    ///         .round_to_multiple(hundredth.clone(), Ceiling)
    ///         .to_debug_string(),
    ///     "(63/20, Greater)"
    /// );
    /// assert_eq!(
    ///     q.clone()
    ///         .round_to_multiple(hundredth.clone(), Nearest)
    ///         .to_debug_string(),
    ///     "(157/50, Less)"
    /// );
    /// ```
    #[inline]
    fn round_to_multiple(mut self, other: Rational, rm: RoundingMode) -> (Rational, Ordering) {
        let o = self.round_to_multiple_assign(other, rm);
        (self, o)
    }
}

impl RoundToMultiple<&Rational> for Rational {
    type Output = Rational;

    /// Rounds a [`Rational`] to an integer multiple of another [`Rational`], according to a
    /// specified rounding mode. The first [`Rational`] is taken by value and the second by
    /// reference. An [`Ordering`] is also returned, indicating whether the returned value is less
    /// than, equal to, or greater than the original value.
    ///
    /// Let $q = \frac{x}{y}$:
    ///
    /// $f(x, y, \mathrm{Down}) = f(x, y, \mathrm{Floor}) = y \lfloor q \rfloor.$
    ///
    /// $f(x, y, \mathrm{Up}) = f(x, y, \mathrm{Ceiling}) = y \lceil q \rceil.$
    ///
    /// $$
    /// f(x, y, \mathrm{Nearest}) = \begin{cases}
    ///     y \lfloor q \rfloor & \text{if} \\quad
    ///         q - \lfloor q \rfloor < \frac{1}{2} \\\\
    ///     y \lceil q \rceil & \text{if} \\quad q - \lfloor q \rfloor > \frac{1}{2} \\\\
    ///     y \lfloor q \rfloor &
    ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor
    ///     \\ \text{is even} \\\\
    ///     y \lceil q \rceil & \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2}
    ///         \\ \text{and} \\ \lfloor q \rfloor \\ \text{is odd.}
    /// \end{cases}
    /// $$
    ///
    /// $f(x, y, \mathrm{Exact}) = x$, but panics if $q \notin \Z$.
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
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::from(-5)
    ///         .round_to_multiple(&Rational::ZERO, Down)
    ///         .to_debug_string(),
    ///     "(0, Greater)"
    /// );
    ///
    /// let q = Rational::exact_from(std::f64::consts::PI);
    /// let hundredth = Rational::from_signeds(1, 100);
    /// assert_eq!(
    ///     q.clone()
    ///         .round_to_multiple(&hundredth, Down)
    ///         .to_debug_string(),
    ///     "(157/50, Less)"
    /// );
    /// assert_eq!(
    ///     q.clone()
    ///         .round_to_multiple(&hundredth, Floor)
    ///         .to_debug_string(),
    ///     "(157/50, Less)"
    /// );
    /// assert_eq!(
    ///     q.clone()
    ///         .round_to_multiple(&hundredth, Up)
    ///         .to_debug_string(),
    ///     "(63/20, Greater)"
    /// );
    /// assert_eq!(
    ///     q.clone()
    ///         .round_to_multiple(&hundredth, Ceiling)
    ///         .to_debug_string(),
    ///     "(63/20, Greater)"
    /// );
    /// assert_eq!(
    ///     q.clone()
    ///         .round_to_multiple(&hundredth, Nearest)
    ///         .to_debug_string(),
    ///     "(157/50, Less)"
    /// );
    /// ```
    #[inline]
    fn round_to_multiple(mut self, other: &Rational, rm: RoundingMode) -> (Rational, Ordering) {
        let o = self.round_to_multiple_assign(other, rm);
        (self, o)
    }
}

impl RoundToMultiple<Rational> for &Rational {
    type Output = Rational;

    /// Rounds a [`Rational`] to an integer multiple of another [`Rational`], according to a
    /// specified rounding mode. The first [`Rational`] is taken by reference and the second by
    /// value. An [`Ordering`] is also returned, indicating whether the returned value is less than,
    /// equal to, or greater than the original value.
    ///
    /// Let $q = \frac{x}{y}$:
    ///
    /// $f(x, y, \mathrm{Down}) = f(x, y, \mathrm{Floor}) = y \lfloor q \rfloor.$
    ///
    /// $f(x, y, \mathrm{Up}) = f(x, y, \mathrm{Ceiling}) = y \lceil q \rceil.$
    ///
    /// $$
    /// f(x, y, \mathrm{Nearest}) = \begin{cases}
    ///     y \lfloor q \rfloor & \text{if} \\quad
    ///         q - \lfloor q \rfloor < \frac{1}{2} \\\\
    ///     y \lceil q \rceil & \text{if} \\quad q - \lfloor q \rfloor > \frac{1}{2} \\\\
    ///     y \lfloor q \rfloor &
    ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor
    ///     \\ \text{is even} \\\\
    ///     y \lceil q \rceil & \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2}
    ///         \\ \text{and} \\ \lfloor q \rfloor \\ \text{is odd.}
    /// \end{cases}
    /// $$
    ///
    /// $f(x, y, \mathrm{Exact}) = x$, but panics if $q \notin \Z$.
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
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     (&Rational::from(-5))
    ///         .round_to_multiple(Rational::ZERO, Down)
    ///         .to_debug_string(),
    ///     "(0, Greater)"
    /// );
    ///
    /// let q = Rational::exact_from(std::f64::consts::PI);
    /// let hundredth = Rational::from_signeds(1, 100);
    /// assert_eq!(
    ///     (&q).round_to_multiple(hundredth.clone(), Down)
    ///         .to_debug_string(),
    ///     "(157/50, Less)"
    /// );
    /// assert_eq!(
    ///     (&q).round_to_multiple(hundredth.clone(), Floor)
    ///         .to_debug_string(),
    ///     "(157/50, Less)"
    /// );
    /// assert_eq!(
    ///     (&q).round_to_multiple(hundredth.clone(), Up)
    ///         .to_debug_string(),
    ///     "(63/20, Greater)"
    /// );
    /// assert_eq!(
    ///     (&q).round_to_multiple(hundredth.clone(), Ceiling)
    ///         .to_debug_string(),
    ///     "(63/20, Greater)"
    /// );
    /// assert_eq!(
    ///     (&q).round_to_multiple(hundredth.clone(), Nearest)
    ///         .to_debug_string(),
    ///     "(157/50, Less)"
    /// );
    /// ```
    fn round_to_multiple(self, other: Rational, mut rm: RoundingMode) -> (Rational, Ordering) {
        if *self == other {
            return (self.clone(), Equal);
        }
        if other == 0u32 {
            if rm == Down || rm == Nearest || rm == if *self >= 0u32 { Floor } else { Ceiling } {
                return (Rational::ZERO, if *self >= 0u32 { Less } else { Greater });
            }
            panic!("Cannot round {self} to zero using RoundingMode {rm}");
        }
        if !other.sign {
            rm.neg_assign();
        }
        let (x, mut o) = Integer::rounding_from(self / &other, rm);
        if !other.sign {
            o = o.reverse();
        }
        (Rational::from(x) * other, o)
    }
}

impl RoundToMultiple<&Rational> for &Rational {
    type Output = Rational;

    /// Rounds a [`Rational`] to an integer multiple of another [`Rational`], according to a
    /// specified rounding mode. Both [`Rational`]s are taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the returned value is less than, equal to, or greater than the
    /// original value.
    ///
    /// Let $q = \frac{x}{y}$:
    ///
    /// $f(x, y, \mathrm{Down}) = f(x, y, \mathrm{Floor}) = y \lfloor q \rfloor.$
    ///
    /// $f(x, y, \mathrm{Up}) = f(x, y, \mathrm{Ceiling}) = y \lceil q \rceil.$
    ///
    /// $$
    /// f(x, y, \mathrm{Nearest}) = \begin{cases}
    ///     y \lfloor q \rfloor & \text{if} \\quad
    ///         q - \lfloor q \rfloor < \frac{1}{2} \\\\
    ///     y \lceil q \rceil & \text{if} \\quad q - \lfloor q \rfloor > \frac{1}{2} \\\\
    ///     y \lfloor q \rfloor &
    ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor
    ///     \\ \text{is even} \\\\
    ///     y \lceil q \rceil & \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2}
    ///         \\ \text{and} \\ \lfloor q \rfloor \\ \text{is odd.}
    /// \end{cases}
    /// $$
    ///
    /// $f(x, y, \mathrm{Exact}) = x$, but panics if $q \notin \Z$.
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
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     (&Rational::from(-5))
    ///         .round_to_multiple(&Rational::ZERO, Down)
    ///         .to_debug_string(),
    ///     "(0, Greater)"
    /// );
    ///
    /// let q = Rational::exact_from(std::f64::consts::PI);
    /// let hundredth = Rational::from_signeds(1, 100);
    /// assert_eq!(
    ///     (&q).round_to_multiple(&hundredth, Down).to_debug_string(),
    ///     "(157/50, Less)"
    /// );
    /// assert_eq!(
    ///     (&q).round_to_multiple(&hundredth, Floor).to_debug_string(),
    ///     "(157/50, Less)"
    /// );
    /// assert_eq!(
    ///     (&q).round_to_multiple(&hundredth, Up).to_debug_string(),
    ///     "(63/20, Greater)"
    /// );
    /// assert_eq!(
    ///     (&q).round_to_multiple(&hundredth, Ceiling)
    ///         .to_debug_string(),
    ///     "(63/20, Greater)"
    /// );
    /// assert_eq!(
    ///     (&q).round_to_multiple(&hundredth, Nearest)
    ///         .to_debug_string(),
    ///     "(157/50, Less)"
    /// );
    /// ```
    fn round_to_multiple(self, other: &Rational, mut rm: RoundingMode) -> (Rational, Ordering) {
        if self == other {
            return (self.clone(), Equal);
        }
        if *other == 0u32 {
            if rm == Down || rm == Nearest || rm == if *self >= 0u32 { Floor } else { Ceiling } {
                return (Rational::ZERO, if *self >= 0 { Less } else { Greater });
            }
            panic!("Cannot round {self} to zero using RoundingMode {rm}");
        }
        if !other.sign {
            rm.neg_assign();
        }
        let (x, mut o) = Integer::rounding_from(self / other, rm);
        if !other.sign {
            o = o.reverse();
        }
        (Rational::from(x) * other, o)
    }
}

impl RoundToMultipleAssign<Rational> for Rational {
    /// Rounds a [`Rational`] to an integer multiple of another [`Rational`] in place, according to
    /// a  specified rounding mode. The [`Rational`] on the right-hand side is taken by value. An
    /// [`Ordering`] is returned, indicating whether the returned value is less than, equal to, or
    /// greater than the original value.
    ///
    /// See the [`RoundToMultiple`] documentation for details.
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
    /// use malachite_base::num::arithmetic::traits::RoundToMultipleAssign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Rational::from(-5);
    /// assert_eq!(x.round_to_multiple_assign(Rational::ZERO, Down), Greater);
    /// assert_eq!(x, 0);
    ///
    /// let q = Rational::exact_from(std::f64::consts::PI);
    /// let hundredth = Rational::from_signeds(1, 100);
    ///
    /// let mut x = q.clone();
    /// assert_eq!(x.round_to_multiple_assign(hundredth.clone(), Down), Less);
    /// assert_eq!(x.to_string(), "157/50");
    ///
    /// let mut x = q.clone();
    /// assert_eq!(x.round_to_multiple_assign(hundredth.clone(), Floor), Less);
    /// assert_eq!(x.to_string(), "157/50");
    ///
    /// let mut x = q.clone();
    /// assert_eq!(x.round_to_multiple_assign(hundredth.clone(), Up), Greater);
    /// assert_eq!(x.to_string(), "63/20");
    ///
    /// let mut x = q.clone();
    /// assert_eq!(
    ///     x.round_to_multiple_assign(hundredth.clone(), Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "63/20");
    ///
    /// let mut x = q.clone();
    /// assert_eq!(x.round_to_multiple_assign(hundredth.clone(), Nearest), Less);
    /// assert_eq!(x.to_string(), "157/50");
    /// ```
    fn round_to_multiple_assign(&mut self, other: Rational, mut rm: RoundingMode) -> Ordering {
        if *self == other {
            return Equal;
        }
        if other == 0u32 {
            if rm == Down || rm == Nearest || rm == if *self >= 0u32 { Floor } else { Ceiling } {
                let o = if *self >= 0 { Less } else { Greater };
                *self = Rational::ZERO;
                return o;
            }
            panic!("Cannot round {self} to zero using RoundingMode {rm}");
        }
        if !other.sign {
            rm.neg_assign();
        }
        *self /= &other;
        let (x, o) = Integer::rounding_from(&*self, rm);
        let other_sign = other.sign;
        *self = Rational::from(x) * other;
        if other_sign {
            o
        } else {
            o.reverse()
        }
    }
}

impl RoundToMultipleAssign<&Rational> for Rational {
    /// Rounds a [`Rational`] to an integer multiple of another [`Rational`] in place, according to
    /// a specified rounding mode. The [`Rational`] on the right-hand side is taken by reference. An
    /// [`Ordering`] is returned, indicating whether the returned value is less than, equal to, or
    /// greater than the original value.
    ///
    /// See the [`RoundToMultiple`] documentation for details.
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
    /// use malachite_base::num::arithmetic::traits::RoundToMultipleAssign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Rational::from(-5);
    /// assert_eq!(x.round_to_multiple_assign(&Rational::ZERO, Down), Greater);
    /// assert_eq!(x, 0);
    ///
    /// let q = Rational::exact_from(std::f64::consts::PI);
    /// let hundredth = Rational::from_signeds(1, 100);
    ///
    /// let mut x = q.clone();
    /// assert_eq!(x.round_to_multiple_assign(&hundredth, Down), Less);
    /// assert_eq!(x.to_string(), "157/50");
    ///
    /// let mut x = q.clone();
    /// assert_eq!(x.round_to_multiple_assign(&hundredth, Floor), Less);
    /// assert_eq!(x.to_string(), "157/50");
    ///
    /// let mut x = q.clone();
    /// assert_eq!(x.round_to_multiple_assign(&hundredth, Up), Greater);
    /// assert_eq!(x.to_string(), "63/20");
    ///
    /// let mut x = q.clone();
    /// assert_eq!(x.round_to_multiple_assign(&hundredth, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "63/20");
    ///
    /// let mut x = q.clone();
    /// assert_eq!(x.round_to_multiple_assign(&hundredth, Nearest), Less);
    /// assert_eq!(x.to_string(), "157/50");
    /// ```
    fn round_to_multiple_assign(&mut self, other: &Rational, mut rm: RoundingMode) -> Ordering {
        if self == other {
            return Equal;
        }
        if *other == 0u32 {
            if rm == Down || rm == Nearest || rm == if *self >= 0u32 { Floor } else { Ceiling } {
                let o = if *self >= 0u32 { Less } else { Greater };
                *self = Rational::ZERO;
                return o;
            }
            panic!("Cannot round {self} to zero using RoundingMode {rm}");
        }
        if !other.sign {
            rm.neg_assign();
        }
        *self /= other;
        let (x, o) = Integer::rounding_from(&*self, rm);
        *self = Rational::from(x) * other;
        if other.sign {
            o
        } else {
            o.reverse()
        }
    }
}
