// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::arithmetic::denominators_in_closed_interval::DenominatorsInClosedRationalInterval;
use crate::arithmetic::traits::DenominatorsInClosedInterval;
use crate::Rational;
use core::iter::{once, Chain, Once};
use core::mem::swap;
use malachite_base::num::arithmetic::traits::{CoprimeWith, UnsignedAbs};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::RoundingFrom;
use malachite_base::num::iterators::{ruler_sequence, RulerSequence};
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::tuples::exhaustive::{
    exhaustive_dependent_pairs, ExhaustiveDependentPairs, ExhaustiveDependentPairsYsGenerator,
};
use malachite_nz::integer::exhaustive::{
    exhaustive_integer_range, exhaustive_integer_range_to_infinity,
    exhaustive_integer_range_to_negative_infinity, ExhaustiveIntegerRange,
    ExhaustiveIntegerRangeToInfinity, ExhaustiveIntegerRangeToNegativeInfinity,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::exhaustive::{
    exhaustive_positive_naturals, ExhaustiveNaturalRangeToInfinity,
};
use malachite_nz::natural::Natural;

/// Generates all positive [`Rational`]s.
///
/// This `struct` is created by [`exhaustive_positive_rationals`]; see its documentation for more.
#[derive(Clone, Debug)]
pub struct ExhaustivePositiveRationals {
    pred_pred: Natural,
    pred: Natural,
}

impl Iterator for ExhaustivePositiveRationals {
    type Item = Rational;

    fn next(&mut self) -> Option<Rational> {
        let mut anm1 = Natural::ZERO;
        swap(&mut self.pred_pred, &mut anm1);
        swap(&mut self.pred, &mut self.pred_pred);
        let k = &anm1 / &self.pred_pred; // floor(a(n - 1) / a(n))
        self.pred = ((k << 1u32) | Natural::ONE) * &self.pred_pred - anm1;
        Some(Rational {
            sign: true,
            numerator: self.pred_pred.clone(),
            denominator: self.pred.clone(),
        })
    }
}

/// Generates all positive [`Rational`]s.
///
/// The [`Rational`]s are ordered as in the [Calkin-Wilf
/// sequence](https://en.wikipedia.org/wiki/Calkin%E2%80%93Wilf_tree#Breadth_first_traversal). Their
/// numerators and denominators are given by the [Stern-Brocot
/// sequence](https://en.wikipedia.org/wiki/Stern%E2%80%93Brocot_tree#Relation_to_Farey_sequences).
/// To generate the latter sequence, this iterator uses the formula
/// $$
/// a_{n+1} = \left ( 2 \left \lfloor \frac{a_{n-1}}{a_n} \right \rfloor +1 \right ) a_n - a_{n-1},
/// $$
/// attributed to David S. Newman at <https://oeis.org/A002487>.
///
/// The output length is infinite. The numerators and denominators of the $n$th element are
/// $O(n^\frac{\log \phi}{\log 2})$.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(\log i \log\log i \log\log\log i)$
///
/// $M(i) = O(\log i \log\log i)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is the iteration number.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_q::exhaustive::exhaustive_positive_rationals;
///
/// assert_eq!(
///     prefix_to_string(exhaustive_positive_rationals(), 20),
///     "[1, 1/2, 2, 1/3, 3/2, 2/3, 3, 1/4, 4/3, 3/5, 5/2, 2/5, 5/3, 3/4, 4, 1/5, 5/4, 4/7, 7/3, \
///     3/8, ...]"
/// )
/// ```
pub const fn exhaustive_positive_rationals() -> ExhaustivePositiveRationals {
    ExhaustivePositiveRationals {
        pred_pred: Natural::ZERO,
        pred: Natural::ONE,
    }
}

/// Generates all non-positive [`Rational`]s.
///
/// Zero is generated first, followed by all the positive [`Rational`]s. See
/// [`exhaustive_positive_rationals`] for details.
///
/// The output length is infinite. The numerators and denominators of the $n$th element are
/// $O(n^\frac{\log \phi}{\log 2})$.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(\log i \log\log i \log\log\log i)$
///
/// $M(i) = O(\log i \log\log i)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is the iteration number.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_q::exhaustive::exhaustive_non_negative_rationals;
///
/// assert_eq!(
///     prefix_to_string(exhaustive_non_negative_rationals(), 20),
///     "[0, 1, 1/2, 2, 1/3, 3/2, 2/3, 3, 1/4, 4/3, 3/5, 5/2, 2/5, 5/3, 3/4, 4, 1/5, 5/4, 4/7, \
///     7/3, ...]"
/// )
/// ```
pub fn exhaustive_non_negative_rationals() -> Chain<Once<Rational>, ExhaustivePositiveRationals> {
    once(Rational::ZERO).chain(exhaustive_positive_rationals())
}

/// Generates all negative [`Rational`]s.
///
/// This `struct` is created by [`exhaustive_negative_rationals`]; see its documentation for more.
#[derive(Clone, Debug)]
pub struct ExhaustiveNegativeRationals {
    xs: ExhaustivePositiveRationals,
}

impl Iterator for ExhaustiveNegativeRationals {
    type Item = Rational;

    fn next(&mut self) -> Option<Rational> {
        self.xs.next().map(|mut q| {
            q.sign = false;
            q
        })
    }
}

/// Generates all negative [`Rational`]s.
///
/// The sequence is the same as the sequence of positive [`Rational`]s, but negated. See
/// [`exhaustive_positive_rationals`] for details.
///
/// The output length is infinite. The absolute values of the numerators and denominators of the
/// $n$th element are $O(n^\frac{\log \phi}{\log 2})$.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(\log i \log\log i \log\log\log i)$
///
/// $M(i) = O(\log i \log\log i)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is the iteration number.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_q::exhaustive::exhaustive_negative_rationals;
///
/// assert_eq!(
///     prefix_to_string(exhaustive_negative_rationals(), 20),
///     "[-1, -1/2, -2, -1/3, -3/2, -2/3, -3, -1/4, -4/3, -3/5, -5/2, -2/5, -5/3, -3/4, -4, -1/5, \
///     -5/4, -4/7, -7/3, -3/8, ...]"
/// )
/// ```
pub const fn exhaustive_negative_rationals() -> ExhaustiveNegativeRationals {
    ExhaustiveNegativeRationals {
        xs: exhaustive_positive_rationals(),
    }
}

/// Generates all nonzero [`Rational`]s.
///
/// This `struct` is created by [`exhaustive_nonzero_rationals`]; see its documentation for more.
#[derive(Clone, Debug)]
pub struct ExhaustiveNonzeroRationals {
    xs: ExhaustivePositiveRationals,
    x: Option<Rational>,
    sign: bool,
}

impl Iterator for ExhaustiveNonzeroRationals {
    type Item = Rational;

    fn next(&mut self) -> Option<Rational> {
        if self.sign {
            self.sign = false;
            let mut x = None;
            swap(&mut self.x, &mut x);
            let mut x = x.unwrap();
            x.sign = false;
            Some(x)
        } else {
            self.sign = true;
            self.x = self.xs.next();
            Some(self.x.clone().unwrap())
        }
    }
}

/// Generates all nonzero [`Rational`]s.
///
/// The sequence is the same the sequence of positive [`Rational`]s, interleaved with its negative.
/// See [`exhaustive_positive_rationals`] for details.
///
/// The output length is infinite. The absolute values of the numerators and denominators of the
/// $n$th element are $O(n^\frac{\log \phi}{\log 2})$.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(\log i \log\log i \log\log\log i)$
///
/// $M(i) = O(\log i \log\log i)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is the iteration number.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_q::exhaustive::exhaustive_nonzero_rationals;
///
/// assert_eq!(
///     prefix_to_string(exhaustive_nonzero_rationals(), 20),
///     "[1, -1, 1/2, -1/2, 2, -2, 1/3, -1/3, 3/2, -3/2, 2/3, -2/3, 3, -3, 1/4, -1/4, 4/3, -4/3, \
///     3/5, -3/5, ...]"
/// )
/// ```
pub const fn exhaustive_nonzero_rationals() -> ExhaustiveNonzeroRationals {
    ExhaustiveNonzeroRationals {
        xs: exhaustive_positive_rationals(),
        x: None,
        sign: false,
    }
}

/// Generates all [`Rational`]s.
///
/// The sequence begins with zero and is followed by the sequence of positive [`Rational`]s,
/// interleaved with its negative. See [`exhaustive_positive_rationals`] for details.
///
/// The output length is infinite. The absolute values of the numerators and denominators of the
/// $n$th element are $O(n^\frac{\log \phi}{\log 2})$.
///
/// # Worst-case complexity per iteration
/// $T(n) = O(\log n \log\log n \log\log\log n)$
///
/// $M(i) = O(\log n \log\log n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is the iteration number.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_q::exhaustive::exhaustive_rationals;
///
/// assert_eq!(
///     prefix_to_string(exhaustive_rationals(), 20),
///     "[0, 1, -1, 1/2, -1/2, 2, -2, 1/3, -1/3, 3/2, -3/2, 2/3, -2/3, 3, -3, 1/4, -1/4, 4/3, \
///     -4/3, 3/5, ...]"
/// )
/// ```
pub fn exhaustive_rationals() -> Chain<Once<Rational>, ExhaustiveNonzeroRationals> {
    once(Rational::ZERO).chain(exhaustive_nonzero_rationals())
}

/// Generates all [`Rational`]s with a specific denominator and with numerators from a given
/// iterator. Numerators that are not coprime with the denominator are skipped.
#[derive(Clone, Debug)]
pub struct RationalsWithDenominator<I: Iterator<Item = Integer>> {
    pub(crate) numerators: I,
    pub(crate) denominator: Natural,
}

impl<I: Iterator<Item = Integer>> Iterator for RationalsWithDenominator<I> {
    type Item = Rational;

    fn next(&mut self) -> Option<Rational> {
        loop {
            let n = self.numerators.next()?;
            if n.unsigned_abs_ref().coprime_with(&self.denominator) {
                return Some(Rational {
                    sign: n >= 0u32,
                    numerator: n.unsigned_abs(),
                    denominator: self.denominator.clone(),
                });
            }
        }
    }
}

/// Generates all [`Rational`]s greater than or equal to some number $a$ and with a specific
/// denominator, in order of increasing absolute value.
///
/// When two [`Rational`]s have the same absolute value, the positive one comes first.
///
/// The output satisfies $(|x_i|, \operatorname{sgn}(-x_i)) <_\mathrm{lex} (|x_j|,
/// \operatorname{sgn}(-x_j))$ whenever $i < j$.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(\log i (\log \log i)^2 \log\log\log i)$
///
/// $M(i) = O(\log i \log \log i)$
///
/// where $T$ is time, $M$ is additional memory, and $i$ is the iteration number.
///
/// # Panics
/// Panics if `d` is zero.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_nz::natural::Natural;
/// use malachite_q::exhaustive::exhaustive_rationals_with_denominator_range_to_infinity;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     prefix_to_string(
///         exhaustive_rationals_with_denominator_range_to_infinity(
///             Natural::from(5u32),
///             Rational::from_signeds(22i32, 7)
///         ),
///         10
///     ),
///     "[16/5, 17/5, 18/5, 19/5, 21/5, 22/5, 23/5, 24/5, 26/5, 27/5, ...]"
/// );
/// assert_eq!(
///     prefix_to_string(
///         exhaustive_rationals_with_denominator_range_to_infinity(
///             Natural::from(2u32),
///             Rational::from_signeds(-22i32, 7)
///         ),
///         10
///     ),
///     "[1/2, -1/2, 3/2, -3/2, 5/2, -5/2, 7/2, 9/2, 11/2, 13/2, ...]"
/// );
/// ```
pub fn exhaustive_rationals_with_denominator_range_to_infinity(
    d: Natural,
    a: Rational,
) -> RationalsWithDenominator<ExhaustiveIntegerRangeToInfinity> {
    assert_ne!(d, 0u32);
    RationalsWithDenominator {
        numerators: exhaustive_integer_range_to_infinity(
            Integer::rounding_from(a * Rational::from(&d), Ceiling).0,
        ),
        denominator: d,
    }
}

/// Generates all [`Rational`]s less than or equal to some number $a$ and with a specific
/// denominator, in order of increasing absolute value.
///
/// When two [`Rational`]s have the same absolute value, the positive one comes first.
///
/// The output satisfies $(|x_i|, \operatorname{sgn}(-x_i)) <_\mathrm{lex} (|x_j|,
/// \operatorname{sgn}(-x_j))$ whenever $i < j$.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(\log i (\log \log i)^2 \log\log\log i)$
///
/// $M(i) = O(\log i \log \log i)$
///
/// where $T$ is time, $M$ is additional memory, and $i$ is the iteration number.
///
/// # Panics
/// Panics if `d` is zero.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_nz::natural::Natural;
/// use malachite_q::exhaustive::exhaustive_rationals_with_denominator_range_to_negative_infinity;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     prefix_to_string(
///         exhaustive_rationals_with_denominator_range_to_negative_infinity(
///             Natural::from(5u32),
///             Rational::from_signeds(-22i32, 7)
///         ),
///         10
///     ),
///     "[-16/5, -17/5, -18/5, -19/5, -21/5, -22/5, -23/5, -24/5, -26/5, -27/5, ...]"
/// );
/// assert_eq!(
///     prefix_to_string(
///         exhaustive_rationals_with_denominator_range_to_negative_infinity(
///             Natural::from(2u32),
///             Rational::from_signeds(22i32, 7)
///         ),
///         10
///     ),
///     "[1/2, -1/2, 3/2, -3/2, 5/2, -5/2, -7/2, -9/2, -11/2, -13/2, ...]"
/// );
/// ```
pub fn exhaustive_rationals_with_denominator_range_to_negative_infinity(
    d: Natural,
    a: Rational,
) -> RationalsWithDenominator<ExhaustiveIntegerRangeToNegativeInfinity> {
    assert_ne!(d, 0u32);
    RationalsWithDenominator {
        numerators: exhaustive_integer_range_to_negative_infinity(
            Integer::rounding_from(a * Rational::from(&d), Floor).0,
        ),
        denominator: d,
    }
}

/// Generates all [`Rational`]s in the half-open range $[a, b)$ and with a specific denominator, in
/// order of increasing absolute value.
///
/// When two [`Rational`]s have the same absolute value, the positive one comes first.
///
/// The output satisfies $(|x_i|, \operatorname{sgn}(-x_i)) <_\mathrm{lex} (|x_j|,
/// \operatorname{sgn}(-x_j))$ whenever $i < j$.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(\log i (\log \log i)^2 \log\log\log i)$
///
/// $M(i) = O(\log i \log \log i)$
///
/// where $T$ is time, $M$ is additional memory, and $i$ is the iteration number.
///
/// # Panics
/// Panics if `d` is zero or if $a \geq b$.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::strings::ToDebugString;
/// use malachite_nz::natural::Natural;
/// use malachite_q::exhaustive::exhaustive_rationals_with_denominator_range;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     exhaustive_rationals_with_denominator_range(
///         Natural::from(2u32),
///         Rational::from_signeds(1i32, 3),
///         Rational::from_signeds(5i32, 2)
///     )
///     .collect_vec()
///     .to_debug_string(),
///     "[1/2, 3/2]"
/// );
/// assert_eq!(
///     exhaustive_rationals_with_denominator_range(
///         Natural::from(2u32),
///         Rational::from_signeds(-5i32, 3),
///         Rational::from_signeds(5i32, 2)
///     )
///     .collect_vec()
///     .to_debug_string(),
///     "[1/2, -1/2, 3/2, -3/2]"
/// );
/// assert_eq!(
///     exhaustive_rationals_with_denominator_range(
///         Natural::from(10u32),
///         Rational::try_from_float_simplest(std::f64::consts::E).unwrap(),
///         Rational::try_from_float_simplest(std::f64::consts::PI).unwrap(),
///     )
///     .collect_vec()
///     .to_debug_string(),
///     "[29/10, 31/10]"
/// );
/// ```
pub fn exhaustive_rationals_with_denominator_range(
    d: Natural,
    a: Rational,
    b: Rational,
) -> RationalsWithDenominator<ExhaustiveIntegerRange> {
    assert_ne!(d, 0u32);
    assert!(a < b);
    let q_d = Rational::from(&d);
    let a_i = Integer::rounding_from(a * &q_d, Ceiling).0;
    let upper_included = b.denominator_ref() == &d;
    let mut b_i = Integer::rounding_from(b * q_d, Floor).0;
    if !upper_included {
        b_i += Integer::ONE;
    }
    RationalsWithDenominator {
        numerators: exhaustive_integer_range(a_i, b_i),
        denominator: d,
    }
}

/// Generates all [`Rational`]s in the closed range $[a, b]$ and with a specific denominator, in
/// order of increasing absolute value.
///
/// When two [`Rational`]s have the same absolute value, the positive one comes first.
///
/// The output satisfies $(|x_i|, \operatorname{sgn}(-x_i)) <_\mathrm{lex} (|x_j|,
/// \operatorname{sgn}(-x_j))$ whenever $i < j$.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(\log i (\log \log i)^2 \log\log\log i)$
///
/// $M(i) = O(\log i \log \log i)$
///
/// where $T$ is time, $M$ is additional memory, and $i$ is the iteration number.
///
/// # Panics
/// Panics if `d` is zero or if $a > b$.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::strings::ToDebugString;
/// use malachite_nz::natural::Natural;
/// use malachite_q::exhaustive::exhaustive_rationals_with_denominator_inclusive_range;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     exhaustive_rationals_with_denominator_inclusive_range(
///         Natural::from(2u32),
///         Rational::from_signeds(1i32, 3),
///         Rational::from_signeds(5i32, 2)
///     )
///     .collect_vec()
///     .to_debug_string(),
///     "[1/2, 3/2, 5/2]"
/// );
/// assert_eq!(
///     exhaustive_rationals_with_denominator_inclusive_range(
///         Natural::from(2u32),
///         Rational::from_signeds(-5i32, 3),
///         Rational::from_signeds(5i32, 2)
///     )
///     .collect_vec()
///     .to_debug_string(),
///     "[1/2, -1/2, 3/2, -3/2, 5/2]"
/// );
/// assert_eq!(
///     exhaustive_rationals_with_denominator_inclusive_range(
///         Natural::from(10u32),
///         Rational::try_from_float_simplest(std::f64::consts::E).unwrap(),
///         Rational::try_from_float_simplest(std::f64::consts::PI).unwrap(),
///     )
///     .collect_vec()
///     .to_debug_string(),
///     "[29/10, 31/10]"
/// );
/// ```
pub fn exhaustive_rationals_with_denominator_inclusive_range(
    d: Natural,
    a: Rational,
    b: Rational,
) -> RationalsWithDenominator<ExhaustiveIntegerRange> {
    assert_ne!(d, 0u32);
    assert!(a <= b);
    let q_d = Rational::from(&d);
    let a_i = Integer::rounding_from(a * &q_d, Ceiling).0;
    let b_i = Integer::rounding_from(b * q_d, Floor).0 + Integer::ONE;
    RationalsWithDenominator {
        numerators: exhaustive_integer_range(a_i, b_i),
        denominator: d,
    }
}

#[derive(Clone, Debug)]
struct ExhaustiveRationalsWithDenominatorRangeToInfinityGenerator {
    a: Rational,
}

impl
    ExhaustiveDependentPairsYsGenerator<
        Natural,
        Rational,
        RationalsWithDenominator<ExhaustiveIntegerRangeToInfinity>,
    > for ExhaustiveRationalsWithDenominatorRangeToInfinityGenerator
{
    #[inline]
    fn get_ys(&self, d: &Natural) -> RationalsWithDenominator<ExhaustiveIntegerRangeToInfinity> {
        exhaustive_rationals_with_denominator_range_to_infinity(d.clone(), self.a.clone())
    }
}

#[inline]
fn exhaustive_rational_range_to_infinity_helper(
    a: Rational,
) -> ExhaustiveDependentPairs<
    Natural,
    Rational,
    RulerSequence<usize>,
    ExhaustiveRationalsWithDenominatorRangeToInfinityGenerator,
    ExhaustiveNaturalRangeToInfinity,
    RationalsWithDenominator<ExhaustiveIntegerRangeToInfinity>,
> {
    exhaustive_dependent_pairs(
        ruler_sequence(),
        exhaustive_positive_naturals(),
        ExhaustiveRationalsWithDenominatorRangeToInfinityGenerator { a },
    )
}

/// Generates all [`Rational`]s greater than or equal to some [`Rational`].
///
/// This `struct` is created by [`exhaustive_rational_range_to_infinity`]; see its documentation for
/// more.
#[derive(Clone, Debug)]
pub struct ExhaustiveRationalRangeToInfinity(
    ExhaustiveDependentPairs<
        Natural,
        Rational,
        RulerSequence<usize>,
        ExhaustiveRationalsWithDenominatorRangeToInfinityGenerator,
        ExhaustiveNaturalRangeToInfinity,
        RationalsWithDenominator<ExhaustiveIntegerRangeToInfinity>,
    >,
);

impl Iterator for ExhaustiveRationalRangeToInfinity {
    type Item = Rational;

    #[inline]
    fn next(&mut self) -> Option<Rational> {
        self.0.next().map(|p| p.1)
    }
}

/// Generates all [`Rational`]s greater than or equal to some [`Rational`] $a$.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(\log i (\log \log i)^2 \log\log\log i)$
///
/// $M(i) = O(\log i \log \log i)$
///
/// where $T$ is time, $M$ is additional memory, and $i$ is the iteration number.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::conversion::traits::ExactFrom;
/// use malachite_q::exhaustive::exhaustive_rational_range_to_infinity;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     prefix_to_string(
///         exhaustive_rational_range_to_infinity(Rational::exact_from(std::f64::consts::PI)),
///         20
///     ),
///     "[4, 7/2, 5, 10/3, 6, 9/2, 7, 13/4, 8, 11/2, 9, 11/3, 10, 13/2, 11, 16/5, 12, 15/2, 13, \
///     13/3, ...]"
/// )
/// ```
#[inline]
pub fn exhaustive_rational_range_to_infinity(a: Rational) -> ExhaustiveRationalRangeToInfinity {
    ExhaustiveRationalRangeToInfinity(exhaustive_rational_range_to_infinity_helper(a))
}

#[derive(Clone, Debug)]
struct ExhaustiveRationalsWithDenominatorRangeToNegativeInfinityGenerator {
    a: Rational,
}

impl
    ExhaustiveDependentPairsYsGenerator<
        Natural,
        Rational,
        RationalsWithDenominator<ExhaustiveIntegerRangeToNegativeInfinity>,
    > for ExhaustiveRationalsWithDenominatorRangeToNegativeInfinityGenerator
{
    #[inline]
    fn get_ys(
        &self,
        d: &Natural,
    ) -> RationalsWithDenominator<ExhaustiveIntegerRangeToNegativeInfinity> {
        exhaustive_rationals_with_denominator_range_to_negative_infinity(d.clone(), self.a.clone())
    }
}

#[inline]
fn exhaustive_rational_range_to_negative_infinity_helper(
    a: Rational,
) -> ExhaustiveDependentPairs<
    Natural,
    Rational,
    RulerSequence<usize>,
    ExhaustiveRationalsWithDenominatorRangeToNegativeInfinityGenerator,
    ExhaustiveNaturalRangeToInfinity,
    RationalsWithDenominator<ExhaustiveIntegerRangeToNegativeInfinity>,
> {
    exhaustive_dependent_pairs(
        ruler_sequence(),
        exhaustive_positive_naturals(),
        ExhaustiveRationalsWithDenominatorRangeToNegativeInfinityGenerator { a },
    )
}

/// Generates all [`Rational`]s less than or equal to some [`Rational`].
///
/// This `struct` is created by [`exhaustive_rational_range_to_negative_infinity`]; see its
/// documentation for more.
#[derive(Clone, Debug)]
pub struct ExhaustiveRationalRangeToNegativeInfinity(
    ExhaustiveDependentPairs<
        Natural,
        Rational,
        RulerSequence<usize>,
        ExhaustiveRationalsWithDenominatorRangeToNegativeInfinityGenerator,
        ExhaustiveNaturalRangeToInfinity,
        RationalsWithDenominator<ExhaustiveIntegerRangeToNegativeInfinity>,
    >,
);

impl Iterator for ExhaustiveRationalRangeToNegativeInfinity {
    type Item = Rational;

    #[inline]
    fn next(&mut self) -> Option<Rational> {
        self.0.next().map(|p| p.1)
    }
}

/// Generates all [`Rational`]s less than or equal to some [`Rational`] $a$.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(\log i (\log \log i)^2 \log\log\log i)$
///
/// $M(i) = O(\log i \log \log i)$
///
/// where $T$ is time, $M$ is additional memory, and $i$ is the iteration number.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::conversion::traits::ExactFrom;
/// use malachite_q::exhaustive::exhaustive_rational_range_to_negative_infinity;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     prefix_to_string(
///         exhaustive_rational_range_to_negative_infinity(Rational::exact_from(
///             std::f64::consts::PI
///         )),
///         20
///     ),
///     "[0, 1/2, 1, 1/3, -1, -1/2, 2, 1/4, -2, 3/2, 3, -1/3, -3, -3/2, -4, 1/5, -5, 5/2, -6, \
///     2/3, ...]"
/// )
/// ```
#[inline]
pub fn exhaustive_rational_range_to_negative_infinity(
    a: Rational,
) -> ExhaustiveRationalRangeToNegativeInfinity {
    ExhaustiveRationalRangeToNegativeInfinity(
        exhaustive_rational_range_to_negative_infinity_helper(a),
    )
}

#[derive(Clone, Debug)]
struct ExhaustiveRationalsWithDenominatorRangeGenerator {
    a: Rational,
    b: Rational,
}

impl
    ExhaustiveDependentPairsYsGenerator<
        Natural,
        Rational,
        RationalsWithDenominator<ExhaustiveIntegerRange>,
    > for ExhaustiveRationalsWithDenominatorRangeGenerator
{
    #[inline]
    fn get_ys(&self, d: &Natural) -> RationalsWithDenominator<ExhaustiveIntegerRange> {
        exhaustive_rationals_with_denominator_range(d.clone(), self.a.clone(), self.b.clone())
    }
}

#[inline]
fn exhaustive_rational_range_helper(
    a: Rational,
    b: Rational,
) -> ExhaustiveDependentPairs<
    Natural,
    Rational,
    RulerSequence<usize>,
    ExhaustiveRationalsWithDenominatorRangeGenerator,
    DenominatorsInClosedRationalInterval,
    RationalsWithDenominator<ExhaustiveIntegerRange>,
> {
    exhaustive_dependent_pairs(
        ruler_sequence(),
        Rational::denominators_in_closed_interval(a.clone(), b.clone()),
        ExhaustiveRationalsWithDenominatorRangeGenerator { a, b },
    )
}

/// Generates all [`Natural`]s in an interval of the form $[a,b)$.
///
/// This `struct` is created by [`exhaustive_rational_range`]; see its documentation for more.
#[allow(private_interfaces)]
#[derive(Clone, Debug)]
pub enum ExhaustiveRationalRange {
    Empty,
    Nonempty(
        ExhaustiveDependentPairs<
            Natural,
            Rational,
            RulerSequence<usize>,
            ExhaustiveRationalsWithDenominatorRangeGenerator,
            DenominatorsInClosedRationalInterval,
            RationalsWithDenominator<ExhaustiveIntegerRange>,
        >,
    ),
}

impl Iterator for ExhaustiveRationalRange {
    type Item = Rational;

    #[inline]
    fn next(&mut self) -> Option<Rational> {
        match self {
            ExhaustiveRationalRange::Empty => None,
            ExhaustiveRationalRange::Nonempty(xs) => xs.next().map(|p| p.1),
        }
    }
}

/// Generates all [`Rational`]s in the half-open interval $[a, b)$.
///
/// `a` must be less than or equal to `b`. If `a` and `b` are equal, the range is empty. To generate
/// all [`Rational`]s in an infinite interval, use [`exhaustive_rational_range_to_infinity`] or
/// [`exhaustive_rational_range_to_negative_infinity`].
///
/// The output length is infinite if $a<b$ and 0 if $a=b$.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(\log i (\log \log i)^2 \log\log\log i)$
///
/// $M(i) = O(\log i \log \log i)$
///
/// where $T$ is time, $M$ is additional memory, and $i$ is the iteration number.
///
/// # Panics
/// Panics if $a>b$.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::conversion::traits::ExactFrom;
/// use malachite_q::exhaustive::exhaustive_rational_range;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     prefix_to_string(
///         exhaustive_rational_range(
///             Rational::exact_from(std::f64::consts::E),
///             Rational::exact_from(std::f64::consts::PI)
///         ),
///         20
///     ),
///     "[3, 11/4, 14/5, 20/7, 17/6, 23/8, 25/8, 30/11, 25/9, 29/10, 26/9, 31/11, 28/9, 31/10, \
///     32/11, 41/15, 34/11, 35/12, 37/12, 39/14, ...]"
/// )
/// ```
#[inline]
pub fn exhaustive_rational_range(a: Rational, b: Rational) -> ExhaustiveRationalRange {
    if a == b {
        ExhaustiveRationalRange::Empty
    } else {
        ExhaustiveRationalRange::Nonempty(exhaustive_rational_range_helper(a, b))
    }
}

#[derive(Clone, Debug)]
struct ExhaustiveRationalsWithDenominatorInclusiveRangeGenerator {
    a: Rational,
    b: Rational,
}

impl
    ExhaustiveDependentPairsYsGenerator<
        Natural,
        Rational,
        RationalsWithDenominator<ExhaustiveIntegerRange>,
    > for ExhaustiveRationalsWithDenominatorInclusiveRangeGenerator
{
    #[inline]
    fn get_ys(&self, d: &Natural) -> RationalsWithDenominator<ExhaustiveIntegerRange> {
        exhaustive_rationals_with_denominator_inclusive_range(
            d.clone(),
            self.a.clone(),
            self.b.clone(),
        )
    }
}

#[inline]
fn exhaustive_rational_inclusive_range_helper(
    a: Rational,
    b: Rational,
) -> ExhaustiveDependentPairs<
    Natural,
    Rational,
    RulerSequence<usize>,
    ExhaustiveRationalsWithDenominatorInclusiveRangeGenerator,
    DenominatorsInClosedRationalInterval,
    RationalsWithDenominator<ExhaustiveIntegerRange>,
> {
    exhaustive_dependent_pairs(
        ruler_sequence(),
        Rational::denominators_in_closed_interval(a.clone(), b.clone()),
        ExhaustiveRationalsWithDenominatorInclusiveRangeGenerator { a, b },
    )
}

/// Generates all [`Rational`]s in an interval of the form $\[a,b\]$.
///
/// This `struct` is created by [`exhaustive_rational_inclusive_range`]; see its documentation for
/// more.
#[allow(private_interfaces)]
#[derive(Clone, Debug)]
pub enum ExhaustiveRationalInclusiveRange {
    Single(bool, Rational),
    Many(
        ExhaustiveDependentPairs<
            Natural,
            Rational,
            RulerSequence<usize>,
            ExhaustiveRationalsWithDenominatorInclusiveRangeGenerator,
            DenominatorsInClosedRationalInterval,
            RationalsWithDenominator<ExhaustiveIntegerRange>,
        >,
    ),
}

impl Iterator for ExhaustiveRationalInclusiveRange {
    type Item = Rational;

    #[inline]
    fn next(&mut self) -> Option<Rational> {
        match self {
            ExhaustiveRationalInclusiveRange::Single(done, x) => {
                if *done {
                    None
                } else {
                    *done = true;
                    Some(x.clone())
                }
            }
            ExhaustiveRationalInclusiveRange::Many(xs) => xs.next().map(|p| p.1),
        }
    }
}

/// Generates all [`Rational`]s in the closed interval $[a, b]$.
///
/// `a` must be less than or equal to `b`. If `a` and `b` are equal, the range contains a single
/// element. To generate all [`Rational`]s in an infinite interval, use
/// [`exhaustive_rational_range_to_infinity`] or [`exhaustive_rational_range_to_negative_infinity`].
///
/// The output length is infinite if $a<b$ and 1 if $a=b$.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(\log i (\log \log i)^2 \log\log\log i)$
///
/// $M(i) = O(\log i \log \log i)$
///
/// where $T$ is time, $M$ is additional memory, and $i$ is the iteration number.
///
/// # Panics
/// Panics if $a>b$.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::conversion::traits::ExactFrom;
/// use malachite_q::exhaustive::exhaustive_rational_inclusive_range;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     prefix_to_string(
///         exhaustive_rational_inclusive_range(
///             Rational::exact_from(std::f64::consts::E),
///             Rational::exact_from(std::f64::consts::PI)
///         ),
///         20
///     ),
///     "[3, 11/4, 14/5, 20/7, 17/6, 23/8, 25/8, 30/11, 25/9, 29/10, 26/9, 31/11, 28/9, 31/10, \
///     32/11, 41/15, 34/11, 35/12, 37/12, 39/14, ...]"
/// )
/// ```
#[inline]
pub fn exhaustive_rational_inclusive_range(
    a: Rational,
    b: Rational,
) -> ExhaustiveRationalInclusiveRange {
    if a == b {
        ExhaustiveRationalInclusiveRange::Single(false, a)
    } else {
        ExhaustiveRationalInclusiveRange::Many(exhaustive_rational_inclusive_range_helper(a, b))
    }
}
