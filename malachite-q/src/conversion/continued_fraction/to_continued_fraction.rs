// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::conversion::traits::ContinuedFraction;
use crate::Rational;
use core::mem::swap;
use malachite_base::num::arithmetic::traits::{DivMod, Floor};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

/// An iterator that produces the continued fraction of a [`Rational`].
///
/// See [`continued_fraction`](Rational::continued_fraction) for more information.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RationalContinuedFraction {
    numerator: Natural,
    denominator: Natural,
}

impl RationalContinuedFraction {
    pub_crate_test! {is_done(&self) -> bool {
        self.denominator == 0u32 || self.numerator == 0u32
    }}
}

impl Iterator for RationalContinuedFraction {
    type Item = Natural;

    fn next(&mut self) -> Option<Natural> {
        if self.denominator == 0u32 || self.numerator == 0u32 {
            None
        } else {
            let n;
            (n, self.numerator) = (&self.numerator).div_mod(&self.denominator);
            swap(&mut self.numerator, &mut self.denominator);
            Some(n)
        }
    }
}

impl ContinuedFraction for Rational {
    type CF = RationalContinuedFraction;

    /// Returns the continued fraction of a [`Rational`], taking the [`Rational`] by value.
    ///
    /// The output has two components. The first is the first value of the continued fraction, which
    /// may be any [`Integer`] and is equal to the floor of the [`Rational`]. The second is an
    /// iterator that produces the remaining values, which are all positive. Using the standard
    /// notation for continued fractions, the first value is the number before the semicolon, and
    /// the second value produces the remaining numbers.
    ///
    /// Each rational number has two continued fraction representations. The shorter of the two
    /// representations (the one that does not end in 1) is returned.
    ///
    /// $f(x) = (a_0, (a_1, a_2, \ldots, a_3)),$ where $x = [a_0; a_1, a_2, \ldots, a_3]$ and $a_3
    /// \neq 1$.
    ///
    /// The output length is $O(n)$, where $n$ is `self.significant_bits()`.
    ///
    /// # Worst-case complexity per iteration
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use itertools::Itertools;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_q::conversion::traits::ContinuedFraction;
    /// use malachite_q::Rational;
    ///
    /// let (head, tail) = Rational::from_signeds(2, 3).continued_fraction();
    /// let tail = tail.collect_vec();
    /// assert_eq!(head, 0);
    /// assert_eq!(tail.to_debug_string(), "[1, 2]");
    ///
    /// let (head, tail) = Rational::from_signeds(355, 113).continued_fraction();
    /// let tail = tail.collect_vec();
    /// assert_eq!(head, 3);
    /// assert_eq!(tail.to_debug_string(), "[7, 16]");
    /// ```
    fn continued_fraction(mut self) -> (Integer, RationalContinuedFraction) {
        let f = (&self).floor();
        self -= Rational::from(&f);
        let (d, n) = self.into_numerator_and_denominator();
        (
            f,
            RationalContinuedFraction {
                numerator: n,
                denominator: d,
            },
        )
    }
}

impl ContinuedFraction for &Rational {
    type CF = RationalContinuedFraction;

    /// Returns the continued fraction of a [`Rational`], taking the [`Rational`] by reference.
    ///
    /// The output has two components. The first is the first value of the continued fraction, which
    /// may be any [`Integer`] and is equal to the floor of the [`Rational`]. The second is an
    /// iterator that produces the remaining values, which are all positive. Using the standard
    /// notation for continued fractions, the first value is the number before the semicolon, and
    /// the second value produces the remaining numbers.
    ///
    /// Each rational number has two continued fraction representations. The shorter of the two
    /// representations (the one that does not end in 1) is returned.
    ///
    /// $f(x) = (a_0, (a_1, a_2, \ldots, a_3)),$ where $x = [a_0; a_1, a_2, \ldots, a_3]$ and $a_3
    /// \neq 1$.
    ///
    /// The output length is $O(n)$, where $n$ is `self.significant_bits()`.
    ///
    /// # Worst-case complexity per iteration
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use itertools::Itertools;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_q::conversion::traits::ContinuedFraction;
    /// use malachite_q::Rational;
    ///
    /// let (head, tail) = (&Rational::from_signeds(2, 3)).continued_fraction();
    /// let tail = tail.collect_vec();
    /// assert_eq!(head, 0);
    /// assert_eq!(tail.to_debug_string(), "[1, 2]");
    ///
    /// let (head, tail) = (&Rational::from_signeds(355, 113)).continued_fraction();
    /// let tail = tail.collect_vec();
    /// assert_eq!(head, 3);
    /// assert_eq!(tail.to_debug_string(), "[7, 16]");
    /// ```
    fn continued_fraction(self) -> (Integer, RationalContinuedFraction) {
        let f = self.floor();
        let (d, n) = (self - Rational::from(&f)).into_numerator_and_denominator();
        (
            f,
            RationalContinuedFraction {
                numerator: n,
                denominator: d,
            },
        )
    }
}
