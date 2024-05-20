// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use core::cmp::Ordering;
use malachite_base::num::arithmetic::traits::{DivRound, DivRoundAssign};
use malachite_base::rounding_modes::RoundingMode;

impl DivRound<Integer> for Integer {
    type Output = Integer;

    /// Divides an [`Integer`] by another [`Integer`], taking both by value and rounding according
    /// to a specified rounding mode. An [`Ordering`] is also returned, indicating whether the
    /// returned value is less than, equal to, or greater than the exact value.
    ///
    /// Let $q = \frac{x}{y}$, and let $g$ be the function that just returns the first element of
    /// the pair, without the [`Ordering`]:
    ///
    /// $$
    /// g(x, y, \mathrm{Down}) = \operatorname{sgn}(q) \lfloor |q| \rfloor.
    /// $$
    ///
    /// $$
    /// g(x, y, \mathrm{Up}) = \operatorname{sgn}(q) \lceil |q| \rceil.
    /// $$
    ///
    /// $$
    /// g(x, y, \mathrm{Floor}) = \lfloor q \rfloor.
    /// $$
    ///
    /// $$
    /// g(x, y, \mathrm{Ceiling}) = \lceil q \rceil.
    /// $$
    ///
    /// $$
    /// g(x, y, \mathrm{Nearest}) = \begin{cases}
    ///     \lfloor q \rfloor & \text{if} \\quad q - \lfloor q \rfloor < \frac{1}{2}, \\\\
    ///     \lceil q \rceil & q - \lfloor q \rfloor > \frac{1}{2}, \\\\
    ///     \lfloor q \rfloor &
    ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2} \\ \text{and}
    ///     \\ \lfloor q \rfloor \\ \text{is even}, \\\\
    ///     \lceil q \rceil &
    ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2} \\ \text{and}
    ///     \\ \lfloor q \rfloor \\ \text{is odd.}
    /// \end{cases}
    /// $$
    ///
    /// $g(x, y, \mathrm{Exact}) = q$, but panics if $q \notin \Z$.
    ///
    /// Then $f(x, y, r) = (g(x, y, r), \operatorname{cmp}(g(x, y, r), q))$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is zero, or if `rm` is `Exact` but `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::num::arithmetic::traits::{DivRound, Pow};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     Integer::from(-10).div_round(Integer::from(4), Down),
    ///     (Integer::from(-2), Greater)
    /// );
    /// assert_eq!(
    ///     (-Integer::from(10u32).pow(12)).div_round(Integer::from(3), Floor),
    ///     (Integer::from(-333333333334i64), Less)
    /// );
    /// assert_eq!(
    ///     Integer::from(-10).div_round(Integer::from(4), Up),
    ///     (Integer::from(-3), Less)
    /// );
    /// assert_eq!(
    ///     (-Integer::from(10u32).pow(12)).div_round(Integer::from(3), Ceiling),
    ///     (Integer::from(-333333333333i64), Greater)
    /// );
    /// assert_eq!(
    ///     Integer::from(-10).div_round(Integer::from(5), Exact),
    ///     (Integer::from(-2), Equal)
    /// );
    /// assert_eq!(
    ///     Integer::from(-10).div_round(Integer::from(3), Nearest),
    ///     (Integer::from(-3), Greater)
    /// );
    /// assert_eq!(
    ///     Integer::from(-20).div_round(Integer::from(3), Nearest),
    ///     (Integer::from(-7), Less)
    /// );
    /// assert_eq!(
    ///     Integer::from(-10).div_round(Integer::from(4), Nearest),
    ///     (Integer::from(-2), Greater)
    /// );
    /// assert_eq!(
    ///     Integer::from(-14).div_round(Integer::from(4), Nearest),
    ///     (Integer::from(-4), Less)
    /// );
    ///
    /// assert_eq!(
    ///     Integer::from(-10).div_round(Integer::from(-4), Down),
    ///     (Integer::from(2), Less)
    /// );
    /// assert_eq!(
    ///     (-Integer::from(10u32).pow(12)).div_round(Integer::from(-3), Floor),
    ///     (Integer::from(333333333333i64), Less)
    /// );
    /// assert_eq!(
    ///     Integer::from(-10).div_round(Integer::from(-4), Up),
    ///     (Integer::from(3), Greater)
    /// );
    /// assert_eq!(
    ///     (-Integer::from(10u32).pow(12)).div_round(Integer::from(-3), Ceiling),
    ///     (Integer::from(333333333334i64), Greater)
    /// );
    /// assert_eq!(
    ///     Integer::from(-10).div_round(Integer::from(-5), Exact),
    ///     (Integer::from(2), Equal)
    /// );
    /// assert_eq!(
    ///     Integer::from(-10).div_round(Integer::from(-3), Nearest),
    ///     (Integer::from(3), Less)
    /// );
    /// assert_eq!(
    ///     Integer::from(-20).div_round(Integer::from(-3), Nearest),
    ///     (Integer::from(7), Greater)
    /// );
    /// assert_eq!(
    ///     Integer::from(-10).div_round(Integer::from(-4), Nearest),
    ///     (Integer::from(2), Less)
    /// );
    /// assert_eq!(
    ///     Integer::from(-14).div_round(Integer::from(-4), Nearest),
    ///     (Integer::from(4), Greater)
    /// );
    /// ```
    #[inline]
    fn div_round(mut self, other: Integer, rm: RoundingMode) -> (Integer, Ordering) {
        let o = self.div_round_assign(other, rm);
        (self, o)
    }
}

impl<'a> DivRound<&'a Integer> for Integer {
    type Output = Integer;

    /// Divides an [`Integer`] by another [`Integer`], taking the first by value and the second by
    /// reference and rounding according to a specified rounding mode. An [`Ordering`] is also
    /// returned, indicating whether the returned value is less than, equal to, or greater than the
    /// exact value.
    ///
    /// Let $q = \frac{x}{y}$, and let $g$ be the function that just returns the first element of
    /// the pair, without the [`Ordering`]:
    ///
    /// $$
    /// g(x, y, \mathrm{Down}) = \operatorname{sgn}(q) \lfloor |q| \rfloor.
    /// $$
    ///
    /// $$
    /// g(x, y, \mathrm{Up}) = \operatorname{sgn}(q) \lceil |q| \rceil.
    /// $$
    ///
    /// $$
    /// g(x, y, \mathrm{Floor}) = \lfloor q \rfloor.
    /// $$
    ///
    /// $$
    /// g(x, y, \mathrm{Ceiling}) = \lceil q \rceil.
    /// $$
    ///
    /// $$
    /// g(x, y, \mathrm{Nearest}) = \begin{cases}
    ///     \lfloor q \rfloor & \text{if} \\quad q - \lfloor q \rfloor < \frac{1}{2}, \\\\
    ///     \lceil q \rceil & q - \lfloor q \rfloor > \frac{1}{2}, \\\\
    ///     \lfloor q \rfloor &
    ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2} \\ \text{and}
    ///     \\ \lfloor q \rfloor \\ \text{is even}, \\\\
    ///     \lceil q \rceil &
    ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2} \\ \text{and}
    ///     \\ \lfloor q \rfloor \\ \text{is odd.}
    /// \end{cases}
    /// $$
    ///
    /// $g(x, y, \mathrm{Exact}) = q$, but panics if $q \notin \Z$.
    ///
    /// Then $f(x, y, r) = (g(x, y, r), \operatorname{cmp}(g(x, y, r), q))$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is zero, or if `rm` is `Exact` but `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::num::arithmetic::traits::{DivRound, Pow};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     Integer::from(-10).div_round(&Integer::from(4), Down),
    ///     (Integer::from(-2), Greater)
    /// );
    /// assert_eq!(
    ///     (-Integer::from(10u32).pow(12)).div_round(&Integer::from(3), Floor),
    ///     (Integer::from(-333333333334i64), Less)
    /// );
    /// assert_eq!(
    ///     Integer::from(-10).div_round(&Integer::from(4), Up),
    ///     (Integer::from(-3), Less)
    /// );
    /// assert_eq!(
    ///     (-Integer::from(10u32).pow(12)).div_round(&Integer::from(3), Ceiling),
    ///     (Integer::from(-333333333333i64), Greater)
    /// );
    /// assert_eq!(
    ///     Integer::from(-10).div_round(&Integer::from(5), Exact),
    ///     (Integer::from(-2), Equal)
    /// );
    /// assert_eq!(
    ///     Integer::from(-10).div_round(&Integer::from(3), Nearest),
    ///     (Integer::from(-3), Greater)
    /// );
    /// assert_eq!(
    ///     Integer::from(-20).div_round(&Integer::from(3), Nearest),
    ///     (Integer::from(-7), Less)
    /// );
    /// assert_eq!(
    ///     Integer::from(-10).div_round(&Integer::from(4), Nearest),
    ///     (Integer::from(-2), Greater)
    /// );
    /// assert_eq!(
    ///     Integer::from(-14).div_round(&Integer::from(4), Nearest),
    ///     (Integer::from(-4), Less)
    /// );
    ///
    /// assert_eq!(
    ///     Integer::from(-10).div_round(&Integer::from(-4), Down),
    ///     (Integer::from(2), Less)
    /// );
    /// assert_eq!(
    ///     (-Integer::from(10u32).pow(12)).div_round(&Integer::from(-3), Floor),
    ///     (Integer::from(333333333333i64), Less)
    /// );
    /// assert_eq!(
    ///     Integer::from(-10).div_round(&Integer::from(-4), Up),
    ///     (Integer::from(3), Greater)
    /// );
    /// assert_eq!(
    ///     (-Integer::from(10u32).pow(12)).div_round(&Integer::from(-3), Ceiling),
    ///     (Integer::from(333333333334i64), Greater)
    /// );
    /// assert_eq!(
    ///     Integer::from(-10).div_round(&Integer::from(-5), Exact),
    ///     (Integer::from(2), Equal)
    /// );
    /// assert_eq!(
    ///     Integer::from(-10).div_round(&Integer::from(-3), Nearest),
    ///     (Integer::from(3), Less)
    /// );
    /// assert_eq!(
    ///     Integer::from(-20).div_round(&Integer::from(-3), Nearest),
    ///     (Integer::from(7), Greater)
    /// );
    /// assert_eq!(
    ///     Integer::from(-10).div_round(&Integer::from(-4), Nearest),
    ///     (Integer::from(2), Less)
    /// );
    /// assert_eq!(
    ///     Integer::from(-14).div_round(&Integer::from(-4), Nearest),
    ///     (Integer::from(4), Greater)
    /// );
    /// ```
    #[inline]
    fn div_round(mut self, other: &'a Integer, rm: RoundingMode) -> (Integer, Ordering) {
        let o = self.div_round_assign(other, rm);
        (self, o)
    }
}

impl<'a> DivRound<Integer> for &'a Integer {
    type Output = Integer;

    /// Divides an [`Integer`] by another [`Integer`], taking the first by reference and the second
    /// by value and rounding according to a specified rounding mode. An [`Ordering`] is also
    /// returned, indicating whether the returned value is less than, equal to, or greater than the
    /// exact value.
    ///
    /// Let $q = \frac{x}{y}$, and let $g$ be the function that just returns the first element of
    /// the pair, without the [`Ordering`]:
    ///
    /// $$
    /// g(x, y, \mathrm{Down}) = \operatorname{sgn}(q) \lfloor |q| \rfloor.
    /// $$
    ///
    /// $$
    /// g(x, y, \mathrm{Up}) = \operatorname{sgn}(q) \lceil |q| \rceil.
    /// $$
    ///
    /// $$
    /// g(x, y, \mathrm{Floor}) = \lfloor q \rfloor.
    /// $$
    ///
    /// $$
    /// g(x, y, \mathrm{Ceiling}) = \lceil q \rceil.
    /// $$
    ///
    /// $$
    /// g(x, y, \mathrm{Nearest}) = \begin{cases}
    ///     \lfloor q \rfloor & \text{if} \\quad q - \lfloor q \rfloor < \frac{1}{2}, \\\\
    ///     \lceil q \rceil & q - \lfloor q \rfloor > \frac{1}{2}, \\\\
    ///     \lfloor q \rfloor &
    ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2} \\ \text{and}
    ///     \\ \lfloor q \rfloor \\ \text{is even}, \\\\
    ///     \lceil q \rceil &
    ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2} \\ \text{and}
    ///     \\ \lfloor q \rfloor \\ \text{is odd.}
    /// \end{cases}
    /// $$
    ///
    /// $g(x, y, \mathrm{Exact}) = q$, but panics if $q \notin \Z$.
    ///
    /// Then $f(x, y, r) = (g(x, y, r), \operatorname{cmp}(g(x, y, r), q))$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is zero, or if `rm` is `Exact` but `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::num::arithmetic::traits::{DivRound, Pow};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     (&Integer::from(-10)).div_round(Integer::from(4), Down),
    ///     (Integer::from(-2), Greater)
    /// );
    /// assert_eq!(
    ///     (&-Integer::from(10u32).pow(12)).div_round(Integer::from(3), Floor),
    ///     (Integer::from(-333333333334i64), Less)
    /// );
    /// assert_eq!(
    ///     Integer::from(-10).div_round(Integer::from(4), Up),
    ///     (Integer::from(-3), Less)
    /// );
    /// assert_eq!(
    ///     (&-Integer::from(10u32).pow(12)).div_round(Integer::from(3), Ceiling),
    ///     (Integer::from(-333333333333i64), Greater)
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10)).div_round(Integer::from(5), Exact),
    ///     (Integer::from(-2), Equal)
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10)).div_round(Integer::from(3), Nearest),
    ///     (Integer::from(-3), Greater)
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-20)).div_round(Integer::from(3), Nearest),
    ///     (Integer::from(-7), Less)
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10)).div_round(Integer::from(4), Nearest),
    ///     (Integer::from(-2), Greater)
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-14)).div_round(Integer::from(4), Nearest),
    ///     (Integer::from(-4), Less)
    /// );
    ///
    /// assert_eq!(
    ///     (&Integer::from(-10)).div_round(Integer::from(-4), Down),
    ///     (Integer::from(2), Less)
    /// );
    /// assert_eq!(
    ///     (&-Integer::from(10u32).pow(12)).div_round(Integer::from(-3), Floor),
    ///     (Integer::from(333333333333i64), Less)
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10)).div_round(Integer::from(-4), Up),
    ///     (Integer::from(3), Greater)
    /// );
    /// assert_eq!(
    ///     (&-Integer::from(10u32).pow(12)).div_round(Integer::from(-3), Ceiling),
    ///     (Integer::from(333333333334i64), Greater)
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10)).div_round(Integer::from(-5), Exact),
    ///     (Integer::from(2), Equal)
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10)).div_round(Integer::from(-3), Nearest),
    ///     (Integer::from(3), Less)
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-20)).div_round(Integer::from(-3), Nearest),
    ///     (Integer::from(7), Greater)
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10)).div_round(Integer::from(-4), Nearest),
    ///     (Integer::from(2), Less)
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-14)).div_round(Integer::from(-4), Nearest),
    ///     (Integer::from(4), Greater)
    /// );
    /// ```
    fn div_round(self, other: Integer, rm: RoundingMode) -> (Integer, Ordering) {
        let q_sign = self.sign == other.sign;
        let (q_abs, o) = (&self.abs).div_round(other.abs, if q_sign { rm } else { -rm });
        (
            Integer::from_sign_and_abs(q_sign, q_abs),
            if q_sign { o } else { o.reverse() },
        )
    }
}

impl<'a, 'b> DivRound<&'b Integer> for &'a Integer {
    type Output = Integer;

    /// Divides an [`Integer`] by another [`Integer`], taking both by reference and rounding
    /// according to a specified rounding mode. An [`Ordering`] is also returned, indicating whether
    /// the returned value is less than, equal to, or greater than the exact value.
    ///
    /// Let $q = \frac{x}{y}$, and let $g$ be the function that just returns the first element of
    /// the pair, without the [`Ordering`]:
    ///
    /// $$
    /// g(x, y, \mathrm{Down}) = \operatorname{sgn}(q) \lfloor |q| \rfloor.
    /// $$
    ///
    /// $$
    /// g(x, y, \mathrm{Up}) = \operatorname{sgn}(q) \lceil |q| \rceil.
    /// $$
    ///
    /// $$
    /// g(x, y, \mathrm{Floor}) = \lfloor q \rfloor.
    /// $$
    ///
    /// $$
    /// g(x, y, \mathrm{Ceiling}) = \lceil q \rceil.
    /// $$
    ///
    /// $$
    /// g(x, y, \mathrm{Nearest}) = \begin{cases}
    ///     \lfloor q \rfloor & \text{if} \\quad q - \lfloor q \rfloor < \frac{1}{2}, \\\\
    ///     \lceil q \rceil & q - \lfloor q \rfloor > \frac{1}{2}, \\\\
    ///     \lfloor q \rfloor &
    ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2} \\ \text{and}
    ///     \\ \lfloor q \rfloor \\ \text{is even}, \\\\
    ///     \lceil q \rceil &
    ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2} \\ \text{and}
    ///     \\ \lfloor q \rfloor \\ \text{is odd.}
    /// \end{cases}
    /// $$
    ///
    /// $g(x, y, \mathrm{Exact}) = q$, but panics if $q \notin \Z$.
    ///
    /// Then $f(x, y, r) = (g(x, y, r), \operatorname{cmp}(g(x, y, r), q))$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is zero, or if `rm` is `Exact` but `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::num::arithmetic::traits::{DivRound, Pow};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     (&Integer::from(-10)).div_round(&Integer::from(4), Down),
    ///     (Integer::from(-2), Greater)
    /// );
    /// assert_eq!(
    ///     (&-Integer::from(10u32).pow(12)).div_round(&Integer::from(3), Floor),
    ///     (Integer::from(-333333333334i64), Less)
    /// );
    /// assert_eq!(
    ///     Integer::from(-10).div_round(&Integer::from(4), Up),
    ///     (Integer::from(-3), Less)
    /// );
    /// assert_eq!(
    ///     (&-Integer::from(10u32).pow(12)).div_round(&Integer::from(3), Ceiling),
    ///     (Integer::from(-333333333333i64), Greater)
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10)).div_round(&Integer::from(5), Exact),
    ///     (Integer::from(-2), Equal)
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10)).div_round(&Integer::from(3), Nearest),
    ///     (Integer::from(-3), Greater)
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-20)).div_round(&Integer::from(3), Nearest),
    ///     (Integer::from(-7), Less)
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10)).div_round(&Integer::from(4), Nearest),
    ///     (Integer::from(-2), Greater)
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-14)).div_round(&Integer::from(4), Nearest),
    ///     (Integer::from(-4), Less)
    /// );
    ///
    /// assert_eq!(
    ///     (&Integer::from(-10)).div_round(&Integer::from(-4), Down),
    ///     (Integer::from(2), Less)
    /// );
    /// assert_eq!(
    ///     (&-Integer::from(10u32).pow(12)).div_round(&Integer::from(-3), Floor),
    ///     (Integer::from(333333333333i64), Less)
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10)).div_round(&Integer::from(-4), Up),
    ///     (Integer::from(3), Greater)
    /// );
    /// assert_eq!(
    ///     (&-Integer::from(10u32).pow(12)).div_round(&Integer::from(-3), Ceiling),
    ///     (Integer::from(333333333334i64), Greater)
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10)).div_round(&Integer::from(-5), Exact),
    ///     (Integer::from(2), Equal)
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10)).div_round(&Integer::from(-3), Nearest),
    ///     (Integer::from(3), Less)
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-20)).div_round(&Integer::from(-3), Nearest),
    ///     (Integer::from(7), Greater)
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10)).div_round(&Integer::from(-4), Nearest),
    ///     (Integer::from(2), Less)
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-14)).div_round(&Integer::from(-4), Nearest),
    ///     (Integer::from(4), Greater)
    /// );
    /// ```
    fn div_round(self, other: &'b Integer, rm: RoundingMode) -> (Integer, Ordering) {
        let q_sign = self.sign == other.sign;
        let (q_abs, o) = (&self.abs).div_round(&other.abs, if q_sign { rm } else { -rm });
        (
            Integer::from_sign_and_abs(q_sign, q_abs),
            if q_sign { o } else { o.reverse() },
        )
    }
}

impl DivRoundAssign<Integer> for Integer {
    /// Divides an [`Integer`] by another [`Integer`] in place, taking the [`Integer`] on the
    /// right-hand side by value and rounding according to a specified rounding mode. An
    /// [`Ordering`] is returned, indicating whether the assigned value is less than, equal to, or
    /// greater than the exact value.
    ///
    /// See the [`DivRound`] documentation for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is zero, or if `rm` is `Exact` but `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::num::arithmetic::traits::{DivRoundAssign, Pow};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut n = Integer::from(-10);
    /// assert_eq!(n.div_round_assign(Integer::from(4), Down), Greater);
    /// assert_eq!(n, -2);
    ///
    /// let mut n = -Integer::from(10u32).pow(12);
    /// assert_eq!(n.div_round_assign(Integer::from(3), Floor), Less);
    /// assert_eq!(n, -333333333334i64);
    ///
    /// let mut n = Integer::from(-10);
    /// assert_eq!(n.div_round_assign(Integer::from(4), Up), Less);
    /// assert_eq!(n, -3);
    ///
    /// let mut n = -Integer::from(10u32).pow(12);
    /// assert_eq!(n.div_round_assign(Integer::from(3), Ceiling), Greater);
    /// assert_eq!(n, -333333333333i64);
    ///
    /// let mut n = Integer::from(-10);
    /// assert_eq!(n.div_round_assign(Integer::from(5), Exact), Equal);
    /// assert_eq!(n, -2);
    ///
    /// let mut n = Integer::from(-10);
    /// assert_eq!(n.div_round_assign(Integer::from(3), Nearest), Greater);
    /// assert_eq!(n, -3);
    ///
    /// let mut n = Integer::from(-20);
    /// assert_eq!(n.div_round_assign(Integer::from(3), Nearest), Less);
    /// assert_eq!(n, -7);
    ///
    /// let mut n = Integer::from(-10);
    /// assert_eq!(n.div_round_assign(Integer::from(4), Nearest), Greater);
    /// assert_eq!(n, -2);
    ///
    /// let mut n = Integer::from(-14);
    /// assert_eq!(n.div_round_assign(Integer::from(4), Nearest), Less);
    /// assert_eq!(n, -4);
    ///
    /// let mut n = Integer::from(-10);
    /// assert_eq!(n.div_round_assign(Integer::from(-4), Down), Less);
    /// assert_eq!(n, 2);
    ///
    /// let mut n = -Integer::from(10u32).pow(12);
    /// assert_eq!(n.div_round_assign(Integer::from(-3), Floor), Less);
    /// assert_eq!(n, 333333333333i64);
    ///
    /// let mut n = Integer::from(-10);
    /// assert_eq!(n.div_round_assign(Integer::from(-4), Up), Greater);
    /// assert_eq!(n, 3);
    ///
    /// let mut n = -Integer::from(10u32).pow(12);
    /// assert_eq!(n.div_round_assign(Integer::from(-3), Ceiling), Greater);
    /// assert_eq!(n, 333333333334i64);
    ///
    /// let mut n = Integer::from(-10);
    /// assert_eq!(n.div_round_assign(Integer::from(-5), Exact), Equal);
    /// assert_eq!(n, 2);
    ///
    /// let mut n = Integer::from(-10);
    /// assert_eq!(n.div_round_assign(Integer::from(-3), Nearest), Less);
    /// assert_eq!(n, 3);
    ///
    /// let mut n = Integer::from(-20);
    /// assert_eq!(n.div_round_assign(Integer::from(-3), Nearest), Greater);
    /// assert_eq!(n, 7);
    ///
    /// let mut n = Integer::from(-10);
    /// assert_eq!(n.div_round_assign(Integer::from(-4), Nearest), Less);
    /// assert_eq!(n, 2);
    ///
    /// let mut n = Integer::from(-14);
    /// assert_eq!(n.div_round_assign(Integer::from(-4), Nearest), Greater);
    /// assert_eq!(n, 4);
    /// ```
    fn div_round_assign(&mut self, other: Integer, rm: RoundingMode) -> Ordering {
        let q_sign = self.sign == other.sign;
        let o = self
            .abs
            .div_round_assign(other.abs, if q_sign { rm } else { -rm });
        self.sign = q_sign || self.abs == 0;
        if q_sign {
            o
        } else {
            o.reverse()
        }
    }
}

impl<'a> DivRoundAssign<&'a Integer> for Integer {
    /// Divides an [`Integer`] by another [`Integer`] in place, taking the [`Integer`] on the
    /// right-hand side by reference and rounding according to a specified rounding mode. An
    /// [`Ordering`] is returned, indicating whether the assigned value is less than, equal to, or
    /// greater than the exact value.
    ///
    /// See the [`DivRound`] documentation for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is zero, or if `rm` is `Exact` but `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::num::arithmetic::traits::{DivRoundAssign, Pow};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut n = Integer::from(-10);
    /// assert_eq!(n.div_round_assign(&Integer::from(4), Down), Greater);
    /// assert_eq!(n, -2);
    ///
    /// let mut n = -Integer::from(10u32).pow(12);
    /// assert_eq!(n.div_round_assign(&Integer::from(3), Floor), Less);
    /// assert_eq!(n, -333333333334i64);
    ///
    /// let mut n = Integer::from(-10);
    /// assert_eq!(n.div_round_assign(&Integer::from(4), Up), Less);
    /// assert_eq!(n, -3);
    ///
    /// let mut n = -Integer::from(10u32).pow(12);
    /// assert_eq!(n.div_round_assign(&Integer::from(3), Ceiling), Greater);
    /// assert_eq!(n, -333333333333i64);
    ///
    /// let mut n = Integer::from(-10);
    /// assert_eq!(n.div_round_assign(&Integer::from(5), Exact), Equal);
    /// assert_eq!(n, -2);
    ///
    /// let mut n = Integer::from(-10);
    /// assert_eq!(n.div_round_assign(&Integer::from(3), Nearest), Greater);
    /// assert_eq!(n, -3);
    ///
    /// let mut n = Integer::from(-20);
    /// assert_eq!(n.div_round_assign(&Integer::from(3), Nearest), Less);
    /// assert_eq!(n, -7);
    ///
    /// let mut n = Integer::from(-10);
    /// assert_eq!(n.div_round_assign(&Integer::from(4), Nearest), Greater);
    /// assert_eq!(n, -2);
    ///
    /// let mut n = Integer::from(-14);
    /// assert_eq!(n.div_round_assign(&Integer::from(4), Nearest), Less);
    /// assert_eq!(n, -4);
    ///
    /// let mut n = Integer::from(-10);
    /// assert_eq!(n.div_round_assign(&Integer::from(-4), Down), Less);
    /// assert_eq!(n, 2);
    ///
    /// let mut n = -Integer::from(10u32).pow(12);
    /// assert_eq!(n.div_round_assign(&Integer::from(-3), Floor), Less);
    /// assert_eq!(n, 333333333333i64);
    ///
    /// let mut n = Integer::from(-10);
    /// assert_eq!(n.div_round_assign(&Integer::from(-4), Up), Greater);
    /// assert_eq!(n, 3);
    ///
    /// let mut n = -Integer::from(10u32).pow(12);
    /// assert_eq!(n.div_round_assign(&Integer::from(-3), Ceiling), Greater);
    /// assert_eq!(n, 333333333334i64);
    ///
    /// let mut n = Integer::from(-10);
    /// assert_eq!(n.div_round_assign(&Integer::from(-5), Exact), Equal);
    /// assert_eq!(n, 2);
    ///
    /// let mut n = Integer::from(-10);
    /// assert_eq!(n.div_round_assign(&Integer::from(-3), Nearest), Less);
    /// assert_eq!(n, 3);
    ///
    /// let mut n = Integer::from(-20);
    /// assert_eq!(n.div_round_assign(&Integer::from(-3), Nearest), Greater);
    /// assert_eq!(n, 7);
    ///
    /// let mut n = Integer::from(-10);
    /// assert_eq!(n.div_round_assign(&Integer::from(-4), Nearest), Less);
    /// assert_eq!(n, 2);
    ///
    /// let mut n = Integer::from(-14);
    /// assert_eq!(n.div_round_assign(&Integer::from(-4), Nearest), Greater);
    /// assert_eq!(n, 4);
    /// ```
    fn div_round_assign(&mut self, other: &'a Integer, rm: RoundingMode) -> Ordering {
        let q_sign = self.sign == other.sign;
        let o = self
            .abs
            .div_round_assign(&other.abs, if q_sign { rm } else { -rm });
        self.sign = q_sign || self.abs == 0;
        if q_sign {
            o
        } else {
            o.reverse()
        }
    }
}
