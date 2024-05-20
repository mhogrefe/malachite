// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::platform::Limb;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{
    DivAssignMod, DivMod, DivRound, DivRoundAssign, Parity,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::One;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
// quotient limbs of a `Limb` divided by the `Natural` and rounded according to a specified
// `RoundingMode`. The limb slice must have at least two elements and cannot have any trailing
// zeros. An `Ordering` is also returned, indicating whether the returned value is less than, equal
// to, or greater than the exact value.
//
// This function returns a `None` iff the rounding mode is `Exact` but the remainder of the division
// would be nonzero.
//
// Note that this function may only return `None`, `Some((0, Less))`, or `Some((1, Greater))`
// because of the restrictions placed on the input slice.
//
// # Worst-case complexity
// Constant time and additional memory.
pub_test! {limbs_limb_div_round_limbs(n: Limb, ds: &[Limb], rm: RoundingMode)
        -> Option<(Limb, Ordering)> {
    if n == 0 {
        Some((0, Equal))
    } else {
        match rm {
            Down | Floor => Some((0, Less)),
            Up | Ceiling => Some((1, Greater)),
            Exact => None,
            // 1 if 2 * n > Natural::from_limbs_asc(ds); otherwise, 0
            Nearest => Some(
                if ds.len() == 2 && ds[1] == 1 && n.get_highest_bit() && (n << 1) > ds[0] {
                    (1, Greater)
                } else {
                    (0, Less)
                },
            ),
        }
    }
}}

// Compares 2x and y
pub(crate) fn double_cmp(x: &Natural, y: &Natural) -> Ordering {
    (x.significant_bits() + 1)
        .cmp(&y.significant_bits())
        .then_with(|| x.cmp_normalized(y))
}

// assumes r != 0
fn div_round_nearest(q: Natural, r: &Natural, d: &Natural) -> (Natural, Ordering) {
    let compare = double_cmp(r, d);
    if compare == Greater || compare == Equal && q.odd() {
        (q.add_limb(1), Greater)
    } else {
        (q, Less)
    }
}

// assumes r != 0
fn div_round_assign_nearest(q: &mut Natural, r: &Natural, d: &Natural) -> Ordering {
    let compare = double_cmp(r, d);
    if compare == Greater || compare == Equal && q.odd() {
        *q += Natural::ONE;
        Greater
    } else {
        Less
    }
}

impl DivRound<Natural> for Natural {
    type Output = Natural;

    /// Divides a [`Natural`] by another [`Natural`], taking both by value and rounding according to
    /// a specified rounding mode. An [`Ordering`] is also returned, indicating whether the returned
    /// value is less than, equal to, or greater than the exact value.
    ///
    /// Let $q = \frac{x}{y}$, and let $g$ be the function that just returns the first element of
    /// the pair, without the [`Ordering`]:
    ///
    /// $$
    /// g(x, y, \mathrm{Down}) = g(x, y, \mathrm{Floor}) = \lfloor q \rfloor.
    /// $$
    ///
    /// $$
    /// g(x, y, \mathrm{Up}) = g(x, y, \mathrm{Ceiling}) = \lceil q \rceil.
    /// $$
    ///
    /// $$
    /// g(x, y, \mathrm{Nearest}) = \begin{cases}
    ///     \lfloor q \rfloor & \text{if} \\quad q - \lfloor q \rfloor < \frac{1}{2}, \\\\
    ///     \lceil q \rceil & \text{if} \\quad  q - \lfloor q \rfloor > \frac{1}{2}, \\\\
    ///     \lfloor q \rfloor &
    ///     \text{if} \\quad  q - \lfloor q \rfloor = \frac{1}{2}
    ///     \\ \text{and} \\ \lfloor q \rfloor \\ \text{is even}, \\\\
    ///     \lceil q \rceil &
    ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2}
    ///     \\ \text{and} \\ \lfloor q \rfloor \\ \text{is odd.}
    /// \end{cases}
    /// $$
    ///
    /// $g(x, y, \mathrm{Exact}) = q$, but panics if $q \notin \N$.
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
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(10u32).div_round(Natural::from(4u32), Down),
    ///     (Natural::from(2u32), Less)
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(12)
    ///         .div_round(Natural::from(3u32), Floor),
    ///     (Natural::from(333333333333u64), Less)
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).div_round(Natural::from(4u32), Up),
    ///     (Natural::from(3u32), Greater)
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(12)
    ///         .div_round(Natural::from(3u32), Ceiling),
    ///     (Natural::from(333333333334u64), Greater)
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).div_round(Natural::from(5u32), Exact),
    ///     (Natural::from(2u32), Equal)
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).div_round(Natural::from(3u32), Nearest),
    ///     (Natural::from(3u32), Less)
    /// );
    /// assert_eq!(
    ///     Natural::from(20u32).div_round(Natural::from(3u32), Nearest),
    ///     (Natural::from(7u32), Greater)
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).div_round(Natural::from(4u32), Nearest),
    ///     (Natural::from(2u32), Less)
    /// );
    /// assert_eq!(
    ///     Natural::from(14u32).div_round(Natural::from(4u32), Nearest),
    ///     (Natural::from(4u32), Greater)
    /// );
    /// ```
    #[inline]
    fn div_round(mut self, other: Natural, rm: RoundingMode) -> (Natural, Ordering) {
        let o = self.div_round_assign(other, rm);
        (self, o)
    }
}

impl<'a> DivRound<&'a Natural> for Natural {
    type Output = Natural;

    /// Divides a [`Natural`] by another [`Natural`], taking the first by value and the second by
    /// reference and rounding according to a specified rounding mode. An [`Ordering`] is also
    /// returned, indicating whether the returned value is less than, equal to, or greater than the
    /// exact value.
    ///
    /// Let $q = \frac{x}{y}$, and let $g$ be the function that just returns the first element of
    /// the pair, without the [`Ordering`]:
    ///
    /// $$
    /// g(x, y, \mathrm{Down}) = g(x, y, \mathrm{Floor}) = \lfloor q \rfloor.
    /// $$
    ///
    /// $$
    /// g(x, y, \mathrm{Up}) = g(x, y, \mathrm{Ceiling}) = \lceil q \rceil.
    /// $$
    ///
    /// $$
    /// g(x, y, \mathrm{Nearest}) = \begin{cases}
    ///     \lfloor q \rfloor & \text{if} \\quad q - \lfloor q \rfloor < \frac{1}{2}, \\\\
    ///     \lceil q \rceil & \text{if} \\quad  q - \lfloor q \rfloor > \frac{1}{2}, \\\\
    ///     \lfloor q \rfloor &
    ///     \text{if} \\quad  q - \lfloor q \rfloor = \frac{1}{2}
    ///     \\ \text{and} \\ \lfloor q \rfloor \\ \text{is even}, \\\\
    ///     \lceil q \rceil &
    ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2}
    ///     \\ \text{and} \\ \lfloor q \rfloor \\ \text{is odd.}
    /// \end{cases}
    /// $$
    ///
    /// $g(x, y, \mathrm{Exact}) = q$, but panics if $q \notin \N$.
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
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(10u32).div_round(&Natural::from(4u32), Down),
    ///     (Natural::from(2u32), Less)
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(12)
    ///         .div_round(&Natural::from(3u32), Floor),
    ///     (Natural::from(333333333333u64), Less)
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).div_round(&Natural::from(4u32), Up),
    ///     (Natural::from(3u32), Greater)
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(12)
    ///         .div_round(&Natural::from(3u32), Ceiling),
    ///     (Natural::from(333333333334u64), Greater)
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).div_round(&Natural::from(5u32), Exact),
    ///     (Natural::from(2u32), Equal)
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).div_round(&Natural::from(3u32), Nearest),
    ///     (Natural::from(3u32), Less)
    /// );
    /// assert_eq!(
    ///     Natural::from(20u32).div_round(&Natural::from(3u32), Nearest),
    ///     (Natural::from(7u32), Greater)
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).div_round(&Natural::from(4u32), Nearest),
    ///     (Natural::from(2u32), Less)
    /// );
    /// assert_eq!(
    ///     Natural::from(14u32).div_round(&Natural::from(4u32), Nearest),
    ///     (Natural::from(4u32), Greater)
    /// );
    /// ```
    #[inline]
    fn div_round(mut self, other: &'a Natural, rm: RoundingMode) -> (Natural, Ordering) {
        let o = self.div_round_assign(other, rm);
        (self, o)
    }
}

impl<'a> DivRound<Natural> for &'a Natural {
    type Output = Natural;

    /// Divides a [`Natural`] by another [`Natural`], taking the first by reference and the second
    /// by value and rounding according to a specified rounding mode. An [`Ordering`] is also
    /// returned, indicating whether the returned value is less than, equal to, or greater than the
    /// exact value.
    ///
    /// Let $q = \frac{x}{y}$, and let $g$ be the function that just returns the first element of
    /// the pair, without the [`Ordering`]:
    ///
    /// $$
    /// g(x, y, \mathrm{Down}) = g(x, y, \mathrm{Floor}) = \lfloor q \rfloor.
    /// $$
    ///
    /// $$
    /// g(x, y, \mathrm{Up}) = g(x, y, \mathrm{Ceiling}) = \lceil q \rceil.
    /// $$
    ///
    /// $$
    /// g(x, y, \mathrm{Nearest}) = \begin{cases}
    ///     \lfloor q \rfloor & \text{if} \\quad q - \lfloor q \rfloor < \frac{1}{2}, \\\\
    ///     \lceil q \rceil & \text{if} \\quad  q - \lfloor q \rfloor > \frac{1}{2}, \\\\
    ///     \lfloor q \rfloor &
    ///     \text{if} \\quad  q - \lfloor q \rfloor = \frac{1}{2}
    ///     \\ \text{and} \\ \lfloor q \rfloor \\ \text{is even}, \\\\
    ///     \lceil q \rceil &
    ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2}
    ///     \\ \text{and} \\ \lfloor q \rfloor \\ \text{is odd.}
    /// \end{cases}
    /// $$
    ///
    /// $g(x, y, \mathrm{Exact}) = q$, but panics if $q \notin \N$.
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
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(10u32)).div_round(Natural::from(4u32), Down),
    ///     (Natural::from(2u32), Less)
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32).pow(12)).div_round(Natural::from(3u32), Floor),
    ///     (Natural::from(333333333333u64), Less)
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32)).div_round(Natural::from(4u32), Up),
    ///     (Natural::from(3u32), Greater)
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32).pow(12)).div_round(Natural::from(3u32), Ceiling),
    ///     (Natural::from(333333333334u64), Greater)
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32)).div_round(Natural::from(5u32), Exact),
    ///     (Natural::from(2u32), Equal)
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32)).div_round(Natural::from(3u32), Nearest),
    ///     (Natural::from(3u32), Less)
    /// );
    /// assert_eq!(
    ///     (&Natural::from(20u32)).div_round(Natural::from(3u32), Nearest),
    ///     (Natural::from(7u32), Greater)
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32)).div_round(Natural::from(4u32), Nearest),
    ///     (Natural::from(2u32), Less)
    /// );
    /// assert_eq!(
    ///     (&Natural::from(14u32)).div_round(Natural::from(4u32), Nearest),
    ///     (Natural::from(4u32), Greater)
    /// );
    /// ```
    fn div_round(self, other: Natural, rm: RoundingMode) -> (Natural, Ordering) {
        let (q, r) = self.div_mod(&other);
        if r == 0 {
            (q, Equal)
        } else {
            match rm {
                Floor | Down => (q, Less),
                Ceiling | Up => (q.add_limb(1), Greater),
                Exact => panic!("Division is not exact"),
                Nearest => div_round_nearest(q, &r, &other),
            }
        }
    }
}

impl<'a, 'b> DivRound<&'b Natural> for &'a Natural {
    type Output = Natural;

    /// Divides a [`Natural`] by another [`Natural`], taking both by reference and rounding
    /// according to a specified rounding mode. An [`Ordering`] is also returned, indicating whether
    /// the returned value is less than, equal to, or greater than the exact value.
    ///
    /// Let $q = \frac{x}{y}$, and let $g$ be the function that just returns the first element of
    /// the pair, without the [`Ordering`]:
    ///
    /// $$
    /// g(x, y, \mathrm{Down}) = g(x, y, \mathrm{Floor}) = \lfloor q \rfloor.
    /// $$
    ///
    /// $$
    /// g(x, y, \mathrm{Up}) = g(x, y, \mathrm{Ceiling}) = \lceil q \rceil.
    /// $$
    ///
    /// $$
    /// g(x, y, \mathrm{Nearest}) = \begin{cases}
    ///     \lfloor q \rfloor & \text{if} \\quad q - \lfloor q \rfloor < \frac{1}{2}, \\\\
    ///     \lceil q \rceil & \text{if} \\quad  q - \lfloor q \rfloor > \frac{1}{2}, \\\\
    ///     \lfloor q \rfloor &
    ///     \text{if} \\quad  q - \lfloor q \rfloor = \frac{1}{2}
    ///     \\ \text{and} \\ \lfloor q \rfloor \\ \text{is even}, \\\\
    ///     \lceil q \rceil &
    ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2}
    ///     \\ \text{and} \\ \lfloor q \rfloor \\ \text{is odd.}
    /// \end{cases}
    /// $$
    ///
    /// $g(x, y, \mathrm{Exact}) = q$, but panics if $q \notin \N$.
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
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(10u32)).div_round(&Natural::from(4u32), Down),
    ///     (Natural::from(2u32), Less)
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32).pow(12)).div_round(&Natural::from(3u32), Floor),
    ///     (Natural::from(333333333333u64), Less)
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32)).div_round(&Natural::from(4u32), Up),
    ///     (Natural::from(3u32), Greater)
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32).pow(12)).div_round(&Natural::from(3u32), Ceiling),
    ///     (Natural::from(333333333334u64), Greater)
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32)).div_round(&Natural::from(5u32), Exact),
    ///     (Natural::from(2u32), Equal)
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32)).div_round(&Natural::from(3u32), Nearest),
    ///     (Natural::from(3u32), Less)
    /// );
    /// assert_eq!(
    ///     (&Natural::from(20u32)).div_round(&Natural::from(3u32), Nearest),
    ///     (Natural::from(7u32), Greater)
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32)).div_round(&Natural::from(4u32), Nearest),
    ///     (Natural::from(2u32), Less)
    /// );
    /// assert_eq!(
    ///     (&Natural::from(14u32)).div_round(&Natural::from(4u32), Nearest),
    ///     (Natural::from(4u32), Greater)
    /// );
    /// ```
    fn div_round(self, other: &'b Natural, rm: RoundingMode) -> (Natural, Ordering) {
        let (q, r) = self.div_mod(other);
        if r == 0 {
            (q, Equal)
        } else {
            match rm {
                Floor | Down => (q, Less),
                Ceiling | Up => (q.add_limb(1), Greater),
                Exact => panic!("Division is not exact: {self} / {other}"),
                Nearest => div_round_nearest(q, &r, other),
            }
        }
    }
}

impl DivRoundAssign<Natural> for Natural {
    /// Divides a [`Natural`] by another [`Natural`] in place, taking the [`Natural`] on the
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
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut n = Natural::from(10u32);
    /// assert_eq!(n.div_round_assign(Natural::from(4u32), Down), Less);
    /// assert_eq!(n, 2);
    ///
    /// let mut n = Natural::from(10u32).pow(12);
    /// assert_eq!(n.div_round_assign(Natural::from(3u32), Floor), Less);
    /// assert_eq!(n, 333333333333u64);
    ///
    /// let mut n = Natural::from(10u32);
    /// assert_eq!(n.div_round_assign(Natural::from(4u32), Up), Greater);
    /// assert_eq!(n, 3);
    ///
    /// let mut n = Natural::from(10u32).pow(12);
    /// assert_eq!(n.div_round_assign(&Natural::from(3u32), Ceiling), Greater);
    /// assert_eq!(n, 333333333334u64);
    ///
    /// let mut n = Natural::from(10u32);
    /// assert_eq!(n.div_round_assign(Natural::from(5u32), Exact), Equal);
    /// assert_eq!(n, 2);
    ///
    /// let mut n = Natural::from(10u32);
    /// assert_eq!(n.div_round_assign(Natural::from(3u32), Nearest), Less);
    /// assert_eq!(n, 3);
    ///
    /// let mut n = Natural::from(20u32);
    /// assert_eq!(n.div_round_assign(Natural::from(3u32), Nearest), Greater);
    /// assert_eq!(n, 7);
    ///
    /// let mut n = Natural::from(10u32);
    /// assert_eq!(n.div_round_assign(Natural::from(4u32), Nearest), Less);
    /// assert_eq!(n, 2);
    ///
    /// let mut n = Natural::from(14u32);
    /// assert_eq!(n.div_round_assign(Natural::from(4u32), Nearest), Greater);
    /// assert_eq!(n, 4);
    /// ```
    fn div_round_assign(&mut self, other: Natural, rm: RoundingMode) -> Ordering {
        let r = self.div_assign_mod(&other);
        if r == 0 {
            Equal
        } else {
            match rm {
                Floor | Down => Less,
                Ceiling | Up => {
                    *self += Natural::ONE;
                    Greater
                }
                Exact => panic!("Division is not exact"),
                Nearest => div_round_assign_nearest(self, &r, &other),
            }
        }
    }
}

impl<'a> DivRoundAssign<&'a Natural> for Natural {
    /// Divides a [`Natural`] by another [`Natural`] in place, taking the [`Natural`] on the
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
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut n = Natural::from(10u32);
    /// assert_eq!(n.div_round_assign(&Natural::from(4u32), Down), Less);
    /// assert_eq!(n, 2);
    ///
    /// let mut n = Natural::from(10u32).pow(12);
    /// assert_eq!(n.div_round_assign(&Natural::from(3u32), Floor), Less);
    /// assert_eq!(n, 333333333333u64);
    ///
    /// let mut n = Natural::from(10u32);
    /// assert_eq!(n.div_round_assign(&Natural::from(4u32), Up), Greater);
    /// assert_eq!(n, 3);
    ///
    /// let mut n = Natural::from(10u32).pow(12);
    /// assert_eq!(n.div_round_assign(&Natural::from(3u32), Ceiling), Greater);
    /// assert_eq!(n, 333333333334u64);
    ///
    /// let mut n = Natural::from(10u32);
    /// assert_eq!(n.div_round_assign(&Natural::from(5u32), Exact), Equal);
    /// assert_eq!(n, 2);
    ///
    /// let mut n = Natural::from(10u32);
    /// assert_eq!(n.div_round_assign(&Natural::from(3u32), Nearest), Less);
    /// assert_eq!(n, 3);
    ///
    /// let mut n = Natural::from(20u32);
    /// assert_eq!(n.div_round_assign(&Natural::from(3u32), Nearest), Greater);
    /// assert_eq!(n, 7);
    ///
    /// let mut n = Natural::from(10u32);
    /// assert_eq!(n.div_round_assign(&Natural::from(4u32), Nearest), Less);
    /// assert_eq!(n, 2);
    ///
    /// let mut n = Natural::from(14u32);
    /// assert_eq!(n.div_round_assign(&Natural::from(4u32), Nearest), Greater);
    /// assert_eq!(n, 4);
    /// ```
    fn div_round_assign(&mut self, other: &'a Natural, rm: RoundingMode) -> Ordering {
        let r = self.div_assign_mod(other);
        if r == 0 {
            Equal
        } else {
            match rm {
                Floor | Down => Less,
                Ceiling | Up => {
                    *self += Natural::ONE;
                    Greater
                }
                Exact => panic!("Division is not exact"),
                Nearest => div_round_assign_nearest(self, &r, other),
            }
        }
    }
}
