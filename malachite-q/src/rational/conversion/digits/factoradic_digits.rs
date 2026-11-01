// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{AddMul, DivAssignMod, DivMod};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::{ExactFrom, FactoradicDigits};
use malachite_nz::natural::Natural;

/// Represents the factorial-number-system digits of the fractional portion of a [`Rational`]
/// number: the coefficients of $1/2!$, $1/3!$, $1/4!$, and so on.
///
/// See [`factoradic_digits`](Rational::factoradic_digits) for more information.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RationalFactoradicDigits {
    // The fraction still to be expanded is `numerator / denominator`, in $[0, 1)$; the denominator
    // never changes, so no step needs a GCD.
    numerator: Natural,
    denominator: Natural,
    base: u64,
}

impl Iterator for RationalFactoradicDigits {
    type Item = u64;

    // With the remaining fraction $r = \sum_{k \geq b} d_k / k!$, $br = d_b + (\text{a fraction in
    // } [0, 1))$, so the next digit is the floor of $br$.
    fn next(&mut self) -> Option<u64> {
        if self.numerator == 0u32 {
            return None;
        }
        self.numerator *= Natural::from(self.base);
        let remainder = self.numerator.div_assign_mod(&self.denominator);
        let digit = u64::exact_from(&self.numerator);
        self.numerator = remainder;
        self.base += 1;
        Some(digit)
    }
}

impl Rational {
    /// Returns the factorial-number-system digits of the absolute value of a [`Rational`].
    ///
    /// The output has two components. The first is a [`Vec`] of the digits of the integer portion
    /// of the [`Rational`], in ascending order: the digit at index $i$ is the coefficient of
    /// $(i+1)!$, as in [`to_factoradic_digits_asc`](FactoradicDigits::to_factoradic_digits_asc).
    /// The second is an iterator of the digits of the fractional portion: the coefficients of
    /// $1/2!$, $1/3!$, $1/4!$, and so on, where the coefficient of $1/k!$ is less than $k$. (The
    /// coefficient of $1/1!$ would always be 0, so it is omitted.)
    ///
    /// Every [`Rational`] has a finite expansion, ending with the coefficient of $1/k!$ for the
    /// least $k$ such that the denominator divides $k!$; the iterator stops there. That $k$ is at
    /// least the largest prime factor of the denominator, so the expansion may be extremely long,
    /// and the digits are produced lazily.
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
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_q::Rational;
    ///
    /// let (before_point, after_point) = Rational::from_signeds(7, 2).factoradic_digits();
    /// assert_eq!(before_point.to_debug_string(), "[1, 1]");
    /// assert_eq!(after_point.collect::<Vec<_>>().to_debug_string(), "[1]");
    ///
    /// let (before_point, after_point) = Rational::from_signeds(100, 7).factoradic_digits();
    /// assert_eq!(before_point.to_debug_string(), "[0, 1, 2]");
    /// assert_eq!(
    ///     after_point.collect::<Vec<_>>().to_debug_string(),
    ///     "[0, 1, 2, 4, 1, 5]"
    /// );
    /// ```
    pub fn factoradic_digits(&self) -> (Vec<u64>, RationalFactoradicDigits) {
        let (floor, numerator) = self.numerator_ref().div_mod(self.denominator_ref());
        (
            floor.to_factoradic_digits_asc(),
            RationalFactoradicDigits {
                numerator,
                denominator: self.denominator_ref().clone(),
                base: 2,
            },
        )
    }

    /// Converts factorial-number-system digits into a nonnegative [`Rational`].
    ///
    /// `before_point` holds the digits of the integer portion in ascending order: the digit at
    /// index $i$ is the coefficient of $(i+1)!$ and must be at most $i + 1$. `after_point` holds
    /// the digits of the fractional portion: the digit at index $j$ is the coefficient of
    /// $1/(j+2)!$ and must be at most $j + 1$. The function returns `None` if any digit is too
    /// large. Trailing zeros are allowed in both.
    ///
    /// $$
    /// f((d_i)_ {i=0}^{k-1}, (e_j)_ {j=0}^{l-1}) = \sum_{i=0}^{k-1}(i+1)! d_i +
    /// \sum_{j=0}^{l-1}\frac{e_j}{(j+2)!}.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(before_point.len(),
    /// after_point.len())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::from_factoradic_digits(&[1, 1], &[1]).to_debug_string(),
    ///     "Some(7/2)"
    /// );
    /// assert_eq!(
    ///     Rational::from_factoradic_digits(&[0, 1, 2], &[0, 1, 2, 4, 1, 5]).to_debug_string(),
    ///     "Some(100/7)"
    /// );
    /// // the coefficient of 1/2! must be at most 1
    /// assert_eq!(
    ///     Rational::from_factoradic_digits(&[], &[2]).to_debug_string(),
    ///     "None"
    /// );
    /// ```
    pub fn from_factoradic_digits(before_point: &[u64], after_point: &[u64]) -> Option<Self> {
        let floor = Natural::from_factoradic_digits_asc(before_point.iter().copied())?;
        // With $D = (l+1)!$, the fractional part is $N / D$, built up one place at a time: $N \gets
        // N(j+2) + e_j$ and $D \gets D(j+2)$.
        let mut numerator = Natural::ZERO;
        let mut denominator = Natural::ONE;
        for (j, &digit) in after_point.iter().enumerate() {
            let base = u64::exact_from(j) + 2;
            if digit >= base {
                return None;
            }
            numerator *= Natural::from(base);
            numerator += Natural::from(digit);
            denominator *= Natural::from(base);
        }
        Some(Self::from_naturals(
            numerator.add_mul(floor, &denominator),
            denominator,
        ))
    }
}
