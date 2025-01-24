// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::conversion::continued_fraction::to_continued_fraction::RationalContinuedFraction;
use crate::conversion::traits::ContinuedFraction;
use crate::conversion::traits::Convergents;
use crate::Rational;
use core::mem::swap;
use malachite_base::num::arithmetic::traits::{AddMulAssign, UnsignedAbs};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

/// An iterator that produces the convergents of a [`Rational`].
///
/// See [`convergents`](Rational::convergents) for more information.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RationalConvergents {
    first: bool,
    previous_numerator: Integer,
    previous_denominator: Natural,
    numerator: Integer,
    denominator: Natural,
    cf: RationalContinuedFraction,
}

impl Iterator for RationalConvergents {
    type Item = Rational;

    fn next(&mut self) -> Option<Rational> {
        if self.first {
            self.first = false;
            Some(Rational::from(&self.numerator))
        } else if let Some(n) = self.cf.next() {
            self.previous_numerator
                .add_mul_assign(&self.numerator, Integer::from(&n));
            self.previous_denominator
                .add_mul_assign(&self.denominator, n);
            swap(&mut self.numerator, &mut self.previous_numerator);
            swap(&mut self.denominator, &mut self.previous_denominator);
            Some(Rational {
                sign: self.numerator >= 0,
                numerator: (&self.numerator).unsigned_abs(),
                denominator: self.denominator.clone(),
            })
        } else {
            None
        }
    }
}

impl Convergents for Rational {
    type C = RationalConvergents;

    /// Returns the convergents of a [`Rational`], taking the [`Rational`] by value.
    ///
    /// The convergents of a number are the sequence of rational numbers whose continued fractions
    /// are the prefixes of the number's continued fraction. The first convergent is the floor of
    /// the number. The sequence of convergents is finite iff the number is rational, in which case
    /// the last convergent is the number itself. Each convergent is closer to the number than the
    /// previous convergent is. The even-indexed convergents are less than or equal to the number,
    /// and the odd-indexed ones are greater than or equal to it.
    ///
    /// $f(x) = ([a_0; a_1, a_2, \ldots, a_i])_{i=0}^{n}$, where $x = [a_0; a_1, a_2, \ldots, a_n]$
    /// and $a_n \neq 1$.
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
    /// use malachite_q::conversion::traits::Convergents;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::from_signeds(2, 3)
    ///         .convergents()
    ///         .collect_vec()
    ///         .to_debug_string(),
    ///     "[0, 1, 2/3]"
    /// );
    /// assert_eq!(
    ///     Rational::from_signeds(355, 113)
    ///         .convergents()
    ///         .collect_vec()
    ///         .to_debug_string(),
    ///     "[3, 22/7, 355/113]",
    /// );
    /// ```
    fn convergents(self) -> RationalConvergents {
        let (floor, cf) = self.continued_fraction();
        RationalConvergents {
            first: true,
            previous_numerator: Integer::ONE,
            previous_denominator: Natural::ZERO,
            numerator: floor,
            denominator: Natural::ONE,
            cf,
        }
    }
}

impl Convergents for &Rational {
    type C = RationalConvergents;

    /// Returns the convergents of a [`Rational`], taking the [`Rational`] by reference.
    ///
    /// The convergents of a number are the sequence of rational numbers whose continued fractions
    /// are the prefixes of the number's continued fraction. The first convergent is the floor of
    /// the number. The sequence of convergents is finite iff the number is rational, in which case
    /// the last convergent is the number itself. Each convergent is closer to the number than the
    /// previous convergent is. The even-indexed convergents are less than or equal to the number,
    /// and the odd-indexed ones are greater than or equal to it.
    ///
    /// $f(x) = ([a_0; a_1, a_2, \ldots, a_i])_{i=0}^{n}$, where $x = [a_0; a_1, a_2, \ldots, a_n]$
    /// and $a_n \neq 1$.
    ///
    /// The output length is $O(n)$, where $n$ is `self.significant_bits()`.
    ///
    /// # Worst-case complexity
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
    /// use malachite_q::conversion::traits::Convergents;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     (&Rational::from_signeds(2, 3))
    ///         .convergents()
    ///         .collect_vec()
    ///         .to_debug_string(),
    ///     "[0, 1, 2/3]"
    /// );
    /// assert_eq!(
    ///     (&Rational::from_signeds(355, 113))
    ///         .convergents()
    ///         .collect_vec()
    ///         .to_debug_string(),
    ///     "[3, 22/7, 355/113]",
    /// );
    /// ```
    fn convergents(self) -> RationalConvergents {
        let (floor, cf) = self.continued_fraction();
        RationalConvergents {
            first: true,
            previous_numerator: Integer::ONE,
            previous_denominator: Natural::ZERO,
            numerator: floor,
            denominator: Natural::ONE,
            cf,
        }
    }
}
