// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use core::cmp::Ordering;
use malachite_base::num::arithmetic::traits::{
    Average, AverageAssign, AverageRound, AverageRoundAssign, ShrRound, ShrRoundAssign,
};
use malachite_base::rounding_modes::RoundingMode::{self, Nearest};

impl Average<Self> for Integer {
    type Output = Self;

    /// Computes the average (arithmetic mean) of two [`Integer`]s, taking both by value and
    /// rounding to the nearest integer. Two-way ties are broken by rounding to the even integer.
    ///
    /// $$
    /// f(x, y) = \begin{cases}
    ///     a & \text{if} \\quad a \in \Z, \\\\
    ///     \lfloor a \rfloor & \text{if} \\quad a \notin \Z
    ///     \\ \text{and} \\ \lfloor a \rfloor \\ \text{is even}, \\\\
    ///     \lceil a \rceil & \text{if} \\quad a \notin \Z
    ///     \\ \text{and} \\ \lfloor a \rfloor \\ \text{is odd,}
    /// \end{cases}
    /// $$
    ///
    /// where $a = \frac{x + y}{2}$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Average;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(4).average(Integer::from(6)), 5);
    /// assert_eq!(Integer::from(-4).average(Integer::from(-5)), -4);
    /// assert_eq!(Integer::from(-5).average(Integer::from(-6)), -6);
    /// ```
    #[inline]
    fn average(self, other: Self) -> Self {
        (self + other).shr_round(1u32, Nearest).0
    }
}

impl Average<&Self> for Integer {
    type Output = Self;

    /// Computes the average (arithmetic mean) of two [`Integer`]s, taking the first by value and
    /// the second by reference and rounding to the nearest integer. Two-way ties are broken by
    /// rounding to the even integer.
    ///
    /// $$
    /// f(x, y) = \begin{cases}
    ///     a & \text{if} \\quad a \in \Z, \\\\
    ///     \lfloor a \rfloor & \text{if} \\quad a \notin \Z
    ///     \\ \text{and} \\ \lfloor a \rfloor \\ \text{is even}, \\\\
    ///     \lceil a \rceil & \text{if} \\quad a \notin \Z
    ///     \\ \text{and} \\ \lfloor a \rfloor \\ \text{is odd,}
    /// \end{cases}
    /// $$
    ///
    /// where $a = \frac{x + y}{2}$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Average;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(4).average(&Integer::from(6)), 5);
    /// assert_eq!(Integer::from(-4).average(&Integer::from(-5)), -4);
    /// assert_eq!(Integer::from(-5).average(&Integer::from(-6)), -6);
    /// ```
    #[inline]
    fn average(self, other: &Self) -> Self {
        (self + other).shr_round(1u32, Nearest).0
    }
}

impl Average<Integer> for &Integer {
    type Output = Integer;

    /// Computes the average (arithmetic mean) of two [`Integer`]s, taking the first by reference
    /// and the second by value and rounding to the nearest integer. Two-way ties are broken by
    /// rounding to the even integer.
    ///
    /// $$
    /// f(x, y) = \begin{cases}
    ///     a & \text{if} \\quad a \in \Z, \\\\
    ///     \lfloor a \rfloor & \text{if} \\quad a \notin \Z
    ///     \\ \text{and} \\ \lfloor a \rfloor \\ \text{is even}, \\\\
    ///     \lceil a \rceil & \text{if} \\quad a \notin \Z
    ///     \\ \text{and} \\ \lfloor a \rfloor \\ \text{is odd,}
    /// \end{cases}
    /// $$
    ///
    /// where $a = \frac{x + y}{2}$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Average;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!((&Integer::from(4)).average(Integer::from(6)), 5);
    /// assert_eq!((&Integer::from(-4)).average(Integer::from(-5)), -4);
    /// assert_eq!((&Integer::from(-5)).average(Integer::from(-6)), -6);
    /// ```
    #[inline]
    fn average(self, other: Integer) -> Integer {
        (self + other).shr_round(1u32, Nearest).0
    }
}

impl Average<&Integer> for &Integer {
    type Output = Integer;

    /// Computes the average (arithmetic mean) of two [`Integer`]s, taking both by reference and
    /// rounding to the nearest integer. Two-way ties are broken by rounding to the even integer.
    ///
    /// $$
    /// f(x, y) = \begin{cases}
    ///     a & \text{if} \\quad a \in \Z, \\\\
    ///     \lfloor a \rfloor & \text{if} \\quad a \notin \Z
    ///     \\ \text{and} \\ \lfloor a \rfloor \\ \text{is even}, \\\\
    ///     \lceil a \rceil & \text{if} \\quad a \notin \Z
    ///     \\ \text{and} \\ \lfloor a \rfloor \\ \text{is odd,}
    /// \end{cases}
    /// $$
    ///
    /// where $a = \frac{x + y}{2}$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Average;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!((&Integer::from(4)).average(&Integer::from(6)), 5);
    /// assert_eq!((&Integer::from(-4)).average(&Integer::from(-5)), -4);
    /// assert_eq!((&Integer::from(-5)).average(&Integer::from(-6)), -6);
    /// ```
    #[inline]
    fn average(self, other: &Integer) -> Integer {
        (self + other).shr_round(1u32, Nearest).0
    }
}

impl AverageAssign<Self> for Integer {
    /// Computes the average (arithmetic mean) of two [`Integer`]s, taking the [`Integer`] on the
    /// right-hand side by value, rounding to the nearest integer, and replacing the first
    /// [`Integer`] with it. Two-way ties are broken by rounding to the even integer.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::AverageAssign;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(-5);
    /// x.average_assign(Integer::from(-6));
    /// assert_eq!(x, -6);
    /// ```
    #[inline]
    fn average_assign(&mut self, other: Self) {
        *self += other;
        self.shr_round_assign(1u32, Nearest);
    }
}

impl AverageAssign<&Self> for Integer {
    /// Computes the average (arithmetic mean) of two [`Integer`]s, taking the [`Integer`] on the
    /// right-hand side by reference, rounding to the nearest integer, and replacing the first
    /// [`Integer`] with it. Two-way ties are broken by rounding to the even integer.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::AverageAssign;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(-5);
    /// x.average_assign(&Integer::from(-6));
    /// assert_eq!(x, -6);
    /// ```
    #[inline]
    fn average_assign(&mut self, other: &Self) {
        *self += other;
        self.shr_round_assign(1u32, Nearest);
    }
}

impl AverageRound<Self> for Integer {
    type Output = Self;

    /// Computes the average (arithmetic mean) of two [`Integer`]s, taking both by value and
    /// rounding according to a specified rounding mode. An [`Ordering`] is also returned,
    /// indicating whether the returned value is less than, equal to, or greater than the exact
    /// value.
    ///
    /// Let $a = \frac{x + y}{2}$. The rounding of an inexact average follows the [`AverageRound`]
    /// documentation in `malachite-base`, and the returned [`Ordering`] indicates whether the
    /// result is less than, equal to, or greater than $a$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the average of `self` and `other` is not an integer.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::num::arithmetic::traits::AverageRound;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     Integer::from(-4).average_round(Integer::from(-7), Floor),
    ///     (Integer::from(-6), Less)
    /// );
    /// assert_eq!(
    ///     Integer::from(-4).average_round(Integer::from(-7), Ceiling),
    ///     (Integer::from(-5), Greater)
    /// );
    /// assert_eq!(
    ///     Integer::from(-4).average_round(Integer::from(-7), Down),
    ///     (Integer::from(-5), Greater)
    /// );
    /// assert_eq!(
    ///     Integer::from(-4).average_round(Integer::from(-6), Exact),
    ///     (Integer::from(-5), Equal)
    /// );
    /// ```
    #[inline]
    fn average_round(self, other: Self, rm: RoundingMode) -> (Self, Ordering) {
        (self + other).shr_round(1u32, rm)
    }
}

impl AverageRound<&Self> for Integer {
    type Output = Self;

    /// Computes the average (arithmetic mean) of two [`Integer`]s, taking the first by value and
    /// the second by reference and rounding according to a specified rounding mode. An [`Ordering`]
    /// is also returned, indicating whether the returned value is less than, equal to, or greater
    /// than the exact value.
    ///
    /// Let $a = \frac{x + y}{2}$. The rounding of an inexact average follows the [`AverageRound`]
    /// documentation in `malachite-base`, and the returned [`Ordering`] indicates whether the
    /// result is less than, equal to, or greater than $a$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the average of `self` and `other` is not an integer.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::num::arithmetic::traits::AverageRound;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     Integer::from(-4).average_round(&Integer::from(-7), Floor),
    ///     (Integer::from(-6), Less)
    /// );
    /// assert_eq!(
    ///     Integer::from(-4).average_round(&Integer::from(-7), Ceiling),
    ///     (Integer::from(-5), Greater)
    /// );
    /// assert_eq!(
    ///     Integer::from(-4).average_round(&Integer::from(-7), Down),
    ///     (Integer::from(-5), Greater)
    /// );
    /// assert_eq!(
    ///     Integer::from(-4).average_round(&Integer::from(-6), Exact),
    ///     (Integer::from(-5), Equal)
    /// );
    /// ```
    #[inline]
    fn average_round(self, other: &Self, rm: RoundingMode) -> (Self, Ordering) {
        (self + other).shr_round(1u32, rm)
    }
}

impl AverageRound<Integer> for &Integer {
    type Output = Integer;

    /// Computes the average (arithmetic mean) of two [`Integer`]s, taking the first by reference
    /// and the second by value and rounding according to a specified rounding mode. An [`Ordering`]
    /// is also returned, indicating whether the returned value is less than, equal to, or greater
    /// than the exact value.
    ///
    /// Let $a = \frac{x + y}{2}$. The rounding of an inexact average follows the [`AverageRound`]
    /// documentation in `malachite-base`, and the returned [`Ordering`] indicates whether the
    /// result is less than, equal to, or greater than $a$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the average of `self` and `other` is not an integer.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::num::arithmetic::traits::AverageRound;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     (&Integer::from(-4)).average_round(Integer::from(-7), Floor),
    ///     (Integer::from(-6), Less)
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-4)).average_round(Integer::from(-7), Ceiling),
    ///     (Integer::from(-5), Greater)
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-4)).average_round(Integer::from(-7), Down),
    ///     (Integer::from(-5), Greater)
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-4)).average_round(Integer::from(-6), Exact),
    ///     (Integer::from(-5), Equal)
    /// );
    /// ```
    #[inline]
    fn average_round(self, other: Integer, rm: RoundingMode) -> (Integer, Ordering) {
        (self + other).shr_round(1u32, rm)
    }
}

impl AverageRound<&Integer> for &Integer {
    type Output = Integer;

    /// Computes the average (arithmetic mean) of two [`Integer`]s, taking both by reference and
    /// rounding according to a specified rounding mode. An [`Ordering`] is also returned,
    /// indicating whether the returned value is less than, equal to, or greater than the exact
    /// value.
    ///
    /// Let $a = \frac{x + y}{2}$. The rounding of an inexact average follows the [`AverageRound`]
    /// documentation in `malachite-base`, and the returned [`Ordering`] indicates whether the
    /// result is less than, equal to, or greater than $a$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the average of `self` and `other` is not an integer.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::num::arithmetic::traits::AverageRound;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     (&Integer::from(-4)).average_round(&Integer::from(-7), Floor),
    ///     (Integer::from(-6), Less)
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-4)).average_round(&Integer::from(-7), Ceiling),
    ///     (Integer::from(-5), Greater)
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-4)).average_round(&Integer::from(-7), Down),
    ///     (Integer::from(-5), Greater)
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-4)).average_round(&Integer::from(-6), Exact),
    ///     (Integer::from(-5), Equal)
    /// );
    /// ```
    #[inline]
    fn average_round(self, other: &Integer, rm: RoundingMode) -> (Integer, Ordering) {
        (self + other).shr_round(1u32, rm)
    }
}

impl AverageRoundAssign<Self> for Integer {
    /// Computes the average (arithmetic mean) of two [`Integer`]s, taking the [`Integer`] on the
    /// right-hand side by value, rounding according to a specified rounding mode, and replacing the
    /// first [`Integer`] with it. An [`Ordering`] is returned, indicating whether the assigned
    /// value is less than, equal to, or greater than the exact value.
    ///
    /// Let $a = \frac{x + y}{2}$. The rounding of an inexact average follows the [`AverageRound`]
    /// documentation in `malachite-base`, and the returned [`Ordering`] indicates whether the
    /// result is less than, equal to, or greater than $a$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the average of `self` and `other` is not an integer.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::num::arithmetic::traits::AverageRoundAssign;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(-4);
    /// assert_eq!(x.average_round_assign(Integer::from(-7), Floor), Less);
    /// assert_eq!(x, -6);
    /// ```
    #[inline]
    fn average_round_assign(&mut self, other: Self, rm: RoundingMode) -> Ordering {
        *self += other;
        self.shr_round_assign(1u32, rm)
    }
}

impl AverageRoundAssign<&Self> for Integer {
    /// Computes the average (arithmetic mean) of two [`Integer`]s, taking the [`Integer`] on the
    /// right-hand side by reference, rounding according to a specified rounding mode, and replacing
    /// the first [`Integer`] with it. An [`Ordering`] is returned, indicating whether the assigned
    /// value is less than, equal to, or greater than the exact value.
    ///
    /// Let $a = \frac{x + y}{2}$. The rounding of an inexact average follows the [`AverageRound`]
    /// documentation in `malachite-base`, and the returned [`Ordering`] indicates whether the
    /// result is less than, equal to, or greater than $a$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the average of `self` and `other` is not an integer.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::num::arithmetic::traits::AverageRoundAssign;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(-4);
    /// assert_eq!(x.average_round_assign(&Integer::from(-7), Floor), Less);
    /// assert_eq!(x, -6);
    /// ```
    #[inline]
    fn average_round_assign(&mut self, other: &Self, rm: RoundingMode) -> Ordering {
        *self += other;
        self.shr_round_assign(1u32, rm)
    }
}
