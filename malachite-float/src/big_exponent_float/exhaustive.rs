// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::Finite;
use alloc::vec::IntoIter;
use core::iter::{Chain, Once, once};

use core::mem::swap;
use malachite_base::iterators::bit_distributor::BitDistributorOutputType;
use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, One, Zero};
use malachite_base::num::exhaustive::{
    PrimitiveIntIncreasingRange, primitive_int_increasing_inclusive_range,
};
use malachite_base::num::iterators::{BitDistributorSequence, bit_distributor_sequence};
use malachite_base::num::logic::traits::NotAssign;
use malachite_base::num::modular::traits::NegModPowerOf2;
use malachite_base::tuples::exhaustive::{
    ExhaustiveDependentPairs, ExhaustiveDependentPairsYsGenerator, LexDependentPairs,
    exhaustive_dependent_pairs, lex_dependent_pairs,
};
use malachite_nz::integer::Integer;
use malachite_nz::integer::exhaustive::{IntegerUpDown, exhaustive_integers};
use malachite_nz::natural::Natural;
use malachite_nz::natural::exhaustive::{ExhaustiveNaturalRange, exhaustive_natural_range};
use malachite_nz::platform::Limb;

// The type of `exhaustive_integers`.
type ExhaustiveIntegers = Chain<Once<Integer>, IntegerUpDown>;

// Negates a `BigExponentFloat` without needing `Neg`.
fn negate(mut x: BigExponentFloat) -> BigExponentFloat {
    if let BigExponentFloat(Finite { sign, .. }) = &mut x {
        sign.not_assign();
    }
    x
}

/// Generates all positive [`BigExponentFloat`]s with a specified `sci_exponent` and precision.
///
/// This `struct` is created by
/// [`exhaustive_positive_big_exponent_floats_with_sci_exponent_and_precision`]; see its
/// documentation for more.
#[derive(Clone, Debug)]
pub struct ExhaustivePositiveBigExponentFloatsWithSciExponentAndPrecision {
    exponent: Integer,
    shift: u64,
    low: Natural,
    ks: ExhaustiveNaturalRange,
}

impl Iterator for ExhaustivePositiveBigExponentFloatsWithSciExponentAndPrecision {
    type Item = BigExponentFloat;

    #[inline]
    fn next(&mut self) -> Option<BigExponentFloat> {
        self.ks.next().map(|k| {
            BigExponentFloat(Finite {
                sign: true,
                exponent: self.exponent.clone(),
                significand: ((k << 1u32) + &self.low) << self.shift,
            })
        })
    }
}

/// Generates all positive [`BigExponentFloat`]s with a specified `sci_exponent` and precision.
///
/// Positive [`BigExponentFloat`]s may be written as $m2^e$, where $1\leq m<2$ and $e$ is an
/// integer; $e$ is the sci-exponent. The precision is the number of bits from the most significant
/// 1 of $m$ to its least significant 1, inclusive, so a [`BigExponentFloat`] with precision $p$ has
/// a significand $m$ whose last binary digit, $2^{1-p}$, is 1. There is one such value for $p=1$,
/// namely $2^e$, and $2^{p-2}$ of them for each $p>1$.
///
/// The output length is 1 if `prec` is 1 and $2^{p-2}$ otherwise.
///
/// # Worst-case complexity per iteration
/// $T(n) = O(n)$
///
/// $M(n) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
///
/// # Panics
/// Panics if `prec` is zero.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_float::big_exponent_float::exhaustive::*;
/// use malachite_nz::integer::Integer;
///
/// assert_eq!(
///     exhaustive_positive_big_exponent_floats_with_sci_exponent_and_precision(Integer::ZERO, 4)
///         .map(|x| format!("{x:#x}"))
///         .collect_vec(),
///     ["0x1.2", "0x1.6", "0x1.a", "0x1.e"]
/// );
/// ```
pub fn exhaustive_positive_big_exponent_floats_with_sci_exponent_and_precision(
    sci_exponent: Integer,
    prec: u64,
) -> ExhaustivePositiveBigExponentFloatsWithSciExponentAndPrecision {
    assert_ne!(prec, 0);
    // The odd significands with exactly `prec` bits are low + 2k, for k in [0, count).
    let (low, count) = if prec == 1 {
        (Natural::ONE, Natural::ONE)
    } else {
        (
            Natural::power_of_2(prec - 1) + Natural::ONE,
            Natural::power_of_2(prec - 2),
        )
    };
    ExhaustivePositiveBigExponentFloatsWithSciExponentAndPrecision {
        exponent: sci_exponent + Integer::ONE,
        shift: prec.neg_mod_power_of_2(Limb::LOG_WIDTH),
        low,
        ks: exhaustive_natural_range(Natural::ZERO, count),
    }
}

#[derive(Clone, Debug)]
struct BigExponentFloatsWithSciExponentAndPrecisionGenerator {
    sci_exponent: Integer,
}

impl
    ExhaustiveDependentPairsYsGenerator<
        u64,
        BigExponentFloat,
        ExhaustivePositiveBigExponentFloatsWithSciExponentAndPrecision,
    > for BigExponentFloatsWithSciExponentAndPrecisionGenerator
{
    #[inline]
    fn get_ys(
        &self,
        &prec: &u64,
    ) -> ExhaustivePositiveBigExponentFloatsWithSciExponentAndPrecision {
        exhaustive_positive_big_exponent_floats_with_sci_exponent_and_precision(
            self.sci_exponent.clone(),
            prec,
        )
    }
}

#[inline]
fn exhaustive_positive_big_exponent_floats_with_sci_exponent_helper(
    sci_exponent: Integer,
) -> LexDependentPairs<
    u64,
    BigExponentFloat,
    BigExponentFloatsWithSciExponentAndPrecisionGenerator,
    PrimitiveIntIncreasingRange<u64>,
    ExhaustivePositiveBigExponentFloatsWithSciExponentAndPrecision,
> {
    lex_dependent_pairs(
        primitive_int_increasing_inclusive_range(1, u64::MAX),
        BigExponentFloatsWithSciExponentAndPrecisionGenerator { sci_exponent },
    )
}

/// Generates all positive [`BigExponentFloat`]s with a specified `sci_exponent`.
///
/// This `struct` is created by [`exhaustive_positive_big_exponent_floats_with_sci_exponent`]; see
/// its documentation for more.
#[derive(Clone, Debug)]
pub struct ExhaustivePositiveBigExponentFloatsWithSciExponent(
    LexDependentPairs<
        u64,
        BigExponentFloat,
        BigExponentFloatsWithSciExponentAndPrecisionGenerator,
        PrimitiveIntIncreasingRange<u64>,
        ExhaustivePositiveBigExponentFloatsWithSciExponentAndPrecision,
    >,
);

impl Iterator for ExhaustivePositiveBigExponentFloatsWithSciExponent {
    type Item = BigExponentFloat;

    #[inline]
    fn next(&mut self) -> Option<BigExponentFloat> {
        self.0.next().map(|p| p.1)
    }
}

/// Generates all positive [`BigExponentFloat`]s with a specified `sci_exponent`.
///
/// Positive [`BigExponentFloat`]s may be written as $m2^e$, where $1\leq m<2$ and $e$ is an
/// integer; $e$ is the sci-exponent. The values are generated in order of increasing precision.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(n) = O(n)$
///
/// $M(n) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is the precision of the output.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_float::big_exponent_float::exhaustive::*;
/// use malachite_nz::integer::Integer;
///
/// assert_eq!(
///     exhaustive_positive_big_exponent_floats_with_sci_exponent(Integer::ZERO)
///         .take(8)
///         .map(|x| format!("{x:#x}"))
///         .collect_vec(),
///     ["0x1.0", "0x1.8", "0x1.4", "0x1.c", "0x1.2", "0x1.6", "0x1.a", "0x1.e"]
/// );
/// ```
#[inline]
pub fn exhaustive_positive_big_exponent_floats_with_sci_exponent(
    sci_exponent: Integer,
) -> ExhaustivePositiveBigExponentFloatsWithSciExponent {
    ExhaustivePositiveBigExponentFloatsWithSciExponent(
        exhaustive_positive_big_exponent_floats_with_sci_exponent_helper(sci_exponent),
    )
}

#[derive(Clone, Debug)]
struct BigExponentFloatsWithPrecisionAndSciExponentGenerator {
    precision: u64,
}

impl
    ExhaustiveDependentPairsYsGenerator<
        Integer,
        BigExponentFloat,
        ExhaustivePositiveBigExponentFloatsWithSciExponentAndPrecision,
    > for BigExponentFloatsWithPrecisionAndSciExponentGenerator
{
    #[inline]
    fn get_ys(
        &self,
        exp: &Integer,
    ) -> ExhaustivePositiveBigExponentFloatsWithSciExponentAndPrecision {
        exhaustive_positive_big_exponent_floats_with_sci_exponent_and_precision(
            exp.clone(),
            self.precision,
        )
    }
}

#[inline]
fn exhaustive_big_exponent_floats_with_precision_helper(
    prec: u64,
) -> ExhaustiveDependentPairs<
    Integer,
    BigExponentFloat,
    BitDistributorSequence,
    BigExponentFloatsWithPrecisionAndSciExponentGenerator,
    ExhaustiveIntegers,
    ExhaustivePositiveBigExponentFloatsWithSciExponentAndPrecision,
> {
    exhaustive_dependent_pairs(
        bit_distributor_sequence(
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
        ),
        exhaustive_integers(),
        BigExponentFloatsWithPrecisionAndSciExponentGenerator { precision: prec },
    )
}

/// Generates all positive [`BigExponentFloat`]s with a specified precision.
///
/// This `struct` is created by [`exhaustive_positive_big_exponent_floats_with_precision`]; see its
/// documentation for more.
#[derive(Clone, Debug)]
pub struct ExhaustivePositiveBigExponentFloatsWithPrecision(
    ExhaustiveDependentPairs<
        Integer,
        BigExponentFloat,
        BitDistributorSequence,
        BigExponentFloatsWithPrecisionAndSciExponentGenerator,
        ExhaustiveIntegers,
        ExhaustivePositiveBigExponentFloatsWithSciExponentAndPrecision,
    >,
);

impl Iterator for ExhaustivePositiveBigExponentFloatsWithPrecision {
    type Item = BigExponentFloat;

    #[inline]
    fn next(&mut self) -> Option<BigExponentFloat> {
        self.0.next().map(|p| p.1)
    }
}

/// Generates all positive [`BigExponentFloat`]s with a specified precision.
///
/// The precision is the number of bits from the most significant 1 to the least significant 1,
/// inclusive. Every sci-exponent, an unbounded [`Integer`], is eventually reached.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(n, m) = O(n + m)$
///
/// $M(n, m) = O(n + m)$
///
/// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is the number of bits of
/// the output's exponent.
///
/// # Panics
/// Panics if `prec` is zero.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_float::big_exponent_float::exhaustive::*;
///
/// assert_eq!(
///     exhaustive_positive_big_exponent_floats_with_precision(2)
///         .take(8)
///         .map(|x| format!("{x:#x}"))
///         .collect_vec(),
///     ["0x1.8", "0x3.0", "0x0.c", "0x6.0", "0x0.6", "0xc.0", "0x0.3", "0x18.0"]
/// );
/// ```
#[inline]
pub fn exhaustive_positive_big_exponent_floats_with_precision(
    prec: u64,
) -> ExhaustivePositiveBigExponentFloatsWithPrecision {
    assert_ne!(prec, 0);
    ExhaustivePositiveBigExponentFloatsWithPrecision(
        exhaustive_big_exponent_floats_with_precision_helper(prec),
    )
}

// Alternates the values of a generator of positive `BigExponentFloat`s with their negatives.
#[derive(Clone, Debug)]
struct WithNegatives<I: Iterator<Item = BigExponentFloat>> {
    toggle: bool,
    xs: I,
    x: BigExponentFloat,
}

impl<I: Iterator<Item = BigExponentFloat>> Iterator for WithNegatives<I> {
    type Item = BigExponentFloat;

    #[inline]
    fn next(&mut self) -> Option<BigExponentFloat> {
        self.toggle.not_assign();
        Some(if self.toggle {
            self.x = self.xs.next().unwrap();
            self.x.clone()
        } else {
            let mut out = BigExponentFloat::NAN;
            swap(&mut out, &mut self.x);
            negate(out)
        })
    }
}

/// Generates all [`BigExponentFloat`]s with a specified precision.
///
/// This `struct` is created by [`exhaustive_big_exponent_floats_with_precision`]; see its
/// documentation for more.
#[derive(Clone, Debug)]
pub struct ExhaustiveBigExponentFloatsWithPrecision(
    WithNegatives<ExhaustivePositiveBigExponentFloatsWithPrecision>,
);

impl Iterator for ExhaustiveBigExponentFloatsWithPrecision {
    type Item = BigExponentFloat;

    #[inline]
    fn next(&mut self) -> Option<BigExponentFloat> {
        self.0.next()
    }
}

/// Generates all nonzero [`BigExponentFloat`]s with a specified precision. (Zero has no
/// precision.)
///
/// Each positive value is followed by its negative.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(n, m) = O(n + m)$
///
/// $M(n, m) = O(n + m)$
///
/// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is the number of bits of
/// the output's exponent.
///
/// # Panics
/// Panics if `prec` is zero.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_float::big_exponent_float::exhaustive::*;
///
/// assert_eq!(
///     exhaustive_big_exponent_floats_with_precision(1)
///         .take(8)
///         .map(|x| format!("{x:#x}"))
///         .collect_vec(),
///     ["0x1.0", "-0x1.0", "0x2.0", "-0x2.0", "0x0.8", "-0x0.8", "0x4.0", "-0x4.0"]
/// );
/// ```
#[inline]
pub fn exhaustive_big_exponent_floats_with_precision(
    prec: u64,
) -> ExhaustiveBigExponentFloatsWithPrecision {
    ExhaustiveBigExponentFloatsWithPrecision(WithNegatives {
        toggle: false,
        xs: exhaustive_positive_big_exponent_floats_with_precision(prec),
        x: BigExponentFloat::NAN,
    })
}

#[derive(Clone, Debug)]
pub(crate) struct ExhaustivePositiveFiniteBigExponentFloatsGenerator;

impl
    ExhaustiveDependentPairsYsGenerator<
        Integer,
        BigExponentFloat,
        ExhaustivePositiveBigExponentFloatsWithSciExponent,
    > for ExhaustivePositiveFiniteBigExponentFloatsGenerator
{
    #[inline]
    fn get_ys(&self, sci_exponent: &Integer) -> ExhaustivePositiveBigExponentFloatsWithSciExponent {
        exhaustive_positive_big_exponent_floats_with_sci_exponent(sci_exponent.clone())
    }
}

/// Generates all positive finite [`BigExponentFloat`]s.
///
/// This `struct` is created by [`exhaustive_positive_finite_big_exponent_floats`]; see its
/// documentation for more.
#[derive(Clone, Debug)]
pub struct ExhaustivePositiveFiniteBigExponentFloats(
    ExhaustiveDependentPairs<
        Integer,
        BigExponentFloat,
        BitDistributorSequence,
        ExhaustivePositiveFiniteBigExponentFloatsGenerator,
        ExhaustiveIntegers,
        ExhaustivePositiveBigExponentFloatsWithSciExponent,
    >,
);

impl Iterator for ExhaustivePositiveFiniteBigExponentFloats {
    type Item = BigExponentFloat;

    #[inline]
    fn next(&mut self) -> Option<BigExponentFloat> {
        self.0.next().map(|p| p.1)
    }
}

/// Generates all positive finite [`BigExponentFloat`]s.
///
/// Each value is generated exactly once: a [`BigExponentFloat`] has no stored precision, so there
/// are no separate copies of a value at different precisions, as there are for
/// [`Float`](crate::Float).
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(n) = O(n)$
///
/// $M(n) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `out.complexity()`.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_float::big_exponent_float::exhaustive::*;
///
/// assert_eq!(
///     exhaustive_positive_finite_big_exponent_floats()
///         .take(8)
///         .map(|x| format!("{x:#x}"))
///         .collect_vec(),
///     ["0x1.0", "0x2.0", "0x1.8", "0x3.0", "0x0.8", "0x4.0", "0x0.c", "0x6.0"]
/// );
/// ```
#[inline]
pub fn exhaustive_positive_finite_big_exponent_floats() -> ExhaustivePositiveFiniteBigExponentFloats
{
    ExhaustivePositiveFiniteBigExponentFloats(exhaustive_dependent_pairs(
        bit_distributor_sequence(
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
        ),
        exhaustive_integers(),
        ExhaustivePositiveFiniteBigExponentFloatsGenerator,
    ))
}

/// Generates all negative finite [`BigExponentFloat`]s.
///
/// This `struct` is created by [`exhaustive_negative_finite_big_exponent_floats`]; see its
/// documentation for more.
#[derive(Clone, Debug)]
pub struct ExhaustiveNegativeFiniteBigExponentFloats(ExhaustivePositiveFiniteBigExponentFloats);

impl Iterator for ExhaustiveNegativeFiniteBigExponentFloats {
    type Item = BigExponentFloat;

    #[inline]
    fn next(&mut self) -> Option<BigExponentFloat> {
        self.0.next().map(negate)
    }
}

/// Generates all negative finite [`BigExponentFloat`]s.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(n) = O(n)$
///
/// $M(n) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `out.complexity()`.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_float::big_exponent_float::exhaustive::*;
///
/// assert_eq!(
///     exhaustive_negative_finite_big_exponent_floats()
///         .take(4)
///         .map(|x| format!("{x:#x}"))
///         .collect_vec(),
///     ["-0x1.0", "-0x2.0", "-0x1.8", "-0x3.0"]
/// );
/// ```
#[inline]
pub fn exhaustive_negative_finite_big_exponent_floats() -> ExhaustiveNegativeFiniteBigExponentFloats
{
    ExhaustiveNegativeFiniteBigExponentFloats(exhaustive_positive_finite_big_exponent_floats())
}

/// Generates all nonzero finite [`BigExponentFloat`]s.
///
/// This `struct` is created by [`exhaustive_nonzero_finite_big_exponent_floats`]; see its
/// documentation for more.
#[derive(Clone, Debug)]
pub struct ExhaustiveNonzeroFiniteBigExponentFloats(
    WithNegatives<ExhaustivePositiveFiniteBigExponentFloats>,
);

impl Iterator for ExhaustiveNonzeroFiniteBigExponentFloats {
    type Item = BigExponentFloat;

    #[inline]
    fn next(&mut self) -> Option<BigExponentFloat> {
        self.0.next()
    }
}

/// Generates all nonzero finite [`BigExponentFloat`]s.
///
/// Each positive value is followed by its negative.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(n) = O(n)$
///
/// $M(n) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `out.complexity()`.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_float::big_exponent_float::exhaustive::*;
///
/// assert_eq!(
///     exhaustive_nonzero_finite_big_exponent_floats()
///         .take(6)
///         .map(|x| format!("{x:#x}"))
///         .collect_vec(),
///     ["0x1.0", "-0x1.0", "0x2.0", "-0x2.0", "0x1.8", "-0x1.8"]
/// );
/// ```
#[inline]
pub fn exhaustive_nonzero_finite_big_exponent_floats() -> ExhaustiveNonzeroFiniteBigExponentFloats {
    ExhaustiveNonzeroFiniteBigExponentFloats(WithNegatives {
        toggle: false,
        xs: exhaustive_positive_finite_big_exponent_floats(),
        x: BigExponentFloat::NAN,
    })
}

/// Generates all non-negative finite [`BigExponentFloat`]s: zero, followed by the positive ones.
pub type ExhaustiveNonNegativeFiniteBigExponentFloats =
    Chain<Once<BigExponentFloat>, ExhaustivePositiveFiniteBigExponentFloats>;

/// Generates all non-negative finite [`BigExponentFloat`]s.
///
/// Zero is generated first, followed by the values of
/// [`exhaustive_positive_finite_big_exponent_floats`].
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(n) = O(n)$
///
/// $M(n) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `out.complexity()`.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_float::big_exponent_float::exhaustive::*;
///
/// assert_eq!(
///     exhaustive_non_negative_finite_big_exponent_floats()
///         .take(4)
///         .map(|x| format!("{x:#x}"))
///         .collect_vec(),
///     ["0x0.0", "0x1.0", "0x2.0", "0x1.8"]
/// );
/// ```
#[inline]
pub fn exhaustive_non_negative_finite_big_exponent_floats()
-> ExhaustiveNonNegativeFiniteBigExponentFloats {
    once(BigExponentFloat::ZERO).chain(exhaustive_positive_finite_big_exponent_floats())
}

/// Generates all non-positive finite [`BigExponentFloat`]s: zero, followed by the negative ones.
pub type ExhaustiveNonPositiveFiniteBigExponentFloats =
    Chain<Once<BigExponentFloat>, ExhaustiveNegativeFiniteBigExponentFloats>;

/// Generates all non-positive finite [`BigExponentFloat`]s.
///
/// Zero is generated first, followed by the values of
/// [`exhaustive_negative_finite_big_exponent_floats`].
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(n) = O(n)$
///
/// $M(n) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `out.complexity()`.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_float::big_exponent_float::exhaustive::*;
///
/// assert_eq!(
///     exhaustive_non_positive_finite_big_exponent_floats()
///         .take(4)
///         .map(|x| format!("{x:#x}"))
///         .collect_vec(),
///     ["0x0.0", "-0x1.0", "-0x2.0", "-0x1.8"]
/// );
/// ```
#[inline]
pub fn exhaustive_non_positive_finite_big_exponent_floats()
-> ExhaustiveNonPositiveFiniteBigExponentFloats {
    once(BigExponentFloat::ZERO).chain(exhaustive_negative_finite_big_exponent_floats())
}

/// Generates all finite [`BigExponentFloat`]s, or all [`BigExponentFloat`]s: some special values,
/// followed by the nonzero finite ones.
pub type ExhaustiveBigExponentFloats =
    Chain<IntoIter<BigExponentFloat>, ExhaustiveNonzeroFiniteBigExponentFloats>;

/// Generates all finite [`BigExponentFloat`]s.
///
/// Zero is generated first, followed by the values of
/// [`exhaustive_nonzero_finite_big_exponent_floats`].
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(n) = O(n)$
///
/// $M(n) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `out.complexity()`.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_float::big_exponent_float::exhaustive::*;
///
/// assert_eq!(
///     exhaustive_finite_big_exponent_floats()
///         .take(5)
///         .map(|x| format!("{x:#x}"))
///         .collect_vec(),
///     ["0x0.0", "0x1.0", "-0x1.0", "0x2.0", "-0x2.0"]
/// );
/// ```
#[inline]
pub fn exhaustive_finite_big_exponent_floats() -> ExhaustiveBigExponentFloats {
    alloc::vec![BigExponentFloat::ZERO]
        .into_iter()
        .chain(exhaustive_nonzero_finite_big_exponent_floats())
}

/// Generates all [`BigExponentFloat`]s.
///
/// NaN, $\infty$, $-\infty$, and zero are generated first, followed by the values of
/// [`exhaustive_nonzero_finite_big_exponent_floats`].
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(n) = O(n)$
///
/// $M(n) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `out.complexity()`.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_float::big_exponent_float::exhaustive::*;
///
/// assert_eq!(
///     exhaustive_big_exponent_floats()
///         .take(6)
///         .map(|x| format!("{x:#x}"))
///         .collect_vec(),
///     ["NaN", "Infinity", "-Infinity", "0x0.0", "0x1.0", "-0x1.0"]
/// );
/// ```
#[inline]
pub fn exhaustive_big_exponent_floats() -> ExhaustiveBigExponentFloats {
    alloc::vec![
        BigExponentFloat::NAN,
        BigExponentFloat::INFINITY,
        BigExponentFloat::NEGATIVE_INFINITY,
        BigExponentFloat::ZERO,
    ]
    .into_iter()
    .chain(exhaustive_nonzero_finite_big_exponent_floats())
}
