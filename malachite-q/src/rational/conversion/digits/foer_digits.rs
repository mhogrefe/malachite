// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational::conversion::digits::digits::RationalDigits;
use alloc::vec::Vec;
use malachite_base::foer_sequences::FoerSequence;
use malachite_base::num::arithmetic::traits::{DivExactAssign, IsPowerOf2};
use malachite_base::num::exponential::traits::CheckedLogBase2;
use malachite_base::num::number_theory::traits::Gcd;
use malachite_nz::natural::Natural;

/// A digit after the point of a [`Rational`], produced by [`RationalFoerDigits`], together with its
/// place in the expansion.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct FoerDigit {
    /// The digit.
    pub digit: Natural,
    /// Whether the digit belongs to the repeating part of the expansion.
    pub repeating: bool,
    /// Whether the digit is the first of a repetition of the repeating part. The first repeating
    /// digit is one; seeing another means that one whole period has been produced.
    pub period_start: bool,
}

/// Represents the base-$b$ digits of the fractional portion of a [`Rational`] number, each marked
/// with whether it repeats.
///
/// See [`foer_digits`](Rational::foer_digits) for more information.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RationalFoerDigits {
    digits: RationalDigits,
    index: u64,
    non_repeating_len: u64,
    terminating: bool,
    period_remainder: Option<Rational>,
}

impl RationalFoerDigits {
    /// Returns the number of digits after the point that come before the repeating part. When the
    /// expansion terminates, every digit is non-repeating, and this is the number of digits after
    /// the point.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    #[inline]
    pub const fn non_repeating_len(&self) -> u64 {
        self.non_repeating_len
    }

    /// Returns whether the expansion terminates, that is, whether it has no repeating part.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    #[inline]
    pub const fn is_terminating(&self) -> bool {
        self.terminating
    }
}

impl Iterator for RationalFoerDigits {
    type Item = FoerDigit;

    fn next(&mut self) -> Option<FoerDigit> {
        let i = self.index;
        let repeating = !self.terminating && i >= self.non_repeating_len;
        // The repeating part restarts exactly when the remaining fraction equals the one that
        // remained when the repeating part first began.
        let period_start = repeating
            && if let Some(period_remainder) = &self.period_remainder {
                self.digits.remainder() == period_remainder
            } else {
                self.period_remainder = Some(self.digits.remainder().clone());
                true
            };
        let digit = self.digits.next()?;
        self.index += 1;
        Some(FoerDigit {
            digit,
            repeating,
            period_start,
        })
    }
}

// Returns the number of digits after the point that precede the repeating part of `p / q`, in
// lowest terms, in base `base`, and whether the expansion terminates. Writing $q = q_1 q_2$, where
// every prime factor of $q_1$ divides the base and $q_2$ is coprime to it, the count is the least
// $k$ with $q_1 \mid b^k$, and the expansion terminates exactly when $q_2 = 1$. Each division of
// $q$ by $\gcd(q, b)$ removes one factor of $b$'s share, so $k$ is the number of divisions until
// the GCD is 1, and what is left is $q_2$.
pub(crate) fn non_repeating_len(denominator: &Natural, base: &Natural) -> (u64, bool) {
    if let Some(log_base) = base.checked_log_base_2() {
        return power_of_2_non_repeating_len(denominator, log_base);
    }
    let mut q = denominator.clone();
    let mut k = 0;
    loop {
        let g = (&q).gcd(base);
        if g == 1u32 {
            return (k, q == 1u32);
        }
        q.div_exact_assign(g);
        k += 1;
    }
}

// As `non_repeating_len`, for the base $2^k$, where `log_base` is $k$: the count is the number of
// factors of 2 in the denominator divided by $k$, rounded up.
pub(crate) fn power_of_2_non_repeating_len(denominator: &Natural, log_base: u64) -> (u64, bool) {
    (
        denominator.trailing_zeros().unwrap().div_ceil(log_base),
        denominator.is_power_of_2(),
    )
}

// Returns the digits of `remainder`, in $[0, 1)$, as a `FoerSequence`, given the number of digits
// before the repeating part and whether the expansion terminates, from `non_repeating_len`. `step`
// produces the next digit, updating the remainder. The repeating part ends when the remainder
// returns to its value at the start of the repeating part, so only that one value is kept.
pub(crate) fn fractional_digits_to_foer_sequence(
    mut remainder: Rational,
    (non_repeating_len, terminating): (u64, bool),
    mut step: impl FnMut(&mut Rational) -> Natural,
) -> FoerSequence<Natural> {
    let non_repeating: Vec<Natural> = (0..non_repeating_len)
        .map(|_| step(&mut remainder))
        .collect();
    if terminating {
        return FoerSequence::from_vec(non_repeating);
    }
    let start = remainder.clone();
    let mut repeating = Vec::new();
    loop {
        repeating.push(step(&mut remainder));
        if remainder == start {
            return FoerSequence::from_vecs(non_repeating, repeating);
        }
    }
}

impl Rational {
    /// Returns the base-$b$ digits of a [`Rational`], marking which digits after the point repeat.
    ///
    /// The output has two components. The first is a [`Vec`] of the digits of the integer portion
    /// of the [`Rational`], least- to most-significant. The second is an iterator of the digits of
    /// the fractional portion, each a [`FoerDigit`] saying whether it belongs to the repeating part
    /// and whether it starts a repetition of it.
    ///
    /// Where the repeating part begins is known in advance, from the denominator, and is given by
    /// [`non_repeating_len`](RationalFoerDigits::non_repeating_len); the length of the repeating
    /// part is not computed, but the second digit with `period_start` set marks the end of the
    /// first period. To display at most $L$ digits and mark the repeating part when a whole period
    /// fits, take $L + 1$ digits and look for that second start.
    ///
    /// The digits are the same as those of [`digits`](Rational::digits), and like them are produced
    /// lazily. [`to_digits`](Rational::to_digits) instead computes the whole repeating part, which
    /// may be very long.
    ///
    /// # Worst-case complexity per iteration
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// base.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `base` is less than 2.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    ///
    /// let (before_point, after_point) =
    ///     Rational::from_signeds(25, 12).foer_digits(&Natural::from(10u32));
    /// assert_eq!(before_point.to_debug_string(), "[2]");
    /// assert_eq!(after_point.non_repeating_len(), 2);
    /// let marked: Vec<String> = after_point
    ///     .take(5)
    ///     .map(|d| {
    ///         if d.repeating {
    ///             format!("({})", d.digit)
    ///         } else {
    ///             d.digit.to_string()
    ///         }
    ///     })
    ///     .collect();
    /// assert_eq!(marked.concat(), "08(3)(3)(3)");
    /// ```
    pub fn foer_digits(&self, base: &Natural) -> (Vec<Natural>, RationalFoerDigits) {
        let (before_point, digits) = self.digits(base);
        let (non_repeating_len, terminating) = non_repeating_len(self.denominator_ref(), base);
        (
            before_point,
            RationalFoerDigits {
                digits,
                index: 0,
                non_repeating_len,
                terminating,
                period_remainder: None,
            },
        )
    }
}
